use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*;

// Combinators in this file: 37
#[rustfmt::skip]
pub static SUMEULER: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            PTR(2, false, false),
            PTR(1, false, false),
        ], 
        vec![ // 1 
            PTR(5, false, false),
            INT(1),
            INT(30),
        ], 
        // AExp1
        vec![ // 2 
            PTR(3, false, false),
            PRM(ADD,false),
            INT(0),
        ], 
        // AExp2
        vec![ // 3 
            COM(3,2),
            PTR(4, false, false),
        ], 
        vec![ // 4 
            COM(5,4),
            COM(4,6),
        ], 
        // AExp3
        vec![ // 5 
            COM(3,8),
            PTR(6, false, false),
        ], 
        vec![ // 6 
            COM(3,10),
            COM(1,12),
        ], 
        // AExp4
        vec![ // 7 
            COM(2,14),
            PTR(8, false, false),
        ], 
        vec![ // 8 
            COM(4,16),
            COM(4,18),
        ], 
        // AExp5
        vec![ // 9 
            COM(2,22),
            PTR(10, false, false),
        ], 
        vec![ // 10 
            COM(2,24),
            COM(1,27),
        ], 
        // AExp6
        vec![ // 11 
            Y,
            PTR(12, false, false),
            INT(0),
        ], 
        vec![ // 12 
            COM(4,29),
            COM(3,31),
        ], 
        // AExp7
        vec![ // 13 
            COM(2,33),
            PTR(15, false, false),
        ], 
        vec![ // 14 
            COM(5,37),
            COM(3,40),
        ], 
        vec![ // 15 
            COM(4,35),
            PTR(14, false, false),
        ], 
        // AExp8
        vec![ // 16 
            COM(3,70),
            PTR(17, false, false),
        ], 
        vec![ // 17 
            COM(3,72),
            COM(1,74),
        ], 
        // AExp9
        vec![ // 18 
            COM(3,44),
            COM(2,46),
        ], 
        // AExp10
        vec![ // 19 
            COM(3,48),
            COM(2,1),
        ], 
        // AExp11
        vec![ // 20 
            COM(2,49),
            PTR(23, false, false),
        ], 
        vec![ // 21 
            COM(5,62),
            COM(3,65),
            COM(4,66),
        ], 
        vec![ // 22 
            COM(4,6),
            PTR(21, false, false),
            COM(1,69),
        ], 
        vec![ // 23 
            COM(7,51),
            COM(7,56),
            COM(2,59),
            COM(3,60),
            PTR(22, false, false),
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
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 3 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp3
        vec![ // 4 
            ARG(4),
            ARG(2),
            PTR(0, true, true),
        ], 
        vec![ // 5 
            ARG(0),
            ARG(1),
            ARG(3),
        ], 
        // AExp4
        vec![ // 6 
            ARG(0),
            ARG(2),
            PTR(0, true, true),
        ], 
        vec![ // 7 
            ARG(1),
            ARG(3),
        ], 
        // AExp5
        vec![ // 8 
            PTR(7, false, false),
            PTR(9, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 9 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp6
        vec![ // 10 
            COM(4,21),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 11 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp7
        vec![ // 12 
            PTR(16, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 13 
            PRM(ADD,false),
            ARG(0),
            INT(1),
        ], 
        // AExp8
        vec![ // 14 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 15 
            ARG(0),
            ARG(1),
        ], 
        // AExp9
        vec![ // 16 
            ARG(3),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 17 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp10
        vec![ // 18 
            COM(4,21),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 19 
            ARG(1),
            ARG(3),
        ], 
        vec![ // 20 
            ARG(0),
            ARG(2),
        ], 
        // AExp11
        vec![ // 21 
            ARG(3),
            ARG(0),
            ARG(1),
        ], 
        // AExp12
        vec![ // 22 
            PTR(11, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 23 
            ARG(0),
            ARG(1),
        ], 
        // AExp13
        vec![ // 24 
            PTR(13, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 25 
            ARG(0),
            ARG(1),
        ], 
        vec![ // 26 
            COM(2,42),
            ARG(1),
        ], 
        // AExp14
        vec![ // 27 
            PTR(16, false, false),
            INT(1),
            PTR(0, true, true),
        ], 
        vec![ // 28 
            PRM(SUB,false),
            ARG(0),
            INT(1),
        ], 
        // AExp15
        vec![ // 29 
            ARG(3),
            ARG(2),
            PTR(0, true, true),
        ], 
        vec![ // 30 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp16
        vec![ // 31 
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 32 
            PRM(ADD,false),
            ARG(1),
            INT(1),
        ], 
        // AExp17
        vec![ // 33 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 34 
            ARG(0),
            ARG(1),
        ], 
        // AExp18
        vec![ // 35 
            ARG(3),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 36 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp19
        vec![ // 37 
            ARG(1),
            ARG(3),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 38 
            ARG(0),
            ARG(2),
            ARG(3),
            ARG(4),
        ], 
        vec![ // 39 
            ARG(2),
            ARG(4),
        ], 
        // AExp20
        vec![ // 40 
            COM(4,21),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 41 
            ARG(0),
            ARG(2),
        ], 
        // AExp21
        vec![ // 42 
            PRM(EQ,false),
            PTR(0, true, true),
            INT(1),
        ], 
        vec![ // 43 
            PTR(18, false, false),
            ARG(0),
            ARG(1),
        ], 
        // AExp22
        vec![ // 44 
            PRM(EQ,false),
            INT(0),
            ARG(2),
            PTR(0, true, true),
            ARG(1),
        ], 
        vec![ // 45 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp23
        vec![ // 46 
            PTR(18, false, false),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 47 
            PTR(19, false, false),
            ARG(0),
            ARG(1),
        ], 
        // AExp24
        vec![ // 48 
            PTR(20, false, false),
            ARG(1),
            ARG(2),
            ARG(0),
        ], 
        // AExp25
        vec![ // 49 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 50 
            ARG(0),
            ARG(1),
        ], 
        // AExp26
        vec![ // 51 
            ARG(0),
            PTR(3, true, true),
            ARG(4),
            PTR(2, true, true),
            ARG(5),
            PTR(1, true, true),
            ARG(6),
            PTR(0, true, true),
        ], 
        vec![ // 52 
            PRM(ADD,false),
            ARG(6),
            ARG(6),
        ], 
        vec![ // 53 
            ARG(3),
            ARG(6),
        ], 
        vec![ // 54 
            ARG(2),
            ARG(4),
            ARG(6),
        ], 
        vec![ // 55 
            ARG(1),
            ARG(4),
        ], 
        // AExp27
        vec![ // 56 
            PRM(LE,false),
            ARG(6),
            ARG(1),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 57 
            ARG(3),
            ARG(6),
            ARG(4),
        ], 
        vec![ // 58 
            PRM(LE,false),
            ARG(5),
            ARG(1),
            ARG(0),
            ARG(2),
        ], 
        // AExp28
        vec![ // 59 
            ARG(1),
            INT(0),
            ARG(0),
        ], 
        // AExp29
        vec![ // 60 
            ARG(2),
            INT(1),
            PTR(0, true, true),
        ], 
        vec![ // 61 
            PRM(SUB,false),
            ARG(0),
            ARG(1),
        ], 
        // AExp30
        vec![ // 62 
            PRM(LE,false),
            ARG(2),
            ARG(4),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 63 
            ARG(1),
            ARG(2),
            ARG(4),
            ARG(3),
        ], 
        vec![ // 64 
            ARG(0),
            ARG(4),
            ARG(3),
        ], 
        // AExp31
        vec![ // 65 
            ARG(2),
            ARG(1),
            ARG(0),
        ], 
        // AExp32
        vec![ // 66 
            ARG(3),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 67 
            PRM(SUB,false),
            ARG(1),
            ARG(0),
        ], 
        vec![ // 68 
            PRM(ADD,false),
            ARG(2),
            INT(1),
        ], 
        // AExp33
        vec![ // 69 
            PRM(ADD,false),
            ARG(0),
            ARG(0),
        ], 
        // AExp34
        vec![ // 70 
            PRM(LE,false),
            ARG(1),
            ARG(2),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 71 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp35
        vec![ // 72 
            COM(4,21),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 73 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp36
        vec![ // 74 
            PTR(16, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 75 
            PRM(ADD,false),
            ARG(0),
            INT(1),
        ], 
    ],

}});
