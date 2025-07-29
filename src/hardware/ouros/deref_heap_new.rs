// Dereference Heap, also handles thread management:
//           +-----------------+
// addr  <-->|    Dereference  |===> out_main
// port_a ==>|                 |
// port_b ==>|       Heap      |===> out_sub
//           +-----------------+

use super::config::APP_LENGTH;
use super::ouros_core::{is_int, is_prm};
use super::program::{app_length, is_whnf, ActiveApp, App, Atom, FrozenApp, Program};
use crate::hardware::common::memory::DualPortMemStat;
use crate::hardware::common::{DualPortMem, Register, Stack};
use crate::hardware::ouros::ouros_core::is_ptr;
use crate::hardware::utils::fire;
use crate::hw_module::{HwInput, HwModule};
use std::fmt;

#[derive(Default)]
pub struct DrfHeapInput {
    pub start: bool,
    pub port_a_valid: bool,
    pub port_a_bits: ActiveApp,
    pub port_b_valid: bool,
    pub port_b_bits: FrozenApp,
    pub out_main_ready: bool,
    pub out_sub_ready: bool,
    pub addr_consumed: usize, // address request from reducer (for GC)
}

impl HwInput for DrfHeapInput {}

type StackCell = (bool, usize);
type AddrStack = Stack<StackCell, 512>;
type FrameRecord = [usize; 8];
type FrameStack = Stack<FrameRecord, 64>;

fn stack_cell_with(cell: Option<&StackCell>, p: impl FnOnce(&StackCell) -> bool) -> bool {
    match cell {
        Some(c) => p(c),
        None => false,
    }
}

fn find_more_dmder(address: usize) -> impl Fn(&AddrStack) -> bool {
    move |s| {
        stack_cell_with(s.top(), |(_, addr)| *addr == address)
            && stack_cell_with(s.second(), |(flag, _)| !*flag)
    }
}

fn find_new_frame(address: usize) -> impl Fn(&AddrStack) -> bool {
    move |s| {
        stack_cell_with(s.top(), |(_, addr)| *addr == address)
            && stack_cell_with(s.second(), |(flag, _)| *flag)
    }
}

fn find_free_stack(s: &AddrStack, target_addr: usize) -> bool {
    s.elements() == 0 || stack_cell_with(s.top(), |(flag, addr)| *flag && *addr == target_addr)
}

fn extend_to_app<const N: usize>(atms: &[Atom; N]) -> App {
    let mut extended: App = std::array::from_fn(|_| Atom::NOP);
    for (i, a) in atms.iter().enumerate() {
        extended[i] = a.clone();
    }
    extended
}

enum HeapPort {
    A,
    B,
}

#[derive(Default, Clone, PartialEq, Debug)]
pub enum Stm {
    #[default]
    IDLE,
    WHNF,
    IA,
    RESUME,
}

#[derive(Default, Clone, Debug)]
enum StmSub {
    #[default]
    IDLE,
    WORK,
}

#[derive(Default)]
pub struct DrfHeapStat {
    active_threads: u8,
    pub work_threads: Vec<(u8, u8)>, // (occupied resources, active threads)
    pub holder_contents: Vec<Option<ActiveApp>>,
    pub heap_stm: Vec<Stm>,
    pub serving_id: Vec<u8>,
    pub stm_cycles: [u32; 4],
    pub wasted_cycles: u32,
}

/// branch conditions for `consume_next()`
#[derive(Debug)]
enum CONSUMEs {
    NoInput,
    /// input is an IA
    InputIA,
    /// input is an WHNF; demander found in any stacks
    InputWHNFWithDmder,
    /// input is an WHNF; no demander is found; different frame under the WHNF
    InputWHNFNoDmderNewFrame,
    /// input is an WHNF; no demander is found; the WHNF the only element on the stack
    InputWHNFNoDmderNoFrame,
}

/// branch conditions for the `WHNF` state
enum WHNFs {
    MoreDmders,
    NewFrame,
    NoNewFrame,
}

/// branch conditions for the `IA` state, part 1
#[derive(PartialEq, Debug)]
enum IAs1 {
    NoExist,
    ExistWHNF,
    ExistIAWorkingNormal,
    ExistIAWorkingAtNewFrame,
    ExistIAFresh,
}

/// branch conditions for the `IA` state, part 2
#[derive(Debug)]
enum IAs2 {
    NextStrictArgLocal,
    NextStrictArgNewStk,
    NoMoreArgsCanEmit,
    NoMoreArgsNoEmit,
}

/// branch conditions for the `RESUME` state
enum RESUMEs {
    TopInWHNF,
    TopInIA,
}

/// branch conditions for the `WORK` state
#[derive(Debug)]
enum WORKs {
    NotDemanded,
    DmderFound,
    DmderNotFound,
}

