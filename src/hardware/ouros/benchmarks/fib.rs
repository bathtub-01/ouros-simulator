use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*;

// FIXME: operator is not infixed here!
#[rustfmt::skip]
pub static FIB: LazyLock<Program> = LazyLock::new(|| {
    vec![
        // main
        vec![ // 0
            PTR(1),
            INT(5),
        ],
        // fib
        vec![ // 1
            COM(5,28,[0,4,1,2,4,3]), // XXX(XX)X
            PRM(LE, false),
            INT(1),
            PTR(3),
            INT(1)
        ],
        vec![ // 2
            COM(5, 12, [0,4,1,2,3,0]), // X(XXX)X
            PTR(1),
            PRM(SUB, false),
            INT(2),
            PRM(ADD, false)
        ],
        vec![ // 3
            COM(5, 49, [0,4,1,4,2,3]), // // XX(X(XXX))
            PTR(2),
            PTR(1),
            PRM(SUB, false),
            INT(1)
        ]
    ]
});
