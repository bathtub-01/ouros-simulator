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
            Com(1,9),
            Int(17),
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
            Com(1,9),
            Ptr(0, true, true),
        ], 
        vec![ // 3 
            Prm(Sub,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp3
        vec![ // 4 
            Com(1,9),
            Ptr(0, true, true),
        ], 
        vec![ // 5 
            Prm(Sub,false),
            Arg(0, true),
            Int(2),
        ], 
        // AExp4
        vec![ // 6 
            Prm(Add,false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 7 
            Com(1,4),
            Arg(0, false),
        ], 
        vec![ // 8 
            Com(1,2),
            Arg(0, false),
        ], 
        // AExp5
        vec![ // 9 
            Prm(LE,false),
            Arg(0, false),
            Int(1),
            Ptr(0, true, true),
            Int(1),
        ], 
        vec![ // 10 
            Com(1,6),
            Arg(0, false),
        ], 
    ],

}});
