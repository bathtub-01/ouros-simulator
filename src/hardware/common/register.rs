use crate::hw_module::HwModule;

#[derive(Default)]
pub struct Register<V: Clone + Default> {
    input: V,
    value: V,
}

// Stack elements like `u32` also implements `Clone`, and
// that will be zero-cost.
impl<V: Clone + Default> Register<V> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn connect(&mut self, v: &V) {
        self.input = v.clone();
    }

    pub fn value(&self) -> &V {
        &self.value
    }
}

impl<V: Clone + Default> HwModule for Register<V> {
    fn update_local(&mut self) {
        self.value = self.input.clone();
    }

    fn tick_children(&mut self) {}

    fn gen_output(&mut self) {}
}
