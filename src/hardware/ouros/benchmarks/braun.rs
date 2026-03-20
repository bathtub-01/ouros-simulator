use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 32
#[rustfmt::skip]
pub static BRAUN: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            COM(1,2),
            PTR(3, false, false),
        ], 
        vec![ // 1 
            COM(2,58),
            INT(0),
            INT(255),
        ], 
        vec![ // 2 
            COM(2,52),
            INT(6000),
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
            COM(1,41),
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
            COM(2,44),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 40 
            COM(1,41),
            ARG(1, true),
        ], 
        // AExp22
        vec![ // 41 
            ARG(0, true),
            COM(2,1),
            COM(2,39),
        ], 
        // AExp23
        vec![ // 42 
            COM(5,47),
            ARG(0, true),
            PTR(0, true, true),
            ARG(2, true),
        ], 
        vec![ // 43 
            COM(2,44),
            ARG(1, true),
            ARG(3, true),
        ], 
        // AExp24
        vec![ // 44 
            ARG(1, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 45 
            COM(5,47),
            ARG(0, false),
            COM(2,1),
            COM(2,1),
        ], 
        vec![ // 46 
            COM(4,42),
            ARG(0, false),
        ], 
        // AExp25
        vec![ // 47 
            ARG(3, true),
            ARG(0, true),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp26
        vec![ // 48 
            COM(2,52),
            PTR(0, true, true),
        ], 
        vec![ // 49 
            PRM(SUB,false),
            ARG(0, true),
            INT(1),
        ], 
        // AExp27
        vec![ // 50 
            COM(4,29),
            ARG(1, false),
            PTR(0, true, true),
        ], 
        vec![ // 51 
            COM(1,48),
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp28
        vec![ // 52 
            PRM(LE,false),
            ARG(0, false),
            INT(0),
            PTR(0, true, true),
            COM(2,0),
        ], 
        vec![ // 53 
            COM(2,50),
            ARG(0, false),
            ARG(1, true),
        ], 
        // AExp29
        vec![ // 54 
            COM(2,58),
            PTR(0, true, true),
        ], 
        vec![ // 55 
            PRM(ADD,false),
            ARG(0, true),
            INT(1),
        ], 
        // AExp30
        vec![ // 56 
            COM(4,29),
            ARG(0, false),
            PTR(0, true, true),
        ], 
        vec![ // 57 
            COM(1,54),
            ARG(0, false),
            ARG(1, true),
        ], 
        // AExp31
        vec![ // 58 
            PRM(LE,false),
            ARG(0, false),
            ARG(1, false),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 59 
            COM(2,56),
            ARG(0, false),
            ARG(1, false),
        ], 
    ],

}});