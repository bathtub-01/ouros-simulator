use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 105
#[rustfmt::skip]
pub static CLAUSIFY: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            COM(1,0),
            COM(1,5),
            PTR(18, false, false),
        ], 
        vec![ // 1 
            COM(5,147),
            INT(0),
        ], 
        vec![ // 2 
            COM(5,147),
            INT(0),
        ], 
        vec![ // 3 
            COM(2,217),
            PTR(2, false, false),
            PTR(1, false, false),
        ], 
        vec![ // 4 
            COM(5,147),
            INT(0),
        ], 
        vec![ // 5 
            COM(2,217),
            PTR(4, false, false),
        ], 
        vec![ // 6 
            COM(1,0),
            PTR(5, false, false),
            PTR(3, false, false),
        ], 
        vec![ // 7 
            COM(5,147),
            INT(0),
        ], 
        vec![ // 8 
            COM(5,147),
            INT(0),
        ], 
        vec![ // 9 
            COM(2,217),
            PTR(8, false, false),
            PTR(7, false, false),
        ], 
        vec![ // 10 
            COM(5,147),
            INT(0),
        ], 
        vec![ // 11 
            COM(2,217),
            PTR(10, false, false),
        ], 
        vec![ // 12 
            COM(1,0),
            PTR(11, false, false),
            PTR(9, false, false),
        ], 
        vec![ // 13 
            COM(2,217),
            PTR(12, false, false),
            PTR(6, false, false),
        ], 
        vec![ // 14 
            COM(2,211),
            INT(2),
            PTR(13, false, false),
        ], 
        vec![ // 15 
            COM(5,147),
            INT(0),
        ], 
        vec![ // 16 
            COM(2,14),
            COM(6,101),
            PTR(15, false, false),
        ], 
        vec![ // 17 
            COM(1,0),
            PTR(16, false, false),
            PTR(14, false, false),
        ], 
        vec![ // 18 
            COM(1,0),
            COM(1,24),
            PTR(17, false, false),
        ], 
        // AExp1
        vec![ // 19 
            COM(1,26),
            PTR(21, false, false),
        ], 
        vec![ // 20 
            COM(2,33),
            COM(2,65),
        ], 
        vec![ // 21 
            COM(3,27),
            PTR(20, false, false),
            COM(1,79),
        ], 
        // AExp2
        vec![ // 22 
            COM(1,49),
            COM(1,82),
        ], 
        // AExp3
        vec![ // 23 
            COM(1,93),
            PTR(24, false, false),
        ], 
        vec![ // 24 
            COM(2,117),
            COM(1,87),
        ], 
        // AExp4
        vec![ // 25 
            COM(1,144),
            COM(2,0),
        ], 
        // AExp5
        vec![ // 26 
            COM(2,14),
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
            PRM(ADD,false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 3 
            COM(1,5),
            ARG(1, true),
        ], 
        vec![ // 4 
            COM(1,9),
            ARG(0, true),
        ], 
        // AExp3
        vec![ // 5 
            ARG(0, true),
            INT(0),
            COM(2,2),
        ], 
        // AExp4
        vec![ // 6 
            PRM(ADD,false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 7 
            PTR(26, false, false),
            ARG(1, true),
        ], 
        vec![ // 8 
            PTR(26, false, false),
            ARG(0, true),
        ], 
        // AExp5
        vec![ // 9 
            ARG(0, true),
            COM(2,6),
        ], 
        // AExp6
        vec![ // 10 
            ARG(0, true),
            ARG(2, true),
            PTR(0, true, true),
        ], 
        vec![ // 11 
            ARG(1, true),
            ARG(3, true),
        ], 
        // AExp7
        vec![ // 12 
            ARG(3, true),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 13 
            COM(4,10),
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp8
        vec![ // 14 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 15 
            COM(4,12),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp9
        vec![ // 16 
            COM(1,0),
            COM(1,154),
            PTR(0, true, true),
        ], 
        vec![ // 17 
            COM(1,206),
            ARG(0, true),
        ], 
        // AExp10
        vec![ // 18 
            COM(1,0),
            PTR(25, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 19 
            COM(1,16),
            ARG(0, true),
        ], 
        // AExp11
        vec![ // 20 
            COM(1,0),
            PTR(23, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 21 
            COM(1,18),
            ARG(0, true),
        ], 
        // AExp12
        vec![ // 22 
            COM(1,0),
            PTR(22, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 23 
            COM(1,20),
            ARG(0, true),
        ], 
        // AExp13
        vec![ // 24 
            COM(1,0),
            PTR(19, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 25 
            COM(1,22),
            ARG(0, true),
        ], 
        // AExp14
        vec![ // 26 
            COM(2,14),
            ARG(0, true),
            COM(2,0),
        ], 
        // AExp15
        vec![ // 27 
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 28 
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp16
        vec![ // 29 
            COM(2,39),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 30 
            COM(1,49),
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp17
        vec![ // 31 
            COM(3,27),
            COM(1,51),
            PTR(0, true, true),
        ], 
        vec![ // 32 
            COM(1,57),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp18
        vec![ // 33 
            COM(3,29),
            PTR(0, true, true),
            ARG(1, false),
        ], 
        vec![ // 34 
            COM(2,31),
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp19
        vec![ // 35 
            COM(4,41),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 36 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp20
        vec![ // 37 
            ARG(2, true),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 38 
            COM(3,35),
            ARG(1, true),
        ], 
        // AExp21
        vec![ // 39 
            Y,
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 40 
            COM(3,37),
            ARG(1, true),
        ], 
        // AExp22
        vec![ // 41 
            ARG(3, true),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp23
        vec![ // 42 
            COM(4,41),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 43 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp24
        vec![ // 44 
            ARG(0, true),
            ARG(2, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 45 
            COM(3,42),
            ARG(1, false),
            ARG(2, false),
            ARG(3, false),
        ], 
        vec![ // 46 
            ARG(1, false),
            ARG(3, false),
        ], 
        // AExp25
        vec![ // 47 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 48 
            COM(4,44),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp26
        vec![ // 49 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 50 
            COM(3,47),
            ARG(0, true),
        ], 
        // AExp27
        vec![ // 51 
            ARG(0, true),
            COM(2,1),
            COM(2,0),
        ], 
        // AExp28
        vec![ // 52 
            COM(2,59),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 53 
            ARG(1, true),
            ARG(4, true),
            ARG(2, false),
        ], 
        vec![ // 54 
            ARG(0, true),
            ARG(3, true),
            ARG(2, false),
        ], 
        // AExp29
        vec![ // 55 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 56 
            COM(5,52),
            ARG(0, true),
            ARG(1, true),
            ARG(3, true),
        ], 
        // AExp30
        vec![ // 57 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 58 
            COM(4,55),
            ARG(0, true),
        ], 
        // AExp31
        vec![ // 59 
            ARG(0, true),
            ARG(1, true),
            COM(2,1),
        ], 
        // AExp32
        vec![ // 60 
            COM(1,67),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 61 
            COM(1,77),
            PRM(EQ,false),
            ARG(1, true),
            ARG(3, true),
        ], 
        vec![ // 62 
            COM(1,77),
            PRM(EQ,false),
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp33
        vec![ // 63 
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 64 
            COM(4,60),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp34
        vec![ // 65 
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 66 
            COM(3,63),
            ARG(1, true),
        ], 
        // AExp35
        vec![ // 67 
            ARG(0, true),
            COM(2,0),
        ], 
        // AExp36
        vec![ // 68 
            COM(2,0),
        ], 
        // AExp37
        vec![ // 69 
            COM(1,67),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 70 
            ARG(1, true),
            ARG(3, true),
            ARG(5, true),
        ], 
        vec![ // 71 
            ARG(0, true),
            ARG(2, true),
            ARG(4, true),
        ], 
        // AExp38
        vec![ // 72 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 73 
            COM(6,69),
            ARG(0, true),
            ARG(1, true),
            ARG(3, true),
            ARG(4, true),
        ], 
        // AExp39
        vec![ // 74 
            ARG(2, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 75 
            COM(5,72),
            ARG(0, true),
            ARG(1, true),
            ARG(3, false),
        ], 
        vec![ // 76 
            ARG(3, false),
            COM(2,1),
            COM(2,68),
        ], 
        // AExp40
        vec![ // 77 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 78 
            COM(4,74),
            ARG(0, true),
        ], 
        // AExp41
        vec![ // 79 
            COM(4,41),
            ARG(0, true),
            COM(2,0),
        ], 
        // AExp42
        vec![ // 80 
            COM(1,84),
            PTR(0, true, true),
        ], 
        vec![ // 81 
            COM(2,85),
            PRM(EQ,false),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp43
        vec![ // 82 
            ARG(0, true),
            COM(2,80),
        ], 
        // AExp44
        vec![ // 83 
            COM(2,0),
        ], 
        // AExp45
        vec![ // 84 
            ARG(0, true),
            COM(2,1),
            COM(2,83),
        ], 
        // AExp46
        vec![ // 85 
            COM(1,49),
            PTR(0, true, true),
        ], 
        vec![ // 86 
            COM(1,57),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp47
        vec![ // 87 
            ARG(0, true),
            COM(2,0),
            COM(2,0),
        ], 
        // AExp48
        vec![ // 88 
            COM(4,41),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 89 
            ARG(1, true),
            ARG(3, true),
        ], 
        vec![ // 90 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp49
        vec![ // 91 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 92 
            COM(4,88),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp50
        vec![ // 93 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 94 
            COM(3,91),
            ARG(0, true),
        ], 
        // AExp51
        vec![ // 95 
            ARG(0, true),
            ARG(1, true),
            PTR(2, true, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 96 
            ARG(4, true),
            ARG(5, false),
            ARG(6, false),
        ], 
        vec![ // 97 
            ARG(3, true),
            ARG(5, false),
            ARG(6, false),
        ], 
        vec![ // 98 
            ARG(2, true),
            ARG(5, false),
            ARG(6, false),
        ], 
        // AExp52
        vec![ // 99 
            COM(2,117),
            PTR(0, true, true),
        ], 
        vec![ // 100 
            COM(2,117),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp53
        vec![ // 101 
            ARG(2, true),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp54
        vec![ // 102 
            COM(2,99),
            PTR(0, true, true),
        ], 
        vec![ // 103 
            COM(3,101),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp55
        vec![ // 104 
            ARG(5, true),
            ARG(0, false),
            ARG(0, false),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 105 
            ARG(2, true),
            ARG(3, true),
            ARG(4, true),
        ], 
        // AExp56
        vec![ // 106 
            ARG(3, true),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 107 
            COM(1,129),
            ARG(2, true),
            ARG(1, true),
        ], 
        // AExp57
        vec![ // 108 
            COM(6,104),
            PTR(1, true, true),
            PTR(0, true, true),
            COM(4,106),
        ], 
        vec![ // 109 
            COM(2,0),
            ARG(0, false),
        ], 
        vec![ // 110 
            COM(3,0),
            ARG(0, false),
        ], 
        // AExp58
        vec![ // 111 
            ARG(3, true),
            PTR(0, true, true),
            ARG(1, true),
        ], 
        vec![ // 112 
            COM(1,129),
            ARG(2, true),
            ARG(0, true),
        ], 
        // AExp59
        vec![ // 113 
            COM(7,95),
            ARG(0, true),
            PTR(1, true, true),
            COM(2,102),
            PTR(0, true, true),
            COM(4,111),
        ], 
        vec![ // 114 
            COM(1,108),
            ARG(1, false),
        ], 
        vec![ // 115 
            COM(3,0),
            ARG(1, false),
        ], 
        // AExp60
        vec![ // 116 
            ARG(0, true),
            COM(2,0),
            COM(2,0),
        ], 
        // AExp61
        vec![ // 117 
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 118 
            COM(2,113),
            ARG(1, true),
            COM(1,116),
        ], 
        // AExp62
        vec![ // 119 
            COM(4,41),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 120 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp63
        vec![ // 121 
            COM(4,41),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 122 
            COM(4,41),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp64
        vec![ // 123 
            PRM(LE,false),
            ARG(0, false),
            ARG(2, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 124 
            COM(3,121),
            ARG(0, false),
            ARG(2, false),
            ARG(3, false),
        ], 
        vec![ // 125 
            COM(3,119),
            ARG(1, true),
            ARG(2, false),
            ARG(3, false),
        ], 
        // AExp65
        vec![ // 126 
            ARG(2, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 127 
            COM(4,123),
            ARG(0, false),
            ARG(1, true),
        ], 
        vec![ // 128 
            COM(4,41),
            ARG(0, false),
            COM(2,0),
        ], 
        // AExp66
        vec![ // 129 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 130 
            COM(3,126),
            ARG(0, true),
        ], 
        // AExp67
        vec![ // 131 
            COM(1,144),
            PTR(0, true, true),
        ], 
        vec![ // 132 
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp68
        vec![ // 133 
            COM(4,41),
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 134 
            COM(6,41),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp69
        vec![ // 135 
            COM(4,41),
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 136 
            COM(5,146),
            ARG(1, true),
        ], 
        // AExp70
        vec![ // 137 
            COM(4,41),
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 138 
            COM(5,147),
            ARG(1, true),
        ], 
        // AExp71
        vec![ // 139 
            ARG(2, true),
            PTR(3, true, true),
            PTR(2, true, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 140 
            COM(2,137),
            ARG(0, false),
        ], 
        vec![ // 141 
            COM(2,135),
            ARG(0, false),
        ], 
        vec![ // 142 
            COM(3,133),
            ARG(0, false),
        ], 
        vec![ // 143 
            COM(2,131),
            ARG(1, true),
        ], 
        // AExp72
        vec![ // 144 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 145 
            COM(3,139),
            ARG(0, true),
        ], 
        // AExp73
        vec![ // 146 
            ARG(3, true),
            ARG(0, true),
        ], 
        // AExp74
        vec![ // 147 
            ARG(4, true),
            ARG(0, true),
        ], 
        // AExp75
        vec![ // 148 
            COM(6,101),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 149 
            COM(1,154),
            ARG(1, true),
        ], 
        vec![ // 150 
            COM(1,154),
            ARG(0, true),
        ], 
        // AExp76
        vec![ // 151 
            COM(2,164),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 152 
            COM(1,154),
            ARG(1, true),
        ], 
        vec![ // 153 
            COM(1,154),
            ARG(0, true),
        ], 
        // AExp77
        vec![ // 154 
            ARG(0, true),
            COM(2,148),
            COM(2,151),
            COM(5,146),
            COM(5,147),
        ], 
        // AExp78
        vec![ // 155 
            COM(6,101),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 156 
            COM(2,164),
            ARG(2, true),
            ARG(0, false),
        ], 
        vec![ // 157 
            COM(2,164),
            ARG(1, true),
            ARG(0, false),
        ], 
        // AExp79
        vec![ // 158 
            COM(2,178),
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 159 
            COM(6,41),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp80
        vec![ // 160 
            COM(2,178),
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 161 
            COM(5,146),
            ARG(1, true),
        ], 
        // AExp81
        vec![ // 162 
            COM(2,178),
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 163 
            COM(5,147),
            ARG(1, true),
        ], 
        // AExp82
        vec![ // 164 
            ARG(0, true),
            PTR(3, true, true),
            PTR(2, true, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 165 
            COM(2,162),
            ARG(1, false),
        ], 
        vec![ // 166 
            COM(2,160),
            ARG(1, false),
        ], 
        vec![ // 167 
            COM(3,158),
            ARG(1, false),
        ], 
        vec![ // 168 
            COM(3,155),
            ARG(1, false),
        ], 
        // AExp83
        vec![ // 169 
            COM(6,101),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 170 
            COM(2,164),
            ARG(0, false),
            ARG(2, true),
        ], 
        vec![ // 171 
            COM(2,164),
            ARG(0, false),
            ARG(1, true),
        ], 
        // AExp84
        vec![ // 172 
            COM(6,41),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 173 
            COM(6,41),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp85
        vec![ // 174 
            COM(6,41),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 175 
            COM(5,146),
            ARG(1, true),
        ], 
        // AExp86
        vec![ // 176 
            COM(6,41),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 177 
            COM(5,147),
            ARG(1, true),
        ], 
        // AExp87
        vec![ // 178 
            ARG(1, true),
            PTR(3, true, true),
            PTR(2, true, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 179 
            COM(2,176),
            ARG(0, false),
        ], 
        vec![ // 180 
            COM(2,174),
            ARG(0, false),
        ], 
        vec![ // 181 
            COM(3,172),
            ARG(0, false),
        ], 
        vec![ // 182 
            COM(3,169),
            ARG(0, false),
        ], 
        // AExp88
        vec![ // 183 
            COM(6,101),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 184 
            COM(1,206),
            ARG(1, true),
        ], 
        vec![ // 185 
            COM(1,206),
            ARG(0, true),
        ], 
        // AExp89
        vec![ // 186 
            COM(6,41),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 187 
            COM(1,206),
            ARG(1, true),
        ], 
        vec![ // 188 
            COM(1,206),
            ARG(0, true),
        ], 
        // AExp90
        vec![ // 189 
            COM(1,206),
            PTR(0, true, true),
        ], 
        vec![ // 190 
            COM(5,146),
            ARG(0, true),
        ], 
        // AExp91
        vec![ // 191 
            COM(1,206),
            PTR(0, true, true),
        ], 
        vec![ // 192 
            COM(5,146),
            ARG(0, true),
        ], 
        // AExp92
        vec![ // 193 
            COM(6,41),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 194 
            COM(1,191),
            ARG(1, true),
        ], 
        vec![ // 195 
            COM(1,189),
            ARG(0, true),
        ], 
        // AExp93
        vec![ // 196 
            COM(1,206),
            PTR(0, true, true),
        ], 
        vec![ // 197 
            COM(5,146),
            ARG(0, true),
        ], 
        // AExp94
        vec![ // 198 
            COM(1,206),
            PTR(0, true, true),
        ], 
        vec![ // 199 
            COM(5,146),
            ARG(0, true),
        ], 
        // AExp95
        vec![ // 200 
            COM(6,101),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 201 
            COM(1,198),
            ARG(1, true),
        ], 
        vec![ // 202 
            COM(1,196),
            ARG(0, true),
        ], 
        // AExp96
        vec![ // 203 
            COM(5,146),
            PTR(0, true, true),
        ], 
        vec![ // 204 
            COM(5,147),
            ARG(0, true),
        ], 
        // AExp97
        vec![ // 205 
            ARG(0, true),
            COM(2,193),
            COM(2,200),
            COM(1,206),
            COM(1,203),
        ], 
        // AExp98
        vec![ // 206 
            ARG(0, true),
            COM(2,183),
            COM(2,186),
            COM(1,205),
            COM(5,147),
        ], 
        // AExp99
        vec![ // 207 
            COM(2,211),
            PTR(0, true, true),
        ], 
        vec![ // 208 
            PRM(SUB,false),
            ARG(0, true),
            INT(1),
        ], 
        // AExp100
        vec![ // 209 
            COM(4,41),
            ARG(1, false),
            PTR(0, true, true),
        ], 
        vec![ // 210 
            COM(1,207),
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp101
        vec![ // 211 
            PRM(LE,false),
            ARG(0, false),
            INT(0),
            PTR(0, true, true),
            COM(2,0),
        ], 
        vec![ // 212 
            COM(2,209),
            ARG(0, false),
            ARG(1, true),
        ], 
        // AExp102
        vec![ // 213 
            COM(6,41),
            PTR(0, true, true),
        ], 
        vec![ // 214 
            COM(5,146),
            ARG(0, true),
        ], 
        // AExp103
        vec![ // 215 
            COM(6,41),
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 216 
            COM(5,146),
            ARG(1, true),
        ], 
        // AExp104
        vec![ // 217 
            COM(6,101),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 218 
            COM(2,215),
            ARG(0, false),
            ARG(1, false),
        ], 
        vec![ // 219 
            COM(1,213),
            ARG(0, false),
            ARG(1, false),
        ], 
    ],

}});