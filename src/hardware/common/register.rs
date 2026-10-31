use crate::{error, hw_module::HwModule};
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
    fn update_local(&mut self) -> Result<(), error::sim::TickHw> {
        self.value = self.input.clone();
        Ok(())
    }
    fn tick_children(&mut self) -> std::result::Result<(), error::sim::TickHw> {
        Ok(())
    }

    fn tick(&mut self) -> std::result::Result<(), error::sim::TickHw> {
        self.update_local()
    }
}
