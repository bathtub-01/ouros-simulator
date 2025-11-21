use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 68
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
            PTR(40, false, false),
            PTR(37, false, false),
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
            COM(1,63),
            PTR(12, false, false),
        ], 
        vec![ // 14 
            COM(4,2),
            INT(9),
            COM(2,0),
        ], 
        vec![ // 15 
            COM(4,2),
            INT(5),
            PTR(14, false, false),
        ], 
        vec![ // 16 
            COM(1,61),
            PTR(15, false, false),
        ], 
        vec![ // 17 
            COM(3,58),
            PTR(16, false, false),
            PTR(13, false, false),
        ], 
        vec![ // 18 
            COM(4,2),
            INT(9),
            COM(2,0),
        ], 
        vec![ // 19 
            COM(4,2),
            INT(6),
            PTR(18, false, false),
        ], 
        vec![ // 20 
            COM(1,56),
            PTR(19, false, false),
        ], 
        vec![ // 21 
            COM(3,53),
            PTR(20, false, false),
            PTR(17, false, false),
        ], 
        vec![ // 22 
            COM(4,2),
            INT(8),
            COM(2,0),
        ], 
        vec![ // 23 
            COM(4,2),
            INT(5),
            PTR(22, false, false),
        ], 
        vec![ // 24 
            COM(1,51),
            PTR(23, false, false),
        ], 
        vec![ // 25 
            COM(3,48),
            PTR(24, false, false),
            PTR(21, false, false),
        ], 
        vec![ // 26 
            COM(4,2),
            INT(7),
            COM(2,0),
        ], 
        vec![ // 27 
            COM(4,2),
            INT(4),
            PTR(26, false, false),
        ], 
        vec![ // 28 
            COM(1,46),
            PTR(27, false, false),
        ], 
        vec![ // 29 
            COM(3,43),
            PTR(28, false, false),
            PTR(25, false, false),
        ], 
        vec![ // 30 
            COM(4,2),
            INT(9),
            COM(2,0),
        ], 
        vec![ // 31 
            COM(4,2),
            INT(8),
            PTR(30, false, false),
        ], 
        vec![ // 32 
            COM(1,41),
            PTR(31, false, false),
        ], 
        vec![ // 33 
            COM(3,38),
            PTR(32, false, false),
            PTR(29, false, false),
        ], 
        vec![ // 34 
            COM(4,2),
            INT(6),
            COM(2,0),
        ], 
        vec![ // 35 
            COM(4,2),
            INT(5),
            PTR(34, false, false),
        ], 
        vec![ // 36 
            COM(1,36),
            PTR(35, false, false),
        ], 
        vec![ // 37 
            COM(3,33),
            PTR(36, false, false),
            PTR(33, false, false),
        ], 
        vec![ // 38 
            COM(4,2),
            INT(3),
            COM(2,0),
        ], 
        vec![ // 39 
            COM(4,2),
            INT(2),
            PTR(38, false, false),
        ], 
        vec![ // 40 
            COM(1,31),
            PTR(39, false, false),
        ], 
        // AExp4
        vec![ // 41 
            COM(3,85),
            COM(1,88),
        ], 
        // AExp5
        vec![ // 42 
            COM(3,92),
            PTR(47, false, false),
        ], 
        vec![ // 43 
            PTR(62, false, false),
            INT(1),
            INT(9),
        ], 
        vec![ // 44 
            PTR(58, false, false),
            PTR(43, false, false),
        ], 
        vec![ // 45 
            COM(3,98),
            COM(3,100),
            PTR(44, false, false),
        ], 
        vec![ // 46 
            COM(3,96),
            PTR(45, false, false),
        ], 
        vec![ // 47 
            COM(3,94),
            PTR(46, false, false),
        ], 
        // AExp6
        vec![ // 48 
            COM(3,103),
            COM(2,105),
        ], 
        // AExp7
        vec![ // 49 
            COM(2,108),
            PTR(51, false, false),
        ], 
        vec![ // 50 
            COM(5,112),
            COM(6,114),
        ], 
        vec![ // 51 
            COM(4,110),
            PTR(50, false, false),
        ], 
        // AExp8
        vec![ // 52 
            COM(2,118),
            PTR(53, false, false),
        ], 
        vec![ // 53 
            COM(4,120),
            COM(4,122),
        ], 
        // AExp9
        vec![ // 54 
            COM(4,125),
            COM(3,127),
        ], 
        // AExp10
        vec![ // 55 
            COM(2,130),
            PTR(57, false, false),
        ], 
        vec![ // 56 
            COM(6,135),
            COM(3,138),
            COM(3,140),
        ], 
        vec![ // 57 
            COM(4,132),
            PTR(56, false, false),
        ], 
        // AExp11
        vec![ // 58 
            COM(3,70),
            PTR(61, false, false),
        ], 
        vec![ // 59 
            COM(5,79),
            COM(3,81),
        ], 
        vec![ // 60 
            COM(6,75),
            PTR(59, false, false),
            COM(2,83),
        ], 
        vec![ // 61 
            COM(4,72),
            PTR(60, false, false),
        ], 
        // AExp12
        vec![ // 62 
            COM(3,142),
            PTR(63, false, false),
        ], 
        vec![ // 63 
            COM(3,144),
            COM(1,146),
        ], 
        // AExp13
        vec![ // 64 
            COM(2,68),
            COM(2,69),
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
            PTR(41, false, false),
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
            PTR(41, false, false),
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
            PTR(41, false, false),
            PTR(0, true, true),
            COM(2,1),
        ], 
        vec![ // 16 
            PTR(42, false, false),
            ARG(1),
            ARG(0),
        ], 
        // AExp7
        vec![ // 17 
            PTR(41, false, false),
            PTR(0, true, true),
            COM(2,0),
        ], 
        vec![ // 18 
            PTR(42, false, false),
            ARG(0),
            ARG(1),
        ], 
        // AExp8
        vec![ // 19 
            PTR(41, false, false),
            PTR(0, true, true),
            COM(2,1),
        ], 
        vec![ // 20 
            PTR(42, false, false),
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
            COM(2,65),
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
            COM(2,66),
            PTR(0, true, true),
        ], 
        vec![ // 32 
            COM(4,2),
            INT(1),
            ARG(0),
        ], 
        // AExp15
        vec![ // 33 
            COM(2,65),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 34 
            ARG(1),
            ARG(2),
        ], 
        vec![ // 35 
            ARG(0),
            ARG(2),
        ], 
        // AExp16
        vec![ // 36 
            COM(2,66),
            PTR(0, true, true),
        ], 
        vec![ // 37 
            COM(4,2),
            INT(4),
            ARG(0),
        ], 
        // AExp17
        vec![ // 38 
            COM(2,65),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 39 
            ARG(1),
            ARG(2),
        ], 
        vec![ // 40 
            ARG(0),
            ARG(2),
        ], 
        // AExp18
        vec![ // 41 
            COM(2,66),
            PTR(0, true, true),
        ], 
        vec![ // 42 
            COM(4,2),
            INT(7),
            ARG(0),
        ], 
        // AExp19
        vec![ // 43 
            COM(2,65),
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
        // AExp20
        vec![ // 46 
            COM(2,66),
            PTR(0, true, true),
        ], 
        vec![ // 47 
            COM(4,2),
            INT(1),
            ARG(0),
        ], 
        // AExp21
        vec![ // 48 
            COM(2,65),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 49 
            ARG(1),
            ARG(2),
        ], 
        vec![ // 50 
            ARG(0),
            ARG(2),
        ], 
        // AExp22
        vec![ // 51 
            COM(2,66),
            PTR(0, true, true),
        ], 
        vec![ // 52 
            COM(4,2),
            INT(2),
            ARG(0),
        ], 
        // AExp23
        vec![ // 53 
            COM(2,65),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 54 
            ARG(1),
            ARG(2),
        ], 
        vec![ // 55 
            ARG(0),
            ARG(2),
        ], 
        // AExp24
        vec![ // 56 
            COM(2,66),
            PTR(0, true, true),
        ], 
        vec![ // 57 
            COM(4,2),
            INT(3),
            ARG(0),
        ], 
        // AExp25
        vec![ // 58 
            COM(2,65),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 59 
            ARG(1),
            ARG(2),
        ], 
        vec![ // 60 
            ARG(0),
            ARG(2),
        ], 
        // AExp26
        vec![ // 61 
            COM(2,66),
            PTR(0, true, true),
        ], 
        vec![ // 62 
            COM(4,2),
            INT(1),
            ARG(0),
        ], 
        // AExp27
        vec![ // 63 
            COM(2,66),
            PTR(0, true, true),
        ], 
        vec![ // 64 
            COM(4,2),
            INT(3),
            ARG(0),
        ], 
        // AExp28
        vec![ // 65 
            ARG(0),
            ARG(1),
            COM(2,1),
        ], 
        // AExp29
        vec![ // 66 
            PTR(64, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 67 
            PTR(58, false, false),
            ARG(0),
            ARG(1),
        ], 
        // AExp30
        vec![ // 68 
            ARG(1),
            COM(2,1),
            ARG(0),
        ], 
        // AExp31
        vec![ // 69 
            COM(2,0),
        ], 
        // AExp32
        vec![ // 70 
            ARG(1),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 71 
            ARG(0),
            ARG(2),
        ], 
        // AExp33
        vec![ // 72 
            ARG(1),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 73 
            ARG(0),
            ARG(2),
            ARG(3),
        ], 
        vec![ // 74 
            COM(4,2),
            ARG(2),
            ARG(3),
        ], 
        // AExp34
        vec![ // 75 
            COM(2,21),
            ARG(2),
            ARG(4),
            PTR(2, true, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 76 
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(5),
        ], 
        vec![ // 77 
            ARG(0),
            ARG(2),
            ARG(3),
            ARG(4),
            ARG(5),
        ], 
        vec![ // 78 
            PTR(58, false, false),
            ARG(3),
            ARG(5),
        ], 
        // AExp35
        vec![ // 79 
            COM(4,2),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 80 
            ARG(0),
            ARG(2),
            ARG(3),
            ARG(4),
        ], 
        // AExp36
        vec![ // 81 
            PTR(58, false, false),
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 82 
            COM(4,2),
            ARG(1),
            ARG(2),
        ], 
        // AExp37
        vec![ // 83 
            PTR(58, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 84 
            COM(4,2),
            ARG(0),
            ARG(1),
        ], 
        // AExp38
        vec![ // 85 
            ARG(1),
            INT(3),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 86 
            COM(1,90),
            ARG(2),
        ], 
        vec![ // 87 
            ARG(0),
            ARG(2),
        ], 
        // AExp39
        vec![ // 88 
            COM(1,90),
            PTR(0, true, true),
        ], 
        vec![ // 89 
            COM(1,91),
            ARG(0),
        ], 
        // AExp40
        vec![ // 90 
            ARG(0),
            INT(0),
            INT(88),
        ], 
        // AExp41
        vec![ // 91 
            ARG(0),
            COM(2,1),
            COM(2,0),
        ], 
        // AExp42
        vec![ // 92 
            PTR(10, false, false),
            ARG(2),
            PTR(0, true, true),
            COM(3,1),
        ], 
        vec![ // 93 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp43
        vec![ // 94 
            PTR(48, false, false),
            ARG(1),
            ARG(2),
            PTR(0, true, true),
            COM(3,0),
        ], 
        vec![ // 95 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp44
        vec![ // 96 
            PTR(49, false, false),
            COM(2,116),
            PTR(0, true, true),
        ], 
        vec![ // 97 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp45
        vec![ // 98 
            ARG(0),
            PTR(0, true, true),
            ARG(2),
        ], 
        vec![ // 99 
            ARG(1),
            ARG(2),
        ], 
        // AExp46
        vec![ // 100 
            PTR(52, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 101 
            PTR(58, false, false),
            ARG(0),
            ARG(2),
        ], 
        vec![ // 102 
            PTR(54, false, false),
            ARG(1),
            ARG(2),
        ], 
        // AExp47
        vec![ // 103 
            PRM(EQ,false),
            PTR(0, true, true),
            INT(9),
        ], 
        vec![ // 104 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp48
        vec![ // 105 
            PRM(ADD,false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 106 
            PTR(8, false, false),
            ARG(1),
        ], 
        vec![ // 107 
            PTR(8, false, false),
            ARG(0),
        ], 
        // AExp49
        vec![ // 108 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 109 
            ARG(0),
            ARG(1),
        ], 
        // AExp50
        vec![ // 110 
            ARG(3),
            ERR(0),
            PTR(0, true, true),
        ], 
        vec![ // 111 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp51
        vec![ // 112 
            ARG(4),
            ARG(3),
            PTR(0, true, true),
        ], 
        vec![ // 113 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(4),
        ], 
        // AExp52
        vec![ // 114 
            ARG(0),
            ARG(2),
            PTR(0, true, true),
        ], 
        vec![ // 115 
            ARG(1),
            ARG(3),
        ], 
        // AExp53
        vec![ // 116 
            ARG(0),
            PTR(0, true, true),
            ARG(1),
            COM(3,23),
        ], 
        vec![ // 117 
            ARG(1),
            COM(3,0),
            COM(3,0),
            COM(3,23),
        ], 
        // AExp54
        vec![ // 118 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 119 
            ARG(0),
            ARG(1),
        ], 
        // AExp55
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
        // AExp56
        vec![ // 122 
            COM(4,2),
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
        // AExp57
        vec![ // 125 
            COM(1,129),
            PTR(0, true, true),
        ], 
        vec![ // 126 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(3),
        ], 
        // AExp58
        vec![ // 127 
            PTR(42, false, false),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 128 
            PTR(55, false, false),
            ARG(2),
            ARG(0),
        ], 
        // AExp59
        vec![ // 129 
            ARG(0),
            COM(3,0),
            COM(3,23),
            COM(3,1),
        ], 
        // AExp60
        vec![ // 130 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 131 
            ARG(0),
            ARG(1),
        ], 
        // AExp61
        vec![ // 132 
            ARG(3),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 133 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        vec![ // 134 
            COM(4,2),
            ARG(1),
            COM(2,0),
        ], 
        // AExp62
        vec![ // 135 
            PRM(LE,false),
            ARG(2),
            ARG(4),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 136 
            ARG(1),
            ARG(2),
            ARG(4),
            ARG(5),
        ], 
        vec![ // 137 
            ARG(0),
            ARG(3),
            ARG(4),
            ARG(5),
        ], 
        // AExp63
        vec![ // 138 
            COM(4,2),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 139 
            ARG(0),
            ARG(2),
        ], 
        // AExp64
        vec![ // 140 
            COM(4,2),
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 141 
            COM(4,2),
            ARG(1),
            ARG(2),
        ], 
        // AExp65
        vec![ // 142 
            PRM(LE,false),
            ARG(1),
            ARG(2),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 143 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp66
        vec![ // 144 
            COM(4,2),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 145 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp67
        vec![ // 146 
            PTR(62, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 147 
            PRM(ADD,false),
            ARG(0),
            INT(1),
        ], 
    ],

}});