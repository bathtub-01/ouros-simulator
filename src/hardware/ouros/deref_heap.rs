// Dereference Heap, also handles thread management:
//           +-----------------+
// addr  <-->|    Dereference  |===> to_reducer
// port_a ==>|                 |
// port_b ==>|       Heap      |===> to_self
//           +-----------------+

use super::config::APP_LENGTH;
use super::program::{app_length, is_whnf, ActiveApp, App, Atom, FrozenApp, Program};
use crate::hardware::common::memory::DualPortMemStat;
use crate::hardware::common::{DualPortMem, Register, Stack};
use crate::hardware::utils::fire;
use crate::hw_module::{HwInput, HwModule};

#[derive(Default)]
pub struct DrfHeapInput {
    pub start: bool,
    pub port_a_valid: bool,
    pub port_a_bits: ActiveApp,
    pub port_b_valid: bool,
    pub port_b_bits: FrozenApp,
    pub to_reducer_ready: bool,
    pub to_self_ready: bool,
    pub out_sub_ready: bool,
    pub addr_consumed: usize, // address request from reducer (for GC)
}

impl HwInput for DrfHeapInput {}

type StackCell = usize;

#[derive(Default)]
enum Dest {
    #[default]
    ToReducer,
    ToSelf,
}

#[derive(Default, Clone, PartialEq, Debug)]
enum Stm {
    #[default]
    IDLE,
    WHNF,
    // WHNFsub,
    IA,
    IAw,
    OPa,
    OPaw,
    OPb,
    OPbw,
}

#[derive(Default, Clone)]
struct HeapCell {
    exist: bool,
    working: bool,
    app: App,
}

#[derive(Default)]
pub struct DrfHeapStat {
    pub work_threads: Vec<u8>,
    pub holder_contents: Vec<Option<ActiveApp>>,
    pub stm_cycles: [u32; 8],
    pub wasted_cycles: u32,
}

