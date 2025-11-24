use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 109
#[rustfmt::skip]
pub static COUNTDOWN: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            PTR(5, false, false),
            PTR(4, false, false),
        ], 
        vec![ // 1 
            COM(4,2),
            INT(10),
            COM(2,0),
        ], 
        vec![ // 2 
            COM(4,2),
            INT(4),
            PTR(1, false, false),
        ], 
        vec![ // 3 
            COM(4,2),
            INT(3),
            PTR(2, false, false),
        ], 
        vec![ // 4 
            COM(2,7),
            PTR(3, false, false),
            INT(70),
        ], 
        // AExp1
        vec![ // 5 
            Y,
            COM(3,5),
            INT(0),
        ], 
        // AExp2
        vec![ // 6 
            COM(3,63),
            PTR(11, false, false),
        ], 
        vec![ // 7 
            COM(4,2),
            COM(1,163),
            COM(2,0),
        ], 
        vec![ // 8 
            COM(4,2),
            COM(1,162),
            PTR(7, false, false),
        ], 
        vec![ // 9 
            COM(1,69),
            PTR(8, false, false),
        ], 
        vec![ // 10 
            COM(5,67),
            PTR(9, false, false),
        ], 
        vec![ // 11 
            COM(4,65),
            PTR(10, false, false),
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
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 4 
            PRM(ADD,false),
            ARG(1, true),
            INT(1),
        ], 
        // AExp4
        vec![ // 5 
            ARG(2, true),
            ARG(1, false),
            PTR(0, true, true),
        ], 
        vec![ // 6 
            COM(3,3),
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp5
        vec![ // 7 
            COM(1,15),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 8 
            COM(1,196),
            ARG(0, true),
        ], 
        vec![ // 9 
            COM(2,23),
            ARG(1, true),
        ], 
        // AExp6
        vec![ // 10 
            COM(2,21),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 11 
            ARG(1, true),
            ARG(3, true),
        ], 
        vec![ // 12 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp7
        vec![ // 13 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 14 
            COM(4,10),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp8
        vec![ // 15 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 16 
            COM(3,13),
            ARG(0, true),
        ], 
        // AExp9
        vec![ // 17 
            COM(4,2),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 18 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp10
        vec![ // 19 
            ARG(2, true),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 20 
            COM(3,17),
            ARG(1, true),
        ], 
        // AExp11
        vec![ // 21 
            Y,
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 22 
            COM(3,19),
            ARG(1, true),
        ], 
        // AExp12
        vec![ // 23 
            COM(1,34),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 24 
            COM(1,47),
            ARG(1, true),
        ], 
        // AExp13
        vec![ // 25 
            COM(4,2),
            ARG(2, true),
            PTR(0, true, true),
        ], 
        vec![ // 26 
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp14
        vec![ // 27 
            PRM(EQ,false),
            ARG(4, true),
            ARG(0, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 28 
            COM(3,25),
            ARG(1, false),
            ARG(2, false),
            ARG(3, true),
        ], 
        vec![ // 29 
            ARG(1, false),
            ARG(2, false),
        ], 
        // AExp15
        vec![ // 30 
            ARG(2, true),
            PTR(0, true, true),
        ], 
        vec![ // 31 
            COM(5,27),
            ARG(0, true),
            ARG(1, true),
            ARG(3, true),
        ], 
        // AExp16
        vec![ // 32 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 33 
            COM(4,30),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp17
        vec![ // 34 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 35 
            COM(3,32),
            ARG(0, true),
        ], 
        // AExp18
        vec![ // 36 
            COM(1,178),
            PTR(0, true, true),
        ], 
        vec![ // 37 
            COM(4,2),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp19
        vec![ // 38 
            COM(1,15),
            COM(1,53),
            PTR(0, true, true),
        ], 
        vec![ // 39 
            COM(2,36),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp20
        vec![ // 40 
            COM(1,49),
            ARG(2, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 41 
            COM(4,2),
            ARG(1, true),
            COM(2,0),
        ], 
        vec![ // 42 
            COM(2,38),
            ARG(0, true),
            ARG(2, false),
        ], 
        // AExp21
        vec![ // 43 
            ARG(1, true),
            PTR(0, true, true),
            ARG(0, false),
        ], 
        vec![ // 44 
            COM(2,194),
            ARG(0, false),
        ], 
        // AExp22
        vec![ // 45 
            COM(3,40),
            ARG(0, false),
            PTR(0, true, true),
        ], 
        vec![ // 46 
            COM(2,43),
            ARG(0, false),
        ], 
        // AExp23
        vec![ // 47 
            ARG(0, true),
            COM(2,0),
            COM(1,45),
        ], 
        // AExp24
        vec![ // 48 
            COM(2,0),
        ], 
        // AExp25
        vec![ // 49 
            ARG(0, true),
            COM(2,1),
            COM(2,48),
        ], 
        // AExp26
        vec![ // 50 
            COM(1,61),
            PTR(6, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 51 
            COM(1,47),
            ARG(1, true),
        ], 
        vec![ // 52 
            COM(1,47),
            ARG(0, true),
        ], 
        // AExp27
        vec![ // 53 
            ARG(0, true),
            COM(2,50),
        ], 
        // AExp28
        vec![ // 54 
            COM(1,15),
            PTR(0, true, true),
            ARG(1, true),
        ], 
        vec![ // 55 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp29
        vec![ // 56 
            COM(2,21),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 57 
            ARG(1, true),
            ARG(4, true),
            ARG(2, false),
        ], 
        vec![ // 58 
            COM(3,54),
            ARG(0, true),
            ARG(2, false),
            ARG(3, true),
        ], 
        // AExp30
        vec![ // 59 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 60 
            COM(5,56),
            ARG(0, true),
            ARG(1, true),
            ARG(3, true),
        ], 
        // AExp31
        vec![ // 61 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 62 
            COM(4,59),
            ARG(0, true),
        ], 
        // AExp32
        vec![ // 63 
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 64 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp33
        vec![ // 65 
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 66 
            ARG(0, true),
            ARG(2, true),
            ARG(3, true),
        ], 
        // AExp34
        vec![ // 67 
            COM(1,15),
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 68 
            COM(5,76),
            ARG(1, true),
            ARG(2, true),
            ARG(3, true),
            ARG(4, true),
        ], 
        // AExp35
        vec![ // 69 
            COM(4,2),
            COM(1,160),
            PTR(0, true, true),
        ], 
        vec![ // 70 
            COM(4,2),
            COM(1,161),
            ARG(0, true),
        ], 
        // AExp36
        vec![ // 71 
            ARG(5, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 72 
            COM(3,148),
            ARG(4, false),
            ARG(1, true),
            ARG(3, true),
        ], 
        vec![ // 73 
            COM(4,127),
            ARG(4, false),
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp37
        vec![ // 74 
            COM(4,2),
            PTR(0, true, true),
            COM(2,0),
        ], 
        vec![ // 75 
            COM(6,71),
            ARG(0, true),
            ARG(1, true),
            ARG(2, true),
            ARG(3, true),
            ARG(4, true),
        ], 
        // AExp38
        vec![ // 76 
            COM(3,97),
            ARG(4, false),
            ARG(1, false),
            ARG(3, false),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 77 
            COM(5,74),
            ARG(0, true),
            ARG(1, false),
            ARG(2, true),
            ARG(3, false),
            ARG(4, false),
        ], 
        // AExp39
        vec![ // 78 
            PRM(EQ,false),
            ARG(1, true),
            INT(3),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 79 
            ARG(2, true),
            ARG(0, true),
        ], 
        // AExp40
        vec![ // 80 
            COM(1,99),
            PTR(0, true, true),
        ], 
        vec![ // 81 
            PRM(LE,false),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp41
        vec![ // 82 
            COM(3,78),
            PTR(0, true, true),
        ], 
        vec![ // 83 
            COM(2,80),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp42
        vec![ // 84 
            PRM(LT,false),
            ARG(2, false),
            INT(3),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 85 
            ARG(3, false),
            COM(2,1),
        ], 
        vec![ // 86 
            COM(2,82),
            ARG(0, true),
            ARG(1, true),
            ARG(2, false),
            ARG(3, false),
        ], 
        // AExp43
        vec![ // 87 
            PRM(LT,false),
            ARG(1, true),
            INT(1),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 88 
            ARG(2, false),
            COM(2,1),
        ], 
        vec![ // 89 
            ARG(2, false),
            ARG(0, true),
        ], 
        // AExp44
        vec![ // 90 
            PRM(EQ,false),
            PTR(0, true, true),
            INT(0),
        ], 
        vec![ // 91 
            COM(2,100),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp45
        vec![ // 92 
            COM(3,87),
            PTR(0, true, true),
        ], 
        vec![ // 93 
            COM(2,90),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp46
        vec![ // 94 
            PRM(LT,false),
            ARG(2, false),
            INT(2),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 95 
            COM(2,92),
            ARG(0, false),
            ARG(1, false),
            ARG(2, false),
            ARG(3, false),
        ], 
        vec![ // 96 
            COM(4,84),
            ARG(0, false),
            ARG(1, false),
            ARG(2, false),
            ARG(3, false),
        ], 
        // AExp47
        vec![ // 97 
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 98 
            COM(4,94),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp48
        vec![ // 99 
            ARG(0, true),
            COM(2,1),
            COM(2,0),
        ], 
        // AExp49
        vec![ // 100 
            COM(1,124),
            ARG(0, true),
            ARG(1, true),
            COM(2,1),
        ], 
        // AExp50
        vec![ // 101 
            PRM(LE,false),
            ARG(5, false),
            ARG(1, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 102 
            ARG(3, true),
            ARG(5, false),
        ], 
        vec![ // 103 
            PRM(LE,false),
            ARG(4, true),
            ARG(1, false),
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp51
        vec![ // 104 
            ARG(1, true),
            INT(0),
            ARG(0, true),
        ], 
        // AExp52
        vec![ // 105 
            ARG(2, true),
            INT(1),
            PTR(0, true, true),
        ], 
        vec![ // 106 
            PRM(SUB,false),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp53
        vec![ // 107 
            ARG(2, true),
            ARG(1, true),
            ARG(0, true),
        ], 
        // AExp54
        vec![ // 108 
            ARG(3, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 109 
            PRM(SUB,false),
            ARG(1, true),
            ARG(0, true),
        ], 
        vec![ // 110 
            PRM(ADD,false),
            ARG(2, true),
            INT(1),
        ], 
        // AExp55
        vec![ // 111 
            PRM(LE,false),
            ARG(0, false),
            ARG(2, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 112 
            COM(4,108),
            ARG(0, false),
            ARG(2, false),
            ARG(1, false),
        ], 
        vec![ // 113 
            COM(3,107),
            ARG(2, false),
            ARG(1, false),
        ], 
        // AExp56
        vec![ // 114 
            PRM(ADD,false),
            ARG(0, false),
            ARG(0, false),
        ], 
        // AExp57
        vec![ // 115 
            COM(3,111),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 116 
            COM(1,114),
            ARG(1, true),
        ], 
        // AExp58
        vec![ // 117 
            ARG(0, true),
            ARG(2, true),
            PTR(0, true, true),
        ], 
        vec![ // 118 
            COM(2,115),
            ARG(1, true),
        ], 
        // AExp59
        vec![ // 119 
            COM(6,101),
            PTR(3, true, true),
            ARG(0, false),
            PTR(2, true, true),
            PTR(1, true, true),
            ARG(2, false),
            PTR(0, true, true),
        ], 
        vec![ // 120 
            PRM(ADD,false),
            ARG(2, false),
            ARG(2, false),
        ], 
        vec![ // 121 
            COM(3,117),
            ARG(1, true),
            ARG(2, false),
        ], 
        vec![ // 122 
            COM(3,105),
            ARG(0, false),
            ARG(2, false),
        ], 
        vec![ // 123 
            COM(2,104),
            ARG(0, false),
        ], 
        // AExp60
        vec![ // 124 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 125 
            COM(3,119),
            ARG(0, true),
        ], 
        // AExp61
        vec![ // 126 
            ARG(3, true),
            ARG(0, true),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp62
        vec![ // 127 
            ARG(3, true),
            INT(5),
            PTR(0, true, true),
        ], 
        vec![ // 128 
            COM(4,126),
            ARG(0, true),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp63
        vec![ // 129 
            ARG(2, true),
            PTR(0, true, true),
        ], 
        vec![ // 130 
            PRM(SUB,false),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp64
        vec![ // 131 
            PRM(EQ,false),
            ARG(2, true),
            INT(3),
            INT(0),
            PTR(0, true, true),
        ], 
        vec![ // 132 
            COM(3,129),
            ARG(0, true),
            ARG(1, true),
            ARG(3, true),
        ], 
        // AExp65
        vec![ // 133 
            ARG(2, true),
            PTR(0, true, true),
        ], 
        vec![ // 134 
            COM(1,157),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp66
        vec![ // 135 
            PRM(LT,false),
            ARG(2, false),
            INT(3),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 136 
            COM(3,133),
            ARG(0, false),
            ARG(1, false),
            ARG(3, false),
        ], 
        vec![ // 137 
            COM(4,131),
            ARG(0, false),
            ARG(1, false),
            ARG(2, false),
            ARG(3, false),
        ], 
        // AExp67
        vec![ // 138 
            ARG(2, true),
            PTR(0, true, true),
        ], 
        vec![ // 139 
            COM(2,159),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp68
        vec![ // 140 
            ARG(2, true),
            PTR(0, true, true),
        ], 
        vec![ // 141 
            PRM(ADD,false),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp69
        vec![ // 142 
            PRM(LT,false),
            ARG(2, true),
            INT(1),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 143 
            COM(3,140),
            ARG(0, false),
            ARG(1, false),
            ARG(3, false),
        ], 
        vec![ // 144 
            COM(3,138),
            ARG(0, false),
            ARG(1, false),
            ARG(3, false),
        ], 
        // AExp70
        vec![ // 145 
            PRM(LT,false),
            ARG(2, false),
            INT(2),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 146 
            COM(4,142),
            ARG(0, false),
            ARG(1, false),
            ARG(2, false),
            ARG(3, false),
        ], 
        vec![ // 147 
            COM(4,135),
            ARG(0, false),
            ARG(1, false),
            ARG(2, false),
            ARG(3, false),
        ], 
        // AExp71
        vec![ // 148 
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 149 
            COM(4,145),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp72
        vec![ // 150 
            PRM(EQ,false),
            ARG(2, false),
            INT(1),
            PTR(0, true, true),
            ARG(1, true),
        ], 
        vec![ // 151 
            COM(1,124),
            ARG(2, false),
            INT(2),
            ARG(0, true),
        ], 
        // AExp73
        vec![ // 152 
            COM(1,157),
            PTR(0, true, true),
        ], 
        vec![ // 153 
            PRM(ADD,false),
            ARG(0, false),
            ARG(0, false),
        ], 
        // AExp74
        vec![ // 154 
            PRM(ADD,false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 155 
            PRM(EQ,false),
            ARG(2, true),
            INT(0),
            ARG(0, false),
            INT(0),
        ], 
        vec![ // 156 
            COM(1,152),
            ARG(0, false),
            ARG(1, true),
        ], 
        // AExp75
        vec![ // 157 
            COM(3,150),
            PTR(0, true, true),
            ARG(0, false),
        ], 
        vec![ // 158 
            COM(3,154),
            ARG(0, false),
        ], 
        // AExp76
        vec![ // 159 
            COM(1,124),
            ARG(0, true),
            ARG(1, true),
            COM(2,0),
        ], 
        // AExp77
        vec![ // 160 
            ARG(0, true),
            INT(0),
            COM(1,0),
        ], 
        // AExp78
        vec![ // 161 
            ARG(0, true),
            INT(3),
            COM(1,0),
        ], 
        // AExp79
        vec![ // 162 
            ARG(0, true),
            INT(2),
            COM(1,0),
        ], 
        // AExp80
        vec![ // 163 
            ARG(0, true),
            INT(1),
            COM(1,0),
        ], 
        // AExp81
        vec![ // 164 
            ARG(2, true),
            PTR(0, true, true),
            ARG(1, true),
        ], 
        vec![ // 165 
            COM(4,2),
            ARG(0, true),
            COM(2,0),
        ], 
        // AExp82
        vec![ // 166 
            COM(1,184),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 167 
            COM(1,178),
            ARG(1, true),
        ], 
        vec![ // 168 
            COM(2,191),
            ARG(0, true),
        ], 
        // AExp83
        vec![ // 169 
            ARG(1, true),
            PTR(0, true, true),
            COM(1,0),
        ], 
        vec![ // 170 
            COM(4,2),
            ARG(0, true),
        ], 
        // AExp84
        vec![ // 171 
            COM(2,166),
            PTR(0, true, true),
        ], 
        vec![ // 172 
            COM(2,169),
            ARG(0, true),
        ], 
        // AExp85
        vec![ // 173 
            COM(4,2),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 174 
            COM(1,171),
            ARG(0, false),
            ARG(1, false),
        ], 
        vec![ // 175 
            COM(3,164),
            ARG(0, false),
            ARG(1, false),
        ], 
        // AExp86
        vec![ // 176 
            COM(1,49),
            ARG(1, false),
            PTR(0, true, true),
            COM(2,0),
        ], 
        vec![ // 177 
            COM(2,173),
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp87
        vec![ // 178 
            ARG(0, true),
            COM(2,0),
            COM(2,176),
        ], 
        // AExp88
        vec![ // 179 
            COM(4,2),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 180 
            ARG(1, true),
            ARG(3, true),
        ], 
        vec![ // 181 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp89
        vec![ // 182 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 183 
            COM(4,179),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp90
        vec![ // 184 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 185 
            COM(3,182),
            ARG(0, true),
        ], 
        // AExp91
        vec![ // 186 
            ARG(4, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 187 
            ARG(1, true),
            ARG(3, true),
        ], 
        vec![ // 188 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp92
        vec![ // 189 
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 190 
            COM(5,186),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp93
        vec![ // 191 
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 192 
            COM(3,189),
            ARG(1, true),
        ], 
        // AExp94
        vec![ // 193 
            ARG(1, true),
            ARG(0, true),
        ], 
        // AExp95
        vec![ // 194 
            ARG(1, true),
            INT(4),
            PTR(0, true, true),
        ], 
        vec![ // 195 
            COM(2,193),
            ARG(0, true),
        ], 
        // AExp96
        vec![ // 196 
            COM(1,15),
            COM(1,201),
            PTR(0, true, true),
        ], 
        vec![ // 197 
            COM(1,224),
            ARG(0, true),
        ], 
        // AExp97
        vec![ // 198 
            COM(1,15),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 199 
            COM(1,201),
            ARG(1, true),
        ], 
        vec![ // 200 
            COM(1,216),
            ARG(0, true),
        ], 
        // AExp98
        vec![ // 201 
            ARG(0, true),
            PTR(0, true, true),
            COM(2,198),
        ], 
        vec![ // 202 
            COM(4,2),
            COM(2,0),
            COM(2,0),
        ], 
        // AExp99
        vec![ // 203 
            COM(4,2),
            PTR(0, true, true),
            COM(2,0),
        ], 
        vec![ // 204 
            COM(4,2),
            ARG(0, true),
            COM(2,0),
        ], 
        // AExp100
        vec![ // 205 
            COM(4,2),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 206 
            COM(4,2),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp101
        vec![ // 207 
            COM(1,184),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 208 
            ARG(0, true),
            ARG(2, true),
        ], 
        vec![ // 209 
            COM(4,2),
            ARG(1, true),
        ], 
        // AExp102
        vec![ // 210 
            COM(4,2),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 211 
            COM(3,207),
            ARG(1, true),
            ARG(2, false),
            ARG(3, false),
        ], 
        vec![ // 212 
            COM(3,205),
            ARG(0, true),
            ARG(2, false),
            ARG(3, false),
        ], 
        // AExp103
        vec![ // 213 
            ARG(2, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 214 
            COM(4,210),
            ARG(0, false),
            ARG(1, true),
        ], 
        vec![ // 215 
            COM(1,203),
            ARG(0, false),
        ], 
        // AExp104
        vec![ // 216 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 217 
            COM(3,213),
            ARG(0, true),
        ], 
        // AExp105
        vec![ // 218 
            COM(1,184),
            PTR(0, true, true),
        ], 
        vec![ // 219 
            COM(4,2),
            ARG(0, true),
        ], 
        // AExp106
        vec![ // 220 
            COM(2,21),
            ARG(1, false),
            PTR(0, true, true),
        ], 
        vec![ // 221 
            COM(1,218),
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp107
        vec![ // 222 
            COM(2,220),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 223 
            COM(1,224),
            ARG(1, true),
        ], 
        // AExp108
        vec![ // 224 
            ARG(0, true),
            PTR(0, true, true),
            COM(2,222),
        ], 
        vec![ // 225 
            COM(4,2),
            COM(2,0),
            COM(2,0),
        ], 
    ],

}});