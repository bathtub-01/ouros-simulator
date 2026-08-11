use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 42
#[rustfmt::skip]
pub static SUMEULER: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            Ptr(2, false, false),
            Ptr(1, false, false),
        ], 
        vec![ // 1 
            Com(2,11),
            Int(1),
            Int(300),
        ], 
        // AExp1
        vec![ // 2 
            Com(2,5),
            Prm(Add,false),
            Int(0),
        ], 
        // AExp2
        vec![ // 3 
            Y,
            Com(3,30),
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
            Arg(3, true),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 3 
            Arg(4, true),
            Arg(1, true),
        ], 
        // AExp3
        vec![ // 4 
            Arg(3, true),
            Com(3,0),
            Com(5,2),
            Arg(1, true),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp4
        vec![ // 5 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 6 
            Com(4,4),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp5
        vec![ // 7 
            Com(2,70),
            Ptr(0, true, true),
        ], 
        vec![ // 8 
            Prm(Add,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp6
        vec![ // 9 
            Com(4,20),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 10 
            Com(1,7),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp7
        vec![ // 11 
            Com(1,18),
            Com(1,26),
            Ptr(0, true, true),
        ], 
        vec![ // 12 
            Com(2,9),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp8
        vec![ // 13 
            Com(2,0),
        ], 
        // AExp9
        vec![ // 14 
            Com(4,20),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 15 
            Arg(3, true),
            Arg(1, true),
        ], 
        vec![ // 16 
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp10
        vec![ // 17 
            Arg(2, true),
            Com(2,13),
            Com(4,14),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp11
        vec![ // 18 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 19 
            Com(3,17),
            Arg(0, true),
        ], 
        // AExp12
        vec![ // 20 
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp13
        vec![ // 21 
            Com(2,70),
            Int(1),
            Ptr(0, true, true),
        ], 
        vec![ // 22 
            Prm(Sub,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp14
        vec![ // 23 
            Com(1,39),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 24 
            Com(1,21),
            Arg(0, false),
        ], 
        vec![ // 25 
            Com(2,41),
            Arg(0, false),
        ], 
        // AExp15
        vec![ // 26 
            Ptr(3, false, false),
            Ptr(0, true, true),
        ], 
        vec![ // 27 
            Com(1,23),
            Arg(0, true),
        ], 
        // AExp16
        vec![ // 28 
            Arg(3, true),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 29 
            Prm(Add,false),
            Arg(2, true),
            Int(1),
        ], 
        // AExp17
        vec![ // 30 
            Arg(2, true),
            Com(2,0),
            Com(4,28),
            Arg(1, true),
            Arg(0, true),
        ], 
        // AExp18
        vec![ // 31 
            Com(2,0),
        ], 
        // AExp19
        vec![ // 32 
            Arg(3, true),
        ], 
        // AExp20
        vec![ // 33 
            Com(4,20),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 34 
            Com(1,39),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp21
        vec![ // 35 
            Arg(3, false),
            Arg(1, false),
            Com(4,32),
            Com(4,33),
            Arg(1, false),
            Arg(3, false),
            Arg(2, false),
            Ptr(0, true, true),
        ], 
        vec![ // 36 
            Arg(0, true),
            Arg(2, false),
        ], 
        // AExp22
        vec![ // 37 
            Arg(2, true),
            Com(1,31),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 38 
            Com(4,35),
            Arg(1, true),
        ], 
        // AExp23
        vec![ // 39 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 40 
            Com(3,37),
            Arg(0, true),
        ], 
        // AExp24
        vec![ // 41 
            Prm(EQ,false),
            Ptr(0, true, true),
            Int(1),
        ], 
        vec![ // 42 
            Com(2,45),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp25
        vec![ // 43 
            Com(2,45),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 44 
            Com(2,47),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp26
        vec![ // 45 
            Prm(EQ,false),
            Int(0),
            Arg(1, false),
            Com(2,0),
            Com(2,1),
            Ptr(0, true, true),
            Arg(0, false),
        ], 
        vec![ // 46 
            Com(2,43),
            Arg(0, false),
            Arg(1, false),
        ], 
        // AExp27
        vec![ // 47 
            Com(1,63),
            Arg(0, true),
            Arg(1, true),
            Com(2,1),
        ], 
        // AExp28
        vec![ // 48 
            Arg(0, true),
            Arg(2, false),
            Ptr(0, true, true),
        ], 
        vec![ // 49 
            Arg(1, true),
            Arg(2, false),
        ], 
        // AExp29
        vec![ // 50 
            Arg(2, true),
            Int(0),
            Arg(0, true),
        ], 
        // AExp30
        vec![ // 51 
            Arg(2, true),
            Int(1),
            Ptr(0, true, true),
        ], 
        vec![ // 52 
            Prm(Sub,false),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp31
        vec![ // 53 
            Prm(LE,false),
            Arg(2, false),
            Arg(0, false),
            Com(3,50),
            Com(3,51),
            Arg(0, false),
            Arg(2, false),
        ], 
        // AExp32
        vec![ // 54 
            Arg(3, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 55 
            Prm(Sub,false),
            Arg(1, true),
            Arg(2, true),
        ], 
        vec![ // 56 
            Prm(Add,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp33
        vec![ // 57 
            Prm(LE,false),
            Arg(0, false),
            Arg(2, false),
            Com(4,20),
            Com(4,54),
            Ptr(0, true, true),
            Arg(2, false),
            Arg(0, false),
        ], 
        vec![ // 58 
            Prm(Add,false),
            Arg(1, false),
            Arg(1, false),
        ], 
        // AExp34
        vec![ // 59 
            Com(1,63),
            Arg(0, true),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 60 
            Com(3,57),
            Arg(2, true),
        ], 
        // AExp35
        vec![ // 61 
            Prm(LE,false),
            Arg(2, false),
            Arg(0, false),
            Com(3,53),
            Com(3,59),
            Arg(0, false),
            Arg(2, false),
            Arg(1, true),
        ], 
        // AExp36
        vec![ // 62 
            Prm(Add,false),
            Arg(0, false),
            Arg(0, false),
        ], 
        // AExp37
        vec![ // 63 
            Com(3,48),
            Ptr(0, true, true),
            Com(1,62),
        ], 
        vec![ // 64 
            Com(3,61),
            Arg(0, true),
        ], 
        // AExp38
        vec![ // 65 
            Com(2,0),
        ], 
        // AExp39
        vec![ // 66 
            Com(2,70),
            Ptr(0, true, true),
        ], 
        vec![ // 67 
            Prm(Add,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp40
        vec![ // 68 
            Com(4,20),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 69 
            Com(1,66),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp41
        vec![ // 70 
            Prm(LE,false),
            Arg(0, false),
            Arg(1, false),
            Com(2,65),
            Com(2,68),
            Arg(0, false),
            Arg(1, false),
        ], 
    ],

}});