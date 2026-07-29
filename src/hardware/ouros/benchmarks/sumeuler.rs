use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 40
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
            Com(2,71),
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
            Com(2,71),
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
            Arg(3, true),
        ], 
        // AExp18
        vec![ // 34 
            Com(4,21),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 35 
            Com(1,40),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp19
        vec![ // 36 
            Arg(0, false),
            Arg(2, false),
            Com(4,33),
            Com(4,34),
            Arg(2, false),
            Arg(0, false),
            Arg(3, false),
            Ptr(0, true, true),
        ], 
        vec![ // 37 
            Arg(1, true),
            Arg(3, false),
        ], 
        // AExp20
        vec![ // 38 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 39 
            Com(4,36),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp21
        vec![ // 40 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 41 
            Com(3,38),
            Arg(0, true),
        ], 
        // AExp22
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
        // AExp23
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
        // AExp24
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
        // AExp25
        vec![ // 48 
            Com(1,64),
            Arg(0, true),
            Arg(1, true),
            Com(2,1),
        ], 
        // AExp26
        vec![ // 49 
            Arg(0, true),
            Arg(2, false),
            Ptr(0, true, true),
        ], 
        vec![ // 50 
            Arg(1, true),
            Arg(2, false),
        ], 
        // AExp27
        vec![ // 51 
            Arg(2, true),
            Int(0),
            Arg(0, true),
        ], 
        // AExp28
        vec![ // 52 
            Arg(2, true),
            Int(1),
            Ptr(0, true, true),
        ], 
        vec![ // 53 
            Prm(Sub,false),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp29
        vec![ // 54 
            Prm(LE,false),
            Arg(2, false),
            Arg(0, false),
            Com(3,51),
            Com(3,52),
            Arg(0, false),
            Arg(2, false),
        ], 
        // AExp30
        vec![ // 55 
            Arg(3, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 56 
            Prm(Sub,false),
            Arg(1, true),
            Arg(2, true),
        ], 
        vec![ // 57 
            Prm(Add,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp31
        vec![ // 58 
            Prm(LE,false),
            Arg(0, false),
            Arg(2, false),
            Com(4,21),
            Com(4,55),
            Ptr(0, true, true),
            Arg(2, false),
            Arg(0, false),
        ], 
        vec![ // 59 
            Prm(Add,false),
            Arg(1, false),
            Arg(1, false),
        ], 
        // AExp32
        vec![ // 60 
            Com(1,64),
            Arg(0, true),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 61 
            Com(3,58),
            Arg(2, true),
        ], 
        // AExp33
        vec![ // 62 
            Prm(LE,false),
            Arg(2, false),
            Arg(0, false),
            Com(3,54),
            Com(3,60),
            Arg(0, false),
            Arg(2, false),
            Arg(1, true),
        ], 
        // AExp34
        vec![ // 63 
            Prm(Add,false),
            Arg(0, false),
            Arg(0, false),
        ], 
        // AExp35
        vec![ // 64 
            Com(3,49),
            Ptr(0, true, true),
            Com(1,63),
        ], 
        vec![ // 65 
            Com(3,62),
            Arg(0, true),
        ], 
        // AExp36
        vec![ // 66 
            Com(2,0),
        ], 
        // AExp37
        vec![ // 67 
            Com(2,71),
            Ptr(0, true, true),
        ], 
        vec![ // 68 
            Prm(Add,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp38
        vec![ // 69 
            Com(4,21),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 70 
            Com(1,67),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp39
        vec![ // 71 
            Prm(LE,false),
            Arg(0, false),
            Arg(1, false),
            Com(2,66),
            Com(2,69),
            Arg(0, false),
            Arg(1, false),
        ], 
    ],

}});