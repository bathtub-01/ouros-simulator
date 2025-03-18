// Dereference Heap, also handles thread management:
//           +-----------------+
// addr  <-->|    Dereference  |===> to_reducer
// port_a ==>|                 |
// port_b ==>|       Heap      |===> to_self
//           +-----------------+

use crate::hardware::common::{DualPortMem, Register, Stack};
use crate::hardware::ouros::config::{APP_LENGTH, HOLES};
use crate::hardware::ouros::program::{ActiveApp, App, Atom, FrozenApp};
use crate::hardware::utils::fire;
use crate::hw_module::{HwInput, HwModule};

#[derive(Default)]
struct DrfHeapInput {
    port_a_valid: bool,
    port_a_bits: ActiveApp,
    port_b_valid: bool,
    port_b_bits: FrozenApp,
    to_reducer_ready: bool,
    to_self_ready: bool,
    // address request from reducer (for GC)
    addr_consumed: usize,
}

impl HwInput for DrfHeapInput {}

type StackCell = usize;

#[derive(Default)]
enum Dest {
    #[default]
    ToReducer,
    ToSelf,
}

#[derive(Default, Clone, PartialEq)]
enum Stm {
    #[default]
    IDLE,
    WAIT, // waiting for output firing
}

#[derive(Default, Clone)]
struct HeapCell {
    working: bool,
    stack_idx: u8,
    app: App,
}

pub struct DrfHeap {
    input: DrfHeapInput,
    stm: Register<Stm>,
    thread_stack: [Stack<StackCell, 128>; 8],
    heap_mem: DualPortMem<HeapCell>,
    holder: (Dest, bool, ActiveApp), // (destination, valid, app)
}

impl DrfHeap {
    pub fn new(heap_size: usize) -> Self {
        Self {
            input: Default::default(),
            stm: Default::default(),
            thread_stack: std::array::from_fn(|_| Stack::new()),
            heap_mem: DualPortMem::new(heap_size),
            holder: Default::default(),
        }
    }

    fn port_a_fire(&self) -> bool {
        fire(self.input.port_a_valid, self.port_a_ready())
    }

    fn output_fire(&self) -> bool {
        fire(self.to_reducer_valid(), self.input.to_reducer_ready)
            || fire(self.to_self_valid(), self.input.to_self_ready)
    }

    pub fn port_a_ready(&self) -> bool {
        // port_a is ready to handle a new application in this cycle
        *self.stm.value() == Stm::IDLE || (*self.stm.value() == Stm::WAIT && self.output_fire())
    }

    pub fn port_b_ready(&self) -> bool {
        // FIXME
        true
    }

    pub fn to_reducer_valid(&self) -> bool {
        match self.holder.0 {
            Dest::ToReducer => self.holder.1,
            Dest::ToSelf => false,
        }
    }

    pub fn to_reducer_bits(&self) -> &ActiveApp {
        &self.holder.2
    }

    pub fn to_self_valid(&self) -> bool {
        match self.holder.0 {
            Dest::ToReducer => false,
            Dest::ToSelf => self.holder.1,
        }
    }

    pub fn to_self_bits(&self) -> &ActiveApp {
        &self.holder.2
    }
}

fn arity_of(atom: &Atom) -> u8 {
    use Atom::*;
    match atom {
        COM(a, _, _) => *a,
        PRM(_) => 2,
        Y => 1,
        _ => 0,
    }
}

/// The length of an application, stripping off NOPs.
fn app_length(app: &App) -> usize {
    let found = app.iter().enumerate().find(|&(_, atom)| *atom != Atom::NOP);
    match found {
        Some((idx, _)) => idx,
        None => APP_LENGTH,
    }
}

/// Determine whether an Application is in Weak-Head-Normal-Form.
fn is_whnf(app: &App) -> bool {
    // +, a, b --- false
    // +, a    --- true
    arity_of(&app[0]) >= app_length(app) as u8
}

impl HwModule for DrfHeap {
    fn update_local(&mut self) {
        // handle port_a

        /// handle new input based on its shape, and jump to next state
        fn handle_new_input(h: &mut DrfHeap) {
            h.stm.connect(&Stm::IDLE);
        }

        match *self.stm.value() {
            Stm::IDLE => {
                if self.port_a_fire() {
                    handle_new_input(self);
                }
            }
            Stm::WAIT => {
                if self.output_fire() {
                    if self.port_a_fire() {
                        handle_new_input(self);
                    } else {
                        self.stm.connect(&Stm::IDLE);
                    }
                }
            }
        }

        // handle port_b

        // always write frozen applications
        if fire(self.input.port_b_valid, self.port_b_ready()) {
            self.heap_mem.input.link(|input| {
                input.port_b.is_write = true;
                input.port_b.addr = self.input.port_b_bits.heap_addr;
                input.port_b.din = HeapCell {
                    working: false,
                    stack_idx: 0,
                    app: {
                        let mut extended: [Atom; APP_LENGTH] = std::array::from_fn(|_| Atom::NOP);
                        for (i, a) in self.input.port_b_bits.load.iter().enumerate() {
                            extended[i] = a.clone();
                        }
                        extended
                    },
                };
            });
        }
    }

    fn tick_children(&mut self) {
        self.stm.tick();
        for stk in &mut self.thread_stack {
            stk.tick();
        }
        self.heap_mem.tick();
    }
}
