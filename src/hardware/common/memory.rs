use crate::hardware::common::Register;
use crate::hw_module::{HwModule, HwStates};

#[derive(Default)]
struct MemInput<T: Clone + Default> {
    is_write: bool,
    addr: usize,
    din: T,
}

#[derive(Default)]
struct MemOutput<T: Clone + Default> {
    dout: T,
}

#[derive(Default)]
struct SingleLocal<T: Clone + Default> {
    ram: Vec<T>,
}

/// Synchronous single port read-write memory
pub struct SinglePortMem<T: Clone + Default> {
    states: HwStates<MemInput<T>, SingleLocal<T>, MemOutput<T>>,
}

impl<T: Default + Clone> SinglePortMem<T> {
    fn new(depth: usize) -> Self {
        Self {
            states: HwStates {
                input: Default::default(),
                local: SingleLocal {
                    ram: vec![T::default(); depth],
                },
                output: Default::default(),
            },
        }
    }
}

impl<T: Default + Clone> HwModule for SinglePortMem<T> {
    fn update_local(&mut self) {
        let input = &self.states.input;
        let local = &mut self.states.local;

        if input.is_write {
            local.ram[input.addr] = input.din.clone();
        }
    }

    fn tick_children(&mut self) {}

    fn gen_output(&mut self) {
        let input = &self.states.input;
        let local = &self.states.local;
        let output = &mut self.states.output;

        if !input.is_write {
            output.dout = local.ram[input.addr].clone();
        }
    }
}

#[test]
fn single_port_mem_spec() {
    let mut mem: SinglePortMem<u32> = SinglePortMem::new(1024);

    for i in 50..100 {
        mem.states.link_input(|input| {
            input.addr = i;
            input.is_write = true;
            input.din = i as u32 + 100;
        });
        mem.tick();
    }

    for i in 50..100 {
        mem.states.link_input(|input| {
            input.addr = i;
            input.is_write = false;
        });
        mem.tick();
        assert_eq!(mem.states.output.dout, i as u32 + 100);
    }
}

#[derive(Default)]
struct DualInput<T: Clone + Default> {
    port_a: MemInput<T>,
    port_b: MemInput<T>,
}

#[derive(Default)]
struct DualOutput<T: Clone + Default> {
    port_a: MemOutput<T>,
    port_b: MemOutput<T>,
}

#[derive(Default)]
struct DualLocal<T: Clone + Default> {
    ram: Vec<T>,
}

/// Synchronous dual port read-write memory.
/// Read-after-write for the same address.
pub struct DualPortMem<T: Clone + Default> {
    states: HwStates<DualInput<T>, DualLocal<T>, DualOutput<T>>,
}

impl<T: Clone + Default> DualPortMem<T> {
    fn new(depth: usize) -> Self {
        Self {
            states: HwStates {
                input: Default::default(),
                local: DualLocal {
                    ram: vec![T::default(); depth],
                },
                output: Default::default(),
            },
        }
    }
}

impl<T: Clone + Default> HwModule for DualPortMem<T> {
    fn update_local(&mut self) {
        let input = &self.states.input;
        let local = &mut self.states.local;

        assert!(
            !(input.port_a.is_write
                && input.port_b.is_write
                && input.port_a.addr == input.port_b.addr),
            "DualPortMem: writing on the same addr is not allowed."
        );

        if input.port_a.is_write {
            local.ram[input.port_a.addr] = input.port_a.din.clone();
        }

        if input.port_b.is_write {
            local.ram[input.port_b.addr] = input.port_b.din.clone();
        }
    }

    fn tick_children(&mut self) {}

    fn gen_output(&mut self) {
        let input = &self.states.input;
        let local = &self.states.local;
        let output = &mut self.states.output;

        if !input.port_a.is_write {
            output.port_a.dout = local.ram[input.port_a.addr].clone();
        }

        if !input.port_b.is_write {
            output.port_b.dout = local.ram[input.port_b.addr].clone();
        }
    }
}

#[test]
fn dual_port_mem_spec() {
    let mut mem: DualPortMem<u32> = DualPortMem::new(1024);

    // Common read/write
    for i in 50..100 {
        mem.states.link_input(|input| {
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
        mem.states.link_input(|input| {
            input.port_a.addr = i;
            input.port_a.is_write = false;
            input.port_b.addr = i + 200;
            input.port_b.is_write = false;
        });
        mem.tick();
        assert_eq!(mem.states.output.port_a.dout, i as u32 + 100);
        assert_eq!(mem.states.output.port_b.dout, i as u32 + 100);
    }

    // Read-after-write
    for i in 500..600 {
        mem.states.link_input(|input| {
            input.port_a.addr = i;
            input.port_a.is_write = true;
            input.port_a.din = i as u32 + 100;
            input.port_b.addr = i;
            input.port_b.is_write = false;
        });
        mem.tick();
        assert_eq!(mem.states.output.port_b.dout, i as u32 + 100);
    }

    for i in 500..600 {
        mem.states.link_input(|input| {
            input.port_b.addr = i;
            input.port_b.is_write = true;
            input.port_b.din = i as u32 - 100;
            input.port_a.addr = i;
            input.port_a.is_write = false;
        });
        mem.tick();
        assert_eq!(mem.states.output.port_a.dout, i as u32 - 100);
    }
}
