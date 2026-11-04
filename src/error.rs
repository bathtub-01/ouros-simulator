use std::fmt;

use rayon::ThreadPoolBuildError;

use crate::hardware::ouros::ouros_core::{OurosCore, OurosCoreStat, OurosCoreStatsError};

#[derive(Debug, thiserror::Error)]
pub enum Ouros {
    #[error("Error in simulation: {0}")]
    Simulation(#[from] Simulation),
    #[error("IO: {0}")]
    IO(#[from] std::io::Error),
    #[error("Initialising Rayon thread pool: {0}")]
    RayonThreadPool(#[from] ThreadPoolBuildError),
    #[error("Program lookup failed: {program_name}")]
    ProgramNotFound { program_name: String },
    #[error("Invalid CLI arguments: {reason}")]
    InvalidCliArgs { reason: String },
    #[error("Patterns are invalid: {reason}")]
    InvalidPatterns { reason: String },
}

impl From<(&OurosCore, String, sim::TickHw)> for Ouros {
    fn from((ouros_core, name, run): (&OurosCore, String, sim::TickHw)) -> Self {
        Ouros::Simulation(Simulation {
            stats: Box::new(ouros_core.get_stat().into()),
            run: SimulationRun::Run(Box::new(run)),
            name,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SimulationRun {
    #[error("Excedeed cycle limit of {0}")]
    CycleLimit(u32),

    #[error("Running Simulation: {0}")]
    Run(#[from] Box<sim::TickHw>),
}

#[derive(Debug, thiserror::Error)]
pub struct Simulation {
    pub stats: Box<OurosCoreStatsError>,
    pub run: SimulationRun,
    pub name: String,
}

impl fmt::Display for Simulation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Finish implementing this")
    }
}

pub mod sim {
    use std::fmt;

    use crate::hardware::{common, ouros};

    #[derive(Debug, thiserror::Error)]
    pub enum TickHw {
        #[error("TickHw Statistics DerefHeap: {0}")]
        StatisticsDerefHeap(ouros::deref_heap_new::Error),
        #[error("TickHw Reducer: {0}")]
        Reducer(ouros::reducer::Error),
        #[error("TickHw RArbiter: {0}")]
        Rarbiter(common::arbiter::Error),
        #[error("TickHw Ouros core: {0}")]
        Core(ouros::ouros_core::Error),
        #[error("TickHw Garbage Collector: {0}")]
        Gbc(ouros::garbage_collector::Error),
        #[error("TickHw Alu: {0}")]
        Alu(ouros::alu::Error),
        #[error("TickHw DerefHeap: {0}")]
        DerefHeap(ouros::deref_heap_new::Error),
        #[error("TickHw IO unit: {0}")]
        IOunit(String),
    }
}
