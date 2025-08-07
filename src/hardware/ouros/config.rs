// Parameter configuration of the architecture.

// core-wise setup, can be configured to instantiate ouros core
pub struct OurosConfig {
    working_threads: usize,
    heap_size: usize,
    thread_stack_size: usize,
}

// system-wise setup, changing them will require also modifying the compiler
pub const HOLES: usize = 6;
pub const APP_LENGTH: usize = 8;

// stats detail level
pub const DLV_THREADS: u8 = 100;
pub const DLV_FULL_LOG: u8 = 250;
pub const DLV_STM_DIST: u8 = 240;
pub const DLV_BUSY_RATE: u8 = 150;
pub const DLV_BUFFER_USAGE: u8 = 180;
pub const DLV_MEM_USAGE: u8 = 170;
