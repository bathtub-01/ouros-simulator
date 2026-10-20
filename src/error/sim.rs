use std::fmt;

use crate::hardware::{common, ouros};

/// This is the snaphot of the state of the App_State that we are running
#[derive(Debug, Copy, Clone)]
pub struct SimState;

// TODO: parametise this with the error value to wrap it nicely! SimState<Error>
impl fmt::Display for SimState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SimState")
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TickHw {
    // #[error("TickHw Statistics: {0} {1}")]
    // Statistics(String, SimState),
    #[error("TickHw Statistics DerefHeap: {0} {1}")]
    StatisticsDerefHeap(ouros::deref_heap_new::Error, SimState),
    #[error("TickHw Reducer: {0} {1}")]
    Reducer(ouros::reducer::Error, SimState),
    #[error("TickHw RArbiter: {0} {1}")]
    Rarbiter(common::arbiter::Error, SimState),
    #[error("TickHw Ouros core: {0} {1}")]
    Core(ouros::ouros_core::Error, SimState),
    #[error("TickHw Garbage Collector: {0} {1}")]
    Gbc(ouros::garbage_collector::Error, SimState),
    #[error("TickHw Alu: {0} {1}")]
    Alu(ouros::alu::Error, SimState),
    #[error("TickHw DerefHeap: {0} {1}")]
    DerefHeap(ouros::deref_heap_new::Error, SimState),
}

impl TickHw {
    /// just to ensure that we always return a state
    fn get_state(self) -> SimState {
        match self {
            TickHw::StatisticsDerefHeap(_, state) => state,
            TickHw::Reducer(_, state) => state,
            TickHw::Rarbiter(_, state) => state,
            TickHw::Core(_, state) => state,
            TickHw::Gbc(_, state) => state,
            TickHw::Alu(_, state) => state,
            TickHw::DerefHeap(_, state) => state,
        }
    }
}
