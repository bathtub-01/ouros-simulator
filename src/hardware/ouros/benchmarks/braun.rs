use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 40
#[rustfmt::skip]
pub static BRAUN: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            Com(1,2),
            Ptr(3, false, false),
        ], 
        vec![ // 1 
            Com(2,57),
            Int(0),
            Int(255),
        ], 
        vec![ // 2 
            Com(2,50),
            Int(2),
            Ptr(1, false, false),
        ], 
        vec![ // 3 
            Com(1,9),
            Com(1,15),
            Ptr(2, false, false),
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
            Int(1),
        ], 
        // AExp3
        vec![ // 3 
            Com(2,1),
        ], 
        // AExp4
        vec![ // 4 
            Com(1,12),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 5 
            Arg(0, true),
            Arg(2, true),
        ], 
        vec![ // 6 
            Arg(3, true),
            Arg(1, true),
        ], 
        // AExp5
        vec![ // 7 
            Arg(2, true),
            Com(1,3),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 8 
            Com(4,4),
            Arg(1, true),
        ], 
        // AExp6
        vec![ // 9 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 10 
            Com(3,7),
            Arg(0, true),
        ], 
        // AExp7
        vec![ // 11 
            Com(2,0),
        ], 
        // AExp8
        vec![ // 12 
            Arg(0, true),
            Com(1,11),
            Com(1,0),
        ], 
        // AExp9
        vec![ // 13 
            Com(1,29),
            Ptr(0, true, true),
        ], 
        vec![ // 14 
            Com(1,39),
            Arg(0, true),
        ], 
        // AExp10
        vec![ // 15 
            Com(1,23),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 16 
            Com(1,13),
            Arg(0, false),
        ], 
        // AExp11
        vec![ // 17 
            Com(2,0),
        ], 
        // AExp12
        vec![ // 18 
            Arg(0, true),
            Com(2,1),
            Com(2,17),
        ], 
        // AExp13
        vec![ // 19 
            Com(2,0),
        ], 
        // AExp14
        vec![ // 20 
            Com(2,0),
        ], 
        // AExp15
        vec![ // 21 
            Prm(EQ,false),
            Arg(2, true),
            Arg(0, true),
            Com(2,20),
            Com(1,23),
            Arg(3, true),
            Arg(1, true),
        ], 
        // AExp16
        vec![ // 22 
            Arg(2, true),
            Com(2,19),
            Com(4,21),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp17
        vec![ // 23 
            Arg(0, true),
            Com(1,18),
            Com(3,22),
        ], 
        // AExp18
        vec![ // 24 
            Com(1,36),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 25 
            Com(1,29),
            Arg(1, true),
        ], 
        vec![ // 26 
            Com(1,29),
            Arg(0, true),
        ], 
        // AExp19
        vec![ // 27 
            Com(4,30),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 28 
            Com(2,24),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp20
        vec![ // 29 
            Arg(0, true),
            Com(3,27),
            Com(2,0),
        ], 
        // AExp21
        vec![ // 30 
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp22
        vec![ // 31 
            Com(4,30),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 32 
            Com(1,36),
            Arg(2, true),
            Arg(1, true),
        ], 
        // AExp23
        vec![ // 33 
            Com(4,30),
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 34 
            Com(3,31),
            Arg(0, true),
            Arg(1, true),
            Arg(3, true),
        ], 
        // AExp24
        vec![ // 35 
            Arg(2, true),
            Com(4,30),
            Com(4,33),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp25
        vec![ // 36 
            Arg(0, true),
            Com(1,0),
            Com(3,35),
        ], 
        // AExp26
        vec![ // 37 
            Com(2,43),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 38 
            Com(1,39),
            Arg(1, true),
        ], 
        // AExp27
        vec![ // 39 
            Arg(0, true),
            Com(2,1),
            Com(2,37),
        ], 
        // AExp28
        vec![ // 40 
            Com(5,44),
            Arg(3, true),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 41 
            Com(2,43),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp29
        vec![ // 42 
            Com(5,44),
            Arg(0, true),
            Com(2,1),
            Com(2,1),
        ], 
        // AExp30
        vec![ // 43 
            Arg(1, true),
            Com(4,40),
            Com(1,42),
            Arg(0, true),
        ], 
        // AExp31
        vec![ // 44 
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp32
        vec![ // 45 
            Com(2,0),
        ], 
        // AExp33
        vec![ // 46 
            Com(2,50),
            Ptr(0, true, true),
        ], 
        vec![ // 47 
            Prm(Sub,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp34
        vec![ // 48 
            Com(4,30),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 49 
            Com(1,46),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp35
        vec![ // 50 
            Prm(LE,false),
            Arg(0, false),
            Int(0),
            Com(1,0),
            Com(1,45),
            Ptr(0, true, true),
        ], 
        vec![ // 51 
            Com(2,48),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp36
        vec![ // 52 
            Com(2,0),
        ], 
        // AExp37
        vec![ // 53 
            Com(2,57),
            Ptr(0, true, true),
        ], 
        vec![ // 54 
            Prm(Add,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp38
        vec![ // 55 
            Com(4,30),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 56 
            Com(1,53),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp39
        vec![ // 57 
            Prm(LE,false),
            Arg(0, false),
            Arg(1, false),
            Com(2,52),
            Com(2,55),
            Arg(0, false),
            Arg(1, false),
        ], 
    ],

}});