use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 16
#[rustfmt::skip]
pub static TREEPARI: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            COM(1,2),
            PTR(2, false, false),
        ], 
        vec![ // 1 
            PTR(8, false, false),
            INT(10),
        ], 
        vec![ // 2 
            PTR(3, false, false),
            PTR(6, false, false),
            PTR(1, false, false),
        ], 
        // AExp1
        vec![ // 3 
            COM(2,3),
            PTR(5, false, false),
        ], 
        vec![ // 4 
            COM(6,8),
            COM(3,11),
        ], 
        vec![ // 5 
            COM(4,5),
            PTR(4, false, false),
        ], 
        // AExp2
        vec![ // 6 
            COM(2,17),
            PTR(7, false, false),
        ], 
        vec![ // 7 
            COM(3,18),
            COM(2,20),
        ], 
        // AExp3
        vec![ // 8 
            COM(2,21),
            PTR(9, false, false),
        ], 
        vec![ // 9 
            COM(3,23),
            COM(1,25),
            COM(1,28),
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
            ARG(0),
            INT(0),
            INT(42),
        ], 
        // AExp3
        vec![ // 3 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 4 
            ARG(0),
            ARG(1),
        ], 
        // AExp4
        vec![ // 5 
            ARG(3),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 6 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(3),
        ], 
        vec![ // 7 
            ARG(1),
            COM(2,0),
            COM(2,1),
            COM(2,0),
        ], 
        // AExp5
        vec![ // 8 
            COM(2,14),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 9 
            ARG(1),
            ARG(3),
            COM(2,1),
            COM(2,0),
        ], 
        vec![ // 10 
            ARG(0),
            ARG(2),
            ARG(4),
            ARG(5),
        ], 
        // AExp6
        vec![ // 11 
            COM(2,14),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 12 
            ARG(0),
            ARG(2),
        ], 
        vec![ // 13 
            ARG(0),
            ARG(1),
        ], 
        // AExp7
        vec![ // 14 
            ARG(0),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 15 
            ARG(1),
            COM(2,0),
            COM(2,1),
        ], 
        vec![ // 16 
            ARG(1),
            COM(2,1),
            COM(2,0),
        ], 
        // AExp8
        vec![ // 17 
            ARG(1),
            COM(2,0),
            ARG(0),
        ], 
        // AExp9
        vec![ // 18 
            ARG(1),
            PTR(0, true, true),
            ARG(0),
        ], 
        vec![ // 19 
            ARG(2),
            COM(2,1),
            ARG(0),
        ], 
        // AExp10
        vec![ // 20 
            COM(2,0),
        ], 
        // AExp11
        vec![ // 21 
            PRM(EQ,false),
            ARG(1),
            INT(0),
            PTR(0, true, true),
            COM(2,0),
        ], 
        vec![ // 22 
            ARG(0),
            ARG(1),
        ], 
        // AExp12
        vec![ // 23 
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 24 
            ARG(1),
            ARG(2),
        ], 
        // AExp13
        vec![ // 25 
            COM(4,29),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 26 
            PTR(8, false, false),
            ARG(0),
        ], 
        vec![ // 27 
            PTR(8, false, false),
            ARG(0),
        ], 
        // AExp14
        vec![ // 28 
            PRM(SUB,false),
            ARG(0),
            INT(1),
        ], 
        // AExp15
        vec![ // 29 
            ARG(3),
            ARG(0),
            ARG(1),
        ], 
    ],

}});