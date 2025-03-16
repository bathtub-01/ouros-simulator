// Parameter configuration of the architecture.
// `OurosConfig` is for core-wise setup, and can be configured to instantiate ouros core.
// Hard-coded constants are system-wise setup, changing them will require also modifying the compiler

pub struct OurosConfig {
    working_threads: usize,
    heap_size: usize,
    thread_stack_size: usize,
}

pub const HOLES: usize = 6;
pub const APP_LENGTH: usize = 8;
