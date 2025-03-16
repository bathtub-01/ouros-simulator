// Dereference Heap, also handles thread management:
//           +-----------------+
// addr  <-->|    Dereference  |===> to_reducer
// port_a ==>|                 |
// port_b ==>|       Heap      |===> to_self
//           +-----------------+

use crate::hardware::common::{DualPortMem, Register, Stack};
use crate::hardware::ouros::config::{APP_LENGTH, HOLES};
use crate::hardware::ouros::program::{App, Atom, DataFlowPkt};
use crate::hardware::utils::fire;
use crate::hw_module::{HwInput, HwModule};

#[derive(Default)]
struct DrfHeapInput {
    port_a_valid: bool,
    port_a_bits: DataFlowPkt<App>,
    port_b_valid: bool,
    port_b_bits: DataFlowPkt<[Atom; HOLES - 1]>,
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

#[derive(Default, Clone)]
enum Stm {
    #[default]
    IDLE,
}

pub struct DrfHeap {
    input: DrfHeapInput,
    stm: Register<Stm>,
    thread_stack: [Stack<StackCell, 128>; 8],
    heap_mem: DualPortMem<DataFlowPkt<App>>,
    holder: (Dest, bool, DataFlowPkt<App>), // (destination, valid, app)
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

    pub fn to_reducer_valid(&self) -> bool {
        match self.holder.0 {
            Dest::ToReducer => self.holder.1,
            Dest::ToSelf => false,
        }
    }

    pub fn to_reducer_bits(&self) -> &DataFlowPkt<App> {
        &self.holder.2
    }

    pub fn to_self_valid(&self) -> bool {
        match self.holder.0 {
            Dest::ToReducer => false,
            Dest::ToSelf => self.holder.1,
        }
    }

    pub fn to_self_bits(&self) -> &DataFlowPkt<App> {
        &self.holder.2
    }
}

impl HwModule for DrfHeap {
    fn update_local(&mut self) {
        // handle port_a
        match *self.stm.value() {
            Stm::IDLE => {}
        }

        // handle port_b
        // always write fronzen applications
    }

    fn tick_children(&mut self) {
        self.stm.tick();
        for stk in &mut self.thread_stack {
            stk.tick();
        }
        self.heap_mem.tick();
    }
}
