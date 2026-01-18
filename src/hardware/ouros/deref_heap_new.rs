// Dereference Heap, also handles thread management:
//           +-----------------+
// addr  <-->|    Dereference  |===> out_main
// port_a ==>|                 |
// port_b ==>|       Heap      |===> out_sub
//           +-----------------+

use super::config::*;
use super::program::*;
use crate::hardware::common::memory::DualPortMemStat;
use crate::hardware::common::{DualPortMem, Register, Stack};
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
    pub free_addr: usize,
    pub free_addr_valid: bool,
    pub found: bool,
}

impl HwInput for DrfHeapInput {}

type StackCell = (bool, usize);
type AddrStack = Stack<StackCell, 512>;
type FrameRecord = [usize; MAX_THREADS];
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

/// cancel all the unique PTRs in an app
fn dash_app(app: &App) -> App {
    let mut res = app.clone();
    for atom in res.iter_mut() {
        if let Atom::PTR(p, _, false) = atom {
            *atom = Atom::PTR(*p, false, false);
        }
    }
    res
}

/// setup the evaluated flag in seq if the 1st arg is a literal
fn mask_seq(app: &App) -> App {
    match app[0] {
        Atom::SEQ(false) => {
            if is_lit_atom(&app[1]) {
                let mut res: App = app.clone();
                res[0] = Atom::SEQ(true);
                res
            } else {
                app.clone()
            }
        }
        _ => app.clone(),
    }
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

#[derive(Default)]
pub struct DrfHeapStat {
    active_threads: u8,
    pub work_threads: Vec<(u8, u8)>, // (occupied resources, active threads)
    pub holder_contents: Vec<Option<ActiveApp>>,
    pub heap_stm: Vec<Stm>,
    pub serving_id: Vec<u8>,
    pub stm_cycles: [u32; 4],
    pub heap_update: u32,
    pub update_avoided: u32,
}

/// branch conditions for `consume_next()`
#[derive(Debug, PartialEq)]
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

/// select the first pointer to deref, returns (arg position, pointer value)
fn select_1st_arg(app: &App) -> (usize, usize) {
    match app[0] {
        Atom::PTR(p, _, false) => (0, p),
        Atom::PRM(_, _) | Atom::SEQ(false) => match app[1] {
            Atom::PTR(p, _, false) => (1, p),
            Atom::NOP => unreachable!(),
            _ => match app[2] {
                Atom::PTR(p, _, false) => (2, p),
                _ => unreachable!("app: {:?}", app),
            },
        },
        Atom::TRY => match app[1] {
            Atom::PTR(p, _, false) => (1, p),
            _ => panic!("TRY literal should not enter the heap"),
        },
        Atom::SEQ(true) => match app[2] {
            Atom::PTR(p, _, false) => (2, p),
            // NOTE: currently rejecting things like `seq a 1` (direct it to reducer will be easier)
            _ => {
                println!("app: {:?}", app);
                unreachable!()
            }
        },
        _ => {
            println!("app: {:?}", app);
            unreachable!()
        }
    }
}

/// select the next strict arg, returns (arg position, pointer value)
fn select_next_arg(app: &App, current: usize) -> (usize, usize) {
    match app[2] {
        Atom::PTR(p, _, false) => (2, p),
        _ => unreachable!(),
    }
}

fn vec_to_app(v: Vec<Atom>) -> App {
    assert!(v.len() <= APP_LENGTH);
    let mut res: App = std::array::from_fn(|_| Atom::NOP);
    v.iter().enumerate().for_each(|(i, a)| res[i] = a.clone());
    res
}

/// Dereference `app`'s PTR at position `arg_id`, with `target`
/// - when returning `(app, None)`, `app` is the deref result
/// - when returning `(app1, Some(app2))`,
///   `app1` is the deref result,
///   `app2` goes to port b
fn deref(app: &App, arg_id: usize, target: &App, free_addr: usize) -> (App, Option<App>) {
    assert!(is_ptr(&app[arg_id]));
    let unique: bool = match app[arg_id] {
        Atom::PTR(_, true, _) => true,
        _ => false,
    };
    let target_dashed = if unique { target } else { &dash_app(target) };
    let app_len = app_length(app);
    let target_len = app_length(target);
    let mut res_v: Vec<Atom> = Vec::new();

    match app[0] {
        Atom::SEQ(false) => {
            if arg_id == 1 {
                let mut res = app.clone();
                res[0] = Atom::SEQ(true);
                return (res, None);
            } else {
                /* do nothing when arg_id == 2 */
                return (app.clone(), None);
            }
        }
        Atom::SEQ(true) => {
            assert_eq!(arg_id, 2);
            res_v.extend_from_slice(&target_dashed[0..target_len]);
            res_v.extend_from_slice(&app[3..app_len]);
        }
        Atom::TRY => {
            assert_eq!(arg_id, 1);
            res_v.extend_from_slice(&target_dashed[0..target_len]);
            res_v.extend_from_slice(&app[3..app_len]);
        }
        _ => {
            res_v.extend_from_slice(&app[0..arg_id]);
            res_v.extend_from_slice(&target_dashed[0..target_len]);
            res_v.extend_from_slice(&app[arg_id + 1..app_len]);
        }
    }

    if res_v.len() <= APP_LENGTH {
        (vec_to_app(res_v), None)
    } else {
        let mut wb_app = res_v[APP_LENGTH..res_v.len()].to_vec();
        wb_app.insert(0, Atom::PTR(free_addr, unique, false));
        (
            vec_to_app(wb_app),
            Some(vec_to_app(res_v[0..APP_LENGTH].to_vec())),
        )
    }
}

#[test]
fn deref_spec() {
    use Atom::*;
    let app: App = [
        PTR(0, true, false),
        INT(1),
        PTR(2, true, false),
        INT(3),
        NOP,
        NOP,
        NOP,
        NOP,
    ];
    let target1: App = [
        PTR(11, true, false),
        PTR(22, true, false),
        PTR(33, true, false),
        PTR(44, true, false),
        PTR(55, true, false),
        NOP,
        NOP,
        NOP,
    ];
    let target2: App = [
        PTR(11, true, false),
        PTR(22, true, false),
        PTR(33, true, false),
        PTR(44, true, false),
        PTR(55, true, false),
        PTR(66, true, false),
        PTR(77, true, false),
        NOP,
    ];
    let res1: App = [
        PTR(0, true, false),
        INT(1),
        PTR(11, true, false),
        PTR(22, true, false),
        PTR(33, true, false),
        PTR(44, true, false),
        PTR(55, true, false),
        INT(3),
    ];
    let res2_1: App = [
        PTR(11, true, false),
        PTR(22, true, false),
        PTR(33, true, false),
        PTR(44, true, false),
        PTR(55, true, false),
        PTR(66, true, false),
        PTR(77, true, false),
        INT(1),
    ];
    let res2_2: App = [
        PTR(42, true, false),
        PTR(2, true, false),
        INT(3),
        NOP,
        NOP,
        NOP,
        NOP,
        NOP,
    ];
    assert_eq!(deref(&app, 2, &target1, 42), (res1, None));
    assert_eq!(deref(&app, 0, &target2, 42), (res2_2, Some(res2_1)));
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

pub struct DrfHeap {
    pub input: DrfHeapInput,
    stm: Register<Stm>,
    holder_in: Register<ActiveApp>,
    addr_holder: Register<usize>,
    ia_addr: Register<usize>,
    thread_stack: [AddrStack; MAX_THREADS],
    frame_stack: [FrameStack; MAX_THREADS],
    pub heap_mem: DualPortMem<App>,
    working_heap: DualPortMem<bool>,
    holder_out: (bool, ActiveApp), // output-reg: (valid, app)
    working: Register<bool>,       // track whether the machine is working
    arg_id: Register<usize>,
    non_exist: Register<bool>,
    stat: DrfHeapStat,
    stat_detail_lv: u8,
}

impl DrfHeap {
    pub fn new(heap_size: usize) -> Self {
        Self {
            input: Default::default(),
            stm: Default::default(),
            holder_in: Default::default(),
            addr_holder: Default::default(),
            ia_addr: Default::default(),
            thread_stack: std::array::from_fn(|_| Stack::new()),
            frame_stack: std::array::from_fn(|_| Stack::new()),
            heap_mem: DualPortMem::new(heap_size),
            working_heap: DualPortMem::new(heap_size),
            holder_out: Default::default(),
            working: Default::default(),
            stat: Default::default(),
            arg_id: Default::default(),
            non_exist: Default::default(),
            stat_detail_lv: Default::default(),
        }
    }

    /// When creating a `DrfHeap`, put a compiled program into the heap memory.
    pub fn program(mut self, prog: &Vec<Vec<Atom>>) -> Self {
        fn convert(atms: &Vec<Atom>) -> App {
            assert!(atms.len() <= APP_LENGTH);
            let mut app: App = std::array::from_fn(|_| Atom::NOP);
            for (i, atm) in atms.iter().enumerate() {
                app[i] = atm.clone();
            }
            app
        }
        // convert Vec<Vec<Atom>> to Vec<HeapCell>
        let img: Vec<App> = prog.iter().map(convert).collect();
        self.heap_mem.image(&img);
        self
    }

    /// Setup the detail level of stats, 0~lowest, 255~highest
    pub fn detail(mut self, lv: u8) -> Self {
        self.stat_detail_lv = lv;
        self.heap_mem.record_stat = lv >= DLV_MEM_USAGE;
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
        let local_release: bool = match *self.stm.value() {
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
        local_release && self.input.free_addr_valid
    }

    pub fn port_b_ready(&self) -> bool {
        let borrowed: bool = match *self.stm.value() {
            Stm::IDLE => false,
            Stm::WHNF => match self.getWHNFs() {
                WHNFs::MoreDmders | WHNFs::NewFrame => false,
                WHNFs::NoNewFrame => !self.can_avoid_update(),
            },
            Stm::IA => {
                let ias1 = self.getIAs1();
                match self.getIAs2(&ias1) {
                    IAs2::NoMoreArgsNoEmit => true,
                    // IAs2::NoMoreArgsCanEmit => ias1 == IAs1::ExistWHNF && self.need_split(),
                    _ => false,
                }
            }
            Stm::RESUME => true,
        };
        !borrowed
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
        let target = self.heap_mem.dout_a();
        if self.holder_out.0 {
            return self.holder_out.1.clone();
        }

        match *self.stm.value() {
            Stm::IDLE => Default::default(),
            Stm::WHNF => {
                let (deref_res, _) = self.gen_output_whnf();
                self.gen_active_app(deref_res)
            }
            Stm::IA => {
                let ias1 = self.getIAs1();
                if ias1 == IAs1::ExistIAFresh {
                    return self.gen_active_app(self.dash_when_shared());
                }
                match self.getIAs2(&ias1) {
                    IAs2::NoMoreArgsCanEmit => {
                        let (updated_dmder, _) = deref(
                            &self.holder_in.value().load,
                            *self.arg_id.value(),
                            target,
                            self.input.free_addr,
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
        self.port_b_fire() && self.waited_by().is_some()
    }

    pub fn out_sub_bits(&self) -> ActiveApp {
        let load = extend_to_app(&self.input.port_b_bits.load);
        ActiveApp {
            stack_idx: self.find_dmder_stk(),
            load: dash_app(&load),
        }
    }

    pub fn out_big_drf_valid(&self) -> bool {
        self.need_split()
    }

    pub fn out_big_drg_bits(&self) -> FrozenApp {
        let mk_frozen = |a: Option<App>| match a {
            Some(big) => FrozenApp {
                heap_addr: self.input.free_addr,
                load: big,
            },
            None => Default::default(),
        };
        match *self.stm.value() {
            Stm::WHNF => mk_frozen(self.gen_output_whnf().1),
            Stm::IA if self.getIAs1() == IAs1::ExistWHNF => {
                let dmder = &self.holder_in.value().load;
                let target = self.heap_mem.dout_a();
                let (_, obig) = deref(dmder, *self.arg_id.value(), target, self.input.free_addr);
                mk_frozen(obig)
            }
            _ => Default::default(),
        }
    }

    /// machine's work has been finished
    pub fn done(&self) -> bool {
        !self.working.value()
    }

    pub fn search(&self) -> usize {
        if self.port_a_fire() && self.getCONSUMEs() == CONSUMEs::InputIA {
            let in_app = mask_seq(&self.input.port_a_bits.load);
            let (_, p) = select_1st_arg(&in_app);
            p
        } else if *self.stm.value() == Stm::IA {
            match self.getIAs2(&self.getIAs1()) {
                IAs2::NextStrictArgLocal | IAs2::NextStrictArgNewStk => {
                    let dmder = &self.holder_in.value().load;
                    let (_, p) = select_next_arg(dmder, 0);
                    p
                }
                _ => 0,
            }
        } else {
            0
        }
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
        let target = self.heap_mem.dout_a();
        let stk = &self.thread_stack[self.holder_in.value().stack_idx as usize];
        if *self.non_exist.value() {
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
        let target_in_whnf = !*self.non_exist.value() && is_whnf(self.heap_mem.dout_a());
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
                Atom::PTR(_, _, _) => false,
                Atom::PRM(_, _) | Atom::SEQ(_) => *self.arg_id.value() == 1 && is_ptr(&ia[2]),
                Atom::TRY => *self.arg_id.value() == 1 && *s1 != IAs1::ExistWHNF,
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
                || (is_prm(&ia[0]) && (is_int(&ia[1]) || is_int(&ia[2])))
                || is_seq_evaluated(&ia[0])
                || (is_try(&ia[0]) && *self.arg_id.value() == 1))
                && target_in_whnf
            {
                IAs2::NoMoreArgsCanEmit
            } else {
                IAs2::NoMoreArgsNoEmit // TRY second should go here; what if the 2nd is in compute? We can use meta data to improve PRM/SEQ/TRY?
            }
        }
    }

    fn getRESUMEs(&self) -> RESUMEs {
        let top = self.heap_mem.dout_a();
        if is_whnf(top) {
            RESUMEs::TopInWHNF
        } else {
            RESUMEs::TopInIA
        }
    }

    /// check whether the app on port b is being waited by any thread
    fn waited_by(&self) -> Option<usize> {
        let addr = self.input.port_b_bits.heap_addr;
        let a_thread = self.holder_in.value().stack_idx as usize;
        if *self.stm.value() == Stm::IA
            && self.getIAs1() == IAs1::NoExist
            && *self.addr_holder.value() == addr
        {
            Some(a_thread)
        } else {
            self.thread_stack
                .iter()
                .position(|s| stack_cell_with(s.top(), |(_, a)| *a == addr))
        }
    }

    /// preparations in each `update_local`
    fn update_prepare(&mut self) {
        // always give default inputs at the beginning of a cycle
        self.heap_mem.input.default_input();
        self.working_heap.input.default_input();
        for stk in &mut self.thread_stack {
            stk.input.default_input();
        }
        for stk in &mut self.frame_stack {
            stk.input.default_input();
        }

        // clear holder when output fires
        if self.output_fire() {
            self.holder_out.0 = false;
        }
    }

    /// read the pointed target
    fn read_target(&mut self, p: usize) {
        self.heap_mem.read_a(p);
        self.working_heap.read_a(p);
        self.addr_holder.connect(&p);
    }

    /// write the incoming app
    fn write_incoming(&mut self, p: HeapPort) {
        let current_stk = &mut self.thread_stack[self.input.port_a_bits.stack_idx as usize];
        let addr = current_stk.top().unwrap().1;
        let cell = dash_app(&self.input.port_a_bits.load);
        current_stk.pop();
        match p {
            HeapPort::A => self.heap_mem.write_a(addr, cell),
            HeapPort::B => self.heap_mem.write_b(addr, cell),
        }
        self.working_heap.write_a(addr, false); // for future re-allocation
    }

    /// write the incoming WHNF
    fn write_whnf(&mut self, p: HeapPort) {
        let addr = *self.addr_holder.value();
        let cell = dash_app(&self.holder_in.value().load);
        match p {
            HeapPort::A => self.heap_mem.write_a(addr, cell),
            HeapPort::B => self.heap_mem.write_b(addr, cell),
        }
        self.working_heap.write_b(addr, false); // for future re-allocation
    }

    /// when sub port is firing, find which stack is demanding the app
    fn find_dmder_stk(&self) -> u8 {
        if let Some(i) = self.waited_by() {
            i as u8
        } else {
            0
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
        self.read_target(p);
        self.arg_id.connect(&arg_id);
    }

    /// push the target, set its working flag
    fn push_target(&mut self, new_frame: bool) {
        let current_stk = &mut self.thread_stack[self.holder_in.value().stack_idx as usize];
        self.working_heap.write_b(*self.addr_holder.value(), true);
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
        (sensitive1 && same1) || (sensitive2 && same2)
    }

    /// take shortcuts, unless 'sensitive cases' are encountered
    fn step_to_next(&mut self, s1: &IAs1) {
        if self.is_sensitive(s1) {
            self.stm.connect(&Stm::IDLE);
        } else {
            self.consume_next();
        }
    }

    fn gen_output_whnf(&self) -> (App, Option<App>) {
        let dmder = self.heap_mem.dout_a();
        let target = &self.holder_in.value().load;
        let (arg_id, _) = select_1st_arg(dmder);
        deref(dmder, arg_id, target, self.input.free_addr)
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

    pub fn need_split(&self) -> bool {
        match self.stm.value() {
            Stm::IDLE => false,
            Stm::WHNF => {
                let dmder = self.heap_mem.dout_a();
                let target = &self.holder_in.value().load;
                app_length(dmder) + app_length(target) - 1 > APP_LENGTH
            }
            Stm::IA => match self.getIAs1() {
                IAs1::ExistWHNF => {
                    let dmder = &self.holder_in.value().load;
                    let target = self.heap_mem.dout_a();
                    app_length(dmder) + app_length(target) - 1 > APP_LENGTH
                }
                _ => false,
            },
            Stm::RESUME => false,
        }
    }

    /// if the resolved pointer is unique, update can be avoided
    fn can_avoid_update(&self) -> bool {
        // return false;
        match self.heap_mem.dout_a()[0] {
            Atom::PTR(_, true, _) => true,
            Atom::PRM(_, _) => match &self.heap_mem.dout_a()[1] {
                Atom::PTR(_, unique, _) => *unique,
                _ => match &self.heap_mem.dout_a()[2] {
                    Atom::PTR(_, true, _) => true,
                    _ => false,
                },
            },
            Atom::SEQ(false) => match &self.heap_mem.dout_a()[1] {
                Atom::PTR(_, unique, _) => *unique,
                _ => false,
            },
            Atom::SEQ(true) => match &self.heap_mem.dout_a()[2] {
                Atom::PTR(_, unique, _) => *unique,
                _ => false,
            },
            _ => false,
        }
    }

    fn dash_when_shared(&self) -> App {
        let dmder = &self.holder_in.value().load;
        let target = self.heap_mem.dout_a();
        if is_unique_ptr(&dmder[*self.arg_id.value()]) {
            target.clone()
        } else {
            dash_app(target)
        }
    }

    /// consumes the next task; must not use heap port b!
    fn consume_next(&mut self) {
        let in_app = mask_seq(&self.input.port_a_bits.load);
        self.holder_in.connect(&ActiveApp {
            stack_idx: self.input.port_a_bits.stack_idx,
            load: in_app.clone(),
        });
        match self.getCONSUMEs() {
            CONSUMEs::NoInput => {
                self.stm.connect(&Stm::IDLE);
                return;
            }
            CONSUMEs::InputIA => {
                self.select_1st_arg_read(&in_app);
                self.ia_addr.connect(
                    &self.thread_stack[self.input.port_a_bits.stack_idx as usize]
                        .top()
                        .unwrap()
                        .1,
                );
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
                self.stm.connect(&Stm::RESUME);
            }
            CONSUMEs::InputWHNFNoDmderNoFrame => {
                self.frame_stack[self.input.port_a_bits.stack_idx as usize].pop();
                self.write_incoming(HeapPort::A);
                self.stm.connect(&Stm::IDLE);
            }
        }
    }

    fn step_whnf(&mut self) {
        let whnf_addr = *self.addr_holder.value();

        match self.getWHNFs() {
            WHNFs::MoreDmders => {
                self.find_pop_read(find_more_dmder(whnf_addr), false);
                self.stm.connect(&Stm::WHNF);
            }
            WHNFs::NewFrame => {
                // don't need to write WHNF here, since RESUME will do
                self.find_pop_read(find_new_frame(whnf_addr), true);
                self.stm.connect(&Stm::RESUME);
                self.stat.heap_update += 1;
            }
            WHNFs::NoNewFrame => {
                if let Some((stk_id, stack)) = self
                    .thread_stack
                    .iter_mut()
                    .enumerate()
                    .find(|(_, s)| stack_cell_with(s.top(), |(_, addr)| *addr == whnf_addr))
                {
                    // pop when the whnf is the last item on that stack
                    stack.pop();
                    self.frame_stack[stk_id].pop();
                }

                if self.can_avoid_update() {
                    /* update avoided */
                    self.stat.update_avoided += 1;
                } else {
                    self.stat.heap_update += 1;
                    self.write_whnf(HeapPort::B);
                }
                self.consume_next();
            }
        }
    }

    fn step_ia(&mut self) {
        let dmder = &self.holder_in.value().load;
        let mut updated_dmder = dmder.clone();
        let target = self.heap_mem.dout_a().clone();
        let ias1 = self.getIAs1();

        match ias1 {
            IAs1::NoExist => {
                self.push_target(false);
            }
            IAs1::ExistWHNF => {
                let (deref_res, _) =
                    deref(dmder, *self.arg_id.value(), &target, self.input.free_addr);
                updated_dmder = deref_res;
                self.holder_in.input.load = updated_dmder.clone();
            }
            IAs1::ExistIAWorkingNormal => {
                // change this to `self.push_target(false);` will disable stack riding
                self.push_target(true);
            }
            IAs1::ExistIAWorkingAtNewFrame => { /* do nothing here */ }
            IAs1::ExistIAFresh => {
                self.push_target(false);
            }
        }

        match self.getIAs2(&ias1) {
            IAs2::NextStrictArgNewStk => {
                let current_idx = self.holder_in.value().stack_idx as usize;
                self.select_next_arg_read(&updated_dmder);
                let frame_record = self.frame_stack[current_idx].top().unwrap();
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
                    // !NEW! update current frame record, too (not good, hurts SUMEULER)
                    // self.frame_stack[current_idx].modify({
                    //     let mut record = self.frame_stack[current_idx].top().unwrap().clone();
                    //     record[stk_id] = *self.addr_holder.value(); // next top of current stk
                    //     record
                    // });
                };
            }
            IAs2::NextStrictArgLocal => {
                self.select_next_arg_read(&updated_dmder);
            }
            IAs2::NoMoreArgsNoEmit => {
                self.cancel_new_frame(&ias1);
                self.heap_mem.write_b(*self.ia_addr.value(), updated_dmder);
                self.step_to_next(&ias1);
            }
            IAs2::NoMoreArgsCanEmit => {
                self.cancel_new_frame(&ias1);
                self.step_to_next(&ias1);
            }
        }
    }

    fn step_resume(&mut self) {
        self.write_whnf(HeapPort::B);
        match self.getRESUMEs() {
            RESUMEs::TopInWHNF => {
                let current_stk = &mut self.thread_stack[self.holder_in.value().stack_idx as usize];
                let whnf_addr = current_stk.top().unwrap().1;
                let dmder_addr = current_stk.second().unwrap().1;
                self.holder_in.input.load = self.heap_mem.dout_a().clone();
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

    fn handle_port_a(&mut self) {
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
        if self.port_b_fire() {
            let addr = self.input.port_b_bits.heap_addr;
            let app = extend_to_app(&self.input.port_b_bits.load);
            self.heap_mem.write_b(addr, app);
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
                self.thread_stack[0].push((false, 0));
                self.frame_stack[0].push(Default::default());
                // put output register
                self.holder_out = (
                    true,
                    ActiveApp {
                        stack_idx: 0,
                        load: self.heap_mem.dout_a().clone(),
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
        self.non_exist.connect(&self.input.found);

        self.handle_port_a();
        self.handle_port_b();
    }

    fn update_stat(&mut self) {
        if self.stat_detail_lv >= DLV_FULL_LOG {
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
        }

        if self.stat_detail_lv >= DLV_THREADS {
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
            self.stat.work_threads.push((
                occupied as u8,
                self.stat.active_threads + if *self.stm.value() != Stm::IDLE { 1 } else { 0 },
            ));
        }

        if self.stat_detail_lv >= DLV_STM_DIST {
            match *self.stm.value() {
                Stm::IDLE => self.stat.stm_cycles[0] += 1,
                Stm::WHNF => self.stat.stm_cycles[1] += 1,
                Stm::IA => self.stat.stm_cycles[2] += 1,
                Stm::RESUME => self.stat.stm_cycles[3] += 1,
            }
        }
    }

    fn tick_children(&mut self) {
        self.stm.tick();
        for stk in &mut self.thread_stack {
            stk.tick();
        }
        for stk in &mut self.frame_stack {
            stk.tick();
        }
        self.heap_mem.tick();
        self.working_heap.tick();
        self.working.tick();
        self.holder_in.tick();
        self.arg_id.tick();
        self.addr_holder.tick();
        self.ia_addr.tick();
        self.non_exist.tick();
    }
}
