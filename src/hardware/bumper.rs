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
pub struct BumperOutput {
    res: u32,
    stm: bool,
}

#[derive(Default)]
pub struct Bumper {
    states: HwStates<BumperInput, BumperLocal, BumperOutput>,
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

    fn gen_output(&mut self) {
        self.states.output.res = *self.states.local.counter.value();
        self.states.output.stm = *self.states.local.running.value();
    }
}

impl Bumper {
    pub fn new() -> Self {
        Default::default()
    }
}

#[test]
fn bumper_spec() {
    let mut bumper = Bumper::new();

    bumper.states.link_input(|input| {
        input.start = true;
    });

    bumper.tick();
    assert_eq!(bumper.states.output.stm, true);
    bumper.tick();
    assert_eq!(bumper.states.output.res, 1);
    bumper.tick();
    assert_eq!(bumper.states.output.res, 2);

    bumper.states.link_input(|input| {
        input.start = false;
    });

    bumper.tick();
    assert_eq!(bumper.states.output.stm, false);
    assert_eq!(bumper.states.output.res, 3);

    bumper.tick();
    assert_eq!(bumper.states.output.res, 3);
}
