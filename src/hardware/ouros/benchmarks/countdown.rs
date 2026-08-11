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
            Ptr(7, false, false),
            Ptr(6, false, false),
        ], 
        vec![ // 1 
            Com(4,2),
            Int(25),
            Com(2,0),
        ], 
        vec![ // 2 
            Com(4,2),
            Int(10),
            Ptr(1, false, false),
        ], 
        vec![ // 3 
            Com(4,2),
            Int(7),
            Ptr(2, false, false),
        ], 
        vec![ // 4 
            Com(4,2),
            Int(3),
            Ptr(3, false, false),
        ], 
        vec![ // 5 
            Com(4,2),
            Int(1),
            Ptr(4, false, false),
        ], 
        vec![ // 6 
            Com(2,6),
            Ptr(5, false, false),
            Int(765),
        ], 
        // AExp1
        vec![ // 7 
            Y,
            Com(3,5),
            Int(0),
        ], 
        // AExp2
        vec![ // 8 
            Com(3,57),
            Ptr(13, false, false),
        ], 
        vec![ // 9 
            Com(4,2),
            Com(6,1),
            Com(2,0),
        ], 
        vec![ // 10 
            Com(4,2),
            Com(6,124),
            Ptr(9, false, false),
        ], 
        vec![ // 11 
            Com(1,63),
            Ptr(10, false, false),
        ], 
        vec![ // 12 
            Com(5,61),
            Ptr(11, false, false),
        ], 
        vec![ // 13 
            Com(4,59),
            Ptr(12, false, false),
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
            Com(1,155),
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
            Com(2,30),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 22 
            Com(1,40),
            Arg(1, true),
        ], 
        // AExp14
        vec![ // 23 
            Com(2,0),
        ], 
        // AExp15
        vec![ // 24 
            Com(2,30),
        ], 
        // AExp16
        vec![ // 25 
            Com(4,2),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 26 
            Com(2,30),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp17
        vec![ // 27 
            Prm(EQ,false),
            Arg(3, true),
            Arg(1, false),
            Com(1,24),
            Com(3,25),
            Arg(2, true),
            Arg(1, false),
            Arg(0, true),
        ], 
        // AExp18
        vec![ // 28 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 29 
            Com(4,27),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp19
        vec![ // 30 
            Arg(1, true),
            Com(1,23),
            Com(3,28),
            Arg(0, true),
        ], 
        // AExp20
        vec![ // 31 
            Com(1,139),
            Ptr(0, true, true),
        ], 
        vec![ // 32 
            Com(4,2),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp21
        vec![ // 33 
            Com(1,14),
            Com(1,46),
            Ptr(0, true, true),
        ], 
        vec![ // 34 
            Com(2,31),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp22
        vec![ // 35 
            Arg(1, true),
            Ptr(0, true, true),
            Arg(0, false),
        ], 
        vec![ // 36 
            Com(7,154),
            Arg(0, false),
        ], 
        // AExp23
        vec![ // 37 
            Com(4,2),
            Ptr(0, true, true),
            Com(2,0),
        ], 
        vec![ // 38 
            Com(2,35),
            Arg(0, true),
        ], 
        // AExp24
        vec![ // 39 
            Com(1,42),
            Arg(1, false),
            Com(2,33),
            Com(2,37),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp25
        vec![ // 40 
            Arg(0, true),
            Com(2,0),
            Com(2,39),
        ], 
        // AExp26
        vec![ // 41 
            Com(2,0),
        ], 
        // AExp27
        vec![ // 42 
            Arg(0, true),
            Com(2,1),
            Com(2,41),
        ], 
        // AExp28
        vec![ // 43 
            Com(1,55),
            Ptr(8, false, false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 44 
            Com(1,40),
            Arg(1, true),
        ], 
        vec![ // 45 
            Com(1,40),
            Arg(0, true),
        ], 
        // AExp29
        vec![ // 46 
            Arg(0, true),
            Com(2,43),
        ], 
        // AExp30
        vec![ // 47 
            Com(2,0),
        ], 
        // AExp31
        vec![ // 48 
            Com(1,14),
            Ptr(0, true, true),
        ], 
        vec![ // 49 
            Arg(1, true),
            Arg(0, true),
        ], 
        // AExp32
        vec![ // 50 
            Com(2,19),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 51 
            Arg(0, true),
            Arg(2, true),
            Arg(4, false),
        ], 
        vec![ // 52 
            Com(2,48),
            Arg(1, true),
            Arg(3, true),
            Arg(4, false),
        ], 
        // AExp33
        vec![ // 53 
            Arg(2, true),
            Com(2,47),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 54 
            Com(5,50),
            Arg(1, true),
        ], 
        // AExp34
        vec![ // 55 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 56 
            Com(3,53),
            Arg(0, true),
        ], 
        // AExp35
        vec![ // 57 
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 58 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp36
        vec![ // 59 
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 60 
            Arg(0, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        // AExp37
        vec![ // 61 
            Com(1,14),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 62 
            Com(2,75),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
            Arg(4, true),
        ], 
        // AExp38
        vec![ // 63 
            Com(4,2),
            Com(6,0),
            Ptr(0, true, true),
        ], 
        vec![ // 64 
            Com(4,2),
            Com(6,123),
            Arg(0, true),
        ], 
        // AExp39
        vec![ // 65 
            Arg(0, true),
            Arg(4, true),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        // AExp40
        vec![ // 66 
            Com(5,65),
            Ptr(0, true, true),
            Arg(2, true),
            Arg(1, true),
            Arg(3, false),
        ], 
        vec![ // 67 
            Arg(0, true),
            Arg(3, false),
        ], 
        // AExp41
        vec![ // 68 
            Com(2,0),
        ], 
        // AExp42
        vec![ // 69 
            Arg(5, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 70 
            Com(2,111),
            Arg(0, false),
            Arg(3, true),
            Arg(4, true),
        ], 
        vec![ // 71 
            Com(5,106),
            Arg(0, false),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp43
        vec![ // 72 
            Com(4,2),
            Ptr(0, true, true),
            Com(2,0),
        ], 
        vec![ // 73 
            Com(6,69),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
            Arg(4, true),
        ], 
        // AExp44
        vec![ // 74 
            Com(1,85),
            Arg(3, false),
            Arg(1, true),
            Arg(2, true),
            Com(5,68),
            Com(5,72),
            Arg(3, false),
            Arg(0, true),
        ], 
        // AExp45
        vec![ // 75 
            Com(4,66),
            Ptr(0, true, true),
            Arg(1, false),
        ], 
        vec![ // 76 
            Com(4,74),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp46
        vec![ // 77 
            Com(2,1),
        ], 
        // AExp47
        vec![ // 78 
            Prm(EQ,false),
            Ptr(0, true, true),
            Int(0),
        ], 
        vec![ // 79 
            Com(2,86),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp48
        vec![ // 80 
            Com(2,1),
        ], 
        // AExp49
        vec![ // 81 
            Com(1,104),
            Ptr(0, true, true),
        ], 
        vec![ // 82 
            Prm(LE,false),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp50
        vec![ // 83 
            Com(2,0),
        ], 
        // AExp51
        vec![ // 84 
            Com(2,0),
        ], 
        // AExp52
        vec![ // 85 
            Arg(0, true),
            Com(2,77),
            Com(2,78),
            Com(2,80),
            Com(2,81),
            Com(3,83),
            Com(5,84),
        ], 
        // AExp53
        vec![ // 86 
            Com(1,102),
            Arg(0, true),
            Arg(1, true),
            Com(2,1),
        ], 
        // AExp54
        vec![ // 87 
            Arg(0, true),
            Arg(2, false),
            Ptr(0, true, true),
        ], 
        vec![ // 88 
            Arg(1, true),
            Arg(2, false),
        ], 
        // AExp55
        vec![ // 89 
            Arg(2, true),
            Int(0),
            Arg(0, true),
        ], 
        // AExp56
        vec![ // 90 
            Arg(2, true),
            Int(1),
            Ptr(0, true, true),
        ], 
        vec![ // 91 
            Prm(Sub,false),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp57
        vec![ // 92 
            Prm(LE,false),
            Arg(2, false),
            Arg(0, false),
            Com(3,89),
            Com(3,90),
            Arg(0, false),
            Arg(2, false),
        ], 
        // AExp58
        vec![ // 93 
            Arg(3, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 94 
            Prm(Sub,false),
            Arg(1, true),
            Arg(2, true),
        ], 
        vec![ // 95 
            Prm(Add,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp59
        vec![ // 96 
            Prm(LE,false),
            Arg(0, false),
            Arg(2, false),
            Com(4,2),
            Com(4,93),
            Ptr(0, true, true),
            Arg(2, false),
            Arg(0, false),
        ], 
        vec![ // 97 
            Prm(Add,false),
            Arg(1, false),
            Arg(1, false),
        ], 
        // AExp60
        vec![ // 98 
            Com(1,102),
            Arg(0, true),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 99 
            Com(3,96),
            Arg(2, true),
        ], 
        // AExp61
        vec![ // 100 
            Prm(LE,false),
            Arg(2, false),
            Arg(0, false),
            Com(3,92),
            Com(3,98),
            Arg(0, false),
            Arg(2, false),
            Arg(1, true),
        ], 
        // AExp62
        vec![ // 101 
            Prm(Add,false),
            Arg(0, false),
            Arg(0, false),
        ], 
        // AExp63
        vec![ // 102 
            Com(3,87),
            Ptr(0, true, true),
            Com(1,101),
        ], 
        vec![ // 103 
            Com(3,100),
            Arg(0, true),
        ], 
        // AExp64
        vec![ // 104 
            Arg(0, true),
            Com(2,1),
            Com(2,0),
        ], 
        // AExp65
        vec![ // 105 
            Arg(6, true),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp66
        vec![ // 106 
            Com(7,105),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp67
        vec![ // 107 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 108 
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp68
        vec![ // 109 
            Int(0),
        ], 
        // AExp69
        vec![ // 110 
            Int(0),
        ], 
        // AExp70
        vec![ // 111 
            Com(3,107),
            Ptr(0, true, true),
            Com(1,0),
        ], 
        vec![ // 112 
            Arg(0, true),
            Prm(Add,false),
            Com(2,113),
            Com(2,122),
            Prm(Sub,false),
            Com(3,109),
            Com(5,110),
            Arg(1, true),
        ], 
        // AExp71
        vec![ // 113 
            Com(1,102),
            Arg(0, true),
            Arg(1, true),
            Com(2,0),
        ], 
        // AExp72
        vec![ // 114 
            Com(2,122),
            Ptr(0, true, true),
        ], 
        vec![ // 115 
            Prm(Add,false),
            Arg(0, false),
            Arg(0, false),
        ], 
        // AExp73
        vec![ // 116 
            Int(0),
        ], 
        // AExp74
        vec![ // 117 
            Prm(Add,false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 118 
            Prm(EQ,false),
            Arg(2, true),
            Int(0),
            Com(1,0),
            Com(1,116),
            Arg(0, false),
        ], 
        vec![ // 119 
            Com(1,114),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp75
        vec![ // 120 
            Com(1,102),
            Arg(1, true),
            Int(2),
            Ptr(0, true, true),
        ], 
        vec![ // 121 
            Com(3,117),
            Arg(0, true),
        ], 
        // AExp76
        vec![ // 122 
            Prm(EQ,false),
            Arg(1, false),
            Int(1),
            Com(2,120),
            Com(2,0),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp77
        vec![ // 123 
            Arg(3, true),
        ], 
        // AExp78
        vec![ // 124 
            Arg(2, true),
        ], 
        // AExp79
        vec![ // 125 
            Arg(2, true),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 126 
            Com(4,2),
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp80
        vec![ // 127 
            Com(1,145),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 128 
            Com(1,139),
            Arg(1, true),
        ], 
        vec![ // 129 
            Com(2,152),
            Arg(0, true),
        ], 
        // AExp81
        vec![ // 130 
            Arg(1, true),
            Ptr(0, true, true),
            Com(1,0),
        ], 
        vec![ // 131 
            Com(4,2),
            Arg(0, true),
        ], 
        // AExp82
        vec![ // 132 
            Com(2,127),
            Ptr(0, true, true),
        ], 
        vec![ // 133 
            Com(2,130),
            Arg(0, true),
        ], 
        // AExp83
        vec![ // 134 
            Com(4,2),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 135 
            Com(1,132),
            Arg(0, false),
            Arg(1, false),
        ], 
        vec![ // 136 
            Com(3,125),
            Arg(0, false),
            Arg(1, false),
        ], 
        // AExp84
        vec![ // 137 
            Com(2,0),
        ], 
        // AExp85
        vec![ // 138 
            Com(1,42),
            Arg(1, false),
            Com(2,134),
            Com(2,137),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp86
        vec![ // 139 
            Arg(0, true),
            Com(2,0),
            Com(2,138),
        ], 
        // AExp87
        vec![ // 140 
            Com(2,0),
        ], 
        // AExp88
        vec![ // 141 
            Com(4,2),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 142 
            Arg(3, true),
            Arg(1, true),
        ], 
        vec![ // 143 
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp89
        vec![ // 144 
            Arg(2, true),
            Com(2,140),
            Com(4,141),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp90
        vec![ // 145 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 146 
            Com(3,144),
            Arg(0, true),
        ], 
        // AExp91
        vec![ // 147 
            Arg(4, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 148 
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 149 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp92
        vec![ // 150 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 151 
            Com(5,147),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp93
        vec![ // 152 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 153 
            Com(3,150),
            Arg(1, true),
        ], 
        // AExp94
        vec![ // 154 
            Arg(5, true),
            Arg(0, true),
        ], 
        // AExp95
        vec![ // 155 
            Com(1,14),
            Com(1,160),
            Ptr(0, true, true),
        ], 
        vec![ // 156 
            Com(1,182),
            Arg(0, true),
        ], 
        // AExp96
        vec![ // 157 
            Com(1,14),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 158 
            Com(1,160),
            Arg(1, true),
        ], 
        vec![ // 159 
            Com(1,174),
            Arg(0, true),
        ], 
        // AExp97
        vec![ // 160 
            Arg(0, true),
            Ptr(0, true, true),
            Com(2,157),
        ], 
        vec![ // 161 
            Com(4,2),
            Com(2,0),
            Com(2,0),
        ], 
        // AExp98
        vec![ // 162 
            Com(4,2),
            Ptr(0, true, true),
            Com(2,0),
        ], 
        vec![ // 163 
            Com(4,2),
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp99
        vec![ // 164 
            Com(4,2),
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 165 
            Com(4,2),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp100
        vec![ // 166 
            Com(1,145),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 167 
            Arg(0, true),
            Arg(2, true),
        ], 
        vec![ // 168 
            Com(4,2),
            Arg(1, true),
        ], 
        // AExp101
        vec![ // 169 
            Com(4,2),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 170 
            Com(3,166),
            Arg(0, true),
            Arg(1, false),
            Arg(2, false),
        ], 
        vec![ // 171 
            Com(3,164),
            Arg(1, false),
            Arg(2, false),
            Arg(3, true),
        ], 
        // AExp102
        vec![ // 172 
            Arg(2, true),
            Com(1,162),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 173 
            Com(4,169),
            Arg(1, true),
        ], 
        // AExp103
        vec![ // 174 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 175 
            Com(3,172),
            Arg(0, true),
        ], 
        // AExp104
        vec![ // 176 
            Com(1,145),
            Ptr(0, true, true),
        ], 
        vec![ // 177 
            Com(4,2),
            Arg(0, true),
        ], 
        // AExp105
        vec![ // 178 
            Com(2,19),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 179 
            Com(1,176),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp106
        vec![ // 180 
            Com(2,178),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 181 
            Com(1,182),
            Arg(1, true),
        ], 
        // AExp107
        vec![ // 182 
            Arg(0, true),
            Ptr(0, true, true),
            Com(2,180),
        ], 
        vec![ // 183 
            Com(4,2),
            Com(2,0),
            Com(2,0),
        ], 
    ],

}});