use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*;

/**
A minimal Haskell program:

-- T => COM(2,0,[0,0,0,0,0,0])
-- F =>  COM(2,0,[1,0,0,0,0,0])
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

/**
-- the computation in a should be shared
main = let a = tfAnd T
           b = ftAnd (a T) (a F)
       in tfOr b b
 */
#[rustfmt::skip]
pub static BOOL_NEST: LazyLock<Program> = LazyLock::new(|| {
    vec![
        // main
        vec![ // 0
            PTR(4),
            PTR(2),
            PTR(2)
        ],
        // tfAnd
        vec![ // 1
            COM(3,2,[1,2,0,0,0,0]), // XXX
            COM(2,0,[1,0,0,0,0,0]), // X
        ],
        // b
        vec![ // 2
            COM(4,15,[0,1,2,1,3,0]),
            PTR(1),
            PTR(3), // a
            COM(2,0,[0,0,0,0,0,0]), // T
            COM(2,0,[1,0,0,0,0,0]), // F
        ],
        // a
        vec![ // 3
            PTR(1),
            COM(2,0,[0,0,0,0,0,0]), // X
        ],
        // tfOr
        vec![ // 4
            COM(2,1,[1,0,0,0,0,0]), // XX
            COM(2,0,[0,0,0,0,0,0]), // T
        ],
    ]
});

/**
main = let a = 4 + 5
           b = a * 2
       in
          if 42 == 42 then a * b else a - b
*/
#[rustfmt::skip]
pub static ALU_OP: LazyLock<Program> = LazyLock::new(|| {
    vec![
        // main
        vec![ // 0
            PRM(EQ, false),
            INT(42),
            INT(42),
            PTR(2),
            PTR(1)
        ],
        // a * b
        vec![ // 1
            PRM(MUL, false),
            PTR(3),
            PTR(4)
        ],
        // a - b
        vec![ // 2
            PRM(SUB, false),
            PTR(3),
            PTR(4)
        ],
        // a
        vec![ // 3
            PRM(ADD, false),
            INT(4),
            INT(5)
        ],
        // b
        vec![ // 4
            PRM(MUL, false),
            PTR(3),
            INT(2)
        ],
    ]
});
