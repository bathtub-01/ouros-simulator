use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 63
#[rustfmt::skip]
pub static TAUT: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            PTR(1, false, false),
            PTR(26, false, false),
            INT(0),
            INT(1),
        ], 
        // AExp1
        vec![ // 1 
            COM(2,2),
            COM(1,4),
        ], 
        // AExp2
        vec![ // 2 
            COM(2,7),
            COM(2,8),
        ], 
        // AExp3
        vec![ // 3 
            COM(2,10),
            PTR(4, false, false),
        ], 
        vec![ // 4 
            COM(4,12),
            COM(4,14),
        ], 
        // AExp4
        vec![ // 5 
            COM(2,19),
            PTR(6, false, false),
        ], 
        vec![ // 6 
            COM(6,25),
            COM(3,27),
            COM(1,0),
            COM(3,29),
            COM(2,31),
            COM(2,32),
        ], 
        // AExp5
        vec![ // 7 
            COM(3,46),
            PTR(8, false, false),
            COM(1,53),
        ], 
        vec![ // 8 
            COM(2,48),
            COM(1,51),
        ], 
        // AExp6
        vec![ // 9 
            COM(3,55),
            PTR(11, false, false),
        ], 
        vec![ // 10 
            COM(5,59),
            COM(3,62),
        ], 
        vec![ // 11 
            COM(4,57),
            PTR(10, false, false),
        ], 
        // AExp7
        vec![ // 12 
            COM(2,63),
            PTR(14, false, false),
        ], 
        vec![ // 13 
            COM(3,66),
            COM(1,69),
            COM(1,72),
        ], 
        vec![ // 14 
            COM(3,46),
            PTR(13, false, false),
            COM(1,75),
        ], 
        // AExp8
        vec![ // 15 
            COM(3,76),
            PTR(16, false, false),
        ], 
        vec![ // 16 
            COM(4,78),
            COM(3,80),
        ], 
        // AExp9
        vec![ // 17 
            Y,
            PTR(18, false, false),
            INT(0),
        ], 
        vec![ // 18 
            COM(4,82),
            COM(3,84),
        ], 
        // AExp10
        vec![ // 19 
            COM(2,86),
            PTR(21, false, false),
        ], 
        vec![ // 20 
            COM(3,89),
            COM(1,91),
        ], 
        vec![ // 21 
            COM(3,87),
            PTR(20, false, false),
        ], 
        // AExp11
        vec![ // 22 
            COM(2,93),
            PTR(24, false, false),
        ], 
        vec![ // 23 
            COM(5,97),
            COM(3,100),
        ], 
        vec![ // 24 
            COM(4,95),
            PTR(23, false, false),
        ], 
        // AExp12
        vec![ // 25 
            COM(5,102),
            COM(2,103),
            COM(1,106),
            COM(2,107),
            COM(1,110),
        ], 
        // AExp13
        vec![ // 26 
            COM(7,111),
            PTR(32, false, false),
            PTR(30, false, false),
        ], 
        vec![ // 27 
            PTR(3, false, false),
            COM(6,123),
            PTR(36, false, false),
        ], 
        vec![ // 28 
            PTR(33, false, false),
            COM(7,62),
            PTR(27, false, false),
        ], 
        vec![ // 29 
            COM(6,123),
            INT(42),
        ], 
        vec![ // 30 
            COM(7,111),
            PTR(29, false, false),
            PTR(28, false, false),
        ], 
        vec![ // 31 
            PTR(3, false, false),
            COM(1,120),
            PTR(36, false, false),
        ], 
        vec![ // 32 
            PTR(33, false, false),
            COM(7,62),
            PTR(31, false, false),
        ], 
        // AExp14
        vec![ // 33 
            COM(2,112),
            PTR(35, false, false),
        ], 
        vec![ // 34 
            COM(5,116),
            COM(6,118),
        ], 
        vec![ // 35 
            COM(4,114),
            PTR(34, false, false),
        ], 
        // AExp15
        vec![ // 36 
            COM(4,17),
            INT(0),
            PTR(41, false, false),
        ], 
        vec![ // 37 
            COM(4,17),
            INT(5),
            COM(2,0),
        ], 
        vec![ // 38 
            COM(4,17),
            INT(4),
            PTR(37, false, false),
        ], 
        vec![ // 39 
            COM(4,17),
            INT(3),
            PTR(38, false, false),
        ], 
        vec![ // 40 
            COM(4,17),
            INT(2),
            PTR(39, false, false),
        ], 
        vec![ // 41 
            COM(4,17),
            INT(1),
            PTR(40, false, false),
        ], 
        // AExp16
        vec![ // 42 
            COM(2,35),
            COM(1,0),
        ], 
        // AExp17
        vec![ // 43 
            COM(2,36),
            PTR(45, false, false),
        ], 
        vec![ // 44 
            COM(5,40),
            COM(5,42),
        ], 
        vec![ // 45 
            COM(4,38),
            PTR(44, false, false),
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
            PTR(2, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 3 
            ARG(0),
            ARG(1),
        ], 
        // AExp3
        vec![ // 4 
            PTR(3, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 5 
            PTR(7, false, false),
            ARG(0),
        ], 
        vec![ // 6 
            COM(3,18),
            PTR(5, false, false),
            ARG(0),
        ], 
        // AExp4
        vec![ // 7 
            ARG(1),
            COM(2,1),
            ARG(0),
        ], 
        // AExp5
        vec![ // 8 
            ARG(0),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 9 
            PTR(2, false, false),
            ARG(1),
        ], 
        // AExp6
        vec![ // 10 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 11 
            ARG(0),
            ARG(1),
        ], 
        // AExp7
        vec![ // 12 
            ARG(3),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 13 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp8
        vec![ // 14 
            COM(4,17),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 15 
            ARG(1),
            ARG(3),
        ], 
        vec![ // 16 
            ARG(0),
            ARG(2),
        ], 
        // AExp9
        vec![ // 17 
            ARG(3),
            ARG(0),
            ARG(1),
        ], 
        // AExp10
        vec![ // 18 
            ARG(0),
            ARG(2),
            ARG(1),
        ], 
        // AExp11
        vec![ // 19 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 20 
            ARG(0),
            ARG(1),
        ], 
        // AExp12
        vec![ // 21 
            ARG(6),
            PTR(2, true, true),
            ARG(1),
            PTR(1, true, true),
            PTR(0, true, true),
            ARG(4),
        ], 
        vec![ // 22 
            ARG(3),
            ARG(5),
        ], 
        vec![ // 23 
            ARG(2),
            ARG(5),
        ], 
        vec![ // 24 
            ARG(0),
            ARG(5),
        ], 
        // AExp13
        vec![ // 25 
            COM(7,21),
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(3),
            PTR(0, true, true),
        ], 
        vec![ // 26 
            ARG(4),
            ARG(5),
        ], 
        // AExp14
        vec![ // 27 
            ARG(0),
            ARG(1),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 28 
            ARG(0),
            ARG(2),
        ], 
        // AExp15
        vec![ // 29 
            ARG(0),
            ARG(1),
            COM(2,1),
            PTR(0, true, true),
        ], 
        vec![ // 30 
            ARG(0),
            ARG(2),
        ], 
        // AExp16
        vec![ // 31 
            ARG(0),
            ARG(1),
            COM(2,1),
            COM(2,0),
        ], 
        // AExp17
        vec![ // 32 
            COM(2,33),
            ARG(1),
            ARG(0),
        ], 
        // AExp18
        vec![ // 33 
            COM(1,0),
            PTR(42, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 34 
            PTR(43, false, false),
            ARG(0),
            ARG(1),
        ], 
        // AExp19
        vec![ // 35 
            ARG(1),
            ERR(4),
            ARG(0),
        ], 
        // AExp20
        vec![ // 36 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 37 
            ARG(0),
            ARG(1),
        ], 
        // AExp21
        vec![ // 38 
            ARG(3),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 39 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp22
        vec![ // 40 
            ARG(3),
            PTR(0, true, true),
        ], 
        vec![ // 41 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(4),
        ], 
        // AExp23
        vec![ // 42 
            PRM(EQ,false),
            ARG(0),
            ARG(3),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 43 
            COM(3,45),
            ARG(4),
        ], 
        vec![ // 44 
            ARG(1),
            ARG(2),
        ], 
        // AExp24
        vec![ // 45 
            ARG(2),
            ARG(0),
        ], 
        // AExp25
        vec![ // 46 
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 47 
            ARG(1),
            ARG(2),
        ], 
        // AExp26
        vec![ // 48 
            PTR(3, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 49 
            ARG(0),
            ARG(1),
        ], 
        vec![ // 50 
            PTR(9, false, false),
            ARG(1),
        ], 
        // AExp27
        vec![ // 51 
            PTR(12, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 52 
            PTR(17, false, false),
            ARG(0),
        ], 
        // AExp28
        vec![ // 53 
            PTR(19, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 54 
            PTR(25, false, false),
            ARG(0),
        ], 
        // AExp29
        vec![ // 55 
            ARG(1),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 56 
            ARG(0),
            ARG(2),
        ], 
        // AExp30
        vec![ // 57 
            ARG(1),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 58 
            ARG(0),
            ARG(2),
            ARG(3),
        ], 
        // AExp31
        vec![ // 59 
            COM(4,17),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 60 
            PTR(9, false, false),
            ARG(2),
            ARG(4),
        ], 
        vec![ // 61 
            ARG(0),
            ARG(1),
            ARG(3),
        ], 
        // AExp32
        vec![ // 62 
            ARG(2),
            ARG(0),
            ARG(1),
        ], 
        // AExp33
        vec![ // 63 
            PRM(EQ,false),
            ARG(1),
            INT(0),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 64 
            COM(4,17),
            COM(2,0),
            COM(2,0),
        ], 
        vec![ // 65 
            ARG(0),
            ARG(1),
        ], 
        // AExp34
        vec![ // 66 
            PTR(15, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 67 
            ARG(1),
            ARG(2),
        ], 
        vec![ // 68 
            ARG(0),
            ARG(2),
        ], 
        // AExp35
        vec![ // 69 
            PTR(3, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 70 
            PTR(12, false, false),
            ARG(0),
        ], 
        vec![ // 71 
            COM(4,17),
            COM(2,0),
        ], 
        // AExp36
        vec![ // 72 
            PTR(3, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 73 
            PTR(12, false, false),
            ARG(0),
        ], 
        vec![ // 74 
            COM(4,17),
            COM(2,1),
        ], 
        // AExp37
        vec![ // 75 
            PRM(SUB,false),
            ARG(0),
            INT(1),
        ], 
        // AExp38
        vec![ // 76 
            Y,
            PTR(0, true, true),
            ARG(1),
        ], 
        vec![ // 77 
            ARG(0),
            ARG(2),
        ], 
        // AExp39
        vec![ // 78 
            ARG(3),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 79 
            ARG(0),
            ARG(2),
        ], 
        // AExp40
        vec![ // 80 
            COM(4,17),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 81 
            ARG(0),
            ARG(2),
        ], 
        // AExp41
        vec![ // 82 
            ARG(3),
            ARG(2),
            PTR(0, true, true),
        ], 
        vec![ // 83 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp42
        vec![ // 84 
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 85 
            PRM(ADD,false),
            ARG(1),
            INT(1),
        ], 
        // AExp43
        vec![ // 86 
            ARG(1),
            COM(2,0),
            ARG(0),
        ], 
        // AExp44
        vec![ // 87 
            COM(4,17),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 88 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp45
        vec![ // 89 
            PTR(19, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 90 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp46
        vec![ // 91 
            PTR(22, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 92 
            PRM(EQ,true),
            ARG(0),
        ], 
        // AExp47
        vec![ // 93 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 94 
            ARG(0),
            ARG(1),
        ], 
        // AExp48
        vec![ // 95 
            ARG(3),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 96 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp49
        vec![ // 97 
            ARG(1),
            ARG(3),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 98 
            ARG(0),
            ARG(2),
            ARG(3),
            ARG(4),
        ], 
        vec![ // 99 
            ARG(2),
            ARG(4),
        ], 
        // AExp50
        vec![ // 100 
            COM(4,17),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 101 
            ARG(0),
            ARG(2),
        ], 
        // AExp51
        vec![ // 102 
            ARG(4),
            ARG(0),
            ARG(1),
            ARG(2),
            PTR(25, false, false),
            ARG(3),
        ], 
        // AExp52
        vec![ // 103 
            PTR(15, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 104 
            PTR(25, false, false),
            ARG(1),
        ], 
        vec![ // 105 
            PTR(25, false, false),
            ARG(0),
        ], 
        // AExp53
        vec![ // 106 
            COM(2,0),
        ], 
        // AExp54
        vec![ // 107 
            PTR(15, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 108 
            PTR(25, false, false),
            ARG(1),
        ], 
        vec![ // 109 
            PTR(25, false, false),
            ARG(0),
        ], 
        // AExp55
        vec![ // 110 
            COM(4,17),
            ARG(0),
            COM(2,0),
        ], 
        // AExp56
        vec![ // 111 
            ARG(4),
            ARG(0),
            ARG(1),
        ], 
        // AExp57
        vec![ // 112 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 113 
            ARG(0),
            ARG(1),
        ], 
        // AExp58
        vec![ // 114 
            ARG(3),
            ERR(0),
            PTR(0, true, true),
        ], 
        vec![ // 115 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp59
        vec![ // 116 
            ARG(4),
            ARG(3),
            PTR(0, true, true),
        ], 
        vec![ // 117 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(4),
        ], 
        // AExp60
        vec![ // 118 
            ARG(0),
            ARG(2),
            PTR(0, true, true),
        ], 
        vec![ // 119 
            ARG(1),
            ARG(3),
        ], 
        // AExp61
        vec![ // 120 
            COM(7,111),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 121 
            COM(6,123),
            ARG(0),
        ], 
        vec![ // 122 
            COM(6,123),
            INT(42),
        ], 
        // AExp62
        vec![ // 123 
            ARG(5),
            ARG(0),
        ], 
    ],

}});