// This file contains the definition for simulatable hardware modules.

// Simulation is based on state-transfer semantics. The state of *next* cycle
// only depends on the state of *this* cycle (A module with pure combinatory
// circuit cannot be expressed in this framework). This design enables updating
// all the modules concurrently.

// Usage of a hardware module:
// At each cycle, hw module fields represent states in *this* cycle.
// To go to the next cycle, first use `link` to set `input` port values
// at *this* cycle. Then use `tick` to update states to *next* cycle.
//
// At the end of `tick`, tick all sub-modules to turn them to *next* cycle.
//
// The outputs of hardware modules are implemented as methods. They are always
// derived from current states.

/// At a certain clock cycle, it contains states of the input ports
pub trait HwInput {
    /// Update the input state at this cycle.
    fn link(&mut self, new_input: impl FnOnce(&mut Self)) {
        new_input(self);
    }

    /// Default input of this module
    fn default_input(&mut self) {}
}

/// The behavior of a hardware module at each clock cycle:
///   * `update_local`: update its local states (`input` should first be setup
///     through `link_input`)
///   * `tick_children`: tick all the sub-modules (can run in parallel)
///
/// By calling `tick` at cycle `n`, the whole module is **updated**. The `local`
/// states are now in cycle `n+1`.
pub trait HwModule {
    fn update_local(&mut self);
    fn update_stat(&mut self) {}
    fn tick_children(&mut self);

    /// After `input` get setup, use `tick` to update local states.
    fn tick(&mut self) {
        let _span = tracy_client::span!("tick");
        self.update_stat(); // now `input` and `local` are in the same cycle
        self.update_local();
        self.tick_children();
    }
}
