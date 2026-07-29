use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 11
#[rustfmt::skip]
pub static TREESUM: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            Com(1,7),
            Ptr(1, false, false),
        ], 
        vec![ // 1 
            Com(1,15),
            Int(13),
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
            Prm(Add,false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 3 
            Com(1,7),
            Arg(1, true),
        ], 
        vec![ // 4 
            Com(1,7),
            Arg(0, true),
        ], 
        // AExp3
        vec![ // 5 
            Prm(Add,false),
            Ptr(0, true, true),
            Int(1),
        ], 
        vec![ // 6 
            Com(2,2),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp4
        vec![ // 7 
            Arg(0, true),
            Int(1),
            Com(2,5),
        ], 
        // AExp5
        vec![ // 8 
            Com(4,16),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 9 
            Com(1,15),
            Arg(0, false),
        ], 
        vec![ // 10 
            Com(1,15),
            Arg(0, false),
        ], 
        // AExp6
        vec![ // 11 
            Prm(Sub,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp7
        vec![ // 12 
            Com(1,8),
            Ptr(0, true, true),
        ], 
        vec![ // 13 
            Com(1,11),
            Arg(0, true),
        ], 
        // AExp8
        vec![ // 14 
            Com(2,0),
        ], 
        // AExp9
        vec![ // 15 
            Prm(EQ,false),
            Arg(0, false),
            Int(0),
            Com(1,12),
            Com(1,14),
            Arg(0, false),
        ], 
        // AExp10
        vec![ // 16 
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
        ], 
    ],

}});