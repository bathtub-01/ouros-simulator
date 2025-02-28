use crate::hardware::common::utils::fire;
use crate::hw_module::{HwModule, HwStates};
use std::collections::VecDeque;

#[derive(Default)]
struct FIFOInput<T: Clone + Default> {
    in_valid: bool,
    out_ready: bool,
    din: T,
}

#[derive(Default)]
struct FIFOLocal<T: Clone + Default> {
    queue: VecDeque<T>,
}

#[derive(Default)]
struct FIFOOut<T: Clone + Default> {
    in_ready: bool,
    out_valid: bool,
    dout: T,
}

/// FIFO, with size N. Supports pipelining when full.
pub struct FIFO<T: Clone + Default, const N: usize> {
    states: HwStates<FIFOInput<T>, FIFOLocal<T>, FIFOOut<T>>,
}

impl<T: Clone + Default, const N: usize> FIFO<T, N> {
    fn new() -> Self {
        Self {
            states: HwStates {
                input: Default::default(),
                local: FIFOLocal {
                    queue: VecDeque::with_capacity(N),
                },
                output: Default::default(),
            },
        }
    }
}

impl<T: Clone + Default, const N: usize> HwModule for FIFO<T, N> {
    fn update_local(&mut self) {
        let input = &self.states.input;
        let local = &mut self.states.local;
        let output = &self.states.output;

        if fire(output.out_valid, input.out_ready) {
            local.queue.pop_front();
        }

        // to allow pipelining
        if input.in_valid && (output.in_ready || fire(output.out_valid, input.out_ready)) {
            local.queue.push_back(input.din.clone());
        }
    }

    fn tick_children(&mut self) {}

    fn gen_output(&mut self) {
        let input = &self.states.input;
        let local = &self.states.local;
        let output = &mut self.states.output;

        output.in_ready = fire(output.out_valid, input.out_ready) || local.queue.len() < N;

        match local.queue.front() {
            None => { /* output.dout will not be used */ }
            Some(v) => {
                output.dout = v.clone();
            }
        }
        output.out_valid = !local.queue.is_empty();
    }
}

#[test]
fn fifo_spec() {
    let mut fifo: FIFO<u32, 4> = FIFO::new();

    fifo.tick();

    // fill the fifo (1,2,3,4)
    for i in 1..10 {
        fifo.states.link_input(|input| {
            input.in_valid = true;
            input.din = i;
            input.out_ready = false;
        });
        fifo.tick();
    }

    // pipelining
    for i in 5..10 {
        fifo.states.link_input(|input| {
            input.in_valid = true;
            input.din = i;
            input.out_ready = true;
        });
        fifo.tick();
        assert_eq!(fifo.states.output.dout, i - 3);
    }
}
