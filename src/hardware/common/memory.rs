use crate::hw_module::{HwInput, HwModule};

#[derive(Default)]
pub struct MemInput<T: Clone + Default> {
    pub is_write: bool,
    pub addr: usize,
    pub din: T,
}

impl<T: Clone + Default> HwInput for MemInput<T> {
    fn default_input(&mut self) {
        self.is_write = false;
    }
}

/// Synchronous single port read-write memory
pub struct SinglePortMem<T: Clone + Default> {
    input: MemInput<T>,
    ram: Vec<T>,
    holder: T,
}

impl<T: Default + Clone> SinglePortMem<T> {
    pub fn new(depth: usize) -> Self {
        Self {
            input: Default::default(),
            ram: vec![T::default(); depth],
            holder: T::default(),
        }
    }

    pub fn dout(&self) -> &T {
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
pub struct DualInput<T: Clone + Default> {
    pub port_a: MemInput<T>,
    pub port_b: MemInput<T>,
}

impl<T: Clone + Default> HwInput for DualInput<T> {
    fn default_input(&mut self) {
        self.port_a.is_write = false;
        self.port_a.addr = 0;
        self.port_b.is_write = false;
        self.port_b.addr = 0;
    }
}

#[derive(Default)]
pub struct DualPortMemStat {
    pub a_reads: u32,
    pub a_writes: u32,
    pub b_reads: u32,
    pub b_writes: u32,
}

/// Synchronous dual port read-write memory.
/// Read-after-write for the same address.
pub struct DualPortMem<T: Clone + Default> {
    pub input: DualInput<T>,
    ram: Vec<T>,
    holder_a: T,
    holder_b: T,
    stat: DualPortMemStat,
}

impl<T: Clone + Default> DualPortMem<T> {
    pub fn new(depth: usize) -> Self {
        Self {
            input: Default::default(),
            ram: vec![T::default(); depth],
            holder_a: T::default(),
            holder_b: T::default(),
            stat: Default::default(),
        }
    }

    /// Set default contents in the ram.
    pub fn image(&mut self, img: &Vec<T>) {
        assert!(self.ram.len() >= img.len());
        self.ram.splice(0..img.len(), img.clone());
    }

    pub fn dout_a(&self) -> &T {
        &self.holder_a
    }

    pub fn dout_b(&self) -> &T {
        &self.holder_b
    }

    pub fn write_a(&mut self, addr: usize, din: T) {
        self.input.port_a.is_write = true;
        self.input.port_a.addr = addr;
        self.input.port_a.din = din;

        self.stat.a_writes += 1;
    }

    pub fn read_a(&mut self, addr: usize) {
        self.input.port_a.is_write = false;
        self.input.port_a.addr = addr;

        self.stat.a_reads += 1;
    }

    pub fn write_b(&mut self, addr: usize, din: T) {
        self.input.port_b.is_write = true;
        self.input.port_b.addr = addr;
        self.input.port_b.din = din;

        self.stat.b_writes += 1;
    }

    pub fn read_b(&mut self, addr: usize) {
        self.input.port_b.is_write = false;
        self.input.port_b.addr = addr;

        self.stat.b_reads += 1;
    }

    pub fn get_stat(&self) -> &DualPortMemStat {
        &self.stat
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
