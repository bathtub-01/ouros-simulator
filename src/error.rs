use rayon::ThreadPoolBuildError;

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

#[derive(Debug, thiserror::Error)]
pub enum Initialisation {
    #[error("Initialising Rayon thread pool: {0}")]
    RayonThreadPool(#[from] ThreadPoolBuildError),
    #[error("Program lookup failed: {program_name}")]
    ProgramNotFound { program_name: String },
    #[error("Invalid CLI arguments: {reason}")]
    InvalidCliArgs { reason: String },
}

#[derive(Debug, thiserror::Error)]
pub enum SimulationMode {
    #[error("Simulation: {0}")]
    Simulation(Simulation),
}

#[derive(Debug, thiserror::Error)]
pub enum Simulation {
    #[error("Some shite: {0}")]
    StringError(String)
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

pub fn report_ouros(e: Ouros) {
    dbg!(&e);
    // TODO: yet to report it properly
    // match e {}
}
