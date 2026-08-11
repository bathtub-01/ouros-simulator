use crate::hw_module::HwModule;
use vstd::prelude::*;

#[derive(Default)]
pub struct Register<V: Clone + Default> {
    pub input: V,
    value: V,
}

impl<V: Clone + Default> Register<V> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn init(v: V) -> Self {
        Self {
            input: v.clone(),
            value: v,
        }
    }
    pub fn connect(&mut self, v: &V) {
        self.input = v.clone();
    }
    pub fn value(&self) -> &V {
        &self.value
    }
}
impl<V: Clone + Default> HwModule for Register<V> {
    fn update_local(&mut self) -> Result<(), String> {
        self.value = self.input.clone();
        Ok(())
    }
    fn tick_children(&mut self) {}

    fn tick(&mut self) {
        self.update_local();
    }
}
