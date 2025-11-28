use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 39
#[rustfmt::skip]
pub static BRAUN: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            COM(1,2),
            PTR(3, false, false),
        ], 
        vec![ // 1 
            COM(2,72),
            INT(0),
            INT(255),
        ], 
        vec![ // 2 
            COM(2,66),
            INT(2),
            PTR(1, false, false),
        ], 
        vec![ // 3 
            COM(1,8),
            COM(1,13),
            PTR(2, false, false),
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
            INT(0),
            INT(1),
        ], 
        // AExp3
        vec![ // 3 
            COM(1,10),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 4 
            ARG(1, true),
            ARG(3, true),
        ], 
        vec![ // 5 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp4
        vec![ // 6 
            ARG(2, true),
            COM(2,1),
            PTR(0, true, true),
        ], 
        vec![ // 7 
            COM(4,3),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp5
        vec![ // 8 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 9 
            COM(3,6),
            ARG(0, true),
        ], 
        // AExp6
        vec![ // 10 
            ARG(0, true),
            COM(2,0),
        ], 
        // AExp7
        vec![ // 11 
            COM(1,28),
            PTR(0, true, true),
        ], 
        vec![ // 12 
            COM(1,51),
            ARG(0, true),
        ], 
        // AExp8
        vec![ // 13 
            COM(2,20),
            ARG(0, false),
            PTR(0, true, true),
        ], 
        vec![ // 14 
            COM(1,11),
            ARG(0, false),
        ], 
        // AExp9
        vec![ // 15 
            COM(2,0),
        ], 
        // AExp10
        vec![ // 16 
            PRM(EQ,false),
            ARG(0, true),
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 17 
            COM(2,20),
            ARG(1, true),
            ARG(3, true),
        ], 
        // AExp11
        vec![ // 18 
            ARG(0, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 19 
            COM(4,16),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp12
        vec![ // 20 
            ARG(0, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 21 
            COM(3,18),
            ARG(1, false),
        ], 
        vec![ // 22 
            ARG(1, false),
            COM(2,1),
            COM(2,15),
        ], 
        // AExp13
        vec![ // 23 
            COM(2,37),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 24 
            COM(1,28),
            ARG(1, true),
        ], 
        vec![ // 25 
            COM(1,28),
            ARG(0, true),
        ], 
        // AExp14
        vec![ // 26 
            COM(4,29),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 27 
            COM(2,23),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp15
        vec![ // 28 
            ARG(0, true),
            COM(3,26),
            COM(2,0),
        ], 
        // AExp16
        vec![ // 29 
            ARG(3, true),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp17
        vec![ // 30 
            COM(4,29),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 31 
            COM(2,37),
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp18
        vec![ // 32 
            COM(4,29),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 33 
            COM(3,30),
            ARG(1, true),
            ARG(2, true),
            ARG(3, true),
        ], 
        // AExp19
        vec![ // 34 
            ARG(0, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 35 
            COM(4,32),
            ARG(1, false),
            ARG(2, false),
        ], 
        vec![ // 36 
            COM(4,29),
            ARG(1, false),
            ARG(2, false),
        ], 
        // AExp20
        vec![ // 37 
            ARG(0, true),
            ARG(1, false),
            PTR(0, true, true),
        ], 
        vec![ // 38 
            COM(3,34),
            ARG(1, false),
        ], 
        // AExp21
        vec![ // 39 
            SEQ(false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 40 
            COM(5,61),
            ARG(0, true),
            ARG(2, false),
            ARG(1, false),
        ], 
        vec![ // 41 
            SEQ(false),
            ARG(2, false),
            ARG(1, false),
        ], 
        // AExp22
        vec![ // 42 
            COM(1,51),
            PTR(0, true, true),
        ], 
        vec![ // 43 
            ARG(0, true),
            COM(2,1),
        ], 
        // AExp23
        vec![ // 44 
            COM(1,51),
            PTR(0, true, true),
        ], 
        vec![ // 45 
            ARG(0, true),
            COM(2,0),
        ], 
        // AExp24
        vec![ // 46 
            COM(3,39),
            ARG(0, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 47 
            COM(1,44),
            ARG(1, false),
        ], 
        vec![ // 48 
            COM(1,42),
            ARG(1, false),
        ], 
        // AExp25
        vec![ // 49 
            COM(2,46),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 50 
            COM(1,60),
            ARG(1, true),
        ], 
        // AExp26
        vec![ // 51 
            ARG(0, true),
            COM(2,1),
            COM(2,49),
        ], 
        // AExp27
        vec![ // 52 
            ARG(0, true),
            COM(2,0),
            COM(2,0),
        ], 
        // AExp28
        vec![ // 53 
            COM(4,29),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 54 
            ARG(1, true),
            COM(2,1),
        ], 
        // AExp29
        vec![ // 55 
            ARG(2, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 56 
            ARG(1, false),
            COM(2,0),
        ], 
        vec![ // 57 
            COM(2,53),
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp30
        vec![ // 58 
            COM(3,55),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 59 
            COM(1,60),
            ARG(1, true),
        ], 
        // AExp31
        vec![ // 60 
            ARG(0, true),
            COM(1,52),
            COM(2,58),
        ], 
        // AExp32
        vec![ // 61 
            ARG(3, true),
            ARG(0, true),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp33
        vec![ // 62 
            COM(2,66),
            PTR(0, true, true),
        ], 
        vec![ // 63 
            PRM(SUB,false),
            ARG(0, true),
            INT(1),
        ], 
        // AExp34
        vec![ // 64 
            COM(4,29),
            ARG(1, false),
            PTR(0, true, true),
        ], 
        vec![ // 65 
            COM(1,62),
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp35
        vec![ // 66 
            PRM(LE,false),
            ARG(0, false),
            INT(0),
            PTR(0, true, true),
            COM(2,0),
        ], 
        vec![ // 67 
            COM(2,64),
            ARG(0, false),
            ARG(1, true),
        ], 
        // AExp36
        vec![ // 68 
            COM(2,72),
            PTR(0, true, true),
        ], 
        vec![ // 69 
            PRM(ADD,false),
            ARG(0, true),
            INT(1),
        ], 
        // AExp37
        vec![ // 70 
            COM(4,29),
            ARG(0, false),
            PTR(0, true, true),
        ], 
        vec![ // 71 
            COM(1,68),
            ARG(0, false),
            ARG(1, true),
        ], 
        // AExp38
        vec![ // 72 
            PRM(LE,false),
            ARG(0, false),
            ARG(1, false),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 73 
            COM(2,70),
            ARG(0, false),
            ARG(1, false),
        ], 
    ],

}});