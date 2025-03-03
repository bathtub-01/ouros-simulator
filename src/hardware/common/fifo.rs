use crate::hardware::utils::fire;
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
        if fire(self.out_valid(), input!(self).out_ready) {
            local!(self).queue.pop_front();
        }

        // to allow pipelining
        if fire(input!(self).in_valid, self.in_ready()) {
            local!(self).queue.push_back(input!(self).din.clone());
        }
    }

    fn tick_children(&mut self) {}
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
    for i in 5..20 {
        fifo.states.link_input(|input| {
            input.in_valid = true;
            input.din = i;
            input.out_ready = true;
        });
        assert!(fire(fifo.states.input.in_valid, fifo.in_ready()));
        assert!(fire(fifo.out_valid(), fifo.states.input.out_ready));
        assert_eq!(fifo.dout(), Some(&(i - 4)));
        fifo.tick();
    }
}
