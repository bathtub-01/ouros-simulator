use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*;

// Functions in this file: 2
// Apps in this file: 4
// Combinators in this file: 3
#[rustfmt::skip]
pub static FIB: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0Fib.main
        vec![ // 0 
            PTR(1),
            INT(17),
        ], 
         // FUN1Fib.fib
        vec![ // 1 
            COM(5,28,[0,4,1,2,4,3]), //XXX(XX)X
            PRM(LE,false),
            INT(1),
            PTR(3),
            INT(1),
        ], 
        vec![ // 2 
            COM(5,21,[0,1,2,4,3,0]), //X(X(XXX))
            PRM(ADD,false),
            PTR(1),
            PRM(SUB,false),
            INT(2),
        ], 
        vec![ // 3 
            COM(5,49,[0,4,1,2,4,3]), //XX(X(XXX))
            PTR(2),
            PTR(1),
            PRM(SUB,false),
            INT(1),
        ], 
    ]
});
