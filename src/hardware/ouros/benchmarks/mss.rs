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
            PTR(3, false, false),
            PTR(2, false, false),
        ], 
        vec![ // 1 
            PRM(SUB,false),
            INT(0),
            INT(20),
        ], 
        vec![ // 2 
            PTR(10, false, false),
            PTR(1, false, false),
            INT(20),
        ], 
        // AExp1
        vec![ // 3 
            COM(2,2),
            COM(1,4),
        ], 
        // AExp2
        vec![ // 4 
            COM(2,6),
            COM(2,8),
        ], 
        // AExp3
        vec![ // 5 
            COM(3,9),
            PTR(6, false, false),
        ], 
        vec![ // 6 
            COM(5,11),
            COM(4,13),
        ], 
        // AExp4
        vec![ // 7 
            COM(2,15),
            PTR(8, false, false),
        ], 
        vec![ // 8 
            COM(4,17),
            COM(4,19),
        ], 
        // AExp5
        vec![ // 9 
            PTR(5, false, false),
            PRM(ADD,false),
            INT(0),
        ], 
        // AExp6
        vec![ // 10 
            COM(3,54),
            PTR(11, false, false),
        ], 
        vec![ // 11 
            COM(3,56),
            COM(1,58),
        ], 
        // AExp7
        vec![ // 12 
            COM(2,25),
            PTR(13, false, false),
        ], 
        vec![ // 13 
            COM(4,27),
            COM(4,29),
        ], 
        // AExp8
        vec![ // 14 
            COM(3,32),
            PTR(15, false, false),
        ], 
        vec![ // 15 
            COM(4,34),
            COM(3,36),
        ], 
        // AExp9
        vec![ // 16 
            COM(2,38),
            COM(2,39),
        ], 
        // AExp10
        vec![ // 17 
            COM(2,42),
            PTR(18, false, false),
        ], 
        vec![ // 18 
            COM(4,45),
            COM(1,47),
        ], 
        // AExp11
        vec![ // 19 
            COM(2,49),
            PTR(20, false, false),
        ], 
        vec![ // 20 
            COM(3,50),
            COM(4,52),
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
            PTR(4, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 3 
            ARG(0),
            ARG(1),
        ], 
        // AExp3
        vec![ // 4 
            PTR(7, false, false),
            PTR(9, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 5 
            COM(1,23),
            ARG(0),
        ], 
        // AExp4
        vec![ // 6 
            ARG(1),
            INT(0),
            PTR(0, true, true),
        ], 
        vec![ // 7 
            PTR(5, false, false),
            ARG(0),
        ], 
        // AExp5
        vec![ // 8 
            PRM(LE,true),
            ARG(0),
            ARG(1),
            ARG(1),
            ARG(0),
        ], 
        // AExp6
        vec![ // 9 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 10 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp7
        vec![ // 11 
            ARG(4),
            ARG(2),
            PTR(0, true, true),
        ], 
        vec![ // 12 
            ARG(0),
            ARG(1),
            ARG(3),
        ], 
        // AExp8
        vec![ // 13 
            ARG(0),
            ARG(2),
            PTR(0, true, true),
        ], 
        vec![ // 14 
            ARG(1),
            ARG(3),
        ], 
        // AExp9
        vec![ // 15 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 16 
            ARG(0),
            ARG(1),
        ], 
        // AExp10
        vec![ // 17 
            ARG(3),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 18 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp11
        vec![ // 19 
            COM(4,22),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 20 
            ARG(1),
            ARG(3),
        ], 
        vec![ // 21 
            ARG(0),
            ARG(2),
        ], 
        // AExp12
        vec![ // 22 
            ARG(3),
            ARG(0),
            ARG(1),
        ], 
        // AExp13
        vec![ // 23 
            PTR(12, false, false),
            PTR(16, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 24 
            PTR(17, false, false),
            ARG(0),
        ], 
        // AExp14
        vec![ // 25 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 26 
            ARG(0),
            ARG(1),
        ], 
        // AExp15
        vec![ // 27 
            ARG(3),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 28 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp16
        vec![ // 29 
            PTR(14, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 30 
            ARG(1),
            ARG(3),
        ], 
        vec![ // 31 
            ARG(0),
            ARG(2),
        ], 
        // AExp17
        vec![ // 32 
            Y,
            PTR(0, true, true),
            ARG(1),
        ], 
        vec![ // 33 
            ARG(0),
            ARG(2),
        ], 
        // AExp18
        vec![ // 34 
            ARG(3),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 35 
            ARG(0),
            ARG(2),
        ], 
        // AExp19
        vec![ // 36 
            COM(4,22),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 37 
            ARG(0),
            ARG(2),
        ], 
        // AExp20
        vec![ // 38 
            ARG(1),
            COM(2,0),
            ARG(0),
        ], 
        // AExp21
        vec![ // 39 
            COM(4,22),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 40 
            PTR(16, false, false),
            ARG(1),
        ], 
        vec![ // 41 
            COM(4,22),
            ARG(0),
            ARG(1),
        ], 
        // AExp22
        vec![ // 42 
            ARG(1),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 43 
            ARG(0),
            ARG(1),
        ], 
        vec![ // 44 
            COM(4,22),
            COM(2,0),
            COM(2,0),
        ], 
        // AExp23
        vec![ // 45 
            COM(4,22),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 46 
            ARG(0),
            ARG(1),
        ], 
        // AExp24
        vec![ // 47 
            PTR(17, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 48 
            PTR(19, false, false),
            ARG(0),
        ], 
        // AExp25
        vec![ // 49 
            ARG(1),
            ERR(1),
            ARG(0),
        ], 
        // AExp26
        vec![ // 50 
            ARG(2),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 51 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp27
        vec![ // 52 
            COM(4,22),
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 53 
            PTR(19, false, false),
            ARG(1),
        ], 
        // AExp28
        vec![ // 54 
            PRM(LE,false),
            ARG(1),
            ARG(2),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 55 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp29
        vec![ // 56 
            COM(4,22),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 57 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp30
        vec![ // 58 
            PTR(10, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 59 
            PRM(ADD,false),
            ARG(0),
            INT(1),
        ], 
    ],

}});