/// select the first pointer to deref, returns (arg position, pointer value)
fn select_1st_arg(app: &App) -> (usize, usize) {
    match app[0] {
        Atom::PTR(p) => (0, p),
        Atom::PRM(_, _) => match app[1] {
            Atom::PTR(p) => (1, p),
            Atom::INT(_) => match app[2] {
                Atom::PTR(p) => (2, p),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

/// select the next strict arg, returns (arg position, pointer value)
fn select_next_arg(app: &App, current: usize) -> (usize, usize) {
    match app[2] {
        Atom::PTR(p) => (2, p),
        _ => unreachable!(),
    }
}

/// Dereference `app`'s PTR at position `arg_id`, with `target`
fn deref(app: &App, arg_id: usize, target: &App) -> App {
    assert!(is_ptr(&app[arg_id]));
    let app_len = app_length(app);
    let target_len = app_length(target);
    // TODO: fix this with runtime splition
    if app_len + target_len - 1 > APP_LENGTH {
        panic!("dheap: deref: deref result too long!");
    }

    let mut res: App = std::array::from_fn(|_| Atom::NOP);
    for i in 0..arg_id {
        res[i] = app[i].clone();
    }
    for i in 0..target_len {
        res[arg_id + i] = target[i].clone();
    }
    for i in 0..app_len - arg_id - 1 {
        res[arg_id + target_len + i] = app[arg_id + 1 + i].clone();
    }
    res
}

#[test]
fn deref_spec() {
    use Atom::*;
    let app: App = [PTR(0), INT(1), PTR(2), INT(3), NOP, NOP, NOP, NOP];
    let target: App = [PTR(11), PTR(22), PTR(33), PTR(44), PTR(55), NOP, NOP, NOP];
    let res1: App = [
        PTR(11),
        PTR(22),
        PTR(33),
        PTR(44),
        PTR(55),
        INT(1),
        PTR(2),
        INT(3),
    ];
    let res2: App = [
        PTR(0),
        INT(1),
        PTR(11),
        PTR(22),
        PTR(33),
        PTR(44),
        PTR(55),
        INT(3),
    ];
    assert_eq!(deref(&app, 0, &target), res1);
    assert_eq!(deref(&app, 2, &target), res2);
}

impl fmt::Display for Stm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stm::IDLE => write!(f, "IDLE"),
            Stm::WHNF => write!(f, "WHNF"),
            Stm::IA => write!(f, "IA"),
            Stm::RESUME => write!(f, "RESUME"),
        }
    }
}

#[derive(Default, Clone)]
struct HeapCell {
    exist: bool,
    app: App,
}

pub struct DrfHeap {
    pub input: DrfHeapInput,
    stm: Register<Stm>,
    stm_sub: Register<StmSub>,
    holder_in: Register<ActiveApp>,
    holder_in_sub: Register<FrozenApp>,
    addr_holder: Register<usize>,
    ia_addr: Register<usize>,
    father_stk: Register<u8>,
    thread_stack: [AddrStack; 8],
    frame_stack: [FrameStack; 8],
    heap_mem: DualPortMem<HeapCell>,
    demand_heap: DualPortMem<bool>,
    working_heap: DualPortMem<bool>,
    holder_out: (bool, ActiveApp), // output-reg: (valid, app)
    holder_out_sub: (bool, ActiveApp),
    working: Register<bool>, // track whether the machine is working
    addr_bumper: Register<usize>,
    arg_id: Register<usize>,
    stat: DrfHeapStat,
}

impl DrfHeap {
    pub fn new(heap_size: usize) -> Self {
        Self {
            input: Default::default(),
            stm: Default::default(),
            stm_sub: Default::default(),
            holder_in: Default::default(),
            holder_in_sub: Default::default(),
            addr_holder: Default::default(),
            ia_addr: Default::default(),
            father_stk: Default::default(),
            thread_stack: std::array::from_fn(|_| Stack::new()),
            frame_stack: std::array::from_fn(|_| Stack::new()),
            heap_mem: DualPortMem::new(heap_size),
            working_heap: DualPortMem::new(heap_size),
            demand_heap: DualPortMem::new(heap_size),
            holder_out: Default::default(),
            working: Default::default(),
            addr_bumper: Default::default(),
            stat: Default::default(),
            holder_out_sub: Default::default(),
            arg_id: Default::default(),
        }
    }

    /// When creating a `DrfHeap`, put a compiled program into the heap memory.
    pub fn program(mut self, prog: &Program) -> Self {
        fn convert(atms: &Vec<Atom>) -> HeapCell {
            assert!(atms.len() <= APP_LENGTH);
            let mut app: App = std::array::from_fn(|_| Atom::NOP);
            for (i, atm) in atms.iter().enumerate() {
                app[i] = atm.clone();
            }
            HeapCell { exist: true, app }
        }
        // convert Vec<Vec<Atom>> to Vec<HeapCell>
        let img: Vec<HeapCell> = prog.iter().map(convert).collect();
        self.heap_mem.image(&img);
        self.addr_bumper.connect(&img.len());
        self.addr_bumper.tick();
        self
    }

    fn port_a_fire(&self) -> bool {
        fire(self.input.port_a_valid, self.port_a_ready())
    }

    fn port_b_fire(&self) -> bool {
        fire(self.input.port_b_valid, self.port_b_ready())
    }

    fn output_fire(&self) -> bool {
        fire(self.out_main_valid(), self.input.out_main_ready)
    }

    fn out_sub_fire(&self) -> bool {
        fire(self.out_sub_valid(), self.input.out_sub_ready)
    }

    pub fn port_a_ready(&self) -> bool {
        let out_clear = !self.holder_out.0 || self.input.out_main_ready;
        let local: bool = match *self.stm.value() {
            Stm::IDLE => true,
            Stm::WHNF => match self.getWHNFs() {
                WHNFs::NoNewFrame => true,
                _ => false,
            },
            Stm::IA => {
                let ias1 = self.getIAs1();
                match self.getIAs2(&ias1) {
                    IAs2::NoMoreArgsCanEmit | IAs2::NoMoreArgsNoEmit => !self.is_sensitive(&ias1),
                    _ => false,
                }
            }
            Stm::RESUME => match self.getRESUMEs() {
                RESUMEs::TopInWHNF => false,
                RESUMEs::TopInIA => true,
            },
        };
        out_clear && local
    }

    pub fn port_b_ready(&self) -> bool {
        let out_clear = !self.holder_out_sub.0 || self.input.out_sub_ready;
        let local: bool = match *self.stm_sub.value() {
            StmSub::IDLE => true,
            StmSub::WORK => match self.getWORKs() {
                WORKs::NotDemanded => true,
                WORKs::DmderFound => true,
                WORKs::DmderNotFound => false,
            },
        };
        let borrowed: bool = match *self.stm.value() {
            Stm::IDLE => false,
            Stm::WHNF => match self.getWHNFs() {
                WHNFs::MoreDmders => false,
                WHNFs::NewFrame => false,
                WHNFs::NoNewFrame => true,
            },
            Stm::IA => {
                let ias1 = self.getIAs1();
                match self.getIAs2(&ias1) {
                    IAs2::NoMoreArgsNoEmit => true,
                    _ => false,
                }
            }
            Stm::RESUME => true,
        };
        // if !(out_clear && local && !borrowed) {
        //     println!(
        //         "STRANGE!OUT CLEAR {}, LOCAL {}, BORROWED {}, stm {:?}",
        //         out_clear,
        //         local,
        //         borrowed,
        //         self.stm.value()
        //     );
        // }
        out_clear && local && !borrowed
    }

    pub fn out_main_valid(&self) -> bool {
        let derived: bool = {
            match *self.stm.value() {
                Stm::IDLE => false,
                Stm::WHNF => true,
                Stm::IA => {
                    let ias1 = self.getIAs1();
                    if ias1 == IAs1::ExistIAFresh {
                        return true;
                    }
                    match self.getIAs2(&ias1) {
                        IAs2::NoMoreArgsCanEmit => true,
                        _ => false,
                    }
                }
                Stm::RESUME => false,
            }
        };
        self.holder_out.0 || derived
    }

    pub fn out_main_bits(&self) -> ActiveApp {
        if self.holder_out.0 {
            // println!("return earlier");
            return self.holder_out.1.clone();
        }

        // println!("we're here!");
        match *self.stm.value() {
            Stm::IDLE => Default::default(),
            Stm::WHNF => {
                let deref_res = self.gen_output_whnf();
                self.gen_active_app(deref_res)
            }
            Stm::IA => {
                let ias1 = self.getIAs1();
                if ias1 == IAs1::ExistIAFresh {
                    // println!("we're here!");
                    return self.gen_active_app(self.heap_mem.dout_a().app.clone());
                }
                match self.getIAs2(&ias1) {
                    IAs2::NoMoreArgsCanEmit => {
                        let updated_dmder = deref(
                            &self.holder_in.value().load,
                            *self.arg_id.value(),
                            &self.heap_mem.dout_a().app,
                        );
                        self.gen_active_app(updated_dmder)
                    }
                    _ => Default::default(),
                }
            }
            Stm::RESUME => Default::default(),
        }
    }

    pub fn out_sub_valid(&self) -> bool {
        let derived: bool = {
            match *self.stm_sub.value() {
                StmSub::IDLE => false,
                StmSub::WORK => match self.getWORKs() {
                    WORKs::DmderFound => true,
                    _ => false,
                },
            }
        };
        self.holder_out_sub.0 || derived
    }

    pub fn out_sub_bits(&self) -> ActiveApp {
        if self.holder_out_sub.0 {
            return self.holder_out_sub.1.clone();
        }
        match *self.stm_sub.value() {
            StmSub::IDLE => Default::default(),
            StmSub::WORK => match self.getWORKs() {
                WORKs::DmderFound => {
                    let load = extend_to_app(&self.holder_in_sub.value().load);
                    ActiveApp {
                        stack_idx: self.find_dmder_stk(),
                        load,
                    }
                }
                _ => Default::default(),
            },
        }
    }

    /// machine's work has been finished
    pub fn done(&self) -> bool {
        !self.working.value()
    }

    pub fn free_addr(&self) -> usize {
        *self.addr_bumper.value()
    }

    pub fn get_stat(&self) -> &DrfHeapStat {
        &self.stat
    }

    pub fn get_mem_stat(&self) -> &DualPortMemStat {
        self.heap_mem.get_stat()
    }

    fn getCONSUMEs(&self) -> CONSUMEs {
        if !self.port_a_fire() {
            CONSUMEs::NoInput
        } else {
            let stk = &self.thread_stack[self.input.port_a_bits.stack_idx as usize];
            if is_whnf(&self.input.port_a_bits.load) {
                if self
                    .thread_stack
                    .iter()
                    .any(find_more_dmder(stk.top().unwrap().1))
                {
                    CONSUMEs::InputWHNFWithDmder
                } else {
                    if stk.second() != None {
                        if stack_cell_with(stk.second(), |(flag, _)| !*flag) {
                            panic!("strange new frame!");
                        }
                        CONSUMEs::InputWHNFNoDmderNewFrame
                    } else {
                        CONSUMEs::InputWHNFNoDmderNoFrame
                    }
                }
            } else {
                CONSUMEs::InputIA
            }
        }
    }

    fn getWHNFs(&self) -> WHNFs {
        if self
            .thread_stack
            .iter()
            .any(find_more_dmder(*self.addr_holder.value()))
        {
            // println!("MoreDmders!");
            // println!(
            //     "addr holder: {}, stack top: {:?}, stack depth: {}",
            //     self.addr_holder.value(),
            //     self.thread_stack[0].top(),
            //     self.thread_stack[0].elements()
            // );
            WHNFs::MoreDmders
        } else {
            if self
                .thread_stack
                .iter()
                .any(find_new_frame(*self.addr_holder.value()))
            {
                WHNFs::NewFrame
            } else {
                WHNFs::NoNewFrame
            }
        }
    }

    fn getIAs1(&self) -> IAs1 {
        // println!(
        //     "exist: {} working: {}",
        //     self.heap_mem.dout_a().exist,
        //     *self.working_heap.dout_a()
        // );
        let target = &self.heap_mem.dout_a().app;
        let stk = &self.thread_stack[self.holder_in.value().stack_idx as usize];
        if !self.heap_mem.dout_a().exist {
            IAs1::NoExist
        } else {
            if is_whnf(target) {
                // println!("existWHNF");
                IAs1::ExistWHNF
            } else {
                if !*self.working_heap.dout_a() {
                    // println!("existIAFresh");
                    IAs1::ExistIAFresh
                } else {
                    if stack_cell_with(stk.top(), |(flag, _)| !*flag) {
                        IAs1::ExistIAWorkingNormal
                    } else {
                        IAs1::ExistIAWorkingAtNewFrame
                    }
                }
            }
        }
    }

    fn getIAs2(&self, s1: &IAs1) -> IAs2 {
        let ia = &self.holder_in.value().load;
        let target_in_whnf = self.heap_mem.dout_a().exist && is_whnf(&self.heap_mem.dout_a().app);
        let frame_record = self.frame_stack[self.holder_in.value().stack_idx as usize]
            .top()
            .unwrap();
        let idle_stack: bool = self
            .thread_stack
            .iter()
            .enumerate()
            .any(|(idx, s)| find_free_stack(s, frame_record[idx]));
        let local_stack: bool = *s1 == IAs1::ExistWHNF || *s1 == IAs1::ExistIAWorkingAtNewFrame;
        let more_strict_args: bool = {
            match ia[0] {
                Atom::PTR(_) => false,
                Atom::PRM(_, _) => *self.arg_id.value() == 1 && !is_int(&ia[2]),
                // more on this to support strict args in the future
                _ => unreachable!(),
            }
        };

        if more_strict_args && local_stack {
            IAs2::NextStrictArgLocal
        } else if more_strict_args && idle_stack {
            IAs2::NextStrictArgNewStk
        } else {
            if (is_ptr(&ia[0])
                || (is_prm(&ia[0]) && is_int(&ia[1]))
                || (is_prm(&ia[0]) && is_int(&ia[2])))
                && target_in_whnf
            {
                IAs2::NoMoreArgsCanEmit
            } else {
                IAs2::NoMoreArgsNoEmit
            }
        }
    }

    fn getRESUMEs(&self) -> RESUMEs {
        let top = self.heap_mem.dout_a().app.clone();
        if is_whnf(&top) {
            RESUMEs::TopInWHNF
        } else {
            RESUMEs::TopInIA
        }
    }

    fn getWORKs(&self) -> WORKs {
        if *self.demand_heap.dout_b() {
            let addr = self.holder_in_sub.value().heap_addr;
            if self
                .thread_stack
                .iter()
                .any(|s| stack_cell_with(s.top(), |(_, a)| *a == addr))
            {
                WORKs::DmderFound
            } else {
                WORKs::DmderNotFound
            }
        } else {
            WORKs::NotDemanded
        }
    }

    /// preparations in each `update_local`
    fn update_prepare(&mut self) {
        // always give default inputs at the beginning of a cycle
        self.heap_mem.input.default_input();
        self.demand_heap.input.default_input();
        self.working_heap.input.default_input();
        for stk in &mut self.thread_stack {
            stk.input.default_input();
        }
        for stk in &mut self.frame_stack {
            stk.input.default_input();
        }

        // rise addr bumper
        self.addr_bumper
            .connect(&(self.addr_bumper.value() + self.input.addr_consumed));

        // clear holder when output fires
        if self.output_fire() {
            self.holder_out.0 = false;
        }

        if self.out_sub_fire() {
            self.holder_out_sub.0 = false;
        }
    }

    /// read the pointed target
    fn read_target(&mut self, p: usize) {
        self.heap_mem.read_a(p);
        self.working_heap.read_a(p);
        self.demand_heap.write_a(p, true);
        self.addr_holder.connect(&p);
    }

    /// write the incoming app
    fn write_incoming(&mut self, p: HeapPort) {
        let current_stk = &mut self.thread_stack[self.input.port_a_bits.stack_idx as usize];
        let addr = current_stk.top().unwrap().1;
        let cell = HeapCell {
            exist: true,
            app: self.input.port_a_bits.load.clone(),
        };
        current_stk.pop();
        match p {
            HeapPort::A => self.heap_mem.write_a(addr, cell),
            HeapPort::B => self.heap_mem.write_b(addr, cell),
        }
    }

    /// write the incoming WHNF
    fn write_whnf(&mut self, p: HeapPort) {
        let addr = *self.addr_holder.value();
        let cell = HeapCell {
            exist: true,
            app: self.holder_in.value().load.clone(),
        };
        match p {
            HeapPort::A => self.heap_mem.write_a(addr, cell),
            HeapPort::B => self.heap_mem.write_b(addr, cell),
        }
    }

    /// put output register
    fn put_output(&mut self, stack_idx: u8, load: App) {
        // only when unable to fire in current cycle
        if !self.input.out_main_ready {
            self.holder_out = (true, ActiveApp { stack_idx, load });
        }
    }

    /// when sub port is firing, find which stack is demanding the app
    fn find_dmder_stk(&self) -> u8 {
        if let Some((stk_idx, _)) = self.thread_stack.iter().enumerate().find(|(_, s)| {
            stack_cell_with(s.top(), |(_, c)| *c == self.holder_in_sub.value().heap_addr)
        }) {
            return stk_idx as u8;
        } else {
            unreachable!()
        }
    }

    /// put output register of the sub port
    fn put_output_sub(&mut self) {
        let load = extend_to_app(&self.holder_in_sub.value().load);
        // only when unable to fire in current cycle
        if !self.input.out_sub_ready {
            self.holder_out_sub = (
                true,
                ActiveApp {
                    stack_idx: self.find_dmder_stk(),
                    load,
                },
            );
        }
    }

    /// find the stack that satisfies `p`, pop the stack and read the second item
    fn find_pop_read(&mut self, p: impl Fn(&AddrStack) -> bool, pop_frame: bool) {
        if let Some((stk_id, stack)) = self
            .thread_stack
            .iter_mut()
            .enumerate()
            .find(|(_, s)| p(*s))
        {
            stack.pop();
            if pop_frame {
                self.frame_stack[stk_id].pop();
            }
            self.heap_mem.read_a(stack.second().unwrap().1);
            self.holder_in.input.stack_idx = stk_id as u8;
        } else {
            unreachable!()
        }
    }

    /// select first arg from `app` and read it
    fn select_1st_arg_read(&mut self, app: &App) {
        let (arg_id, p) = select_1st_arg(app);
        self.read_target(p);
        self.arg_id.connect(&arg_id);
    }

    /// select next arg from `app` and read it
    fn select_next_arg_read(&mut self, app: &App) {
        let (arg_id, p) = select_next_arg(app, *self.arg_id.value());
        // println!("arg_id: {}, p: {}", arg_id, p);
        self.read_target(p);
        self.arg_id.connect(&arg_id);
    }

    /// push the target, set its working flag
    fn push_target(&mut self, new_frame: bool) {
        let current_stk = &mut self.thread_stack[self.holder_in.value().stack_idx as usize];
        self.working_heap.write_b(*self.addr_holder.value(), true);
        // println!(
        //     "pushed: {}, stack depth: {}",
        //     *self.addr_holder.value(),
        //     current_stk.elements()
        // );
        current_stk.push((new_frame, *self.addr_holder.value()));
    }

    /// ''sensitive'' cases:
    /// 1. we push an item to wait for an app, but that app is returning in this cycle
    /// 2. we ride on an idle stack, but the old waited app is returning in this cycle
    fn is_sensitive(&self, s1: &IAs1) -> bool {
        let sensitive1 = *s1 == IAs1::ExistIAWorkingNormal;
        let sensitive2 = *s1 == IAs1::ExistIAFresh || *s1 == IAs1::NoExist;
        let returning_app = self.thread_stack[self.input.port_a_bits.stack_idx as usize]
            .top()
            .unwrap()
            .1;
        let current_stk = &self.thread_stack[self.holder_in.value().stack_idx as usize];
        let same1 = returning_app == *self.addr_holder.value();
        let same2 = stack_cell_with(current_stk.top(), |(_, addr)| *addr == returning_app);
        // if self.input.port_a_valid
        //     && self.input.port_a_bits.stack_idx == self.holder_in.value().stack_idx
        // {
        //     panic!("strange shit!");
        // }
        (sensitive1 && same1) || (sensitive2 && same2)
    }

    /// take shortcuts, unless 'sensitive cases' are encountered
    fn step_to_next(&mut self, s1: &IAs1) {
        if self.is_sensitive(s1) {
            // if self.input.port_a_valid {
            //     println!("sensi! returning: {:?}", self.input.port_a_bits.load);
            // }
            self.stm.connect(&Stm::IDLE);
        } else {
            self.consume_next();
        }
    }

    fn gen_output_whnf(&self) -> App {
        let dmder = &self.heap_mem.dout_a().app;
        let target = &self.holder_in.value().load;
        let (arg_id, _) = select_1st_arg(dmder);
        deref(dmder, arg_id, target)
    }

    fn gen_active_app(&self, app: App) -> ActiveApp {
        ActiveApp {
            stack_idx: self.holder_in.value().stack_idx,
            load: app,
        }
    }

    fn gen_frame_record(&self) -> FrameRecord {
        let father_stk_id = self.holder_in.value().stack_idx as usize;
        let mut res = self.frame_stack[father_stk_id].top().unwrap().clone();
        res[father_stk_id] = self.addr_holder.input;
        res
    }

    /// if currently in a new frame & not pushing, cancel the new frame
    fn cancel_new_frame(&mut self, s1: &IAs1) {
        match s1 {
            IAs1::ExistWHNF | IAs1::ExistIAWorkingAtNewFrame => {
                let stk_idx = self.holder_in.value().stack_idx as usize;
                let stk = &self.thread_stack[stk_idx];
                if stack_cell_with(stk.top(), |(flag, _)| *flag) {
                    self.frame_stack[stk_idx].pop();
                }
            }
            _ => {}
        }
    }

    /// consumes the next task; must not use heap port b!
    fn consume_next(&mut self) {
        self.holder_in.connect(&self.input.port_a_bits.clone());
        match self.getCONSUMEs() {
            CONSUMEs::NoInput => {
                self.stm.connect(&Stm::IDLE);
                return;
            }
            CONSUMEs::InputIA => {
                let ia = &self.input.port_a_bits.load;
                self.select_1st_arg_read(&ia.clone());
                self.ia_addr.connect(
                    &self.thread_stack[self.input.port_a_bits.stack_idx as usize]
                        .top()
                        .unwrap()
                        .1,
                );
                self.father_stk.connect(&self.input.port_a_bits.stack_idx);
                self.stm.connect(&Stm::IA);
            }
            CONSUMEs::InputWHNFWithDmder => {
                let current_stk = &self.thread_stack[self.input.port_a_bits.stack_idx as usize];
                let current_top = current_stk.top().unwrap().1;
                self.find_pop_read(find_more_dmder(current_top), false);
                self.addr_holder.connect(&current_top);
                self.stm.connect(&Stm::WHNF);
            }
            CONSUMEs::InputWHNFNoDmderNewFrame => {
                let current_stk = &mut self.thread_stack[self.input.port_a_bits.stack_idx as usize];
                let current_top = current_stk.top().unwrap().1;
                current_stk.pop();
                self.heap_mem.read_a(current_stk.second().unwrap().1);
                self.addr_holder.connect(&current_top);
                self.frame_stack[self.input.port_a_bits.stack_idx as usize].pop();
                // println!(
                //     "enter2 RESUME: stk{}-{:?}-{:?}",
                //     self.input.port_a_bits.stack_idx,
                //     current_stk.second().unwrap(),
                //     self.heap_mem.ram[current_stk.second().unwrap().1].app
                // );
                self.stm.connect(&Stm::RESUME);
            }
            CONSUMEs::InputWHNFNoDmderNoFrame => {
                self.frame_stack[self.input.port_a_bits.stack_idx as usize].pop();
                self.write_incoming(HeapPort::A);
                self.stm.connect(&Stm::IDLE);
            }
        }
    }

    fn consume_next_sub(&mut self) {
        if self.port_b_fire() {
            // if self.input.port_b_bits.heap_addr == 117 {
            //     println!("117 comes!");
            // }
            let addr = self.input.port_b_bits.heap_addr;
            let app = extend_to_app(&self.input.port_b_bits.load);
            self.heap_mem.write_b(addr, HeapCell { exist: true, app });
            self.holder_in_sub.connect(&self.input.port_b_bits.clone());
            // when the same heap cell is read in the same cycle, leave it to port_a
            if self.demand_heap.input.port_a.addr == addr {
                self.demand_heap.read_b(0); // demand flag of `main` is always false
            } else {
                self.demand_heap.read_b(addr);
            }
            self.stm_sub.connect(&StmSub::WORK);
        } else {
            self.stm_sub.connect(&StmSub::IDLE);
        }
    }

    fn step_whnf(&mut self) {
        let deref_res = self.gen_output_whnf();
        let whnf_addr = *self.addr_holder.value();
        self.put_output(self.holder_in.value().stack_idx, deref_res);
        match self.getWHNFs() {
            WHNFs::MoreDmders => {
                self.find_pop_read(find_more_dmder(whnf_addr), false);
                self.stm.connect(&Stm::WHNF);
            }
            WHNFs::NewFrame => {
                self.write_whnf(HeapPort::B);
                self.find_pop_read(find_new_frame(whnf_addr), true);
                self.stm.connect(&Stm::RESUME);
            }
            WHNFs::NoNewFrame => {
                if let Some((stk_id, stack)) = self
                    .thread_stack
                    .iter_mut()
                    .enumerate()
                    .find(|(_, s)| stack_cell_with(s.top(), |(_, addr)| *addr == whnf_addr))
                {
                    stack.pop();
                    self.frame_stack[stk_id].pop();
                }
                self.write_whnf(HeapPort::B);
                self.consume_next();
            }
        }
    }

    fn step_ia(&mut self) {
        let dmder = &self.holder_in.value().load;
        let mut updated_dmder = dmder.clone();
        let target = self.heap_mem.dout_a().app.clone();
        let ias1 = self.getIAs1();
        // if *self.addr_holder.value() == 117 {
        //     println!(
        //         "117: exist: {} working: {}, {:?},{:?}",
        //         self.heap_mem.dout_a().exist,
        //         *self.working_heap.dout_a(),
        //         self.heap_mem.dout_a().app,
        //         ias1
        //     );
        // }
        match ias1 {
            IAs1::NoExist => {
                self.push_target(false);
            }
            IAs1::ExistWHNF => {
                updated_dmder = deref(dmder, *self.arg_id.value(), &target);
                self.holder_in.input.load = updated_dmder.clone();
            }
            IAs1::ExistIAWorkingNormal => {
                // change this to `self.push_target(false);` will turn off stack riding
                self.push_target(true);
            }
            IAs1::ExistIAWorkingAtNewFrame => { /* do nothing here */ }
            IAs1::ExistIAFresh => {
                self.push_target(false);
                self.put_output(self.holder_in.value().stack_idx, target.clone());
            }
        }

        match self.getIAs2(&ias1) {
            IAs2::NextStrictArgNewStk => {
                self.select_next_arg_read(&updated_dmder);
                let frame_record = self.frame_stack[self.holder_in.value().stack_idx as usize]
                    .top()
                    .unwrap();
                if let Some((stk_id, _)) = self
                    .thread_stack
                    .iter()
                    .enumerate()
                    .find(|(idx, s)| find_free_stack(s, frame_record[*idx]))
                {
                    // FIXME: ensure using a new stack
                    if stk_id as u8 == self.holder_in.value().stack_idx {
                        panic!("GOT YA!");
                    }
                    self.holder_in.input.stack_idx = stk_id as u8;
                    self.frame_stack[stk_id].push(self.gen_frame_record());
                    // println!("{} pushed frame", stk_id);
                };
            }
            IAs2::NextStrictArgLocal => {
                self.select_next_arg_read(&updated_dmder);
            }
            IAs2::NoMoreArgsNoEmit => {
                // if *self.ia_addr.value() == 242 {
                //     println!("updating ram[242]: {:?}", updated_dmder);
                // }
                self.cancel_new_frame(&ias1);
                self.heap_mem.write_b(
                    *self.ia_addr.value(),
                    HeapCell {
                        exist: true,
                        app: updated_dmder,
                    },
                );
                self.step_to_next(&ias1);
                // if self.heap_mem.input.port_b.addr == 242 {
                //     println!("writing 242!");
                // }
            }
            IAs2::NoMoreArgsCanEmit => {
                self.cancel_new_frame(&ias1);
                self.put_output(self.holder_in.value().stack_idx, updated_dmder);
                self.step_to_next(&ias1);
            }
        }
    }

    fn step_resume(&mut self) {
        // println!(
        //     "RESUME!, stk{} top: {:?}-{:?}",
        //     self.holder_in.value().stack_idx,
        //     self.thread_stack[self.holder_in.value().stack_idx as usize]
        //         .top()
        //         .unwrap(),
        //     self.heap_mem.ram[self.thread_stack[self.holder_in.value().stack_idx as usize]
        //         .top()
        //         .unwrap()
        //         .1]
        //         .app
        // );

        self.write_whnf(HeapPort::B);
        match self.getRESUMEs() {
            RESUMEs::TopInWHNF => {
                let current_stk = &mut self.thread_stack[self.holder_in.value().stack_idx as usize];
                let whnf_addr = current_stk.top().unwrap().1;
                let dmder_addr = current_stk.second().unwrap().1;
                self.holder_in.input.load = self.heap_mem.dout_a().app.clone();
                current_stk.pop();
                self.heap_mem.read_a(dmder_addr);
                self.addr_holder.connect(&whnf_addr);
                self.stm.connect(&Stm::WHNF);
            }
            RESUMEs::TopInIA => {
                self.consume_next();
            }
        }
    }

    fn step_work(&mut self) {
        match self.getWORKs() {
            WORKs::NotDemanded => {
                self.consume_next_sub();
            }
            WORKs::DmderFound => {
                self.put_output_sub();
                self.consume_next_sub();
            }
            WORKs::DmderNotFound => {
                self.demand_heap
                    .read_b(self.holder_in_sub.value().heap_addr);
            }
        }
    }

    fn handle_port_a(&mut self) {
        // halt the machine if output is not consumed yet
        // TODO: to maintain the reading signals on memories
        if self.holder_out.0 && !self.input.out_main_ready {
            return;
        }

        match *self.stm.value() {
            Stm::IDLE => self.consume_next(),
            Stm::WHNF => self.step_whnf(),
            Stm::IA => self.step_ia(),
            Stm::RESUME => self.step_resume(),
        }
    }

    fn handle_port_b(&mut self) {
        if self.holder_out_sub.0 && !self.input.out_sub_ready {
            return;
        }

        // if self.input.port_b_valid {
        //     println!(
        //         "before:b-in: {}-{:?}",
        //         self.input.port_b_bits.heap_addr, self.input.port_b_bits.load
        //     );
        //     println!(
        //         "before:b-write: {}-{:?}",
        //         self.heap_mem.input.port_b.addr, self.heap_mem.input.port_b.din.app
        //     )
        // }

        match *self.stm_sub.value() {
            StmSub::IDLE => self.consume_next_sub(),
            StmSub::WORK => self.step_work(),
        }

        // if self.input.port_b_valid {
        //     println!(
        //         "after:b-in: {}-{:?}",
        //         self.input.port_b_bits.heap_addr, self.input.port_b_bits.load
        //     );
        //     println!(
        //         "after:b-write: {}-{:?}",
        //         self.heap_mem.input.port_b.addr, self.heap_mem.input.port_b.din.app
        //     )
        // }
    }
}

impl HwModule for DrfHeap {
    fn update_local(&mut self) {
        self.update_prepare();

        // start the machine (demand flag of `main`, at addr 0, need to be false.)
        if !self.working.value() {
            if self.input.start {
                self.working.connect(&true);
                // push to stack
                self.thread_stack[0].push((false, 0));
                self.frame_stack[0].push(Default::default());
                // put output register
                self.holder_out = (
                    true,
                    ActiveApp {
                        stack_idx: 0,
                        load: self.heap_mem.dout_a().app.clone(),
                    },
                );
            }
            return;
        }
        // stop the machine when finished
        if self.port_a_fire() {
            // if its main in WHNF
            if self.thread_stack[0].elements() == 1
                && self.input.port_a_bits.stack_idx == 0
                && is_whnf(&self.input.port_a_bits.load)
            {
                self.working.connect(&false);
                return;
            }
        }

        self.handle_port_a();
        self.handle_port_b();
    }

    fn update_stat(&mut self) {
        if self.holder_out.0 {
            self.stat
                .holder_contents
                .push(Some(self.holder_out.1.clone()));
        } else if self.output_fire() {
            self.stat.holder_contents.push(Some(self.out_main_bits()));
        } else {
            self.stat.holder_contents.push(None);
        }

        self.stat.heap_stm.push(self.stm.value().clone());
        self.stat.serving_id.push(self.holder_in.value().stack_idx);

        let occupied = self
            .thread_stack
            .iter()
            .filter(|stk| stk.elements() != 0)
            .count();
        if self.port_a_fire() {
            self.stat.active_threads -= 1;
        }
        if self.output_fire() {
            self.stat.active_threads += 1;
        }
        if self.out_sub_fire() {
            self.stat.active_threads += 1;
        }
        self.stat
            .work_threads
            .push((occupied as u8, self.stat.active_threads));

        match *self.stm.value() {
            Stm::IDLE => self.stat.stm_cycles[0] += 1,
            Stm::WHNF => self.stat.stm_cycles[1] += 1,
            Stm::IA => self.stat.stm_cycles[2] += 1,
            Stm::RESUME => self.stat.stm_cycles[3] += 1,
        }
    }

    fn tick_children(&mut self) {
        // println!(
        //     "ram[262]: {}-{:?}",
        //     self.heap_mem.ram[262].exist,
        //     self.heap_mem.ram[262].app,
        //     // self.heap_mem.ram[167].exist,
        //     // self.heap_mem.ram[167].app
        // );
        // if self.heap_mem.input.port_a.is_write
        //     && self.heap_mem.input.port_b.is_write
        //     && self.heap_mem.input.port_a.addr == self.heap_mem.input.port_b.addr
        // {
        //     println!(
        //         "strange write on {}, stm {}, port_b ready {}, ias1 {:?}, ias2 {:?}, port_a ready {}, consumes {:?}",
        //         self.heap_mem.input.port_a.addr,
        //         self.stm.value(),
        //         self.port_b_ready(),
        //         self.getIAs1(),
        //         self.getIAs2(&self.getIAs1()),
        //         self.port_a_ready(),
        //         self.getCONSUMEs()
        //     );
        //     println!(
        //         "port a stack id {}, current stack id {}",
        //         self.input.port_a_bits.stack_idx,
        //         self.holder_in.value().stack_idx
        //     );
        //     println!("port a write: {:?}", self.heap_mem.input.port_a.din.app);
        //     println!("port b write: {:?}", self.heap_mem.input.port_b.din.app);
        // }
        // println!(
        //     "sub-stm:{:?}, condition:{:?}, addr:{}",
        //     self.stm_sub.value(),
        //     self.getWORKs(),
        //     self.holder_in_sub.value().heap_addr
        // );
        // if self.out_sub_fire() {
        //     println!("sub out fire: {:?}", self.out_sub_bits());
        // }

        self.stm.tick();
        self.stm_sub.tick();
        for stk in &mut self.thread_stack {
            stk.tick();
        }
        for stk in &mut self.frame_stack {
            stk.tick();
        }
        self.heap_mem.tick();
        self.demand_heap.tick();
        self.working_heap.tick();
        self.working.tick();
        self.holder_in.tick();
        self.holder_in_sub.tick();
        self.addr_bumper.tick();
        self.arg_id.tick();
        self.father_stk.tick();
        self.addr_holder.tick();
        self.ia_addr.tick();
    }
}
