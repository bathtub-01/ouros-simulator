use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 15
#[rustfmt::skip]
pub static UART: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            Ptr(1, false, false),
            Int(1993),
        ], 
        // AExp1
        vec![ // 1 
            Com(1,5),
            Ptr(2, false, false),
            Ptr(4, false, false),
            Com(1,3),
        ], 
        // AExp2
        vec![ // 2 
            Com(4,14),
            Com(2,10),
            Ptr(3, false, false),
            Com(1,12),
        ], 
        vec![ // 3 
            Com(3,15),
            Ptr(2, false, false),
        ], 
        // AExp3
        vec![ // 4 
            Com(1,5),
            Ptr(2, false, false),
            Ptr(5, false, false),
            Com(1,17),
        ], 
        vec![ // 5 
            IO(false),
            Int(0),
        ], 
        // AExp4
        vec![ // 6 
            IO(true),
            Int(4),
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
            Arg(2, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp3
        vec![ // 3 
            Com(1,21),
            Ptr(2, false, false),
            Ptr(0, true, true),
            Ptr(1, false, false),
        ], 
        vec![ // 4 
            Ptr(6, false, false),
            Arg(0, true),
        ], 
        // AExp4
        vec![ // 5 
            Arg(0, true),
            Com(3,0),
        ], 
        // AExp5
        vec![ // 6 
            Com(1,0),
            Ptr(0, true, true),
        ], 
        vec![ // 7 
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp6
        vec![ // 8 
            Arg(0, true),
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 9 
            Com(2,6),
            Arg(1, true),
        ], 
        // AExp7
        vec![ // 10 
            Com(1,0),
            Ptr(0, true, true),
        ], 
        vec![ // 11 
            Com(3,8),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp8
        vec![ // 12 
            Com(1,0),
            Ptr(0, true, true),
        ], 
        vec![ // 13 
            Com(3,2),
            Arg(0, true),
        ], 
        // AExp9
        vec![ // 14 
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp10
        vec![ // 15 
            Com(1,5),
            Arg(0, true),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 16 
            Com(2,0),
            Arg(2, true),
        ], 
        // AExp11
        vec![ // 17 
            Com(1,20),
            Ptr(2, false, false),
            Ptr(0, true, true),
        ], 
        vec![ // 18 
            Com(1,0),
            Arg(0, true),
        ], 
        // AExp12
        vec![ // 19 
            Arg(2, true),
        ], 
        // AExp13
        vec![ // 20 
            Arg(0, true),
            Com(3,19),
        ], 
        // AExp14
        vec![ // 21 
            Arg(0, true),
            Com(3,1),
        ], 
    ],

}});
