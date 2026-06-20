use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 39
#[rustfmt::skip]
pub static SUMEULER: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            Ptr(2, false, false),
            Ptr(1, false, false),
        ], 
        vec![ // 1 
            Com(2,12),
            Int(1),
            Int(30),
        ], 
        // AExp1
        vec![ // 2 
            Com(2,6),
            Prm(Add,false),
            Int(0),
        ], 
        // AExp2
        vec![ // 3 
            Y,
            Com(3,31),
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
            Arg(0, true),
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 3 
            Arg(1, true),
            Arg(3, true),
        ], 
        // AExp3
        vec![ // 4 
            Arg(3, true),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 5 
            Com(4,2),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp4
        vec![ // 6 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 7 
            Com(4,4),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp5
        vec![ // 8 
            Com(2,78),
            Ptr(0, true, true),
        ], 
        vec![ // 9 
            Prm(Add,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp6
        vec![ // 10 
            Com(4,21),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 11 
            Com(1,8),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp7
        vec![ // 12 
            Com(1,19),
            Com(1,27),
            Ptr(0, true, true),
        ], 
        vec![ // 13 
            Com(2,10),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp8
        vec![ // 14 
            Com(4,21),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 15 
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 16 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp9
        vec![ // 17 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 18 
            Com(4,14),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp10
        vec![ // 19 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 20 
            Com(3,17),
            Arg(0, true),
        ], 
        // AExp11
        vec![ // 21 
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp12
        vec![ // 22 
            Com(2,78),
            Int(1),
            Ptr(0, true, true),
        ], 
        vec![ // 23 
            Prm(Sub,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp13
        vec![ // 24 
            Com(1,40),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 25 
            Com(1,22),
            Arg(0, false),
        ], 
        vec![ // 26 
            Com(2,42),
            Arg(0, false),
        ], 
        // AExp14
        vec![ // 27 
            Ptr(3, false, false),
            Ptr(0, true, true),
        ], 
        vec![ // 28 
            Com(1,24),
            Arg(0, true),
        ], 
        // AExp15
        vec![ // 29 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 30 
            Prm(Add,false),
            Arg(1, true),
            Int(1),
        ], 
        // AExp16
        vec![ // 31 
            Arg(2, true),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 32 
            Com(3,29),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp17
        vec![ // 33 
            Com(4,21),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 34 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp18
        vec![ // 35 
            Arg(0, true),
            Arg(2, false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 36 
            Com(3,33),
            Arg(1, false),
            Arg(2, false),
            Arg(3, false),
        ], 
        vec![ // 37 
            Arg(1, false),
            Arg(3, false),
        ], 
        // AExp19
        vec![ // 38 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 39 
            Com(4,35),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp20
        vec![ // 40 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 41 
            Com(3,38),
            Arg(0, true),
        ], 
        // AExp21
        vec![ // 42 
            Prm(EQ,false),
            Ptr(0, true, true),
            Int(1),
        ], 
        vec![ // 43 
            Com(2,46),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp22
        vec![ // 44 
            Com(2,46),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 45 
            Com(2,48),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp23
        vec![ // 46 
            Prm(EQ,false),
            Int(0),
            Arg(1, false),
            Ptr(0, true, true),
            Arg(0, false),
        ], 
        vec![ // 47 
            Com(2,44),
            Arg(0, false),
            Arg(1, false),
        ], 
        // AExp24
        vec![ // 48 
            Com(1,72),
            Arg(0, true),
            Arg(1, true),
            Com(2,1),
        ], 
        // AExp25
        vec![ // 49 
            Prm(LE,false),
            Arg(5, false),
            Arg(1, false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 50 
            Arg(3, true),
            Arg(5, false),
        ], 
        vec![ // 51 
            Prm(LE,false),
            Arg(4, true),
            Arg(1, false),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp26
        vec![ // 52 
            Arg(1, true),
            Int(0),
            Arg(0, true),
        ], 
        // AExp27
        vec![ // 53 
            Arg(2, true),
            Int(1),
            Ptr(0, true, true),
        ], 
        vec![ // 54 
            Prm(Sub,false),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp28
        vec![ // 55 
            Arg(2, true),
            Arg(1, true),
            Arg(0, true),
        ], 
        // AExp29
        vec![ // 56 
            Arg(3, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 57 
            Prm(Sub,false),
            Arg(1, true),
            Arg(0, true),
        ], 
        vec![ // 58 
            Prm(Add,false),
            Arg(2, true),
            Int(1),
        ], 
        // AExp30
        vec![ // 59 
            Prm(LE,false),
            Arg(0, false),
            Arg(2, false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 60 
            Com(4,56),
            Arg(0, false),
            Arg(2, false),
            Arg(1, false),
        ], 
        vec![ // 61 
            Com(3,55),
            Arg(2, false),
            Arg(1, false),
        ], 
        // AExp31
        vec![ // 62 
            Prm(Add,false),
            Arg(0, false),
            Arg(0, false),
        ], 
        // AExp32
        vec![ // 63 
            Com(3,59),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 64 
            Com(1,62),
            Arg(1, true),
        ], 
        // AExp33
        vec![ // 65 
            Arg(0, true),
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 66 
            Com(2,63),
            Arg(1, true),
        ], 
        // AExp34
        vec![ // 67 
            Com(6,49),
            Ptr(3, true, true),
            Arg(0, false),
            Ptr(2, true, true),
            Ptr(1, true, true),
            Arg(2, false),
            Ptr(0, true, true),
        ], 
        vec![ // 68 
            Prm(Add,false),
            Arg(2, false),
            Arg(2, false),
        ], 
        vec![ // 69 
            Com(3,65),
            Arg(1, true),
            Arg(2, false),
        ], 
        vec![ // 70 
            Com(3,53),
            Arg(0, false),
            Arg(2, false),
        ], 
        vec![ // 71 
            Com(2,52),
            Arg(0, false),
        ], 
        // AExp35
        vec![ // 72 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 73 
            Com(3,67),
            Arg(0, true),
        ], 
        // AExp36
        vec![ // 74 
            Com(2,78),
            Ptr(0, true, true),
        ], 
        vec![ // 75 
            Prm(Add,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp37
        vec![ // 76 
            Com(4,21),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 77 
            Com(1,74),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp38
        vec![ // 78 
            Prm(LE,false),
            Arg(0, false),
            Arg(1, false),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 79 
            Com(2,76),
            Arg(0, false),
            Arg(1, false),
        ], 
    ],

}});
