use crate::hw_module::{HwInput, HwModule};

#[derive(Default)]
pub struct MemInput<T: Clone + Default> {
    pub enable: bool,
    pub is_write: bool,
    pub addr: usize,
    pub din: T,
}

impl<T: Clone + Default> HwInput for MemInput<T> {
    fn default_input(&mut self) {
        self.enable = false;
        self.is_write = false;
    }
}

/// Synchronous single port read-write memory
pub struct SinglePortMem<T: Clone + Default> {
    pub input: MemInput<T>,
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

    /// Set default contents in the ram.
    pub fn image(&mut self, img: &Vec<T>) {
        assert!(self.ram.len() >= img.len());
        self.ram.splice(0..img.len(), img.clone());
    }

    pub fn dout(&self) -> &T {
        &self.holder
    }

    pub fn write(&mut self, addr: usize, din: T) {
        self.input.enable = true;
        self.input.is_write = true;
        self.input.addr = addr;
        self.input.din = din;
    }

    pub fn read(&mut self, addr: usize) {
        self.input.enable = true;
        self.input.is_write = false;
        self.input.addr = addr;
    }
}

impl<T: Default + Clone> HwModule for SinglePortMem<T> {
    fn update_local(&mut self) {
        if self.input.enable {
            if self.input.is_write {
                self.ram[self.input.addr] = self.input.din.clone();
            } else {
                self.holder = self.ram[self.input.addr].clone();
            }
        }
    }

    fn tick_children(&mut self) {}
}

#[test]
fn single_port_mem_spec() {
    let mut mem: SinglePortMem<u32> = SinglePortMem::new(1024);

    for i in 50..100 {
        mem.write(i, i as u32 + 100);
        mem.tick();
    }

    for i in 50..100 {
        mem.read(i);
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
        self.port_a.enable = false;
        self.port_a.is_write = false;
        self.port_a.addr = 0;
        self.port_b.enable = false;
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
/// When a write happens, the old value at the address will be read out.
pub struct DualPortMem<T: Clone + Default> {
    pub input: DualInput<T>,
    pub ram: Vec<T>,
    holder_a: T,
    holder_b: T,
    stat: DualPortMemStat,
    pub record_stat: bool,
}

impl<T: Clone + Default> DualPortMem<T> {
    pub fn new(depth: usize) -> Self {
        Self {
            input: Default::default(),
            ram: vec![T::default(); depth],
            holder_a: T::default(),
            holder_b: T::default(),
            stat: Default::default(),
            record_stat: Default::default(),
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
        self.input.port_a.enable = true;
        self.input.port_a.is_write = true;
        self.input.port_a.addr = addr;
        self.input.port_a.din = din;
    }

    pub fn read_a(&mut self, addr: usize) {
        self.input.port_a.enable = true;
        self.input.port_a.is_write = false;
        self.input.port_a.addr = addr;
    }

    pub fn write_b(&mut self, addr: usize, din: T) {
        self.input.port_b.enable = true;
        self.input.port_b.is_write = true;
        self.input.port_b.addr = addr;
        self.input.port_b.din = din;
    }

    pub fn read_b(&mut self, addr: usize) {
        self.input.port_b.enable = true;
        self.input.port_b.is_write = false;
        self.input.port_b.addr = addr;
    }

    pub fn get_stat(&self) -> &DualPortMemStat {
        &self.stat
    }
}

impl<T: Clone + Default> HwModule for DualPortMem<T> {
    fn update_stat(&mut self) {
        if self.record_stat {
            if self.input.port_a.enable {
                if self.input.port_a.is_write {
                    self.stat.a_writes += 1;
                } else {
                    self.stat.a_reads += 1;
                }
            }

            if self.input.port_b.enable {
                if self.input.port_b.is_write {
                    self.stat.b_writes += 1;
                } else {
                    self.stat.b_reads += 1;
                }
            }
        }
    }

    fn update_local(&mut self) {
        assert!(
            !(self.input.port_a.is_write
                && self.input.port_b.is_write
                && self.input.port_a.addr == self.input.port_b.addr),
            "DualPortMem: writing on the same addr is not allowed."
        );

        // A bit ugly, but maintains read-after-write
        if self.input.port_a.is_write && self.input.port_a.enable {
            self.holder_a = self.ram[self.input.port_a.addr].clone();
            self.ram[self.input.port_a.addr] = self.input.port_a.din.clone();
        }

        if self.input.port_b.is_write && self.input.port_b.enable {
            self.holder_b = self.ram[self.input.port_b.addr].clone();
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
