use crate::hw_module::{HwInput, HwModule};

#[derive(Default)]
struct MemInput<T: Clone + Default> {
    is_write: bool,
    addr: usize,
    din: T,
}

impl<T: Clone + Default> HwInput for MemInput<T> {}

/// Synchronous single port read-write memory
pub struct SinglePortMem<T: Clone + Default> {
    input: MemInput<T>,
    ram: Vec<T>,
    holder: T,
}

impl<T: Default + Clone> SinglePortMem<T> {
    fn new(depth: usize) -> Self {
        Self {
            input: Default::default(),
            ram: vec![T::default(); depth],
            holder: T::default(),
        }
    }

    fn dout(&self) -> &T {
        &self.holder
    }
}

impl<T: Default + Clone> HwModule for SinglePortMem<T> {
    fn update_local(&mut self) {
        if self.input.is_write {
            self.ram[self.input.addr] = self.input.din.clone();
        } else {
            self.holder = self.ram[self.input.addr].clone();
        }
    }

    fn tick_children(&mut self) {}
}

#[test]
fn single_port_mem_spec() {
    let mut mem: SinglePortMem<u32> = SinglePortMem::new(1024);

    for i in 50..100 {
        mem.input.link(|input| {
            input.addr = i;
            input.is_write = true;
            input.din = i as u32 + 100;
        });
        mem.tick();
    }

    for i in 50..100 {
        mem.input.link(|input| {
            input.addr = i;
            input.is_write = false;
        });
        mem.tick();
        assert_eq!(mem.dout(), &(i as u32 + 100));
    }
}

#[derive(Default)]
struct DualInput<T: Clone + Default> {
    port_a: MemInput<T>,
    port_b: MemInput<T>,
}

impl<T: Clone + Default> HwInput for DualInput<T> {}

/// Synchronous dual port read-write memory.
/// Read-after-write for the same address.
pub struct DualPortMem<T: Clone + Default> {
    input: DualInput<T>,
    ram: Vec<T>,
    holder_a: T,
    holder_b: T,
}

impl<T: Clone + Default> DualPortMem<T> {
    fn new(depth: usize) -> Self {
        Self {
            input: Default::default(),
            ram: vec![T::default(); depth],
            holder_a: T::default(),
            holder_b: T::default(),
        }
    }

    fn dout_a(&self) -> &T {
        &self.holder_a
    }

    fn dout_b(&self) -> &T {
        &self.holder_b
    }
}

impl<T: Clone + Default> HwModule for DualPortMem<T> {
    fn update_local(&mut self) {
        assert!(
            !(self.input.port_a.is_write
                && self.input.port_b.is_write
                && self.input.port_a.addr == self.input.port_b.addr),
            "DualPortMem: writing on the same addr is not allowed."
        );

        // A bit ugly, but maintains read-after-write
        if self.input.port_a.is_write {
            self.ram[self.input.port_a.addr] = self.input.port_a.din.clone();
        }

        if self.input.port_b.is_write {
            self.ram[self.input.port_b.addr] = self.input.port_b.din.clone();
        }

        if !self.input.port_a.is_write {
            self.holder_a = self.ram[self.input.port_a.addr].clone();
        }

        if !self.input.port_b.is_write {
            self.holder_b = self.ram[self.input.port_b.addr].clone();
        }
    }

    fn tick_children(&mut self) {}
}

#[test]
fn dual_port_mem_spec() {
    let mut mem: DualPortMem<u32> = DualPortMem::new(1024);

    // Common read/write
    for i in 50..100 {
        mem.input.link(|input| {
            input.port_a.addr = i;
            input.port_a.is_write = true;
            input.port_a.din = i as u32 + 100;
            input.port_b.addr = i + 200;
            input.port_b.is_write = true;
            input.port_b.din = i as u32 + 100;
        });
        mem.tick();
    }

    for i in 50..100 {
        mem.input.link(|input| {
            input.port_a.addr = i;
            input.port_a.is_write = false;
            input.port_b.addr = i + 200;
            input.port_b.is_write = false;
        });
        mem.tick();
        assert_eq!(mem.dout_a(), &(i as u32 + 100));
        assert_eq!(mem.dout_b(), &(i as u32 + 100));
    }

    // Read-after-write
    for i in 500..600 {
        mem.input.link(|input| {
            input.port_a.addr = i;
            input.port_a.is_write = true;
            input.port_a.din = i as u32 + 100;
            input.port_b.addr = i;
            input.port_b.is_write = false;
        });
        mem.tick();
        assert_eq!(mem.dout_b(), &(i as u32 + 100));
    }

    for i in 500..600 {
        mem.input.link(|input| {
            input.port_b.addr = i;
            input.port_b.is_write = true;
            input.port_b.din = i as u32 - 100;
            input.port_a.addr = i;
            input.port_a.is_write = false;
        });
        mem.tick();
        assert_eq!(mem.dout_a(), &(i as u32 - 100));
    }
}
