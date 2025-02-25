use crate::hw_module::HwModule;

struct Bumper {
    input: bool,
    local: u32,
    output: u32,
}

impl HwModule for Bumper {
    type Input = bool;
    type Local = u32;
    type Output = u32;

    fn update_input(&mut self, i: bool) {
        self.input = i;
    }

    fn update_local(&mut self) {
        self.local = self.local + 1
    }

    fn update_output(&mut self) -> u32 {
        self.output = self.local;
        self.output
    }
}
