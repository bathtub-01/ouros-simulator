use crate::hardware::common::utils::fire;
use crate::hardware::common::Register;
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
    holder: Register<T>,
}

#[derive(Default)]
struct FIFOOut<T: Clone + Default> {
    in_ready: bool,
    out_valid: bool,
    dout: T,
}

// FIFO, with size N
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
                    holder: Default::default(),
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

        if fire(input.in_valid, output.in_ready) {
            local.queue.push_back(input.din.clone());
        }

        match local.queue.front() {
            None => {}
            Some(v) => {
                local.holder.connect(v);
            }
        }

        if fire(output.out_valid, input.out_ready) {
            local.queue.pop_front();
        }
    }

    fn tick_children(&mut self) {
        self.states.local.holder.tick();
    }

    fn gen_output(&mut self) {
        let local = &self.states.local;
        let output = &mut self.states.output;

        output.in_ready = local.queue.len() < N;
        output.dout = local.holder.value().clone();
        output.out_valid = !local.queue.is_empty();
    }
}
