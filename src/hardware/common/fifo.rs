use crate::hardware::utils::fire;
use crate::hw_module::{HwInput, HwModule};
use std::collections::VecDeque;

#[derive(Default)]
pub struct FIFOInput<T: Clone + Default> {
    pub in_valid: bool,
    pub out_ready: bool,
    pub din: T,
}

impl<T: Clone + Default> HwInput for FIFOInput<T> {
    fn default_input(&mut self) {
        self.in_valid = false;
        self.out_ready = false;
    }
}

/// FIFO, with size N. Supports pipelining when full.
pub struct FIFO<T: Clone + Default, const N: usize> {
    pub input: FIFOInput<T>,
    queue: VecDeque<T>,
}

impl<T: Clone + Default, const N: usize> FIFO<T, N> {
    pub fn new() -> Self {
        Self {
            input: Default::default(),
            queue: VecDeque::with_capacity(N),
        }
    }

    pub fn in_ready(&self) -> bool {
        self.queue.len() < N || fire(self.input.out_ready, self.out_valid())
    }

    pub fn out_valid(&self) -> bool {
        !self.queue.is_empty()
    }

    pub fn dout(&self) -> Option<&T> {
        match self.queue.front() {
            None => None,
            Some(v) => Some(v),
        }
    }
}

impl<T: Clone + Default, const N: usize> HwModule for FIFO<T, N> {
    fn update_local(&mut self) {
        if fire(self.out_valid(), self.input.out_ready) {
            self.queue.pop_front();
        }

        // to allow pipelining
        if fire(self.input.in_valid, self.in_ready()) {
            self.queue.push_back(self.input.din.clone());
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
        fifo.input.link(|input| {
            input.in_valid = true;
            input.din = i;
            input.out_ready = false;
        });
        fifo.tick();
    }

    // pipelining
    for i in 5..20 {
        fifo.input.link(|input| {
            input.in_valid = true;
            input.din = i;
            input.out_ready = true;
        });
        assert!(fire(fifo.input.in_valid, fifo.in_ready()));
        assert!(fire(fifo.out_valid(), fifo.input.out_ready));
        assert_eq!(fifo.dout(), Some(&(i - 4)));
        fifo.tick();
    }
}
