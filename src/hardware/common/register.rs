use crate::hw_module::HwModule;
use vstd::prelude::*;

#[derive(Default)]
pub struct Register<V: Clone + Default> {
    pub input: V,
    pub value: V,
}
#[allow(non_snake_case)]
pub unsafe fn _verus_external_fn_specification_44__60__32_Register_32__60__32_V_32__62__32_as_32_Default_32__62__32__58__58__32_default<
    V,
>() -> Register<V>
where
    V: Default + Clone + Default,
{
    unsafe { <Register<V> as Default>::default() }
}
impl<V: Clone + Default> Register<V> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn init(v: V) -> Self {
        Self {
            input: Default::default(),
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
    fn update_local(&mut self) {
        self.value = self.input.clone();
    }
    fn tick_children(&mut self) {}

    fn tick(&mut self) {
        self.update_local();
    }
}
