use std::default;

use crate::hardware::common::memory::DualInput;
use crate::hardware::common::{DualPortMem, Register};
use crate::hardware::ouros::config::*;
use crate::hw_module::{HwInput, HwModule};

#[derive(Default)]
enum CollectorState {
    #[default]
    IDLE,
    MARK,
    SWEEP,
}

#[derive(Default, Clone, Debug)]
enum CellState {
    #[default]
    Unmarked,
    Marked,
    FreeList,
    WorkList,
}

/// Basic cell of the GC bookkeeping memory
#[derive(Default, Clone, Debug)]
struct GCCell {
    state: CellState,
    ptr: usize,
}

#[derive(Default)]
pub struct GbgCollectorInput {}

impl HwInput for GbgCollectorInput {}

pub struct GbgCollector {
    pub input: GbgCollectorInput,
    pub gc_mem: DualPortMem<GCCell>,
    reg_free_head: Register<usize>,
    reg_work_head: Register<usize>,
}

impl GbgCollector {
    pub fn new(heap_size: usize, free_from: usize) -> Self {
        Self {
            input: Default::default(),
            gc_mem: DualPortMem::new(heap_size),
            reg_free_head: Register::init(free_from),
            reg_work_head: Default::default(),
        }
    }

    /// setup the freelist before execution
    pub fn init_freelist(mut self) -> Self {
        let from = *self.reg_free_head.value();
        let heap_size = self.gc_mem.ram.len();
        let mut img: Vec<GCCell> = vec![Default::default(); heap_size];
        for i in from..heap_size {
            img[i] = GCCell {
                state: CellState::FreeList,
                ptr: i + 1,
            };
        }
        self.gc_mem.image(&img);
        self
    }
}

impl HwModule for GbgCollector {
    fn update_local(&mut self) {
        todo!()
    }

    fn tick_children(&mut self) {
        todo!()
    }
}

#[test]
fn inspect_init_result() {
    let gc = GbgCollector::new(100, 8).init_freelist();
    println!("{:#?}", gc.gc_mem.ram);
}
