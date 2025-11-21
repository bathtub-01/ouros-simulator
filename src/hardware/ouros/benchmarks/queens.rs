use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 31
#[rustfmt::skip]
pub static QUEENS: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            COM(1,2),
            INT(6),
        ], 
        // AExp1
        vec![ // 1 
            COM(2,19),
            PTR(2, false, false),
        ], 
        vec![ // 2 
            COM(4,21),
            COM(4,23),
        ], 
        // AExp2
        vec![ // 3 
            COM(3,26),
            PTR(4, false, false),
        ], 
        vec![ // 4 
            COM(4,28),
            COM(3,30),
        ], 
        // AExp3
        vec![ // 5 
            COM(3,35),
            COM(2,37),
        ], 
        // AExp4
        vec![ // 6 
            COM(2,39),
            PTR(10, false, false),
        ], 
        vec![ // 7 
            COM(7,51),
            COM(3,54),
            COM(2,56),
        ], 
        vec![ // 8 
            COM(7,46),
            COM(3,49),
            PTR(7, false, false),
        ], 
        vec![ // 9 
            COM(6,43),
            PTR(8, false, false),
        ], 
        vec![ // 10 
            COM(5,41),
            PTR(9, false, false),
        ], 
        // AExp5
        vec![ // 11 
            COM(2,59),
            PTR(12, false, false),
        ], 
        vec![ // 12 
            COM(2,62),
            COM(1,64),
        ], 
        // AExp6
        vec![ // 13 
            Y,
            PTR(14, false, false),
            INT(0),
        ], 
        vec![ // 14 
            COM(4,4),
            COM(3,6),
        ], 
        // AExp7
        vec![ // 15 
            COM(2,8),
            PTR(17, false, false),
        ], 
        vec![ // 16 
            COM(4,14),
            COM(2,17),
        ], 
        vec![ // 17 
            COM(4,11),
            PTR(16, false, false),
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
            PTR(13, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 3 
            PTR(15, false, false),
            ARG(0),
            ARG(0),
        ], 
        // AExp3
        vec![ // 4 
            ARG(3),
            ARG(2),
            PTR(0, true, true),
        ], 
        vec![ // 5 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp4
        vec![ // 6 
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 7 
            PRM(ADD,false),
            ARG(1),
            INT(1),
        ], 
        // AExp5
        vec![ // 8 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 9 
            ARG(0),
            ARG(1),
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
            ARG(3),
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
            ARG(3),
        ], 
        // AExp8
        vec![ // 14 
            PTR(1, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 15 
            ARG(0),
            ARG(2),
            ARG(3),
        ], 
        vec![ // 16 
            COM(2,32),
            ARG(1),
        ], 
        // AExp9
        vec![ // 17 
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 18 
            PRM(SUB,false),
            ARG(1),
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
            PTR(3, false, false),
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
            PTR(1, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 33 
            PTR(11, false, false),
            ARG(0),
        ], 
        vec![ // 34 
            PTR(5, false, false),
            ARG(1),
        ], 
        // AExp17
        vec![ // 35 
            PTR(6, false, false),
            ARG(2),
            INT(1),
            ARG(1),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 36 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp18
        vec![ // 37 
            COM(4,10),
            PTR(0, true, true),
            COM(2,0),
        ], 
        vec![ // 38 
            COM(4,10),
            ARG(1),
            ARG(0),
        ], 
        // AExp19
        vec![ // 39 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 40 
            ARG(0),
            ARG(1),
        ], 
        // AExp20
        vec![ // 41 
            ARG(4),
            COM(2,1),
            PTR(0, true, true),
        ], 
        vec![ // 42 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(3),
        ], 
        // AExp21
        vec![ // 43 
            COM(1,58),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 44 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(4),
            ARG(5),
        ], 
        vec![ // 45 
            PRM(EQ,true),
            ARG(1),
            ARG(4),
        ], 
        // AExp22
        vec![ // 46 
            COM(1,58),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 47 
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(4),
            ARG(5),
            ARG(6),
        ], 
        vec![ // 48 
            ARG(0),
            ARG(2),
            ARG(4),
            ARG(5),
        ], 
        // AExp23
        vec![ // 49 
            PRM(EQ,true),
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 50 
            PRM(ADD,false),
            ARG(2),
            ARG(1),
        ], 
        // AExp24
        vec![ // 51 
            COM(1,58),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 52 
            ARG(1),
            ARG(3),
            ARG(4),
            ARG(6),
        ], 
        vec![ // 53 
            ARG(0),
            ARG(2),
            ARG(4),
            ARG(5),
        ], 
        // AExp25
        vec![ // 54 
            PRM(EQ,true),
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 55 
            PRM(SUB,false),
            ARG(2),
            ARG(1),
        ], 
        // AExp26
        vec![ // 56 
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 57 
            PRM(ADD,false),
            ARG(1),
            INT(1),
        ], 
        // AExp27
        vec![ // 58 
            ARG(0),
            COM(2,0),
        ], 
        // AExp28
        vec![ // 59 
            PRM(EQ,false),
            ARG(1),
            INT(1),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 60 
            COM(4,10),
            INT(1),
            COM(2,0),
        ], 
        vec![ // 61 
            ARG(0),
            ARG(1),
        ], 
        // AExp29
        vec![ // 62 
            COM(4,10),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 63 
            ARG(0),
            ARG(1),
        ], 
        // AExp30
        vec![ // 64 
            PTR(11, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 65 
            PRM(SUB,false),
            ARG(0),
            INT(1),
        ], 
    ],

}});