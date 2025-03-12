// Reducer for structured combinators:
//          +-----------------+===> spine
//          |                 |
// input ==>|     Reducer     |===> app1
//          |                 |===> app2
//          +-----------------+===> app3
// The reducer is pipelined, and can handle one input in each
// clcok cycle. Inputs and Outputs has ready-valid signals,
// the reducer will be stalled if outputs fail to emit.

use crate::hardware::common::Register;
use crate::hardware::ouros::combinator::{all_patterns, parse_pat, ParseRes};
use crate::hardware::ouros::program::{App, Atom, HOLES};
use crate::hw_module::{HwModule, HwStates};
use crate::{input, local};

#[derive(Default)]
struct ReducerInput {
    in_valid: bool,
    in_app: App,
    spine_ready: bool,
    app1_ready: bool,
    app2_ready: bool,
    app3_ready: bool,
    // receive free address from GC
    free_addr: usize,
}

struct ReducerLocal {
    decode_table: [ParseRes; 64],
    spine_holder: [Atom; HOLES],
    app1_holder: [Atom; HOLES - 1],
    app2_holder: [Atom; HOLES - 2],
    app3_holder: [Atom; HOLES - 3],
}

impl Default for ReducerLocal {
    fn default() -> Self {
        let parsed: Vec<ParseRes> = all_patterns().iter().map(|p| parse_pat(p)).collect();
        let decode_table: [ParseRes; 64] = parsed
            .try_into()
            .expect("pattern decode table size should match");

        Self {
            decode_table,
            ..Default::default()
        }
    }
}

pub struct Reducer {
    states: HwStates<ReducerInput, ReducerLocal>,
}
