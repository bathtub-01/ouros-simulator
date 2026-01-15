use std::cmp::min;

use crate::hardware::common::Register;
use crate::hw_module::{HwInput, HwModule};

const CONSUMERS: usize = 8;

#[derive(Default)]
pub struct AddrBoxInput {
    pub addr_consume: [bool; CONSUMERS], // addr consumed by demander
    pub free_addr_bits: usize,           // free addr from GC
    pub free_addr_valid: bool,
}

impl HwInput for AddrBoxInput {}

#[derive(Default)]
pub struct AddrBox {
    pub input: AddrBoxInput,
    addr_regs: [Register<(bool, usize)>; CONSUMERS], // 1 for DrfHeap, 7 for Reducer
}

impl AddrBox {
    pub fn new() -> Self {
        Default::default()
    }

    /// ready-signal for requesting addr from GC
    pub fn addr_request(&self) -> bool {
        self.input.addr_consume.iter().any(|&b| b)
            || self.addr_regs.iter().any(|reg| !reg.value().0)
    }

    fn addr_fire(&self, idx: usize) -> bool {
        self.addr_regs[idx].value().0 && self.input.addr_consume[idx]
    }

    fn any_addr_fire(&self) -> bool {
        self.addr_regs
            .iter()
            .zip(self.input.addr_consume)
            .map(|(reg, csm)| reg.value().0 && csm)
            .any(|b| b)
    }

    /// whether a slot can consume the free addr in this cycle
    /// idx-0 has the highest priority; idx-7 has the lowest priority
    fn can_consume(&self, idx: usize) -> bool {
        let wants_consume = self.addr_fire(idx) || !self.addr_regs[idx].value().0;
        if idx == 0 {
            wants_consume
        } else {
            let higher_priority_wants =
                (0..idx).any(|i| self.addr_fire(i) || !self.addr_regs[i].value().0);
            wants_consume && !higher_priority_wants
        }
    }

    pub fn consume_addr_valid(&self) -> [bool; CONSUMERS] {
        std::array::from_fn(|i| self.addr_regs[i].value().0)
    }

    pub fn consume_addr_bits(&self) -> [usize; CONSUMERS] {
        std::array::from_fn(|i| self.addr_regs[i].value().1)
    }
}

impl HwModule for AddrBox {
    fn update_local(&mut self) {
        let mut used: bool = false;
        // this should be a property: only one of the regs can get the result
        for i in 0..self.addr_regs.len() {
            // clear the valid bit
            if self.addr_fire(i) {
                self.addr_regs[i].connect(&(false, 0));
            }

            // consume the free addr
            if self.can_consume(i) && self.input.free_addr_valid {
                assert!(!used);
                self.addr_regs[i].connect(&(true, self.input.free_addr_bits));
                used = true;
            }
        }
    }

    fn tick_children(&mut self) {
        self.addr_regs.iter_mut().for_each(|reg| {
            reg.tick();
        });
    }
}

#[test]
fn addr_box_spec_init() {
    use super::garbage_collector::GbgCollector;
    let size = 1024;
    let from = 42;
    let mut gc = GbgCollector::new(size, from).init_freelist();
    let mut ab = AddrBox::new();
    let mut ctr = from;

    for i in 0..CONSUMERS {
        ab.input.free_addr_valid = gc.addr_out_valid();
        ab.input.free_addr_bits = gc.addr_out_bits();
        gc.input.addr_out_ready = ab.addr_request();
        gc.tick();
        ab.tick();
        assert!(ab.consume_addr_valid()[i]);
        assert_eq!(ab.consume_addr_bits()[i], ctr);
        ctr = ctr + 1;
    }

    use rand::Rng;
    let mut rng = rand::rng();

    for _ in 0..800 {
        let consume: [bool; CONSUMERS] = rng.random();
        ab.input.free_addr_valid = gc.addr_out_valid();
        ab.input.free_addr_bits = gc.addr_out_bits();
        ab.input.addr_consume = consume.clone();
        // gc should be assigned later ab's assignment!
        gc.input.addr_out_ready = ab.addr_request();
        let pre = ab.consume_addr_valid().iter().position(|&b| !b);
        gc.tick();
        ab.tick();
        if consume.iter().any(|&b| b) {
            let hi_prio = consume.iter().position(|&b| b).unwrap();
            let now = match pre {
                Some(i) => min(hi_prio, i),
                None => hi_prio,
            };
            assert!(ab.consume_addr_valid()[now]);
            assert_eq!(ab.consume_addr_bits()[now], ctr);
        }
        ctr = ctr + 1;
    }
}
