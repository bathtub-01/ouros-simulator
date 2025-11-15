use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 6
#[rustfmt::skip]
pub static FIB: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            PTR(1, false, false),
            INT(17),
        ], 
        // AExp1
        vec![ // 1 
            COM(2,2),
            PTR(2, false, false),
        ], 
        vec![ // 2 
            COM(3,4),
            COM(1,7),
            COM(1,9),
        ], 
    ],
    comb_img: vec![
        // AExp0
        vec![ // 0 
            ARG(0),
        ], 
        // AExp1
        vec![ // 1 
            ARG(1),
        ], 
        // AExp2
        vec![ // 2 
            PRM(LE,false),
            ARG(1),
            INT(1),
            PTR(0, true, true),
            INT(1),
        ], 
        vec![ // 3 
            ARG(0),
            ARG(1),
        ], 
        // AExp3
        vec![ // 4 
            PRM(ADD,false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 5 
            ARG(1),
            ARG(2),
        ], 
        vec![ // 6 
            ARG(0),
            ARG(2),
        ], 
        // AExp4
        vec![ // 7 
            PTR(1, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 8 
            PRM(SUB,false),
            ARG(0),
            INT(1),
        ], 
        // AExp5
        vec![ // 9 
            PTR(1, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 10 
            PRM(SUB,false),
            ARG(0),
            INT(2),
        ], 
    ],

}});