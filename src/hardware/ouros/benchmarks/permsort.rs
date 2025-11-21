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
            PTR(9, false, false),
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
            PTR(10, false, false),
            PTR(7, false, false),
        ], 
        // AExp1
        vec![ // 9 
            COM(2,3),
            COM(2,0),
        ], 
        // AExp2
        vec![ // 10 
            COM(2,4),
            COM(1,6),
        ], 
        // AExp3
        vec![ // 11 
            COM(2,8),
            PTR(13, false, false),
        ], 
        vec![ // 12 
            COM(5,12),
            COM(3,15),
        ], 
        vec![ // 13 
            COM(4,10),
            PTR(12, false, false),
        ], 
        // AExp4
        vec![ // 14 
            COM(2,17),
            PTR(16, false, false),
        ], 
        vec![ // 15 
            COM(4,20),
            COM(2,23),
        ], 
        vec![ // 16 
            COM(3,18),
            PTR(15, false, false),
        ], 
        // AExp5
        vec![ // 17 
            COM(2,26),
            COM(2,28),
        ], 
        // AExp6
        vec![ // 18 
            COM(2,31),
            PTR(19, false, false),
        ], 
        vec![ // 19 
            COM(4,33),
            COM(4,35),
        ], 
        // AExp7
        vec![ // 20 
            COM(3,38),
            PTR(21, false, false),
        ], 
        vec![ // 21 
            COM(4,40),
            COM(3,42),
        ], 
        // AExp8
        vec![ // 22 
            COM(2,44),
            PTR(24, false, false),
        ], 
        vec![ // 23 
            COM(6,51),
            COM(3,54),
            COM(3,56),
        ], 
        vec![ // 24 
            COM(5,46),
            COM(1,49),
            PTR(23, false, false),
        ], 
        // AExp9
        vec![ // 25 
            COM(2,59),
            PTR(26, false, false),
        ], 
        vec![ // 26 
            COM(4,61),
            COM(4,63),
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
            ARG(3),
            ARG(0),
            ARG(1),
        ], 
        // AExp3
        vec![ // 3 
            ARG(1),
            ERR(3),
            ARG(0),
        ], 
        // AExp4
        vec![ // 4 
            PTR(9, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 5 
            ARG(0),
            ARG(1),
        ], 
        // AExp5
        vec![ // 6 
            PTR(11, false, false),
            PTR(14, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 7 
            PTR(17, false, false),
            ARG(0),
        ], 
        // AExp6
        vec![ // 8 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 9 
            ARG(0),
            ARG(1),
        ], 
        // AExp7
        vec![ // 10 
            ARG(3),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 11 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp8
        vec![ // 12 
            ARG(1),
            ARG(3),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 13 
            ARG(0),
            ARG(2),
            ARG(3),
            ARG(4),
        ], 
        vec![ // 14 
            ARG(2),
            ARG(4),
        ], 
        // AExp9
        vec![ // 15 
            COM(4,2),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 16 
            ARG(0),
            ARG(2),
        ], 
        // AExp10
        vec![ // 17 
            ARG(1),
            COM(2,1),
            ARG(0),
        ], 
        // AExp11
        vec![ // 18 
            ARG(2),
            COM(2,1),
            PTR(0, true, true),
        ], 
        vec![ // 19 
            ARG(0),
            ARG(1),
        ], 
        // AExp12
        vec![ // 20 
            COM(1,25),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 21 
            ARG(0),
            ARG(2),
            ARG(3),
        ], 
        vec![ // 22 
            PRM(LE,false),
            ARG(1),
            ARG(2),
        ], 
        // AExp13
        vec![ // 23 
            PTR(14, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 24 
            COM(4,2),
            ARG(0),
            ARG(1),
        ], 
        // AExp14
        vec![ // 25 
            ARG(0),
            COM(2,0),
        ], 
        // AExp15
        vec![ // 26 
            ARG(1),
            PTR(0, true, true),
            ARG(0),
        ], 
        vec![ // 27 
            COM(4,2),
            COM(2,0),
            COM(2,0),
        ], 
        // AExp16
        vec![ // 28 
            PTR(18, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 29 
            PTR(17, false, false),
            ARG(1),
        ], 
        vec![ // 30 
            PTR(22, false, false),
            ARG(0),
        ], 
        // AExp17
        vec![ // 31 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 32 
            ARG(0),
            ARG(1),
        ], 
        // AExp18
        vec![ // 33 
            ARG(3),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 34 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp19
        vec![ // 35 
            PTR(20, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 36 
            ARG(1),
            ARG(3),
        ], 
        vec![ // 37 
            ARG(0),
            ARG(2),
        ], 
        // AExp20
        vec![ // 38 
            Y,
            PTR(0, true, true),
            ARG(1),
        ], 
        vec![ // 39 
            ARG(0),
            ARG(2),
        ], 
        // AExp21
        vec![ // 40 
            ARG(3),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 41 
            ARG(0),
            ARG(2),
        ], 
        // AExp22
        vec![ // 42 
            COM(4,2),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 43 
            ARG(0),
            ARG(2),
        ], 
        // AExp23
        vec![ // 44 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 45 
            ARG(0),
            ARG(1),
        ], 
        // AExp24
        vec![ // 46 
            ARG(4),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 47 
            ARG(1),
            ARG(2),
            ARG(3),
        ], 
        vec![ // 48 
            ARG(0),
            ARG(2),
        ], 
        // AExp25
        vec![ // 49 
            COM(4,2),
            PTR(0, true, true),
            COM(2,0),
        ], 
        vec![ // 50 
            COM(4,2),
            ARG(0),
            COM(2,0),
        ], 
        // AExp26
        vec![ // 51 
            COM(4,2),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 52 
            ARG(1),
            ARG(3),
            ARG(4),
            ARG(5),
        ], 
        vec![ // 53 
            ARG(0),
            ARG(2),
            ARG(4),
            ARG(5),
        ], 
        // AExp27
        vec![ // 54 
            COM(4,2),
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 55 
            COM(4,2),
            ARG(1),
            ARG(2),
        ], 
        // AExp28
        vec![ // 56 
            PTR(25, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 57 
            ARG(0),
            ARG(2),
        ], 
        vec![ // 58 
            COM(4,2),
            ARG(1),
        ], 
        // AExp29
        vec![ // 59 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 60 
            ARG(0),
            ARG(1),
        ], 
        // AExp30
        vec![ // 61 
            ARG(3),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 62 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp31
        vec![ // 63 
            COM(4,2),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 64 
            ARG(1),
            ARG(3),
        ], 
        vec![ // 65 
            ARG(0),
            ARG(2),
        ], 
    ],

}});