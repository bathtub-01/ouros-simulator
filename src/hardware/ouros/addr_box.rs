use super::config::{CONSUMERS, CONSUMERS_REDUCER};
use crate::hardware::common::Register;
use crate::hw_module::{HwInput, HwModule};

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
    feedback_regs: [Register<(bool, usize)>; CONSUMERS_REDUCER],
}

/// Shift registers for free addrs, can be consumed by the Reducer (0~6) and DrfHeap (7)
impl AddrBox {
    pub fn new() -> Self {
        Default::default()
    }

    /// ready-signal for requesting addr from GC
    pub fn addr_request(&self) -> bool {
        self.can_consume(CONSUMERS - 1)
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

    /// whether a slot can consume the free addr from upper stream in this cycle
    fn can_consume(&self, idx: usize) -> bool {
        let wants_consume = self.addr_fire(idx) || !self.addr_regs[idx].value().0;
        if idx == 0 {
            wants_consume
        } else {
            let higher_priority_wants =
                (0..idx).any(|i| self.addr_fire(i) || !self.addr_regs[i].value().0);
            wants_consume || higher_priority_wants
        }
    }

    pub fn consume_addr_valid(&self) -> [bool; CONSUMERS] {
        std::array::from_fn(|i| self.addr_regs[i].value().0)
    }

    pub fn consume_addr_bits(&self) -> [usize; CONSUMERS] {
        std::array::from_fn(|i| self.addr_regs[i].value().1)
    }

    pub fn feedback_valid(&self) -> bool {
        self.feedback_regs.iter().any(|reg| reg.value().0) || self.any_addr_fire()
    }

    pub fn feedback_bits(&self) -> usize {
        if self.addr_fire(CONSUMERS - 1) {
            self.addr_regs[CONSUMERS - 1].value().1
        } else if self.addr_fire(0) {
            self.addr_regs[0].value().1
        } else {
            let first = self.feedback_regs.iter().position(|reg| reg.value().0);
            match first {
                Some(i) => self.feedback_regs[i].value().1,
                None => 0,
            }
        }
    }

    fn feedback_chosen(&self) -> Option<usize> {
        if self.addr_fire(CONSUMERS - 1) {
            Some(CONSUMERS - 1)
        } else if self.addr_fire(0) {
            Some(0)
        } else {
            self.feedback_regs.iter().position(|reg| reg.value().0)
        }
    }
}

impl HwModule for AddrBox {
    fn update_local(&mut self) {
        for i in 0..CONSUMERS {
            // reg value shifting
            if self.can_consume(i) {
                if i == CONSUMERS - 1 {
                    self.addr_regs[i]
                        .connect(&(self.input.free_addr_valid, self.input.free_addr_bits));
                } else {
                    self.addr_regs[i].connect(&(
                        self.addr_regs[i + 1].value().0 && !self.addr_fire(i + 1),
                        self.addr_regs[i + 1].value().1,
                    ));
                }
            }
        }

        for i in 0..CONSUMERS_REDUCER {
            if self.addr_fire(i) {
                self.feedback_regs[i].connect(self.addr_regs[i].value());
            }
        }

        if let Some(i) = self.feedback_chosen() {
            if i < CONSUMERS_REDUCER {
                self.feedback_regs[i].connect(&(false, 0));
            }
        }
    }

    fn tick_children(&mut self) {
        self.addr_regs.iter_mut().for_each(|reg| {
            reg.tick();
        });
        self.feedback_regs.iter_mut().for_each(|reg| {
            reg.tick();
        });
    }
}

#[test]
fn addr_box_spec() {
    use super::garbage_collector::GbgCollector;
    let size = 1024;
    let from = 42;
    let mut gc = GbgCollector::new(size, from).init_freelist();
    let mut ab = AddrBox::new();
    let mut ctr = from;

    for _ in 0..CONSUMERS {
        ab.input.free_addr_valid = gc.addr_out_valid();
        ab.input.free_addr_bits = gc.addr_out_bits();
        gc.input.addr_out_ready = ab.addr_request();
        gc.tick();
        ab.tick();
        ctr = ctr + 1;
    }

    println!("{:?}", ab.consume_addr_valid());
    println!("{:?}", ab.consume_addr_bits());
}
