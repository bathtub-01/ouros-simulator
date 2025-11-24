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
            Y,
            COM(3,6),
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
            PTR(1, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 3 
            COM(1,17),
            ARG(0, false),
            ARG(0, false),
        ], 
        // AExp3
        vec![ // 4 
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 5 
            PRM(ADD,false),
            ARG(1, true),
            INT(1),
        ], 
        // AExp4
        vec![ // 6 
            ARG(2, true),
            ARG(1, false),
            PTR(0, true, true),
        ], 
        vec![ // 7 
            COM(3,4),
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp5
        vec![ // 8 
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 9 
            PRM(SUB,false),
            ARG(1, true),
            INT(1),
        ], 
        // AExp6
        vec![ // 10 
            COM(1,24),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 11 
            COM(2,8),
            ARG(1, true),
            ARG(2, true),
        ], 
        vec![ // 12 
            COM(2,32),
            ARG(0, true),
        ], 
        // AExp7
        vec![ // 13 
            ARG(3, true),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp8
        vec![ // 14 
            PRM(EQ,false),
            ARG(2, false),
            INT(0),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 15 
            COM(4,13),
            COM(2,0),
            COM(2,0),
        ], 
        vec![ // 16 
            COM(3,10),
            ARG(0, true),
            ARG(1, true),
            ARG(2, false),
        ], 
        // AExp9
        vec![ // 17 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 18 
            COM(3,14),
            ARG(0, true),
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
            COM(4,13),
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
            COM(1,24),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 33 
            COM(1,63),
            ARG(0, true),
        ], 
        vec![ // 34 
            COM(2,37),
            ARG(1, true),
        ], 
        // AExp17
        vec![ // 35 
            COM(4,13),
            PTR(0, true, true),
            COM(2,0),
        ], 
        vec![ // 36 
            COM(4,13),
            ARG(1, true),
            ARG(0, true),
        ], 
        // AExp18
        vec![ // 37 
            COM(1,56),
            ARG(1, false),
            INT(1),
            ARG(0, false),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 38 
            COM(2,35),
            ARG(0, false),
            ARG(1, false),
        ], 
        // AExp19
        vec![ // 39 
            PRM(EQ,true),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 40 
            PRM(ADD,false),
            ARG(2, true),
            ARG(1, true),
        ], 
        // AExp20
        vec![ // 41 
            PRM(EQ,true),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 42 
            PRM(SUB,false),
            ARG(2, true),
            ARG(1, true),
        ], 
        // AExp21
        vec![ // 43 
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 44 
            PRM(ADD,false),
            ARG(1, true),
            INT(1),
        ], 
        // AExp22
        vec![ // 45 
            COM(1,58),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 46 
            COM(2,43),
            ARG(1, true),
            ARG(2, false),
            ARG(4, true),
        ], 
        vec![ // 47 
            COM(3,41),
            ARG(0, true),
            ARG(2, false),
            ARG(3, true),
        ], 
        // AExp23
        vec![ // 48 
            COM(1,58),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 49 
            COM(5,45),
            ARG(0, false),
            ARG(1, true),
            ARG(2, false),
            ARG(3, false),
            ARG(4, true),
        ], 
        vec![ // 50 
            COM(3,39),
            ARG(0, false),
            ARG(2, false),
            ARG(3, false),
        ], 
        // AExp24
        vec![ // 51 
            COM(1,58),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 52 
            COM(5,48),
            ARG(0, false),
            ARG(1, true),
            ARG(2, true),
            ARG(3, false),
            ARG(4, true),
        ], 
        vec![ // 53 
            PRM(EQ,true),
            ARG(0, false),
            ARG(3, false),
        ], 
        // AExp25
        vec![ // 54 
            ARG(3, true),
            COM(2,1),
            PTR(0, true, true),
        ], 
        vec![ // 55 
            COM(5,51),
            ARG(0, true),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp26
        vec![ // 56 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 57 
            COM(4,54),
            ARG(0, true),
        ], 
        // AExp27
        vec![ // 58 
            ARG(0, true),
            COM(2,0),
        ], 
        // AExp28
        vec![ // 59 
            COM(1,63),
            PTR(0, true, true),
        ], 
        vec![ // 60 
            PRM(SUB,false),
            ARG(0, true),
            INT(1),
        ], 
        // AExp29
        vec![ // 61 
            COM(4,13),
            ARG(0, false),
            PTR(0, true, true),
        ], 
        vec![ // 62 
            COM(1,59),
            ARG(0, false),
        ], 
        // AExp30
        vec![ // 63 
            PRM(EQ,false),
            ARG(0, false),
            INT(1),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 64 
            COM(4,13),
            INT(1),
            COM(2,0),
        ], 
        vec![ // 65 
            COM(1,61),
            ARG(0, false),
        ], 
    ],

}});