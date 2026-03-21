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
            COM(1,9),
            INT(17),
        ], 
    ],
    comb_img: vec![
        // AExp0
        vec![ // 0 
            ARG(0, true),
        ], 
        // AExp1
        vec![ // 1 
            ARG(1, true),
        ], 
        // AExp2
        vec![ // 2 
            COM(1,9),
            PTR(0, true, true),
        ], 
        vec![ // 3 
            PRM(SUB,false),
            ARG(0, true),
            INT(1),
        ], 
        // AExp3
        vec![ // 4 
            COM(1,9),
            PTR(0, true, true),
        ], 
        vec![ // 5 
            PRM(SUB,false),
            ARG(0, true),
            INT(2),
        ], 
        // AExp4
        vec![ // 6 
            PRM(ADD,false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 7 
            COM(1,4),
            ARG(0, false),
        ], 
        vec![ // 8 
            COM(1,2),
            ARG(0, false),
        ], 
        // AExp5
        vec![ // 9 
            PRM(LE,false),
            ARG(0, false),
            INT(1),
            PTR(0, true, true),
            INT(1),
        ], 
        vec![ // 10 
            COM(1,6),
            ARG(0, false),
        ], 
    ],

}});