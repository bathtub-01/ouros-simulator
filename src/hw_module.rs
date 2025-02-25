// This file contains the definition for simulatable hardware modules

/// At a certain clock cycle, hardware contains THREE states:
///   * `input`: state of its input ports
///   * `local`: the module's local state
///   * `output`: state of its output ports
///
/// The behavior of a hardware module at each clock cycle:
///   * `update_input`: update its `input` port state
///   * `update_local`: update its local `state`
///   * `update_output`: update its `output` port state
///
/// By calling `tick`, the whole module is **updated**. Specifically,
/// the state of `input`, `local` and `output` turns from "values in
/// this cycle" to "values in the upcoming cycle".
pub trait HwModule {
    type Input;
    type Local;
    type Output;

    fn update_input(&mut self, i: Self::Input);
    fn update_local(&mut self);
    fn update_output(&mut self) -> Self::Output;

    fn tick(&mut self, i: Self::Input) -> Self::Output {
        self.update_input(i);
        self.update_local();
        self.update_output()
    }
}
