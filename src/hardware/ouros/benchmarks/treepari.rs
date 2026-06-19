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
            Com(1,2),
            Ptr(2, false, false),
        ], 
        vec![ // 1 
            Com(1,28),
            Int(10),
        ], 
        vec![ // 2 
            Com(1,12),
            Com(1,21),
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
            Com(2,14),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 4 
            Arg(0, false),
            Arg(2, true),
        ], 
        vec![ // 5 
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp4
        vec![ // 6 
            Com(2,14),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 7 
            Arg(0, true),
            Arg(2, true),
            Com(2,1),
            Com(2,0),
        ], 
        vec![ // 8 
            Com(3,3),
            Arg(1, true),
            Arg(3, true),
            Arg(4, true),
        ], 
        // AExp5
        vec![ // 9 
            Arg(2, false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 10 
            Com(5,6),
            Arg(0, false),
            Arg(1, true),
            Arg(2, false),
        ], 
        vec![ // 11 
            Arg(0, false),
            Com(2,0),
            Com(2,1),
            Com(2,0),
        ], 
        // AExp6
        vec![ // 12 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 13 
            Com(3,9),
            Arg(0, true),
        ], 
        // AExp7
        vec![ // 14 
            Arg(0, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 15 
            Arg(1, false),
            Com(2,0),
            Com(2,1),
        ], 
        vec![ // 16 
            Arg(1, false),
            Com(2,1),
            Com(2,0),
        ], 
        // AExp8
        vec![ // 17 
            Com(2,0),
        ], 
        // AExp9
        vec![ // 18 
            Com(2,0),
        ], 
        // AExp10
        vec![ // 19 
            Arg(0, true),
            Ptr(0, true, true),
            Com(2,18),
        ], 
        vec![ // 20 
            Arg(1, true),
            Com(2,1),
            Com(2,17),
        ], 
        // AExp11
        vec![ // 21 
            Arg(0, true),
            Com(2,0),
            Com(2,19),
        ], 
        // AExp12
        vec![ // 22 
            Com(4,30),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 23 
            Com(1,28),
            Arg(0, false),
        ], 
        vec![ // 24 
            Com(1,28),
            Arg(0, false),
        ], 
        // AExp13
        vec![ // 25 
            Prm(Sub,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp14
        vec![ // 26 
            Com(1,22),
            Ptr(0, true, true),
        ], 
        vec![ // 27 
            Com(1,25),
            Arg(0, true),
        ], 
        // AExp15
        vec![ // 28 
            Prm(EQ,false),
            Arg(0, false),
            Int(0),
            Ptr(0, true, true),
            Com(2,0),
        ], 
        vec![ // 29 
            Com(1,26),
            Arg(0, false),
        ], 
        // AExp16
        vec![ // 30 
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
        ], 
    ],

}});
