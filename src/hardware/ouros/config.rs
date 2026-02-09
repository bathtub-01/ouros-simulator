// Parameter configuration of the architecture.

// core-wise setup, can be configured to instantiate ouros core
pub struct OurosConfig {
    working_threads: usize,
    heap_size: usize,
    thread_stack_size: usize,
}

pub const BIG_HEAP: usize = 1024 * 256; // a big heap size that won't trigger GC
pub const HEAP_SIZE: usize = 1024 * 3;
pub const PROG_SIZE: usize = 1024;
pub const MAX_THREADS: usize = 4;
pub const BUFFER_SIZE: usize = MAX_THREADS + 1;

pub const ALU_PIPE: bool = false;
pub const REDUCER_PIPE: bool = false;

/// max number of addresses can be consumed in one cycle
pub const CONSUMERS: usize = CONSUMERS_REDUCER + CONSUMERS_DHEAP;
pub const CONSUMERS_REDUCER: usize = 7;
pub const CONSUMERS_DHEAP: usize = 1;

pub const GC_AT: f32 = 0.1; // start GC when freelist is shorter than this
pub const CACHE_SIZE: usize = 8;

// system-wise setup, changing them will require also modifying the compiler
pub const HOLES: usize = 6;
pub const APP_LENGTH: usize = 8;

// stats detail level
pub const DLV_THREADS: u8 = 100;
pub const DLV_FULL_LOG: u8 = 250;
pub const DLV_STM_DIST: u8 = 240;
pub const DLV_BUSY_RATE: u8 = 150;
pub const DLV_GC: u8 = 150;
pub const DLV_BUFFER_USAGE: u8 = 180;
pub const DLV_MEM_USAGE: u8 = 170;
