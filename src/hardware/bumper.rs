// Example module to demonstrate the framework's usage.

use crate::hardware::common::Register;
use crate::hw_module::{HwModule, HwStates};

#[derive(Default)]
pub struct BumperInput {
    start: bool,
}

#[derive(Default)]
pub struct BumperLocal {
    running: Register<bool>,
    counter: Register<u32>,
}

#[derive(Default)]
pub struct Bumper {
    states: HwStates<BumperInput, BumperLocal>,
}

impl HwModule for Bumper {
    fn update_local(&mut self) {
        // Update local state based on input
        let input = &self.states.input;
        let local = &mut self.states.local;

        local.running.connect(&input.start);

        if *local.running.value() {
            local.counter.connect(&(local.counter.value() + 1));
        }
    }

    fn tick_children(&mut self) {
        self.states.local.counter.tick();
        self.states.local.running.tick();
    }
}

impl Bumper {
    fn new() -> Self {
        Default::default()
    }

    fn res(&self) -> u32 {
        self.states.local.counter.value().clone()
    }

    fn stm(&self) -> bool {
        self.states.local.running.value().clone()
    }
}

#[test]
fn bumper_spec() {
    let mut bumper = Bumper::new();

    bumper.states.link_input(|input| {
        input.start = true;
    });

    bumper.tick();
    assert_eq!(bumper.stm(), true);
    bumper.tick();
    assert_eq!(bumper.res(), 1);
    bumper.tick();
    assert_eq!(bumper.res(), 2);

    bumper.states.link_input(|input| {
        input.start = false;
    });

    bumper.tick();
    assert_eq!(bumper.stm(), false);
    assert_eq!(bumper.res(), 3);

    bumper.tick();
    assert_eq!(bumper.res(), 3);
}
