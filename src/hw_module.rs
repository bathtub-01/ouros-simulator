// This file contains the definition for simulatable hardware modules.

// Simulation is based on state-transfer semantics. The state of *next* cycle
// only depends on the state of *this* cycle (A module with pure combinatory
// circuit cannot be expressed in this framework). This design enables updating
// all the modules concurrently.

// Usage of a hardware module:
// At each cycle, `local` represent states in *this* cycle.
// To go to the next cycle, first use `link_input` to set `input` port values
// at *this* cycle. Then use `tick` to update `local` to *next* cycle.
//
// At the end of `tick`, tick all sub-modules to turn them to *next* cycle.
//
// The outputs of hardware modules are implemented as methods. They are always
// derived from current `input` and `local` states.

/// At a certain clock cycle, hardware contains TWO states:
///   * `input`: state of its input ports
///   * `local`: the module's local state
#[derive(Default)]
pub struct HwStates<I: Default, L: Default> {
    pub input: I,
    pub local: L,
}

impl<I: Default, L: Default> HwStates<I, L> {
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
///   * `tick_children`: tick all the sub-modules (can run in parallel)
///
/// By calling `tick` at cycle `n`, the whole module is **updated**. The `local`
/// states are now in cycle `n+1`.
pub trait HwModule {
    fn update_local(&mut self);
    fn tick_children(&mut self);

    /// After `input` get setup, use `tick` to update local states.
    fn tick(&mut self) {
        self.update_local();
        self.tick_children();
    }
}
