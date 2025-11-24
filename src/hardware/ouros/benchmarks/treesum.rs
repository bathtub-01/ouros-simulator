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
            COM(1,7),
            PTR(1, false, false),
        ], 
        vec![ // 1 
            COM(1,14),
            INT(13),
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
            PRM(ADD,false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 3 
            COM(1,7),
            ARG(1, true),
        ], 
        vec![ // 4 
            COM(1,7),
            ARG(0, true),
        ], 
        // AExp3
        vec![ // 5 
            PRM(ADD,false),
            PTR(0, true, true),
            INT(1),
        ], 
        vec![ // 6 
            COM(2,2),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp4
        vec![ // 7 
            ARG(0, true),
            INT(1),
            COM(2,5),
        ], 
        // AExp5
        vec![ // 8 
            COM(4,16),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 9 
            COM(1,14),
            ARG(0, false),
        ], 
        vec![ // 10 
            COM(1,14),
            ARG(0, false),
        ], 
        // AExp6
        vec![ // 11 
            PRM(SUB,false),
            ARG(0, true),
            INT(1),
        ], 
        // AExp7
        vec![ // 12 
            COM(1,8),
            PTR(0, true, true),
        ], 
        vec![ // 13 
            COM(1,11),
            ARG(0, true),
        ], 
        // AExp8
        vec![ // 14 
            PRM(EQ,false),
            ARG(0, false),
            INT(0),
            PTR(0, true, true),
            COM(2,0),
        ], 
        vec![ // 15 
            COM(1,12),
            ARG(0, false),
        ], 
        // AExp9
        vec![ // 16 
            ARG(3, true),
            ARG(0, true),
            ARG(1, true),
        ], 
    ],

}});