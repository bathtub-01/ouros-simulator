use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 42
#[rustfmt::skip]
pub static TRIBELIE: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            COM(1,5),
            COM(1,28),
            PTR(1, false, false),
        ], 
        // AExp1
        vec![ // 1 
            Y,
            PTR(26, false, false),
            PTR(3, false, false),
        ], 
        vec![ // 2 
            COM(4,20),
            COM(2,1),
            COM(2,0),
        ], 
        vec![ // 3 
            COM(4,20),
            COM(2,0),
            PTR(2, false, false),
        ], 
        vec![ // 4 
            COM(4,20),
            COM(2,1),
            COM(2,0),
        ], 
        vec![ // 5 
            COM(4,20),
            COM(2,0),
            PTR(4, false, false),
        ], 
        vec![ // 6 
            COM(4,20),
            COM(2,1),
            COM(2,0),
        ], 
        vec![ // 7 
            COM(4,20),
            COM(2,0),
            PTR(6, false, false),
        ], 
        vec![ // 8 
            COM(4,20),
            COM(2,1),
            COM(2,0),
        ], 
        vec![ // 9 
            COM(4,20),
            COM(2,0),
            PTR(8, false, false),
        ], 
        vec![ // 10 
            COM(4,20),
            COM(2,1),
            COM(2,0),
        ], 
        vec![ // 11 
            COM(4,20),
            COM(2,0),
            PTR(10, false, false),
        ], 
        vec![ // 12 
            COM(4,20),
            COM(2,1),
            COM(2,0),
        ], 
        vec![ // 13 
            COM(4,20),
            COM(2,0),
            PTR(12, false, false),
        ], 
        vec![ // 14 
            COM(4,20),
            COM(2,1),
            COM(2,0),
        ], 
        vec![ // 15 
            COM(4,20),
            COM(2,0),
            PTR(14, false, false),
        ], 
        vec![ // 16 
            COM(4,20),
            COM(2,1),
            COM(2,0),
        ], 
        vec![ // 17 
            COM(4,20),
            COM(2,0),
            PTR(16, false, false),
        ], 
        vec![ // 18 
            COM(2,76),
            PTR(17, false, false),
        ], 
        vec![ // 19 
            COM(6,65),
            PTR(18, false, false),
            PTR(15, false, false),
        ], 
        vec![ // 20 
            COM(5,61),
            PTR(19, false, false),
            PTR(13, false, false),
        ], 
        vec![ // 21 
            COM(4,57),
            PTR(20, false, false),
            PTR(11, false, false),
        ], 
        vec![ // 22 
            COM(3,53),
            PTR(21, false, false),
            PTR(9, false, false),
        ], 
        vec![ // 23 
            COM(6,49),
            PTR(22, false, false),
            PTR(7, false, false),
        ], 
        vec![ // 24 
            COM(6,43),
            PTR(23, false, false),
        ], 
        vec![ // 25 
            COM(5,41),
            PTR(24, false, false),
            PTR(5, false, false),
        ], 
        vec![ // 26 
            COM(3,39),
            PTR(25, false, false),
        ], 
        // AExp2
        vec![ // 27 
            COM(2,11),
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
            COM(1,0),
            PTR(27, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 3 
            COM(1,18),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp3
        vec![ // 4 
            ARG(0, true),
            ARG(1, true),
            INT(0),
            INT(1),
        ], 
        // AExp4
        vec![ // 5 
            COM(2,2),
            PTR(0, true, true),
        ], 
        vec![ // 6 
            COM(2,4),
            ARG(0, true),
        ], 
        // AExp5
        vec![ // 7 
            ARG(0, true),
            ARG(2, true),
            PTR(0, true, true),
        ], 
        vec![ // 8 
            ARG(1, true),
            ARG(3, true),
        ], 
        // AExp6
        vec![ // 9 
            ARG(3, true),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 10 
            COM(4,7),
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp7
        vec![ // 11 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 12 
            COM(4,9),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp8
        vec![ // 13 
            COM(4,20),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 14 
            ARG(1, true),
            ARG(3, true),
        ], 
        vec![ // 15 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp9
        vec![ // 16 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 17 
            COM(4,13),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp10
        vec![ // 18 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 19 
            COM(3,16),
            ARG(0, true),
        ], 
        // AExp11
        vec![ // 20 
            ARG(3, true),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp12
        vec![ // 21 
            COM(1,29),
            PTR(0, true, true),
            ARG(1, true),
        ], 
        vec![ // 22 
            COM(2,30),
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp13
        vec![ // 23 
            COM(1,29),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 24 
            COM(3,36),
            ARG(1, true),
            ARG(2, true),
            ARG(3, false),
        ], 
        vec![ // 25 
            COM(2,32),
            ARG(0, true),
            ARG(3, false),
        ], 
        // AExp14
        vec![ // 26 
            COM(7,21),
            ARG(2, false),
            PTR(0, true, true),
            ARG(3, false),
        ], 
        vec![ // 27 
            COM(4,23),
            ARG(0, true),
            ARG(1, true),
            ARG(2, false),
            ARG(3, false),
        ], 
        // AExp15
        vec![ // 28 
            ARG(0, true),
            COM(4,26),
        ], 
        // AExp16
        vec![ // 29 
            ARG(0, true),
            COM(2,0),
        ], 
        // AExp17
        vec![ // 30 
            ARG(0, true),
            PTR(0, true, true),
            COM(2,1),
        ], 
        vec![ // 31 
            ARG(1, true),
            COM(2,1),
            COM(2,0),
        ], 
        // AExp18
        vec![ // 32 
            ARG(0, true),
            PTR(0, true, true),
            COM(2,1),
        ], 
        vec![ // 33 
            ARG(1, true),
            COM(2,1),
            COM(2,0),
        ], 
        // AExp19
        vec![ // 34 
            ARG(0, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 35 
            ARG(1, true),
            COM(2,1),
            COM(2,0),
        ], 
        // AExp20
        vec![ // 36 
            ARG(0, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 37 
            ARG(2, false),
            COM(2,0),
            COM(2,1),
        ], 
        vec![ // 38 
            COM(2,34),
            ARG(1, true),
            ARG(2, false),
        ], 
        // AExp21
        vec![ // 39 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 40 
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp22
        vec![ // 41 
            Y,
            PTR(0, true, true),
            ARG(1, true),
        ], 
        vec![ // 42 
            ARG(0, true),
            ARG(2, true),
            ARG(3, true),
            ARG(4, true),
        ], 
        // AExp23
        vec![ // 43 
            ARG(5, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 44 
            ARG(0, true),
            ARG(2, true),
            ARG(4, true),
        ], 
        vec![ // 45 
            ARG(1, true),
            ARG(3, true),
        ], 
        // AExp24
        vec![ // 46 
            ARG(6, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 47 
            ARG(0, true),
            ARG(1, true),
            ARG(3, true),
            ARG(5, true),
        ], 
        vec![ // 48 
            ARG(2, true),
            ARG(4, true),
        ], 
        // AExp25
        vec![ // 49 
            Y,
            PTR(0, true, true),
            ARG(1, true),
        ], 
        vec![ // 50 
            COM(7,46),
            ARG(0, true),
            ARG(2, true),
            ARG(3, true),
            ARG(4, true),
            ARG(5, true),
        ], 
        // AExp26
        vec![ // 51 
            Y,
            PTR(0, true, true),
            ARG(1, true),
        ], 
        vec![ // 52 
            COM(7,46),
            ARG(0, true),
            ARG(2, true),
            ARG(3, true),
            ARG(4, true),
            ARG(5, true),
        ], 
        // AExp27
        vec![ // 53 
            COM(6,51),
            PTR(0, true, true),
            ARG(1, true),
        ], 
        vec![ // 54 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp28
        vec![ // 55 
            Y,
            PTR(0, true, true),
            ARG(1, true),
        ], 
        vec![ // 56 
            COM(7,46),
            ARG(0, true),
            ARG(2, true),
            ARG(3, true),
            ARG(4, true),
            ARG(5, true),
        ], 
        // AExp29
        vec![ // 57 
            COM(6,55),
            PTR(0, true, true),
            ARG(1, true),
        ], 
        vec![ // 58 
            ARG(0, true),
            ARG(2, true),
            ARG(3, true),
        ], 
        // AExp30
        vec![ // 59 
            Y,
            PTR(0, true, true),
            ARG(1, true),
        ], 
        vec![ // 60 
            COM(7,46),
            ARG(0, true),
            ARG(2, true),
            ARG(3, true),
            ARG(4, true),
            ARG(5, true),
        ], 
        // AExp31
        vec![ // 61 
            COM(6,59),
            PTR(0, true, true),
            ARG(1, true),
        ], 
        vec![ // 62 
            ARG(0, true),
            ARG(2, true),
            ARG(3, true),
            ARG(4, true),
        ], 
        // AExp32
        vec![ // 63 
            Y,
            PTR(0, true, true),
            ARG(1, true),
        ], 
        vec![ // 64 
            COM(7,46),
            ARG(0, true),
            ARG(2, true),
            ARG(3, true),
            ARG(4, true),
            ARG(5, true),
        ], 
        // AExp33
        vec![ // 65 
            COM(6,63),
            PTR(0, true, true),
            ARG(1, true),
        ], 
        vec![ // 66 
            ARG(0, true),
            ARG(2, true),
            ARG(3, true),
            ARG(4, true),
            ARG(5, true),
        ], 
        // AExp34
        vec![ // 67 
            ARG(0, true),
            PTR(0, true, true),
            ARG(2, true),
        ], 
        vec![ // 68 
            ARG(1, true),
            ARG(3, true),
            ARG(4, true),
            ARG(5, true),
            ARG(6, true),
        ], 
        // AExp35
        vec![ // 69 
            Y,
            PTR(0, true, true),
            ARG(1, true),
        ], 
        vec![ // 70 
            COM(7,46),
            ARG(0, true),
            ARG(2, true),
            ARG(3, true),
            ARG(4, true),
            ARG(5, true),
        ], 
        // AExp36
        vec![ // 71 
            COM(4,20),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 72 
            ARG(4, true),
            ARG(6, true),
        ], 
        vec![ // 73 
            ARG(0, true),
            ARG(1, true),
            ARG(2, true),
            ARG(3, true),
            ARG(5, true),
        ], 
        // AExp37
        vec![ // 74 
            COM(7,71),
            PTR(0, true, true),
        ], 
        vec![ // 75 
            COM(3,80),
            ARG(0, true),
            ARG(1, true),
            ARG(2, true),
            ARG(3, true),
        ], 
        // AExp38
        vec![ // 76 
            COM(7,67),
            COM(6,69),
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 77 
            COM(4,74),
            ARG(1, true),
        ], 
        // AExp39
        vec![ // 78 
            ARG(0, true),
            ARG(1, true),
            ARG(2, true),
            ARG(3, true),
            ARG(6, true),
            ARG(4, true),
            ARG(5, true),
        ], 
        // AExp40
        vec![ // 79 
            ARG(6, true),
            ARG(0, true),
            ARG(1, true),
            ARG(2, true),
            ARG(3, true),
            ARG(4, true),
            ARG(5, true),
        ], 
        // AExp41
        vec![ // 80 
            COM(7,78),
            PTR(0, true, true),
        ], 
        vec![ // 81 
            COM(7,79),
            ARG(0, true),
            ARG(1, true),
            ARG(2, true),
        ], 
    ],

}});