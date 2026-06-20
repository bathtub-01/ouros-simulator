use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 42
#[rustfmt::skip]
pub static QUEENS2: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            Com(1,4),
            Int(5),
        ], 
        // AExp1
        vec![ // 1 
            Com(3,40),
            Ptr(5, false, false),
        ], 
        vec![ // 2 
            Com(4,15),
            Int(2),
            Com(2,0),
        ], 
        vec![ // 3 
            Com(4,15),
            Int(1),
            Ptr(2, false, false),
        ], 
        vec![ // 4 
            Com(1,44),
            Ptr(3, false, false),
        ], 
        vec![ // 5 
            Com(2,42),
            Ptr(4, false, false),
        ], 
        // AExp2
        vec![ // 6 
            Com(1,51),
            Ptr(8, false, false),
        ], 
        vec![ // 7 
            Prm(EQ,false),
            Int(2),
        ], 
        vec![ // 8 
            Com(1,79),
            Ptr(7, false, false),
        ], 
        // AExp3
        vec![ // 9 
            Com(2,81),
            Ptr(11, false, false),
        ], 
        vec![ // 10 
            Prm(EQ,false),
            Int(0),
        ], 
        vec![ // 11 
            Com(1,79),
            Ptr(10, false, false),
        ], 
        // AExp4
        vec![ // 12 
            Com(2,72),
            Ptr(14, false, false),
        ], 
        vec![ // 13 
            Prm(EQ,false),
            Int(1),
        ], 
        vec![ // 14 
            Com(1,79),
            Ptr(13, false, false),
        ], 
        // AExp5
        vec![ // 15 
            Y,
            Com(3,8),
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
            Com(2,16),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 3 
            Com(2,88),
            Arg(0, false),
            Com(2,0),
        ], 
        // AExp3
        vec![ // 4 
            Ptr(15, false, false),
            Ptr(0, true, true),
        ], 
        vec![ // 5 
            Com(1,2),
            Arg(0, true),
        ], 
        // AExp4
        vec![ // 6 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 7 
            Prm(Add,false),
            Arg(1, true),
            Int(1),
        ], 
        // AExp5
        vec![ // 8 
            Arg(2, true),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 9 
            Com(3,6),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp6
        vec![ // 10 
            Com(2,55),
            Ptr(0, true, true),
        ], 
        vec![ // 11 
            Prm(Sub,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp7
        vec![ // 12 
            Com(1,24),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 13 
            Com(1,38),
            Arg(1, true),
        ], 
        vec![ // 14 
            Com(1,10),
            Arg(0, true),
        ], 
        // AExp8
        vec![ // 15 
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp9
        vec![ // 16 
            Prm(EQ,false),
            Arg(0, false),
            Int(0),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 17 
            Com(4,15),
            Com(2,0),
            Com(2,0),
        ], 
        vec![ // 18 
            Com(2,12),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp10
        vec![ // 19 
            Com(2,30),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 20 
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 21 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp11
        vec![ // 22 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 23 
            Com(4,19),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp12
        vec![ // 24 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 25 
            Com(3,22),
            Arg(0, true),
        ], 
        // AExp13
        vec![ // 26 
            Com(4,15),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 27 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp14
        vec![ // 28 
            Arg(2, true),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 29 
            Com(3,26),
            Arg(1, true),
        ], 
        // AExp15
        vec![ // 30 
            Y,
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 31 
            Com(3,28),
            Arg(1, true),
        ], 
        // AExp16
        vec![ // 32 
            Com(1,51),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 33 
            Com(1,38),
            Arg(1, true),
        ], 
        vec![ // 34 
            Com(4,15),
            Arg(0, true),
        ], 
        // AExp17
        vec![ // 35 
            Com(2,30),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 36 
            Com(2,32),
            Arg(0, false),
            Arg(1, false),
        ], 
        vec![ // 37 
            Ptr(1, false, false),
            Arg(0, false),
            Arg(1, false),
        ], 
        // AExp18
        vec![ // 38 
            Arg(0, true),
            Com(2,0),
            Com(2,35),
        ], 
        // AExp19
        vec![ // 39 
            Com(2,0),
        ], 
        // AExp20
        vec![ // 40 
            Arg(1, true),
            Ptr(0, true, true),
            Com(2,39),
        ], 
        vec![ // 41 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp21
        vec![ // 42 
            Com(4,15),
            Ptr(0, true, true),
            Com(2,0),
        ], 
        vec![ // 43 
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp22
        vec![ // 44 
            Com(4,15),
            Ptr(0, true, true),
        ], 
        vec![ // 45 
            Com(4,15),
            Int(0),
            Arg(0, true),
        ], 
        // AExp23
        vec![ // 46 
            Com(4,15),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 47 
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 48 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp24
        vec![ // 49 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 50 
            Com(4,46),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp25
        vec![ // 51 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 52 
            Com(3,49),
            Arg(0, true),
        ], 
        // AExp26
        vec![ // 53 
            Com(2,16),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 54 
            Com(1,61),
            Arg(1, true),
        ], 
        // AExp27
        vec![ // 55 
            Com(1,51),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 56 
            Com(2,53),
            Arg(0, true),
            Arg(1, false),
        ], 
        vec![ // 57 
            Com(4,15),
            Arg(1, false),
        ], 
        // AExp28
        vec![ // 58 
            Com(2,70),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 59 
            Ptr(9, false, false),
            Arg(0, false),
        ], 
        vec![ // 60 
            Ptr(6, false, false),
            Arg(0, false),
        ], 
        // AExp29
        vec![ // 61 
            Com(2,70),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 62 
            Ptr(12, false, false),
            Arg(0, false),
        ], 
        vec![ // 63 
            Com(1,58),
            Arg(0, false),
        ], 
        // AExp30
        vec![ // 64 
            Com(4,15),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 65 
            Com(2,70),
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 66 
            Com(2,30),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp31
        vec![ // 67 
            Arg(0, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 68 
            Com(4,64),
            Arg(1, false),
            Arg(2, false),
        ], 
        vec![ // 69 
            Com(4,15),
            Arg(1, false),
            Arg(2, false),
        ], 
        // AExp32
        vec![ // 70 
            Arg(0, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 71 
            Com(3,67),
            Arg(1, true),
        ], 
        // AExp33
        vec![ // 72 
            Com(4,15),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 73 
            Com(1,51),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp34
        vec![ // 74 
            Arg(0, true),
            Arg(2, false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 75 
            Com(4,15),
            Arg(2, false),
            Com(2,0),
        ], 
        vec![ // 76 
            Arg(1, true),
            Arg(3, true),
        ], 
        // AExp35
        vec![ // 77 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 78 
            Com(4,74),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp36
        vec![ // 79 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 80 
            Com(3,77),
            Arg(0, true),
        ], 
        // AExp37
        vec![ // 81 
            Com(1,51),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 82 
            Com(1,83),
            Arg(1, true),
        ], 
        // AExp38
        vec![ // 83 
            Arg(0, true),
            Com(2,0),
            Com(2,1),
        ], 
        // AExp39
        vec![ // 84 
            Com(2,88),
            Ptr(0, true, true),
        ], 
        vec![ // 85 
            Prm(Sub,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp40
        vec![ // 86 
            Com(4,15),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 87 
            Com(1,84),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp41
        vec![ // 88 
            Prm(LE,false),
            Arg(0, false),
            Int(0),
            Ptr(0, true, true),
            Com(2,0),
        ], 
        vec![ // 89 
            Com(2,86),
            Arg(0, false),
            Arg(1, true),
        ], 
    ],

}});
