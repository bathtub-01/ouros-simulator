// Top level module of the Ouros core.

use crate::hardware::common::{Arbiter, FIFO};
use crate::hw_module::{HwInput, HwModule};

use super::deref_heap::DrfHeap;
use super::program::{ActiveApp, FrozenApp};
use super::reducer::Reducer;

#[derive(Default)]
struct OurosCoreInput {
    start: bool,
}

impl HwInput for OurosCoreInput {}

// TODO: add more detailed FIFO depth configuration.
struct OurosCore {
    input: OurosCoreInput,
    dheap: DrfHeap,
    reducer: Reducer,

    buffers_dheap_a_0: FIFO<ActiveApp, 8>,
    buffers_dheap_a_1: FIFO<ActiveApp, 8>,
    arbiter_dheap_a: Arbiter<ActiveApp, 2>,

    buffers_dheap_b_0: FIFO<FrozenApp, 8>,
    buffers_dheap_b_1: FIFO<FrozenApp, 8>,
    buffers_dheap_b_2: FIFO<FrozenApp, 8>,
    arbiter_dheap_b: Arbiter<FrozenApp, 3>,

    buffers_reducer_0: FIFO<ActiveApp, 8>,
    buffers_reducer_1: FIFO<ActiveApp, 8>,
    arbiter_reducer: Arbiter<ActiveApp, 2>,
}

impl OurosCore {
    pub fn new() -> Self {
        Self {
            input: Default::default(),
            dheap: DrfHeap::new(128),
            reducer: Reducer::new(),

            buffers_dheap_a_0: FIFO::new(),
            buffers_dheap_a_1: FIFO::new(),
            arbiter_dheap_a: Arbiter::new(),

            buffers_dheap_b_0: FIFO::new(),
            buffers_dheap_b_1: FIFO::new(),
            buffers_dheap_b_2: FIFO::new(),
            arbiter_dheap_b: Arbiter::new(),

            buffers_reducer_0: FIFO::new(),
            buffers_reducer_1: FIFO::new(),
            arbiter_reducer: Arbiter::new(),
        }
    }

    pub fn done(&self) -> bool {
        self.dheap.done()
    }
}

fn assign_some<T: Clone>(sink: &mut T, source: Option<&T>) {
    match source {
        None => {}
        Some(v) => {
            *sink = v.clone();
        }
    }
}

/// Connects input buffers of the arbiter
fn buffers_arbiter<T: Clone + Default, const N: usize, const A: usize>(
    buffers: [&mut FIFO<T, N>; A],
    arbiter: &mut Arbiter<T, A>,
) {
    // first handle arbiter's inputs
    arbiter.input.link(|input| {
        for (i, b) in buffers.iter().enumerate() {
            assign_some(&mut input.in_bits[i], b.dout());
            input.in_valid[i] = b.out_valid();
        }
    });
    let select = arbiter.select();
    for (i, b) in buffers.into_iter().enumerate() {
        b.input.out_ready = arbiter.in_ready(i, select);
    }
}

impl HwModule for OurosCore {
    fn update_local(&mut self) {
        // connect buffers to arbiters
        buffers_arbiter(
            [&mut self.buffers_dheap_a_0, &mut self.buffers_dheap_a_1],
            &mut self.arbiter_dheap_a,
        );
        buffers_arbiter(
            [
                &mut self.buffers_dheap_b_0,
                &mut self.buffers_dheap_b_1,
                &mut self.buffers_dheap_b_2,
            ],
            &mut self.arbiter_dheap_b,
        );
        buffers_arbiter(
            [&mut self.buffers_reducer_0, &mut self.buffers_reducer_1],
            &mut self.arbiter_reducer,
        );

        // connect arbiters as components' input

        // connect components' output to buffers
    }

    fn tick_children(&mut self) {
        self.dheap.tick();
        self.reducer.tick();

        self.buffers_dheap_a_0.tick();
        self.buffers_dheap_a_1.tick();
        self.arbiter_dheap_a.tick();

        self.buffers_dheap_b_0.tick();
        self.buffers_dheap_b_1.tick();
        self.buffers_dheap_b_2.tick();
        self.arbiter_dheap_b.tick();

        self.buffers_reducer_0.tick();
        self.buffers_reducer_1.tick();
        self.arbiter_reducer.tick();
    }
}
