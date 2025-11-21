use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 102
#[rustfmt::skip]
pub static SUMPUZ: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            COM(1,2),
            PTR(9, false, false),
        ], 
        vec![ // 1 
            COM(4,3),
            INT(2),
            COM(2,0),
        ], 
        vec![ // 2 
            COM(4,3),
            INT(1),
            PTR(1, false, false),
        ], 
        vec![ // 3 
            COM(4,3),
            INT(2),
            PTR(2, false, false),
        ], 
        vec![ // 4 
            COM(4,3),
            PTR(3, false, false),
            COM(2,0),
        ], 
        vec![ // 5 
            COM(4,3),
            INT(1),
            COM(2,0),
        ], 
        vec![ // 6 
            COM(4,3),
            INT(2),
            PTR(5, false, false),
        ], 
        vec![ // 7 
            COM(4,3),
            INT(1),
            PTR(6, false, false),
        ], 
        vec![ // 8 
            COM(4,3),
            INT(0),
            PTR(7, false, false),
        ], 
        vec![ // 9 
            COM(4,3),
            PTR(8, false, false),
            PTR(4, false, false),
        ], 
        // AExp1
        vec![ // 10 
            COM(5,20),
            COM(2,23),
            PTR(12, false, false),
        ], 
        vec![ // 11 
            COM(4,32),
            COM(1,34),
        ], 
        vec![ // 12 
            COM(5,26),
            COM(2,29),
            PTR(11, false, false),
        ], 
        // AExp2
        vec![ // 13 
            Y,
            PTR(14, false, false),
            INT(0),
        ], 
        vec![ // 14 
            COM(4,36),
            COM(3,38),
        ], 
        // AExp3
        vec![ // 15 
            COM(2,40),
            PTR(16, false, false),
        ], 
        vec![ // 16 
            COM(3,41),
            COM(2,42),
        ], 
        // AExp4
        vec![ // 17 
            COM(6,43),
            PTR(25, false, false),
            PTR(21, false, false),
        ], 
        vec![ // 18 
            COM(4,89),
            COM(1,92),
        ], 
        vec![ // 19 
            COM(3,82),
            COM(2,84),
            COM(1,87),
        ], 
        vec![ // 20 
            COM(4,77),
            COM(2,80),
            PTR(19, false, false),
            PTR(18, false, false),
        ], 
        vec![ // 21 
            COM(6,63),
            PTR(20, false, false),
        ], 
        vec![ // 22 
            COM(3,56),
            COM(2,59),
        ], 
        vec![ // 23 
            COM(5,54),
            PTR(22, false, false),
            COM(2,62),
        ], 
        vec![ // 24 
            COM(2,49),
            COM(1,52),
        ], 
        vec![ // 25 
            COM(4,46),
            PTR(24, false, false),
            PTR(23, false, false),
        ], 
        // AExp5
        vec![ // 26 
            COM(2,94),
            COM(2,0),
        ], 
        // AExp6
        vec![ // 27 
            COM(2,94),
            COM(2,1),
        ], 
        // AExp7
        vec![ // 28 
            COM(5,95),
            PTR(30, false, false),
            COM(3,106),
        ], 
        vec![ // 29 
            COM(4,101),
            COM(2,104),
        ], 
        vec![ // 30 
            COM(4,98),
            PTR(29, false, false),
        ], 
        // AExp8
        vec![ // 31 
            COM(2,108),
            PTR(33, false, false),
        ], 
        vec![ // 32 
            COM(5,112),
            COM(5,114),
        ], 
        vec![ // 33 
            COM(4,110),
            PTR(32, false, false),
        ], 
        // AExp9
        vec![ // 34 
            COM(2,118),
            PTR(35, false, false),
        ], 
        vec![ // 35 
            COM(4,120),
            COM(4,122),
        ], 
        // AExp10
        vec![ // 36 
            COM(3,126),
            PTR(38, false, false),
        ], 
        vec![ // 37 
            COM(5,130),
            COM(3,133),
        ], 
        vec![ // 38 
            COM(4,128),
            PTR(37, false, false),
        ], 
        // AExp11
        vec![ // 39 
            PTR(41, false, false),
            PTR(40, false, false),
        ], 
        vec![ // 40 
            COM(3,125),
            PTR(43, false, false),
        ], 
        // AExp12
        vec![ // 41 
            COM(2,136),
            PTR(42, false, false),
        ], 
        vec![ // 42 
            COM(5,138),
            COM(4,140),
        ], 
        // AExp13
        vec![ // 43 
            COM(2,142),
            PTR(45, false, false),
        ], 
        vec![ // 44 
            COM(5,146),
            COM(3,148),
        ], 
        vec![ // 45 
            COM(4,144),
            PTR(44, false, false),
        ], 
        // AExp14
        vec![ // 46 
            PTR(34, false, false),
            PTR(27, false, false),
        ], 
        // AExp15
        vec![ // 47 
            COM(2,150),
            PTR(48, false, false),
        ], 
        vec![ // 48 
            COM(4,152),
            COM(4,154),
        ], 
        // AExp16
        vec![ // 49 
            COM(2,156),
            PTR(50, false, false),
        ], 
        vec![ // 50 
            COM(4,158),
            COM(4,160),
        ], 
        // AExp17
        vec![ // 51 
            COM(3,163),
            PTR(52, false, false),
        ], 
        vec![ // 52 
            COM(4,165),
            COM(3,167),
        ], 
        // AExp18
        vec![ // 53 
            COM(3,171),
            PTR(57, false, false),
            PTR(55, false, false),
        ], 
        vec![ // 54 
            COM(5,184),
            COM(3,186),
        ], 
        vec![ // 55 
            COM(5,182),
            PTR(54, false, false),
        ], 
        vec![ // 56 
            COM(4,178),
            COM(1,180),
        ], 
        vec![ // 57 
            COM(6,173),
            COM(2,176),
            PTR(56, false, false),
        ], 
        // AExp19
        vec![ // 58 
            COM(4,140),
            COM(3,133),
        ], 
        // AExp20
        vec![ // 59 
            COM(3,189),
            PTR(60, false, false),
            COM(2,196),
        ], 
        vec![ // 60 
            COM(2,192),
            COM(3,194),
        ], 
        // AExp21
        vec![ // 61 
            COM(2,200),
            COM(2,0),
        ], 
        // AExp22
        vec![ // 62 
            COM(2,201),
            COM(2,1),
        ], 
        // AExp23
        vec![ // 63 
            COM(3,202),
            PTR(64, false, false),
        ], 
        vec![ // 64 
            COM(3,204),
            COM(1,206),
        ], 
        // AExp24
        vec![ // 65 
            COM(4,208),
            COM(3,0),
        ], 
        // AExp25
        vec![ // 66 
            COM(2,199),
            COM(1,0),
        ], 
        // AExp26
        vec![ // 67 
            COM(2,7),
            PTR(69, false, false),
        ], 
        vec![ // 68 
            COM(6,11),
            COM(3,13),
        ], 
        vec![ // 69 
            COM(5,9),
            PTR(68, false, false),
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
            COM(3,4),
            ARG(0),
            ARG(0),
            ARG(0),
        ], 
        // AExp3
        vec![ // 3 
            ARG(3),
            ARG(0),
            ARG(1),
        ], 
        // AExp4
        vec![ // 4 
            COM(2,6),
            PTR(0, true, true),
            ARG(0),
        ], 
        vec![ // 5 
            COM(3,15),
            ARG(1),
            ARG(2),
        ], 
        // AExp5
        vec![ // 6 
            PTR(67, false, false),
            ARG(0),
            ARG(1),
            INT(0),
        ], 
        // AExp6
        vec![ // 7 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 8 
            ARG(0),
            ARG(1),
        ], 
        // AExp7
        vec![ // 9 
            ARG(3),
            ARG(4),
            PTR(0, true, true),
        ], 
        vec![ // 10 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(4),
        ], 
        // AExp8
        vec![ // 11 
            ARG(2),
            ARG(5),
            PTR(0, true, true),
        ], 
        vec![ // 12 
            ARG(0),
            ARG(1),
            ARG(3),
            ARG(4),
        ], 
        // AExp9
        vec![ // 13 
            PRM(ADD,false),
            PTR(0, true, true),
            ARG(1),
        ], 
        vec![ // 14 
            ARG(0),
            ARG(2),
        ], 
        // AExp10
        vec![ // 15 
            COM(2,6),
            PTR(0, true, true),
            ARG(0),
        ], 
        vec![ // 16 
            COM(3,17),
            ARG(2),
            ARG(1),
        ], 
        // AExp11
        vec![ // 17 
            COM(2,6),
            PTR(0, true, true),
            ARG(1),
        ], 
        vec![ // 18 
            COM(3,19),
            ARG(0),
            ARG(2),
        ], 
        // AExp12
        vec![ // 19 
            PTR(10, false, false),
            ARG(0),
            ARG(1),
            ARG(2),
            INT(0),
            INT(1),
        ], 
        // AExp13
        vec![ // 20 
            COM(1,35),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 21 
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(4),
        ], 
        vec![ // 22 
            ARG(0),
            ARG(2),
            ARG(3),
        ], 
        // AExp14
        vec![ // 23 
            PRM(EQ,false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 24 
            PTR(13, false, false),
            ARG(1),
        ], 
        vec![ // 25 
            PTR(13, false, false),
            ARG(0),
        ], 
        // AExp15
        vec![ // 26 
            COM(1,35),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 27 
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(4),
        ], 
        vec![ // 28 
            ARG(0),
            ARG(2),
            ARG(4),
        ], 
        // AExp16
        vec![ // 29 
            PRM(EQ,false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 30 
            PTR(13, false, false),
            ARG(1),
        ], 
        vec![ // 31 
            PTR(13, false, false),
            ARG(0),
        ], 
        // AExp17
        vec![ // 32 
            COM(1,0),
            PTR(15, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 33 
            PTR(17, false, false),
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(0),
        ], 
        // AExp18
        vec![ // 34 
            ARG(0),
            INT(0),
            COM(2,0),
        ], 
        // AExp19
        vec![ // 35 
            ARG(0),
            COM(2,0),
        ], 
        // AExp20
        vec![ // 36 
            ARG(3),
            ARG(2),
            PTR(0, true, true),
        ], 
        vec![ // 37 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp21
        vec![ // 38 
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 39 
            PRM(ADD,false),
            ARG(1),
            INT(1),
        ], 
        // AExp22
        vec![ // 40 
            ARG(1),
            COM(2,0),
            ARG(0),
        ], 
        // AExp23
        vec![ // 41 
            ARG(2),
            COM(2,1),
            ARG(0),
        ], 
        // AExp24
        vec![ // 42 
            COM(2,0),
        ], 
        // AExp25
        vec![ // 43 
            ARG(2),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 44 
            ARG(1),
            ARG(3),
            ARG(4),
            ARG(5),
        ], 
        vec![ // 45 
            ARG(0),
            ARG(4),
            ARG(5),
        ], 
        // AExp26
        vec![ // 46 
            ARG(2),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 47 
            ARG(1),
            ARG(3),
        ], 
        vec![ // 48 
            ARG(0),
            ARG(3),
        ], 
        // AExp27
        vec![ // 49 
            PRM(EQ,false),
            PTR(1, true, true),
            INT(0),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 50 
            ARG(0),
            ARG(1),
        ], 
        vec![ // 51 
            PTR(26, false, false),
            ARG(1),
        ], 
        // AExp28
        vec![ // 52 
            COM(4,3),
            PTR(0, true, true),
            COM(2,0),
        ], 
        vec![ // 53 
            PTR(27, false, false),
            ARG(0),
        ], 
        // AExp29
        vec![ // 54 
            ARG(4),
            PTR(0, true, true),
            ARG(1),
        ], 
        vec![ // 55 
            ARG(0),
            ARG(2),
            ARG(3),
        ], 
        // AExp30
        vec![ // 56 
            PRM(EQ,false),
            PTR(1, true, true),
            INT(1),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 57 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        vec![ // 58 
            PTR(26, false, false),
            ARG(1),
        ], 
        // AExp31
        vec![ // 59 
            PTR(28, false, false),
            ARG(1),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 60 
            PTR(27, false, false),
            ARG(0),
        ], 
        vec![ // 61 
            COM(4,3),
            INT(1),
            COM(2,0),
        ], 
        // AExp32
        vec![ // 62 
            COM(2,0),
        ], 
        // AExp33
        vec![ // 63 
            ARG(2),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 64 
            ARG(0),
            ARG(1),
            ARG(3),
            ARG(4),
            ARG(5),
        ], 
        // AExp34
        vec![ // 65 
            COM(1,0),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 66 
            PTR(49, false, false),
            ARG(1),
            ARG(4),
        ], 
        vec![ // 67 
            ARG(0),
            ARG(2),
            ARG(3),
            ARG(5),
            ARG(6),
        ], 
        // AExp35
        vec![ // 68 
            PTR(53, false, false),
            PTR(2, true, true),
            ARG(3),
            PTR(1, true, true),
            ARG(5),
            PTR(0, true, true),
        ], 
        vec![ // 69 
            ARG(2),
            ARG(4),
            ARG(6),
        ], 
        vec![ // 70 
            PTR(61, false, false),
            ARG(1),
        ], 
        vec![ // 71 
            PTR(26, false, false),
            ARG(0),
        ], 
        // AExp36
        vec![ // 72 
            PTR(49, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 73 
            COM(7,68),
            ARG(2),
            ARG(0),
            ARG(1),
            ARG(3),
            ARG(4),
            ARG(5),
            ARG(6),
        ], 
        // AExp37
        vec![ // 74 
            COM(7,65),
            PTR(1, true, true),
            ARG(1),
            ARG(5),
            ARG(6),
            PTR(0, true, true),
        ], 
        vec![ // 75 
            ARG(2),
            ARG(4),
            ARG(5),
            ARG(6),
        ], 
        vec![ // 76 
            COM(7,72),
            ARG(3),
            ARG(0),
            ARG(4),
        ], 
        // AExp38
        vec![ // 77 
            COM(7,74),
            PTR(1, true, true),
            PTR(0, true, true),
            ARG(2),
            ARG(3),
        ], 
        vec![ // 78 
            ARG(1),
            ARG(3),
        ], 
        vec![ // 79 
            ARG(0),
            ARG(3),
        ], 
        // AExp39
        vec![ // 80 
            PTR(17, false, false),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 81 
            PTR(62, false, false),
            ARG(0),
        ], 
        // AExp40
        vec![ // 82 
            ARG(0),
            PTR(0, true, true),
            ARG(2),
        ], 
        vec![ // 83 
            ARG(1),
            ARG(2),
        ], 
        // AExp41
        vec![ // 84 
            PTR(28, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 85 
            PTR(63, false, false),
            ARG(0),
            INT(9),
        ], 
        vec![ // 86 
            PTR(61, false, false),
            ARG(1),
        ], 
        // AExp42
        vec![ // 87 
            PTR(65, false, false),
            PTR(0, true, true),
            INT(1),
            INT(0),
        ], 
        vec![ // 88 
            PTR(62, false, false),
            ARG(0),
        ], 
        // AExp43
        vec![ // 89 
            PTR(28, false, false),
            ARG(2),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 90 
            PTR(27, false, false),
            ARG(1),
        ], 
        vec![ // 91 
            ARG(0),
            ARG(3),
        ], 
        // AExp44
        vec![ // 92 
            PTR(63, false, false),
            PTR(0, true, true),
            INT(9),
        ], 
        vec![ // 93 
            PTR(65, false, false),
            ARG(0),
            INT(1),
            INT(0),
        ], 
        // AExp45
        vec![ // 94 
            ARG(1),
            ARG(0),
        ], 
        // AExp46
        vec![ // 95 
            PTR(31, false, false),
            ARG(2),
            ARG(4),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 96 
            ARG(1),
            ARG(3),
            ARG(4),
        ], 
        vec![ // 97 
            ARG(0),
            ARG(2),
            ARG(3),
            ARG(4),
        ], 
        // AExp47
        vec![ // 98 
            PTR(34, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 99 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(3),
        ], 
        vec![ // 100 
            COM(3,125),
            COM(4,3),
            ARG(3),
        ], 
        // AExp48
        vec![ // 101 
            PTR(36, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 102 
            ARG(0),
            ARG(2),
            ARG(3),
        ], 
        vec![ // 103 
            COM(1,134),
            ARG(1),
        ], 
        // AExp49
        vec![ // 104 
            PTR(39, false, false),
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 105 
            PTR(46, false, false),
            ARG(1),
        ], 
        // AExp50
        vec![ // 106 
            PTR(47, false, false),
            ARG(2),
            ARG(0),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 107 
            COM(4,3),
            ARG(1),
            COM(2,0),
        ], 
        // AExp51
        vec![ // 108 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 109 
            ARG(0),
            ARG(1),
        ], 
        // AExp52
        vec![ // 110 
            ARG(3),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 111 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp53
        vec![ // 112 
            ARG(3),
            PTR(0, true, true),
        ], 
        vec![ // 113 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(4),
        ], 
        // AExp54
        vec![ // 114 
            PRM(EQ,false),
            ARG(0),
            ARG(3),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 115 
            COM(3,117),
            ARG(4),
        ], 
        vec![ // 116 
            ARG(1),
            ARG(2),
        ], 
        // AExp55
        vec![ // 117 
            ARG(2),
            ARG(0),
        ], 
        // AExp56
        vec![ // 118 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 119 
            ARG(0),
            ARG(1),
        ], 
        // AExp57
        vec![ // 120 
            ARG(3),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 121 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp58
        vec![ // 122 
            COM(4,3),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 123 
            ARG(1),
            ARG(3),
        ], 
        vec![ // 124 
            ARG(0),
            ARG(2),
        ], 
        // AExp59
        vec![ // 125 
            ARG(0),
            ARG(2),
            ARG(1),
        ], 
        // AExp60
        vec![ // 126 
            ARG(1),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 127 
            ARG(0),
            ARG(2),
        ], 
        // AExp61
        vec![ // 128 
            ARG(1),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 129 
            ARG(0),
            ARG(2),
            ARG(3),
        ], 
        // AExp62
        vec![ // 130 
            COM(4,3),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 131 
            PTR(36, false, false),
            ARG(2),
            ARG(4),
        ], 
        vec![ // 132 
            ARG(0),
            ARG(1),
            ARG(3),
        ], 
        // AExp63
        vec![ // 133 
            ARG(2),
            ARG(0),
            ARG(1),
        ], 
        // AExp64
        vec![ // 134 
            COM(4,3),
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 135 
            COM(1,134),
            ARG(0),
        ], 
        // AExp65
        vec![ // 136 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 137 
            ARG(0),
            ARG(1),
        ], 
        // AExp66
        vec![ // 138 
            ARG(4),
            ARG(3),
            PTR(0, true, true),
        ], 
        vec![ // 139 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(3),
        ], 
        // AExp67
        vec![ // 140 
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 141 
            ARG(0),
            ARG(2),
            ARG(3),
        ], 
        // AExp68
        vec![ // 142 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 143 
            ARG(0),
            ARG(1),
        ], 
        // AExp69
        vec![ // 144 
            ARG(3),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 145 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp70
        vec![ // 146 
            PRM(EQ,false),
            ARG(1),
            ARG(3),
            PTR(0, true, true),
            ARG(4),
        ], 
        vec![ // 147 
            ARG(0),
            ARG(2),
            ARG(3),
            ARG(4),
        ], 
        // AExp71
        vec![ // 148 
            COM(4,3),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 149 
            ARG(0),
            ARG(2),
        ], 
        // AExp72
        vec![ // 150 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 151 
            ARG(0),
            ARG(1),
        ], 
        // AExp73
        vec![ // 152 
            ARG(3),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 153 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp74
        vec![ // 154 
            PRM(EQ,false),
            ARG(0),
            ARG(2),
            PTR(0, true, true),
            COM(2,1),
        ], 
        vec![ // 155 
            ARG(1),
            ARG(3),
        ], 
        // AExp75
        vec![ // 156 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 157 
            ARG(0),
            ARG(1),
        ], 
        // AExp76
        vec![ // 158 
            ARG(3),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 159 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp77
        vec![ // 160 
            PTR(51, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 161 
            ARG(1),
            ARG(3),
        ], 
        vec![ // 162 
            ARG(0),
            ARG(2),
        ], 
        // AExp78
        vec![ // 163 
            Y,
            PTR(0, true, true),
            ARG(1),
        ], 
        vec![ // 164 
            ARG(0),
            ARG(2),
        ], 
        // AExp79
        vec![ // 165 
            ARG(3),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 166 
            ARG(0),
            ARG(2),
        ], 
        // AExp80
        vec![ // 167 
            COM(4,3),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 168 
            ARG(0),
            ARG(2),
        ], 
        // AExp81
        vec![ // 169 
            ARG(0),
            ARG(4),
            ARG(5),
            ARG(6),
            PTR(0, true, true),
        ], 
        vec![ // 170 
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(6),
        ], 
        // AExp82
        vec![ // 171 
            COM(7,169),
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 172 
            ARG(1),
            ARG(2),
        ], 
        // AExp83
        vec![ // 173 
            PTR(49, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 174 
            ARG(1),
            ARG(2),
            ARG(4),
            ARG(5),
        ], 
        vec![ // 175 
            ARG(0),
            ARG(3),
            ARG(5),
        ], 
        // AExp84
        vec![ // 176 
            PTR(58, false, false),
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 177 
            PTR(26, false, false),
            ARG(1),
        ], 
        // AExp85
        vec![ // 178 
            PTR(28, false, false),
            ARG(1),
            PTR(0, true, true),
            ARG(2),
        ], 
        vec![ // 179 
            ARG(0),
            ARG(3),
        ], 
        // AExp86
        vec![ // 180 
            COM(4,3),
            PTR(0, true, true),
            COM(2,0),
        ], 
        vec![ // 181 
            PTR(27, false, false),
            ARG(0),
        ], 
        // AExp87
        vec![ // 182 
            PTR(59, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 183 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(4),
        ], 
        // AExp88
        vec![ // 184 
            PRM(ADD,false),
            PTR(0, true, true),
            ARG(1),
        ], 
        vec![ // 185 
            ARG(0),
            ARG(2),
            ARG(3),
            ARG(4),
        ], 
        // AExp89
        vec![ // 186 
            PRM(ADD,false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 187 
            COM(2,197),
            ARG(2),
            ARG(1),
        ], 
        vec![ // 188 
            COM(2,197),
            ARG(2),
            ARG(0),
        ], 
        // AExp90
        vec![ // 189 
            PRM(LE,false),
            ARG(2),
            INT(9),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 190 
            ARG(1),
            ARG(2),
        ], 
        vec![ // 191 
            ARG(0),
            ARG(2),
        ], 
        // AExp91
        vec![ // 192 
            PTR(59, false, false),
            PTR(0, true, true),
            ARG(0),
        ], 
        vec![ // 193 
            PRM(SUB,false),
            ARG(1),
            INT(10),
        ], 
        // AExp92
        vec![ // 194 
            ARG(2),
            PTR(0, true, true),
            ARG(1),
        ], 
        vec![ // 195 
            PRM(ADD,false),
            ARG(0),
            INT(1),
        ], 
        // AExp93
        vec![ // 196 
            ARG(1),
            INT(0),
            ARG(0),
        ], 
        // AExp94
        vec![ // 197 
            PTR(66, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 198 
            PTR(31, false, false),
            ARG(1),
            ARG(0),
        ], 
        // AExp95
        vec![ // 199 
            ARG(1),
            ERR(4),
            ARG(0),
        ], 
        // AExp96
        vec![ // 200 
            ARG(1),
            ERR(3),
            ARG(0),
        ], 
        // AExp97
        vec![ // 201 
            ARG(1),
            COM(2,0),
            ARG(0),
        ], 
        // AExp98
        vec![ // 202 
            PRM(LE,false),
            ARG(1),
            ARG(2),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 203 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp99
        vec![ // 204 
            COM(4,3),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 205 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp100
        vec![ // 206 
            PTR(63, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 207 
            PRM(ADD,false),
            ARG(0),
            INT(1),
        ], 
        // AExp101
        vec![ // 208 
            ARG(1),
            ARG(2),
            PTR(0, true, true),
        ], 
        vec![ // 209 
            ARG(0),
            ARG(3),
        ], 
    ],

}});