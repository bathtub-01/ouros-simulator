// This file contains the definition for simulatable hardware modules

/// At a certain clock cycle, hardware contains THREE states:
///   * `input`: state of its input ports
///   * `local`: the module's local state
///   * `output`: state of its output ports
///
#[derive(Default)]
pub struct HwStates<I: Default, L: Default, O: Default> {
    pub input: I,
    pub local: L,
    pub output: O,
}

impl<I: Default, L: Default, O: Default> HwStates<I, L, O> {
    /// External world talk to the hw module through this.
    pub fn link_input(&mut self, new_input: impl FnOnce(&mut I)) {
        new_input(&mut self.input);
    }

    pub fn new() -> Self {
        Default::default()
    }
}

/// The behavior of a hardware module at each clock cycle:
///   * `update_input`: update its `input` port state
///   * `update_local`: update its local `state`
///   * `update_output`: update its `output` port state
///
/// By calling `tick`, the whole module is **updated**. Specifically,
/// the state of `input`, `local` and `output` turns from "values in
/// this cycle" to "values in the upcoming cycle".
pub trait HwModule {
    type Output;

    fn tick(&mut self);
    fn gen_output(&mut self) -> &Self::Output;
}
