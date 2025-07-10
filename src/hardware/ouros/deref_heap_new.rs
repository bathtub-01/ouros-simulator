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

type StackCell = (bool, usize);
type AddrStack = Stack<StackCell, 128>;

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

fn find_free_stack(s: &AddrStack) -> bool {
    s.elements() == 0 || stack_cell_with(s.top(), |(flag, _)| *flag)
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

/// branch conditions for `consume_next()`
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
#[derive(PartialEq)]
enum IAs1 {
    NoExist,
    ExistWHNF,
    ExistIAWorkingNormal,
    ExistIAWorkingAtNewFrame,
    ExistIAFresh,
}

/// branch conditions for the `IA` state, part 2
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

/// select the dereference pointer, returns (arg position, pointer value)
fn select_arg(app: &App) -> (usize, usize) {
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
        INT(2),
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
    holder_in: Register<ActiveApp>,
    addr_holder: usize,
    addr_holder_sub: usize,
    thread_stack: [AddrStack; 8],
    heap_mem: DualPortMem<HeapCell>,
    demand_heap: DualPortMem<bool>,
    working_heap: DualPortMem<bool>,
    holder_out: (bool, ActiveApp), // output-reg: (valid, app)
    holder_out_sub: Register<(bool, App)>,
    working: Register<bool>, // track whether the machine is working
    same_addr: Register<bool>,
    addr_bumper: Register<usize>,
    arg_id: usize,
    // stat: DrfHeapStat,
}

impl DrfHeap {
    pub fn new(heap_size: usize) -> Self {
        Self {
            input: Default::default(),
            stm: Default::default(),
            holder_in: Default::default(),
            addr_holder: Default::default(),
            addr_holder_sub: Default::default(),
            thread_stack: std::array::from_fn(|_| Stack::new()),
            heap_mem: DualPortMem::new(heap_size),
            working_heap: DualPortMem::new(heap_size),
            demand_heap: DualPortMem::new(heap_size),
            holder_out: Default::default(),
            working: Default::default(),
            addr_bumper: Default::default(),
            // stat: Default::default(),
            holder_out_sub: Default::default(),
            same_addr: Default::default(),
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
            .any(find_more_dmder(self.addr_holder))
        {
            WHNFs::MoreDmders
        } else {
            if self
                .thread_stack
                .iter()
                .any(find_new_frame(self.addr_holder))
            {
                WHNFs::NewFrame
            } else {
                WHNFs::NoNewFrame
            }
        }
    }

    fn getIAs1(&self) -> IAs1 {
        let target = &self.heap_mem.dout_a().app;
        let stk = &self.thread_stack[self.holder_in.value().stack_idx as usize];
        if !self.heap_mem.dout_a().exist {
            IAs1::NoExist
        } else {
            if is_whnf(target) {
                IAs1::ExistWHNF
            } else {
                if !*self.working_heap.dout_a() {
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
        let idle_stack: bool = self.thread_stack.iter().any(|s| find_free_stack(s));
        let local_stack: bool = *s1 == IAs1::ExistWHNF || *s1 == IAs1::ExistIAWorkingAtNewFrame;
        let more_strict_args: bool = {
            match ia[0] {
                Atom::PTR(_) => false,
                Atom::PRM(_, _) => self.arg_id == 1 && !is_int(&ia[2]),
                // more on this to support strict args in the future
                _ => unreachable!(),
            }
        };

        if more_strict_args && local_stack {
            IAs2::NextStrictArgLocal
        } else if more_strict_args && idle_stack {
            IAs2::NextStrictArgNewStk
        } else {
            if (is_ptr(&ia[0]) || is_prm(&ia[0]) && is_int(&ia[1])) && target_in_whnf {
                IAs2::NoMoreArgsCanEmit
            } else {
                IAs2::NoMoreArgsNoEmit
            }
        }
    }

    fn getRESUMEs(&self) -> RESUMEs {
        use RESUMEs::*;
        let top = self.heap_mem.dout_a().app.clone();
        if is_whnf(&top) {
            TopInWHNF
        } else {
            TopInIA
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

        // rise addr bumper
        self.addr_bumper
            .connect(&(self.addr_bumper.value() + self.input.addr_consumed));

        // clear holder when output fires
        if self.output_fire() {
            self.holder_out.0 = false;
        }

        if self.out_sub_fire() {
            self.holder_out_sub.input.0 = false;
        }

        self.same_addr.connect(&false);

        self.addr_holder_sub = self.input.port_b_bits.heap_addr;
    }

    /// read the pointed target
    fn read_target(&mut self, p: usize) {
        self.heap_mem.read_a(p);
        self.working_heap.read_a(p);
        self.demand_heap.write_a(p, true);
        self.addr_holder = p;

        // if the requested app is also entering at the same cycle
        if self.port_b_fire() && self.input.port_b_bits.heap_addr == p {
            self.same_addr.connect(&true);
        }
    }

    /// write the incoming IA
    fn write_ia(&mut self, p: HeapPort) {
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

    fn write_whnf(&mut self, p: HeapPort) {
        let addr = self.addr_holder;
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
        self.holder_out = (true, ActiveApp { stack_idx, load });
    }

    /// find the stack that satisfies `p`, pop the stack and read the second item
    fn find_pop_read(&mut self, p: impl Fn(&AddrStack) -> bool) {
        if let Some((stk_id, stack)) = self
            .thread_stack
            .iter_mut()
            .enumerate()
            .find(|(_, s)| p(*s))
        {
            stack.pop();
            self.heap_mem.read_a(stack.second().unwrap().1);
            self.holder_in.input.stack_idx = stk_id as u8;
        } else {
            unreachable!()
        }
    }

    /// select next arg from `app` and read it
    fn select_arg_read(&mut self, app: &App) {
        let (arg_id, p) = select_arg(app);
        self.read_target(p);
        self.arg_id = arg_id;
    }

    /// push the target, set its working flag
    fn push_target(&mut self, new_frame: bool) {
        let current_stk = &mut self.thread_stack[self.holder_in.value().stack_idx as usize];
        self.working_heap.write_b(self.addr_holder, true);
        current_stk.push((new_frame, self.addr_holder));
    }

    /// take shortcuts, unless 'sensitive cases' are encountered
    fn step_to_next(&mut self, s1: &IAs1) {
        let a_sensitive = *s1 == IAs1::ExistIAWorkingNormal;
        let b_sensitive = *s1 == IAs1::NoExist;
        let a_same = self.input.port_a_valid
            && self.thread_stack[self.input.port_a_bits.stack_idx as usize]
                .top()
                .unwrap()
                .1
                == self.addr_holder;
        let b_same =
            self.input.port_b_valid && self.input.port_b_bits.heap_addr == self.addr_holder;
        if (a_sensitive && a_same) || (b_sensitive && b_same) {
            self.stm.connect(&Stm::IDLE);
        } else {
            self.consume_next();
        }
    }

    fn consume_next(&mut self) {
        self.holder_in.connect(&self.input.port_a_bits.clone());
        match self.getCONSUMEs() {
            CONSUMEs::NoInput => {
                self.stm.connect(&Stm::IDLE);
            }
            CONSUMEs::InputIA => {
                let ia = &self.input.port_a_bits.load;
                self.select_arg_read(&ia.clone());
                self.stm.connect(&Stm::IA);
            }
            CONSUMEs::InputWHNFWithDmder => {
                let current_stk = &self.thread_stack[self.input.port_a_bits.stack_idx as usize];
                let current_top = current_stk.top().unwrap().1;
                self.find_pop_read(find_more_dmder(current_top));
                self.addr_holder = current_top;
                self.stm.connect(&Stm::WHNF);
            }
            CONSUMEs::InputWHNFNoDmderNewFrame => {
                let current_stk = &mut self.thread_stack[self.input.port_a_bits.stack_idx as usize];
                self.heap_mem.read_a(current_stk.second().unwrap().1);
                self.write_ia(HeapPort::B);
                self.stm.connect(&Stm::RESUME);
            }
            CONSUMEs::InputWHNFNoDmderNoFrame => {
                self.write_ia(HeapPort::A);
                self.stm.connect(&Stm::IDLE);
            }
        }
    }

    fn step_whnf(&mut self) {
        let dmder = &self.heap_mem.dout_a().app;
        let target = &self.holder_in.value().load;
        let (arg_id, _) = select_arg(dmder);
        let deref_res = deref(dmder, arg_id, target);
        let whnf_addr = self.addr_holder;
        self.put_output(self.holder_in.value().stack_idx, deref_res);
        match self.getWHNFs() {
            WHNFs::MoreDmders => {
                self.find_pop_read(find_more_dmder(whnf_addr));
                self.stm.connect(&Stm::WHNF);
            }
            WHNFs::NewFrame => {
                self.write_whnf(HeapPort::B);
                self.find_pop_read(find_new_frame(whnf_addr));
                self.stm.connect(&Stm::RESUME);
            }
            WHNFs::NoNewFrame => {
                self.write_whnf(HeapPort::B);
                self.consume_next();
            }
        }
    }

    fn step_ia(&mut self) {
        let dmder = &self.holder_in.value().load;
        let mut updated_dmder = dmder.clone();
        let target = self.heap_mem.dout_a().app.clone();
        let ia_addr = self.thread_stack[self.input.port_a_bits.stack_idx as usize]
            .top()
            .unwrap()
            .1;
        let ias1 = self.getIAs1();
        match ias1 {
            IAs1::NoExist => {
                self.push_target(false);
            }
            IAs1::ExistWHNF => {
                updated_dmder = deref(dmder, self.arg_id, &target);
                self.holder_in.input.load = updated_dmder.clone();
            }
            IAs1::ExistIAWorkingNormal => {
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
                if let Some((stk_id, _)) = self
                    .thread_stack
                    .iter()
                    .enumerate()
                    .find(|(_, s)| find_free_stack(s))
                {
                    self.holder_in.input.stack_idx = stk_id as u8;
                };
                self.select_arg_read(&updated_dmder);
            }
            IAs2::NextStrictArgLocal => {
                self.select_arg_read(&updated_dmder);
            }
            IAs2::NoMoreArgsNoEmit => {
                self.heap_mem.write_b(
                    ia_addr,
                    HeapCell {
                        exist: true,
                        app: updated_dmder,
                    },
                );
                self.step_to_next(&ias1);
            }
            IAs2::NoMoreArgsCanEmit => {
                self.put_output(self.holder_in.value().stack_idx, updated_dmder);
                self.step_to_next(&ias1);
            }
        }
    }

    fn step_resume(&mut self) {
        match self.getRESUMEs() {
            RESUMEs::TopInWHNF => {
                let current_stk = &mut self.thread_stack[self.input.port_a_bits.stack_idx as usize];
                let current_top = current_stk.top().unwrap().1;
                self.holder_in.input.load = self.heap_mem.dout_a().app.clone();
                current_stk.pop();
                self.heap_mem.read_a(current_top);
                self.addr_holder = current_top;
                self.stm.connect(&Stm::WHNF);
            }
            RESUMEs::TopInIA => {
                self.consume_next();
            }
        }
    }

    fn handle_port_a(&mut self) {
        // halt the machine if output is not consumed yet
        if self.holder_out.0 {
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
        if !self.out_sub_fire() && !self.same_addr.value() {
            self.holder_out_sub.input.0 = *self.demand_heap.dout_b();
        }

        // holder_out_sub can be used now
        if fire(self.input.port_b_valid, self.port_b_ready()) {
            let addr = self.input.port_b_bits.heap_addr;
            let app = {
                let mut extended: App = std::array::from_fn(|_| Atom::NOP);
                for (i, a) in self.input.port_b_bits.load.iter().enumerate() {
                    extended[i] = a.clone();
                }
                extended
            };

            self.holder_out_sub.input.1 = app.clone();

            self.demand_heap.read_b(addr);
            if self.heap_mem.input.port_b.is_write {
                panic!("port_b: competition!");
            }
            self.heap_mem.write_b(addr, HeapCell { exist: true, app });
        }
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
                self.thread_stack[0].push(0);
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

    fn tick_children(&mut self) {
        todo!()
    }
}
