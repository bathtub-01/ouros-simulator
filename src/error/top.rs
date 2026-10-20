use std::fmt;

use rayon::ThreadPoolBuildError;

use crate::hardware::ouros::ouros_core::{OurosCoreStat, OurosCoreStatsError};

#[derive(Debug, thiserror::Error)]
pub enum Ouros {
    #[error("Program initialisation error: {0}")]
    Init(#[from] Initialisation),
    #[error("Pure simulation CLI mode: {0}")]
    SimulationMode(#[from] SimulationMode),
    #[error("Garbage collector CLI mode: {0}")]
    GarbageCollectorMode(#[from] GarbageCollectorMode),
    #[error("Benchmarking CLI mode: {0}")]
    BenchmarkingMode(#[from] BenchmarkingMode),
    #[error("Inspection CLI mode: {0}")]
    InspectionMode(#[from] InspectionMode),
}

pub fn report_ouros(e: Ouros) {
    dbg!(&e);
    // TODO: yet to report it properly
    // match e {}
}

#[derive(Debug, thiserror::Error)]
pub enum Initialisation {
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
pub enum SimulationMode {
    #[error("Simulation mode: {0}")]
    Simulation(Simulation),
}

#[derive(Debug, thiserror::Error)]
pub enum GarbageCollectorMode {
    #[error("Initialising IO: {0}")]
    InitIO(std::io::Error),
    #[error("Simulation: {0}")]
    Simulation(Simulation),
    #[error("Output IO had some issues: {0}")]
    OutputIO(std::io::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum InspectionMode {
    #[error("Initialising IO: {0}")]
    InitIO(std::io::Error),
    #[error("Simulation: {0}")]
    Simulation(Simulation),
    #[error("Output IO had some issues: {0}")]
    OutputIO(std::io::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum BenchmarkingMode {
    #[error("Initialising IO: {0}")]
    InitIO(std::io::Error),
    #[error("Simulation: {0}")]
    Simulation(Simulation),
    #[error("Output IO had some issues: {0}")]
    OutputIO(std::io::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum SimulationRun {
    #[error("Excedeed cycle limit of {0}")]
    CycleLimit(u32),

    #[error("Running Simulation: {0}")]
    Run(#[from] crate::error::sim::TickHw),
}

#[derive(Debug, thiserror::Error)]
pub struct Simulation {
    pub stats: Box<OurosCoreStatsError>,
    pub run: SimulationRun,
}

impl fmt::Display for Simulation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Finish implementing this")
    }
}
