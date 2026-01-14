use crate::hw_module::HwModule;
use vstd::prelude::*;

verus! {
#[derive(Default)]
pub struct Register<V: Clone + Default> {
    pub input: V,
    pub value: V,
}

pub assume_specification<V> [<Register<V> as Default>::default] () -> Register<V>
           where
           V: Default + Clone + Default,;

// Stack elements like `u32` also implements `Clone`, and
// that will be zero-cost.
impl<V: Clone + Default> Register<V> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn connect(&mut self, v: &V) {
        self.input = v.clone();
    }

    pub fn value(&self) -> (res: &V)
        ensures
        *res == self.value
    {
        &self.value
    }
}

impl<V: Clone + Default> HwModule for Register<V> {
    fn update_local(&mut self)
    {
        self.value = self.input.clone();
    }

    fn tick_children(&mut self) {}

    fn tick(&mut self) {
        self.update_local();
    }
}
}
