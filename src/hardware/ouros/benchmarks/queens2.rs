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
            PTR(1, false, false),
            INT(5),
        ], 
        // AExp1
        vec![ // 1 
            COM(2,2),
            COM(1,4),
        ], 
        // AExp2
        vec![ // 2 
            Y,
            PTR(3, false, false),
            INT(0),
        ], 
        vec![ // 3 
            COM(4,6),
            COM(3,8),
        ], 
        // AExp3
        vec![ // 4 
            COM(3,11),
            PTR(5, false, false),
        ], 
        vec![ // 5 
            COM(3,14),
            COM(1,17),
        ], 
        // AExp4
        vec![ // 6 
            COM(2,19),
            PTR(7, false, false),
        ], 
        vec![ // 7 
            COM(4,21),
            COM(4,23),
        ], 
        // AExp5
        vec![ // 8 
            COM(3,26),
            PTR(9, false, false),
        ], 
        vec![ // 9 
            COM(4,28),
            COM(3,30),
        ], 
        // AExp6
        vec![ // 10 
            COM(2,32),
            PTR(11, false, false),
        ], 
        vec![ // 11 
            COM(3,33),
            COM(2,36),
        ], 
        // AExp7
        vec![ // 12 
            COM(4,39),
            PTR(16, false, false),
            COM(2,45),
        ], 
        vec![ // 13 
            COM(4,10),
            INT(2),
            COM(2,0),
        ], 
        vec![ // 14 
            COM(4,10),
            INT(1),
            PTR(13, false, false),
        ], 
        vec![ // 15 
            COM(1,43),
            PTR(14, false, false),
        ], 
        vec![ // 16 
            COM(2,41),
            PTR(15, false, false),
        ], 
        // AExp8
        vec![ // 17 
            COM(2,46),
            PTR(18, false, false),
        ], 
        vec![ // 18 
            COM(4,48),
            COM(4,50),
        ], 
        // AExp9
        vec![ // 19 
            COM(3,53),
            COM(2,56),
        ], 
        // AExp10
        vec![ // 20 
            COM(2,58),
            COM(1,61),
        ], 
        // AExp11
        vec![ // 21 
            COM(3,64),
            PTR(22, false, false),
        ], 
        vec![ // 22 
            COM(4,66),
            COM(4,69),
        ], 
        // AExp12
        vec![ // 23 
            COM(2,72),
            PTR(25, false, false),
        ], 
        vec![ // 24 
            PRM(EQ,false),
            INT(1),
        ], 
        vec![ // 25 
            PTR(26, false, false),
            PTR(24, false, false),
        ], 
        // AExp13
        vec![ // 26 
            COM(2,74),
            PTR(27, false, false),
        ], 
        vec![ // 27 
            COM(4,76),
            COM(4,78),
        ], 
        // AExp14
        vec![ // 28 
            PTR(17, false, false),
            PTR(30, false, false),
        ], 
        vec![ // 29 
            PRM(EQ,false),
            INT(2),
        ], 
        vec![ // 30 
            PTR(26, false, false),
            PTR(29, false, false),
        ], 
        // AExp15
        vec![ // 31 
            COM(2,81),
            PTR(33, false, false),
        ], 
        vec![ // 32 
            PRM(EQ,false),
            INT(0),
        ], 
        vec![ // 33 
            PTR(26, false, false),
            PTR(32, false, false),
        ], 
        // AExp16
        vec![ // 34 
            COM(2,83),
            COM(2,1),
        ], 
        // AExp17
        vec![ // 35 
            COM(3,84),
            PTR(36, false, false),
        ], 
        vec![ // 36 
            COM(3,86),
            COM(1,88),
        ], 
    ],
    comb_img: vec![
        // AExp0
        vec![ // 0 
            ARG(0),
        ], 
        // AExp1
        vec![ // 1 
            ARG(1),
        ], 
        // AExp2
        vec![ // 2 
            PTR(2, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 3 
            ARG(0),
            ARG(1),
        ], 
        // AExp3
        vec![ // 4 
            PTR(4, false, false),
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 5 
            PTR(35, false, false),
            ARG(0),
            COM(2,0),
        ], 
        // AExp4
        vec![ // 6 
            ARG(3),
            ARG(2),
            PTR(0, true, true),
        ], 
        vec![ // 7 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp5
        vec![ // 8 
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 9 
            PRM(ADD,false),
            ARG(1),
            INT(1),
        ], 
        // AExp6
        vec![ // 10 
            ARG(3),
            ARG(0),
            ARG(1),
        ], 
        // AExp7
        vec![ // 11 
            PRM(EQ,false),
            ARG(1),
            INT(0),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 12 
            COM(4,10),
            COM(2,0),
            COM(2,0),
        ], 
        vec![ // 13 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp8
        vec![ // 14 
            PTR(6, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 15 
            PTR(10, false, false),
            ARG(2),
        ], 
        vec![ // 16 
            ARG(0),
            ARG(1),
        ], 
        // AExp9
        vec![ // 17 
            PTR(19, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 18 
            PRM(SUB,false),
            ARG(0),
            INT(1),
        ], 
        // AExp10
        vec![ // 19 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 20 
            ARG(0),
            ARG(1),
        ], 
        // AExp11
        vec![ // 21 
            ARG(3),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 22 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp12
        vec![ // 23 
            PTR(8, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 24 
            ARG(1),
            ARG(3),
        ], 
        vec![ // 25 
            ARG(0),
            ARG(2),
        ], 
        // AExp13
        vec![ // 26 
            Y,
            PTR(0, true, true),
            ARG(1),
        ], 
        vec![ // 27 
            ARG(0),
            ARG(2),
        ], 
        // AExp14
        vec![ // 28 
            ARG(3),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 29 
            ARG(0),
            ARG(2),
        ], 
        // AExp15
        vec![ // 30 
            COM(4,10),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 31 
            ARG(0),
            ARG(2),
        ], 
        // AExp16
        vec![ // 32 
            ARG(1),
            COM(2,0),
            ARG(0),
        ], 
        // AExp17
        vec![ // 33 
            PTR(8, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 34 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        vec![ // 35 
            PTR(12, false, false),
            ARG(1),
            ARG(2),
        ], 
        // AExp18
        vec![ // 36 
            PTR(17, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 37 
            PTR(10, false, false),
            ARG(1),
        ], 
        vec![ // 38 
            COM(4,10),
            ARG(0),
        ], 
        // AExp19
        vec![ // 39 
            ARG(2),
            PTR(0, true, true),
            ARG(1),
        ], 
        vec![ // 40 
            ARG(0),
            ARG(3),
        ], 
        // AExp20
        vec![ // 41 
            COM(4,10),
            PTR(0, true, true),
            COM(2,0),
        ], 
        vec![ // 42 
            ARG(0),
            ARG(1),
        ], 
        // AExp21
        vec![ // 43 
            COM(4,10),
            PTR(0, true, true),
        ], 
        vec![ // 44 
            COM(4,10),
            INT(0),
            ARG(0),
        ], 
        // AExp22
        vec![ // 45 
            COM(2,0),
        ], 
        // AExp23
        vec![ // 46 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 47 
            ARG(0),
            ARG(1),
        ], 
        // AExp24
        vec![ // 48 
            ARG(3),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 49 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp25
        vec![ // 50 
            COM(4,10),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 51 
            ARG(1),
            ARG(3),
        ], 
        vec![ // 52 
            ARG(0),
            ARG(2),
        ], 
        // AExp26
        vec![ // 53 
            PTR(17, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 54 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        vec![ // 55 
            COM(4,10),
            ARG(2),
        ], 
        // AExp27
        vec![ // 56 
            PTR(4, false, false),
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 57 
            PTR(20, false, false),
            ARG(1),
        ], 
        // AExp28
        vec![ // 58 
            PTR(21, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 59 
            PTR(23, false, false),
            ARG(1),
        ], 
        vec![ // 60 
            ARG(0),
            ARG(1),
        ], 
        // AExp29
        vec![ // 61 
            PTR(21, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 62 
            PTR(31, false, false),
            ARG(0),
        ], 
        vec![ // 63 
            PTR(28, false, false),
            ARG(0),
        ], 
        // AExp30
        vec![ // 64 
            ARG(1),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 65 
            ARG(0),
            ARG(2),
        ], 
        // AExp31
        vec![ // 66 
            ARG(1),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 67 
            ARG(0),
            ARG(2),
            ARG(3),
        ], 
        vec![ // 68 
            COM(4,10),
            ARG(2),
            ARG(3),
        ], 
        // AExp32
        vec![ // 69 
            COM(4,10),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 70 
            PTR(21, false, false),
            ARG(1),
            ARG(3),
        ], 
        vec![ // 71 
            PTR(8, false, false),
            ARG(0),
            ARG(2),
        ], 
        // AExp33
        vec![ // 72 
            COM(4,10),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 73 
            PTR(17, false, false),
            ARG(0),
            ARG(1),
        ], 
        // AExp34
        vec![ // 74 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 75 
            ARG(0),
            ARG(1),
        ], 
        // AExp35
        vec![ // 76 
            ARG(3),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 77 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp36
        vec![ // 78 
            ARG(0),
            ARG(2),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 79 
            COM(4,10),
            ARG(2),
            COM(2,0),
        ], 
        vec![ // 80 
            ARG(1),
            ARG(3),
        ], 
        // AExp37
        vec![ // 81 
            PTR(17, false, false),
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 82 
            PTR(34, false, false),
            ARG(1),
        ], 
        // AExp38
        vec![ // 83 
            ARG(1),
            COM(2,0),
            ARG(0),
        ], 
        // AExp39
        vec![ // 84 
            PRM(LE,false),
            ARG(1),
            INT(0),
            PTR(0, true, true),
            COM(2,0),
        ], 
        vec![ // 85 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp40
        vec![ // 86 
            COM(4,10),
            ARG(2),
            PTR(0, true, true),
        ], 
        vec![ // 87 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp41
        vec![ // 88 
            PTR(35, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 89 
            PRM(SUB,false),
            ARG(0),
            INT(1),
        ], 
    ],

}});