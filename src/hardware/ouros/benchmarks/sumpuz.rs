use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 120
#[rustfmt::skip]
pub static SUMPUZ: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            Com(1,2),
            Ptr(9, false, false),
        ], 
        vec![ // 1 
            Com(4,3),
            Int(2),
            Com(2,0),
        ], 
        vec![ // 2 
            Com(4,3),
            Int(1),
            Ptr(1, false, false),
        ], 
        vec![ // 3 
            Com(4,3),
            Int(2),
            Ptr(2, false, false),
        ], 
        vec![ // 4 
            Com(4,3),
            Ptr(3, false, false),
            Com(2,0),
        ], 
        vec![ // 5 
            Com(4,3),
            Int(1),
            Com(2,0),
        ], 
        vec![ // 6 
            Com(4,3),
            Int(2),
            Ptr(5, false, false),
        ], 
        vec![ // 7 
            Com(4,3),
            Int(1),
            Ptr(6, false, false),
        ], 
        vec![ // 8 
            Com(4,3),
            Int(0),
            Ptr(7, false, false),
        ], 
        vec![ // 9 
            Com(4,3),
            Ptr(8, false, false),
            Ptr(4, false, false),
        ], 
        // AExp1
        vec![ // 10 
            Com(1,133),
            Ptr(11, false, false),
        ], 
        vec![ // 11 
            Com(3,118),
            Com(2,139),
        ], 
        // AExp2
        vec![ // 12 
            Com(1,116),
            Com(1,90),
        ], 
        // AExp3
        vec![ // 13 
            Y,
            Com(3,39),
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
            Com(3,4),
            Arg(0, false),
            Arg(0, false),
            Arg(0, false),
        ], 
        // AExp3
        vec![ // 3 
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp4
        vec![ // 4 
            Com(2,6),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 5 
            Com(3,15),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp5
        vec![ // 6 
            Com(1,13),
            Arg(0, true),
            Arg(1, true),
            Int(0),
        ], 
        // AExp6
        vec![ // 7 
            Prm(Add,false),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 8 
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp7
        vec![ // 9 
            Arg(0, true),
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 10 
            Com(3,7),
            Arg(1, true),
            Arg(3, true),
            Arg(4, true),
        ], 
        // AExp8
        vec![ // 11 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
            Arg(3, true),
            Arg(0, true),
        ], 
        vec![ // 12 
            Com(5,9),
            Arg(1, true),
        ], 
        // AExp9
        vec![ // 13 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 14 
            Com(4,11),
            Arg(0, true),
        ], 
        // AExp10
        vec![ // 15 
            Com(2,6),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 16 
            Com(3,17),
            Arg(2, true),
            Arg(1, true),
        ], 
        // AExp11
        vec![ // 17 
            Com(2,6),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 18 
            Com(3,19),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp12
        vec![ // 19 
            Com(3,32),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
            Int(0),
            Int(1),
        ], 
        // AExp13
        vec![ // 20 
            Prm(EQ,false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 21 
            Ptr(13, false, false),
            Arg(1, true),
        ], 
        vec![ // 22 
            Ptr(13, false, false),
            Arg(0, true),
        ], 
        // AExp14
        vec![ // 23 
            Prm(EQ,false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 24 
            Ptr(13, false, false),
            Arg(1, true),
        ], 
        vec![ // 25 
            Ptr(13, false, false),
            Arg(0, true),
        ], 
        // AExp15
        vec![ // 26 
            Arg(0, true),
            Int(0),
            Com(2,0),
        ], 
        // AExp16
        vec![ // 27 
            Com(1,0),
            Com(1,42),
            Ptr(0, true, true),
        ], 
        vec![ // 28 
            Com(4,87),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
            Com(1,26),
        ], 
        // AExp17
        vec![ // 29 
            Com(1,36),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 30 
            Com(3,27),
            Arg(0, false),
            Arg(1, true),
            Arg(2, false),
        ], 
        vec![ // 31 
            Com(2,23),
            Arg(0, false),
            Arg(2, false),
        ], 
        // AExp18
        vec![ // 32 
            Com(1,36),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 33 
            Com(3,29),
            Arg(0, false),
            Arg(1, false),
            Arg(2, true),
        ], 
        vec![ // 34 
            Com(2,20),
            Arg(0, false),
            Arg(1, false),
        ], 
        // AExp19
        vec![ // 35 
            Com(2,0),
        ], 
        // AExp20
        vec![ // 36 
            Arg(0, true),
            Com(1,35),
            Com(1,0),
        ], 
        // AExp21
        vec![ // 37 
            Arg(3, true),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 38 
            Prm(Add,false),
            Arg(2, true),
            Int(1),
        ], 
        // AExp22
        vec![ // 39 
            Arg(2, true),
            Com(2,0),
            Com(4,37),
            Arg(1, true),
            Arg(0, true),
        ], 
        // AExp23
        vec![ // 40 
            Com(2,0),
        ], 
        // AExp24
        vec![ // 41 
            Arg(1, true),
            Com(2,1),
            Com(2,40),
        ], 
        // AExp25
        vec![ // 42 
            Arg(0, true),
            Com(2,0),
            Com(2,41),
        ], 
        // AExp26
        vec![ // 43 
            Com(2,0),
        ], 
        // AExp27
        vec![ // 44 
            Com(4,3),
            Ptr(0, true, true),
            Com(2,0),
        ], 
        vec![ // 45 
            Com(1,90),
            Arg(0, true),
        ], 
        // AExp28
        vec![ // 46 
            Prm(EQ,false),
            Ptr(0, true, true),
            Int(0),
            Com(1,43),
            Com(1,44),
            Arg(0, false),
        ], 
        vec![ // 47 
            Com(1,89),
            Arg(0, false),
        ], 
        // AExp29
        vec![ // 48 
            Com(2,0),
        ], 
        // AExp30
        vec![ // 49 
            Com(3,102),
            Arg(0, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 50 
            Com(1,90),
            Arg(1, true),
        ], 
        vec![ // 51 
            Com(4,3),
            Int(1),
            Com(2,0),
        ], 
        // AExp31
        vec![ // 52 
            Prm(EQ,false),
            Ptr(0, true, true),
            Int(1),
            Com(2,48),
            Com(2,49),
            Arg(1, true),
            Arg(0, false),
        ], 
        vec![ // 53 
            Com(1,89),
            Arg(0, false),
        ], 
        // AExp32
        vec![ // 54 
            Com(2,0),
        ], 
        // AExp33
        vec![ // 55 
            Arg(1, true),
            Com(2,52),
            Com(4,54),
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp34
        vec![ // 56 
            Arg(0, true),
            Com(1,46),
            Com(3,55),
            Arg(1, true),
        ], 
        // AExp35
        vec![ // 57 
            Arg(4, true),
            Arg(0, true),
            Arg(1, true),
            Arg(5, true),
            Arg(2, true),
            Arg(6, true),
            Arg(3, true),
        ], 
        // AExp36
        vec![ // 58 
            Com(2,0),
        ], 
        // AExp37
        vec![ // 59 
            Com(1,0),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 60 
            Arg(1, true),
            Arg(3, false),
            Arg(4, false),
            Arg(5, false),
            Arg(6, false),
        ], 
        vec![ // 61 
            Arg(0, true),
            Arg(2, true),
            Arg(3, false),
            Arg(4, false),
            Arg(5, false),
            Arg(6, false),
        ], 
        // AExp38
        vec![ // 62 
            Com(1,175),
            Ptr(2, true, true),
            Arg(4, true),
            Ptr(1, true, true),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 63 
            Arg(0, true),
            Arg(2, true),
            Arg(5, false),
            Arg(6, true),
        ], 
        vec![ // 64 
            Com(1,189),
            Arg(5, false),
        ], 
        vec![ // 65 
            Com(1,89),
            Arg(3, true),
        ], 
        // AExp39
        vec![ // 66 
            Com(1,150),
            Ptr(0, true, true),
        ], 
        vec![ // 67 
            Com(7,62),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
            Arg(4, true),
            Arg(5, true),
            Arg(6, true),
        ], 
        // AExp40
        vec![ // 68 
            Com(4,87),
            Arg(2, true),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 69 
            Com(1,190),
            Arg(1, true),
        ], 
        // AExp41
        vec![ // 70 
            Com(3,102),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 71 
            Com(2,196),
            Arg(0, true),
            Int(9),
        ], 
        vec![ // 72 
            Com(1,189),
            Arg(1, true),
        ], 
        // AExp42
        vec![ // 73 
            Com(1,198),
            Ptr(0, true, true),
            Int(1),
            Int(0),
        ], 
        vec![ // 74 
            Com(1,190),
            Arg(0, true),
        ], 
        // AExp43
        vec![ // 75 
            Com(2,70),
            Ptr(0, true, true),
            Arg(0, false),
        ], 
        vec![ // 76 
            Com(1,73),
            Arg(0, false),
        ], 
        // AExp44
        vec![ // 77 
            Com(2,196),
            Ptr(0, true, true),
            Int(9),
        ], 
        vec![ // 78 
            Com(1,198),
            Arg(0, true),
            Int(1),
            Int(0),
        ], 
        // AExp45
        vec![ // 79 
            Com(3,102),
            Arg(1, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 80 
            Com(1,90),
            Arg(0, true),
        ], 
        vec![ // 81 
            Com(1,77),
            Arg(2, true),
        ], 
        // AExp46
        vec![ // 82 
            Com(1,150),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 83 
            Com(3,79),
            Arg(0, true),
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 84 
            Com(1,75),
            Arg(2, true),
        ], 
        // AExp47
        vec![ // 85 
            Com(7,59),
            Ptr(0, true, true),
            Com(4,82),
        ], 
        vec![ // 86 
            Com(7,66),
            Com(3,68),
            Arg(0, true),
        ], 
        // AExp48
        vec![ // 87 
            Arg(0, true),
            Com(3,56),
            Ptr(0, true, true),
            Arg(2, true),
            Arg(3, true),
            Arg(1, true),
        ], 
        vec![ // 88 
            Com(7,57),
            Com(4,58),
            Com(1,85),
        ], 
        // AExp49
        vec![ // 89 
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp50
        vec![ // 90 
            Arg(0, true),
            Com(2,1),
        ], 
        // AExp51
        vec![ // 91 
            Ptr(10, false, false),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 92 
            Ptr(12, false, false),
            Arg(0, true),
        ], 
        // AExp52
        vec![ // 93 
            Com(1,126),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 94 
            Com(2,91),
            Arg(0, true),
            Arg(2, true),
        ], 
        vec![ // 95 
            Com(1,127),
            Arg(1, true),
        ], 
        // AExp53
        vec![ // 96 
            Com(1,116),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 97 
            Com(3,93),
            Arg(0, false),
            Arg(1, true),
            Arg(2, true),
        ], 
        vec![ // 98 
            Com(3,118),
            Com(4,3),
            Arg(0, false),
        ], 
        // AExp54
        vec![ // 99 
            Com(2,0),
        ], 
        // AExp55
        vec![ // 100 
            Com(4,3),
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp56
        vec![ // 101 
            Com(2,143),
            Arg(0, true),
            Arg(3, true),
            Com(1,99),
            Com(1,100),
            Arg(1, true),
        ], 
        // AExp57
        vec![ // 102 
            Com(2,109),
            Arg(0, false),
            Arg(2, false),
            Com(3,96),
            Com(4,101),
            Arg(2, false),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp58
        vec![ // 103 
            Com(2,0),
        ], 
        // AExp59
        vec![ // 104 
            Com(2,109),
        ], 
        // AExp60
        vec![ // 105 
            Com(3,110),
            Arg(0, true),
        ], 
        // AExp61
        vec![ // 106 
            Prm(EQ,false),
            Arg(1, false),
            Arg(2, true),
            Com(1,104),
            Com(3,105),
            Arg(3, true),
            Arg(1, false),
            Arg(0, true),
        ], 
        // AExp62
        vec![ // 107 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 108 
            Com(4,106),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp63
        vec![ // 109 
            Arg(1, true),
            Com(1,103),
            Com(3,107),
            Arg(0, true),
        ], 
        // AExp64
        vec![ // 110 
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp65
        vec![ // 111 
            Com(2,0),
        ], 
        // AExp66
        vec![ // 112 
            Com(4,3),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 113 
            Arg(3, true),
            Arg(1, true),
        ], 
        vec![ // 114 
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp67
        vec![ // 115 
            Arg(2, true),
            Com(2,111),
            Com(4,112),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp68
        vec![ // 116 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 117 
            Com(3,115),
            Arg(0, true),
        ], 
        // AExp69
        vec![ // 118 
            Arg(0, true),
            Arg(2, true),
            Arg(1, true),
        ], 
        // AExp70
        vec![ // 119 
            Com(2,0),
        ], 
        // AExp71
        vec![ // 120 
            Com(2,0),
        ], 
        // AExp72
        vec![ // 121 
            Arg(2, true),
            Arg(1, true),
            Arg(0, true),
        ], 
        // AExp73
        vec![ // 122 
            Com(4,3),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 123 
            Com(1,126),
            Arg(3, true),
            Arg(1, true),
        ], 
        vec![ // 124 
            Com(3,121),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp74
        vec![ // 125 
            Arg(2, true),
            Com(2,120),
            Com(4,122),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp75
        vec![ // 126 
            Arg(0, true),
            Com(1,119),
            Com(3,125),
        ], 
        // AExp76
        vec![ // 127 
            Com(4,3),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 128 
            Com(1,127),
            Arg(0, false),
        ], 
        // AExp77
        vec![ // 129 
            Arg(0, true),
            Ptr(0, true, true),
            Arg(2, true),
        ], 
        vec![ // 130 
            Arg(4, true),
            Arg(3, true),
            Arg(1, true),
        ], 
        // AExp78
        vec![ // 131 
            Arg(3, true),
            Com(2,0),
            Ptr(0, true, true),
            Arg(2, true),
            Arg(0, true),
        ], 
        vec![ // 132 
            Com(5,129),
            Arg(1, true),
        ], 
        // AExp79
        vec![ // 133 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 134 
            Com(4,131),
            Arg(0, true),
        ], 
        // AExp80
        vec![ // 135 
            Com(2,0),
        ], 
        // AExp81
        vec![ // 136 
            Com(4,3),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 137 
            Com(2,139),
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp82
        vec![ // 138 
            Prm(EQ,false),
            Arg(2, false),
            Arg(0, false),
            Com(3,136),
            Com(3,0),
            Arg(1, true),
            Arg(0, false),
            Arg(2, false),
        ], 
        // AExp83
        vec![ // 139 
            Arg(1, true),
            Com(1,135),
            Com(3,138),
            Arg(0, true),
        ], 
        // AExp84
        vec![ // 140 
            Com(2,0),
        ], 
        // AExp85
        vec![ // 141 
            Com(2,1),
        ], 
        // AExp86
        vec![ // 142 
            Prm(EQ,false),
            Arg(2, false),
            Arg(0, true),
            Com(2,143),
            Com(2,141),
            Arg(2, false),
            Arg(1, true),
        ], 
        // AExp87
        vec![ // 143 
            Arg(1, true),
            Com(1,140),
            Com(3,142),
            Arg(0, true),
        ], 
        // AExp88
        vec![ // 144 
            Com(2,0),
        ], 
        // AExp89
        vec![ // 145 
            Com(2,155),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 146 
            Arg(0, true),
            Arg(2, true),
        ], 
        vec![ // 147 
            Arg(3, true),
            Arg(1, true),
        ], 
        // AExp90
        vec![ // 148 
            Arg(2, true),
            Com(1,144),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 149 
            Com(4,145),
            Arg(1, true),
        ], 
        // AExp91
        vec![ // 150 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 151 
            Com(3,148),
            Arg(0, true),
        ], 
        // AExp92
        vec![ // 152 
            Com(4,3),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 153 
            Arg(3, true),
            Arg(1, true),
        ], 
        // AExp93
        vec![ // 154 
            Arg(2, true),
            Com(2,0),
            Com(4,152),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp94
        vec![ // 155 
            Y,
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 156 
            Com(3,154),
            Arg(1, true),
        ], 
        // AExp95
        vec![ // 157 
            Arg(0, true),
            Arg(4, true),
            Arg(5, true),
            Arg(6, false),
            Ptr(0, true, true),
        ], 
        vec![ // 158 
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
            Arg(6, false),
        ], 
        // AExp96
        vec![ // 159 
            Com(3,178),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 160 
            Com(1,89),
            Arg(1, true),
        ], 
        // AExp97
        vec![ // 161 
            Com(4,3),
            Ptr(0, true, true),
            Com(2,0),
        ], 
        vec![ // 162 
            Com(1,90),
            Arg(0, true),
        ], 
        // AExp98
        vec![ // 163 
            Com(3,102),
            Arg(0, true),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 164 
            Com(1,161),
            Arg(2, true),
        ], 
        // AExp99
        vec![ // 165 
            Com(1,150),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 166 
            Com(3,163),
            Arg(0, true),
            Arg(2, true),
            Arg(3, false),
        ], 
        vec![ // 167 
            Com(2,159),
            Arg(1, true),
            Arg(3, false),
        ], 
        // AExp100
        vec![ // 168 
            Prm(Add,false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 169 
            Com(2,186),
            Arg(2, false),
            Arg(1, true),
        ], 
        vec![ // 170 
            Com(2,186),
            Arg(2, false),
            Arg(0, true),
        ], 
        // AExp101
        vec![ // 171 
            Prm(Add,false),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 172 
            Com(3,168),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        // AExp102
        vec![ // 173 
            Com(1,185),
            Ptr(0, true, true),
        ], 
        vec![ // 174 
            Com(4,171),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        // AExp103
        vec![ // 175 
            Com(7,157),
            Com(4,165),
            Ptr(0, true, true),
        ], 
        vec![ // 176 
            Com(4,173),
            Arg(0, true),
        ], 
        // AExp104
        vec![ // 177 
            Arg(2, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp105
        vec![ // 178 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 179 
            Com(3,177),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp106
        vec![ // 180 
            Arg(2, true),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 181 
            Prm(Add,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp107
        vec![ // 182 
            Com(1,185),
            Ptr(0, true, true),
            Com(3,180),
        ], 
        vec![ // 183 
            Prm(Sub,false),
            Arg(0, true),
            Int(10),
        ], 
        // AExp108
        vec![ // 184 
            Arg(1, true),
            Int(0),
            Arg(0, true),
        ], 
        // AExp109
        vec![ // 185 
            Prm(LE,false),
            Arg(0, false),
            Int(9),
            Com(1,182),
            Com(2,184),
            Arg(0, false),
        ], 
        // AExp110
        vec![ // 186 
            Com(1,188),
            Ptr(0, true, true),
        ], 
        vec![ // 187 
            Com(2,109),
            Arg(1, true),
            Arg(0, true),
        ], 
        // AExp111
        vec![ // 188 
            Arg(0, true),
            Err(4),
            Com(1,0),
        ], 
        // AExp112
        vec![ // 189 
            Arg(0, true),
            Err(3),
            Com(2,0),
        ], 
        // AExp113
        vec![ // 190 
            Arg(0, true),
            Com(2,0),
            Com(2,1),
        ], 
        // AExp114
        vec![ // 191 
            Com(2,0),
        ], 
        // AExp115
        vec![ // 192 
            Com(2,196),
            Ptr(0, true, true),
        ], 
        vec![ // 193 
            Prm(Add,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp116
        vec![ // 194 
            Com(4,3),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 195 
            Com(1,192),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp117
        vec![ // 196 
            Prm(LE,false),
            Arg(0, false),
            Arg(1, false),
            Com(2,191),
            Com(2,194),
            Arg(0, false),
            Arg(1, false),
        ], 
        // AExp118
        vec![ // 197 
            Arg(3, true),
        ], 
        // AExp119
        vec![ // 198 
            Arg(0, true),
            Com(2,0),
            Com(4,197),
        ], 
    ],

}});