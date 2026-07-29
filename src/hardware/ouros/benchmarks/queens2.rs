use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 45
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
            Com(3,39),
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
            Com(1,43),
            Ptr(3, false, false),
        ], 
        vec![ // 5 
            Com(2,41),
            Ptr(4, false, false),
        ], 
        // AExp2
        vec![ // 6 
            Com(1,50),
            Ptr(8, false, false),
        ], 
        vec![ // 7 
            Prm(EQ,false),
            Int(2),
        ], 
        vec![ // 8 
            Com(2,76),
            Ptr(7, false, false),
        ], 
        // AExp3
        vec![ // 9 
            Com(2,78),
            Ptr(11, false, false),
        ], 
        vec![ // 10 
            Prm(EQ,false),
            Int(0),
        ], 
        vec![ // 11 
            Com(2,76),
            Ptr(10, false, false),
        ], 
        // AExp4
        vec![ // 12 
            Com(2,71),
            Ptr(14, false, false),
        ], 
        vec![ // 13 
            Prm(EQ,false),
            Int(1),
        ], 
        vec![ // 14 
            Com(2,76),
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
            Com(1,17),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 3 
            Com(2,86),
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
            Com(2,54),
            Ptr(0, true, true),
        ], 
        vec![ // 11 
            Prm(Sub,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp7
        vec![ // 12 
            Com(1,23),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 13 
            Com(1,37),
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
            Com(4,15),
            Com(2,0),
            Com(2,0),
        ], 
        // AExp10
        vec![ // 17 
            Prm(EQ,false),
            Arg(0, false),
            Int(0),
            Com(2,12),
            Com(2,16),
            Arg(0, false),
        ], 
        // AExp11
        vec![ // 18 
            Com(2,29),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 19 
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 20 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp12
        vec![ // 21 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 22 
            Com(4,18),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp13
        vec![ // 23 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 24 
            Com(3,21),
            Arg(0, true),
        ], 
        // AExp14
        vec![ // 25 
            Com(4,15),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 26 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp15
        vec![ // 27 
            Arg(2, true),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 28 
            Com(3,25),
            Arg(1, true),
        ], 
        // AExp16
        vec![ // 29 
            Y,
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 30 
            Com(3,27),
            Arg(1, true),
        ], 
        // AExp17
        vec![ // 31 
            Com(1,50),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 32 
            Com(1,37),
            Arg(1, true),
        ], 
        vec![ // 33 
            Com(4,15),
            Arg(0, true),
        ], 
        // AExp18
        vec![ // 34 
            Com(2,29),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 35 
            Com(2,31),
            Arg(0, false),
            Arg(1, false),
        ], 
        vec![ // 36 
            Ptr(1, false, false),
            Arg(0, false),
            Arg(1, false),
        ], 
        // AExp19
        vec![ // 37 
            Arg(0, true),
            Com(2,0),
            Com(2,34),
        ], 
        // AExp20
        vec![ // 38 
            Com(2,0),
        ], 
        // AExp21
        vec![ // 39 
            Arg(1, true),
            Ptr(0, true, true),
            Com(2,38),
        ], 
        vec![ // 40 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp22
        vec![ // 41 
            Com(4,15),
            Ptr(0, true, true),
            Com(2,0),
        ], 
        vec![ // 42 
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp23
        vec![ // 43 
            Com(4,15),
            Ptr(0, true, true),
        ], 
        vec![ // 44 
            Com(4,15),
            Int(0),
            Arg(0, true),
        ], 
        // AExp24
        vec![ // 45 
            Com(4,15),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 46 
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 47 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp25
        vec![ // 48 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 49 
            Com(4,45),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp26
        vec![ // 50 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 51 
            Com(3,48),
            Arg(0, true),
        ], 
        // AExp27
        vec![ // 52 
            Com(1,17),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 53 
            Com(1,60),
            Arg(1, true),
        ], 
        // AExp28
        vec![ // 54 
            Com(1,50),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 55 
            Com(2,52),
            Arg(0, true),
            Arg(1, false),
        ], 
        vec![ // 56 
            Com(4,15),
            Arg(1, false),
        ], 
        // AExp29
        vec![ // 57 
            Com(2,69),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 58 
            Ptr(9, false, false),
            Arg(0, false),
        ], 
        vec![ // 59 
            Ptr(6, false, false),
            Arg(0, false),
        ], 
        // AExp30
        vec![ // 60 
            Com(2,69),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 61 
            Ptr(12, false, false),
            Arg(0, false),
        ], 
        vec![ // 62 
            Com(1,57),
            Arg(0, false),
        ], 
        // AExp31
        vec![ // 63 
            Com(4,15),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 64 
            Com(2,69),
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 65 
            Com(2,29),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp32
        vec![ // 66 
            Arg(0, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 67 
            Com(4,63),
            Arg(1, false),
            Arg(2, false),
        ], 
        vec![ // 68 
            Com(4,15),
            Arg(1, false),
            Arg(2, false),
        ], 
        // AExp33
        vec![ // 69 
            Arg(0, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 70 
            Com(3,66),
            Arg(1, true),
        ], 
        // AExp34
        vec![ // 71 
            Com(4,15),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 72 
            Com(1,50),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp35
        vec![ // 73 
            Com(2,76),
        ], 
        // AExp36
        vec![ // 74 
            Com(4,15),
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp37
        vec![ // 75 
            Arg(0, false),
            Arg(1, false),
            Com(1,73),
            Com(3,74),
            Arg(1, false),
            Arg(0, false),
        ], 
        // AExp38
        vec![ // 76 
            Arg(1, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 77 
            Com(2,75),
            Arg(0, true),
        ], 
        // AExp39
        vec![ // 78 
            Com(1,50),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 79 
            Com(1,80),
            Arg(1, true),
        ], 
        // AExp40
        vec![ // 80 
            Arg(0, true),
            Com(2,0),
            Com(2,1),
        ], 
        // AExp41
        vec![ // 81 
            Com(2,0),
        ], 
        // AExp42
        vec![ // 82 
            Com(2,86),
            Ptr(0, true, true),
        ], 
        vec![ // 83 
            Prm(Sub,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp43
        vec![ // 84 
            Com(4,15),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 85 
            Com(1,82),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp44
        vec![ // 86 
            Prm(LE,false),
            Arg(0, false),
            Int(0),
            Com(1,0),
            Com(1,81),
            Ptr(0, true, true),
        ], 
        vec![ // 87 
            Com(2,84),
            Arg(0, false),
            Arg(1, true),
        ], 
    ],

}});