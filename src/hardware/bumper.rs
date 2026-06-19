// Example module to demonstrate the framework's usage.

use crate::hardware::common::Register;
use crate::hw_module::{HwInput, HwModule};

#[derive(Default)]
pub struct BumperInput {
    start: bool,
}

impl HwInput for BumperInput {}

#[derive(Default)]
pub struct Bumper {
    input: BumperInput,
    running: Register<bool>,
    counter: Register<u32>,
}

impl HwModule for Bumper {
    fn update_local(&mut self) {
        // Update local state based on input
        self.running.connect(&self.input.start);

        if *self.running.value() {
            self.counter.connect(&(self.counter.value() + 1));
        }
    }

    fn tick_children(&mut self) {
        self.counter.tick();
        self.running.tick();
    }
}

impl Bumper {
    fn new() -> Self {
        Default::default()
    }

    fn res(&self) -> u32 {
        *self.counter.value()
    }

    fn stm(&self) -> bool {
        *self.running.value()
    }
}

#[test]
fn bumper_spec() {
    let mut bumper = Bumper::new();

    bumper.input.link(|input| {
        input.start = true;
    });

    bumper.tick();
    assert_eq!(bumper.stm(), true);
    bumper.tick();
    assert_eq!(bumper.res(), 1);
    bumper.tick();
    assert_eq!(bumper.res(), 2);

    bumper.input.link(|input| {
        input.start = false;
    });

    bumper.tick();
    assert_eq!(bumper.stm(), false);
    assert_eq!(bumper.res(), 3);

    bumper.tick();
    assert_eq!(bumper.res(), 3);
}
