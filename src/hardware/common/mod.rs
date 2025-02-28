pub mod register;
pub use register::Register;

pub mod memory;
pub use memory::{DualPortMem, SinglePortMem};

pub mod fifo;
pub use fifo::FIFO;

pub mod utils;
