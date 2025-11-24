use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 17
#[rustfmt::skip]
pub static TREEPARI: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            COM(1,2),
            PTR(2, false, false),
        ], 
        vec![ // 1 
            COM(1,28),
            INT(10),
        ], 
        vec![ // 2 
            COM(1,12),
            COM(1,21),
            PTR(1, false, false),
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
            ARG(0, true),
            INT(0),
            INT(42),
        ], 
        // AExp3
        vec![ // 3 
            COM(2,14),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 4 
            ARG(0, false),
            ARG(2, true),
        ], 
        vec![ // 5 
            ARG(0, false),
            ARG(1, true),
        ], 
        // AExp4
        vec![ // 6 
            COM(2,14),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 7 
            ARG(0, true),
            ARG(2, true),
            COM(2,1),
            COM(2,0),
        ], 
        vec![ // 8 
            COM(3,3),
            ARG(1, true),
            ARG(3, true),
            ARG(4, true),
        ], 
        // AExp5
        vec![ // 9 
            ARG(2, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 10 
            COM(5,6),
            ARG(0, false),
            ARG(1, true),
            ARG(2, false),
        ], 
        vec![ // 11 
            ARG(0, false),
            COM(2,0),
            COM(2,1),
            COM(2,0),
        ], 
        // AExp6
        vec![ // 12 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 13 
            COM(3,9),
            ARG(0, true),
        ], 
        // AExp7
        vec![ // 14 
            ARG(0, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 15 
            ARG(1, false),
            COM(2,0),
            COM(2,1),
        ], 
        vec![ // 16 
            ARG(1, false),
            COM(2,1),
            COM(2,0),
        ], 
        // AExp8
        vec![ // 17 
            COM(2,0),
        ], 
        // AExp9
        vec![ // 18 
            COM(2,0),
        ], 
        // AExp10
        vec![ // 19 
            ARG(0, true),
            PTR(0, true, true),
            COM(2,18),
        ], 
        vec![ // 20 
            ARG(1, true),
            COM(2,1),
            COM(2,17),
        ], 
        // AExp11
        vec![ // 21 
            ARG(0, true),
            COM(2,0),
            COM(2,19),
        ], 
        // AExp12
        vec![ // 22 
            COM(4,30),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 23 
            COM(1,28),
            ARG(0, false),
        ], 
        vec![ // 24 
            COM(1,28),
            ARG(0, false),
        ], 
        // AExp13
        vec![ // 25 
            PRM(SUB,false),
            ARG(0, true),
            INT(1),
        ], 
        // AExp14
        vec![ // 26 
            COM(1,22),
            PTR(0, true, true),
        ], 
        vec![ // 27 
            COM(1,25),
            ARG(0, true),
        ], 
        // AExp15
        vec![ // 28 
            PRM(EQ,false),
            ARG(0, false),
            INT(0),
            PTR(0, true, true),
            COM(2,0),
        ], 
        vec![ // 29 
            COM(1,26),
            ARG(0, false),
        ], 
        // AExp16
        vec![ // 30 
            ARG(3, true),
            ARG(0, true),
            ARG(1, true),
        ], 
    ],

}});