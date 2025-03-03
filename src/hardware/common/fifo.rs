use crate::hardware::common::utils::fire;
use crate::hw_module::{HwModule, HwStates};
use crate::{input, local};
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

// #[derive(Default)]
// struct FIFOOut<T: Clone + Default> {
//     in_ready: bool,
//     out_valid: bool,
//     dout: T,
// }

/// FIFO, with size N. Supports pipelining when full.
pub struct FIFO<T: Clone + Default, const N: usize> {
    states: HwStates<FIFOInput<T>, FIFOLocal<T>>,
}

impl<T: Clone + Default, const N: usize> FIFO<T, N> {
    fn new() -> Self {
        Self {
            states: HwStates {
                input: Default::default(),
                local: FIFOLocal {
                    queue: VecDeque::with_capacity(N),
                },
            },
        }
    }

    fn in_ready(&self) -> bool {
        self.states.local.queue.len() < N || fire(self.states.input.out_ready, self.out_valid())
    }

    fn out_valid(&self) -> bool {
        !local!(self).queue.is_empty()
    }

    fn dout(&self) -> Option<&T> {
        match self.states.local.queue.front() {
            None => None,
            Some(v) => Some(&v),
        }
    }
}

impl<T: Clone + Default, const N: usize> HwModule for FIFO<T, N> {
    fn update_local(&mut self) {
        // let input = &self.states.input;
        // let local = &mut self.states.local;

        if fire(self.out_valid(), input!(self).out_ready) {
            local!(self).queue.pop_front();
        }

        // to allow pipelining
        if fire(input!(self).in_valid, self.in_ready()) {
            local!(self).queue.push_back(input!(self).din.clone());
        }
    }

    fn tick_children(&mut self) {}

    // fn gen_output(&mut self) {
    //     let input = &self.states.input;
    //     let local = &self.states.local;

    //     output.in_ready = fire(output.out_valid, input.out_ready) || local.queue.len() < N;

    //     match local.queue.front() {
    //         None => { /* output.dout will not be used */ }
    //         Some(v) => {
    //             output.dout = v.clone();
    //         }
    //     }
    //     output.out_valid = !local.queue.is_empty();
    // }
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
        // FIXME: need a way to express combinatory logic...
        //        maybe using reference fields?
        assert!(fire(fifo.states.input.in_valid, fifo.in_ready()));
        assert!(fire(fifo.out_valid(), fifo.states.input.out_ready));
        fifo.tick();
        assert_eq!(fifo.dout(), Some(&(i - 3)));
    }
}
