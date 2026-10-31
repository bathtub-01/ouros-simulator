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
}

