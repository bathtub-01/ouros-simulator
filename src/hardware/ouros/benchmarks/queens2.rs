use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 49
#[rustfmt::skip]
pub static QUEENS2: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            Com(1,4),
            Int(11),
        ], 
        // AExp1
        vec![ // 1 
            Com(2,37),
            Ptr(5, false, false),
        ], 
        vec![ // 2 
            Com(4,14),
            Int(2),
            Com(2,0),
        ], 
        vec![ // 3 
            Com(4,14),
            Int(1),
            Ptr(2, false, false),
        ], 
        vec![ // 4 
            Com(1,40),
            Ptr(3, false, false),
        ], 
        vec![ // 5 
            Com(2,38),
            Ptr(4, false, false),
        ], 
        // AExp2
        vec![ // 6 
            Com(1,47),
            Ptr(8, false, false),
        ], 
        vec![ // 7 
            Prm(EQ,false),
            Int(2),
        ], 
        vec![ // 8 
            Com(2,72),
            Ptr(7, false, false),
        ], 
        // AExp3
        vec![ // 9 
            Com(2,73),
            Ptr(11, false, false),
        ], 
        vec![ // 10 
            Prm(EQ,false),
            Int(0),
        ], 
        vec![ // 11 
            Com(2,72),
            Ptr(10, false, false),
        ], 
        // AExp4
        vec![ // 12 
            Com(2,66),
            Ptr(14, false, false),
        ], 
        vec![ // 13 
            Prm(EQ,false),
            Int(1),
        ], 
        vec![ // 14 
            Com(2,72),
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
            Com(1,16),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 3 
            Com(2,81),
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
            Arg(3, true),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 7 
            Prm(Add,false),
            Arg(2, true),
            Int(1),
        ], 
        // AExp5
        vec![ // 8 
            Arg(2, true),
            Com(2,0),
            Com(4,6),
            Arg(1, true),
            Arg(0, true),
        ], 
        // AExp6
        vec![ // 9 
            Com(2,51),
            Ptr(0, true, true),
        ], 
        vec![ // 10 
            Prm(Sub,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp7
        vec![ // 11 
            Com(1,22),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 12 
            Com(1,35),
            Arg(1, true),
        ], 
        vec![ // 13 
            Com(1,9),
            Arg(0, true),
        ], 
        // AExp8
        vec![ // 14 
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp9
        vec![ // 15 
            Com(4,14),
            Com(2,0),
            Com(2,0),
        ], 
        // AExp10
        vec![ // 16 
            Prm(EQ,false),
            Arg(0, false),
            Int(0),
            Com(2,11),
            Com(2,15),
            Arg(0, false),
        ], 
        // AExp11
        vec![ // 17 
            Com(2,0),
        ], 
        // AExp12
        vec![ // 18 
            Com(2,27),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 19 
            Arg(3, true),
            Arg(1, true),
        ], 
        vec![ // 20 
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp13
        vec![ // 21 
            Arg(2, true),
            Com(2,17),
            Com(4,18),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp14
        vec![ // 22 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 23 
            Com(3,21),
            Arg(0, true),
        ], 
        // AExp15
        vec![ // 24 
            Com(4,14),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 25 
            Arg(3, true),
            Arg(1, true),
        ], 
        // AExp16
        vec![ // 26 
            Arg(2, true),
            Com(2,0),
            Com(4,24),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp17
        vec![ // 27 
            Y,
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 28 
            Com(3,26),
            Arg(1, true),
        ], 
        // AExp18
        vec![ // 29 
            Com(1,47),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 30 
            Com(1,35),
            Arg(1, true),
        ], 
        vec![ // 31 
            Com(4,14),
            Arg(0, true),
        ], 
        // AExp19
        vec![ // 32 
            Com(2,27),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 33 
            Com(2,29),
            Arg(0, false),
            Arg(1, false),
        ], 
        vec![ // 34 
            Ptr(1, false, false),
            Arg(0, false),
            Arg(1, false),
        ], 
        // AExp20
        vec![ // 35 
            Arg(0, true),
            Com(2,0),
            Com(2,32),
        ], 
        // AExp21
        vec![ // 36 
            Com(2,0),
        ], 
        // AExp22
        vec![ // 37 
            Arg(1, true),
            Arg(0, true),
            Com(3,36),
        ], 
        // AExp23
        vec![ // 38 
            Com(4,14),
            Ptr(0, true, true),
            Com(2,0),
        ], 
        vec![ // 39 
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp24
        vec![ // 40 
            Com(4,14),
            Ptr(0, true, true),
        ], 
        vec![ // 41 
            Com(4,14),
            Int(0),
            Arg(0, true),
        ], 
        // AExp25
        vec![ // 42 
            Com(2,0),
        ], 
        // AExp26
        vec![ // 43 
            Com(4,14),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 44 
            Arg(3, true),
            Arg(1, true),
        ], 
        vec![ // 45 
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp27
        vec![ // 46 
            Arg(2, true),
            Com(2,42),
            Com(4,43),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp28
        vec![ // 47 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 48 
            Com(3,46),
            Arg(0, true),
        ], 
        // AExp29
        vec![ // 49 
            Com(1,16),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 50 
            Com(1,57),
            Arg(1, true),
        ], 
        // AExp30
        vec![ // 51 
            Com(1,47),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 52 
            Com(2,49),
            Arg(0, true),
            Arg(1, false),
        ], 
        vec![ // 53 
            Com(4,14),
            Arg(1, false),
        ], 
        // AExp31
        vec![ // 54 
            Com(1,65),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 55 
            Ptr(9, false, false),
            Arg(0, false),
        ], 
        vec![ // 56 
            Ptr(6, false, false),
            Arg(0, false),
        ], 
        // AExp32
        vec![ // 57 
            Com(1,65),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 58 
            Ptr(12, false, false),
            Arg(0, false),
        ], 
        vec![ // 59 
            Com(1,54),
            Arg(0, false),
        ], 
        // AExp33
        vec![ // 60 
            Com(2,0),
        ], 
        // AExp34
        vec![ // 61 
            Com(4,14),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 62 
            Com(1,65),
            Arg(3, true),
            Arg(1, true),
        ], 
        vec![ // 63 
            Com(2,27),
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp35
        vec![ // 64 
            Arg(2, true),
            Com(4,14),
            Com(4,61),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp36
        vec![ // 65 
            Arg(0, true),
            Com(1,60),
            Com(3,64),
        ], 
        // AExp37
        vec![ // 66 
            Com(4,14),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 67 
            Com(1,47),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp38
        vec![ // 68 
            Com(2,0),
        ], 
        // AExp39
        vec![ // 69 
            Com(2,72),
        ], 
        // AExp40
        vec![ // 70 
            Com(4,14),
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp41
        vec![ // 71 
            Arg(2, false),
            Arg(0, false),
            Com(1,69),
            Com(3,70),
            Arg(0, false),
            Arg(2, false),
            Arg(1, true),
        ], 
        // AExp42
        vec![ // 72 
            Arg(1, true),
            Com(1,68),
            Com(3,71),
            Arg(0, true),
        ], 
        // AExp43
        vec![ // 73 
            Com(1,47),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 74 
            Com(1,75),
            Arg(1, true),
        ], 
        // AExp44
        vec![ // 75 
            Arg(0, true),
            Com(2,0),
            Com(2,1),
        ], 
        // AExp45
        vec![ // 76 
            Com(2,0),
        ], 
        // AExp46
        vec![ // 77 
            Com(2,81),
            Ptr(0, true, true),
        ], 
        vec![ // 78 
            Prm(Sub,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp47
        vec![ // 79 
            Com(4,14),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 80 
            Com(1,77),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp48
        vec![ // 81 
            Prm(LE,false),
            Arg(0, false),
            Int(0),
            Com(1,0),
            Com(1,76),
            Ptr(0, true, true),
        ], 
        vec![ // 82 
            Com(2,79),
            Arg(0, false),
            Arg(1, true),
        ], 
    ],

}});