use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 106
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
            COM(1,145),
            PTR(11, false, false),
        ], 
        vec![ // 11 
            COM(3,130),
            COM(1,153),
        ], 
        // AExp2
        vec![ // 12 
            COM(1,128),
            COM(1,99),
        ], 
        // AExp3
        vec![ // 13 
            Y,
            COM(3,40),
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
            COM(3,34),
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
            SEQ(false),
            ARG(2, false),
            PTR(0, true, true),
        ], 
        vec![ // 28 
            COM(4,95),
            ARG(0, true),
            ARG(1, true),
            ARG(2, false),
            COM(1,26),
        ], 
        // AExp17
        vec![ // 29 
            COM(1,0),
            COM(1,44),
            PTR(0, true, true),
        ], 
        vec![ // 30 
            COM(3,27),
            ARG(0, true),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp18
        vec![ // 31 
            COM(1,37),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 32 
            COM(3,29),
            ARG(0, false),
            ARG(1, true),
            ARG(2, false),
        ], 
        vec![ // 33 
            COM(2,23),
            ARG(0, false),
            ARG(2, false),
        ], 
        // AExp19
        vec![ // 34 
            COM(1,37),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 35 
            COM(3,31),
            ARG(0, false),
            ARG(1, false),
            ARG(2, true),
        ], 
        vec![ // 36 
            COM(2,20),
            ARG(0, false),
            ARG(1, false),
        ], 
        // AExp20
        vec![ // 37 
            ARG(0, true),
            COM(2,0),
        ], 
        // AExp21
        vec![ // 38 
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 39 
            PRM(ADD,false),
            ARG(1, true),
            INT(1),
        ], 
        // AExp22
        vec![ // 40 
            ARG(2, true),
            ARG(1, false),
            PTR(0, true, true),
        ], 
        vec![ // 41 
            COM(3,38),
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp23
        vec![ // 42 
            COM(2,0),
        ], 
        // AExp24
        vec![ // 43 
            ARG(1, true),
            COM(2,1),
            COM(2,42),
        ], 
        // AExp25
        vec![ // 44 
            ARG(0, true),
            COM(2,0),
            COM(2,43),
        ], 
        // AExp26
        vec![ // 45 
            COM(4,3),
            PTR(0, true, true),
            COM(2,0),
        ], 
        vec![ // 46 
            COM(1,99),
            ARG(0, true),
        ], 
        // AExp27
        vec![ // 47 
            PRM(EQ,false),
            PTR(1, true, true),
            INT(0),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 48 
            COM(1,45),
            ARG(0, false),
        ], 
        vec![ // 49 
            COM(1,98),
            ARG(0, false),
        ], 
        // AExp28
        vec![ // 50 
            COM(3,110),
            ARG(1, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 51 
            COM(1,99),
            ARG(0, true),
        ], 
        vec![ // 52 
            COM(4,3),
            INT(1),
            COM(2,0),
        ], 
        // AExp29
        vec![ // 53 
            PRM(EQ,false),
            PTR(1, true, true),
            INT(1),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 54 
            COM(2,50),
            ARG(0, false),
            ARG(1, true),
        ], 
        vec![ // 55 
            COM(1,98),
            ARG(0, false),
        ], 
        // AExp30
        vec![ // 56 
            COM(2,0),
        ], 
        // AExp31
        vec![ // 57 
            ARG(2, true),
            PTR(0, true, true),
            COM(2,56),
        ], 
        vec![ // 58 
            COM(2,53),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp32
        vec![ // 59 
            ARG(0, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 60 
            COM(3,57),
            ARG(1, false),
        ], 
        vec![ // 61 
            COM(1,47),
            ARG(1, false),
        ], 
        // AExp33
        vec![ // 62 
            COM(1,0),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 63 
            COM(1,166),
            ARG(1, true),
            ARG(4, true),
        ], 
        vec![ // 64 
            ARG(0, true),
            ARG(2, true),
            ARG(3, true),
            ARG(5, true),
            ARG(6, true),
        ], 
        // AExp34
        vec![ // 65 
            COM(1,192),
            PTR(2, true, true),
            ARG(3, true),
            PTR(1, true, true),
            ARG(5, true),
            PTR(0, true, true),
        ], 
        vec![ // 66 
            ARG(2, true),
            ARG(4, true),
            ARG(6, true),
        ], 
        vec![ // 67 
            COM(1,207),
            ARG(1, true),
        ], 
        vec![ // 68 
            COM(1,98),
            ARG(0, true),
        ], 
        // AExp35
        vec![ // 69 
            COM(1,166),
            PTR(0, true, true),
        ], 
        vec![ // 70 
            COM(7,65),
            ARG(2, true),
            ARG(0, true),
            ARG(1, true),
            ARG(3, true),
            ARG(4, true),
            ARG(5, true),
            ARG(6, true),
        ], 
        // AExp36
        vec![ // 71 
            COM(7,62),
            PTR(1, true, true),
            ARG(1, true),
            ARG(5, false),
            ARG(6, false),
            PTR(0, true, true),
        ], 
        vec![ // 72 
            ARG(2, true),
            ARG(4, false),
            ARG(5, false),
            ARG(6, false),
        ], 
        vec![ // 73 
            COM(7,69),
            ARG(3, true),
            ARG(0, true),
            ARG(4, false),
        ], 
        // AExp37
        vec![ // 74 
            COM(4,95),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 75 
            COM(1,208),
            ARG(0, true),
        ], 
        // AExp38
        vec![ // 76 
            SEQ(false),
            ARG(2, false),
            PTR(0, true, true),
        ], 
        vec![ // 77 
            COM(2,74),
            ARG(0, true),
            ARG(1, true),
            ARG(2, false),
        ], 
        // AExp39
        vec![ // 78 
            COM(3,110),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 79 
            COM(2,213),
            ARG(0, true),
            INT(9),
        ], 
        vec![ // 80 
            COM(1,207),
            ARG(1, true),
        ], 
        // AExp40
        vec![ // 81 
            COM(3,215),
            PTR(0, true, true),
            INT(1),
            INT(0),
        ], 
        vec![ // 82 
            COM(1,208),
            ARG(0, true),
        ], 
        // AExp41
        vec![ // 83 
            COM(2,78),
            PTR(0, true, true),
            ARG(0, false),
        ], 
        vec![ // 84 
            COM(1,81),
            ARG(0, false),
        ], 
        // AExp42
        vec![ // 85 
            COM(2,213),
            PTR(0, true, true),
            INT(9),
        ], 
        vec![ // 86 
            COM(3,215),
            ARG(0, true),
            INT(1),
            INT(0),
        ], 
        // AExp43
        vec![ // 87 
            COM(3,110),
            ARG(1, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 88 
            COM(1,99),
            ARG(0, true),
        ], 
        vec![ // 89 
            COM(1,85),
            ARG(2, true),
        ], 
        // AExp44
        vec![ // 90 
            COM(7,71),
            PTR(1, true, true),
            PTR(0, true, true),
            COM(3,87),
            ARG(0, false),
        ], 
        vec![ // 91 
            COM(1,83),
            ARG(0, false),
        ], 
        vec![ // 92 
            COM(3,76),
            ARG(0, false),
        ], 
        // AExp45
        vec![ // 93 
            ARG(1, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 94 
            COM(1,90),
            ARG(0, true),
            ARG(2, true),
            ARG(3, true),
            ARG(4, true),
        ], 
        // AExp46
        vec![ // 95 
            ARG(0, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 96 
            COM(5,93),
            ARG(1, true),
            ARG(2, false),
            ARG(3, false),
        ], 
        vec![ // 97 
            COM(2,59),
            ARG(2, false),
            ARG(3, false),
        ], 
        // AExp47
        vec![ // 98 
            ARG(0, true),
            COM(2,0),
        ], 
        // AExp48
        vec![ // 99 
            ARG(0, true),
            COM(2,1),
        ], 
        // AExp49
        vec![ // 100 
            PTR(10, false, false),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 101 
            PTR(12, false, false),
            ARG(1, true),
        ], 
        // AExp50
        vec![ // 102 
            COM(2,137),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 103 
            COM(2,100),
            ARG(1, true),
            ARG(2, true),
        ], 
        vec![ // 104 
            COM(1,139),
            ARG(0, true),
        ], 
        // AExp51
        vec![ // 105 
            COM(1,128),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 106 
            COM(3,102),
            ARG(0, true),
            ARG(1, true),
            ARG(2, false),
        ], 
        vec![ // 107 
            COM(3,130),
            COM(4,3),
            ARG(2, false),
        ], 
        // AExp52
        vec![ // 108 
            COM(1,159),
            ARG(2, true),
            ARG(0, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 109 
            COM(4,3),
            ARG(1, true),
            COM(2,0),
        ], 
        // AExp53
        vec![ // 110 
            COM(1,120),
            ARG(0, false),
            ARG(2, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 111 
            COM(3,108),
            ARG(1, false),
            ARG(2, false),
        ], 
        vec![ // 112 
            COM(3,105),
            ARG(0, false),
            ARG(1, false),
            ARG(2, false),
        ], 
        // AExp54
        vec![ // 113 
            PRM(EQ,false),
            ARG(0, true),
            ARG(3, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 114 
            COM(3,122),
            ARG(4, true),
        ], 
        vec![ // 115 
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp55
        vec![ // 116 
            ARG(2, true),
            PTR(0, true, true),
        ], 
        vec![ // 117 
            COM(5,113),
            ARG(0, true),
            ARG(1, true),
            ARG(3, true),
        ], 
        // AExp56
        vec![ // 118 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 119 
            COM(4,116),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp57
        vec![ // 120 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 121 
            COM(3,118),
            ARG(0, true),
        ], 
        // AExp58
        vec![ // 122 
            ARG(2, true),
            ARG(0, true),
        ], 
        // AExp59
        vec![ // 123 
            COM(4,3),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 124 
            ARG(1, true),
            ARG(3, true),
        ], 
        vec![ // 125 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp60
        vec![ // 126 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 127 
            COM(4,123),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp61
        vec![ // 128 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 129 
            COM(3,126),
            ARG(0, true),
        ], 
        // AExp62
        vec![ // 130 
            ARG(0, true),
            ARG(2, true),
            ARG(1, true),
        ], 
        // AExp63
        vec![ // 131 
            ARG(2, true),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp64
        vec![ // 132 
            COM(4,3),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 133 
            COM(2,137),
            ARG(1, true),
            ARG(3, true),
        ], 
        vec![ // 134 
            COM(3,131),
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp65
        vec![ // 135 
            ARG(0, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 136 
            COM(4,132),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp66
        vec![ // 137 
            ARG(0, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 138 
            COM(3,135),
            ARG(1, true),
        ], 
        // AExp67
        vec![ // 139 
            COM(4,3),
            ARG(0, false),
            PTR(0, true, true),
        ], 
        vec![ // 140 
            COM(1,139),
            ARG(0, false),
        ], 
        // AExp68
        vec![ // 141 
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 142 
            ARG(0, true),
            ARG(2, true),
            ARG(3, true),
        ], 
        // AExp69
        vec![ // 143 
            ARG(3, true),
            ARG(2, false),
            PTR(0, true, true),
        ], 
        vec![ // 144 
            COM(4,141),
            ARG(0, true),
            ARG(1, true),
            ARG(2, false),
        ], 
        // AExp70
        vec![ // 145 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 146 
            COM(4,143),
            ARG(0, true),
        ], 
        // AExp71
        vec![ // 147 
            COM(4,3),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 148 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp72
        vec![ // 149 
            PRM(EQ,false),
            ARG(0, true),
            ARG(2, false),
            PTR(0, true, true),
            ARG(3, false),
        ], 
        vec![ // 150 
            COM(3,147),
            ARG(1, true),
            ARG(2, false),
            ARG(3, false),
        ], 
        // AExp73
        vec![ // 151 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 152 
            COM(4,149),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp74
        vec![ // 153 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 154 
            COM(3,151),
            ARG(0, true),
        ], 
        // AExp75
        vec![ // 155 
            PRM(EQ,false),
            ARG(0, true),
            ARG(2, true),
            PTR(0, true, true),
            COM(2,1),
        ], 
        vec![ // 156 
            ARG(1, true),
            ARG(3, true),
        ], 
        // AExp76
        vec![ // 157 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 158 
            COM(4,155),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp77
        vec![ // 159 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 160 
            COM(3,157),
            ARG(0, true),
        ], 
        // AExp78
        vec![ // 161 
            COM(2,172),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 162 
            ARG(1, true),
            ARG(3, true),
        ], 
        vec![ // 163 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp79
        vec![ // 164 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 165 
            COM(4,161),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp80
        vec![ // 166 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 167 
            COM(3,164),
            ARG(0, true),
        ], 
        // AExp81
        vec![ // 168 
            COM(4,3),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 169 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp82
        vec![ // 170 
            ARG(2, true),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 171 
            COM(3,168),
            ARG(1, true),
        ], 
        // AExp83
        vec![ // 172 
            Y,
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 173 
            COM(3,170),
            ARG(1, true),
        ], 
        // AExp84
        vec![ // 174 
            ARG(0, true),
            ARG(4, true),
            ARG(5, true),
            ARG(6, false),
            PTR(0, true, true),
        ], 
        vec![ // 175 
            ARG(1, true),
            ARG(2, true),
            ARG(3, true),
            ARG(6, false),
        ], 
        // AExp85
        vec![ // 176 
            COM(3,194),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 177 
            COM(1,98),
            ARG(1, true),
        ], 
        // AExp86
        vec![ // 178 
            COM(4,3),
            PTR(0, true, true),
            COM(2,0),
        ], 
        vec![ // 179 
            COM(1,99),
            ARG(0, true),
        ], 
        // AExp87
        vec![ // 180 
            COM(3,110),
            ARG(0, true),
            PTR(0, true, true),
            ARG(1, true),
        ], 
        vec![ // 181 
            COM(1,178),
            ARG(2, true),
        ], 
        // AExp88
        vec![ // 182 
            COM(1,166),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 183 
            COM(3,180),
            ARG(0, true),
            ARG(2, true),
            ARG(3, false),
        ], 
        vec![ // 184 
            COM(2,176),
            ARG(1, true),
            ARG(3, false),
        ], 
        // AExp89
        vec![ // 185 
            PRM(ADD,false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 186 
            COM(2,204),
            ARG(2, false),
            ARG(1, true),
        ], 
        vec![ // 187 
            COM(2,204),
            ARG(2, false),
            ARG(0, true),
        ], 
        // AExp90
        vec![ // 188 
            PRM(ADD,false),
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 189 
            COM(3,185),
            ARG(1, true),
            ARG(2, true),
            ARG(3, true),
        ], 
        // AExp91
        vec![ // 190 
            COM(1,201),
            PTR(0, true, true),
        ], 
        vec![ // 191 
            COM(4,188),
            ARG(0, true),
            ARG(1, true),
            ARG(2, true),
            ARG(3, true),
        ], 
        // AExp92
        vec![ // 192 
            COM(7,174),
            COM(4,182),
            PTR(0, true, true),
        ], 
        vec![ // 193 
            COM(4,190),
            ARG(0, true),
        ], 
        // AExp93
        vec![ // 194 
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 195 
            COM(3,131),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp94
        vec![ // 196 
            ARG(2, true),
            PTR(0, true, true),
            ARG(1, true),
        ], 
        vec![ // 197 
            PRM(ADD,false),
            ARG(0, true),
            INT(1),
        ], 
        // AExp95
        vec![ // 198 
            COM(1,201),
            PTR(0, true, true),
            COM(3,196),
        ], 
        vec![ // 199 
            PRM(SUB,false),
            ARG(0, true),
            INT(10),
        ], 
        // AExp96
        vec![ // 200 
            ARG(1, true),
            INT(0),
            ARG(0, true),
        ], 
        // AExp97
        vec![ // 201 
            PRM(LE,false),
            ARG(0, false),
            INT(9),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 202 
            COM(2,200),
            ARG(0, false),
        ], 
        vec![ // 203 
            COM(1,198),
            ARG(0, false),
        ], 
        // AExp98
        vec![ // 204 
            COM(1,206),
            PTR(0, true, true),
        ], 
        vec![ // 205 
            COM(1,120),
            ARG(1, true),
            ARG(0, true),
        ], 
        // AExp99
        vec![ // 206 
            ARG(0, true),
            ERR(4),
            COM(1,0),
        ], 
        // AExp100
        vec![ // 207 
            ARG(0, true),
            ERR(3),
            COM(2,0),
        ], 
        // AExp101
        vec![ // 208 
            ARG(0, true),
            COM(2,0),
            COM(2,1),
        ], 
        // AExp102
        vec![ // 209 
            COM(2,213),
            PTR(0, true, true),
        ], 
        vec![ // 210 
            PRM(ADD,false),
            ARG(0, true),
            INT(1),
        ], 
        // AExp103
        vec![ // 211 
            COM(4,3),
            ARG(0, false),
            PTR(0, true, true),
        ], 
        vec![ // 212 
            COM(1,209),
            ARG(0, false),
            ARG(1, true),
        ], 
        // AExp104
        vec![ // 213 
            PRM(LE,false),
            ARG(0, false),
            ARG(1, false),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 214 
            COM(2,211),
            ARG(0, false),
            ARG(1, false),
        ], 
        // AExp105
        vec![ // 215 
            ARG(0, true),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 216 
            COM(3,0),
            ARG(2, true),
        ], 
    ],

}});