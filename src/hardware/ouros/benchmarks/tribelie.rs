use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 41
#[rustfmt::skip]
pub static TRIBELIE: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            PTR(1, false, false),
            PTR(7, false, false),
            PTR(10, false, false),
        ], 
        // AExp1
        vec![ // 1 
            COM(3,2),
            COM(2,4),
            COM(2,6),
        ], 
        // AExp2
        vec![ // 2 
            PTR(3, false, false),
            PRM(ADD,false),
            INT(0),
        ], 
        // AExp3
        vec![ // 3 
            COM(3,7),
            PTR(4, false, false),
        ], 
        vec![ // 4 
            COM(5,9),
            COM(4,11),
        ], 
        // AExp4
        vec![ // 5 
            COM(2,13),
            PTR(6, false, false),
        ], 
        vec![ // 6 
            COM(4,15),
            COM(4,17),
        ], 
        // AExp5
        vec![ // 7 
            COM(2,21),
            PTR(8, false, false),
        ], 
        vec![ // 8 
            COM(5,24),
            COM(4,26),
        ], 
        // AExp6
        vec![ // 9 
            COM(4,34),
            COM(2,37),
        ], 
        // AExp7
        vec![ // 10 
            Y,
            PTR(35, false, false),
            PTR(12, false, false),
        ], 
        vec![ // 11 
            COM(4,20),
            COM(2,1),
            COM(2,0),
        ], 
        vec![ // 12 
            COM(4,20),
            COM(2,0),
            PTR(11, false, false),
        ], 
        vec![ // 13 
            COM(4,20),
            COM(2,1),
            COM(2,0),
        ], 
        vec![ // 14 
            COM(4,20),
            COM(2,0),
            PTR(13, false, false),
        ], 
        vec![ // 15 
            COM(4,20),
            COM(2,1),
            COM(2,0),
        ], 
        vec![ // 16 
            COM(4,20),
            COM(2,0),
            PTR(15, false, false),
        ], 
        vec![ // 17 
            COM(4,20),
            COM(2,1),
            COM(2,0),
        ], 
        vec![ // 18 
            COM(4,20),
            COM(2,0),
            PTR(17, false, false),
        ], 
        vec![ // 19 
            COM(4,20),
            COM(2,1),
            COM(2,0),
        ], 
        vec![ // 20 
            COM(4,20),
            COM(2,0),
            PTR(19, false, false),
        ], 
        vec![ // 21 
            COM(4,20),
            COM(2,1),
            COM(2,0),
        ], 
        vec![ // 22 
            COM(4,20),
            COM(2,0),
            PTR(21, false, false),
        ], 
        vec![ // 23 
            COM(4,20),
            COM(2,1),
            COM(2,0),
        ], 
        vec![ // 24 
            COM(4,20),
            COM(2,0),
            PTR(23, false, false),
        ], 
        vec![ // 25 
            COM(4,20),
            COM(2,1),
            COM(2,0),
        ], 
        vec![ // 26 
            COM(4,20),
            COM(2,0),
            PTR(25, false, false),
        ], 
        vec![ // 27 
            COM(4,67),
            COM(6,69),
            COM(4,74),
            PTR(26, false, false),
        ], 
        vec![ // 28 
            COM(7,63),
            COM(6,65),
            PTR(27, false, false),
            PTR(24, false, false),
        ], 
        vec![ // 29 
            COM(6,59),
            COM(6,61),
            PTR(28, false, false),
            PTR(22, false, false),
        ], 
        vec![ // 30 
            COM(5,55),
            COM(6,57),
            PTR(29, false, false),
            PTR(20, false, false),
        ], 
        vec![ // 31 
            COM(4,51),
            COM(6,53),
            PTR(30, false, false),
            PTR(18, false, false),
        ], 
        vec![ // 32 
            COM(6,49),
            PTR(31, false, false),
            PTR(16, false, false),
        ], 
        vec![ // 33 
            COM(6,43),
            PTR(32, false, false),
        ], 
        vec![ // 34 
            COM(5,41),
            PTR(33, false, false),
            PTR(14, false, false),
        ], 
        vec![ // 35 
            COM(3,39),
            PTR(34, false, false),
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
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 3 
            ARG(1),
            ARG(2),
        ], 
        // AExp3
        vec![ // 4 
            COM(1,0),
            PTR(2, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 5 
            PTR(5, false, false),
            ARG(0),
            ARG(1),
        ], 
        // AExp4
        vec![ // 6 
            ARG(0),
            ARG(1),
            INT(0),
            INT(1),
        ], 
        // AExp5
        vec![ // 7 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 8 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp6
        vec![ // 9 
            ARG(4),
            ARG(2),
            PTR(0, true, true),
        ], 
        vec![ // 10 
            ARG(0),
            ARG(1),
            ARG(3),
        ], 
        // AExp7
        vec![ // 11 
            ARG(0),
            ARG(2),
            PTR(0, true, true),
        ], 
        vec![ // 12 
            ARG(1),
            ARG(3),
        ], 
        // AExp8
        vec![ // 13 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 14 
            ARG(0),
            ARG(1),
        ], 
        // AExp9
        vec![ // 15 
            ARG(3),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 16 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp10
        vec![ // 17 
            COM(4,20),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 18 
            ARG(1),
            ARG(3),
        ], 
        vec![ // 19 
            ARG(0),
            ARG(2),
        ], 
        // AExp11
        vec![ // 20 
            ARG(3),
            ARG(0),
            ARG(1),
        ], 
        // AExp12
        vec![ // 21 
            ARG(1),
            ARG(0),
        ], 
        // AExp13
        vec![ // 22 
            COM(1,29),
            PTR(0, true, true),
            ARG(1),
        ], 
        vec![ // 23 
            COM(2,30),
            ARG(0),
            ARG(2),
        ], 
        // AExp14
        vec![ // 24 
            COM(7,22),
            ARG(3),
            PTR(0, true, true),
            ARG(4),
        ], 
        vec![ // 25 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(4),
        ], 
        // AExp15
        vec![ // 26 
            COM(1,29),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 27 
            PTR(9, false, false),
            ARG(1),
            ARG(2),
            ARG(3),
        ], 
        vec![ // 28 
            COM(2,32),
            ARG(0),
            ARG(3),
        ], 
        // AExp16
        vec![ // 29 
            ARG(0),
            COM(2,0),
        ], 
        // AExp17
        vec![ // 30 
            ARG(0),
            PTR(0, true, true),
            COM(2,1),
        ], 
        vec![ // 31 
            ARG(1),
            COM(2,1),
            COM(2,0),
        ], 
        // AExp18
        vec![ // 32 
            ARG(0),
            PTR(0, true, true),
            COM(2,1),
        ], 
        vec![ // 33 
            ARG(1),
            COM(2,1),
            COM(2,0),
        ], 
        // AExp19
        vec![ // 34 
            ARG(1),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 35 
            ARG(3),
            COM(2,0),
            COM(2,1),
        ], 
        vec![ // 36 
            ARG(0),
            ARG(2),
            ARG(3),
        ], 
        // AExp20
        vec![ // 37 
            ARG(0),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 38 
            ARG(1),
            COM(2,1),
            COM(2,0),
        ], 
        // AExp21
        vec![ // 39 
            ARG(2),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 40 
            ARG(0),
            ARG(1),
        ], 
        // AExp22
        vec![ // 41 
            Y,
            PTR(0, true, true),
            ARG(1),
        ], 
        vec![ // 42 
            ARG(0),
            ARG(2),
            ARG(3),
            ARG(4),
        ], 
        // AExp23
        vec![ // 43 
            ARG(5),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 44 
            ARG(0),
            ARG(2),
            ARG(4),
        ], 
        vec![ // 45 
            ARG(1),
            ARG(3),
        ], 
        // AExp24
        vec![ // 46 
            ARG(6),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 47 
            ARG(0),
            ARG(1),
            ARG(3),
            ARG(5),
        ], 
        vec![ // 48 
            ARG(2),
            ARG(4),
        ], 
        // AExp25
        vec![ // 49 
            Y,
            PTR(0, true, true),
            ARG(1),
        ], 
        vec![ // 50 
            COM(7,46),
            ARG(0),
            ARG(2),
            ARG(3),
            ARG(4),
            ARG(5),
        ], 
        // AExp26
        vec![ // 51 
            ARG(0),
            PTR(0, true, true),
            ARG(2),
        ], 
        vec![ // 52 
            ARG(1),
            ARG(3),
        ], 
        // AExp27
        vec![ // 53 
            Y,
            PTR(0, true, true),
            ARG(1),
        ], 
        vec![ // 54 
            COM(7,46),
            ARG(0),
            ARG(2),
            ARG(3),
            ARG(4),
            ARG(5),
        ], 
        // AExp28
        vec![ // 55 
            ARG(0),
            PTR(0, true, true),
            ARG(2),
        ], 
        vec![ // 56 
            ARG(1),
            ARG(3),
            ARG(4),
        ], 
        // AExp29
        vec![ // 57 
            Y,
            PTR(0, true, true),
            ARG(1),
        ], 
        vec![ // 58 
            COM(7,46),
            ARG(0),
            ARG(2),
            ARG(3),
            ARG(4),
            ARG(5),
        ], 
        // AExp30
        vec![ // 59 
            ARG(0),
            PTR(0, true, true),
            ARG(2),
        ], 
        vec![ // 60 
            ARG(1),
            ARG(3),
            ARG(4),
            ARG(5),
        ], 
        // AExp31
        vec![ // 61 
            Y,
            PTR(0, true, true),
            ARG(1),
        ], 
        vec![ // 62 
            COM(7,46),
            ARG(0),
            ARG(2),
            ARG(3),
            ARG(4),
            ARG(5),
        ], 
        // AExp32
        vec![ // 63 
            ARG(0),
            PTR(0, true, true),
            ARG(2),
        ], 
        vec![ // 64 
            ARG(1),
            ARG(3),
            ARG(4),
            ARG(5),
            ARG(6),
        ], 
        // AExp33
        vec![ // 65 
            Y,
            PTR(0, true, true),
            ARG(1),
        ], 
        vec![ // 66 
            COM(7,46),
            ARG(0),
            ARG(2),
            ARG(3),
            ARG(4),
            ARG(5),
        ], 
        // AExp34
        vec![ // 67 
            COM(7,63),
            ARG(0),
            PTR(0, true, true),
            ARG(2),
        ], 
        vec![ // 68 
            ARG(1),
            ARG(3),
        ], 
        // AExp35
        vec![ // 69 
            Y,
            PTR(0, true, true),
            ARG(1),
        ], 
        vec![ // 70 
            COM(7,46),
            ARG(0),
            ARG(2),
            ARG(3),
            ARG(4),
            ARG(5),
        ], 
        // AExp36
        vec![ // 71 
            COM(4,20),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 72 
            ARG(4),
            ARG(6),
        ], 
        vec![ // 73 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(5),
        ], 
        // AExp37
        vec![ // 74 
            COM(7,71),
            PTR(0, true, true),
        ], 
        vec![ // 75 
            COM(3,78),
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(3),
        ], 
        // AExp38
        vec![ // 76 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(6),
            ARG(4),
            ARG(5),
        ], 
        // AExp39
        vec![ // 77 
            ARG(6),
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(4),
            ARG(5),
        ], 
        // AExp40
        vec![ // 78 
            COM(7,76),
            PTR(0, true, true),
        ], 
        vec![ // 79 
            COM(7,77),
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
    ],

}});