use crate::hardware::utils::fire;
use crate::hw_module::{HwInput, HwModule};

use super::Register;

#[derive(Default)]
pub struct RingInput<T: Clone + Default + PartialEq> {
    pub in_fire: bool,
    pub out_fire: bool,
    pub din: T,
    pub search: T,
}

impl<T: Clone + Default + PartialEq> HwInput for RingInput<T> {
    fn default_input(&mut self) {
        self.in_fire = false;
        self.out_fire = false;
    }
}

/// Ring queue, allows combinational searching
pub struct Ring<T: Clone + Default + PartialEq, const N: usize> {
    pub input: RingInput<T>,
    pub reg_bank: Vec<(bool, T)>,
    head: Register<usize>,
    tail: Register<usize>,
}

impl<T: Clone + Default + PartialEq, const N: usize> Ring<T, N> {
    pub fn new() -> Self {
        Self {
            input: Default::default(),
            reg_bank: vec![Default::default(); N],
            head: Default::default(),
            tail: Default::default(),
        }
    }

    pub fn found(&self) -> bool {
        // mask the firing case
        !(self.input.out_fire && self.reg_bank[*self.head.value()].1 == self.input.search)
            && self
                .reg_bank
                .iter()
                .any(|(valid, value)| *valid && *value == self.input.search)
    }

    pub fn put(&mut self, v: &T) {
        self.input.in_fire = true;
        self.input.din = v.clone();
    }
}

impl<T: Clone + Default + PartialEq, const N: usize> HwModule for Ring<T, N> {
    fn update_local(&mut self) {
        if self.input.out_fire {
            self.reg_bank[*self.head.value()] = (false, Default::default());
            self.head.connect(&((*self.head.value() + 1) % N));
        }
        if self.input.in_fire {
            self.reg_bank[*self.tail.value()] = (true, self.input.din.clone());
            self.tail.connect(&((*self.tail.value() + 1) % N));
        }
    }

    fn tick_children(&mut self) {
        self.head.tick();
        self.tail.tick();
    }
}

#[test]
fn ring_spec() {
    let mut ring: Ring<u32, 5> = Ring::new();

    ring.input.default_input();
    ring.tick();

    for i in 0..5 {
        ring.put(&(42 + i));
        ring.tick();
    }

    ring.input.default_input();
    for i in 0..5 {
        ring.input.search = 42 + i;
        assert!(ring.found());
    }

    for i in 0..3 {
        ring.input.out_fire = true;
        ring.input.search = 42 + i;
        assert!(!ring.found());
        ring.tick();
    }

    ring.input.default_input();
    for i in 3..5 {
        ring.input.search = 42 + i;
        assert!(ring.found());
    }
}
