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
            COM(2,15),
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
            Y,
            COM(3,26),
            INT(0),
        ], 
        // AExp2
        vec![ // 6 
            COM(3,28),
            PTR(36, false, false),
            PTR(33, false, false),
        ], 
        vec![ // 7 
            COM(4,2),
            INT(7),
            COM(2,0),
        ], 
        vec![ // 8 
            COM(4,2),
            INT(5),
            PTR(7, false, false),
        ], 
        vec![ // 9 
            COM(1,63),
            PTR(8, false, false),
        ], 
        vec![ // 10 
            COM(4,2),
            INT(9),
            COM(2,0),
        ], 
        vec![ // 11 
            COM(4,2),
            INT(5),
            PTR(10, false, false),
        ], 
        vec![ // 12 
            COM(1,61),
            PTR(11, false, false),
        ], 
        vec![ // 13 
            COM(3,58),
            PTR(12, false, false),
            PTR(9, false, false),
        ], 
        vec![ // 14 
            COM(4,2),
            INT(9),
            COM(2,0),
        ], 
        vec![ // 15 
            COM(4,2),
            INT(6),
            PTR(14, false, false),
        ], 
        vec![ // 16 
            COM(1,56),
            PTR(15, false, false),
        ], 
        vec![ // 17 
            COM(3,53),
            PTR(16, false, false),
            PTR(13, false, false),
        ], 
        vec![ // 18 
            COM(4,2),
            INT(8),
            COM(2,0),
        ], 
        vec![ // 19 
            COM(4,2),
            INT(5),
            PTR(18, false, false),
        ], 
        vec![ // 20 
            COM(1,51),
            PTR(19, false, false),
        ], 
        vec![ // 21 
            COM(3,48),
            PTR(20, false, false),
            PTR(17, false, false),
        ], 
        vec![ // 22 
            COM(4,2),
            INT(7),
            COM(2,0),
        ], 
        vec![ // 23 
            COM(4,2),
            INT(4),
            PTR(22, false, false),
        ], 
        vec![ // 24 
            COM(1,46),
            PTR(23, false, false),
        ], 
        vec![ // 25 
            COM(3,43),
            PTR(24, false, false),
            PTR(21, false, false),
        ], 
        vec![ // 26 
            COM(4,2),
            INT(9),
            COM(2,0),
        ], 
        vec![ // 27 
            COM(4,2),
            INT(8),
            PTR(26, false, false),
        ], 
        vec![ // 28 
            COM(1,41),
            PTR(27, false, false),
        ], 
        vec![ // 29 
            COM(3,38),
            PTR(28, false, false),
            PTR(25, false, false),
        ], 
        vec![ // 30 
            COM(4,2),
            INT(6),
            COM(2,0),
        ], 
        vec![ // 31 
            COM(4,2),
            INT(5),
            PTR(30, false, false),
        ], 
        vec![ // 32 
            COM(1,36),
            PTR(31, false, false),
        ], 
        vec![ // 33 
            COM(3,33),
            PTR(32, false, false),
            PTR(29, false, false),
        ], 
        vec![ // 34 
            COM(4,2),
            INT(3),
            COM(2,0),
        ], 
        vec![ // 35 
            COM(4,2),
            INT(2),
            PTR(34, false, false),
        ], 
        vec![ // 36 
            COM(1,31),
            PTR(35, false, false),
        ], 
        // AExp3
        vec![ // 37 
            COM(3,92),
            PTR(42, false, false),
        ], 
        vec![ // 38 
            COM(2,146),
            INT(1),
            INT(9),
        ], 
        vec![ // 39 
            COM(2,83),
            PTR(38, false, false),
        ], 
        vec![ // 40 
            COM(2,101),
            PTR(39, false, false),
        ], 
        vec![ // 41 
            COM(3,96),
            PTR(40, false, false),
        ], 
        vec![ // 42 
            COM(3,94),
            PTR(41, false, false),
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
            ARG(3, true),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp3
        vec![ // 3 
            COM(2,87),
            PTR(0, true, true),
            COM(2,1),
        ], 
        vec![ // 4 
            PTR(37, false, false),
            ARG(1, true),
            ARG(0, true),
        ], 
        // AExp4
        vec![ // 5 
            PTR(6, false, false),
            ARG(0, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 6 
            COM(2,87),
            COM(3,23),
            COM(2,0),
        ], 
        vec![ // 7 
            COM(2,3),
            ARG(0, false),
            ARG(1, true),
        ], 
        // AExp5
        vec![ // 8 
            PTR(6, false, false),
            ARG(1, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 9 
            COM(2,87),
            COM(3,23),
            COM(2,1),
        ], 
        vec![ // 10 
            COM(2,5),
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp6
        vec![ // 11 
            COM(2,87),
            PTR(0, true, true),
            COM(2,0),
        ], 
        vec![ // 12 
            PTR(37, false, false),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp7
        vec![ // 13 
            COM(2,87),
            PTR(0, true, true),
            COM(2,1),
        ], 
        vec![ // 14 
            PTR(37, false, false),
            ARG(1, true),
            ARG(0, true),
        ], 
        // AExp8
        vec![ // 15 
            COM(2,21),
            PTR(4, true, true),
            PTR(3, true, true),
            PTR(2, true, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 16 
            COM(2,13),
            ARG(0, false),
            ARG(1, false),
        ], 
        vec![ // 17 
            COM(2,11),
            ARG(0, false),
            ARG(1, false),
        ], 
        vec![ // 18 
            COM(2,8),
            ARG(0, false),
            ARG(1, false),
        ], 
        vec![ // 19 
            PTR(5, false, false),
            ARG(1, false),
        ], 
        vec![ // 20 
            PTR(5, false, false),
            ARG(0, false),
        ], 
        // AExp9
        vec![ // 21 
            PRM(EQ,false),
            ARG(0, false),
            ARG(1, false),
            PTR(0, true, true),
            COM(3,0),
        ], 
        vec![ // 22 
            PRM(LE,false),
            ARG(0, false),
            ARG(1, false),
            COM(3,23),
            COM(3,1),
        ], 
        // AExp10
        vec![ // 23 
            ARG(2, true),
        ], 
        // AExp11
        vec![ // 24 
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 25 
            PRM(ADD,false),
            ARG(1, true),
            INT(1),
        ], 
        // AExp12
        vec![ // 26 
            ARG(2, true),
            ARG(1, false),
            PTR(0, true, true),
        ], 
        vec![ // 27 
            COM(3,24),
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp13
        vec![ // 28 
            COM(2,65),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 29 
            ARG(1, true),
            ARG(2, false),
        ], 
        vec![ // 30 
            ARG(0, true),
            ARG(2, false),
        ], 
        // AExp14
        vec![ // 31 
            COM(2,66),
            PTR(0, true, true),
        ], 
        vec![ // 32 
            COM(4,2),
            INT(1),
            ARG(0, true),
        ], 
        // AExp15
        vec![ // 33 
            COM(2,65),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 34 
            ARG(1, true),
            ARG(2, false),
        ], 
        vec![ // 35 
            ARG(0, true),
            ARG(2, false),
        ], 
        // AExp16
        vec![ // 36 
            COM(2,66),
            PTR(0, true, true),
        ], 
        vec![ // 37 
            COM(4,2),
            INT(4),
            ARG(0, true),
        ], 
        // AExp17
        vec![ // 38 
            COM(2,65),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 39 
            ARG(1, true),
            ARG(2, false),
        ], 
        vec![ // 40 
            ARG(0, true),
            ARG(2, false),
        ], 
        // AExp18
        vec![ // 41 
            COM(2,66),
            PTR(0, true, true),
        ], 
        vec![ // 42 
            COM(4,2),
            INT(7),
            ARG(0, true),
        ], 
        // AExp19
        vec![ // 43 
            COM(2,65),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 44 
            ARG(1, true),
            ARG(2, false),
        ], 
        vec![ // 45 
            ARG(0, true),
            ARG(2, false),
        ], 
        // AExp20
        vec![ // 46 
            COM(2,66),
            PTR(0, true, true),
        ], 
        vec![ // 47 
            COM(4,2),
            INT(1),
            ARG(0, true),
        ], 
        // AExp21
        vec![ // 48 
            COM(2,65),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 49 
            ARG(1, true),
            ARG(2, false),
        ], 
        vec![ // 50 
            ARG(0, true),
            ARG(2, false),
        ], 
        // AExp22
        vec![ // 51 
            COM(2,66),
            PTR(0, true, true),
        ], 
        vec![ // 52 
            COM(4,2),
            INT(2),
            ARG(0, true),
        ], 
        // AExp23
        vec![ // 53 
            COM(2,65),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 54 
            ARG(1, true),
            ARG(2, false),
        ], 
        vec![ // 55 
            ARG(0, true),
            ARG(2, false),
        ], 
        // AExp24
        vec![ // 56 
            COM(2,66),
            PTR(0, true, true),
        ], 
        vec![ // 57 
            COM(4,2),
            INT(3),
            ARG(0, true),
        ], 
        // AExp25
        vec![ // 58 
            COM(2,65),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 59 
            ARG(1, true),
            ARG(2, false),
        ], 
        vec![ // 60 
            ARG(0, true),
            ARG(2, false),
        ], 
        // AExp26
        vec![ // 61 
            COM(2,66),
            PTR(0, true, true),
        ], 
        vec![ // 62 
            COM(4,2),
            INT(1),
            ARG(0, true),
        ], 
        // AExp27
        vec![ // 63 
            COM(2,66),
            PTR(0, true, true),
        ], 
        vec![ // 64 
            COM(4,2),
            INT(3),
            ARG(0, true),
        ], 
        // AExp28
        vec![ // 65 
            ARG(0, true),
            ARG(1, true),
            COM(2,1),
        ], 
        // AExp29
        vec![ // 66 
            COM(1,69),
            PTR(0, true, true),
        ], 
        vec![ // 67 
            COM(2,83),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp30
        vec![ // 68 
            COM(2,0),
        ], 
        // AExp31
        vec![ // 69 
            ARG(0, true),
            COM(2,1),
            COM(2,68),
        ], 
        // AExp32
        vec![ // 70 
            COM(2,83),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 71 
            COM(4,2),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp33
        vec![ // 72 
            COM(4,2),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 73 
            COM(3,70),
            ARG(1, true),
            ARG(2, true),
            ARG(3, true),
        ], 
        // AExp34
        vec![ // 74 
            COM(2,83),
            PTR(0, true, true),
        ], 
        vec![ // 75 
            COM(4,2),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp35
        vec![ // 76 
            COM(2,21),
            ARG(0, false),
            ARG(2, false),
            PTR(2, true, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 77 
            COM(2,74),
            ARG(0, false),
            ARG(1, false),
            ARG(3, false),
        ], 
        vec![ // 78 
            COM(4,72),
            ARG(0, false),
            ARG(1, false),
            ARG(2, false),
            ARG(3, false),
        ], 
        vec![ // 79 
            COM(2,83),
            ARG(1, false),
            ARG(3, false),
        ], 
        // AExp36
        vec![ // 80 
            ARG(0, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 81 
            COM(4,76),
            ARG(1, false),
            ARG(2, false),
        ], 
        vec![ // 82 
            COM(4,2),
            ARG(1, false),
            ARG(2, false),
        ], 
        // AExp37
        vec![ // 83 
            ARG(0, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 84 
            COM(3,80),
            ARG(1, true),
        ], 
        // AExp38
        vec![ // 85 
            COM(1,90),
            PTR(0, true, true),
        ], 
        vec![ // 86 
            COM(1,91),
            ARG(0, true),
        ], 
        // AExp39
        vec![ // 87 
            ARG(0, true),
            INT(3),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 88 
            COM(1,90),
            ARG(1, false),
        ], 
        vec![ // 89 
            COM(1,85),
            ARG(1, false),
        ], 
        // AExp40
        vec![ // 90 
            ARG(0, true),
            INT(0),
            INT(88),
        ], 
        // AExp41
        vec![ // 91 
            ARG(0, true),
            COM(2,1),
            COM(2,0),
        ], 
        // AExp42
        vec![ // 92 
            PTR(6, false, false),
            ARG(2, false),
            PTR(0, true, true),
            COM(3,1),
        ], 
        vec![ // 93 
            ARG(0, true),
            ARG(1, true),
            ARG(2, false),
        ], 
        // AExp43
        vec![ // 94 
            COM(2,106),
            ARG(1, false),
            ARG(2, false),
            PTR(0, true, true),
            COM(3,0),
        ], 
        vec![ // 95 
            ARG(0, true),
            ARG(1, false),
            ARG(2, false),
        ], 
        // AExp44
        vec![ // 96 
            COM(1,114),
            COM(2,116),
            PTR(0, true, true),
        ], 
        vec![ // 97 
            ARG(0, true),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp45
        vec![ // 98 
            COM(1,123),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 99 
            COM(2,83),
            ARG(0, true),
            ARG(2, false),
        ], 
        vec![ // 100 
            COM(3,127),
            ARG(1, true),
            ARG(2, false),
        ], 
        // AExp46
        vec![ // 101 
            COM(3,98),
            PTR(0, true, true),
            ARG(1, false),
        ], 
        vec![ // 102 
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp47
        vec![ // 103 
            PRM(ADD,false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 104 
            PTR(5, false, false),
            ARG(1, true),
        ], 
        vec![ // 105 
            PTR(5, false, false),
            ARG(0, true),
        ], 
        // AExp48
        vec![ // 106 
            PRM(EQ,false),
            PTR(0, true, true),
            INT(9),
        ], 
        vec![ // 107 
            COM(2,103),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp49
        vec![ // 108 
            ARG(0, true),
            ARG(2, true),
            PTR(0, true, true),
        ], 
        vec![ // 109 
            ARG(1, true),
            ARG(3, true),
        ], 
        // AExp50
        vec![ // 110 
            ARG(3, false),
            ARG(2, false),
            PTR(0, true, true),
        ], 
        vec![ // 111 
            COM(6,108),
            ARG(0, true),
            ARG(1, true),
            ARG(2, false),
            ARG(3, false),
        ], 
        // AExp51
        vec![ // 112 
            ARG(2, true),
            ERR(0),
            PTR(0, true, true),
        ], 
        vec![ // 113 
            COM(4,110),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp52
        vec![ // 114 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 115 
            COM(3,112),
            ARG(0, true),
        ], 
        // AExp53
        vec![ // 116 
            ARG(0, true),
            PTR(0, true, true),
            ARG(1, false),
            COM(3,23),
        ], 
        vec![ // 117 
            ARG(1, false),
            COM(3,0),
            COM(3,0),
            COM(3,23),
        ], 
        // AExp54
        vec![ // 118 
            COM(4,2),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 119 
            ARG(1, true),
            ARG(3, true),
        ], 
        vec![ // 120 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp55
        vec![ // 121 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 122 
            COM(4,118),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp56
        vec![ // 123 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 124 
            COM(3,121),
            ARG(0, true),
        ], 
        // AExp57
        vec![ // 125 
            PTR(37, false, false),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 126 
            COM(1,140),
            ARG(2, true),
            ARG(0, true),
        ], 
        // AExp58
        vec![ // 127 
            COM(1,129),
            PTR(0, true, true),
        ], 
        vec![ // 128 
            COM(3,125),
            ARG(0, true),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp59
        vec![ // 129 
            ARG(0, true),
            COM(3,0),
            COM(3,23),
            COM(3,1),
        ], 
        // AExp60
        vec![ // 130 
            COM(4,2),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 131 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp61
        vec![ // 132 
            COM(4,2),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 133 
            COM(4,2),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp62
        vec![ // 134 
            PRM(LE,false),
            ARG(0, false),
            ARG(2, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 135 
            COM(3,132),
            ARG(0, false),
            ARG(2, false),
            ARG(3, false),
        ], 
        vec![ // 136 
            COM(3,130),
            ARG(1, true),
            ARG(2, false),
            ARG(3, false),
        ], 
        // AExp63
        vec![ // 137 
            ARG(2, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 138 
            COM(4,134),
            ARG(0, false),
            ARG(1, true),
        ], 
        vec![ // 139 
            COM(4,2),
            ARG(0, false),
            COM(2,0),
        ], 
        // AExp64
        vec![ // 140 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 141 
            COM(3,137),
            ARG(0, true),
        ], 
        // AExp65
        vec![ // 142 
            COM(2,146),
            PTR(0, true, true),
        ], 
        vec![ // 143 
            PRM(ADD,false),
            ARG(0, true),
            INT(1),
        ], 
        // AExp66
        vec![ // 144 
            COM(4,2),
            ARG(0, false),
            PTR(0, true, true),
        ], 
        vec![ // 145 
            COM(1,142),
            ARG(0, false),
            ARG(1, true),
        ], 
        // AExp67
        vec![ // 146 
            PRM(LE,false),
            ARG(0, false),
            ARG(1, false),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 147 
            COM(2,144),
            ARG(0, false),
            ARG(1, false),
        ], 
    ],

}});