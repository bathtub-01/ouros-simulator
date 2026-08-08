use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 108
#[rustfmt::skip]
pub static COUNTDOWN: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            Ptr(5, false, false),
            Ptr(4, false, false),
        ], 
        vec![ // 1 
            Com(4,2),
            Int(10),
            Com(2,0),
        ], 
        vec![ // 2 
            Com(4,2),
            Int(4),
            Ptr(1, false, false),
        ], 
        vec![ // 3 
            Com(4,2),
            Int(3),
            Ptr(2, false, false),
        ], 
        vec![ // 4 
            Com(2,6),
            Ptr(3, false, false),
            Int(70),
        ], 
        // AExp1
        vec![ // 5 
            Y,
            Com(3,5),
            Int(0),
        ], 
        // AExp2
        vec![ // 6 
            Com(2,56),
            Ptr(11, false, false),
        ], 
        vec![ // 7 
            Com(4,2),
            Com(6,1),
            Com(2,0),
        ], 
        vec![ // 8 
            Com(4,2),
            Com(6,119),
            Ptr(7, false, false),
        ], 
        vec![ // 9 
            Com(1,60),
            Ptr(8, false, false),
        ], 
        vec![ // 10 
            Com(5,58),
            Ptr(9, false, false),
        ], 
        vec![ // 11 
            Com(4,57),
            Ptr(10, false, false),
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
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp3
        vec![ // 3 
            Arg(3, true),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 4 
            Prm(Add,false),
            Arg(2, true),
            Int(1),
        ], 
        // AExp4
        vec![ // 5 
            Arg(2, true),
            Com(2,0),
            Com(4,3),
            Arg(1, true),
            Arg(0, true),
        ], 
        // AExp5
        vec![ // 6 
            Com(1,14),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 7 
            Com(1,148),
            Arg(0, true),
        ], 
        vec![ // 8 
            Com(2,21),
            Arg(1, true),
        ], 
        // AExp6
        vec![ // 9 
            Com(2,0),
        ], 
        // AExp7
        vec![ // 10 
            Com(2,19),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 11 
            Arg(3, true),
            Arg(1, true),
        ], 
        vec![ // 12 
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp8
        vec![ // 13 
            Arg(2, true),
            Com(2,9),
            Com(4,10),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp9
        vec![ // 14 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 15 
            Com(3,13),
            Arg(0, true),
        ], 
        // AExp10
        vec![ // 16 
            Com(4,2),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 17 
            Arg(3, true),
            Arg(1, true),
        ], 
        // AExp11
        vec![ // 18 
            Arg(2, true),
            Com(2,0),
            Com(4,16),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp12
        vec![ // 19 
            Y,
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 20 
            Com(3,18),
            Arg(1, true),
        ], 
        // AExp13
        vec![ // 21 
            Com(2,29),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 22 
            Com(1,39),
            Arg(1, true),
        ], 
        // AExp14
        vec![ // 23 
            Com(2,0),
        ], 
        // AExp15
        vec![ // 24 
            Com(2,29),
        ], 
        // AExp16
        vec![ // 25 
            Com(4,2),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 26 
            Com(2,29),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp17
        vec![ // 27 
            Prm(EQ,false),
            Arg(1, true),
            Arg(2, false),
            Com(1,24),
            Com(3,25),
            Arg(0, true),
            Arg(2, false),
        ], 
        // AExp18
        vec![ // 28 
            Arg(0, true),
            Com(3,27),
            Arg(2, true),
            Arg(1, true),
        ], 
        // AExp19
        vec![ // 29 
            Arg(1, true),
            Com(1,23),
            Com(3,28),
            Arg(0, true),
        ], 
        // AExp20
        vec![ // 30 
            Com(1,134),
            Ptr(0, true, true),
        ], 
        vec![ // 31 
            Com(4,2),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp21
        vec![ // 32 
            Com(1,14),
            Com(1,45),
            Ptr(0, true, true),
        ], 
        vec![ // 33 
            Com(2,30),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp22
        vec![ // 34 
            Arg(1, true),
            Ptr(0, true, true),
            Arg(0, false),
        ], 
        vec![ // 35 
            Com(7,147),
            Arg(0, false),
        ], 
        // AExp23
        vec![ // 36 
            Com(4,2),
            Ptr(0, true, true),
            Com(2,0),
        ], 
        vec![ // 37 
            Com(2,34),
            Arg(0, true),
        ], 
        // AExp24
        vec![ // 38 
            Com(1,41),
            Arg(1, false),
            Com(2,32),
            Com(2,36),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp25
        vec![ // 39 
            Arg(0, true),
            Com(2,0),
            Com(2,38),
        ], 
        // AExp26
        vec![ // 40 
            Com(2,0),
        ], 
        // AExp27
        vec![ // 41 
            Arg(0, true),
            Com(2,1),
            Com(2,40),
        ], 
        // AExp28
        vec![ // 42 
            Com(1,54),
            Ptr(6, false, false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 43 
            Com(1,39),
            Arg(1, true),
        ], 
        vec![ // 44 
            Com(1,39),
            Arg(0, true),
        ], 
        // AExp29
        vec![ // 45 
            Arg(0, true),
            Com(2,42),
        ], 
        // AExp30
        vec![ // 46 
            Com(2,0),
        ], 
        // AExp31
        vec![ // 47 
            Com(1,14),
            Ptr(0, true, true),
        ], 
        vec![ // 48 
            Arg(1, true),
            Arg(0, true),
        ], 
        // AExp32
        vec![ // 49 
            Com(2,19),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 50 
            Arg(0, true),
            Arg(2, true),
            Arg(4, false),
        ], 
        vec![ // 51 
            Com(2,47),
            Arg(1, true),
            Arg(3, true),
            Arg(4, false),
        ], 
        // AExp33
        vec![ // 52 
            Arg(2, true),
            Com(2,46),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 53 
            Com(5,49),
            Arg(1, true),
        ], 
        // AExp34
        vec![ // 54 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 55 
            Com(3,52),
            Arg(0, true),
        ], 
        // AExp35
        vec![ // 56 
            Arg(1, true),
            Arg(0, true),
        ], 
        // AExp36
        vec![ // 57 
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp37
        vec![ // 58 
            Com(1,14),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 59 
            Com(2,72),
            Arg(3, true),
            Arg(4, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp38
        vec![ // 60 
            Com(4,2),
            Com(6,0),
            Ptr(0, true, true),
        ], 
        vec![ // 61 
            Com(4,2),
            Com(6,118),
            Arg(0, true),
        ], 
        // AExp39
        vec![ // 62 
            Arg(0, true),
            Arg(4, true),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        // AExp40
        vec![ // 63 
            Com(5,62),
            Ptr(0, true, true),
            Arg(2, true),
            Arg(1, true),
            Arg(3, false),
        ], 
        vec![ // 64 
            Arg(0, true),
            Arg(3, false),
        ], 
        // AExp41
        vec![ // 65 
            Com(2,0),
        ], 
        // AExp42
        vec![ // 66 
            Arg(5, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 67 
            Com(2,107),
            Arg(0, false),
            Arg(3, true),
            Arg(4, true),
        ], 
        vec![ // 68 
            Com(5,102),
            Arg(0, false),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp43
        vec![ // 69 
            Com(4,2),
            Ptr(0, true, true),
            Com(2,0),
        ], 
        vec![ // 70 
            Com(6,66),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
            Arg(4, true),
        ], 
        // AExp44
        vec![ // 71 
            Com(1,82),
            Arg(3, false),
            Arg(1, true),
            Arg(2, true),
            Com(5,65),
            Com(5,69),
            Arg(3, false),
            Arg(0, true),
        ], 
        // AExp45
        vec![ // 72 
            Com(4,63),
            Ptr(0, true, true),
            Arg(1, false),
        ], 
        vec![ // 73 
            Com(4,71),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp46
        vec![ // 74 
            Com(2,1),
        ], 
        // AExp47
        vec![ // 75 
            Prm(EQ,false),
            Ptr(0, true, true),
            Int(0),
        ], 
        vec![ // 76 
            Com(2,83),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp48
        vec![ // 77 
            Com(2,1),
        ], 
        // AExp49
        vec![ // 78 
            Com(1,100),
            Ptr(0, true, true),
        ], 
        vec![ // 79 
            Prm(LE,false),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp50
        vec![ // 80 
            Com(2,0),
        ], 
        // AExp51
        vec![ // 81 
            Com(2,0),
        ], 
        // AExp52
        vec![ // 82 
            Arg(0, true),
            Com(2,74),
            Com(2,75),
            Com(2,77),
            Com(2,78),
            Com(3,80),
            Com(5,81),
        ], 
        // AExp53
        vec![ // 83 
            Com(1,98),
            Arg(0, true),
            Arg(1, true),
            Com(2,1),
        ], 
        // AExp54
        vec![ // 84 
            Arg(0, true),
            Arg(2, false),
            Ptr(0, true, true),
        ], 
        vec![ // 85 
            Arg(1, true),
            Arg(2, false),
        ], 
        // AExp55
        vec![ // 86 
            Arg(2, true),
            Int(0),
            Arg(0, true),
        ], 
        // AExp56
        vec![ // 87 
            Arg(2, true),
            Int(1),
            Ptr(0, true, true),
        ], 
        vec![ // 88 
            Prm(Sub,false),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp57
        vec![ // 89 
            Prm(LE,false),
            Arg(2, false),
            Arg(0, false),
            Com(3,86),
            Com(3,87),
            Arg(0, false),
            Arg(2, false),
        ], 
        // AExp58
        vec![ // 90 
            Arg(3, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 91 
            Prm(Sub,false),
            Arg(1, true),
            Arg(2, true),
        ], 
        vec![ // 92 
            Prm(Add,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp59
        vec![ // 93 
            Prm(LE,false),
            Arg(2, false),
            Arg(1, false),
            Com(4,2),
            Com(4,90),
            Ptr(0, true, true),
            Arg(1, false),
            Arg(2, false),
        ], 
        vec![ // 94 
            Prm(Add,false),
            Arg(0, false),
            Arg(0, false),
        ], 
        // AExp60
        vec![ // 95 
            Com(1,98),
            Arg(0, true),
            Arg(1, true),
            Com(3,93),
        ], 
        // AExp61
        vec![ // 96 
            Prm(LE,false),
            Arg(2, false),
            Arg(0, false),
            Com(3,89),
            Com(2,95),
            Arg(0, false),
            Arg(2, false),
            Arg(1, true),
        ], 
        // AExp62
        vec![ // 97 
            Prm(Add,false),
            Arg(0, false),
            Arg(0, false),
        ], 
        // AExp63
        vec![ // 98 
            Com(3,84),
            Ptr(0, true, true),
            Com(1,97),
        ], 
        vec![ // 99 
            Com(3,96),
            Arg(0, true),
        ], 
        // AExp64
        vec![ // 100 
            Arg(0, true),
            Com(2,1),
            Com(2,0),
        ], 
        // AExp65
        vec![ // 101 
            Arg(6, true),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp66
        vec![ // 102 
            Com(7,101),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp67
        vec![ // 103 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 104 
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp68
        vec![ // 105 
            Int(0),
        ], 
        // AExp69
        vec![ // 106 
            Int(0),
        ], 
        // AExp70
        vec![ // 107 
            Com(3,103),
            Ptr(0, true, true),
            Com(1,0),
        ], 
        vec![ // 108 
            Arg(0, true),
            Prm(Add,false),
            Com(2,109),
            Com(2,117),
            Prm(Sub,false),
            Com(3,105),
            Com(5,106),
            Arg(1, true),
        ], 
        // AExp71
        vec![ // 109 
            Com(1,98),
            Arg(0, true),
            Arg(1, true),
            Com(2,0),
        ], 
        // AExp72
        vec![ // 110 
            Com(2,117),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 111 
            Prm(Add,false),
            Arg(1, false),
            Arg(1, false),
        ], 
        // AExp73
        vec![ // 112 
            Int(0),
        ], 
        // AExp74
        vec![ // 113 
            Prm(Add,false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 114 
            Prm(EQ,false),
            Arg(1, true),
            Int(0),
            Com(1,0),
            Com(1,112),
            Arg(2, false),
        ], 
        vec![ // 115 
            Com(2,110),
            Arg(0, true),
            Arg(2, false),
        ], 
        // AExp75
        vec![ // 116 
            Com(1,98),
            Arg(1, true),
            Int(2),
            Com(3,113),
            Arg(0, true),
        ], 
        // AExp76
        vec![ // 117 
            Prm(EQ,false),
            Arg(1, false),
            Int(1),
            Com(2,116),
            Com(2,0),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp77
        vec![ // 118 
            Arg(3, true),
        ], 
        // AExp78
        vec![ // 119 
            Arg(2, true),
        ], 
        // AExp79
        vec![ // 120 
            Arg(2, true),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 121 
            Com(4,2),
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp80
        vec![ // 122 
            Com(1,140),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 123 
            Com(1,134),
            Arg(1, true),
        ], 
        vec![ // 124 
            Com(1,146),
            Arg(0, true),
        ], 
        // AExp81
        vec![ // 125 
            Arg(1, true),
            Ptr(0, true, true),
            Com(1,0),
        ], 
        vec![ // 126 
            Com(4,2),
            Arg(0, true),
        ], 
        // AExp82
        vec![ // 127 
            Com(2,122),
            Ptr(0, true, true),
        ], 
        vec![ // 128 
            Com(2,125),
            Arg(0, true),
        ], 
        // AExp83
        vec![ // 129 
            Com(4,2),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 130 
            Com(1,127),
            Arg(0, false),
            Arg(1, false),
        ], 
        vec![ // 131 
            Com(3,120),
            Arg(0, false),
            Arg(1, false),
        ], 
        // AExp84
        vec![ // 132 
            Com(2,0),
        ], 
        // AExp85
        vec![ // 133 
            Com(1,41),
            Arg(1, false),
            Com(2,129),
            Com(2,132),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp86
        vec![ // 134 
            Arg(0, true),
            Com(2,0),
            Com(2,133),
        ], 
        // AExp87
        vec![ // 135 
            Com(2,0),
        ], 
        // AExp88
        vec![ // 136 
            Com(4,2),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 137 
            Arg(3, true),
            Arg(1, true),
        ], 
        vec![ // 138 
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp89
        vec![ // 139 
            Arg(2, true),
            Com(2,135),
            Com(4,136),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp90
        vec![ // 140 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 141 
            Com(3,139),
            Arg(0, true),
        ], 
        // AExp91
        vec![ // 142 
            Arg(4, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 143 
            Arg(3, true),
            Arg(1, true),
        ], 
        vec![ // 144 
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp92
        vec![ // 145 
            Arg(2, true),
            Com(5,142),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp93
        vec![ // 146 
            Arg(0, true),
            Com(3,145),
        ], 
        // AExp94
        vec![ // 147 
            Arg(5, true),
            Arg(0, true),
        ], 
        // AExp95
        vec![ // 148 
            Com(1,14),
            Com(1,153),
            Ptr(0, true, true),
        ], 
        vec![ // 149 
            Com(1,175),
            Arg(0, true),
        ], 
        // AExp96
        vec![ // 150 
            Com(1,14),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 151 
            Com(1,153),
            Arg(1, true),
        ], 
        vec![ // 152 
            Com(1,167),
            Arg(0, true),
        ], 
        // AExp97
        vec![ // 153 
            Arg(0, true),
            Ptr(0, true, true),
            Com(2,150),
        ], 
        vec![ // 154 
            Com(4,2),
            Com(2,0),
            Com(2,0),
        ], 
        // AExp98
        vec![ // 155 
            Com(4,2),
            Ptr(0, true, true),
            Com(2,0),
        ], 
        vec![ // 156 
            Com(4,2),
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp99
        vec![ // 157 
            Com(4,2),
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 158 
            Com(4,2),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp100
        vec![ // 159 
            Com(1,140),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 160 
            Arg(0, true),
            Arg(2, true),
        ], 
        vec![ // 161 
            Com(4,2),
            Arg(1, true),
        ], 
        // AExp101
        vec![ // 162 
            Com(4,2),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 163 
            Com(3,159),
            Arg(0, true),
            Arg(1, false),
            Arg(2, false),
        ], 
        vec![ // 164 
            Com(3,157),
            Arg(1, false),
            Arg(2, false),
            Arg(3, true),
        ], 
        // AExp102
        vec![ // 165 
            Arg(2, true),
            Com(1,155),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 166 
            Com(4,162),
            Arg(1, true),
        ], 
        // AExp103
        vec![ // 167 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 168 
            Com(3,165),
            Arg(0, true),
        ], 
        // AExp104
        vec![ // 169 
            Com(1,140),
            Ptr(0, true, true),
        ], 
        vec![ // 170 
            Com(4,2),
            Arg(0, true),
        ], 
        // AExp105
        vec![ // 171 
            Com(2,19),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 172 
            Com(1,169),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp106
        vec![ // 173 
            Com(2,171),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 174 
            Com(1,175),
            Arg(1, true),
        ], 
        // AExp107
        vec![ // 175 
            Arg(0, true),
            Ptr(0, true, true),
            Com(2,173),
        ], 
        vec![ // 176 
            Com(4,2),
            Com(2,0),
            Com(2,0),
        ], 
    ],

}});