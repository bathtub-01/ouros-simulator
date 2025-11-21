use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 34
#[rustfmt::skip]
pub static ORDLIST: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            PTR(5, false, false),
            PTR(4, false, false),
            INT(0),
            INT(1),
        ], 
        vec![ // 1 
            COM(3,71),
            COM(2,1),
        ], 
        vec![ // 2 
            COM(1,0),
            COM(3,71),
            PTR(1, false, false),
        ], 
        vec![ // 3 
            COM(1,0),
            COM(3,71),
            PTR(2, false, false),
        ], 
        vec![ // 4 
            COM(1,0),
            COM(3,71),
            PTR(3, false, false),
        ], 
        // AExp1
        vec![ // 5 
            COM(2,2),
            PTR(6, false, false),
        ], 
        vec![ // 6 
            COM(3,4),
            COM(1,7),
            COM(1,10),
        ], 
        // AExp2
        vec![ // 7 
            COM(2,13),
            COM(2,14),
        ], 
        // AExp3
        vec![ // 8 
            COM(3,16),
            PTR(9, false, false),
        ], 
        vec![ // 9 
            COM(4,18),
            COM(3,20),
        ], 
        // AExp4
        vec![ // 10 
            COM(2,23),
            PTR(11, false, false),
        ], 
        vec![ // 11 
            COM(4,25),
            COM(4,27),
        ], 
        // AExp5
        vec![ // 12 
            COM(3,30),
            COM(2,33),
        ], 
        // AExp6
        vec![ // 13 
            COM(2,36),
            PTR(15, false, false),
        ], 
        vec![ // 14 
            COM(4,39),
            COM(2,42),
        ], 
        vec![ // 15 
            COM(3,37),
            PTR(14, false, false),
        ], 
        // AExp7
        vec![ // 16 
            COM(2,45),
            PTR(18, false, false),
        ], 
        vec![ // 17 
            COM(6,50),
            COM(3,53),
            COM(3,55),
        ], 
        vec![ // 18 
            COM(4,47),
            PTR(17, false, false),
        ], 
        // AExp8
        vec![ // 19 
            COM(2,57),
            PTR(21, false, false),
        ], 
        vec![ // 20 
            COM(3,62),
            COM(1,65),
            COM(1,68),
        ], 
        vec![ // 21 
            COM(2,59),
            PTR(20, false, false),
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
            COM(1,0),
            PTR(7, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 3 
            ARG(0),
            ARG(1),
        ], 
        // AExp3
        vec![ // 4 
            PTR(8, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 5 
            ARG(1),
            ARG(2),
        ], 
        vec![ // 6 
            ARG(0),
            ARG(2),
        ], 
        // AExp4
        vec![ // 7 
            PTR(10, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 8 
            PTR(19, false, false),
            ARG(0),
        ], 
        vec![ // 9 
            PTR(12, false, false),
            COM(2,1),
        ], 
        // AExp5
        vec![ // 10 
            PTR(10, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 11 
            PTR(19, false, false),
            ARG(0),
        ], 
        vec![ // 12 
            PTR(12, false, false),
            COM(2,0),
        ], 
        // AExp6
        vec![ // 13 
            ARG(1),
            COM(2,1),
            ARG(0),
        ], 
        // AExp7
        vec![ // 14 
            ARG(0),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 15 
            PTR(7, false, false),
            ARG(1),
        ], 
        // AExp8
        vec![ // 16 
            Y,
            PTR(0, true, true),
            ARG(1),
        ], 
        vec![ // 17 
            ARG(0),
            ARG(2),
        ], 
        // AExp9
        vec![ // 18 
            ARG(3),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 19 
            ARG(0),
            ARG(2),
        ], 
        // AExp10
        vec![ // 20 
            COM(4,22),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 21 
            ARG(0),
            ARG(2),
        ], 
        // AExp11
        vec![ // 22 
            ARG(3),
            ARG(0),
            ARG(1),
        ], 
        // AExp12
        vec![ // 23 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 24 
            ARG(0),
            ARG(1),
        ], 
        // AExp13
        vec![ // 25 
            ARG(3),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 26 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp14
        vec![ // 27 
            COM(4,22),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 28 
            ARG(1),
            ARG(3),
        ], 
        vec![ // 29 
            ARG(0),
            ARG(2),
        ], 
        // AExp15
        vec![ // 30 
            COM(1,35),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 31 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        vec![ // 32 
            PTR(13, false, false),
            ARG(2),
        ], 
        // AExp16
        vec![ // 33 
            PTR(13, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 34 
            PTR(16, false, false),
            ARG(0),
            ARG(1),
        ], 
        // AExp17
        vec![ // 35 
            ARG(0),
            COM(2,1),
        ], 
        // AExp18
        vec![ // 36 
            ARG(1),
            COM(2,1),
            ARG(0),
        ], 
        // AExp19
        vec![ // 37 
            ARG(2),
            COM(2,1),
            PTR(0, true, true),
        ], 
        vec![ // 38 
            ARG(0),
            ARG(1),
        ], 
        // AExp20
        vec![ // 39 
            COM(1,44),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 40 
            ARG(0),
            ARG(2),
            ARG(3),
        ], 
        vec![ // 41 
            COM(1,35),
            ARG(1),
            ARG(2),
        ], 
        // AExp21
        vec![ // 42 
            PTR(13, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 43 
            COM(4,22),
            ARG(0),
            ARG(1),
        ], 
        // AExp22
        vec![ // 44 
            ARG(0),
            COM(2,0),
        ], 
        // AExp23
        vec![ // 45 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 46 
            ARG(0),
            ARG(1),
        ], 
        // AExp24
        vec![ // 47 
            ARG(3),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 48 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        vec![ // 49 
            COM(4,22),
            ARG(1),
            COM(2,0),
        ], 
        // AExp25
        vec![ // 50 
            COM(1,35),
            ARG(2),
            ARG(4),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 51 
            ARG(1),
            ARG(2),
            ARG(4),
            ARG(5),
        ], 
        vec![ // 52 
            ARG(0),
            ARG(3),
            ARG(4),
            ARG(5),
        ], 
        // AExp26
        vec![ // 53 
            COM(4,22),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 54 
            ARG(0),
            ARG(2),
        ], 
        // AExp27
        vec![ // 55 
            COM(4,22),
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 56 
            COM(4,22),
            ARG(1),
            ARG(2),
        ], 
        // AExp28
        vec![ // 57 
            ARG(1),
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 58 
            COM(4,22),
            COM(2,0),
            COM(2,0),
        ], 
        // AExp29
        vec![ // 59 
            PTR(8, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 60 
            ARG(0),
            ARG(1),
        ], 
        vec![ // 61 
            PTR(19, false, false),
            ARG(1),
        ], 
        // AExp30
        vec![ // 62 
            PTR(8, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 63 
            ARG(1),
            ARG(2),
        ], 
        vec![ // 64 
            ARG(0),
            ARG(2),
        ], 
        // AExp31
        vec![ // 65 
            PTR(10, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 66 
            PTR(19, false, false),
            ARG(0),
        ], 
        vec![ // 67 
            COM(4,22),
            COM(2,0),
        ], 
        // AExp32
        vec![ // 68 
            PTR(10, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 69 
            PTR(19, false, false),
            ARG(0),
        ], 
        vec![ // 70 
            COM(4,22),
            COM(2,1),
        ], 
        // AExp33
        vec![ // 71 
            ARG(1),
            ARG(0),
        ], 
    ],

}});