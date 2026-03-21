use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 31
#[rustfmt::skip]
pub static MSS: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            COM(1,4),
            PTR(2, false, false),
        ], 
        vec![ // 1 
            PRM(SUB,false),
            INT(0),
            INT(20),
        ], 
        vec![ // 2 
            COM(2,58),
            PTR(1, false, false),
            INT(20),
        ], 
        // AExp1
        vec![ // 3 
            COM(2,13),
            PRM(ADD,false),
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
            COM(1,20),
            PTR(3, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 3 
            COM(1,23),
            ARG(0, true),
        ], 
        // AExp3
        vec![ // 4 
            COM(1,7),
            PTR(0, true, true),
        ], 
        vec![ // 5 
            COM(1,2),
            ARG(0, true),
        ], 
        // AExp4
        vec![ // 6 
            PRM(LE,true),
            ARG(0, false),
            ARG(1, false),
            ARG(1, false),
            ARG(0, false),
        ], 
        // AExp5
        vec![ // 7 
            ARG(0, true),
            INT(0),
            PTR(0, true, true),
        ], 
        vec![ // 8 
            COM(2,13),
            COM(2,6),
        ], 
        // AExp6
        vec![ // 9 
            ARG(0, true),
            ARG(2, true),
            PTR(0, true, true),
        ], 
        vec![ // 10 
            ARG(1, true),
            ARG(3, true),
        ], 
        // AExp7
        vec![ // 11 
            ARG(3, true),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 12 
            COM(4,9),
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp8
        vec![ // 13 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 14 
            COM(4,11),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp9
        vec![ // 15 
            COM(4,22),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 16 
            ARG(1, true),
            ARG(3, true),
        ], 
        vec![ // 17 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp10
        vec![ // 18 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 19 
            COM(4,15),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp11
        vec![ // 20 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 21 
            COM(3,18),
            ARG(0, true),
        ], 
        // AExp12
        vec![ // 22 
            ARG(3, true),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp13
        vec![ // 23 
            COM(1,30),
            COM(1,41),
            PTR(0, true, true),
        ], 
        vec![ // 24 
            COM(1,46),
            ARG(0, true),
        ], 
        // AExp14
        vec![ // 25 
            COM(2,36),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 26 
            ARG(1, true),
            ARG(3, true),
        ], 
        vec![ // 27 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp15
        vec![ // 28 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 29 
            COM(4,25),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp16
        vec![ // 30 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 31 
            COM(3,28),
            ARG(0, true),
        ], 
        // AExp17
        vec![ // 32 
            COM(4,22),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 33 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp18
        vec![ // 34 
            ARG(2, true),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 35 
            COM(3,32),
            ARG(1, true),
        ], 
        // AExp19
        vec![ // 36 
            Y,
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 37 
            COM(3,34),
            ARG(1, true),
        ], 
        // AExp20
        vec![ // 38 
            COM(4,22),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 39 
            COM(1,41),
            ARG(1, false),
        ], 
        vec![ // 40 
            COM(4,22),
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp21
        vec![ // 41 
            ARG(0, true),
            COM(2,0),
            COM(2,38),
        ], 
        // AExp22
        vec![ // 42 
            COM(1,46),
            PTR(0, true, true),
        ], 
        vec![ // 43 
            COM(1,53),
            ARG(0, true),
        ], 
        // AExp23
        vec![ // 44 
            COM(4,22),
            ARG(0, false),
            PTR(0, true, true),
        ], 
        vec![ // 45 
            COM(1,42),
            ARG(0, false),
        ], 
        // AExp24
        vec![ // 46 
            ARG(0, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 47 
            COM(3,44),
            ARG(0, false),
        ], 
        vec![ // 48 
            COM(4,22),
            COM(2,0),
            COM(2,0),
        ], 
        // AExp25
        vec![ // 49 
            COM(4,22),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 50 
            COM(1,53),
            ARG(1, true),
        ], 
        // AExp26
        vec![ // 51 
            ARG(1, false),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 52 
            COM(4,49),
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp27
        vec![ // 53 
            ARG(0, true),
            ERR(1),
            COM(2,51),
        ], 
        // AExp28
        vec![ // 54 
            COM(2,58),
            PTR(0, true, true),
        ], 
        vec![ // 55 
            PRM(ADD,false),
            ARG(0, true),
            INT(1),
        ], 
        // AExp29
        vec![ // 56 
            COM(4,22),
            ARG(0, false),
            PTR(0, true, true),
        ], 
        vec![ // 57 
            COM(1,54),
            ARG(0, false),
            ARG(1, true),
        ], 
        // AExp30
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