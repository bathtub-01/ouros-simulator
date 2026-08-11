use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 122
#[rustfmt::skip]
pub static CLAUSIFY: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            Com(1,26),
            Ptr(1, false, false),
        ], 
        vec![ // 1 
            Com(5,158),
            Int(0),
        ], 
        // AExp1
        vec![ // 2 
            Com(1,51),
            Ptr(4, false, false),
        ], 
        vec![ // 3 
            Com(2,58),
            Com(2,92),
        ], 
        vec![ // 4 
            Com(3,52),
            Ptr(3, false, false),
            Com(1,108),
        ], 
        // AExp2
        vec![ // 5 
            Com(1,74),
            Com(1,111),
        ], 
        // AExp3
        vec![ // 6 
            Com(1,122),
            Ptr(7, false, false),
        ], 
        vec![ // 7 
            Com(2,136),
            Com(1,116),
        ], 
        // AExp4
        vec![ // 8 
            Com(1,155),
            Com(2,0),
        ], 
        // AExp5
        vec![ // 9 
            Com(2,39),
            Prm(Add,false),
            Int(0),
        ], 
    ],
    comb_img: vec![
        // AExp0
        vec![ // 0 
            Arg(0, true),
        ], 
        // AExp1
        vec![ // 1 
            Arg(1, true),
        ], 
        // AExp2
        vec![ // 2 
            Com(1,0),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 3 
            Com(2,221),
            Arg(0, false),
            Arg(0, false),
        ], 
        vec![ // 4 
            Com(2,221),
            Arg(0, false),
        ], 
        // AExp3
        vec![ // 5 
            Com(2,221),
            Ptr(0, true, true),
        ], 
        vec![ // 6 
            Com(1,2),
            Arg(0, true),
        ], 
        // AExp4
        vec![ // 7 
            Com(1,0),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 8 
            Com(2,221),
            Arg(0, false),
            Arg(0, false),
        ], 
        vec![ // 9 
            Com(2,221),
            Arg(0, false),
        ], 
        // AExp5
        vec![ // 10 
            Com(1,0),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 11 
            Com(2,221),
            Arg(0, false),
            Arg(0, false),
        ], 
        vec![ // 12 
            Com(2,221),
            Arg(0, false),
        ], 
        // AExp6
        vec![ // 13 
            Com(2,221),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 14 
            Com(1,10),
            Arg(0, false),
        ], 
        vec![ // 15 
            Com(1,7),
            Arg(0, false),
        ], 
        // AExp7
        vec![ // 16 
            Com(1,0),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 17 
            Com(1,13),
            Arg(0, false),
        ], 
        vec![ // 18 
            Com(1,5),
            Arg(0, false),
        ], 
        // AExp8
        vec![ // 19 
            Com(2,215),
            Int(80),
            Ptr(0, true, true),
        ], 
        vec![ // 20 
            Com(1,16),
            Arg(0, true),
        ], 
        // AExp9
        vec![ // 21 
            Com(1,0),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 22 
            Com(1,19),
            Arg(0, false),
        ], 
        vec![ // 23 
            Com(2,39),
            Com(6,125),
            Arg(0, false),
        ], 
        // AExp10
        vec![ // 24 
            Com(1,0),
            Com(1,49),
            Ptr(0, true, true),
        ], 
        vec![ // 25 
            Com(1,21),
            Arg(0, true),
        ], 
        // AExp11
        vec![ // 26 
            Com(1,0),
            Com(1,31),
            Ptr(0, true, true),
        ], 
        vec![ // 27 
            Com(1,24),
            Arg(0, true),
        ], 
        // AExp12
        vec![ // 28 
            Prm(Add,false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 29 
            Com(1,31),
            Arg(1, true),
        ], 
        vec![ // 30 
            Com(1,35),
            Arg(0, true),
        ], 
        // AExp13
        vec![ // 31 
            Arg(0, true),
            Int(0),
            Com(2,28),
        ], 
        // AExp14
        vec![ // 32 
            Prm(Add,false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 33 
            Ptr(9, false, false),
            Arg(1, true),
        ], 
        vec![ // 34 
            Ptr(9, false, false),
            Arg(0, true),
        ], 
        // AExp15
        vec![ // 35 
            Arg(0, true),
            Com(2,32),
        ], 
        // AExp16
        vec![ // 36 
            Arg(3, true),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 37 
            Arg(4, true),
            Arg(1, true),
        ], 
        // AExp17
        vec![ // 38 
            Arg(3, true),
            Com(3,0),
            Com(5,36),
            Arg(1, true),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp18
        vec![ // 39 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 40 
            Com(4,38),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp19
        vec![ // 41 
            Com(1,0),
            Com(1,165),
            Ptr(0, true, true),
        ], 
        vec![ // 42 
            Com(1,209),
            Arg(0, true),
        ], 
        // AExp20
        vec![ // 43 
            Com(1,0),
            Ptr(8, false, false),
            Ptr(0, true, true),
        ], 
        vec![ // 44 
            Com(1,41),
            Arg(0, true),
        ], 
        // AExp21
        vec![ // 45 
            Com(1,0),
            Ptr(6, false, false),
            Ptr(0, true, true),
        ], 
        vec![ // 46 
            Com(1,43),
            Arg(0, true),
        ], 
        // AExp22
        vec![ // 47 
            Com(1,0),
            Ptr(5, false, false),
            Ptr(0, true, true),
        ], 
        vec![ // 48 
            Com(1,45),
            Arg(0, true),
        ], 
        // AExp23
        vec![ // 49 
            Com(1,0),
            Ptr(2, false, false),
            Ptr(0, true, true),
        ], 
        vec![ // 50 
            Com(1,47),
            Arg(0, true),
        ], 
        // AExp24
        vec![ // 51 
            Com(2,39),
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp25
        vec![ // 52 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 53 
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp26
        vec![ // 54 
            Com(2,63),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 55 
            Com(1,74),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp27
        vec![ // 56 
            Com(3,52),
            Com(1,76),
            Ptr(0, true, true),
        ], 
        vec![ // 57 
            Com(1,83),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp28
        vec![ // 58 
            Com(3,54),
            Ptr(0, true, true),
            Arg(1, false),
        ], 
        vec![ // 59 
            Com(2,56),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp29
        vec![ // 60 
            Com(4,65),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 61 
            Arg(3, true),
            Arg(1, true),
        ], 
        // AExp30
        vec![ // 62 
            Arg(2, true),
            Com(2,0),
            Com(4,60),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp31
        vec![ // 63 
            Y,
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 64 
            Com(3,62),
            Arg(1, true),
        ], 
        // AExp32
        vec![ // 65 
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp33
        vec![ // 66 
            Com(2,0),
        ], 
        // AExp34
        vec![ // 67 
            Arg(3, true),
        ], 
        // AExp35
        vec![ // 68 
            Com(4,65),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 69 
            Com(1,74),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp36
        vec![ // 70 
            Arg(3, false),
            Arg(1, false),
            Com(4,67),
            Com(4,68),
            Arg(1, false),
            Arg(3, false),
            Arg(2, false),
            Ptr(0, true, true),
        ], 
        vec![ // 71 
            Arg(0, true),
            Arg(2, false),
        ], 
        // AExp37
        vec![ // 72 
            Arg(2, true),
            Com(1,66),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 73 
            Com(4,70),
            Arg(1, true),
        ], 
        // AExp38
        vec![ // 74 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 75 
            Com(3,72),
            Arg(0, true),
        ], 
        // AExp39
        vec![ // 76 
            Arg(0, true),
            Com(2,1),
            Com(2,0),
        ], 
        // AExp40
        vec![ // 77 
            Com(2,0),
        ], 
        // AExp41
        vec![ // 78 
            Com(1,86),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 79 
            Arg(0, true),
            Arg(2, true),
            Arg(4, false),
        ], 
        vec![ // 80 
            Arg(3, true),
            Arg(1, true),
            Arg(4, false),
        ], 
        // AExp42
        vec![ // 81 
            Arg(2, true),
            Com(2,77),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 82 
            Com(5,78),
            Arg(1, true),
        ], 
        // AExp43
        vec![ // 83 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 84 
            Com(3,81),
            Arg(0, true),
        ], 
        // AExp44
        vec![ // 85 
            Com(2,1),
        ], 
        // AExp45
        vec![ // 86 
            Arg(0, true),
            Com(1,0),
            Com(1,85),
        ], 
        // AExp46
        vec![ // 87 
            Com(1,95),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 88 
            Com(1,106),
            Prm(EQ,false),
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 89 
            Com(1,106),
            Prm(EQ,false),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp47
        vec![ // 90 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 91 
            Com(4,87),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp48
        vec![ // 92 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 93 
            Com(3,90),
            Arg(1, true),
        ], 
        // AExp49
        vec![ // 94 
            Com(2,0),
        ], 
        // AExp50
        vec![ // 95 
            Arg(0, true),
            Com(1,94),
            Com(1,0),
        ], 
        // AExp51
        vec![ // 96 
            Com(2,0),
        ], 
        // AExp52
        vec![ // 97 
            Arg(0, true),
            Com(2,1),
            Com(2,96),
        ], 
        // AExp53
        vec![ // 98 
            Com(2,0),
        ], 
        // AExp54
        vec![ // 99 
            Com(1,95),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 100 
            Arg(0, true),
            Arg(5, true),
            Arg(2, true),
        ], 
        vec![ // 101 
            Arg(3, true),
            Arg(4, true),
            Arg(1, true),
        ], 
        // AExp55
        vec![ // 102 
            Arg(3, true),
            Com(3,98),
            Ptr(0, true, true),
            Arg(4, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        vec![ // 103 
            Com(6,99),
            Arg(0, true),
        ], 
        // AExp56
        vec![ // 104 
            Arg(2, true),
            Com(2,97),
            Ptr(0, true, true),
            Arg(3, true),
            Arg(0, true),
        ], 
        vec![ // 105 
            Com(5,102),
            Arg(1, true),
        ], 
        // AExp57
        vec![ // 106 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 107 
            Com(4,104),
            Arg(0, true),
        ], 
        // AExp58
        vec![ // 108 
            Com(4,65),
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp59
        vec![ // 109 
            Com(1,113),
            Ptr(0, true, true),
        ], 
        vec![ // 110 
            Com(2,114),
            Prm(EQ,false),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp60
        vec![ // 111 
            Arg(0, true),
            Com(2,109),
        ], 
        // AExp61
        vec![ // 112 
            Com(2,0),
        ], 
        // AExp62
        vec![ // 113 
            Arg(0, true),
            Com(2,1),
            Com(2,112),
        ], 
        // AExp63
        vec![ // 114 
            Com(1,74),
            Ptr(0, true, true),
        ], 
        vec![ // 115 
            Com(1,83),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp64
        vec![ // 116 
            Arg(0, true),
            Com(2,0),
            Com(2,0),
        ], 
        // AExp65
        vec![ // 117 
            Com(2,0),
        ], 
        // AExp66
        vec![ // 118 
            Com(4,65),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 119 
            Arg(3, true),
            Arg(1, true),
        ], 
        vec![ // 120 
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp67
        vec![ // 121 
            Arg(2, true),
            Com(2,117),
            Com(4,118),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp68
        vec![ // 122 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 123 
            Com(3,121),
            Arg(0, true),
        ], 
        // AExp69
        vec![ // 124 
            Arg(2, true),
        ], 
        // AExp70
        vec![ // 125 
            Arg(2, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp71
        vec![ // 126 
            Com(2,136),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 127 
            Com(3,125),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp72
        vec![ // 128 
            Com(2,136),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 129 
            Com(3,126),
            Arg(0, true),
            Arg(3, true),
            Arg(4, true),
        ], 
        // AExp73
        vec![ // 130 
            Arg(4, true),
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 131 
            Com(2,144),
            Arg(0, true),
            Arg(3, true),
        ], 
        // AExp74
        vec![ // 132 
            Arg(0, true),
            Com(5,124),
            Com(5,124),
            Com(4,1),
            Com(5,130),
        ], 
        // AExp75
        vec![ // 133 
            Arg(4, true),
            Ptr(0, true, true),
            Arg(3, true),
        ], 
        vec![ // 134 
            Com(2,144),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp76
        vec![ // 135 
            Arg(0, true),
            Com(2,0),
            Com(2,0),
        ], 
        // AExp77
        vec![ // 136 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 137 
            Arg(1, true),
            Com(5,124),
            Com(5,128),
            Com(1,132),
            Com(5,133),
            Com(1,135),
        ], 
        // AExp78
        vec![ // 138 
            Com(4,65),
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp79
        vec![ // 139 
            Com(4,65),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 140 
            Com(2,144),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp80
        vec![ // 141 
            Com(4,65),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 142 
            Com(4,65),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp81
        vec![ // 143 
            Prm(LE,false),
            Arg(2, false),
            Arg(0, false),
            Com(3,139),
            Com(3,141),
            Arg(2, false),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp82
        vec![ // 144 
            Arg(1, true),
            Com(1,138),
            Com(3,143),
            Arg(0, true),
        ], 
        // AExp83
        vec![ // 145 
            Arg(6, true),
            Ptr(0, true, true),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
            Arg(4, true),
        ], 
        vec![ // 146 
            Arg(0, true),
            Arg(5, true),
        ], 
        // AExp84
        vec![ // 147 
            Com(1,155),
            Ptr(0, true, true),
            Arg(2, true),
        ], 
        vec![ // 148 
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp85
        vec![ // 149 
            Com(4,65),
            Ptr(0, true, true),
        ], 
        vec![ // 150 
            Com(6,65),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp86
        vec![ // 151 
            Com(4,65),
            Ptr(0, true, true),
        ], 
        vec![ // 152 
            Com(5,157),
            Arg(0, true),
        ], 
        // AExp87
        vec![ // 153 
            Com(4,65),
            Ptr(0, true, true),
        ], 
        vec![ // 154 
            Com(5,158),
            Arg(0, true),
        ], 
        // AExp88
        vec![ // 155 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 156 
            Com(7,145),
            Com(4,147),
            Com(2,149),
            Com(1,151),
            Com(1,153),
            Arg(0, true),
        ], 
        // AExp89
        vec![ // 157 
            Arg(3, true),
            Arg(0, true),
        ], 
        // AExp90
        vec![ // 158 
            Arg(4, true),
            Arg(0, true),
        ], 
        // AExp91
        vec![ // 159 
            Com(6,125),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 160 
            Com(1,165),
            Arg(1, true),
        ], 
        vec![ // 161 
            Com(1,165),
            Arg(0, true),
        ], 
        // AExp92
        vec![ // 162 
            Com(1,175),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 163 
            Com(1,165),
            Arg(1, true),
        ], 
        vec![ // 164 
            Com(1,165),
            Arg(0, true),
        ], 
        // AExp93
        vec![ // 165 
            Arg(0, true),
            Com(2,159),
            Com(2,162),
            Com(5,157),
            Com(5,158),
        ], 
        // AExp94
        vec![ // 166 
            Com(6,125),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 167 
            Com(1,175),
            Arg(1, true),
            Arg(2, false),
        ], 
        vec![ // 168 
            Com(1,175),
            Arg(0, true),
            Arg(2, false),
        ], 
        // AExp95
        vec![ // 169 
            Com(2,185),
            Ptr(0, true, true),
        ], 
        vec![ // 170 
            Com(6,65),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp96
        vec![ // 171 
            Com(2,185),
            Ptr(0, true, true),
        ], 
        vec![ // 172 
            Com(5,157),
            Arg(0, true),
        ], 
        // AExp97
        vec![ // 173 
            Com(2,185),
            Ptr(0, true, true),
        ], 
        vec![ // 174 
            Com(5,158),
            Arg(0, true),
        ], 
        // AExp98
        vec![ // 175 
            Arg(0, true),
            Com(3,166),
            Com(2,169),
            Com(1,171),
            Com(1,173),
        ], 
        // AExp99
        vec![ // 176 
            Com(6,125),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 177 
            Com(1,175),
            Arg(2, false),
            Arg(1, true),
        ], 
        vec![ // 178 
            Com(1,175),
            Arg(2, false),
            Arg(0, true),
        ], 
        // AExp100
        vec![ // 179 
            Com(6,65),
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 180 
            Com(6,65),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp101
        vec![ // 181 
            Com(6,65),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 182 
            Com(5,157),
            Arg(0, true),
        ], 
        // AExp102
        vec![ // 183 
            Com(6,65),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 184 
            Com(5,158),
            Arg(0, true),
        ], 
        // AExp103
        vec![ // 185 
            Arg(1, true),
            Com(3,176),
            Com(3,179),
            Com(2,181),
            Com(2,183),
            Arg(0, true),
        ], 
        // AExp104
        vec![ // 186 
            Com(6,125),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 187 
            Com(1,209),
            Arg(1, true),
        ], 
        vec![ // 188 
            Com(1,209),
            Arg(0, true),
        ], 
        // AExp105
        vec![ // 189 
            Com(6,65),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 190 
            Com(1,209),
            Arg(1, true),
        ], 
        vec![ // 191 
            Com(1,209),
            Arg(0, true),
        ], 
        // AExp106
        vec![ // 192 
            Com(1,209),
            Ptr(0, true, true),
        ], 
        vec![ // 193 
            Com(5,157),
            Arg(0, true),
        ], 
        // AExp107
        vec![ // 194 
            Com(1,209),
            Ptr(0, true, true),
        ], 
        vec![ // 195 
            Com(5,157),
            Arg(0, true),
        ], 
        // AExp108
        vec![ // 196 
            Com(6,65),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 197 
            Com(1,194),
            Arg(1, true),
        ], 
        vec![ // 198 
            Com(1,192),
            Arg(0, true),
        ], 
        // AExp109
        vec![ // 199 
            Com(1,209),
            Ptr(0, true, true),
        ], 
        vec![ // 200 
            Com(5,157),
            Arg(0, true),
        ], 
        // AExp110
        vec![ // 201 
            Com(1,209),
            Ptr(0, true, true),
        ], 
        vec![ // 202 
            Com(5,157),
            Arg(0, true),
        ], 
        // AExp111
        vec![ // 203 
            Com(6,125),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 204 
            Com(1,201),
            Arg(1, true),
        ], 
        vec![ // 205 
            Com(1,199),
            Arg(0, true),
        ], 
        // AExp112
        vec![ // 206 
            Com(5,157),
            Ptr(0, true, true),
        ], 
        vec![ // 207 
            Com(5,158),
            Arg(0, true),
        ], 
        // AExp113
        vec![ // 208 
            Arg(0, true),
            Com(2,196),
            Com(2,203),
            Com(1,209),
            Com(1,206),
        ], 
        // AExp114
        vec![ // 209 
            Arg(0, true),
            Com(2,186),
            Com(2,189),
            Com(1,208),
            Com(5,158),
        ], 
        // AExp115
        vec![ // 210 
            Com(2,0),
        ], 
        // AExp116
        vec![ // 211 
            Com(2,215),
            Ptr(0, true, true),
        ], 
        vec![ // 212 
            Prm(Sub,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp117
        vec![ // 213 
            Com(4,65),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 214 
            Com(1,211),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp118
        vec![ // 215 
            Prm(LE,false),
            Arg(0, false),
            Int(0),
            Com(1,0),
            Com(1,210),
            Ptr(0, true, true),
        ], 
        vec![ // 216 
            Com(2,213),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp119
        vec![ // 217 
            Com(6,65),
            Ptr(0, true, true),
        ], 
        vec![ // 218 
            Com(5,157),
            Arg(0, true),
        ], 
        // AExp120
        vec![ // 219 
            Com(6,65),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 220 
            Com(5,157),
            Arg(1, true),
        ], 
        // AExp121
        vec![ // 221 
            Com(6,125),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 222 
            Com(2,219),
            Arg(0, false),
            Arg(1, false),
        ], 
        vec![ // 223 
            Com(1,217),
            Arg(0, false),
            Arg(1, false),
        ], 
    ],

}});