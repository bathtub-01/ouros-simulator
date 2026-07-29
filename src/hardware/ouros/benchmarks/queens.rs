use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 33
#[rustfmt::skip]
pub static QUEENS: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            Com(1,2),
            Int(6),
        ], 
        // AExp1
        vec![ // 1 
            Y,
            Com(3,6),
            Int(0),
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
            Ptr(1, false, false),
            Ptr(0, true, true),
        ], 
        vec![ // 3 
            Com(2,15),
            Arg(0, false),
            Arg(0, false),
        ], 
        // AExp3
        vec![ // 4 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 5 
            Prm(Add,false),
            Arg(1, true),
            Int(1),
        ], 
        // AExp4
        vec![ // 6 
            Arg(2, true),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 7 
            Com(3,4),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp5
        vec![ // 8 
            Com(2,15),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 9 
            Prm(Sub,false),
            Arg(1, true),
            Int(1),
        ], 
        // AExp6
        vec![ // 10 
            Com(1,21),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 11 
            Com(2,8),
            Arg(0, false),
            Arg(1, true),
        ], 
        vec![ // 12 
            Com(2,29),
            Arg(0, false),
        ], 
        // AExp7
        vec![ // 13 
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp8
        vec![ // 14 
            Com(4,13),
            Com(2,0),
            Com(2,0),
        ], 
        // AExp9
        vec![ // 15 
            Prm(EQ,false),
            Arg(1, false),
            Int(0),
            Com(2,10),
            Com(2,14),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp10
        vec![ // 16 
            Com(2,27),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 17 
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 18 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp11
        vec![ // 19 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 20 
            Com(4,16),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp12
        vec![ // 21 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 22 
            Com(3,19),
            Arg(0, true),
        ], 
        // AExp13
        vec![ // 23 
            Com(4,13),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 24 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp14
        vec![ // 25 
            Arg(2, true),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 26 
            Com(3,23),
            Arg(1, true),
        ], 
        // AExp15
        vec![ // 27 
            Y,
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 28 
            Com(3,25),
            Arg(1, true),
        ], 
        // AExp16
        vec![ // 29 
            Com(1,21),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 30 
            Com(1,61),
            Arg(0, true),
        ], 
        vec![ // 31 
            Com(2,35),
            Arg(1, true),
        ], 
        // AExp17
        vec![ // 32 
            Com(2,0),
        ], 
        // AExp18
        vec![ // 33 
            Com(4,13),
            Ptr(0, true, true),
            Com(2,0),
        ], 
        vec![ // 34 
            Com(4,13),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp19
        vec![ // 35 
            Com(1,53),
            Arg(1, false),
            Int(1),
            Arg(0, false),
            Com(2,32),
            Com(2,33),
            Arg(1, false),
            Arg(0, false),
        ], 
        // AExp20
        vec![ // 36 
            Prm(EQ,true),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 37 
            Prm(Add,false),
            Arg(2, true),
            Arg(1, true),
        ], 
        // AExp21
        vec![ // 38 
            Prm(EQ,true),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 39 
            Prm(Sub,false),
            Arg(2, true),
            Arg(1, true),
        ], 
        // AExp22
        vec![ // 40 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 41 
            Prm(Add,false),
            Arg(1, true),
            Int(1),
        ], 
        // AExp23
        vec![ // 42 
            Com(1,55),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 43 
            Com(2,40),
            Arg(1, true),
            Arg(2, false),
            Arg(4, true),
        ], 
        vec![ // 44 
            Com(3,38),
            Arg(0, true),
            Arg(2, false),
            Arg(3, true),
        ], 
        // AExp24
        vec![ // 45 
            Com(1,55),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 46 
            Com(5,42),
            Arg(0, false),
            Arg(1, true),
            Arg(2, false),
            Arg(3, false),
            Arg(4, true),
        ], 
        vec![ // 47 
            Com(3,36),
            Arg(0, false),
            Arg(2, false),
            Arg(3, false),
        ], 
        // AExp25
        vec![ // 48 
            Com(1,55),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 49 
            Com(5,45),
            Arg(0, false),
            Arg(1, true),
            Arg(2, true),
            Arg(3, false),
            Arg(4, true),
        ], 
        vec![ // 50 
            Prm(EQ,true),
            Arg(0, false),
            Arg(3, false),
        ], 
        // AExp26
        vec![ // 51 
            Arg(3, true),
            Com(2,1),
            Ptr(0, true, true),
        ], 
        vec![ // 52 
            Com(5,48),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp27
        vec![ // 53 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 54 
            Com(4,51),
            Arg(0, true),
        ], 
        // AExp28
        vec![ // 55 
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp29
        vec![ // 56 
            Com(1,61),
            Ptr(0, true, true),
        ], 
        vec![ // 57 
            Prm(Sub,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp30
        vec![ // 58 
            Com(4,13),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 59 
            Com(1,56),
            Arg(0, false),
        ], 
        // AExp31
        vec![ // 60 
            Com(4,13),
            Int(1),
            Com(2,0),
        ], 
        // AExp32
        vec![ // 61 
            Prm(EQ,false),
            Arg(0, false),
            Int(1),
            Com(1,58),
            Com(1,60),
            Arg(0, false),
        ], 
    ],

}});