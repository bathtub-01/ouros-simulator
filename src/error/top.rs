use std::fmt;

use rayon::ThreadPoolBuildError;

use crate::hardware::ouros::ouros_core::{OurosCoreStat, OurosCoreStatsError};

#[derive(Debug, thiserror::Error)]
pub enum Ouros {
    #[error("Error in simulation: {0}")]
    Simulation(Simulation),
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

#[derive(Debug, thiserror::Error)]
pub enum SimulationRun {
    #[error("Excedeed cycle limit of {0}")]
    CycleLimit(u32),

    #[error("Running Simulation: {0}")]
    Run(#[from] Box<crate::error::sim::TickHw>),
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
