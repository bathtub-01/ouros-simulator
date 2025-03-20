use crate::hardware::ouros::program::{Atom, Program};
use std::sync::LazyLock;
use Atom::*;

/**
A minimal Haskell program:

data TF = T | F

tfAnd T b = b

tfAnd F _ = F

main = tfAnd T F
 */
#[rustfmt::skip]
pub static BOOL_AND: LazyLock<Program> = LazyLock::new(|| {
    vec![
        // main
        vec![ // 0
            PTR(1),
            COM(2,0,[0,0,0,0,0,0]), // X
            COM(2,0,[1,0,0,0,0,0]), // X
        ],
        // tfAnd
        vec![ // 1
            COM(3,2,[1,2,0,0,0,0]), // XXX
            COM(2,0,[1,0,0,0,0,0]), // X
        ],
    ]
});