// TODO: enable stack depth configuration
pub struct DrfHeap {
    pub input: DrfHeapInput,
    stm: Register<Stm>,
    holder_in: ActiveApp,
    addr_holder: usize,
    addr_holder_sub: usize,
    thread_stack: [Stack<StackCell, 128>; 8],
    heap_mem: DualPortMem<HeapCell>,
    demand_heap: DualPortMem<bool>,
    holder_out: (Dest, bool, ActiveApp), // output-reg: (destination, valid, app)
    holder_out_sub: Register<(bool, App)>,
    working: Register<bool>, // track whether the machine is working
    same_addr: Register<bool>,
    addr_bumper: Register<usize>,
    stat: DrfHeapStat,
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
            demand_heap: DualPortMem::new(heap_size),
            holder_out: Default::default(),
            working: Default::default(),
            addr_bumper: Default::default(),
            stat: Default::default(),
            holder_out_sub: Default::default(),
            same_addr: Default::default(),
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
            HeapCell {
                exist: true,
                working: false,
                app,
            }
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
        fire(self.to_reducer_valid(), self.input.to_reducer_ready)
            || fire(self.to_self_valid(), self.input.to_self_ready)
    }

    fn out_sub_fire(&self) -> bool {
        fire(self.out_sub_valid(), self.input.out_sub_ready)
    }

    /// port_a is ready to handle a new application in this cycle (shortcut)
    pub fn port_a_ready(&self) -> bool {
        let out_clear = !self.holder_out.1 || self.output_fire();
        let local_shortcut: bool = {
            match *self.stm.value() {
                Stm::IDLE => true,
                Stm::WHNF => true,
                Stm::IA => self.heap_mem.dout_a().exist && is_whnf(&self.heap_mem.dout_a().app),
                Stm::OPa => {
                    // ugly but easy to check...
                    if !self.heap_mem.dout_a().exist {
                        match self.holder_in.load[2] {
                            Atom::INT(_) => true,
                            _ => false,
                        }
                    } else if is_whnf(&self.heap_mem.dout_a().app) {
                        match self.holder_in.load[2] {
                            Atom::INT(_) => true,
                            _ => false,
                        }
                    } else {
                        false
                    }
                }
                Stm::OPb => {
                    if is_whnf(&self.heap_mem.dout_a().app) {
                        true
                    } else if self.output_fire() || !self.holder_out.1 {
                        match self.holder_in.load[1] {
                            Atom::INT(_) => false,
                            _ => self.pick_stack() == None,
                        }
                    } else {
                        false
                    }
                }
                _ => false,
            }
        };

        out_clear && local_shortcut
    }

    pub fn port_b_ready(&self) -> bool {
        let out_clear = !self.holder_out_sub.value().0 || self.out_sub_fire();
        let local_used = match *self.stm.value() {
            Stm::WHNF => true,
            Stm::IA => {
                self.heap_mem.dout_a().exist
                    && !is_whnf(&self.heap_mem.dout_a().app)
                    && !self.heap_mem.dout_a().working
            }
            Stm::OPa => {
                if !self.heap_mem.dout_a().exist {
                    match self.holder_in.load[2] {
                        Atom::INT(_) => true,
                        _ => false,
                    }
                } else {
                    !is_whnf(&self.heap_mem.dout_a().app)
                }
            }
            Stm::OPb => {
                if !is_whnf(&self.heap_mem.dout_a().app)
                    && (self.output_fire() || !self.holder_out.1)
                {
                    match self.holder_in.load[1] {
                        Atom::INT(_) => !self.heap_mem.dout_a().working,
                        _ => true,
                    }
                } else {
                    false
                }
            }
            _ => false,
        };

        out_clear && !local_used
    }

    pub fn to_reducer_valid(&self) -> bool {
        match self.holder_out.0 {
            Dest::ToReducer => self.holder_out.1,
            Dest::ToSelf => false,
        }
    }

    pub fn to_reducer_bits(&self) -> &ActiveApp {
        &self.holder_out.2
    }

    pub fn to_self_valid(&self) -> bool {
        match self.holder_out.0 {
            Dest::ToReducer => false,
            Dest::ToSelf => self.holder_out.1,
        }
    }

    pub fn to_self_bits(&self) -> &ActiveApp {
        &self.holder_out.2
    }

    pub fn out_sub_valid(&self) -> bool {
        !self.same_addr.value() && (self.holder_out_sub.value().0 || *self.demand_heap.dout_b())
    }

    pub fn out_sub_bits(&self) -> ActiveApp {
        // since it's demanded, there must be at least one stack waiting
        let idx = {
            match self.thread_stack.iter().position(|s| match s.top() {
                None => false,
                Some(a) => *a == self.addr_holder_sub,
            }) {
                Some(idx) => idx as u8,
                None => panic!("dheap: demanded but not waiting!"),
            }
        };

        ActiveApp {
            stack_idx: idx,
            load: self.holder_out_sub.value().1.clone(),
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

    /// return the index of a free stack, None if all stacks are in use
    fn pick_stack(&self) -> Option<usize> {
        for (i, stk) in self.thread_stack.iter().enumerate() {
            if stk.elements() == 0 {
                return Some(i);
            }
        }
        None
    }

    /// handle new input and jump to next state
    fn handle_new_input(&mut self) {
        if self.port_a_fire() {
            // handle new input
            let stk = &mut self.thread_stack[self.input.port_a_bits.stack_idx as usize];
            // put input into holder
            self.holder_in = self.input.port_a_bits.clone();
            if is_whnf(&self.input.port_a_bits.load) {
                match stk.second() {
                    Some(addr) => {
                        // read demander app
                        self.heap_mem.read_a(*addr);
                        // jump to next state
                        self.stm.connect(&Stm::WHNF);
                    }
                    None => {
                        // A sub thread reaches its end

                        // pop this sub thread
                        stk.pop();

                        // find the demander, if any
                        let top = *stk.top().unwrap();
                        let (idx, demander): (usize, Option<&usize>) = {
                            let mut res = (0, None);
                            for (i, s) in self.thread_stack.iter().enumerate() {
                                if s.top() == Some(&top) {
                                    if s.second() != None {
                                        res = (i, s.second());
                                        break;
                                    }
                                }
                            }
                            res
                        };
                        match demander {
                            Some(addr) => {
                                self.holder_in.stack_idx = idx as u8;
                                // read demander app
                                self.heap_mem.read_a(*addr);
                                // jump to next state
                                self.stm.connect(&Stm::WHNF);
                            }
                            None => {
                                // no one is waiting yet, just write it
                                // NOTE: this rule require all stacks are "stable" in this cycle
                                //        some shortcuts might be problematic
                                self.heap_mem.write_a(
                                    top,
                                    HeapCell {
                                        exist: true,
                                        working: false, // already in WHNF
                                        app: self.input.port_a_bits.load.clone(),
                                    },
                                );

                                // jump to next state
                                self.stm.connect(&Stm::IDLE);
                            }
                        }
                    }
                }
            } else {
                match self.input.port_a_bits.load[0] {
                    Atom::PTR(p) => {
                        // read the target
                        self.heap_mem.read_a(p);
                        self.demand_heap.write_a(p, true);

                        self.addr_holder = p;

                        // if the requested app is also entering at the same cycle
                        if self.port_b_fire() && self.input.port_b_bits.heap_addr == p {
                            self.same_addr.connect(&true);
                        }

                        // jump to next state
                        self.stm.connect(&Stm::IA);
                    }
                    Atom::PRM(_, _) => {
                        // (op a b)
                        // check `a`
                        match self.input.port_a_bits.load[1] {
                            Atom::PTR(p) => {
                                // read the target
                                self.heap_mem.read_a(p);
                                self.demand_heap.write_a(p, true);
                                // if the requested app is also entering at the same cycle
                                if self.port_b_fire() && self.input.port_b_bits.heap_addr == p {
                                    self.same_addr.connect(&true);
                                }
                                // OPa need this
                                self.addr_holder = p;
                                // jump to next state
                                self.stm.connect(&Stm::OPa);
                            }
                            Atom::INT(_) => match self.input.port_a_bits.load[2] {
                                Atom::PTR(p) => {
                                    // read the target
                                    self.heap_mem.read_a(p);
                                    self.demand_heap.write_a(p, true);
                                    // if the requested app is also entering at the same cycle
                                    if self.port_b_fire() && self.input.port_b_bits.heap_addr == p {
                                        self.same_addr.connect(&true);
                                    }

                                    // OPb need this
                                    self.addr_holder = p;
                                    // jump to next state
                                    self.stm.connect(&Stm::OPb);
                                }
                                _ => panic!("dheap: unknown PRM argument `b` type!"),
                            },
                            _ => panic!("dheap: unknown PRM argument `a` type!"),
                        }
                    }
                    _ => {
                        panic!("dheap: unknown input shape (redex should not enter dheap)!")
                    }
                }
            }
        }
    }
}

/// Determine the destination of an output application
fn which_dest(app: &App) -> Dest {
    if is_whnf(app) {
        Dest::ToSelf
    } else {
        match (&app[0], &app[1], &app[2]) {
            (Atom::PTR(_), _, _) => Dest::ToSelf,
            (Atom::PRM(_, _), Atom::PTR(_), _) => Dest::ToSelf,
            (Atom::PRM(_, _), _, Atom::PTR(_)) => Dest::ToSelf,
            _ => Dest::ToReducer,
        }
    }
}

/// Dereference app's head pointer
fn deref(app: &App, target: &App) -> App {
    match app[0] {
        Atom::PTR(_) => {
            let app_len = app_length(app);
            let target_len = app_length(target);
            if app_len + target_len - 1 > APP_LENGTH {
                panic!("dheap: deref: deref result too long!");
            }
            let mut res: App = std::array::from_fn(|_| Atom::NOP);
            for i in 0..target_len {
                res[i] = target[i].clone();
            }
            for i in 1..app_len {
                res[target_len + i - 1] = app[i].clone();
            }
            res
        }
        _ => panic!("dheap: deref: app head is not a PTR!"),
    }
}

#[test]
fn deref_spec() {
    use Atom::*;
    let app: App = [PTR(0), INT(1), INT(2), INT(3), NOP, NOP, NOP, NOP];
    let target1: App = [PTR(11), PTR(22), PTR(33), PTR(44), PTR(55), NOP, NOP, NOP];
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
    let target2: App = [PTR(11), PTR(22), PTR(33), NOP, NOP, NOP, NOP, NOP];
    let res2: App = [PTR(11), PTR(22), PTR(33), INT(1), INT(2), INT(3), NOP, NOP];
    assert_eq!(deref(&app, &target1), res1);
    assert_eq!(deref(&app, &target2), res2);
}

impl HwModule for DrfHeap {
    fn update_local(&mut self) {
        // always give default inputs at the beginning of a cycle
        self.heap_mem.input.default_input();
        self.demand_heap.input.default_input();
        for stk in &mut self.thread_stack {
            stk.input.default_input();
        }

        // rise addr bumper
        self.addr_bumper
            .connect(&(self.addr_bumper.value() + self.input.addr_consumed));

        // start the machine (demand flag of `main`, at addr 0, need to be false.)
        if !self.working.value() {
            if self.input.start {
                self.working.connect(&true);
                // push to stack
                self.thread_stack[0].push(0);
                // put output register
                let main = &self.heap_mem.dout_a().app;
                self.holder_out = (
                    which_dest(main),
                    true,
                    ActiveApp {
                        stack_idx: 0,
                        load: main.clone(),
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

        // clear holder when output fires
        if self.output_fire() {
            self.holder_out.1 = false;
        }

        if self.out_sub_fire() {
            self.holder_out_sub.input.0 = false;
        }

        self.same_addr.connect(&false);

        self.addr_holder_sub = self.input.port_b_bits.heap_addr;

        // handle port_a

        match *self.stm.value() {
            Stm::IDLE => {
                self.handle_new_input();
            }
            Stm::WHNF => {
                let stk = &mut self.thread_stack[self.holder_in.stack_idx as usize];
                // TODO: handle multiple sharer case..

                // no more sharer
                let target = &self.holder_in.load;

                // write incoming WHNF
                match stk.top() {
                    None => panic!("dheap: thread stack error!"),
                    Some(addr) => self.heap_mem.write_b(
                        *addr,
                        HeapCell {
                            exist: true,
                            working: false, // already in WHNF
                            app: target.clone(),
                        },
                    ),
                }

                // pop the stack
                stk.pop();
                match self.heap_mem.dout_a().app[0] {
                    Atom::PRM(_, _) => {
                        let arg = {
                            match target[0] {
                                Atom::INT(i) => i,
                                _ => panic!("dheap: PRM argument must be an Int!"),
                            }
                        };
                        let mut demander = self.heap_mem.dout_a().app.clone();
                        // this WHNF belongs to the first non-literal argument
                        match self.heap_mem.dout_a().app[1] {
                            Atom::PTR(_) => {
                                demander[1] = Atom::INT(arg);
                                // emit demander
                            }
                            Atom::INT(_) => {
                                // argument `b` must be a PTR in this case
                                demander[2] = Atom::INT(arg);
                                // emit demander
                            }
                            _ => panic!("dheap: strange PRM argument `a`!"),
                        }
                        // put output register
                        self.holder_out = (
                            which_dest(&demander),
                            true,
                            ActiveApp {
                                stack_idx: self.holder_in.stack_idx,
                                load: demander,
                            },
                        );
                    }
                    _ => {
                        // put output register
                        let demander = &self.heap_mem.dout_a().app;
                        let deref_res = deref(demander, target);
                        self.holder_out = (
                            which_dest(&deref_res),
                            true,
                            ActiveApp {
                                stack_idx: self.holder_in.stack_idx,
                                load: deref_res,
                            },
                        );
                    }
                }
                // jump to next state
                self.stm.connect(&Stm::IDLE);
                self.handle_new_input();
            }
            Stm::IA => {
                let target = &self.heap_mem.dout_a().app;
                let exist = self.heap_mem.dout_a().exist;
                let demander = &self.holder_in.load;
                let stk = &mut self.thread_stack[self.holder_in.stack_idx as usize];
                if !exist {
                    // write incoming demander (suspend it)
                    match stk.top() {
                        None => panic!("dheap: thread stack error!"),
                        Some(addr) => self.heap_mem.write_a(
                            *addr,
                            HeapCell {
                                exist: true,
                                working: true,
                                app: demander.clone(),
                            },
                        ),
                    }

                    // push target to the stack (wait for its arrival)
                    match demander[0] {
                        Atom::PTR(p) => {
                            stk.push(p);
                        }
                        _ => panic!("dheap: demander head should be a PTR!"),
                    }

                    // jump to next state
                    self.stm.connect(&Stm::IDLE);
                } else if is_whnf(target) {
                    // put output register
                    let deref_res = deref(demander, target);
                    self.holder_out = (
                        which_dest(&deref_res),
                        true,
                        ActiveApp {
                            stack_idx: self.holder_in.stack_idx,
                            load: deref_res,
                        },
                    );
                    // jump to next state (allow shortcut)
                    self.stm.connect(&Stm::IDLE);
                    self.handle_new_input();
                } else {
                    if self.heap_mem.dout_a().working {
                        // TODO: handle multiple sharer case (target in computation)..
                        panic!("not impl yet!");
                    } else {
                        // target is fresh

                        // write incoming demander (suspend it)
                        match stk.top() {
                            None => panic!("dheap: thread stack error!"),
                            Some(addr) => self.heap_mem.write_a(
                                *addr,
                                HeapCell {
                                    exist: true,
                                    working: true,
                                    app: demander.clone(),
                                },
                            ),
                        }

                        // write the updated target info.
                        self.heap_mem.write_b(
                            self.addr_holder,
                            HeapCell {
                                exist: true,
                                working: true,
                                app: self.holder_in.load.clone(),
                            },
                        );

                        // push target to the stack (start new thread)
                        match demander[0] {
                            Atom::PTR(p) => {
                                stk.push(p);
                            }
                            _ => panic!("dheap: demander head should be a PTR!"),
                        }

                        let target = &self.heap_mem.dout_a().app;

                        self.holder_out = (
                            which_dest(target),
                            true,
                            ActiveApp {
                                stack_idx: self.holder_in.stack_idx,
                                load: target.clone(),
                            },
                        );
                        // jump to next state
                        self.stm.connect(&Stm::IDLE);
                    }
                }
            }
            Stm::IAw => unimplemented!(),
            Stm::OPa => {
                let target = &self.heap_mem.dout_a().app;
                let exist = self.heap_mem.dout_a().exist;
                let demander = &self.holder_in.load;
                let stk = &mut self.thread_stack[self.holder_in.stack_idx as usize];

                if !exist {
                    // FIXME: the `working` flag of 'a' should raise?

                    // push target to the stack (wait for its arrival)
                    stk.push(self.addr_holder);

                    // check `b`
                    match self.holder_in.load[2] {
                        Atom::PTR(p) => {
                            // read the target
                            self.heap_mem.read_a(p);
                            self.addr_holder = p;
                            // jump to next state
                            self.stm.connect(&Stm::OPb);
                        }
                        Atom::INT(_) => {
                            // write incoming demander (suspend it)

                            match stk.top() {
                                None => panic!("dheap: thread stack error!"),
                                Some(addr) => self.heap_mem.write_b(
                                    *addr,
                                    HeapCell {
                                        exist: true,
                                        working: true,
                                        app: demander.clone(),
                                    },
                                ),
                            }
                            // jump to next state
                            self.stm.connect(&Stm::IDLE);
                            self.handle_new_input();
                        }
                        _ => panic!("dheap: unknown PRM argument `b` type!"),
                    }
                } else if is_whnf(target) {
                    // in this case, `a` should be an Int
                    let a = {
                        match target[0] {
                            Atom::INT(i) => i,
                            _ => panic!("dheap: PRM argument `a` must be an Int!"),
                        }
                    };
                    // check `b`
                    match self.holder_in.load[2] {
                        Atom::PTR(p) => {
                            // read the target
                            self.heap_mem.read_a(p);
                            self.holder_in.load[1] = Atom::INT(a);
                            self.addr_holder = p;
                            // jump to next state
                            self.stm.connect(&Stm::OPb);
                        }
                        Atom::INT(_) => {
                            // put output register
                            self.holder_out = (
                                Dest::ToReducer,
                                true,
                                ActiveApp {
                                    stack_idx: self.holder_in.stack_idx,
                                    load: {
                                        let mut res: App = self.holder_in.load.clone();
                                        res[1] = Atom::INT(a);
                                        res
                                    },
                                },
                            );
                            // jump to next state
                            self.stm.connect(&Stm::IDLE);
                            self.handle_new_input();
                        }
                        _ => panic!("dheap: unknown PRM argument `b` type!"),
                    }
                } else {
                    // `a` not in WHNF, emit `a`

                    let target = self.heap_mem.dout_a().app.clone();
                    // write the updated target info.
                    self.heap_mem.write_b(
                        self.addr_holder,
                        HeapCell {
                            exist: true,
                            working: true,
                            app: target.clone(),
                        },
                    );

                    // push `a` to the stack
                    stk.push(self.addr_holder);

                    // put register
                    self.holder_out = (
                        which_dest(&target),
                        true,
                        ActiveApp {
                            stack_idx: self.holder_in.stack_idx,
                            load: target,
                        },
                    );

                    // jump to next state
                    match self.holder_in.load[2] {
                        Atom::PTR(p) => {
                            // read the target
                            self.heap_mem.read_a(p);
                            self.addr_holder = p;
                            // jump to next state
                            self.stm.connect(&Stm::OPb);
                        }
                        Atom::INT(_) => {
                            // store the demander
                            match self.thread_stack[self.holder_in.stack_idx as usize].top() {
                                Some(addr) => self.heap_mem.write_a(
                                    *addr,
                                    HeapCell {
                                        exist: true,
                                        working: true,
                                        app: self.holder_in.load.clone(),
                                    },
                                ),
                                None => panic!("dheap: ill stack!"),
                            }

                            // jump to next state
                            self.stm.connect(&Stm::IDLE);
                        }
                        _ => panic!("dheap: unknown PRM argument `b` type!"),
                    }
                }
            }
            Stm::OPaw => unimplemented!(),
            Stm::OPb => {
                let target = &self.heap_mem.dout_a().app;
                // FIXME: b might not exist..
                if is_whnf(target) {
                    // FIXME: a might not be an int yet..

                    // in this case, `b` should be an Int
                    let b = {
                        match target[0] {
                            Atom::INT(i) => i,
                            _ => panic!("dheap: PRM argument `b` must be an Int!"),
                        }
                    };
                    self.holder_in.load[2] = Atom::INT(b);

                    // put register
                    self.holder_out = (
                        Dest::ToReducer,
                        true,
                        ActiveApp {
                            stack_idx: self.holder_in.stack_idx,
                            load: self.holder_in.load.clone(),
                        },
                    );

                    // jump to next state
                    self.stm.connect(&Stm::IDLE);
                    self.handle_new_input();
                } else if self.output_fire() || !self.holder_out.1 {
                    // whether to spark a new thread for `b`
                    let target = self.heap_mem.dout_a().app.clone();
                    match self.holder_in.load[1] {
                        Atom::INT(_) => {
                            // create a thread for `b` in-place

                            // push `b` to the stack
                            self.thread_stack[self.holder_in.stack_idx as usize]
                                .push(self.addr_holder);

                            if !self.heap_mem.dout_a().working {
                                // write the updated target info.
                                self.heap_mem.write_a(
                                    self.addr_holder,
                                    HeapCell {
                                        exist: true,
                                        working: true,
                                        app: target.clone(),
                                    },
                                );
                                // store the demander
                                self.heap_mem.write_b(
                                    *self.thread_stack[self.holder_in.stack_idx as usize]
                                        .top()
                                        .unwrap(),
                                    HeapCell {
                                        exist: true,
                                        working: true,
                                        app: self.holder_in.load.clone(),
                                    },
                                );
                                // put register
                                self.holder_out = (
                                    which_dest(&target),
                                    true,
                                    ActiveApp {
                                        stack_idx: self.holder_in.stack_idx,
                                        load: target,
                                    },
                                );

                                // jump to next state
                                self.stm.connect(&Stm::IDLE);
                            } else {
                                // `b` is already in computation (`a` returns earlier)
                                // `b` is already pushed before this `if` block..
                                // write the demander (suspend)
                                self.heap_mem.write_a(
                                    *self.thread_stack[self.holder_in.stack_idx as usize]
                                        .top()
                                        .unwrap(),
                                    HeapCell {
                                        exist: true,
                                        working: true,
                                        app: self.holder_in.load.clone(),
                                    },
                                );

                                // Can't shortcut this, because `b` might be returning in this cycle, the pushed `b` on stack is invisible at this cycle's shortcut
                                self.stm.connect(&Stm::IDLE);
                            }
                        }
                        _ => {
                            // spark a new thread for `b`

                            match self.pick_stack() {
                                Some(stk_idx) => {
                                    // spark a new thread on an empty stack

                                    // write the updated target info.
                                    self.heap_mem.write_a(
                                        self.addr_holder,
                                        HeapCell {
                                            exist: true,
                                            working: true,
                                            app: target.clone(),
                                        },
                                    );

                                    // store the demander
                                    self.heap_mem.write_b(
                                        *self.thread_stack[self.holder_in.stack_idx as usize]
                                            .second()
                                            .unwrap(),
                                        HeapCell {
                                            exist: true,
                                            working: true,
                                            app: self.holder_in.load.clone(),
                                        },
                                    );

                                    // push the stack
                                    self.thread_stack[stk_idx].push(self.addr_holder);

                                    // put register
                                    self.holder_out = (
                                        which_dest(&target),
                                        true,
                                        ActiveApp {
                                            stack_idx: stk_idx as u8,
                                            load: target,
                                        },
                                    );

                                    // jump to next state
                                    self.stm.connect(&Stm::IDLE);
                                }
                                None => {
                                    // unable to spark a new thread, back to IDLE

                                    // store the demander
                                    self.heap_mem.write_b(
                                        *self.thread_stack[self.holder_in.stack_idx as usize]
                                            .second()
                                            .unwrap(),
                                        HeapCell {
                                            exist: true,
                                            working: true,
                                            app: self.holder_in.load.clone(),
                                        },
                                    );

                                    // jump to next state
                                    self.stm.connect(&Stm::IDLE);
                                    self.handle_new_input();
                                }
                            }
                        }
                    }
                } else {
                    // keep reading b
                    panic!("funny thing");
                    match self.holder_in.load[2] {
                        Atom::PTR(p) => {
                            // read the target
                            self.heap_mem.read_a(p);
                        }
                        _ => {}
                    }
                }
            }
            Stm::OPbw => unimplemented!(),
        }

        // handle port_b

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
            self.heap_mem.write_b(
                addr,
                HeapCell {
                    exist: true,
                    working: false,
                    app,
                },
            );
        }
    }

    fn update_stat(&mut self) {
        if self.holder_out.1 {
            self.stat
                .holder_contents
                .push(Some(self.holder_out.2.clone()));
        } else {
            self.stat.holder_contents.push(None);
        }

        let threads = self
            .thread_stack
            .iter()
            .filter(|stk| stk.elements() != 0)
            .count();
        self.stat.work_threads.push(threads as u8);

        match *self.stm.value() {
            Stm::IDLE => self.stat.stm_cycles[0] += 1,
            Stm::WHNF => self.stat.stm_cycles[1] += 1,
            Stm::IA => self.stat.stm_cycles[2] += 1,
            Stm::IAw => self.stat.stm_cycles[3] += 1,
            Stm::OPa => self.stat.stm_cycles[4] += 1,
            Stm::OPaw => self.stat.stm_cycles[5] += 1,
            Stm::OPb => self.stat.stm_cycles[6] += 1,
            Stm::OPbw => self.stat.stm_cycles[7] += 1,
        }
    }

    fn tick_children(&mut self) {
        self.stm.tick();
        for stk in &mut self.thread_stack {
            stk.tick();
        }
        self.heap_mem.tick();
        self.demand_heap.tick();
        self.working.tick();
        self.holder_out_sub.tick();
        self.addr_bumper.tick();
        self.same_addr.tick();
    }
}
