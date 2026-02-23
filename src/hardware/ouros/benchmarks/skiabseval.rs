use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 143
#[rustfmt::skip]
pub static SKIABSEVAL: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            COM(1,7),
            PTR(5, false, false),
        ], 
        vec![ // 1 
            COM(2,175),
            INT(5),
        ], 
        vec![ // 2 
            COM(2,175),
            INT(1),
        ], 
        vec![ // 3 
            COM(1,206),
            PTR(6, false, false),
        ], 
        vec![ // 4 
            COM(3,21),
            PTR(3, false, false),
            PTR(2, false, false),
        ], 
        vec![ // 5 
            COM(3,21),
            PTR(4, false, false),
            PTR(1, false, false),
        ], 
        // AExp1
        vec![ // 6 
            COM(3,21),
            COM(1,193),
            PTR(27, false, false),
        ], 
        vec![ // 7 
            COM(2,286),
            INT(2),
        ], 
        vec![ // 8 
            COM(2,175),
            INT(1),
        ], 
        vec![ // 9 
            COM(2,286),
            INT(1),
        ], 
        vec![ // 10 
            COM(3,21),
            COM(1,288),
            PTR(9, false, false),
        ], 
        vec![ // 11 
            COM(3,21),
            PTR(10, false, false),
            PTR(8, false, false),
        ], 
        vec![ // 12 
            COM(2,286),
            INT(0),
        ], 
        vec![ // 13 
            COM(3,21),
            PTR(12, false, false),
            PTR(11, false, false),
        ], 
        vec![ // 14 
            COM(3,21),
            PTR(13, false, false),
            PTR(7, false, false),
        ], 
        vec![ // 15 
            COM(2,286),
            INT(1),
        ], 
        vec![ // 16 
            COM(3,21),
            COM(1,288),
            PTR(15, false, false),
        ], 
        vec![ // 17 
            COM(3,21),
            PTR(16, false, false),
            PTR(14, false, false),
        ], 
        vec![ // 18 
            COM(2,286),
            INT(1),
        ], 
        vec![ // 19 
            COM(2,286),
            INT(2),
        ], 
        vec![ // 20 
            COM(2,286),
            INT(1),
        ], 
        vec![ // 21 
            COM(3,21),
            COM(1,285),
            PTR(20, false, false),
        ], 
        vec![ // 22 
            COM(3,21),
            PTR(21, false, false),
            PTR(19, false, false),
        ], 
        vec![ // 23 
            COM(3,21),
            PTR(22, false, false),
            PTR(18, false, false),
        ], 
        vec![ // 24 
            COM(3,21),
            PTR(23, false, false),
            PTR(17, false, false),
        ], 
        vec![ // 25 
            COM(3,283),
            INT(2),
            PTR(24, false, false),
        ], 
        vec![ // 26 
            COM(3,283),
            INT(1),
            PTR(25, false, false),
        ], 
        vec![ // 27 
            COM(3,283),
            INT(0),
            PTR(26, false, false),
        ], 
        // AExp2
        vec![ // 28 
            COM(3,9),
            COM(1,11),
            PTR(29, false, false),
        ], 
        vec![ // 29 
            COM(3,9),
            COM(1,166),
            COM(1,192),
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
            COM(1,7),
            PTR(0, true, true),
        ], 
        vec![ // 3 
            PTR(28, false, false),
            ARG(0, true),
        ], 
        // AExp3
        vec![ // 4 
            PRM(EQ,false),
            ARG(1, true),
            INT(10),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 5 
            ARG(2, true),
            COM(1,0),
        ], 
        vec![ // 6 
            COM(1,2),
            ARG(0, true),
        ], 
        // AExp4
        vec![ // 7 
            ARG(0, false),
            PTR(0, true, true),
        ], 
        vec![ // 8 
            COM(3,4),
            ARG(0, false),
        ], 
        // AExp5
        vec![ // 9 
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 10 
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp6
        vec![ // 11 
            ARG(0, true),
            COM(1,13),
            PTR(0, true, true),
        ], 
        vec![ // 12 
            COM(1,18),
            COM(3,21),
        ], 
        // AExp7
        vec![ // 13 
            ARG(0, true),
            INT(5),
            COM(1,0),
        ], 
        // AExp8
        vec![ // 14 
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 15 
            ARG(0, true),
            ARG(2, true),
            ARG(3, true),
        ], 
        // AExp9
        vec![ // 16 
            ARG(3, true),
            ARG(2, false),
            PTR(0, true, true),
        ], 
        vec![ // 17 
            COM(4,14),
            ARG(0, true),
            ARG(1, true),
            ARG(2, false),
        ], 
        // AExp10
        vec![ // 18 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 19 
            COM(4,16),
            ARG(0, true),
        ], 
        // AExp11
        vec![ // 20 
            ARG(2, true),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp12
        vec![ // 21 
            ARG(2, true),
            INT(2),
            PTR(0, true, true),
        ], 
        vec![ // 22 
            COM(3,20),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp13
        vec![ // 23 
            ARG(3, true),
            PTR(0, true, true),
        ], 
        vec![ // 24 
            ARG(2, true),
            ARG(1, true),
            ARG(0, true),
        ], 
        // AExp14
        vec![ // 25 
            COM(4,168),
            PTR(0, true, true),
        ], 
        vec![ // 26 
            PRM(EQ,false),
            ARG(0, true),
            ARG(1, true),
            COM(1,169),
            COM(1,170),
        ], 
        // AExp15
        vec![ // 27 
            COM(1,173),
            PTR(0, true, true),
        ], 
        vec![ // 28 
            PTR(28, false, false),
            ARG(0, true),
        ], 
        // AExp16
        vec![ // 29 
            COM(1,173),
            PTR(0, true, true),
        ], 
        vec![ // 30 
            PTR(28, false, false),
            ARG(0, true),
        ], 
        // AExp17
        vec![ // 31 
            COM(2,25),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 32 
            COM(1,29),
            ARG(1, true),
        ], 
        vec![ // 33 
            COM(1,27),
            ARG(0, true),
        ], 
        // AExp18
        vec![ // 34 
            COM(1,166),
            PTR(0, true, true),
        ], 
        vec![ // 35 
            COM(2,31),
            ARG(0, true),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp19
        vec![ // 36 
            ARG(2, true),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 37 
            COM(3,34),
            ARG(1, true),
        ], 
        // AExp20
        vec![ // 38 
            COM(4,23),
            PTR(0, true, true),
            ARG(0, false),
        ], 
        vec![ // 39 
            COM(3,36),
            ARG(0, false),
        ], 
        // AExp21
        vec![ // 40 
            COM(1,166),
            PTR(0, true, true),
        ], 
        vec![ // 41 
            COM(4,168),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp22
        vec![ // 42 
            COM(2,175),
            PTR(0, true, true),
        ], 
        vec![ // 43 
            PRM(ADD,false),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp23
        vec![ // 44 
            COM(1,173),
            PTR(0, true, true),
        ], 
        vec![ // 45 
            PTR(28, false, false),
            ARG(0, true),
        ], 
        // AExp24
        vec![ // 46 
            COM(1,173),
            PTR(0, true, true),
        ], 
        vec![ // 47 
            PTR(28, false, false),
            ARG(0, true),
        ], 
        // AExp25
        vec![ // 48 
            COM(2,42),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 49 
            COM(1,46),
            ARG(1, true),
        ], 
        vec![ // 50 
            COM(1,44),
            ARG(0, true),
        ], 
        // AExp26
        vec![ // 51 
            COM(2,40),
            PTR(0, true, true),
        ], 
        vec![ // 52 
            COM(2,48),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp27
        vec![ // 53 
            ARG(2, true),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 54 
            COM(2,51),
            ARG(1, true),
        ], 
        // AExp28
        vec![ // 55 
            COM(4,23),
            PTR(0, true, true),
            ARG(0, false),
        ], 
        vec![ // 56 
            COM(3,53),
            ARG(0, false),
        ], 
        // AExp29
        vec![ // 57 
            PRM(LT,false),
            ARG(2, true),
            INT(12),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 58 
            COM(1,55),
            ARG(0, false),
            ARG(1, false),
            ARG(3, false),
        ], 
        vec![ // 59 
            COM(1,38),
            ARG(0, false),
            ARG(1, false),
            ARG(3, false),
        ], 
        // AExp30
        vec![ // 60 
            COM(4,168),
            PTR(0, true, true),
        ], 
        vec![ // 61 
            COM(3,21),
            COM(1,193),
            ARG(0, true),
        ], 
        // AExp31
        vec![ // 62 
            COM(2,181),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 63 
            COM(1,60),
            ARG(0, false),
            ARG(1, true),
        ], 
        vec![ // 64 
            COM(1,192),
            ARG(0, false),
        ], 
        // AExp32
        vec![ // 65 
            ARG(2, true),
            PTR(0, true, true),
        ], 
        vec![ // 66 
            ARG(1, true),
            ARG(0, true),
            COM(2,62),
        ], 
        // AExp33
        vec![ // 67 
            PRM(EQ,false),
            ARG(2, true),
            INT(9),
            ARG(0, false),
            PTR(0, true, true),
        ], 
        vec![ // 68 
            COM(3,65),
            ARG(0, false),
            ARG(1, true),
            ARG(3, true),
        ], 
        // AExp34
        vec![ // 69 
            PRM(LT,false),
            ARG(2, false),
            INT(11),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 70 
            COM(4,67),
            ARG(0, false),
            ARG(1, false),
            ARG(2, false),
            ARG(3, false),
        ], 
        vec![ // 71 
            COM(4,57),
            ARG(0, false),
            ARG(1, false),
            ARG(2, false),
            ARG(3, false),
        ], 
        // AExp35
        vec![ // 72 
            COM(2,181),
            PTR(0, true, true),
        ], 
        vec![ // 73 
            COM(1,192),
            ARG(0, true),
        ], 
        // AExp36
        vec![ // 74 
            COM(1,166),
            PTR(0, true, true),
        ], 
        vec![ // 75 
            COM(1,72),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp37
        vec![ // 76 
            ARG(2, true),
            ARG(0, true),
            COM(2,74),
        ], 
        // AExp38
        vec![ // 77 
            COM(4,23),
            PTR(0, true, true),
            ARG(0, false),
        ], 
        vec![ // 78 
            COM(3,76),
            ARG(0, false),
        ], 
        // AExp39
        vec![ // 79 
            COM(4,168),
            PTR(0, true, true),
        ], 
        vec![ // 80 
            PTR(28, false, false),
            ARG(0, true),
        ], 
        // AExp40
        vec![ // 81 
            COM(4,168),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 82 
            COM(1,79),
            ARG(0, true),
            ARG(2, true),
        ], 
        vec![ // 83 
            PTR(28, false, false),
            ARG(1, true),
        ], 
        // AExp41
        vec![ // 84 
            COM(2,181),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 85 
            COM(3,81),
            ARG(1, true),
            ARG(2, true),
            ARG(3, true),
        ], 
        vec![ // 86 
            COM(1,192),
            ARG(0, true),
        ], 
        // AExp42
        vec![ // 87 
            COM(1,166),
            PTR(0, true, true),
        ], 
        vec![ // 88 
            COM(4,84),
            ARG(0, true),
            ARG(1, true),
            ARG(2, true),
            ARG(3, true),
        ], 
        // AExp43
        vec![ // 89 
            ARG(3, true),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 90 
            COM(4,87),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp44
        vec![ // 91 
            ARG(2, true),
            ARG(0, false),
            PTR(0, true, true),
        ], 
        vec![ // 92 
            COM(4,89),
            ARG(0, false),
            ARG(1, true),
        ], 
        // AExp45
        vec![ // 93 
            COM(4,23),
            PTR(0, true, true),
            ARG(0, false),
        ], 
        vec![ // 94 
            COM(3,91),
            ARG(0, false),
        ], 
        // AExp46
        vec![ // 95 
            PRM(LT,false),
            ARG(2, true),
            INT(8),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 96 
            COM(1,93),
            ARG(0, false),
            ARG(1, false),
            ARG(3, false),
        ], 
        vec![ // 97 
            COM(1,77),
            ARG(0, false),
            ARG(1, false),
            ARG(3, false),
        ], 
        // AExp47
        vec![ // 98 
            PRM(LT,false),
            ARG(2, false),
            INT(9),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 99 
            COM(4,95),
            ARG(0, false),
            ARG(1, false),
            ARG(2, false),
            ARG(3, false),
        ], 
        vec![ // 100 
            COM(4,69),
            ARG(0, false),
            ARG(1, false),
            ARG(2, false),
            ARG(3, false),
        ], 
        // AExp48
        vec![ // 101 
            COM(2,181),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 102 
            COM(4,168),
            ARG(1, true),
            ARG(2, true),
        ], 
        vec![ // 103 
            COM(1,192),
            ARG(0, true),
        ], 
        // AExp49
        vec![ // 104 
            COM(3,21),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 105 
            PTR(28, false, false),
            ARG(1, true),
        ], 
        vec![ // 106 
            PTR(28, false, false),
            ARG(0, true),
        ], 
        // AExp50
        vec![ // 107 
            COM(3,101),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 108 
            COM(2,104),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp51
        vec![ // 109 
            COM(1,166),
            PTR(0, true, true),
        ], 
        vec![ // 110 
            COM(3,107),
            ARG(0, true),
            ARG(1, true),
            ARG(2, true),
            ARG(3, true),
        ], 
        // AExp52
        vec![ // 111 
            ARG(3, true),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 112 
            COM(4,109),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp53
        vec![ // 113 
            ARG(2, true),
            ARG(0, false),
            PTR(0, true, true),
        ], 
        vec![ // 114 
            COM(4,111),
            ARG(0, false),
            ARG(1, true),
        ], 
        // AExp54
        vec![ // 115 
            COM(4,23),
            PTR(0, true, true),
            ARG(0, false),
        ], 
        vec![ // 116 
            COM(3,113),
            ARG(0, false),
        ], 
        // AExp55
        vec![ // 117 
            COM(2,181),
            PTR(0, true, true),
        ], 
        vec![ // 118 
            COM(1,192),
            ARG(0, true),
        ], 
        // AExp56
        vec![ // 119 
            COM(1,166),
            PTR(0, true, true),
        ], 
        vec![ // 120 
            COM(1,117),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp57
        vec![ // 121 
            ARG(2, true),
            PTR(0, true, true),
        ], 
        vec![ // 122 
            ARG(1, true),
            ARG(0, true),
            COM(2,119),
        ], 
        // AExp58
        vec![ // 123 
            PRM(LT,false),
            ARG(2, true),
            INT(6),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 124 
            COM(3,121),
            ARG(0, false),
            ARG(1, false),
            ARG(3, false),
        ], 
        vec![ // 125 
            COM(1,115),
            ARG(0, false),
            ARG(1, false),
            ARG(3, false),
        ], 
        // AExp59
        vec![ // 126 
            COM(2,181),
            PTR(0, true, true),
        ], 
        vec![ // 127 
            COM(1,192),
            ARG(0, true),
        ], 
        // AExp60
        vec![ // 128 
            COM(1,166),
            PTR(0, true, true),
        ], 
        vec![ // 129 
            COM(1,126),
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp61
        vec![ // 130 
            ARG(2, true),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 131 
            COM(3,128),
            ARG(1, true),
        ], 
        // AExp62
        vec![ // 132 
            COM(4,23),
            PTR(0, true, true),
            ARG(0, false),
        ], 
        vec![ // 133 
            COM(3,130),
            ARG(0, false),
        ], 
        // AExp63
        vec![ // 134 
            COM(3,21),
            PTR(0, true, true),
        ], 
        vec![ // 135 
            PTR(28, false, false),
            ARG(0, true),
        ], 
        // AExp64
        vec![ // 136 
            COM(4,168),
            PTR(0, true, true),
            ARG(1, true),
        ], 
        vec![ // 137 
            COM(1,134),
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp65
        vec![ // 138 
            COM(4,168),
            ARG(2, false),
            PTR(0, true, true),
        ], 
        vec![ // 139 
            COM(3,136),
            ARG(0, true),
            ARG(1, true),
            ARG(2, false),
        ], 
        // AExp66
        vec![ // 140 
            COM(2,181),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 141 
            COM(3,138),
            ARG(1, true),
            ARG(2, true),
            ARG(3, true),
        ], 
        vec![ // 142 
            COM(1,192),
            ARG(0, true),
        ], 
        // AExp67
        vec![ // 143 
            COM(1,166),
            PTR(0, true, true),
        ], 
        vec![ // 144 
            COM(4,140),
            ARG(0, true),
            ARG(1, true),
            ARG(3, true),
            ARG(2, true),
        ], 
        // AExp68
        vec![ // 145 
            COM(4,143),
            ARG(0, true),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 146 
            PTR(28, false, false),
            ARG(2, true),
        ], 
        // AExp69
        vec![ // 147 
            ARG(3, true),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 148 
            COM(3,145),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp70
        vec![ // 149 
            ARG(2, true),
            ARG(0, false),
            PTR(0, true, true),
        ], 
        vec![ // 150 
            COM(4,147),
            ARG(0, false),
            ARG(1, true),
        ], 
        // AExp71
        vec![ // 151 
            COM(4,23),
            PTR(0, true, true),
            ARG(0, false),
        ], 
        vec![ // 152 
            COM(3,149),
            ARG(0, false),
        ], 
        // AExp72
        vec![ // 153 
            PRM(EQ,false),
            ARG(2, true),
            INT(3),
            ARG(0, false),
            PTR(0, true, true),
        ], 
        vec![ // 154 
            COM(1,151),
            ARG(0, false),
            ARG(1, true),
            ARG(3, true),
        ], 
        // AExp73
        vec![ // 155 
            PRM(LT,false),
            ARG(2, false),
            INT(4),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 156 
            COM(4,153),
            ARG(0, false),
            ARG(1, false),
            ARG(2, false),
            ARG(3, false),
        ], 
        vec![ // 157 
            COM(1,132),
            ARG(0, false),
            ARG(1, false),
            ARG(3, false),
        ], 
        // AExp74
        vec![ // 158 
            PRM(LT,false),
            ARG(2, false),
            INT(5),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 159 
            COM(4,155),
            ARG(0, false),
            ARG(1, false),
            ARG(2, false),
            ARG(3, false),
        ], 
        vec![ // 160 
            COM(4,123),
            ARG(0, false),
            ARG(1, false),
            ARG(2, false),
            ARG(3, false),
        ], 
        // AExp75
        vec![ // 161 
            PRM(LT,false),
            ARG(2, false),
            INT(7),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 162 
            COM(4,158),
            ARG(0, false),
            ARG(1, false),
            ARG(2, false),
            ARG(3, false),
        ], 
        vec![ // 163 
            COM(4,98),
            ARG(0, false),
            ARG(1, false),
            ARG(2, false),
            ARG(3, false),
        ], 
        // AExp76
        vec![ // 164 
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 165 
            COM(4,161),
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp77
        vec![ // 166 
            ARG(0, false),
            ARG(0, false),
            PTR(0, true, true),
        ], 
        vec![ // 167 
            COM(3,164),
            ARG(0, false),
        ], 
        // AExp78
        vec![ // 168 
            ARG(3, true),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp79
        vec![ // 169 
            ARG(0, true),
            INT(8),
            COM(1,0),
        ], 
        // AExp80
        vec![ // 170 
            ARG(0, true),
            INT(4),
            COM(1,0),
        ], 
        // AExp81
        vec![ // 171 
            PRM(EQ,false),
            ARG(0, true),
            INT(10),
            INT(0),
            PTR(0, true, true),
        ], 
        vec![ // 172 
            ARG(1, true),
            COM(1,0),
        ], 
        // AExp82
        vec![ // 173 
            ARG(0, true),
            COM(2,171),
        ], 
        // AExp83
        vec![ // 174 
            ARG(1, true),
            ARG(0, true),
        ], 
        // AExp84
        vec![ // 175 
            ARG(1, true),
            INT(10),
            PTR(0, true, true),
        ], 
        vec![ // 176 
            COM(2,174),
            ARG(0, true),
        ], 
        // AExp85
        vec![ // 177 
            COM(4,168),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 178 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp86
        vec![ // 179 
            ARG(2, true),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 180 
            COM(3,177),
            ARG(1, true),
        ], 
        // AExp87
        vec![ // 181 
            Y,
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 182 
            COM(3,179),
            ARG(1, true),
        ], 
        // AExp88
        vec![ // 183 
            PRM(EQ,false),
            ARG(3, true),
            INT(2),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 184 
            ARG(4, true),
            ARG(1, true),
        ], 
        vec![ // 185 
            COM(4,168),
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp89
        vec![ // 186 
            ARG(0, true),
            ARG(2, true),
            PTR(0, true, true),
        ], 
        vec![ // 187 
            COM(4,168),
            ARG(3, true),
            ARG(1, true),
        ], 
        // AExp90
        vec![ // 188 
            COM(5,183),
            ARG(1, true),
            PTR(0, true, true),
            ARG(2, false),
        ], 
        vec![ // 189 
            COM(4,186),
            ARG(0, true),
            ARG(2, false),
        ], 
        // AExp91
        vec![ // 190 
            ARG(1, false),
            PTR(0, true, true),
        ], 
        vec![ // 191 
            COM(3,188),
            ARG(0, true),
            ARG(1, false),
            ARG(2, true),
        ], 
        // AExp92
        vec![ // 192 
            Y,
            COM(3,190),
            ARG(0, true),
            COM(2,0),
        ], 
        // AExp93
        vec![ // 193 
            ARG(0, true),
            INT(9),
            COM(1,0),
        ], 
        // AExp94
        vec![ // 194 
            COM(3,21),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 195 
            COM(1,206),
            ARG(1, true),
        ], 
        vec![ // 196 
            COM(1,206),
            ARG(0, true),
        ], 
        // AExp95
        vec![ // 197 
            PRM(EQ,false),
            ARG(1, true),
            INT(2),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 198 
            ARG(2, true),
            COM(2,194),
        ], 
        // AExp96
        vec![ // 199 
            COM(1,227),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 200 
            COM(1,206),
            ARG(1, true),
        ], 
        // AExp97
        vec![ // 201 
            PRM(EQ,false),
            ARG(1, true),
            INT(1),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 202 
            ARG(2, true),
            COM(2,199),
        ], 
        // AExp98
        vec![ // 203 
            PRM(LT,false),
            ARG(1, false),
            INT(2),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 204 
            COM(3,201),
            ARG(0, false),
            ARG(1, false),
            ARG(2, false),
        ], 
        vec![ // 205 
            COM(3,197),
            ARG(0, false),
            ARG(1, false),
            ARG(2, false),
        ], 
        // AExp99
        vec![ // 206 
            ARG(0, false),
            PTR(0, true, true),
        ], 
        vec![ // 207 
            COM(3,203),
            ARG(0, false),
        ], 
        // AExp100
        vec![ // 208 
            PRM(EQ,false),
            ARG(2, true),
            INT(2),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 209 
            ARG(3, true),
            ARG(0, true),
        ], 
        // AExp101
        vec![ // 210 
            COM(0,279),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 211 
            ARG(0, false),
            ARG(2, true),
        ], 
        vec![ // 212 
            ARG(0, false),
            ARG(1, true),
        ], 
        // AExp102
        vec![ // 213 
            COM(4,208),
            PTR(0, true, true),
        ], 
        vec![ // 214 
            COM(3,210),
            ARG(0, true),
        ], 
        // AExp103
        vec![ // 215 
            PRM(EQ,false),
            ARG(2, true),
            INT(0),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 216 
            ARG(3, true),
            ARG(0, true),
        ], 
        // AExp104
        vec![ // 217 
            PRM(EQ,false),
            ARG(0, true),
            ARG(2, true),
            ARG(1, true),
            COM(1,13),
        ], 
        // AExp105
        vec![ // 218 
            COM(4,215),
            PTR(0, true, true),
            ARG(1, false),
        ], 
        vec![ // 219 
            COM(3,217),
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp106
        vec![ // 220 
            PRM(LT,false),
            ARG(3, false),
            INT(2),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 221 
            COM(2,218),
            ARG(0, true),
            ARG(2, false),
            ARG(3, false),
            ARG(4, false),
        ], 
        vec![ // 222 
            COM(1,213),
            ARG(1, true),
            ARG(2, false),
            ARG(3, false),
            ARG(4, false),
        ], 
        // AExp107
        vec![ // 223 
            ARG(2, true),
            PTR(0, true, true),
        ], 
        vec![ // 224 
            COM(5,220),
            ARG(0, true),
            ARG(1, true),
            ARG(3, true),
        ], 
        // AExp108
        vec![ // 225 
            COM(4,223),
            ARG(0, true),
            ARG(1, true),
            ARG(2, false),
            PTR(0, true, true),
        ], 
        vec![ // 226 
            COM(3,21),
            COM(1,170),
            ARG(2, false),
        ], 
        // AExp109
        vec![ // 227 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 228 
            COM(3,225),
            ARG(0, true),
        ], 
        // AExp110
        vec![ // 229 
            PRM(EQ,false),
            ARG(2, true),
            INT(2),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 230 
            ARG(3, true),
            ARG(0, true),
        ], 
        // AExp111
        vec![ // 231 
            PRM(EQ,false),
            ARG(2, true),
            INT(4),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 232 
            ARG(3, true),
            ARG(1, true),
        ], 
        // AExp112
        vec![ // 233 
            PRM(EQ,false),
            ARG(2, true),
            INT(2),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 234 
            ARG(3, true),
            ARG(0, true),
        ], 
        // AExp113
        vec![ // 235 
            PRM(EQ,false),
            ARG(2, true),
            INT(4),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 236 
            ARG(3, true),
            ARG(1, true),
        ], 
        // AExp114
        vec![ // 237 
            COM(3,21),
            COM(1,170),
            PTR(0, true, true),
        ], 
        vec![ // 238 
            COM(3,21),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp115
        vec![ // 239 
            COM(4,235),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 240 
            COM(2,237),
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp116
        vec![ // 241 
            ARG(2, true),
            PTR(0, true, true),
        ], 
        vec![ // 242 
            COM(3,239),
            ARG(0, true),
            ARG(1, true),
            ARG(3, true),
        ], 
        // AExp117
        vec![ // 243 
            COM(4,233),
            PTR(0, true, true),
            ARG(1, false),
        ], 
        vec![ // 244 
            COM(4,241),
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp118
        vec![ // 245 
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 246 
            COM(2,243),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp119
        vec![ // 247 
            COM(3,21),
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 248 
            COM(3,21),
            COM(1,280),
            ARG(1, true),
        ], 
        // AExp120
        vec![ // 249 
            COM(3,245),
            ARG(0, false),
            ARG(1, false),
            PTR(0, true, true),
        ], 
        vec![ // 250 
            COM(2,247),
            ARG(0, false),
            ARG(1, false),
        ], 
        // AExp121
        vec![ // 251 
            COM(4,231),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 252 
            COM(2,249),
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp122
        vec![ // 253 
            ARG(2, true),
            PTR(0, true, true),
        ], 
        vec![ // 254 
            COM(3,251),
            ARG(0, true),
            ARG(1, true),
            ARG(3, true),
        ], 
        // AExp123
        vec![ // 255 
            COM(4,229),
            PTR(0, true, true),
            ARG(1, false),
        ], 
        vec![ // 256 
            COM(4,253),
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp124
        vec![ // 257 
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 258 
            COM(2,255),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp125
        vec![ // 259 
            PRM(EQ,false),
            ARG(2, true),
            INT(2),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 260 
            ARG(3, true),
            ARG(0, true),
        ], 
        // AExp126
        vec![ // 261 
            PRM(EQ,false),
            ARG(2, true),
            INT(4),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 262 
            ARG(3, true),
            ARG(1, true),
        ], 
        // AExp127
        vec![ // 263 
            COM(3,21),
            PTR(0, true, true),
        ], 
        vec![ // 264 
            COM(3,21),
            COM(1,281),
            ARG(0, true),
        ], 
        // AExp128
        vec![ // 265 
            COM(4,261),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 266 
            COM(1,263),
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp129
        vec![ // 267 
            ARG(2, true),
            PTR(0, true, true),
        ], 
        vec![ // 268 
            COM(3,265),
            ARG(0, true),
            ARG(1, true),
            ARG(3, true),
        ], 
        // AExp130
        vec![ // 269 
            COM(4,259),
            PTR(0, true, true),
            ARG(1, false),
        ], 
        vec![ // 270 
            COM(4,267),
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp131
        vec![ // 271 
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 272 
            COM(2,269),
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp132
        vec![ // 273 
            COM(3,21),
            PTR(0, true, true),
        ], 
        vec![ // 274 
            COM(3,21),
            COM(1,282),
            ARG(0, true),
        ], 
        // AExp133
        vec![ // 275 
            COM(3,271),
            ARG(0, false),
            ARG(1, false),
            PTR(0, true, true),
        ], 
        vec![ // 276 
            COM(1,273),
            ARG(0, false),
            ARG(1, false),
        ], 
        // AExp134
        vec![ // 277 
            COM(3,257),
            ARG(0, false),
            ARG(1, false),
            PTR(0, true, true),
        ], 
        vec![ // 278 
            COM(2,275),
            ARG(0, false),
            ARG(1, false),
        ], 
        // AExp135
        vec![ // 279 
            COM(2,277),
        ], 
        // AExp136
        vec![ // 280 
            ARG(0, true),
            INT(6),
            COM(1,0),
        ], 
        // AExp137
        vec![ // 281 
            ARG(0, true),
            INT(7),
            COM(1,0),
        ], 
        // AExp138
        vec![ // 282 
            ARG(0, true),
            INT(3),
            COM(1,0),
        ], 
        // AExp139
        vec![ // 283 
            ARG(2, true),
            INT(1),
            PTR(0, true, true),
        ], 
        vec![ // 284 
            COM(3,20),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp140
        vec![ // 285 
            ARG(0, true),
            INT(12),
            COM(1,0),
        ], 
        // AExp141
        vec![ // 286 
            ARG(1, true),
            INT(0),
            PTR(0, true, true),
        ], 
        vec![ // 287 
            COM(2,174),
            ARG(0, true),
        ], 
        // AExp142
        vec![ // 288 
            ARG(0, true),
            INT(11),
            COM(1,0),
        ], 
    ],

}});