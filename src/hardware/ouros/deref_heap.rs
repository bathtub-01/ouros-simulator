// Dereference Heap, also handles thread management:
//           +-----------------+
// addr  <-->|    Dereference  |===> to_reducer
// port_a ==>|                 |
// port_b ==>|       Heap      |===> to_self
//           +-----------------+

use crate::hardware::ouros::combinator::{all_patterns, parse_pat, Hole, ParseRes};
use crate::hardware::ouros::program::{App, Atom, DataFlowLink, APP_LENGTH, HOLES};
use crate::hardware::utils::fire;
use crate::hw_module::{HwInput, HwModule};

#[derive(Default)]
struct DHeapInput {
    port_a_valid: bool,
    port_a_bits: DataFlowLink<App>,
    port_b_valid: bool,
    port_b_bits: i32,
    // address request from reducer (for GC)
    addr_consumed: usize,
}
