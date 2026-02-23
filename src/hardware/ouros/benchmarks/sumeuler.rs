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
            PTR(2, false, false),
            PTR(1, false, false),
        ], 
        vec![ // 1 
            COM(2,12),
            INT(1),
            INT(30),
        ], 
        // AExp1
        vec![ // 2 
            COM(2,6),
            PRM(ADD,false),
            INT(0),
        ], 
        // AExp2
        vec![ // 3 
            Y,
            COM(3,31),
            INT(0),
        ], 
    ],
    comb_img: vec![
        // AExp0
        vec![ // 0 
            ARG(0, true),
        ], 
        // AExp1
        vec![ // 1 
            ARG(1, true),
        ], 
        // AExp2
        vec![ // 2 
            ARG(0, true),
            ARG(2, true),
            PTR(0, true, true),
        ], 
        vec![ // 3 
            ARG(1, true),
            ARG(3, true),
        ], 
        // AExp3
        vec![ // 4 
            ARG(3, true),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 5 
            COM(4,2),
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp4
        vec![ // 6 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 7 
            COM(4,4),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp5
        vec![ // 8 
            COM(2,78),
            PTR(0, true, true),
        ], 
        vec![ // 9 
            PRM(ADD,false),
            ARG(0, true),
            INT(1),
        ], 
        // AExp6
        vec![ // 10 
            COM(4,21),
            ARG(0, false),
            PTR(0, true, true),
        ], 
        vec![ // 11 
            COM(1,8),
            ARG(0, false),
            ARG(1, true),
        ], 
        // AExp7
        vec![ // 12 
            COM(1,19),
            COM(1,27),
            PTR(0, true, true),
        ], 
        vec![ // 13 
            COM(2,10),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp8
        vec![ // 14 
            COM(4,21),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 15 
            ARG(1, true),
            ARG(3, true),
        ], 
        vec![ // 16 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp9
        vec![ // 17 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 18 
            COM(4,14),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp10
        vec![ // 19 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 20 
            COM(3,17),
            ARG(0, true),
        ], 
        // AExp11
        vec![ // 21 
            ARG(3, true),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp12
        vec![ // 22 
            COM(2,78),
            INT(1),
            PTR(0, true, true),
        ], 
        vec![ // 23 
            PRM(SUB,false),
            ARG(0, true),
            INT(1),
        ], 
        // AExp13
        vec![ // 24 
            COM(1,40),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 25 
            COM(1,22),
            ARG(0, false),
        ], 
        vec![ // 26 
            COM(2,42),
            ARG(0, false),
        ], 
        // AExp14
        vec![ // 27 
            PTR(3, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 28 
            COM(1,24),
            ARG(0, true),
        ], 
        // AExp15
        vec![ // 29 
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 30 
            PRM(ADD,false),
            ARG(1, true),
            INT(1),
        ], 
        // AExp16
        vec![ // 31 
            ARG(2, true),
            ARG(1, false),
            PTR(0, true, true),
        ], 
        vec![ // 32 
            COM(3,29),
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp17
        vec![ // 33 
            COM(4,21),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 34 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp18
        vec![ // 35 
            ARG(0, true),
            ARG(2, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 36 
            COM(3,33),
            ARG(1, false),
            ARG(2, false),
            ARG(3, false),
        ], 
        vec![ // 37 
            ARG(1, false),
            ARG(3, false),
        ], 
        // AExp19
        vec![ // 38 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 39 
            COM(4,35),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp20
        vec![ // 40 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 41 
            COM(3,38),
            ARG(0, true),
        ], 
        // AExp21
        vec![ // 42 
            PRM(EQ,false),
            PTR(0, true, true),
            INT(1),
        ], 
        vec![ // 43 
            COM(2,46),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp22
        vec![ // 44 
            COM(2,46),
            ARG(1, false),
            PTR(0, true, true),
        ], 
        vec![ // 45 
            COM(2,48),
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp23
        vec![ // 46 
            PRM(EQ,false),
            INT(0),
            ARG(1, false),
            PTR(0, true, true),
            ARG(0, false),
        ], 
        vec![ // 47 
            COM(2,44),
            ARG(0, false),
            ARG(1, false),
        ], 
        // AExp24
        vec![ // 48 
            COM(1,72),
            ARG(0, true),
            ARG(1, true),
            COM(2,1),
        ], 
        // AExp25
        vec![ // 49 
            PRM(LE,false),
            ARG(5, false),
            ARG(1, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 50 
            ARG(3, true),
            ARG(5, false),
        ], 
        vec![ // 51 
            PRM(LE,false),
            ARG(4, true),
            ARG(1, false),
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp26
        vec![ // 52 
            ARG(1, true),
            INT(0),
            ARG(0, true),
        ], 
        // AExp27
        vec![ // 53 
            ARG(2, true),
            INT(1),
            PTR(0, true, true),
        ], 
        vec![ // 54 
            PRM(SUB,false),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp28
        vec![ // 55 
            ARG(2, true),
            ARG(1, true),
            ARG(0, true),
        ], 
        // AExp29
        vec![ // 56 
            ARG(3, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 57 
            PRM(SUB,false),
            ARG(1, true),
            ARG(0, true),
        ], 
        vec![ // 58 
            PRM(ADD,false),
            ARG(2, true),
            INT(1),
        ], 
        // AExp30
        vec![ // 59 
            PRM(LE,false),
            ARG(0, false),
            ARG(2, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 60 
            COM(4,56),
            ARG(0, false),
            ARG(2, false),
            ARG(1, false),
        ], 
        vec![ // 61 
            COM(3,55),
            ARG(2, false),
            ARG(1, false),
        ], 
        // AExp31
        vec![ // 62 
            PRM(ADD,false),
            ARG(0, false),
            ARG(0, false),
        ], 
        // AExp32
        vec![ // 63 
            COM(3,59),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 64 
            COM(1,62),
            ARG(1, true),
        ], 
        // AExp33
        vec![ // 65 
            ARG(0, true),
            ARG(2, true),
            PTR(0, true, true),
        ], 
        vec![ // 66 
            COM(2,63),
            ARG(1, true),
        ], 
        // AExp34
        vec![ // 67 
            COM(6,49),
            PTR(3, true, true),
            ARG(0, false),
            PTR(2, true, true),
            PTR(1, true, true),
            ARG(2, false),
            PTR(0, true, true),
        ], 
        vec![ // 68 
            PRM(ADD,false),
            ARG(2, false),
            ARG(2, false),
        ], 
        vec![ // 69 
            COM(3,65),
            ARG(1, true),
            ARG(2, false),
        ], 
        vec![ // 70 
            COM(3,53),
            ARG(0, false),
            ARG(2, false),
        ], 
        vec![ // 71 
            COM(2,52),
            ARG(0, false),
        ], 
        // AExp35
        vec![ // 72 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 73 
            COM(3,67),
            ARG(0, true),
        ], 
        // AExp36
        vec![ // 74 
            COM(2,78),
            PTR(0, true, true),
        ], 
        vec![ // 75 
            PRM(ADD,false),
            ARG(0, true),
            INT(1),
        ], 
        // AExp37
        vec![ // 76 
            COM(4,21),
            ARG(0, false),
            PTR(0, true, true),
        ], 
        vec![ // 77 
            COM(1,74),
            ARG(0, false),
            ARG(1, true),
        ], 
        // AExp38
        vec![ // 78 
            PRM(LE,false),
            ARG(0, false),
            ARG(1, false),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 79 
            COM(2,76),
            ARG(0, false),
            ARG(1, false),
        ], 
    ],

}});
