// This file contains the definition for simulatable hardware modules.

// Simulation is based on state-transfer semantics. The state of *next* cycle
// only depends on the state of *this* cycle (A module with pure combinatory
// circuit cannot be expressed in this framework). This design enables updating
// all the modules concurrently.

// Usage of a hardware module:
// At each cycle, `local` and `output` represent states in *this* cycle.
// To go to the next cycle, first use `link_input` to set `input` port values
// at *this* cycle. Then use `tick` to update `local` and `output` to *next* cycle.

// In `tick`, `output` from sub-modules are all available as they are  in *this*
// cycle. At the end of `tick`, tick all sub-modules to turn them to *next* cycle.

/// At a certain clock cycle, hardware contains THREE states:
///   * `input`: state of its input ports
///   * `local`: the module's local state
///   * `output`: state of its output ports
#[derive(Default)]
pub struct HwStates<I: Default, L: Default, O: Default> {
    pub input: I,
    pub local: L,
    pub output: O,
}

impl<I: Default, L: Default, O: Default> HwStates<I, L, O> {
    /// External world talks to the hw module through this.
    pub fn link_input(&mut self, new_input: impl FnOnce(&mut I)) {
        new_input(&mut self.input);
    }

    pub fn new() -> Self {
        Default::default()
    }
}

/// The behavior of a hardware module at each clock cycle:
///   * `update_local`: update its local `state` (`input` should first be setup
///     through `link_input`)
///   * `gen_output`: `output` ports are derived from local states
///
/// By calling `tick` at cycle `n`, the whole module is **updated**. The `local`
/// and `output` states are now in cycle `n+1`.
pub trait HwModule {
    fn update_local(&mut self);
    fn gen_output(&mut self);

    /// After `input` get setup, use `tick` to update local states and outputs.
    fn tick(&mut self) {
        self.update_local();
        self.gen_output();
    }
}
