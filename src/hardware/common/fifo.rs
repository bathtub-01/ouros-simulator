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

#[derive(Default)]
pub struct FIFOStat {
    pub length_per_cycle: Vec<u8>,
}

/// FIFO, with size N. Optionally support pipelining.
pub struct FIFO<T: Clone + Default, const N: usize, const P: bool> {
    pub input: FIFOInput<T>,
    pub queue: VecDeque<T>,
    stat: FIFOStat,
    record_stat: bool,
}

impl<T: Clone + Default, const N: usize, const P: bool> FIFO<T, N, P> {
    pub fn new() -> Self {
        Self {
            input: Default::default(),
            queue: VecDeque::with_capacity(N),
            stat: Default::default(),
            record_stat: Default::default(),
        }
    }

    pub fn record_stat(mut self, b: bool) -> Self {
        self.record_stat = b;
        self
    }

    pub fn in_ready(&self) -> bool {
        self.queue.len() < N || (P && fire(self.input.out_ready, self.out_valid()))
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

    pub fn in_fire(&self) -> bool {
        fire(self.input.in_valid, self.in_ready())
    }

    pub fn out_fire(&self) -> bool {
        fire(self.input.out_ready, self.out_valid())
    }

    pub fn get_stat(&self) -> &FIFOStat {
        &self.stat
    }
}

impl<T: Clone + Default, const N: usize, const P: bool> HwModule for FIFO<T, N, P> {
    fn update_local(&mut self) {
        if fire(self.out_valid(), self.input.out_ready) {
            self.queue.pop_front();
        }

        if fire(self.input.in_valid, self.in_ready()) {
            self.queue.push_back(self.input.din.clone());
        }
    }

    fn update_stat(&mut self) {
        if self.record_stat {
            self.stat.length_per_cycle.push(self.queue.len() as u8);
        }
    }

    fn tick_children(&mut self) {}
}

#[test]
fn fifo_spec() {
    let mut fifo: FIFO<u32, 4, true> = FIFO::new();

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
