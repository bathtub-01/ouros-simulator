use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 32
#[rustfmt::skip]
pub static PERMSORT: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            COM(1,3),
            PTR(8, false, false),
        ], 
        vec![ // 1 
            COM(4,2),
            INT(12),
            COM(2,0),
        ], 
        vec![ // 2 
            COM(4,2),
            INT(12),
            PTR(1, false, false),
        ], 
        vec![ // 3 
            COM(4,2),
            INT(6),
            PTR(2, false, false),
        ], 
        vec![ // 4 
            COM(4,2),
            INT(9),
            PTR(3, false, false),
        ], 
        vec![ // 5 
            COM(4,2),
            INT(7),
            PTR(4, false, false),
        ], 
        vec![ // 6 
            COM(4,2),
            INT(6),
            PTR(5, false, false),
        ], 
        vec![ // 7 
            COM(4,2),
            INT(10),
            PTR(6, false, false),
        ], 
        vec![ // 8 
            COM(1,6),
            PTR(7, false, false),
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
            ARG(3, true),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp3
        vec![ // 3 
            ARG(0, true),
            ERR(3),
            COM(2,0),
        ], 
        // AExp4
        vec![ // 4 
            COM(1,15),
            COM(1,24),
            PTR(0, true, true),
        ], 
        vec![ // 5 
            COM(1,29),
            ARG(0, true),
        ], 
        // AExp5
        vec![ // 6 
            COM(1,3),
            PTR(0, true, true),
        ], 
        vec![ // 7 
            COM(1,4),
            ARG(0, true),
        ], 
        // AExp6
        vec![ // 8 
            COM(4,2),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 9 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp7
        vec![ // 10 
            ARG(0, true),
            ARG(2, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 11 
            COM(3,8),
            ARG(1, false),
            ARG(2, false),
            ARG(3, false),
        ], 
        vec![ // 12 
            ARG(1, false),
            ARG(3, false),
        ], 
        // AExp8
        vec![ // 13 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 14 
            COM(4,10),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp9
        vec![ // 15 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 16 
            COM(3,13),
            ARG(0, true),
        ], 
        // AExp10
        vec![ // 17 
            COM(1,24),
            PTR(0, true, true),
        ], 
        vec![ // 18 
            COM(4,2),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp11
        vec![ // 19 
            COM(1,25),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 20 
            COM(2,17),
            ARG(1, false),
            ARG(2, true),
        ], 
        vec![ // 21 
            PRM(LE,false),
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp12
        vec![ // 22 
            ARG(1, true),
            COM(2,1),
            PTR(0, true, true),
        ], 
        vec![ // 23 
            COM(3,19),
            ARG(0, true),
        ], 
        // AExp13
        vec![ // 24 
            ARG(0, true),
            COM(2,1),
            COM(2,22),
        ], 
        // AExp14
        vec![ // 25 
            ARG(0, true),
            COM(2,0),
        ], 
        // AExp15
        vec![ // 26 
            COM(1,36),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 27 
            COM(1,29),
            ARG(1, true),
        ], 
        vec![ // 28 
            COM(1,57),
            ARG(0, true),
        ], 
        // AExp16
        vec![ // 29 
            ARG(0, true),
            PTR(0, true, true),
            COM(2,26),
        ], 
        vec![ // 30 
            COM(4,2),
            COM(2,0),
            COM(2,0),
        ], 
        // AExp17
        vec![ // 31 
            COM(2,42),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 32 
            ARG(1, true),
            ARG(3, true),
        ], 
        vec![ // 33 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp18
        vec![ // 34 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 35 
            COM(4,31),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp19
        vec![ // 36 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 37 
            COM(3,34),
            ARG(0, true),
        ], 
        // AExp20
        vec![ // 38 
            COM(4,2),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 39 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp21
        vec![ // 40 
            ARG(2, true),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 41 
            COM(3,38),
            ARG(1, true),
        ], 
        // AExp22
        vec![ // 42 
            Y,
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 43 
            COM(3,40),
            ARG(1, true),
        ], 
        // AExp23
        vec![ // 44 
            COM(4,2),
            PTR(0, true, true),
            COM(2,0),
        ], 
        vec![ // 45 
            COM(4,2),
            ARG(0, true),
            COM(2,0),
        ], 
        // AExp24
        vec![ // 46 
            COM(4,2),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 47 
            COM(4,2),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp25
        vec![ // 48 
            COM(1,64),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 49 
            ARG(0, true),
            ARG(2, true),
        ], 
        vec![ // 50 
            COM(4,2),
            ARG(1, true),
        ], 
        // AExp26
        vec![ // 51 
            COM(4,2),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 52 
            COM(3,48),
            ARG(1, true),
            ARG(2, false),
            ARG(3, false),
        ], 
        vec![ // 53 
            COM(3,46),
            ARG(0, true),
            ARG(2, false),
            ARG(3, false),
        ], 
        // AExp27
        vec![ // 54 
            ARG(2, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 55 
            COM(4,51),
            ARG(0, false),
            ARG(1, true),
        ], 
        vec![ // 56 
            COM(1,44),
            ARG(0, false),
        ], 
        // AExp28
        vec![ // 57 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 58 
            COM(3,54),
            ARG(0, true),
        ], 
        // AExp29
        vec![ // 59 
            COM(4,2),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 60 
            ARG(1, true),
            ARG(3, true),
        ], 
        vec![ // 61 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp30
        vec![ // 62 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 63 
            COM(4,59),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp31
        vec![ // 64 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 65 
            COM(3,62),
            ARG(0, true),
        ], 
    ],

}});