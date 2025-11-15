use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 38
#[rustfmt::skip]
pub static BRAUN: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            COM(1,2),
            PTR(3, false, false),
        ], 
        vec![ // 1 
            PTR(25, false, false),
            INT(0),
            INT(255),
        ], 
        vec![ // 2 
            PTR(23, false, false),
            INT(2),
            PTR(1, false, false),
        ], 
        vec![ // 3 
            PTR(4, false, false),
            PTR(6, false, false),
            PTR(2, false, false),
        ], 
        // AExp1
        vec![ // 4 
            COM(2,3),
            PTR(5, false, false),
        ], 
        vec![ // 5 
            COM(4,5),
            COM(4,7),
        ], 
        // AExp2
        vec![ // 6 
            COM(2,11),
            COM(1,13),
        ], 
        // AExp3
        vec![ // 7 
            COM(4,15),
            COM(2,18),
            PTR(8, false, false),
        ], 
        vec![ // 8 
            COM(4,19),
            COM(4,21),
        ], 
        // AExp4
        vec![ // 9 
            COM(2,23),
            PTR(10, false, false),
        ], 
        vec![ // 10 
            COM(4,24),
            COM(2,26),
        ], 
        // AExp5
        vec![ // 11 
            COM(3,30),
            PTR(13, false, false),
        ], 
        vec![ // 12 
            COM(5,35),
            COM(3,37),
        ], 
        vec![ // 13 
            COM(4,32),
            PTR(12, false, false),
        ], 
        // AExp6
        vec![ // 14 
            COM(2,39),
            PTR(18, false, false),
        ], 
        vec![ // 15 
            COM(2,47),
            COM(2,1),
        ], 
        vec![ // 16 
            COM(2,45),
            COM(2,0),
        ], 
        vec![ // 17 
            COM(4,42),
            PTR(16, false, false),
            PTR(15, false, false),
        ], 
        vec![ // 18 
            COM(3,40),
            PTR(17, false, false),
        ], 
        // AExp7
        vec![ // 19 
            COM(3,49),
            COM(1,50),
            PTR(22, false, false),
        ], 
        vec![ // 20 
            COM(3,56),
            COM(2,1),
        ], 
        vec![ // 21 
            COM(5,53),
            PTR(20, false, false),
            COM(2,0),
        ], 
        vec![ // 22 
            COM(3,51),
            PTR(21, false, false),
        ], 
        // AExp8
        vec![ // 23 
            COM(3,59),
            PTR(24, false, false),
        ], 
        vec![ // 24 
            COM(3,61),
            COM(1,63),
        ], 
        // AExp9
        vec![ // 25 
            COM(3,65),
            PTR(26, false, false),
        ], 
        vec![ // 26 
            COM(3,67),
            COM(1,69),
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
            INT(0),
            INT(1),
        ], 
        // AExp3
        vec![ // 3 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 4 
            ARG(0),
            ARG(1),
        ], 
        // AExp4
        vec![ // 5 
            ARG(3),
            COM(2,1),
            PTR(0, true, true),
        ], 
        vec![ // 6 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp5
        vec![ // 7 
            COM(1,10),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 8 
            ARG(1),
            ARG(3),
        ], 
        vec![ // 9 
            ARG(0),
            ARG(2),
        ], 
        // AExp6
        vec![ // 10 
            ARG(0),
            COM(2,0),
        ], 
        // AExp7
        vec![ // 11 
            PTR(7, false, false),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 12 
            ARG(0),
            ARG(1),
        ], 
        // AExp8
        vec![ // 13 
            PTR(9, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 14 
            PTR(14, false, false),
            ARG(0),
        ], 
        // AExp9
        vec![ // 15 
            ARG(2),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 16 
            ARG(1),
            ARG(3),
        ], 
        vec![ // 17 
            ARG(3),
            COM(2,1),
            ARG(0),
        ], 
        // AExp10
        vec![ // 18 
            COM(2,0),
        ], 
        // AExp11
        vec![ // 19 
            ARG(1),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 20 
            ARG(0),
            ARG(2),
            ARG(3),
        ], 
        // AExp12
        vec![ // 21 
            PRM(EQ,false),
            ARG(0),
            ARG(2),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 22 
            PTR(7, false, false),
            ARG(1),
            ARG(3),
        ], 
        // AExp13
        vec![ // 23 
            ARG(1),
            ARG(0),
            COM(2,0),
        ], 
        // AExp14
        vec![ // 24 
            COM(4,29),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 25 
            ARG(0),
            ARG(2),
            ARG(3),
        ], 
        // AExp15
        vec![ // 26 
            PTR(11, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 27 
            PTR(9, false, false),
            ARG(1),
        ], 
        vec![ // 28 
            PTR(9, false, false),
            ARG(0),
        ], 
        // AExp16
        vec![ // 29 
            ARG(3),
            ARG(0),
            ARG(1),
        ], 
        // AExp17
        vec![ // 30 
            ARG(1),
            ARG(2),
            PTR(0, true, true),
        ], 
        vec![ // 31 
            ARG(0),
            ARG(2),
        ], 
        // AExp18
        vec![ // 32 
            ARG(1),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 33 
            ARG(0),
            ARG(2),
            ARG(3),
        ], 
        vec![ // 34 
            COM(4,29),
            ARG(2),
            ARG(3),
        ], 
        // AExp19
        vec![ // 35 
            COM(4,29),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 36 
            ARG(0),
            ARG(2),
            ARG(3),
            ARG(4),
        ], 
        // AExp20
        vec![ // 37 
            COM(4,29),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 38 
            PTR(11, false, false),
            ARG(0),
            ARG(2),
        ], 
        // AExp21
        vec![ // 39 
            ARG(1),
            COM(2,1),
            ARG(0),
        ], 
        // AExp22
        vec![ // 40 
            ARG(0),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 41 
            PTR(19, false, false),
            ARG(2),
        ], 
        // AExp23
        vec![ // 42 
            COM(5,58),
            ARG(2),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 43 
            ARG(1),
            ARG(3),
        ], 
        vec![ // 44 
            ARG(0),
            ARG(3),
        ], 
        // AExp24
        vec![ // 45 
            PTR(14, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 46 
            ARG(1),
            ARG(0),
        ], 
        // AExp25
        vec![ // 47 
            PTR(14, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 48 
            ARG(1),
            ARG(0),
        ], 
        // AExp26
        vec![ // 49 
            ARG(2),
            ARG(0),
            ARG(1),
        ], 
        // AExp27
        vec![ // 50 
            ARG(0),
            COM(2,0),
            COM(2,0),
        ], 
        // AExp28
        vec![ // 51 
            ARG(0),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 52 
            PTR(19, false, false),
            ARG(2),
        ], 
        // AExp29
        vec![ // 53 
            ARG(4),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 54 
            ARG(3),
            ARG(1),
        ], 
        vec![ // 55 
            ARG(0),
            ARG(2),
            ARG(3),
        ], 
        // AExp30
        vec![ // 56 
            COM(4,29),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 57 
            ARG(2),
            ARG(0),
        ], 
        // AExp31
        vec![ // 58 
            ARG(3),
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp32
        vec![ // 59 
            PRM(LE,false),
            ARG(1),
            INT(0),
            PTR(0, true, true),
            COM(2,0),
        ], 
        vec![ // 60 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp33
        vec![ // 61 
            COM(4,29),
            ARG(2),
            PTR(0, true, true),
        ], 
        vec![ // 62 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp34
        vec![ // 63 
            PTR(23, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 64 
            PRM(SUB,false),
            ARG(0),
            INT(1),
        ], 
        // AExp35
        vec![ // 65 
            PRM(LE,false),
            ARG(1),
            ARG(2),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 66 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp36
        vec![ // 67 
            COM(4,29),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 68 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp37
        vec![ // 69 
            PTR(25, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 70 
            PRM(ADD,false),
            ARG(0),
            INT(1),
        ], 
    ],

}});