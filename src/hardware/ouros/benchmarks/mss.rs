use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*;

#[rustfmt::skip]
pub static MSS: LazyLock<Program> = LazyLock::new(|| {
    Program {
        heap_img: vec![],
        comb_img: vec![
            // True 0
            vec![ARG(1)],
            // False 1
            vec![ARG(0)],
        ]
    }
});
