use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 10
#[rustfmt::skip]
pub static TREESUM: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            PTR(2, false, false),
            PTR(1, false, false),
        ], 
        vec![ // 1 
            PTR(4, false, false),
            INT(13),
        ], 
        // AExp1
        vec![ // 2 
            COM(2,2),
            PTR(3, false, false),
        ], 
        vec![ // 3 
            COM(3,3),
            COM(2,5),
        ], 
        // AExp2
        vec![ // 4 
            COM(2,8),
            PTR(5, false, false),
        ], 
        vec![ // 5 
            COM(3,10),
            COM(1,12),
            COM(1,15),
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
            ARG(1),
            INT(1),
            ARG(0),
        ], 
        // AExp3
        vec![ // 3 
            PRM(ADD,false),
            PTR(0, true, true),
            INT(1),
        ], 
        vec![ // 4 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp4
        vec![ // 5 
            PRM(ADD,false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 6 
            PTR(2, false, false),
            ARG(1),
        ], 
        vec![ // 7 
            PTR(2, false, false),
            ARG(0),
        ], 
        // AExp5
        vec![ // 8 
            PRM(EQ,false),
            ARG(1),
            INT(0),
            PTR(0, true, true),
            COM(2,0),
        ], 
        vec![ // 9 
            ARG(0),
            ARG(1),
        ], 
        // AExp6
        vec![ // 10 
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 11 
            ARG(1),
            ARG(2),
        ], 
        // AExp7
        vec![ // 12 
            COM(4,16),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 13 
            PTR(4, false, false),
            ARG(0),
        ], 
        vec![ // 14 
            PTR(4, false, false),
            ARG(0),
        ], 
        // AExp8
        vec![ // 15 
            PRM(SUB,false),
            ARG(0),
            INT(1),
        ], 
        // AExp9
        vec![ // 16 
            ARG(3),
            ARG(0),
            ARG(1),
        ], 
    ],

}});