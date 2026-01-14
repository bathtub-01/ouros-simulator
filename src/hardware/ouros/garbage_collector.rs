use std::default;

use crate::hardware::common::{DualPortMem, Register};
use crate::hw_module::{HwInput, HwModule};

#[derive(Default)]
enum CollectorState {
    #[default]
    IDLE,
    MARK,
    SWEEP,
}

#[derive(Default)]
enum CellState {
    #[default]
    FreeList,
    WorkList,
    Unmarked,
    Marked,
}
