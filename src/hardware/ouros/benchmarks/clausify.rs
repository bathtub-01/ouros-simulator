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
            Com(1,0),
            Com(1,5),
            Ptr(18, false, false),
        ], 
        vec![ // 1 
            Com(5,133),
            Int(0),
        ], 
        vec![ // 2 
            Com(5,133),
            Int(0),
        ], 
        vec![ // 3 
            Com(2,196),
            Ptr(2, false, false),
            Ptr(1, false, false),
        ], 
        vec![ // 4 
            Com(5,133),
            Int(0),
        ], 
        vec![ // 5 
            Com(2,196),
            Ptr(4, false, false),
        ], 
        vec![ // 6 
            Com(1,0),
            Ptr(5, false, false),
            Ptr(3, false, false),
        ], 
        vec![ // 7 
            Com(5,133),
            Int(0),
        ], 
        vec![ // 8 
            Com(5,133),
            Int(0),
        ], 
        vec![ // 9 
            Com(2,196),
            Ptr(8, false, false),
            Ptr(7, false, false),
        ], 
        vec![ // 10 
            Com(5,133),
            Int(0),
        ], 
        vec![ // 11 
            Com(2,196),
            Ptr(10, false, false),
        ], 
        vec![ // 12 
            Com(1,0),
            Ptr(11, false, false),
            Ptr(9, false, false),
        ], 
        vec![ // 13 
            Com(2,196),
            Ptr(12, false, false),
            Ptr(6, false, false),
        ], 
        vec![ // 14 
            Com(2,190),
            Int(2),
            Ptr(13, false, false),
        ], 
        vec![ // 15 
            Com(5,133),
            Int(0),
        ], 
        vec![ // 16 
            Com(2,13),
            Com(6,98),
            Ptr(15, false, false),
        ], 
        vec![ // 17 
            Com(1,0),
            Ptr(16, false, false),
            Ptr(14, false, false),
        ], 
        vec![ // 18 
            Com(1,0),
            Com(1,23),
            Ptr(17, false, false),
        ], 
        // AExp1
        vec![ // 19 
            Com(1,25),
            Ptr(21, false, false),
        ], 
        vec![ // 20 
            Com(2,32),
            Com(1,65),
        ], 
        vec![ // 21 
            Com(3,26),
            Ptr(20, false, false),
            Com(1,80),
        ], 
        // AExp2
        vec![ // 22 
            Com(1,48),
            Com(1,83),
        ], 
        // AExp3
        vec![ // 23 
            Com(1,94),
            Ptr(24, false, false),
        ], 
        vec![ // 24 
            Com(2,112),
            Com(1,88),
        ], 
        // AExp4
        vec![ // 25 
            Com(1,130),
            Com(2,0),
        ], 
        // AExp5
        vec![ // 26 
            Com(2,13),
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
            Prm(Add,false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 3 
            Com(1,5),
            Arg(1, true),
        ], 
        vec![ // 4 
            Com(1,9),
            Arg(0, true),
        ], 
        // AExp3
        vec![ // 5 
            Arg(0, true),
            Int(0),
            Com(2,2),
        ], 
        // AExp4
        vec![ // 6 
            Prm(Add,false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 7 
            Ptr(26, false, false),
            Arg(1, true),
        ], 
        vec![ // 8 
            Ptr(26, false, false),
            Arg(0, true),
        ], 
        // AExp5
        vec![ // 9 
            Arg(0, true),
            Com(2,6),
        ], 
        // AExp6
        vec![ // 10 
            Arg(3, true),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 11 
            Arg(4, true),
            Arg(1, true),
        ], 
        // AExp7
        vec![ // 12 
            Arg(3, true),
            Com(3,0),
            Com(5,10),
            Arg(1, true),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp8
        vec![ // 13 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 14 
            Com(4,12),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp9
        vec![ // 15 
            Com(1,0),
            Com(1,140),
            Ptr(0, true, true),
        ], 
        vec![ // 16 
            Com(1,184),
            Arg(0, true),
        ], 
        // AExp10
        vec![ // 17 
            Com(1,0),
            Ptr(25, false, false),
            Ptr(0, true, true),
        ], 
        vec![ // 18 
            Com(1,15),
            Arg(0, true),
        ], 
        // AExp11
        vec![ // 19 
            Com(1,0),
            Ptr(23, false, false),
            Ptr(0, true, true),
        ], 
        vec![ // 20 
            Com(1,17),
            Arg(0, true),
        ], 
        // AExp12
        vec![ // 21 
            Com(1,0),
            Ptr(22, false, false),
            Ptr(0, true, true),
        ], 
        vec![ // 22 
            Com(1,19),
            Arg(0, true),
        ], 
        // AExp13
        vec![ // 23 
            Com(1,0),
            Ptr(19, false, false),
            Ptr(0, true, true),
        ], 
        vec![ // 24 
            Com(1,21),
            Arg(0, true),
        ], 
        // AExp14
        vec![ // 25 
            Com(2,13),
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp15
        vec![ // 26 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 27 
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp16
        vec![ // 28 
            Com(2,37),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 29 
            Com(1,48),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp17
        vec![ // 30 
            Com(3,26),
            Com(1,50),
            Ptr(0, true, true),
        ], 
        vec![ // 31 
            Com(1,57),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp18
        vec![ // 32 
            Com(3,28),
            Ptr(0, true, true),
            Arg(1, false),
        ], 
        vec![ // 33 
            Com(2,30),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp19
        vec![ // 34 
            Com(4,39),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 35 
            Arg(3, true),
            Arg(1, true),
        ], 
        // AExp20
        vec![ // 36 
            Arg(2, true),
            Com(2,0),
            Com(4,34),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp21
        vec![ // 37 
            Y,
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 38 
            Com(3,36),
            Arg(1, true),
        ], 
        // AExp22
        vec![ // 39 
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp23
        vec![ // 40 
            Com(2,0),
        ], 
        // AExp24
        vec![ // 41 
            Arg(3, true),
        ], 
        // AExp25
        vec![ // 42 
            Com(4,39),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 43 
            Com(1,48),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp26
        vec![ // 44 
            Arg(3, false),
            Arg(1, false),
            Com(4,41),
            Com(4,42),
            Arg(1, false),
            Arg(3, false),
            Arg(2, false),
            Ptr(0, true, true),
        ], 
        vec![ // 45 
            Arg(0, true),
            Arg(2, false),
        ], 
        // AExp27
        vec![ // 46 
            Arg(2, true),
            Com(1,40),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 47 
            Com(4,44),
            Arg(1, true),
        ], 
        // AExp28
        vec![ // 48 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 49 
            Com(3,46),
            Arg(0, true),
        ], 
        // AExp29
        vec![ // 50 
            Arg(0, true),
            Com(2,1),
            Com(2,0),
        ], 
        // AExp30
        vec![ // 51 
            Com(2,0),
        ], 
        // AExp31
        vec![ // 52 
            Com(1,60),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 53 
            Arg(0, true),
            Arg(2, true),
            Arg(4, false),
        ], 
        vec![ // 54 
            Arg(3, true),
            Arg(1, true),
            Arg(4, false),
        ], 
        // AExp32
        vec![ // 55 
            Arg(2, true),
            Com(2,51),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 56 
            Com(5,52),
            Arg(1, true),
        ], 
        // AExp33
        vec![ // 57 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 58 
            Com(3,55),
            Arg(0, true),
        ], 
        // AExp34
        vec![ // 59 
            Com(2,1),
        ], 
        // AExp35
        vec![ // 60 
            Arg(0, true),
            Com(1,0),
            Com(1,59),
        ], 
        // AExp36
        vec![ // 61 
            Com(1,67),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 62 
            Com(1,78),
            Prm(EQ,false),
            Arg(3, true),
            Arg(1, true),
        ], 
        vec![ // 63 
            Com(1,78),
            Prm(EQ,false),
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp37
        vec![ // 64 
            Arg(2, true),
            Com(4,61),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp38
        vec![ // 65 
            Arg(0, true),
            Com(3,64),
        ], 
        // AExp39
        vec![ // 66 
            Com(2,0),
        ], 
        // AExp40
        vec![ // 67 
            Arg(0, true),
            Com(1,66),
            Com(1,0),
        ], 
        // AExp41
        vec![ // 68 
            Com(2,0),
        ], 
        // AExp42
        vec![ // 69 
            Arg(0, true),
            Com(2,1),
            Com(2,68),
        ], 
        // AExp43
        vec![ // 70 
            Com(2,0),
        ], 
        // AExp44
        vec![ // 71 
            Com(1,67),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 72 
            Arg(0, true),
            Arg(5, true),
            Arg(2, true),
        ], 
        vec![ // 73 
            Arg(3, true),
            Arg(4, true),
            Arg(1, true),
        ], 
        // AExp45
        vec![ // 74 
            Arg(3, true),
            Com(3,70),
            Ptr(0, true, true),
            Arg(4, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        vec![ // 75 
            Com(6,71),
            Arg(0, true),
        ], 
        // AExp46
        vec![ // 76 
            Arg(2, true),
            Com(2,69),
            Ptr(0, true, true),
            Arg(3, true),
            Arg(0, true),
        ], 
        vec![ // 77 
            Com(5,74),
            Arg(1, true),
        ], 
        // AExp47
        vec![ // 78 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 79 
            Com(4,76),
            Arg(0, true),
        ], 
        // AExp48
        vec![ // 80 
            Com(4,39),
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp49
        vec![ // 81 
            Com(1,85),
            Ptr(0, true, true),
        ], 
        vec![ // 82 
            Com(2,86),
            Prm(EQ,false),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp50
        vec![ // 83 
            Arg(0, true),
            Com(2,81),
        ], 
        // AExp51
        vec![ // 84 
            Com(2,0),
        ], 
        // AExp52
        vec![ // 85 
            Arg(0, true),
            Com(2,1),
            Com(2,84),
        ], 
        // AExp53
        vec![ // 86 
            Com(1,48),
            Ptr(0, true, true),
        ], 
        vec![ // 87 
            Com(1,57),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp54
        vec![ // 88 
            Arg(0, true),
            Com(2,0),
            Com(2,0),
        ], 
        // AExp55
        vec![ // 89 
            Com(2,0),
        ], 
        // AExp56
        vec![ // 90 
            Com(4,39),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 91 
            Arg(3, true),
            Arg(1, true),
        ], 
        vec![ // 92 
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp57
        vec![ // 93 
            Arg(2, true),
            Com(2,89),
            Com(4,90),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp58
        vec![ // 94 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 95 
            Com(3,93),
            Arg(0, true),
        ], 
        // AExp59
        vec![ // 96 
            Arg(0, true),
            Arg(2, true),
            Arg(1, true),
        ], 
        // AExp60
        vec![ // 97 
            Arg(2, true),
        ], 
        // AExp61
        vec![ // 98 
            Arg(2, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp62
        vec![ // 99 
            Com(2,112),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 100 
            Com(3,98),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp63
        vec![ // 101 
            Com(2,112),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 102 
            Com(3,99),
            Arg(0, true),
            Arg(3, true),
            Arg(4, true),
        ], 
        // AExp64
        vec![ // 103 
            Arg(4, true),
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 104 
            Com(2,119),
            Arg(0, true),
            Arg(3, true),
        ], 
        // AExp65
        vec![ // 105 
            Arg(0, true),
            Com(5,97),
            Com(5,97),
            Com(4,1),
            Com(5,103),
        ], 
        // AExp66
        vec![ // 106 
            Arg(4, true),
            Ptr(0, true, true),
            Arg(3, true),
        ], 
        vec![ // 107 
            Com(2,119),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp67
        vec![ // 108 
            Arg(1, true),
            Com(5,97),
            Com(5,101),
            Com(1,105),
            Com(5,106),
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp68
        vec![ // 109 
            Com(3,96),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 110 
            Com(3,108),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp69
        vec![ // 111 
            Arg(0, true),
            Com(2,0),
            Com(2,0),
        ], 
        // AExp70
        vec![ // 112 
            Arg(0, true),
            Com(3,109),
            Arg(1, true),
            Com(1,111),
        ], 
        // AExp71
        vec![ // 113 
            Com(4,39),
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp72
        vec![ // 114 
            Com(4,39),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 115 
            Com(2,119),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp73
        vec![ // 116 
            Com(4,39),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 117 
            Com(4,39),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp74
        vec![ // 118 
            Prm(LE,false),
            Arg(2, false),
            Arg(0, false),
            Com(3,114),
            Com(3,116),
            Arg(2, false),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp75
        vec![ // 119 
            Arg(1, true),
            Com(1,113),
            Com(3,118),
            Arg(0, true),
        ], 
        // AExp76
        vec![ // 120 
            Arg(6, true),
            Ptr(0, true, true),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
            Arg(4, true),
        ], 
        vec![ // 121 
            Arg(0, true),
            Arg(5, true),
        ], 
        // AExp77
        vec![ // 122 
            Com(1,130),
            Ptr(0, true, true),
            Arg(2, true),
        ], 
        vec![ // 123 
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp78
        vec![ // 124 
            Com(4,39),
            Ptr(0, true, true),
        ], 
        vec![ // 125 
            Com(6,39),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp79
        vec![ // 126 
            Com(4,39),
            Ptr(0, true, true),
        ], 
        vec![ // 127 
            Com(5,132),
            Arg(0, true),
        ], 
        // AExp80
        vec![ // 128 
            Com(4,39),
            Ptr(0, true, true),
        ], 
        vec![ // 129 
            Com(5,133),
            Arg(0, true),
        ], 
        // AExp81
        vec![ // 130 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 131 
            Com(7,120),
            Com(4,122),
            Com(2,124),
            Com(1,126),
            Com(1,128),
            Arg(0, true),
        ], 
        // AExp82
        vec![ // 132 
            Arg(3, true),
            Arg(0, true),
        ], 
        // AExp83
        vec![ // 133 
            Arg(4, true),
            Arg(0, true),
        ], 
        // AExp84
        vec![ // 134 
            Com(6,98),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 135 
            Com(1,140),
            Arg(1, true),
        ], 
        vec![ // 136 
            Com(1,140),
            Arg(0, true),
        ], 
        // AExp85
        vec![ // 137 
            Com(1,150),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 138 
            Com(1,140),
            Arg(1, true),
        ], 
        vec![ // 139 
            Com(1,140),
            Arg(0, true),
        ], 
        // AExp86
        vec![ // 140 
            Arg(0, true),
            Com(2,134),
            Com(2,137),
            Com(5,132),
            Com(5,133),
        ], 
        // AExp87
        vec![ // 141 
            Com(6,98),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 142 
            Com(1,150),
            Arg(1, true),
            Arg(2, false),
        ], 
        vec![ // 143 
            Com(1,150),
            Arg(0, true),
            Arg(2, false),
        ], 
        // AExp88
        vec![ // 144 
            Com(2,160),
            Ptr(0, true, true),
        ], 
        vec![ // 145 
            Com(6,39),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp89
        vec![ // 146 
            Com(2,160),
            Ptr(0, true, true),
        ], 
        vec![ // 147 
            Com(5,132),
            Arg(0, true),
        ], 
        // AExp90
        vec![ // 148 
            Com(2,160),
            Ptr(0, true, true),
        ], 
        vec![ // 149 
            Com(5,133),
            Arg(0, true),
        ], 
        // AExp91
        vec![ // 150 
            Arg(0, true),
            Com(3,141),
            Com(2,144),
            Com(1,146),
            Com(1,148),
        ], 
        // AExp92
        vec![ // 151 
            Com(6,98),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 152 
            Com(1,150),
            Arg(2, false),
            Arg(1, true),
        ], 
        vec![ // 153 
            Com(1,150),
            Arg(2, false),
            Arg(0, true),
        ], 
        // AExp93
        vec![ // 154 
            Com(6,39),
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 155 
            Com(6,39),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp94
        vec![ // 156 
            Com(6,39),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 157 
            Com(5,132),
            Arg(0, true),
        ], 
        // AExp95
        vec![ // 158 
            Com(6,39),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 159 
            Com(5,133),
            Arg(0, true),
        ], 
        // AExp96
        vec![ // 160 
            Arg(1, true),
            Com(3,151),
            Com(3,154),
            Com(2,156),
            Com(2,158),
            Arg(0, true),
        ], 
        // AExp97
        vec![ // 161 
            Com(6,98),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 162 
            Com(1,184),
            Arg(1, true),
        ], 
        vec![ // 163 
            Com(1,184),
            Arg(0, true),
        ], 
        // AExp98
        vec![ // 164 
            Com(6,39),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 165 
            Com(1,184),
            Arg(1, true),
        ], 
        vec![ // 166 
            Com(1,184),
            Arg(0, true),
        ], 
        // AExp99
        vec![ // 167 
            Com(1,184),
            Ptr(0, true, true),
        ], 
        vec![ // 168 
            Com(5,132),
            Arg(0, true),
        ], 
        // AExp100
        vec![ // 169 
            Com(1,184),
            Ptr(0, true, true),
        ], 
        vec![ // 170 
            Com(5,132),
            Arg(0, true),
        ], 
        // AExp101
        vec![ // 171 
            Com(6,39),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 172 
            Com(1,169),
            Arg(1, true),
        ], 
        vec![ // 173 
            Com(1,167),
            Arg(0, true),
        ], 
        // AExp102
        vec![ // 174 
            Com(1,184),
            Ptr(0, true, true),
        ], 
        vec![ // 175 
            Com(5,132),
            Arg(0, true),
        ], 
        // AExp103
        vec![ // 176 
            Com(1,184),
            Ptr(0, true, true),
        ], 
        vec![ // 177 
            Com(5,132),
            Arg(0, true),
        ], 
        // AExp104
        vec![ // 178 
            Com(6,98),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 179 
            Com(1,176),
            Arg(1, true),
        ], 
        vec![ // 180 
            Com(1,174),
            Arg(0, true),
        ], 
        // AExp105
        vec![ // 181 
            Com(5,132),
            Ptr(0, true, true),
        ], 
        vec![ // 182 
            Com(5,133),
            Arg(0, true),
        ], 
        // AExp106
        vec![ // 183 
            Arg(0, true),
            Com(2,171),
            Com(2,178),
            Com(1,184),
            Com(1,181),
        ], 
        // AExp107
        vec![ // 184 
            Arg(0, true),
            Com(2,161),
            Com(2,164),
            Com(1,183),
            Com(5,133),
        ], 
        // AExp108
        vec![ // 185 
            Com(2,0),
        ], 
        // AExp109
        vec![ // 186 
            Com(2,190),
            Ptr(0, true, true),
        ], 
        vec![ // 187 
            Prm(Sub,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp110
        vec![ // 188 
            Com(4,39),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 189 
            Com(1,186),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp111
        vec![ // 190 
            Prm(LE,false),
            Arg(0, false),
            Int(0),
            Com(1,0),
            Com(1,185),
            Ptr(0, true, true),
        ], 
        vec![ // 191 
            Com(2,188),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp112
        vec![ // 192 
            Com(6,39),
            Ptr(0, true, true),
        ], 
        vec![ // 193 
            Com(5,132),
            Arg(0, true),
        ], 
        // AExp113
        vec![ // 194 
            Com(6,39),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 195 
            Com(5,132),
            Arg(1, true),
        ], 
        // AExp114
        vec![ // 196 
            Com(6,98),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 197 
            Com(2,194),
            Arg(0, false),
            Arg(1, false),
        ], 
        vec![ // 198 
            Com(1,192),
            Arg(0, false),
            Arg(1, false),
        ], 
    ],

}});