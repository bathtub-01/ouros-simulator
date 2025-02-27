use crate::hardware::common::Register;
use crate::hw_module::{HwModule, HwStates};

#[derive(Default)]
pub struct MemInput<T: Clone + Default> {
    is_write: bool,
    addr: usize,
    din: T,
}

#[derive(Default)]
pub struct MemOutput<T: Clone + Default> {
    dout: T,
}

#[derive(Default)]
pub struct SingleLocal<T: Clone + Default> {
    ram: Vec<T>,
    holder: Register<T>,
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
                    holder: Default::default(),
                },
                output: Default::default(),
            },
        }
    }
}

impl<T: Default + Clone> HwModule for SinglePortMem<T> {
    type Output = MemOutput<T>;

    fn tick(&mut self) {
        let input = &self.states.input;
        let local = &mut self.states.local;

        if input.is_write {
            local.ram[input.addr] = input.din.clone();
        } else {
            local.holder.connect(&local.ram[input.addr]);
        }

        local.holder.tick();
    }

    fn gen_output(&mut self) -> &Self::Output {
        self.states.output.dout = self.states.local.holder.value().clone();

        &self.states.output
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
        assert_eq!(mem.gen_output().dout, i as u32 + 100);
    }
}

pub struct DualPortMem {}
