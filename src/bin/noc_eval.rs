//! Standalone NoC evaluation binary:
//!   cargo run --release --bin noc_eval -- --help
//!
//! The simulator is currently organised as a binary crate, rather than a
//! library crate. Bring in only the shared hardware modules that the NoC
//! needs, and expose their *original* `crate::hardware::...` paths.
//!
//! IMPORTANT: Import `common/mod.rs` (not arbiter.rs/fifo.rs separately):
//! it supplies `common::Register` and declares the related submodules.

#[path = "../hw_module.rs"]
mod hw_module;

#[path = "../hardware/common/mod.rs"]
pub(crate) mod common;

#[path = "../hardware/utils.rs"]
pub(crate) mod utils;

#[path = "../hardware/noc/router.rs"]
pub(crate) mod router;

#[path = "../hardware/noc/network.rs"]
pub(crate) mod network;

// Retain the paths used in the original implementations without importing
// the unrelated Ouros core, benchmarks or simulator main binary.
mod hardware {
    pub(crate) use crate::common;
    pub(crate) use crate::utils;

    pub(crate) mod noc {
        pub(crate) use crate::network;
        pub(crate) use crate::router;
    }
}

#[path = "../hardware/noc/noc_eval.rs"]
mod noc_eval;

fn main() {
    if let Err(err) = noc_eval::run() {
        eprintln!("noc_eval: {err}");
        std::process::exit(1);
    }
}
