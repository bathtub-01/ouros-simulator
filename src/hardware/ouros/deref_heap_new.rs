// Dereference Heap, also handles thread management:
//           +-----------------+
// addr  <-->|    Dereference  |===> out_main
// port_a ==>|                 |
// port_b ==>|       Heap      |===> out_sub
//           +-----------------+

use super::config::APP_LENGTH;
use super::ouros_core::is_int;
use super::program::{app_length, is_whnf, ActiveApp, App, Atom, FrozenApp, Program};
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
    pub addr_consumed: usize, // address request from reducer (for GC)
}

type StackCell = (bool, usize);

fn stack_cell_with(cell: Option<&StackCell>, p: impl FnOnce(&StackCell) -> bool) -> bool {
    match cell {
        Some(c) => p(c),
        None => false,
    }
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
enum IAs1 {
    NoExist,
    ExistWHNF,
    ExistIAWorkingNormal,
    ExistIAWorkingAtNewFrame,
    ExistIAResting,
}

/// branch conditions for the `IA` state, part 2
enum IAs2 {
    NextStrictArgs,
    NoStrictArgs,
}

/// branch conditions for the `RESUME` state
enum RESUMEs {
    TopInWHNF,
    TopInIA,
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
    thread_stack: [Stack<StackCell, 128>; 8],
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
                if self.thread_stack.iter().any(|s| {
                    stack_cell_with(s.top(), |(_, addr)| *addr == stk.top().unwrap().1)
                        && stack_cell_with(s.second(), |(flag, _)| !*flag)
                }) {
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
        if self.thread_stack.iter().any(|s| {
            stack_cell_with(s.top(), |(_, addr)| *addr == self.addr_holder)
                && stack_cell_with(s.second(), |(flag, _)| !*flag)
        }) {
            WHNFs::MoreDmders
        } else {
            if self.thread_stack.iter().any(|s| {
                stack_cell_with(s.top(), |(_, addr)| *addr == self.addr_holder)
                    && stack_cell_with(s.second(), |(flag, _)| *flag)
            }) {
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
                    IAs1::ExistIAResting
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

    fn getIAs2(&self) -> IAs2 {
        let ia = &self.holder_in.value().load;
        let idle_stack: bool = self.thread_stack.iter().any(|s| s.elements() == 0)
            || self
                .thread_stack
                .iter()
                .any(|s| stack_cell_with(s.top(), |(flag, _)| *flag));
        let more_strict_args: bool = {
            match ia[0] {
                Atom::PTR(_) => false,
                Atom::PRM(_, _) => self.arg_id == 1 && !is_int(&ia[2]),
                // more on this to support strict args in the future
                _ => unreachable!(),
            }
        };

        if idle_stack && more_strict_args {
            IAs2::NextStrictArgs
        } else {
            IAs2::NoStrictArgs
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

    fn consume_next(&mut self) {
        self.holder_in.connect(&self.input.port_a_bits.clone());
        match self.getCONSUMEs() {
            CONSUMEs::NoInput => todo!(),
            CONSUMEs::InputIA => todo!(),
            CONSUMEs::InputWHNFWithDmder => todo!(),
            CONSUMEs::InputWHNFNoDmderNewFrame => todo!(),
            CONSUMEs::InputWHNFNoDmderNoFrame => todo!(),
        }
    }

    fn step_whnf(&mut self) {
        match self.getWHNFs() {
            WHNFs::MoreDmders => todo!(),
            WHNFs::NewFrame => todo!(),
            WHNFs::NoNewFrame => todo!(),
        }
    }

    fn step_ia(&mut self) {}

    fn step_resume(&mut self) {
        match self.getRESUMEs() {
            RESUMEs::TopInWHNF => todo!(),
            RESUMEs::TopInIA => todo!(),
        }
    }

    fn handle_port_a(&mut self) {
        // halt the machine if output is not consumed yet
        if self.holder_out.0 {
            return;
        }

        match *self.stm.value() {
            Stm::IDLE => {
                self.consume_next();
            }
            Stm::WHNF => {}
            Stm::IA => {}
            Stm::RESUME => {}
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
