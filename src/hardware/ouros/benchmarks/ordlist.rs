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
            COM(1,11),
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
            COM(1,28),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 3 
            COM(1,69),
            ARG(0, true),
        ], 
        vec![ // 4 
            COM(2,32),
            COM(2,1),
        ], 
        // AExp3
        vec![ // 5 
            COM(1,28),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 6 
            COM(1,69),
            ARG(0, true),
        ], 
        vec![ // 7 
            COM(2,32),
            COM(2,0),
        ], 
        // AExp4
        vec![ // 8 
            COM(2,20),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 9 
            COM(1,5),
            ARG(0, false),
        ], 
        vec![ // 10 
            COM(1,2),
            ARG(0, false),
        ], 
        // AExp5
        vec![ // 11 
            COM(1,0),
            COM(1,15),
            PTR(0, true, true),
        ], 
        vec![ // 12 
            COM(1,8),
            ARG(0, true),
        ], 
        // AExp6
        vec![ // 13 
            ARG(0, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 14 
            COM(1,15),
            ARG(1, true),
        ], 
        // AExp7
        vec![ // 15 
            ARG(0, true),
            COM(2,1),
            COM(2,13),
        ], 
        // AExp8
        vec![ // 16 
            COM(4,22),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 17 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp9
        vec![ // 18 
            ARG(2, true),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 19 
            COM(3,16),
            ARG(1, true),
        ], 
        // AExp10
        vec![ // 20 
            Y,
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 21 
            COM(3,18),
            ARG(1, true),
        ], 
        // AExp11
        vec![ // 22 
            ARG(3, true),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp12
        vec![ // 23 
            COM(4,22),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 24 
            ARG(1, true),
            ARG(3, true),
        ], 
        vec![ // 25 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp13
        vec![ // 26 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 27 
            COM(4,23),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp14
        vec![ // 28 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 29 
            COM(3,26),
            ARG(0, true),
        ], 
        // AExp15
        vec![ // 30 
            COM(1,43),
            PTR(0, true, true),
        ], 
        vec![ // 31 
            COM(1,55),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp16
        vec![ // 32 
            COM(1,35),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 33 
            COM(2,30),
            ARG(0, true),
            ARG(1, false),
        ], 
        vec![ // 34 
            COM(1,43),
            ARG(1, false),
        ], 
        // AExp17
        vec![ // 35 
            ARG(0, true),
            COM(2,1),
        ], 
        // AExp18
        vec![ // 36 
            COM(1,43),
            PTR(0, true, true),
        ], 
        vec![ // 37 
            COM(4,22),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp19
        vec![ // 38 
            COM(1,44),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 39 
            COM(2,36),
            ARG(1, false),
            ARG(2, true),
        ], 
        vec![ // 40 
            COM(1,35),
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp20
        vec![ // 41 
            ARG(1, true),
            COM(2,1),
            PTR(0, true, true),
        ], 
        vec![ // 42 
            COM(3,38),
            ARG(0, true),
        ], 
        // AExp21
        vec![ // 43 
            ARG(0, true),
            COM(2,1),
            COM(2,41),
        ], 
        // AExp22
        vec![ // 44 
            ARG(0, true),
            COM(2,0),
        ], 
        // AExp23
        vec![ // 45 
            COM(4,22),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 46 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp24
        vec![ // 47 
            COM(4,22),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 48 
            COM(4,22),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp25
        vec![ // 49 
            COM(1,35),
            ARG(0, false),
            ARG(2, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 50 
            COM(3,47),
            ARG(0, false),
            ARG(2, false),
            ARG(3, false),
        ], 
        vec![ // 51 
            COM(3,45),
            ARG(1, true),
            ARG(2, false),
            ARG(3, false),
        ], 
        // AExp26
        vec![ // 52 
            ARG(2, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 53 
            COM(4,49),
            ARG(0, false),
            ARG(1, true),
        ], 
        vec![ // 54 
            COM(4,22),
            ARG(0, false),
            COM(2,0),
        ], 
        // AExp27
        vec![ // 55 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 56 
            COM(3,52),
            ARG(0, true),
        ], 
        // AExp28
        vec![ // 57 
            COM(1,28),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 58 
            COM(1,69),
            ARG(0, true),
        ], 
        vec![ // 59 
            COM(4,22),
            COM(2,0),
        ], 
        // AExp29
        vec![ // 60 
            COM(1,28),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 61 
            COM(1,69),
            ARG(0, true),
        ], 
        vec![ // 62 
            COM(4,22),
            COM(2,1),
        ], 
        // AExp30
        vec![ // 63 
            COM(2,20),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 64 
            COM(1,60),
            ARG(0, false),
        ], 
        vec![ // 65 
            COM(1,57),
            ARG(0, false),
        ], 
        // AExp31
        vec![ // 66 
            COM(2,20),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 67 
            COM(1,63),
            ARG(0, false),
        ], 
        vec![ // 68 
            COM(1,69),
            ARG(0, false),
        ], 
        // AExp32
        vec![ // 69 
            ARG(0, true),
            COM(1,66),
            PTR(0, true, true),
        ], 
        vec![ // 70 
            COM(4,22),
            COM(2,0),
            COM(2,0),
        ], 
        // AExp33
        vec![ // 71 
            ARG(1, true),
            ARG(0, true),
        ], 
    ],

}});