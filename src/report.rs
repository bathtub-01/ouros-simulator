use crate::{error, hardware::ouros::ouros_core};

pub enum ReportBuilder {
    EvalGc,
    Benchmark,
    InspectProgram,
    RunBigProgram
}

pub struct Report;


