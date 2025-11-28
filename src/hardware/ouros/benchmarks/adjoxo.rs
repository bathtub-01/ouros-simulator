use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 69
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
            COM(2,28),
            PTR(37, false, false),
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
            COM(1,65),
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
            COM(1,63),
            PTR(11, false, false),
        ], 
        vec![ // 13 
            COM(3,60),
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
            COM(1,58),
            PTR(15, false, false),
        ], 
        vec![ // 17 
            COM(3,55),
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
            COM(1,53),
            PTR(19, false, false),
        ], 
        vec![ // 21 
            COM(3,50),
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
            COM(1,48),
            PTR(23, false, false),
        ], 
        vec![ // 25 
            COM(3,45),
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
            COM(1,43),
            PTR(27, false, false),
        ], 
        vec![ // 29 
            COM(3,40),
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
            COM(1,38),
            PTR(31, false, false),
        ], 
        vec![ // 33 
            COM(3,35),
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
            COM(1,33),
            PTR(35, false, false),
        ], 
        vec![ // 37 
            COM(3,30),
            PTR(36, false, false),
            PTR(33, false, false),
        ], 
        // AExp3
        vec![ // 38 
            COM(3,94),
            PTR(43, false, false),
        ], 
        vec![ // 39 
            COM(2,148),
            INT(1),
            INT(9),
        ], 
        vec![ // 40 
            COM(2,85),
            PTR(39, false, false),
        ], 
        vec![ // 41 
            COM(2,103),
            PTR(40, false, false),
        ], 
        vec![ // 42 
            COM(3,98),
            PTR(41, false, false),
        ], 
        vec![ // 43 
            COM(3,96),
            PTR(42, false, false),
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
            COM(2,89),
            PTR(0, true, true),
            COM(2,1),
        ], 
        vec![ // 4 
            PTR(38, false, false),
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
            COM(2,89),
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
            COM(2,89),
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
            COM(2,89),
            PTR(0, true, true),
            COM(2,0),
        ], 
        vec![ // 12 
            PTR(38, false, false),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp7
        vec![ // 13 
            COM(2,89),
            PTR(0, true, true),
            COM(2,1),
        ], 
        vec![ // 14 
            PTR(38, false, false),
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
            SEQ(false),
            ARG(1, false),
            PTR(0, true, true),
        ], 
        vec![ // 29 
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp14
        vec![ // 30 
            COM(2,67),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 31 
            ARG(1, true),
            ARG(2, false),
        ], 
        vec![ // 32 
            ARG(0, true),
            ARG(2, false),
        ], 
        // AExp15
        vec![ // 33 
            COM(2,68),
            PTR(0, true, true),
        ], 
        vec![ // 34 
            COM(4,2),
            INT(1),
            ARG(0, true),
        ], 
        // AExp16
        vec![ // 35 
            COM(2,67),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 36 
            ARG(1, true),
            ARG(2, false),
        ], 
        vec![ // 37 
            ARG(0, true),
            ARG(2, false),
        ], 
        // AExp17
        vec![ // 38 
            COM(2,68),
            PTR(0, true, true),
        ], 
        vec![ // 39 
            COM(4,2),
            INT(4),
            ARG(0, true),
        ], 
        // AExp18
        vec![ // 40 
            COM(2,67),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 41 
            ARG(1, true),
            ARG(2, false),
        ], 
        vec![ // 42 
            ARG(0, true),
            ARG(2, false),
        ], 
        // AExp19
        vec![ // 43 
            COM(2,68),
            PTR(0, true, true),
        ], 
        vec![ // 44 
            COM(4,2),
            INT(7),
            ARG(0, true),
        ], 
        // AExp20
        vec![ // 45 
            COM(2,67),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 46 
            ARG(1, true),
            ARG(2, false),
        ], 
        vec![ // 47 
            ARG(0, true),
            ARG(2, false),
        ], 
        // AExp21
        vec![ // 48 
            COM(2,68),
            PTR(0, true, true),
        ], 
        vec![ // 49 
            COM(4,2),
            INT(1),
            ARG(0, true),
        ], 
        // AExp22
        vec![ // 50 
            COM(2,67),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 51 
            ARG(1, true),
            ARG(2, false),
        ], 
        vec![ // 52 
            ARG(0, true),
            ARG(2, false),
        ], 
        // AExp23
        vec![ // 53 
            COM(2,68),
            PTR(0, true, true),
        ], 
        vec![ // 54 
            COM(4,2),
            INT(2),
            ARG(0, true),
        ], 
        // AExp24
        vec![ // 55 
            COM(2,67),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 56 
            ARG(1, true),
            ARG(2, false),
        ], 
        vec![ // 57 
            ARG(0, true),
            ARG(2, false),
        ], 
        // AExp25
        vec![ // 58 
            COM(2,68),
            PTR(0, true, true),
        ], 
        vec![ // 59 
            COM(4,2),
            INT(3),
            ARG(0, true),
        ], 
        // AExp26
        vec![ // 60 
            COM(2,67),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 61 
            ARG(1, true),
            ARG(2, false),
        ], 
        vec![ // 62 
            ARG(0, true),
            ARG(2, false),
        ], 
        // AExp27
        vec![ // 63 
            COM(2,68),
            PTR(0, true, true),
        ], 
        vec![ // 64 
            COM(4,2),
            INT(1),
            ARG(0, true),
        ], 
        // AExp28
        vec![ // 65 
            COM(2,68),
            PTR(0, true, true),
        ], 
        vec![ // 66 
            COM(4,2),
            INT(3),
            ARG(0, true),
        ], 
        // AExp29
        vec![ // 67 
            ARG(0, true),
            ARG(1, true),
            COM(2,1),
        ], 
        // AExp30
        vec![ // 68 
            COM(1,71),
            PTR(0, true, true),
        ], 
        vec![ // 69 
            COM(2,85),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp31
        vec![ // 70 
            COM(2,0),
        ], 
        // AExp32
        vec![ // 71 
            ARG(0, true),
            COM(2,1),
            COM(2,70),
        ], 
        // AExp33
        vec![ // 72 
            COM(2,85),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 73 
            COM(4,2),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp34
        vec![ // 74 
            COM(4,2),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 75 
            COM(3,72),
            ARG(1, true),
            ARG(2, true),
            ARG(3, true),
        ], 
        // AExp35
        vec![ // 76 
            COM(2,85),
            PTR(0, true, true),
        ], 
        vec![ // 77 
            COM(4,2),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp36
        vec![ // 78 
            COM(2,21),
            ARG(0, false),
            ARG(2, false),
            PTR(2, true, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 79 
            COM(2,76),
            ARG(0, false),
            ARG(1, false),
            ARG(3, false),
        ], 
        vec![ // 80 
            COM(4,74),
            ARG(0, false),
            ARG(1, false),
            ARG(2, false),
            ARG(3, false),
        ], 
        vec![ // 81 
            COM(2,85),
            ARG(1, false),
            ARG(3, false),
        ], 
        // AExp37
        vec![ // 82 
            ARG(0, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 83 
            COM(4,78),
            ARG(1, false),
            ARG(2, false),
        ], 
        vec![ // 84 
            COM(4,2),
            ARG(1, false),
            ARG(2, false),
        ], 
        // AExp38
        vec![ // 85 
            ARG(0, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 86 
            COM(3,82),
            ARG(1, true),
        ], 
        // AExp39
        vec![ // 87 
            COM(1,92),
            PTR(0, true, true),
        ], 
        vec![ // 88 
            COM(1,93),
            ARG(0, true),
        ], 
        // AExp40
        vec![ // 89 
            ARG(0, true),
            INT(3),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 90 
            COM(1,92),
            ARG(1, false),
        ], 
        vec![ // 91 
            COM(1,87),
            ARG(1, false),
        ], 
        // AExp41
        vec![ // 92 
            ARG(0, true),
            INT(0),
            INT(88),
        ], 
        // AExp42
        vec![ // 93 
            ARG(0, true),
            COM(2,1),
            COM(2,0),
        ], 
        // AExp43
        vec![ // 94 
            PTR(6, false, false),
            ARG(2, false),
            PTR(0, true, true),
            COM(3,1),
        ], 
        vec![ // 95 
            ARG(0, true),
            ARG(1, true),
            ARG(2, false),
        ], 
        // AExp44
        vec![ // 96 
            COM(2,108),
            ARG(1, false),
            ARG(2, false),
            PTR(0, true, true),
            COM(3,0),
        ], 
        vec![ // 97 
            ARG(0, true),
            ARG(1, false),
            ARG(2, false),
        ], 
        // AExp45
        vec![ // 98 
            COM(1,116),
            COM(2,118),
            PTR(0, true, true),
        ], 
        vec![ // 99 
            ARG(0, true),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp46
        vec![ // 100 
            COM(1,125),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 101 
            COM(2,85),
            ARG(0, true),
            ARG(2, false),
        ], 
        vec![ // 102 
            COM(3,129),
            ARG(1, true),
            ARG(2, false),
        ], 
        // AExp47
        vec![ // 103 
            COM(3,100),
            PTR(0, true, true),
            ARG(1, false),
        ], 
        vec![ // 104 
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp48
        vec![ // 105 
            PRM(ADD,false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 106 
            PTR(5, false, false),
            ARG(1, true),
        ], 
        vec![ // 107 
            PTR(5, false, false),
            ARG(0, true),
        ], 
        // AExp49
        vec![ // 108 
            PRM(EQ,false),
            PTR(0, true, true),
            INT(9),
        ], 
        vec![ // 109 
            COM(2,105),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp50
        vec![ // 110 
            ARG(0, true),
            ARG(2, true),
            PTR(0, true, true),
        ], 
        vec![ // 111 
            ARG(1, true),
            ARG(3, true),
        ], 
        // AExp51
        vec![ // 112 
            ARG(3, false),
            ARG(2, false),
            PTR(0, true, true),
        ], 
        vec![ // 113 
            COM(6,110),
            ARG(0, true),
            ARG(1, true),
            ARG(2, false),
            ARG(3, false),
        ], 
        // AExp52
        vec![ // 114 
            ARG(2, true),
            ERR(0),
            PTR(0, true, true),
        ], 
        vec![ // 115 
            COM(4,112),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp53
        vec![ // 116 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 117 
            COM(3,114),
            ARG(0, true),
        ], 
        // AExp54
        vec![ // 118 
            ARG(0, true),
            PTR(0, true, true),
            ARG(1, false),
            COM(3,23),
        ], 
        vec![ // 119 
            ARG(1, false),
            COM(3,0),
            COM(3,0),
            COM(3,23),
        ], 
        // AExp55
        vec![ // 120 
            COM(4,2),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 121 
            ARG(1, true),
            ARG(3, true),
        ], 
        vec![ // 122 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp56
        vec![ // 123 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 124 
            COM(4,120),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp57
        vec![ // 125 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 126 
            COM(3,123),
            ARG(0, true),
        ], 
        // AExp58
        vec![ // 127 
            PTR(38, false, false),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 128 
            COM(1,142),
            ARG(2, true),
            ARG(0, true),
        ], 
        // AExp59
        vec![ // 129 
            COM(1,131),
            PTR(0, true, true),
        ], 
        vec![ // 130 
            COM(3,127),
            ARG(0, true),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp60
        vec![ // 131 
            ARG(0, true),
            COM(3,0),
            COM(3,23),
            COM(3,1),
        ], 
        // AExp61
        vec![ // 132 
            COM(4,2),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 133 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp62
        vec![ // 134 
            COM(4,2),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 135 
            COM(4,2),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp63
        vec![ // 136 
            PRM(LE,false),
            ARG(0, false),
            ARG(2, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 137 
            COM(3,134),
            ARG(0, false),
            ARG(2, false),
            ARG(3, false),
        ], 
        vec![ // 138 
            COM(3,132),
            ARG(1, true),
            ARG(2, false),
            ARG(3, false),
        ], 
        // AExp64
        vec![ // 139 
            ARG(2, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 140 
            COM(4,136),
            ARG(0, false),
            ARG(1, true),
        ], 
        vec![ // 141 
            COM(4,2),
            ARG(0, false),
            COM(2,0),
        ], 
        // AExp65
        vec![ // 142 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 143 
            COM(3,139),
            ARG(0, true),
        ], 
        // AExp66
        vec![ // 144 
            COM(2,148),
            PTR(0, true, true),
        ], 
        vec![ // 145 
            PRM(ADD,false),
            ARG(0, true),
            INT(1),
        ], 
        // AExp67
        vec![ // 146 
            COM(4,2),
            ARG(0, false),
            PTR(0, true, true),
        ], 
        vec![ // 147 
            COM(1,144),
            ARG(0, false),
            ARG(1, true),
        ], 
        // AExp68
        vec![ // 148 
            PRM(LE,false),
            ARG(0, false),
            ARG(1, false),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 149 
            COM(2,146),
            ARG(0, false),
            ARG(1, false),
        ], 
    ],

}});