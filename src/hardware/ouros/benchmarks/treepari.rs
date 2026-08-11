use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 23
#[rustfmt::skip]
pub static TREEPARI: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            Com(1,2),
            Ptr(2, false, false),
        ], 
        vec![ // 1 
            Com(1,30),
            Int(10),
        ], 
        vec![ // 2 
            Com(1,12),
            Com(1,22),
            Ptr(1, false, false),
        ], 
    ],
    comb_img: vec![
        // AExp0
        vec![ // 0 
            Arg(0, true),
        ], 
        // AExp1
        vec![ // 1 
            Arg(1, true),
        ], 
        // AExp2
        vec![ // 2 
            Arg(0, true),
            Int(0),
            Int(42),
        ], 
        // AExp3
        vec![ // 3 
            Arg(0, true),
            Com(2,0),
            Com(2,1),
            Com(2,0),
        ], 
        // AExp4
        vec![ // 4 
            Com(0,17),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 5 
            Arg(0, false),
            Arg(2, true),
        ], 
        vec![ // 6 
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp5
        vec![ // 7 
            Com(0,17),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 8 
            Arg(3, true),
            Arg(4, true),
            Com(2,1),
            Com(2,0),
        ], 
        vec![ // 9 
            Com(3,4),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp6
        vec![ // 10 
            Arg(2, false),
            Com(2,3),
            Ptr(0, true, true),
            Arg(0, true),
            Arg(2, false),
        ], 
        vec![ // 11 
            Com(5,7),
            Arg(1, true),
        ], 
        // AExp7
        vec![ // 12 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 13 
            Com(3,10),
            Arg(0, true),
        ], 
        // AExp8
        vec![ // 14 
            Arg(0, true),
            Com(2,1),
            Com(2,0),
        ], 
        // AExp9
        vec![ // 15 
            Arg(0, true),
            Com(2,0),
            Com(2,1),
        ], 
        // AExp10
        vec![ // 16 
            Arg(0, true),
            Com(1,14),
            Com(1,15),
        ], 
        // AExp11
        vec![ // 17 
            Com(1,16),
        ], 
        // AExp12
        vec![ // 18 
            Com(2,0),
        ], 
        // AExp13
        vec![ // 19 
            Arg(0, true),
            Com(2,1),
            Com(2,18),
        ], 
        // AExp14
        vec![ // 20 
            Com(2,0),
        ], 
        // AExp15
        vec![ // 21 
            Arg(0, true),
            Com(1,19),
            Com(3,20),
        ], 
        // AExp16
        vec![ // 22 
            Arg(0, true),
            Com(2,0),
            Com(1,21),
        ], 
        // AExp17
        vec![ // 23 
            Com(4,31),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 24 
            Com(1,30),
            Arg(0, false),
        ], 
        vec![ // 25 
            Com(1,30),
            Arg(0, false),
        ], 
        // AExp18
        vec![ // 26 
            Prm(Sub,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp19
        vec![ // 27 
            Com(1,23),
            Ptr(0, true, true),
        ], 
        vec![ // 28 
            Com(1,26),
            Arg(0, true),
        ], 
        // AExp20
        vec![ // 29 
            Com(2,0),
        ], 
        // AExp21
        vec![ // 30 
            Prm(EQ,false),
            Arg(0, false),
            Int(0),
            Com(1,27),
            Com(1,29),
            Arg(0, false),
        ], 
        // AExp22
        vec![ // 31 
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
        ], 
    ],

}});