use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 115
#[rustfmt::skip]
pub static CLAUSIFY: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            COM(1,26),
            PTR(1, false, false),
        ], 
        vec![ // 1 
            COM(5,173),
            INT(0),
        ], 
        // AExp1
        vec![ // 2 
            COM(1,52),
            PTR(4, false, false),
        ], 
        vec![ // 3 
            COM(2,59),
            COM(2,91),
        ], 
        vec![ // 4 
            COM(3,53),
            PTR(3, false, false),
            COM(1,105),
        ], 
        // AExp2
        vec![ // 5 
            COM(1,75),
            COM(1,108),
        ], 
        // AExp3
        vec![ // 6 
            COM(1,119),
            PTR(7, false, false),
        ], 
        vec![ // 7 
            COM(2,143),
            COM(1,113),
        ], 
        // AExp4
        vec![ // 8 
            COM(1,170),
            COM(2,0),
        ], 
        // AExp5
        vec![ // 9 
            COM(2,40),
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
            COM(1,0),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 3 
            COM(2,243),
            ARG(0, false),
            ARG(0, false),
        ], 
        vec![ // 4 
            COM(2,243),
            ARG(0, false),
        ], 
        // AExp3
        vec![ // 5 
            COM(2,243),
            PTR(0, true, true),
        ], 
        vec![ // 6 
            COM(1,2),
            ARG(0, true),
        ], 
        // AExp4
        vec![ // 7 
            COM(1,0),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 8 
            COM(2,243),
            ARG(0, false),
            ARG(0, false),
        ], 
        vec![ // 9 
            COM(2,243),
            ARG(0, false),
        ], 
        // AExp5
        vec![ // 10 
            COM(1,0),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 11 
            COM(2,243),
            ARG(0, false),
            ARG(0, false),
        ], 
        vec![ // 12 
            COM(2,243),
            ARG(0, false),
        ], 
        // AExp6
        vec![ // 13 
            COM(2,243),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 14 
            COM(1,10),
            ARG(0, false),
        ], 
        vec![ // 15 
            COM(1,7),
            ARG(0, false),
        ], 
        // AExp7
        vec![ // 16 
            COM(1,0),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 17 
            COM(1,13),
            ARG(0, false),
        ], 
        vec![ // 18 
            COM(1,5),
            ARG(0, false),
        ], 
        // AExp8
        vec![ // 19 
            COM(2,237),
            INT(80),
            PTR(0, true, true),
        ], 
        vec![ // 20 
            COM(1,16),
            ARG(0, true),
        ], 
        // AExp9
        vec![ // 21 
            COM(1,0),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 22 
            COM(1,19),
            ARG(0, false),
        ], 
        vec![ // 23 
            COM(2,40),
            COM(6,127),
            ARG(0, false),
        ], 
        // AExp10
        vec![ // 24 
            COM(1,0),
            COM(1,50),
            PTR(0, true, true),
        ], 
        vec![ // 25 
            COM(1,21),
            ARG(0, true),
        ], 
        // AExp11
        vec![ // 26 
            COM(1,0),
            COM(1,31),
            PTR(0, true, true),
        ], 
        vec![ // 27 
            COM(1,24),
            ARG(0, true),
        ], 
        // AExp12
        vec![ // 28 
            PRM(ADD,false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 29 
            COM(1,31),
            ARG(1, true),
        ], 
        vec![ // 30 
            COM(1,35),
            ARG(0, true),
        ], 
        // AExp13
        vec![ // 31 
            ARG(0, true),
            INT(0),
            COM(2,28),
        ], 
        // AExp14
        vec![ // 32 
            PRM(ADD,false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 33 
            PTR(9, false, false),
            ARG(1, true),
        ], 
        vec![ // 34 
            PTR(9, false, false),
            ARG(0, true),
        ], 
        // AExp15
        vec![ // 35 
            ARG(0, true),
            COM(2,32),
        ], 
        // AExp16
        vec![ // 36 
            ARG(0, true),
            ARG(2, true),
            PTR(0, true, true),
        ], 
        vec![ // 37 
            ARG(1, true),
            ARG(3, true),
        ], 
        // AExp17
        vec![ // 38 
            ARG(3, true),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 39 
            COM(4,36),
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp18
        vec![ // 40 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 41 
            COM(4,38),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp19
        vec![ // 42 
            COM(1,0),
            COM(1,180),
            PTR(0, true, true),
        ], 
        vec![ // 43 
            COM(1,232),
            ARG(0, true),
        ], 
        // AExp20
        vec![ // 44 
            COM(1,0),
            PTR(8, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 45 
            COM(1,42),
            ARG(0, true),
        ], 
        // AExp21
        vec![ // 46 
            COM(1,0),
            PTR(6, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 47 
            COM(1,44),
            ARG(0, true),
        ], 
        // AExp22
        vec![ // 48 
            COM(1,0),
            PTR(5, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 49 
            COM(1,46),
            ARG(0, true),
        ], 
        // AExp23
        vec![ // 50 
            COM(1,0),
            PTR(2, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 51 
            COM(1,48),
            ARG(0, true),
        ], 
        // AExp24
        vec![ // 52 
            COM(2,40),
            ARG(0, true),
            COM(2,0),
        ], 
        // AExp25
        vec![ // 53 
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 54 
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp26
        vec![ // 55 
            COM(2,65),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 56 
            COM(1,75),
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp27
        vec![ // 57 
            COM(3,53),
            COM(1,77),
            PTR(0, true, true),
        ], 
        vec![ // 58 
            COM(1,83),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp28
        vec![ // 59 
            COM(3,55),
            PTR(0, true, true),
            ARG(1, false),
        ], 
        vec![ // 60 
            COM(2,57),
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp29
        vec![ // 61 
            COM(4,67),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 62 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp30
        vec![ // 63 
            ARG(2, true),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 64 
            COM(3,61),
            ARG(1, true),
        ], 
        // AExp31
        vec![ // 65 
            Y,
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 66 
            COM(3,63),
            ARG(1, true),
        ], 
        // AExp32
        vec![ // 67 
            ARG(3, true),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp33
        vec![ // 68 
            COM(4,67),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 69 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp34
        vec![ // 70 
            ARG(0, true),
            ARG(2, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 71 
            COM(3,68),
            ARG(1, false),
            ARG(2, false),
            ARG(3, false),
        ], 
        vec![ // 72 
            ARG(1, false),
            ARG(3, false),
        ], 
        // AExp35
        vec![ // 73 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 74 
            COM(4,70),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp36
        vec![ // 75 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 76 
            COM(3,73),
            ARG(0, true),
        ], 
        // AExp37
        vec![ // 77 
            ARG(0, true),
            COM(2,1),
            COM(2,0),
        ], 
        // AExp38
        vec![ // 78 
            COM(2,85),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 79 
            ARG(1, true),
            ARG(4, true),
            ARG(2, false),
        ], 
        vec![ // 80 
            ARG(0, true),
            ARG(3, true),
            ARG(2, false),
        ], 
        // AExp39
        vec![ // 81 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 82 
            COM(5,78),
            ARG(0, true),
            ARG(1, true),
            ARG(3, true),
        ], 
        // AExp40
        vec![ // 83 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 84 
            COM(4,81),
            ARG(0, true),
        ], 
        // AExp41
        vec![ // 85 
            ARG(0, true),
            ARG(1, true),
            COM(2,1),
        ], 
        // AExp42
        vec![ // 86 
            COM(1,93),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 87 
            COM(1,103),
            PRM(EQ,false),
            ARG(1, true),
            ARG(3, true),
        ], 
        vec![ // 88 
            COM(1,103),
            PRM(EQ,false),
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp43
        vec![ // 89 
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 90 
            COM(4,86),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp44
        vec![ // 91 
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 92 
            COM(3,89),
            ARG(1, true),
        ], 
        // AExp45
        vec![ // 93 
            ARG(0, true),
            COM(2,0),
        ], 
        // AExp46
        vec![ // 94 
            COM(2,0),
        ], 
        // AExp47
        vec![ // 95 
            COM(1,93),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 96 
            ARG(1, true),
            ARG(3, true),
            ARG(5, true),
        ], 
        vec![ // 97 
            ARG(0, true),
            ARG(2, true),
            ARG(4, true),
        ], 
        // AExp48
        vec![ // 98 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 99 
            COM(6,95),
            ARG(0, true),
            ARG(1, true),
            ARG(3, true),
            ARG(4, true),
        ], 
        // AExp49
        vec![ // 100 
            ARG(2, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 101 
            COM(5,98),
            ARG(0, true),
            ARG(1, true),
            ARG(3, false),
        ], 
        vec![ // 102 
            ARG(3, false),
            COM(2,1),
            COM(2,94),
        ], 
        // AExp50
        vec![ // 103 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 104 
            COM(4,100),
            ARG(0, true),
        ], 
        // AExp51
        vec![ // 105 
            COM(4,67),
            ARG(0, true),
            COM(2,0),
        ], 
        // AExp52
        vec![ // 106 
            COM(1,110),
            PTR(0, true, true),
        ], 
        vec![ // 107 
            COM(2,111),
            PRM(EQ,false),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp53
        vec![ // 108 
            ARG(0, true),
            COM(2,106),
        ], 
        // AExp54
        vec![ // 109 
            COM(2,0),
        ], 
        // AExp55
        vec![ // 110 
            ARG(0, true),
            COM(2,1),
            COM(2,109),
        ], 
        // AExp56
        vec![ // 111 
            COM(1,75),
            PTR(0, true, true),
        ], 
        vec![ // 112 
            COM(1,83),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp57
        vec![ // 113 
            ARG(0, true),
            COM(2,0),
            COM(2,0),
        ], 
        // AExp58
        vec![ // 114 
            COM(4,67),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 115 
            ARG(1, true),
            ARG(3, true),
        ], 
        vec![ // 116 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp59
        vec![ // 117 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 118 
            COM(4,114),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp60
        vec![ // 119 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 120 
            COM(3,117),
            ARG(0, true),
        ], 
        // AExp61
        vec![ // 121 
            ARG(0, true),
            ARG(1, true),
            PTR(2, true, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 122 
            ARG(4, true),
            ARG(5, false),
            ARG(6, false),
        ], 
        vec![ // 123 
            ARG(3, true),
            ARG(5, false),
            ARG(6, false),
        ], 
        vec![ // 124 
            ARG(2, true),
            ARG(5, false),
            ARG(6, false),
        ], 
        // AExp62
        vec![ // 125 
            COM(2,143),
            PTR(0, true, true),
        ], 
        vec![ // 126 
            COM(2,143),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp63
        vec![ // 127 
            ARG(2, true),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp64
        vec![ // 128 
            COM(2,125),
            PTR(0, true, true),
        ], 
        vec![ // 129 
            COM(3,127),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp65
        vec![ // 130 
            ARG(5, true),
            ARG(0, false),
            ARG(0, false),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 131 
            ARG(2, true),
            ARG(3, true),
            ARG(4, true),
        ], 
        // AExp66
        vec![ // 132 
            ARG(3, true),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 133 
            COM(1,155),
            ARG(2, true),
            ARG(1, true),
        ], 
        // AExp67
        vec![ // 134 
            COM(6,130),
            PTR(1, true, true),
            PTR(0, true, true),
            COM(4,132),
        ], 
        vec![ // 135 
            COM(2,0),
            ARG(0, false),
        ], 
        vec![ // 136 
            COM(3,0),
            ARG(0, false),
        ], 
        // AExp68
        vec![ // 137 
            ARG(3, true),
            PTR(0, true, true),
            ARG(1, true),
        ], 
        vec![ // 138 
            COM(1,155),
            ARG(2, true),
            ARG(0, true),
        ], 
        // AExp69
        vec![ // 139 
            COM(7,121),
            ARG(0, true),
            PTR(1, true, true),
            COM(2,128),
            PTR(0, true, true),
            COM(4,137),
        ], 
        vec![ // 140 
            COM(1,134),
            ARG(1, false),
        ], 
        vec![ // 141 
            COM(3,0),
            ARG(1, false),
        ], 
        // AExp70
        vec![ // 142 
            ARG(0, true),
            COM(2,0),
            COM(2,0),
        ], 
        // AExp71
        vec![ // 143 
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 144 
            COM(2,139),
            ARG(1, true),
            COM(1,142),
        ], 
        // AExp72
        vec![ // 145 
            COM(4,67),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 146 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp73
        vec![ // 147 
            COM(4,67),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 148 
            COM(4,67),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp74
        vec![ // 149 
            PRM(LE,false),
            ARG(0, false),
            ARG(2, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 150 
            COM(3,147),
            ARG(0, false),
            ARG(2, false),
            ARG(3, false),
        ], 
        vec![ // 151 
            COM(3,145),
            ARG(1, true),
            ARG(2, false),
            ARG(3, false),
        ], 
        // AExp75
        vec![ // 152 
            ARG(2, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 153 
            COM(4,149),
            ARG(0, false),
            ARG(1, true),
        ], 
        vec![ // 154 
            COM(4,67),
            ARG(0, false),
            COM(2,0),
        ], 
        // AExp76
        vec![ // 155 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 156 
            COM(3,152),
            ARG(0, true),
        ], 
        // AExp77
        vec![ // 157 
            COM(1,170),
            PTR(0, true, true),
        ], 
        vec![ // 158 
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp78
        vec![ // 159 
            COM(4,67),
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 160 
            COM(6,67),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp79
        vec![ // 161 
            COM(4,67),
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 162 
            COM(5,172),
            ARG(1, true),
        ], 
        // AExp80
        vec![ // 163 
            COM(4,67),
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 164 
            COM(5,173),
            ARG(1, true),
        ], 
        // AExp81
        vec![ // 165 
            ARG(2, true),
            PTR(3, true, true),
            PTR(2, true, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 166 
            COM(2,163),
            ARG(0, false),
        ], 
        vec![ // 167 
            COM(2,161),
            ARG(0, false),
        ], 
        vec![ // 168 
            COM(3,159),
            ARG(0, false),
        ], 
        vec![ // 169 
            COM(2,157),
            ARG(1, true),
        ], 
        // AExp82
        vec![ // 170 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 171 
            COM(3,165),
            ARG(0, true),
        ], 
        // AExp83
        vec![ // 172 
            ARG(3, true),
            ARG(0, true),
        ], 
        // AExp84
        vec![ // 173 
            ARG(4, true),
            ARG(0, true),
        ], 
        // AExp85
        vec![ // 174 
            COM(6,127),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 175 
            COM(1,180),
            ARG(1, true),
        ], 
        vec![ // 176 
            COM(1,180),
            ARG(0, true),
        ], 
        // AExp86
        vec![ // 177 
            COM(2,190),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 178 
            COM(1,180),
            ARG(1, true),
        ], 
        vec![ // 179 
            COM(1,180),
            ARG(0, true),
        ], 
        // AExp87
        vec![ // 180 
            ARG(0, true),
            COM(2,174),
            COM(2,177),
            COM(5,172),
            COM(5,173),
        ], 
        // AExp88
        vec![ // 181 
            COM(6,127),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 182 
            COM(2,190),
            ARG(2, true),
            ARG(0, false),
        ], 
        vec![ // 183 
            COM(2,190),
            ARG(1, true),
            ARG(0, false),
        ], 
        // AExp89
        vec![ // 184 
            COM(2,204),
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 185 
            COM(6,67),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp90
        vec![ // 186 
            COM(2,204),
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 187 
            COM(5,172),
            ARG(1, true),
        ], 
        // AExp91
        vec![ // 188 
            COM(2,204),
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 189 
            COM(5,173),
            ARG(1, true),
        ], 
        // AExp92
        vec![ // 190 
            ARG(0, true),
            PTR(3, true, true),
            PTR(2, true, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 191 
            COM(2,188),
            ARG(1, false),
        ], 
        vec![ // 192 
            COM(2,186),
            ARG(1, false),
        ], 
        vec![ // 193 
            COM(3,184),
            ARG(1, false),
        ], 
        vec![ // 194 
            COM(3,181),
            ARG(1, false),
        ], 
        // AExp93
        vec![ // 195 
            COM(6,127),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 196 
            COM(2,190),
            ARG(0, false),
            ARG(2, true),
        ], 
        vec![ // 197 
            COM(2,190),
            ARG(0, false),
            ARG(1, true),
        ], 
        // AExp94
        vec![ // 198 
            COM(6,67),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 199 
            COM(6,67),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp95
        vec![ // 200 
            COM(6,67),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 201 
            COM(5,172),
            ARG(1, true),
        ], 
        // AExp96
        vec![ // 202 
            COM(6,67),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 203 
            COM(5,173),
            ARG(1, true),
        ], 
        // AExp97
        vec![ // 204 
            ARG(1, true),
            PTR(3, true, true),
            PTR(2, true, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 205 
            COM(2,202),
            ARG(0, false),
        ], 
        vec![ // 206 
            COM(2,200),
            ARG(0, false),
        ], 
        vec![ // 207 
            COM(3,198),
            ARG(0, false),
        ], 
        vec![ // 208 
            COM(3,195),
            ARG(0, false),
        ], 
        // AExp98
        vec![ // 209 
            COM(6,127),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 210 
            COM(1,232),
            ARG(1, true),
        ], 
        vec![ // 211 
            COM(1,232),
            ARG(0, true),
        ], 
        // AExp99
        vec![ // 212 
            COM(6,67),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 213 
            COM(1,232),
            ARG(1, true),
        ], 
        vec![ // 214 
            COM(1,232),
            ARG(0, true),
        ], 
        // AExp100
        vec![ // 215 
            COM(1,232),
            PTR(0, true, true),
        ], 
        vec![ // 216 
            COM(5,172),
            ARG(0, true),
        ], 
        // AExp101
        vec![ // 217 
            COM(1,232),
            PTR(0, true, true),
        ], 
        vec![ // 218 
            COM(5,172),
            ARG(0, true),
        ], 
        // AExp102
        vec![ // 219 
            COM(6,67),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 220 
            COM(1,217),
            ARG(1, true),
        ], 
        vec![ // 221 
            COM(1,215),
            ARG(0, true),
        ], 
        // AExp103
        vec![ // 222 
            COM(1,232),
            PTR(0, true, true),
        ], 
        vec![ // 223 
            COM(5,172),
            ARG(0, true),
        ], 
        // AExp104
        vec![ // 224 
            COM(1,232),
            PTR(0, true, true),
        ], 
        vec![ // 225 
            COM(5,172),
            ARG(0, true),
        ], 
        // AExp105
        vec![ // 226 
            COM(6,127),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 227 
            COM(1,224),
            ARG(1, true),
        ], 
        vec![ // 228 
            COM(1,222),
            ARG(0, true),
        ], 
        // AExp106
        vec![ // 229 
            COM(5,172),
            PTR(0, true, true),
        ], 
        vec![ // 230 
            COM(5,173),
            ARG(0, true),
        ], 
        // AExp107
        vec![ // 231 
            ARG(0, true),
            COM(2,219),
            COM(2,226),
            COM(1,232),
            COM(1,229),
        ], 
        // AExp108
        vec![ // 232 
            ARG(0, true),
            COM(2,209),
            COM(2,212),
            COM(1,231),
            COM(5,173),
        ], 
        // AExp109
        vec![ // 233 
            COM(2,237),
            PTR(0, true, true),
        ], 
        vec![ // 234 
            PRM(SUB,false),
            ARG(0, true),
            INT(1),
        ], 
        // AExp110
        vec![ // 235 
            COM(4,67),
            ARG(1, false),
            PTR(0, true, true),
        ], 
        vec![ // 236 
            COM(1,233),
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp111
        vec![ // 237 
            PRM(LE,false),
            ARG(0, false),
            INT(0),
            PTR(0, true, true),
            COM(2,0),
        ], 
        vec![ // 238 
            COM(2,235),
            ARG(0, false),
            ARG(1, true),
        ], 
        // AExp112
        vec![ // 239 
            COM(6,67),
            PTR(0, true, true),
        ], 
        vec![ // 240 
            COM(5,172),
            ARG(0, true),
        ], 
        // AExp113
        vec![ // 241 
            COM(6,67),
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 242 
            COM(5,172),
            ARG(1, true),
        ], 
        // AExp114
        vec![ // 243 
            COM(6,127),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 244 
            COM(2,241),
            ARG(0, false),
            ARG(1, false),
        ], 
        vec![ // 245 
            COM(1,239),
            ARG(0, false),
            ARG(1, false),
        ], 
    ],

}});