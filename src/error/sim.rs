use std::fmt;

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
    StatisticsDerefHeap(local::DerefHeap, SimState),
    #[error("TickHw Reducer: {0} {1}")]
    Reducer(local::Reducer, SimState),
    #[error("TickHw RArbiter: {0} {1}")]
    Rarbiter(local::RArbiter, SimState),
    #[error("TickHw Ouros core: {0} {1}")]
    Core(local::Core, SimState),
    #[error("TickHw Garbage Collector: {0} {1}")]
    Gbc(local::Gbc, SimState),
    #[error("TickHw Alu: {0} {1}")]
    Alu(local::Alu, SimState),
    #[error("TickHw DerefHeap: {0} {1}")]
    DerefHeap(local::DerefHeap, SimState),
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

pub mod local {
    // use crate::Atom;

    use crate::hardware;

    use super::*;

    /// RArbiter errors
    #[derive(Debug, thiserror::Error)]
    pub enum RArbiter {
        #[error("Erroneous Selection: Could not select in RArbiter")]
        Selection,
    }

    /// RArbiter errors
    #[derive(Debug, thiserror::Error)]
    pub enum Reducer {
        #[error("Invalid Atom ({invalid_atom}) for parent ({parent})")]
        InvalidAtom {
            parent: hardware::ouros::program::Atom,
            invalid_atom: hardware::ouros::program::Atom,
        },
        #[error("Unimplemented Atom ({unimplemented})")]
        UnimplementedAtom {
            unimplemented: hardware::ouros::program::Atom,
        },
        #[error("Spine leaked ({spine_value}) -> spine_not_ready? {spine_not_ready}")]
        SpineLeaked {
            spine_value: crate::hardware::ouros::reducer::Stm,
            spine_not_ready: bool,
        },
    }

    /// Core errors
    #[derive(Debug, thiserror::Error)]
    pub enum Core {
        #[error("DereHead Failed {err} ({context})")]
        DereHeapFailed { err: DerefHeap, context: String },
        #[error("Alu Failed {err} ({context})")]
        AluFailed { err: Alu, context: String },
    }

    /// DerefHeapNew errors
    #[derive(Debug, thiserror::Error)]
    pub enum DerefHeap {
        // FIX: to specialise errors
        #[error("String based error: {0}")]
        Str(String),
    }

    /// Garbage collector errors
    #[derive(Debug, thiserror::Error)]
    pub enum Gbc {
        #[error("Broken WorkList! addr: {addr}")]
        BrokenWorkList { addr: usize },
    }

    /// ALU component unit
    #[derive(Debug, thiserror::Error)]
    pub enum Alu {
        // FIX: to specialise errors
        #[error("String based error: {0}")]
        Str(String),
    }
}
