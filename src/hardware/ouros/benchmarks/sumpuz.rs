use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 104
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
            COM(1,141),
            PTR(11, false, false),
        ], 
        vec![ // 11 
            COM(3,126),
            COM(1,149),
        ], 
        // AExp2
        vec![ // 12 
            COM(1,124),
            COM(1,95),
        ], 
        // AExp3
        vec![ // 13 
            Y,
            COM(3,38),
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
            COM(3,4),
            ARG(0, false),
            ARG(0, false),
            ARG(0, false),
        ], 
        // AExp3
        vec![ // 3 
            ARG(3, true),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp4
        vec![ // 4 
            COM(2,6),
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 5 
            COM(3,15),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp5
        vec![ // 6 
            COM(1,13),
            ARG(0, true),
            ARG(1, true),
            INT(0),
        ], 
        // AExp6
        vec![ // 7 
            PRM(ADD,false),
            PTR(0, true, true),
            ARG(1, true),
        ], 
        vec![ // 8 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp7
        vec![ // 9 
            ARG(1, true),
            ARG(4, true),
            PTR(0, true, true),
        ], 
        vec![ // 10 
            COM(3,7),
            ARG(0, true),
            ARG(2, true),
            ARG(3, true),
        ], 
        // AExp8
        vec![ // 11 
            ARG(2, true),
            ARG(3, false),
            PTR(0, true, true),
        ], 
        vec![ // 12 
            COM(5,9),
            ARG(0, true),
            ARG(1, true),
            ARG(3, false),
        ], 
        // AExp9
        vec![ // 13 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 14 
            COM(4,11),
            ARG(0, true),
        ], 
        // AExp10
        vec![ // 15 
            COM(2,6),
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 16 
            COM(3,17),
            ARG(2, true),
            ARG(1, true),
        ], 
        // AExp11
        vec![ // 17 
            COM(2,6),
            PTR(0, true, true),
            ARG(1, true),
        ], 
        vec![ // 18 
            COM(3,19),
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp12
        vec![ // 19 
            COM(3,32),
            ARG(0, true),
            ARG(1, true),
            ARG(2, true),
            INT(0),
            INT(1),
        ], 
        // AExp13
        vec![ // 20 
            PRM(EQ,false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 21 
            PTR(13, false, false),
            ARG(1, true),
        ], 
        vec![ // 22 
            PTR(13, false, false),
            ARG(0, true),
        ], 
        // AExp14
        vec![ // 23 
            PRM(EQ,false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 24 
            PTR(13, false, false),
            ARG(1, true),
        ], 
        vec![ // 25 
            PTR(13, false, false),
            ARG(0, true),
        ], 
        // AExp15
        vec![ // 26 
            ARG(0, true),
            INT(0),
            COM(2,0),
        ], 
        // AExp16
        vec![ // 27 
            COM(1,0),
            COM(1,42),
            PTR(0, true, true),
        ], 
        vec![ // 28 
            COM(4,91),
            ARG(0, true),
            ARG(1, true),
            ARG(2, true),
            COM(1,26),
        ], 
        // AExp17
        vec![ // 29 
            COM(1,35),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 30 
            COM(3,27),
            ARG(0, false),
            ARG(1, true),
            ARG(2, false),
        ], 
        vec![ // 31 
            COM(2,23),
            ARG(0, false),
            ARG(2, false),
        ], 
        // AExp18
        vec![ // 32 
            COM(1,35),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 33 
            COM(3,29),
            ARG(0, false),
            ARG(1, false),
            ARG(2, true),
        ], 
        vec![ // 34 
            COM(2,20),
            ARG(0, false),
            ARG(1, false),
        ], 
        // AExp19
        vec![ // 35 
            ARG(0, true),
            COM(2,0),
        ], 
        // AExp20
        vec![ // 36 
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 37 
            PRM(ADD,false),
            ARG(1, true),
            INT(1),
        ], 
        // AExp21
        vec![ // 38 
            ARG(2, true),
            ARG(1, false),
            PTR(0, true, true),
        ], 
        vec![ // 39 
            COM(3,36),
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp22
        vec![ // 40 
            COM(2,0),
        ], 
        // AExp23
        vec![ // 41 
            ARG(1, true),
            COM(2,1),
            COM(2,40),
        ], 
        // AExp24
        vec![ // 42 
            ARG(0, true),
            COM(2,0),
            COM(2,41),
        ], 
        // AExp25
        vec![ // 43 
            COM(4,3),
            PTR(0, true, true),
            COM(2,0),
        ], 
        vec![ // 44 
            COM(1,95),
            ARG(0, true),
        ], 
        // AExp26
        vec![ // 45 
            PRM(EQ,false),
            PTR(1, true, true),
            INT(0),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 46 
            COM(1,43),
            ARG(0, false),
        ], 
        vec![ // 47 
            COM(1,94),
            ARG(0, false),
        ], 
        // AExp27
        vec![ // 48 
            COM(3,106),
            ARG(1, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 49 
            COM(1,95),
            ARG(0, true),
        ], 
        vec![ // 50 
            COM(4,3),
            INT(1),
            COM(2,0),
        ], 
        // AExp28
        vec![ // 51 
            PRM(EQ,false),
            PTR(1, true, true),
            INT(1),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 52 
            COM(2,48),
            ARG(0, false),
            ARG(1, true),
        ], 
        vec![ // 53 
            COM(1,94),
            ARG(0, false),
        ], 
        // AExp29
        vec![ // 54 
            COM(2,0),
        ], 
        // AExp30
        vec![ // 55 
            ARG(2, true),
            PTR(0, true, true),
            COM(2,54),
        ], 
        vec![ // 56 
            COM(2,51),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp31
        vec![ // 57 
            ARG(0, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 58 
            COM(3,55),
            ARG(1, false),
        ], 
        vec![ // 59 
            COM(1,45),
            ARG(1, false),
        ], 
        // AExp32
        vec![ // 60 
            COM(1,0),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 61 
            COM(1,162),
            ARG(1, true),
            ARG(4, true),
        ], 
        vec![ // 62 
            ARG(0, true),
            ARG(2, true),
            ARG(3, true),
            ARG(5, true),
            ARG(6, true),
        ], 
        // AExp33
        vec![ // 63 
            COM(1,188),
            PTR(2, true, true),
            ARG(3, true),
            PTR(1, true, true),
            ARG(5, true),
            PTR(0, true, true),
        ], 
        vec![ // 64 
            ARG(2, true),
            ARG(4, true),
            ARG(6, true),
        ], 
        vec![ // 65 
            COM(1,203),
            ARG(1, true),
        ], 
        vec![ // 66 
            COM(1,94),
            ARG(0, true),
        ], 
        // AExp34
        vec![ // 67 
            COM(1,162),
            PTR(0, true, true),
        ], 
        vec![ // 68 
            COM(7,63),
            ARG(2, true),
            ARG(0, true),
            ARG(1, true),
            ARG(3, true),
            ARG(4, true),
            ARG(5, true),
            ARG(6, true),
        ], 
        // AExp35
        vec![ // 69 
            COM(7,60),
            PTR(1, true, true),
            ARG(1, true),
            ARG(5, false),
            ARG(6, false),
            PTR(0, true, true),
        ], 
        vec![ // 70 
            ARG(2, true),
            ARG(4, false),
            ARG(5, false),
            ARG(6, false),
        ], 
        vec![ // 71 
            COM(7,67),
            ARG(3, true),
            ARG(0, true),
            ARG(4, false),
        ], 
        // AExp36
        vec![ // 72 
            COM(4,91),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 73 
            COM(1,204),
            ARG(0, true),
        ], 
        // AExp37
        vec![ // 74 
            COM(3,106),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 75 
            COM(2,209),
            ARG(0, true),
            INT(9),
        ], 
        vec![ // 76 
            COM(1,203),
            ARG(1, true),
        ], 
        // AExp38
        vec![ // 77 
            COM(3,211),
            PTR(0, true, true),
            INT(1),
            INT(0),
        ], 
        vec![ // 78 
            COM(1,204),
            ARG(0, true),
        ], 
        // AExp39
        vec![ // 79 
            COM(2,74),
            PTR(0, true, true),
            ARG(0, false),
        ], 
        vec![ // 80 
            COM(1,77),
            ARG(0, false),
        ], 
        // AExp40
        vec![ // 81 
            COM(2,209),
            PTR(0, true, true),
            INT(9),
        ], 
        vec![ // 82 
            COM(3,211),
            ARG(0, true),
            INT(1),
            INT(0),
        ], 
        // AExp41
        vec![ // 83 
            COM(3,106),
            ARG(1, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 84 
            COM(1,95),
            ARG(0, true),
        ], 
        vec![ // 85 
            COM(1,81),
            ARG(2, true),
        ], 
        // AExp42
        vec![ // 86 
            COM(7,69),
            PTR(1, true, true),
            PTR(0, true, true),
            COM(3,83),
            ARG(0, false),
        ], 
        vec![ // 87 
            COM(1,79),
            ARG(0, false),
        ], 
        vec![ // 88 
            COM(2,72),
            ARG(0, false),
        ], 
        // AExp43
        vec![ // 89 
            ARG(1, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 90 
            COM(1,86),
            ARG(0, true),
            ARG(2, true),
            ARG(3, true),
            ARG(4, true),
        ], 
        // AExp44
        vec![ // 91 
            ARG(0, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 92 
            COM(5,89),
            ARG(1, true),
            ARG(2, false),
            ARG(3, false),
        ], 
        vec![ // 93 
            COM(2,57),
            ARG(2, false),
            ARG(3, false),
        ], 
        // AExp45
        vec![ // 94 
            ARG(0, true),
            COM(2,0),
        ], 
        // AExp46
        vec![ // 95 
            ARG(0, true),
            COM(2,1),
        ], 
        // AExp47
        vec![ // 96 
            PTR(10, false, false),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 97 
            PTR(12, false, false),
            ARG(1, true),
        ], 
        // AExp48
        vec![ // 98 
            COM(2,133),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 99 
            COM(2,96),
            ARG(1, true),
            ARG(2, true),
        ], 
        vec![ // 100 
            COM(1,135),
            ARG(0, true),
        ], 
        // AExp49
        vec![ // 101 
            COM(1,124),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 102 
            COM(3,98),
            ARG(0, true),
            ARG(1, true),
            ARG(2, false),
        ], 
        vec![ // 103 
            COM(3,126),
            COM(4,3),
            ARG(2, false),
        ], 
        // AExp50
        vec![ // 104 
            COM(1,155),
            ARG(2, true),
            ARG(0, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 105 
            COM(4,3),
            ARG(1, true),
            COM(2,0),
        ], 
        // AExp51
        vec![ // 106 
            COM(1,116),
            ARG(0, false),
            ARG(2, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 107 
            COM(3,104),
            ARG(1, false),
            ARG(2, false),
        ], 
        vec![ // 108 
            COM(3,101),
            ARG(0, false),
            ARG(1, false),
            ARG(2, false),
        ], 
        // AExp52
        vec![ // 109 
            PRM(EQ,false),
            ARG(0, true),
            ARG(3, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 110 
            COM(3,118),
            ARG(4, true),
        ], 
        vec![ // 111 
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp53
        vec![ // 112 
            ARG(2, true),
            PTR(0, true, true),
        ], 
        vec![ // 113 
            COM(5,109),
            ARG(0, true),
            ARG(1, true),
            ARG(3, true),
        ], 
        // AExp54
        vec![ // 114 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 115 
            COM(4,112),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp55
        vec![ // 116 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 117 
            COM(3,114),
            ARG(0, true),
        ], 
        // AExp56
        vec![ // 118 
            ARG(2, true),
            ARG(0, true),
        ], 
        // AExp57
        vec![ // 119 
            COM(4,3),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 120 
            ARG(1, true),
            ARG(3, true),
        ], 
        vec![ // 121 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp58
        vec![ // 122 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 123 
            COM(4,119),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp59
        vec![ // 124 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 125 
            COM(3,122),
            ARG(0, true),
        ], 
        // AExp60
        vec![ // 126 
            ARG(0, true),
            ARG(2, true),
            ARG(1, true),
        ], 
        // AExp61
        vec![ // 127 
            ARG(2, true),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp62
        vec![ // 128 
            COM(4,3),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 129 
            COM(2,133),
            ARG(1, true),
            ARG(3, true),
        ], 
        vec![ // 130 
            COM(3,127),
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp63
        vec![ // 131 
            ARG(0, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 132 
            COM(4,128),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp64
        vec![ // 133 
            ARG(0, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 134 
            COM(3,131),
            ARG(1, true),
        ], 
        // AExp65
        vec![ // 135 
            COM(4,3),
            ARG(0, false),
            PTR(0, true, true),
        ], 
        vec![ // 136 
            COM(1,135),
            ARG(0, false),
        ], 
        // AExp66
        vec![ // 137 
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 138 
            ARG(0, true),
            ARG(2, true),
            ARG(3, true),
        ], 
        // AExp67
        vec![ // 139 
            ARG(3, true),
            ARG(2, false),
            PTR(0, true, true),
        ], 
        vec![ // 140 
            COM(4,137),
            ARG(0, true),
            ARG(1, true),
            ARG(2, false),
        ], 
        // AExp68
        vec![ // 141 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 142 
            COM(4,139),
            ARG(0, true),
        ], 
        // AExp69
        vec![ // 143 
            COM(4,3),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 144 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp70
        vec![ // 145 
            PRM(EQ,false),
            ARG(0, true),
            ARG(2, false),
            PTR(0, true, true),
            ARG(3, false),
        ], 
        vec![ // 146 
            COM(3,143),
            ARG(1, true),
            ARG(2, false),
            ARG(3, false),
        ], 
        // AExp71
        vec![ // 147 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 148 
            COM(4,145),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp72
        vec![ // 149 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 150 
            COM(3,147),
            ARG(0, true),
        ], 
        // AExp73
        vec![ // 151 
            PRM(EQ,false),
            ARG(0, true),
            ARG(2, true),
            PTR(0, true, true),
            COM(2,1),
        ], 
        vec![ // 152 
            ARG(1, true),
            ARG(3, true),
        ], 
        // AExp74
        vec![ // 153 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 154 
            COM(4,151),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp75
        vec![ // 155 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 156 
            COM(3,153),
            ARG(0, true),
        ], 
        // AExp76
        vec![ // 157 
            COM(2,168),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 158 
            ARG(1, true),
            ARG(3, true),
        ], 
        vec![ // 159 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp77
        vec![ // 160 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 161 
            COM(4,157),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp78
        vec![ // 162 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 163 
            COM(3,160),
            ARG(0, true),
        ], 
        // AExp79
        vec![ // 164 
            COM(4,3),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 165 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp80
        vec![ // 166 
            ARG(2, true),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 167 
            COM(3,164),
            ARG(1, true),
        ], 
        // AExp81
        vec![ // 168 
            Y,
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 169 
            COM(3,166),
            ARG(1, true),
        ], 
        // AExp82
        vec![ // 170 
            ARG(0, true),
            ARG(4, true),
            ARG(5, true),
            ARG(6, false),
            PTR(0, true, true),
        ], 
        vec![ // 171 
            ARG(1, true),
            ARG(2, true),
            ARG(3, true),
            ARG(6, false),
        ], 
        // AExp83
        vec![ // 172 
            COM(3,190),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 173 
            COM(1,94),
            ARG(1, true),
        ], 
        // AExp84
        vec![ // 174 
            COM(4,3),
            PTR(0, true, true),
            COM(2,0),
        ], 
        vec![ // 175 
            COM(1,95),
            ARG(0, true),
        ], 
        // AExp85
        vec![ // 176 
            COM(3,106),
            ARG(0, true),
            PTR(0, true, true),
            ARG(1, true),
        ], 
        vec![ // 177 
            COM(1,174),
            ARG(2, true),
        ], 
        // AExp86
        vec![ // 178 
            COM(1,162),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 179 
            COM(3,176),
            ARG(0, true),
            ARG(2, true),
            ARG(3, false),
        ], 
        vec![ // 180 
            COM(2,172),
            ARG(1, true),
            ARG(3, false),
        ], 
        // AExp87
        vec![ // 181 
            PRM(ADD,false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 182 
            COM(2,200),
            ARG(2, false),
            ARG(1, true),
        ], 
        vec![ // 183 
            COM(2,200),
            ARG(2, false),
            ARG(0, true),
        ], 
        // AExp88
        vec![ // 184 
            PRM(ADD,false),
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 185 
            COM(3,181),
            ARG(1, true),
            ARG(2, true),
            ARG(3, true),
        ], 
        // AExp89
        vec![ // 186 
            COM(1,197),
            PTR(0, true, true),
        ], 
        vec![ // 187 
            COM(4,184),
            ARG(0, true),
            ARG(1, true),
            ARG(2, true),
            ARG(3, true),
        ], 
        // AExp90
        vec![ // 188 
            COM(7,170),
            COM(4,178),
            PTR(0, true, true),
        ], 
        vec![ // 189 
            COM(4,186),
            ARG(0, true),
        ], 
        // AExp91
        vec![ // 190 
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 191 
            COM(3,127),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp92
        vec![ // 192 
            ARG(2, true),
            PTR(0, true, true),
            ARG(1, true),
        ], 
        vec![ // 193 
            PRM(ADD,false),
            ARG(0, true),
            INT(1),
        ], 
        // AExp93
        vec![ // 194 
            COM(1,197),
            PTR(0, true, true),
            COM(3,192),
        ], 
        vec![ // 195 
            PRM(SUB,false),
            ARG(0, true),
            INT(10),
        ], 
        // AExp94
        vec![ // 196 
            ARG(1, true),
            INT(0),
            ARG(0, true),
        ], 
        // AExp95
        vec![ // 197 
            PRM(LE,false),
            ARG(0, false),
            INT(9),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 198 
            COM(2,196),
            ARG(0, false),
        ], 
        vec![ // 199 
            COM(1,194),
            ARG(0, false),
        ], 
        // AExp96
        vec![ // 200 
            COM(1,202),
            PTR(0, true, true),
        ], 
        vec![ // 201 
            COM(1,116),
            ARG(1, true),
            ARG(0, true),
        ], 
        // AExp97
        vec![ // 202 
            ARG(0, true),
            ERR(4),
            COM(1,0),
        ], 
        // AExp98
        vec![ // 203 
            ARG(0, true),
            ERR(3),
            COM(2,0),
        ], 
        // AExp99
        vec![ // 204 
            ARG(0, true),
            COM(2,0),
            COM(2,1),
        ], 
        // AExp100
        vec![ // 205 
            COM(2,209),
            PTR(0, true, true),
        ], 
        vec![ // 206 
            PRM(ADD,false),
            ARG(0, true),
            INT(1),
        ], 
        // AExp101
        vec![ // 207 
            COM(4,3),
            ARG(0, false),
            PTR(0, true, true),
        ], 
        vec![ // 208 
            COM(1,205),
            ARG(0, false),
            ARG(1, true),
        ], 
        // AExp102
        vec![ // 209 
            PRM(LE,false),
            ARG(0, false),
            ARG(1, false),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 210 
            COM(2,207),
            ARG(0, false),
            ARG(1, false),
        ], 
        // AExp103
        vec![ // 211 
            ARG(0, true),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 212 
            COM(3,0),
            ARG(2, true),
        ], 
    ],

}});