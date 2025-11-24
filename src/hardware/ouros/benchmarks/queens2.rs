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
            COM(1,4),
            INT(5),
        ], 
        // AExp1
        vec![ // 1 
            COM(3,40),
            PTR(5, false, false),
        ], 
        vec![ // 2 
            COM(4,15),
            INT(2),
            COM(2,0),
        ], 
        vec![ // 3 
            COM(4,15),
            INT(1),
            PTR(2, false, false),
        ], 
        vec![ // 4 
            COM(1,44),
            PTR(3, false, false),
        ], 
        vec![ // 5 
            COM(2,42),
            PTR(4, false, false),
        ], 
        // AExp2
        vec![ // 6 
            COM(1,51),
            PTR(8, false, false),
        ], 
        vec![ // 7 
            PRM(EQ,false),
            INT(2),
        ], 
        vec![ // 8 
            COM(1,79),
            PTR(7, false, false),
        ], 
        // AExp3
        vec![ // 9 
            COM(2,81),
            PTR(11, false, false),
        ], 
        vec![ // 10 
            PRM(EQ,false),
            INT(0),
        ], 
        vec![ // 11 
            COM(1,79),
            PTR(10, false, false),
        ], 
        // AExp4
        vec![ // 12 
            COM(2,72),
            PTR(14, false, false),
        ], 
        vec![ // 13 
            PRM(EQ,false),
            INT(1),
        ], 
        vec![ // 14 
            COM(1,79),
            PTR(13, false, false),
        ], 
        // AExp5
        vec![ // 15 
            Y,
            COM(3,8),
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
            COM(2,16),
            ARG(0, false),
            PTR(0, true, true),
        ], 
        vec![ // 3 
            COM(2,88),
            ARG(0, false),
            COM(2,0),
        ], 
        // AExp3
        vec![ // 4 
            PTR(15, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 5 
            COM(1,2),
            ARG(0, true),
        ], 
        // AExp4
        vec![ // 6 
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 7 
            PRM(ADD,false),
            ARG(1, true),
            INT(1),
        ], 
        // AExp5
        vec![ // 8 
            ARG(2, true),
            ARG(1, false),
            PTR(0, true, true),
        ], 
        vec![ // 9 
            COM(3,6),
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp6
        vec![ // 10 
            COM(2,55),
            PTR(0, true, true),
        ], 
        vec![ // 11 
            PRM(SUB,false),
            ARG(0, true),
            INT(1),
        ], 
        // AExp7
        vec![ // 12 
            COM(1,24),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 13 
            COM(1,38),
            ARG(1, true),
        ], 
        vec![ // 14 
            COM(1,10),
            ARG(0, true),
        ], 
        // AExp8
        vec![ // 15 
            ARG(3, true),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp9
        vec![ // 16 
            PRM(EQ,false),
            ARG(0, false),
            INT(0),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 17 
            COM(4,15),
            COM(2,0),
            COM(2,0),
        ], 
        vec![ // 18 
            COM(2,12),
            ARG(0, false),
            ARG(1, true),
        ], 
        // AExp10
        vec![ // 19 
            COM(2,30),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 20 
            ARG(1, true),
            ARG(3, true),
        ], 
        vec![ // 21 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp11
        vec![ // 22 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 23 
            COM(4,19),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp12
        vec![ // 24 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 25 
            COM(3,22),
            ARG(0, true),
        ], 
        // AExp13
        vec![ // 26 
            COM(4,15),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 27 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp14
        vec![ // 28 
            ARG(2, true),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 29 
            COM(3,26),
            ARG(1, true),
        ], 
        // AExp15
        vec![ // 30 
            Y,
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 31 
            COM(3,28),
            ARG(1, true),
        ], 
        // AExp16
        vec![ // 32 
            COM(1,51),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 33 
            COM(1,38),
            ARG(1, true),
        ], 
        vec![ // 34 
            COM(4,15),
            ARG(0, true),
        ], 
        // AExp17
        vec![ // 35 
            COM(2,30),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 36 
            COM(2,32),
            ARG(0, false),
            ARG(1, false),
        ], 
        vec![ // 37 
            PTR(1, false, false),
            ARG(0, false),
            ARG(1, false),
        ], 
        // AExp18
        vec![ // 38 
            ARG(0, true),
            COM(2,0),
            COM(2,35),
        ], 
        // AExp19
        vec![ // 39 
            COM(2,0),
        ], 
        // AExp20
        vec![ // 40 
            ARG(1, true),
            PTR(0, true, true),
            COM(2,39),
        ], 
        vec![ // 41 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp21
        vec![ // 42 
            COM(4,15),
            PTR(0, true, true),
            COM(2,0),
        ], 
        vec![ // 43 
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp22
        vec![ // 44 
            COM(4,15),
            PTR(0, true, true),
        ], 
        vec![ // 45 
            COM(4,15),
            INT(0),
            ARG(0, true),
        ], 
        // AExp23
        vec![ // 46 
            COM(4,15),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 47 
            ARG(1, true),
            ARG(3, true),
        ], 
        vec![ // 48 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp24
        vec![ // 49 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 50 
            COM(4,46),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp25
        vec![ // 51 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 52 
            COM(3,49),
            ARG(0, true),
        ], 
        // AExp26
        vec![ // 53 
            COM(2,16),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 54 
            COM(1,61),
            ARG(1, true),
        ], 
        // AExp27
        vec![ // 55 
            COM(1,51),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 56 
            COM(2,53),
            ARG(0, true),
            ARG(1, false),
        ], 
        vec![ // 57 
            COM(4,15),
            ARG(1, false),
        ], 
        // AExp28
        vec![ // 58 
            COM(2,70),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 59 
            PTR(9, false, false),
            ARG(0, false),
        ], 
        vec![ // 60 
            PTR(6, false, false),
            ARG(0, false),
        ], 
        // AExp29
        vec![ // 61 
            COM(2,70),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 62 
            PTR(12, false, false),
            ARG(0, false),
        ], 
        vec![ // 63 
            COM(1,58),
            ARG(0, false),
        ], 
        // AExp30
        vec![ // 64 
            COM(4,15),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 65 
            COM(2,70),
            ARG(1, true),
            ARG(3, true),
        ], 
        vec![ // 66 
            COM(2,30),
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp31
        vec![ // 67 
            ARG(0, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 68 
            COM(4,64),
            ARG(1, false),
            ARG(2, false),
        ], 
        vec![ // 69 
            COM(4,15),
            ARG(1, false),
            ARG(2, false),
        ], 
        // AExp32
        vec![ // 70 
            ARG(0, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 71 
            COM(3,67),
            ARG(1, true),
        ], 
        // AExp33
        vec![ // 72 
            COM(4,15),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 73 
            COM(1,51),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp34
        vec![ // 74 
            ARG(0, true),
            ARG(2, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 75 
            COM(4,15),
            ARG(2, false),
            COM(2,0),
        ], 
        vec![ // 76 
            ARG(1, true),
            ARG(3, true),
        ], 
        // AExp35
        vec![ // 77 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 78 
            COM(4,74),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp36
        vec![ // 79 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 80 
            COM(3,77),
            ARG(0, true),
        ], 
        // AExp37
        vec![ // 81 
            COM(1,51),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 82 
            COM(1,83),
            ARG(1, true),
        ], 
        // AExp38
        vec![ // 83 
            ARG(0, true),
            COM(2,0),
            COM(2,1),
        ], 
        // AExp39
        vec![ // 84 
            COM(2,88),
            PTR(0, true, true),
        ], 
        vec![ // 85 
            PRM(SUB,false),
            ARG(0, true),
            INT(1),
        ], 
        // AExp40
        vec![ // 86 
            COM(4,15),
            ARG(1, false),
            PTR(0, true, true),
        ], 
        vec![ // 87 
            COM(1,84),
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp41
        vec![ // 88 
            PRM(LE,false),
            ARG(0, false),
            INT(0),
            PTR(0, true, true),
            COM(2,0),
        ], 
        vec![ // 89 
            COM(2,86),
            ARG(0, false),
            ARG(1, true),
        ], 
    ],

}});