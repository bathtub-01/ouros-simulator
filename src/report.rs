use crate::{error, hardware::ouros::ouros_core};


pub struct OurosState {
    core: ouros_core::OurosCore,
    cycles: u32
}

pub struct OurosOutput(Result<OurosState, error::Simulation>);

pub enum ReportBuilder {
    EvalGc,
    Benchmark,
    InspectProgram,
    RunBigProgram
}

pub struct Report;


