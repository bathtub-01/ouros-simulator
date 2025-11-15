use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*;

// Combinators in this file: 60
#[rustfmt::skip]
pub static ADJOXO: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            PTR(5, false, false),
            PTR(4, false, false),
            PTR(2, false, false),
        ], 
        vec![ // 1 
            COM(4,2),
            INT(5),
            COM(2,0),
        ], 
        vec![ // 2 
            COM(4,2),
            INT(2),
            PTR(1, false, false),
        ], 
        vec![ // 3 
            COM(4,2),
            INT(4),
            COM(2,0),
        ], 
        vec![ // 4 
            COM(4,2),
            INT(1),
            PTR(3, false, false),
        ], 
        // AExp1
        vec![ // 5 
            COM(5,3),
            PTR(7, false, false),
            COM(2,17),
            COM(2,19),
        ], 
        vec![ // 6 
            COM(3,12),
            COM(2,15),
        ], 
        vec![ // 7 
            COM(3,9),
            PTR(6, false, false),
        ], 
        // AExp2
        vec![ // 8 
            Y,
            PTR(9, false, false),
            INT(0),
        ], 
        vec![ // 9 
            COM(4,24),
            COM(3,26),
        ], 
        // AExp3
        vec![ // 10 
            COM(3,28),
            PTR(48, false, false),
            PTR(44, false, false),
        ], 
        vec![ // 11 
            COM(4,2),
            INT(7),
            COM(2,0),
        ], 
        vec![ // 12 
            COM(4,2),
            INT(5),
            PTR(11, false, false),
        ], 
        vec![ // 13 
            COM(4,2),
            INT(3),
            PTR(12, false, false),
        ], 
        vec![ // 14 
            COM(2,50),
            PTR(13, false, false),
        ], 
        vec![ // 15 
            COM(4,2),
            INT(9),
            COM(2,0),
        ], 
        vec![ // 16 
            COM(4,2),
            INT(5),
            PTR(15, false, false),
        ], 
        vec![ // 17 
            COM(4,2),
            INT(1),
            PTR(16, false, false),
        ], 
        vec![ // 18 
            COM(2,50),
            PTR(17, false, false),
        ], 
        vec![ // 19 
            COM(3,46),
            PTR(18, false, false),
            PTR(14, false, false),
        ], 
        vec![ // 20 
            COM(4,2),
            INT(9),
            COM(2,0),
        ], 
        vec![ // 21 
            COM(4,2),
            INT(6),
            PTR(20, false, false),
        ], 
        vec![ // 22 
            COM(4,2),
            INT(3),
            PTR(21, false, false),
        ], 
        vec![ // 23 
            COM(2,50),
            PTR(22, false, false),
        ], 
        vec![ // 24 
            COM(3,43),
            PTR(23, false, false),
            PTR(19, false, false),
        ], 
        vec![ // 25 
            COM(4,2),
            INT(8),
            COM(2,0),
        ], 
        vec![ // 26 
            COM(4,2),
            INT(5),
            PTR(25, false, false),
        ], 
        vec![ // 27 
            COM(4,2),
            INT(2),
            PTR(26, false, false),
        ], 
        vec![ // 28 
            COM(2,50),
            PTR(27, false, false),
        ], 
        vec![ // 29 
            COM(3,40),
            PTR(28, false, false),
            PTR(24, false, false),
        ], 
        vec![ // 30 
            COM(4,2),
            INT(7),
            COM(2,0),
        ], 
        vec![ // 31 
            COM(4,2),
            INT(4),
            PTR(30, false, false),
        ], 
        vec![ // 32 
            COM(4,2),
            INT(1),
            PTR(31, false, false),
        ], 
        vec![ // 33 
            COM(2,50),
            PTR(32, false, false),
        ], 
        vec![ // 34 
            COM(3,37),
            PTR(33, false, false),
            PTR(29, false, false),
        ], 
        vec![ // 35 
            COM(4,2),
            INT(9),
            COM(2,0),
        ], 
        vec![ // 36 
            COM(4,2),
            INT(8),
            PTR(35, false, false),
        ], 
        vec![ // 37 
            COM(4,2),
            INT(7),
            PTR(36, false, false),
        ], 
        vec![ // 38 
            COM(2,50),
            PTR(37, false, false),
        ], 
        vec![ // 39 
            COM(3,34),
            PTR(38, false, false),
            PTR(34, false, false),
        ], 
        vec![ // 40 
            COM(4,2),
            INT(6),
            COM(2,0),
        ], 
        vec![ // 41 
            COM(4,2),
            INT(5),
            PTR(40, false, false),
        ], 
        vec![ // 42 
            COM(4,2),
            INT(4),
            PTR(41, false, false),
        ], 
        vec![ // 43 
            COM(2,50),
            PTR(42, false, false),
        ], 
        vec![ // 44 
            COM(3,31),
            PTR(43, false, false),
            PTR(39, false, false),
        ], 
        vec![ // 45 
            COM(4,2),
            INT(3),
            COM(2,0),
        ], 
        vec![ // 46 
            COM(4,2),
            INT(2),
            PTR(45, false, false),
        ], 
        vec![ // 47 
            COM(4,2),
            INT(1),
            PTR(46, false, false),
        ], 
        vec![ // 48 
            COM(2,50),
            PTR(47, false, false),
        ], 
        // AExp4
        vec![ // 49 
            COM(3,69),
            COM(1,72),
        ], 
        // AExp5
        vec![ // 50 
            COM(3,76),
            PTR(55, false, false),
        ], 
        vec![ // 51 
            PTR(70, false, false),
            INT(1),
            INT(9),
        ], 
        vec![ // 52 
            PTR(66, false, false),
            PTR(51, false, false),
        ], 
        vec![ // 53 
            COM(3,82),
            COM(3,84),
            PTR(52, false, false),
        ], 
        vec![ // 54 
            COM(3,80),
            PTR(53, false, false),
        ], 
        vec![ // 55 
            COM(3,78),
            PTR(54, false, false),
        ], 
        // AExp6
        vec![ // 56 
            COM(3,87),
            COM(2,89),
        ], 
        // AExp7
        vec![ // 57 
            COM(2,92),
            PTR(59, false, false),
        ], 
        vec![ // 58 
            COM(5,96),
            COM(6,98),
        ], 
        vec![ // 59 
            COM(4,94),
            PTR(58, false, false),
        ], 
        // AExp8
        vec![ // 60 
            COM(2,102),
            PTR(61, false, false),
        ], 
        vec![ // 61 
            COM(4,104),
            COM(4,106),
        ], 
        // AExp9
        vec![ // 62 
            COM(4,109),
            COM(3,111),
        ], 
        // AExp10
        vec![ // 63 
            COM(2,114),
            PTR(65, false, false),
        ], 
        vec![ // 64 
            COM(6,119),
            COM(3,122),
            COM(3,124),
        ], 
        vec![ // 65 
            COM(4,116),
            PTR(64, false, false),
        ], 
        // AExp11
        vec![ // 66 
            COM(3,54),
            PTR(69, false, false),
        ], 
        vec![ // 67 
            COM(5,63),
            COM(3,65),
        ], 
        vec![ // 68 
            COM(6,59),
            PTR(67, false, false),
            COM(2,67),
        ], 
        vec![ // 69 
            COM(4,56),
            PTR(68, false, false),
        ], 
        // AExp12
        vec![ // 70 
            COM(3,126),
            PTR(71, false, false),
        ], 
        vec![ // 71 
            COM(3,128),
            COM(1,130),
        ], 
        // AExp13
        vec![ // 72 
            COM(2,52),
            COM(2,53),
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
            COM(2,21),
            PTR(4, true, true),
            PTR(3, true, true),
            PTR(2, true, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 4 
            ARG(2),
            ARG(3),
            ARG(4),
        ], 
        vec![ // 5 
            ARG(1),
            ARG(3),
            ARG(4),
        ], 
        vec![ // 6 
            ARG(0),
            ARG(3),
            ARG(4),
        ], 
        vec![ // 7 
            PTR(8, false, false),
            ARG(4),
        ], 
        vec![ // 8 
            PTR(8, false, false),
            ARG(3),
        ], 
        // AExp4
        vec![ // 9 
            PTR(10, false, false),
            ARG(2),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 10 
            PTR(49, false, false),
            COM(3,23),
            COM(2,1),
        ], 
        vec![ // 11 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp5
        vec![ // 12 
            PTR(10, false, false),
            ARG(1),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 13 
            PTR(49, false, false),
            COM(3,23),
            COM(2,0),
        ], 
        vec![ // 14 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp6
        vec![ // 15 
            PTR(49, false, false),
            PTR(0, true, true),
            COM(2,1),
        ], 
        vec![ // 16 
            PTR(50, false, false),
            ARG(1),
            ARG(0),
        ], 
        // AExp7
        vec![ // 17 
            PTR(49, false, false),
            PTR(0, true, true),
            COM(2,0),
        ], 
        vec![ // 18 
            PTR(50, false, false),
            ARG(0),
            ARG(1),
        ], 
        // AExp8
        vec![ // 19 
            PTR(49, false, false),
            PTR(0, true, true),
            COM(2,1),
        ], 
        vec![ // 20 
            PTR(50, false, false),
            ARG(1),
            ARG(0),
        ], 
        // AExp9
        vec![ // 21 
            PRM(EQ,false),
            ARG(0),
            ARG(1),
            PTR(0, true, true),
            COM(3,0),
        ], 
        vec![ // 22 
            PRM(LE,false),
            ARG(0),
            ARG(1),
            COM(3,23),
            COM(3,1),
        ], 
        // AExp10
        vec![ // 23 
            ARG(2),
        ], 
        // AExp11
        vec![ // 24 
            ARG(3),
            ARG(2),
            PTR(0, true, true),
        ], 
        vec![ // 25 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp12
        vec![ // 26 
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 27 
            PRM(ADD,false),
            ARG(1),
            INT(1),
        ], 
        // AExp13
        vec![ // 28 
            COM(2,49),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 29 
            ARG(1),
            ARG(2),
        ], 
        vec![ // 30 
            ARG(0),
            ARG(2),
        ], 
        // AExp14
        vec![ // 31 
            COM(2,49),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 32 
            ARG(1),
            ARG(2),
        ], 
        vec![ // 33 
            ARG(0),
            ARG(2),
        ], 
        // AExp15
        vec![ // 34 
            COM(2,49),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 35 
            ARG(1),
            ARG(2),
        ], 
        vec![ // 36 
            ARG(0),
            ARG(2),
        ], 
        // AExp16
        vec![ // 37 
            COM(2,49),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 38 
            ARG(1),
            ARG(2),
        ], 
        vec![ // 39 
            ARG(0),
            ARG(2),
        ], 
        // AExp17
        vec![ // 40 
            COM(2,49),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 41 
            ARG(1),
            ARG(2),
        ], 
        vec![ // 42 
            ARG(0),
            ARG(2),
        ], 
        // AExp18
        vec![ // 43 
            COM(2,49),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 44 
            ARG(1),
            ARG(2),
        ], 
        vec![ // 45 
            ARG(0),
            ARG(2),
        ], 
        // AExp19
        vec![ // 46 
            COM(2,49),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 47 
            ARG(1),
            ARG(2),
        ], 
        vec![ // 48 
            ARG(0),
            ARG(2),
        ], 
        // AExp20
        vec![ // 49 
            ARG(0),
            ARG(1),
            COM(2,1),
        ], 
        // AExp21
        vec![ // 50 
            PTR(72, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 51 
            PTR(66, false, false),
            ARG(0),
            ARG(1),
        ], 
        // AExp22
        vec![ // 52 
            ARG(1),
            COM(2,1),
            ARG(0),
        ], 
        // AExp23
        vec![ // 53 
            COM(2,0),
        ], 
        // AExp24
        vec![ // 54 
            ARG(1),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 55 
            ARG(0),
            ARG(2),
        ], 
        // AExp25
        vec![ // 56 
            ARG(1),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 57 
            ARG(0),
            ARG(2),
            ARG(3),
        ], 
        vec![ // 58 
            COM(4,2),
            ARG(2),
            ARG(3),
        ], 
        // AExp26
        vec![ // 59 
            COM(2,21),
            ARG(2),
            ARG(4),
            PTR(2, true, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 60 
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(5),
        ], 
        vec![ // 61 
            ARG(0),
            ARG(2),
            ARG(3),
            ARG(4),
            ARG(5),
        ], 
        vec![ // 62 
            PTR(66, false, false),
            ARG(3),
            ARG(5),
        ], 
        // AExp27
        vec![ // 63 
            COM(4,2),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 64 
            ARG(0),
            ARG(2),
            ARG(3),
            ARG(4),
        ], 
        // AExp28
        vec![ // 65 
            PTR(66, false, false),
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 66 
            COM(4,2),
            ARG(1),
            ARG(2),
        ], 
        // AExp29
        vec![ // 67 
            PTR(66, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 68 
            COM(4,2),
            ARG(0),
            ARG(1),
        ], 
        // AExp30
        vec![ // 69 
            ARG(1),
            INT(3),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 70 
            COM(1,74),
            ARG(2),
        ], 
        vec![ // 71 
            ARG(0),
            ARG(2),
        ], 
        // AExp31
        vec![ // 72 
            COM(1,74),
            PTR(0, true, true),
        ], 
        vec![ // 73 
            COM(1,75),
            ARG(0),
        ], 
        // AExp32
        vec![ // 74 
            ARG(0),
            INT(0),
            INT(88),
        ], 
        // AExp33
        vec![ // 75 
            ARG(0),
            COM(2,1),
            COM(2,0),
        ], 
        // AExp34
        vec![ // 76 
            PTR(10, false, false),
            ARG(2),
            PTR(0, true, true),
            COM(3,1),
        ], 
        vec![ // 77 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp35
        vec![ // 78 
            PTR(56, false, false),
            ARG(1),
            ARG(2),
            PTR(0, true, true),
            COM(3,0),
        ], 
        vec![ // 79 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp36
        vec![ // 80 
            PTR(57, false, false),
            COM(2,100),
            PTR(0, true, true),
        ], 
        vec![ // 81 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp37
        vec![ // 82 
            ARG(0),
            PTR(0, true, true),
            ARG(2),
        ], 
        vec![ // 83 
            ARG(1),
            ARG(2),
        ], 
        // AExp38
        vec![ // 84 
            PTR(60, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 85 
            PTR(66, false, false),
            ARG(0),
            ARG(2),
        ], 
        vec![ // 86 
            PTR(62, false, false),
            ARG(1),
            ARG(2),
        ], 
        // AExp39
        vec![ // 87 
            PRM(EQ,false),
            PTR(0, true, true),
            INT(9),
        ], 
        vec![ // 88 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp40
        vec![ // 89 
            PRM(ADD,false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 90 
            PTR(8, false, false),
            ARG(1),
        ], 
        vec![ // 91 
            PTR(8, false, false),
            ARG(0),
        ], 
        // AExp41
        vec![ // 92 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 93 
            ARG(0),
            ARG(1),
        ], 
        // AExp42
        vec![ // 94 
            ARG(3),
            ERR(0),
            PTR(0, true, true),
        ], 
        vec![ // 95 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp43
        vec![ // 96 
            ARG(4),
            ARG(3),
            PTR(0, true, true),
        ], 
        vec![ // 97 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(4),
        ], 
        // AExp44
        vec![ // 98 
            ARG(0),
            ARG(2),
            PTR(0, true, true),
        ], 
        vec![ // 99 
            ARG(1),
            ARG(3),
        ], 
        // AExp45
        vec![ // 100 
            ARG(0),
            PTR(0, true, true),
            ARG(1),
            COM(3,23),
        ], 
        vec![ // 101 
            ARG(1),
            COM(3,0),
            COM(3,0),
            COM(3,23),
        ], 
        // AExp46
        vec![ // 102 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 103 
            ARG(0),
            ARG(1),
        ], 
        // AExp47
        vec![ // 104 
            ARG(3),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 105 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp48
        vec![ // 106 
            COM(4,2),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 107 
            ARG(1),
            ARG(3),
        ], 
        vec![ // 108 
            ARG(0),
            ARG(2),
        ], 
        // AExp49
        vec![ // 109 
            COM(1,113),
            PTR(0, true, true),
        ], 
        vec![ // 110 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(3),
        ], 
        // AExp50
        vec![ // 111 
            PTR(50, false, false),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 112 
            PTR(63, false, false),
            ARG(2),
            ARG(0),
        ], 
        // AExp51
        vec![ // 113 
            ARG(0),
            COM(3,0),
            COM(3,23),
            COM(3,1),
        ], 
        // AExp52
        vec![ // 114 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 115 
            ARG(0),
            ARG(1),
        ], 
        // AExp53
        vec![ // 116 
            ARG(3),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 117 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        vec![ // 118 
            COM(4,2),
            ARG(1),
            COM(2,0),
        ], 
        // AExp54
        vec![ // 119 
            PRM(LE,false),
            ARG(2),
            ARG(4),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 120 
            ARG(1),
            ARG(2),
            ARG(4),
            ARG(5),
        ], 
        vec![ // 121 
            ARG(0),
            ARG(3),
            ARG(4),
            ARG(5),
        ], 
        // AExp55
        vec![ // 122 
            COM(4,2),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 123 
            ARG(0),
            ARG(2),
        ], 
        // AExp56
        vec![ // 124 
            COM(4,2),
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 125 
            COM(4,2),
            ARG(1),
            ARG(2),
        ], 
        // AExp57
        vec![ // 126 
            PRM(LE,false),
            ARG(1),
            ARG(2),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 127 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp58
        vec![ // 128 
            COM(4,2),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 129 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp59
        vec![ // 130 
            PTR(70, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 131 
            PRM(ADD,false),
            ARG(0),
            INT(1),
        ], 
    ],

}});
