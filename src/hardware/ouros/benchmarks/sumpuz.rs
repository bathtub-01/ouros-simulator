use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 109
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
            Com(1,140),
            Ptr(11, false, false),
        ], 
        vec![ // 11 
            Com(3,125),
            Com(2,145),
        ], 
        // AExp2
        vec![ // 12 
            Com(1,123),
            Com(1,95),
        ], 
        // AExp3
        vec![ // 13 
            Y,
            Com(3,38),
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
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp7
        vec![ // 9 
            Arg(1, true),
            Arg(4, true),
            Ptr(0, true, true),
        ], 
        vec![ // 10 
            Com(3,7),
            Arg(0, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        // AExp8
        vec![ // 11 
            Arg(2, true),
            Arg(3, false),
            Ptr(0, true, true),
        ], 
        vec![ // 12 
            Com(5,9),
            Arg(0, true),
            Arg(1, true),
            Arg(3, false),
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
            Com(4,91),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
            Com(1,26),
        ], 
        // AExp17
        vec![ // 29 
            Com(1,35),
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
            Com(1,35),
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
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp20
        vec![ // 36 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 37 
            Prm(Add,false),
            Arg(1, true),
            Int(1),
        ], 
        // AExp21
        vec![ // 38 
            Arg(2, true),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 39 
            Com(3,36),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp22
        vec![ // 40 
            Com(2,0),
        ], 
        // AExp23
        vec![ // 41 
            Arg(1, true),
            Com(2,1),
            Com(2,40),
        ], 
        // AExp24
        vec![ // 42 
            Arg(0, true),
            Com(2,0),
            Com(2,41),
        ], 
        // AExp25
        vec![ // 43 
            Com(2,0),
        ], 
        // AExp26
        vec![ // 44 
            Com(4,3),
            Ptr(0, true, true),
            Com(2,0),
        ], 
        vec![ // 45 
            Com(1,95),
            Arg(0, true),
        ], 
        // AExp27
        vec![ // 46 
            Prm(EQ,false),
            Ptr(0, true, true),
            Int(0),
            Com(1,43),
            Com(1,44),
            Arg(0, false),
        ], 
        vec![ // 47 
            Com(1,94),
            Arg(0, false),
        ], 
        // AExp28
        vec![ // 48 
            Com(2,0),
        ], 
        // AExp29
        vec![ // 49 
            Com(3,107),
            Arg(0, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 50 
            Com(1,95),
            Arg(1, true),
        ], 
        vec![ // 51 
            Com(4,3),
            Int(1),
            Com(2,0),
        ], 
        // AExp30
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
            Com(1,94),
            Arg(0, false),
        ], 
        // AExp31
        vec![ // 54 
            Com(2,0),
        ], 
        // AExp32
        vec![ // 55 
            Arg(2, true),
            Ptr(0, true, true),
            Com(2,54),
        ], 
        vec![ // 56 
            Com(2,52),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp33
        vec![ // 57 
            Arg(0, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 58 
            Com(3,55),
            Arg(1, false),
        ], 
        vec![ // 59 
            Com(1,46),
            Arg(1, false),
        ], 
        // AExp34
        vec![ // 60 
            Com(1,0),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 61 
            Com(1,156),
            Arg(1, true),
            Arg(4, true),
        ], 
        vec![ // 62 
            Arg(0, true),
            Arg(2, true),
            Arg(3, true),
            Arg(5, true),
            Arg(6, true),
        ], 
        // AExp35
        vec![ // 63 
            Com(1,182),
            Ptr(2, true, true),
            Arg(3, true),
            Ptr(1, true, true),
            Arg(5, true),
            Ptr(0, true, true),
        ], 
        vec![ // 64 
            Arg(2, true),
            Arg(4, true),
            Arg(6, true),
        ], 
        vec![ // 65 
            Com(1,195),
            Arg(1, true),
        ], 
        vec![ // 66 
            Com(1,94),
            Arg(0, true),
        ], 
        // AExp36
        vec![ // 67 
            Com(1,156),
            Ptr(0, true, true),
        ], 
        vec![ // 68 
            Com(7,63),
            Arg(2, true),
            Arg(0, true),
            Arg(1, true),
            Arg(3, true),
            Arg(4, true),
            Arg(5, true),
            Arg(6, true),
        ], 
        // AExp37
        vec![ // 69 
            Com(7,60),
            Ptr(1, true, true),
            Arg(1, true),
            Arg(5, false),
            Arg(6, false),
            Ptr(0, true, true),
        ], 
        vec![ // 70 
            Arg(2, true),
            Arg(4, false),
            Arg(5, false),
            Arg(6, false),
        ], 
        vec![ // 71 
            Com(7,67),
            Arg(3, true),
            Arg(0, true),
            Arg(4, false),
        ], 
        // AExp38
        vec![ // 72 
            Com(4,91),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 73 
            Com(1,196),
            Arg(0, true),
        ], 
        // AExp39
        vec![ // 74 
            Com(3,107),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 75 
            Com(2,202),
            Arg(0, true),
            Int(9),
        ], 
        vec![ // 76 
            Com(1,195),
            Arg(1, true),
        ], 
        // AExp40
        vec![ // 77 
            Com(3,203),
            Ptr(0, true, true),
            Int(1),
            Int(0),
        ], 
        vec![ // 78 
            Com(1,196),
            Arg(0, true),
        ], 
        // AExp41
        vec![ // 79 
            Com(2,74),
            Ptr(0, true, true),
            Arg(0, false),
        ], 
        vec![ // 80 
            Com(1,77),
            Arg(0, false),
        ], 
        // AExp42
        vec![ // 81 
            Com(2,202),
            Ptr(0, true, true),
            Int(9),
        ], 
        vec![ // 82 
            Com(3,203),
            Arg(0, true),
            Int(1),
            Int(0),
        ], 
        // AExp43
        vec![ // 83 
            Com(3,107),
            Arg(1, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 84 
            Com(1,95),
            Arg(0, true),
        ], 
        vec![ // 85 
            Com(1,81),
            Arg(2, true),
        ], 
        // AExp44
        vec![ // 86 
            Com(7,69),
            Ptr(1, true, true),
            Ptr(0, true, true),
            Com(3,83),
            Arg(0, false),
        ], 
        vec![ // 87 
            Com(1,79),
            Arg(0, false),
        ], 
        vec![ // 88 
            Com(2,72),
            Arg(0, false),
        ], 
        // AExp45
        vec![ // 89 
            Arg(1, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 90 
            Com(1,86),
            Arg(0, true),
            Arg(2, true),
            Arg(3, true),
            Arg(4, true),
        ], 
        // AExp46
        vec![ // 91 
            Arg(0, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 92 
            Com(5,89),
            Arg(1, true),
            Arg(2, false),
            Arg(3, false),
        ], 
        vec![ // 93 
            Com(2,57),
            Arg(2, false),
            Arg(3, false),
        ], 
        // AExp47
        vec![ // 94 
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp48
        vec![ // 95 
            Arg(0, true),
            Com(2,1),
        ], 
        // AExp49
        vec![ // 96 
            Ptr(10, false, false),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 97 
            Ptr(12, false, false),
            Arg(1, true),
        ], 
        // AExp50
        vec![ // 98 
            Com(2,132),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 99 
            Com(2,96),
            Arg(1, true),
            Arg(2, true),
        ], 
        vec![ // 100 
            Com(1,134),
            Arg(0, true),
        ], 
        // AExp51
        vec![ // 101 
            Com(1,123),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 102 
            Com(3,98),
            Arg(0, true),
            Arg(1, true),
            Arg(2, false),
        ], 
        vec![ // 103 
            Com(3,125),
            Com(4,3),
            Arg(2, false),
        ], 
        // AExp52
        vec![ // 104 
            Com(2,0),
        ], 
        // AExp53
        vec![ // 105 
            Com(4,3),
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp54
        vec![ // 106 
            Com(2,149),
            Arg(2, true),
            Arg(0, true),
            Com(1,104),
            Com(1,105),
            Arg(1, true),
        ], 
        // AExp55
        vec![ // 107 
            Com(2,115),
            Arg(0, false),
            Arg(2, false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 108 
            Com(3,106),
            Arg(1, false),
            Arg(2, false),
        ], 
        vec![ // 109 
            Com(3,101),
            Arg(0, false),
            Arg(1, false),
            Arg(2, false),
        ], 
        // AExp56
        vec![ // 110 
            Com(2,115),
        ], 
        // AExp57
        vec![ // 111 
            Com(3,117),
            Arg(0, true),
        ], 
        // AExp58
        vec![ // 112 
            Prm(EQ,false),
            Arg(0, false),
            Arg(2, true),
            Com(1,110),
            Com(3,111),
            Arg(3, true),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp59
        vec![ // 113 
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 114 
            Com(4,112),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp60
        vec![ // 115 
            Arg(1, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 116 
            Com(3,113),
            Arg(0, true),
        ], 
        // AExp61
        vec![ // 117 
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp62
        vec![ // 118 
            Com(4,3),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 119 
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 120 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp63
        vec![ // 121 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 122 
            Com(4,118),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp64
        vec![ // 123 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 124 
            Com(3,121),
            Arg(0, true),
        ], 
        // AExp65
        vec![ // 125 
            Arg(0, true),
            Arg(2, true),
            Arg(1, true),
        ], 
        // AExp66
        vec![ // 126 
            Arg(2, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp67
        vec![ // 127 
            Com(4,3),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 128 
            Com(2,132),
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 129 
            Com(3,126),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp68
        vec![ // 130 
            Arg(0, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 131 
            Com(4,127),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp69
        vec![ // 132 
            Arg(0, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 133 
            Com(3,130),
            Arg(1, true),
        ], 
        // AExp70
        vec![ // 134 
            Com(4,3),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 135 
            Com(1,134),
            Arg(0, false),
        ], 
        // AExp71
        vec![ // 136 
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 137 
            Arg(0, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        // AExp72
        vec![ // 138 
            Arg(3, true),
            Arg(2, false),
            Ptr(0, true, true),
        ], 
        vec![ // 139 
            Com(4,136),
            Arg(0, true),
            Arg(1, true),
            Arg(2, false),
        ], 
        // AExp73
        vec![ // 140 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 141 
            Com(4,138),
            Arg(0, true),
        ], 
        // AExp74
        vec![ // 142 
            Com(4,3),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 143 
            Com(2,145),
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp75
        vec![ // 144 
            Prm(EQ,false),
            Arg(0, false),
            Arg(1, false),
            Com(3,142),
            Com(3,0),
            Arg(2, true),
            Arg(1, false),
            Arg(0, false),
        ], 
        // AExp76
        vec![ // 145 
            Arg(1, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 146 
            Com(3,144),
            Arg(0, true),
        ], 
        // AExp77
        vec![ // 147 
            Com(2,1),
        ], 
        // AExp78
        vec![ // 148 
            Prm(EQ,false),
            Arg(0, false),
            Arg(1, true),
            Com(2,149),
            Com(2,147),
            Arg(0, false),
        ], 
        // AExp79
        vec![ // 149 
            Arg(1, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 150 
            Com(2,148),
            Arg(0, true),
        ], 
        // AExp80
        vec![ // 151 
            Com(2,162),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 152 
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 153 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp81
        vec![ // 154 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 155 
            Com(4,151),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp82
        vec![ // 156 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 157 
            Com(3,154),
            Arg(0, true),
        ], 
        // AExp83
        vec![ // 158 
            Com(4,3),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 159 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp84
        vec![ // 160 
            Arg(2, true),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 161 
            Com(3,158),
            Arg(1, true),
        ], 
        // AExp85
        vec![ // 162 
            Y,
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 163 
            Com(3,160),
            Arg(1, true),
        ], 
        // AExp86
        vec![ // 164 
            Arg(0, true),
            Arg(4, true),
            Arg(5, true),
            Arg(6, false),
            Ptr(0, true, true),
        ], 
        vec![ // 165 
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
            Arg(6, false),
        ], 
        // AExp87
        vec![ // 166 
            Com(3,184),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 167 
            Com(1,94),
            Arg(1, true),
        ], 
        // AExp88
        vec![ // 168 
            Com(4,3),
            Ptr(0, true, true),
            Com(2,0),
        ], 
        vec![ // 169 
            Com(1,95),
            Arg(0, true),
        ], 
        // AExp89
        vec![ // 170 
            Com(3,107),
            Arg(0, true),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 171 
            Com(1,168),
            Arg(2, true),
        ], 
        // AExp90
        vec![ // 172 
            Com(1,156),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 173 
            Com(3,170),
            Arg(0, true),
            Arg(2, true),
            Arg(3, false),
        ], 
        vec![ // 174 
            Com(2,166),
            Arg(1, true),
            Arg(3, false),
        ], 
        // AExp91
        vec![ // 175 
            Prm(Add,false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 176 
            Com(2,192),
            Arg(2, false),
            Arg(1, true),
        ], 
        vec![ // 177 
            Com(2,192),
            Arg(2, false),
            Arg(0, true),
        ], 
        // AExp92
        vec![ // 178 
            Prm(Add,false),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 179 
            Com(3,175),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        // AExp93
        vec![ // 180 
            Com(1,191),
            Ptr(0, true, true),
        ], 
        vec![ // 181 
            Com(4,178),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        // AExp94
        vec![ // 182 
            Com(7,164),
            Com(4,172),
            Ptr(0, true, true),
        ], 
        vec![ // 183 
            Com(4,180),
            Arg(0, true),
        ], 
        // AExp95
        vec![ // 184 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 185 
            Com(3,126),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp96
        vec![ // 186 
            Arg(2, true),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 187 
            Prm(Add,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp97
        vec![ // 188 
            Com(1,191),
            Ptr(0, true, true),
            Com(3,186),
        ], 
        vec![ // 189 
            Prm(Sub,false),
            Arg(0, true),
            Int(10),
        ], 
        // AExp98
        vec![ // 190 
            Arg(1, true),
            Int(0),
            Arg(0, true),
        ], 
        // AExp99
        vec![ // 191 
            Prm(LE,false),
            Arg(0, false),
            Int(9),
            Com(1,188),
            Com(2,190),
            Arg(0, false),
        ], 
        // AExp100
        vec![ // 192 
            Com(1,194),
            Ptr(0, true, true),
        ], 
        vec![ // 193 
            Com(2,115),
            Arg(1, true),
            Arg(0, true),
        ], 
        // AExp101
        vec![ // 194 
            Arg(0, true),
            Err(4),
            Com(1,0),
        ], 
        // AExp102
        vec![ // 195 
            Arg(0, true),
            Err(3),
            Com(2,0),
        ], 
        // AExp103
        vec![ // 196 
            Arg(0, true),
            Com(2,0),
            Com(2,1),
        ], 
        // AExp104
        vec![ // 197 
            Com(2,0),
        ], 
        // AExp105
        vec![ // 198 
            Com(2,202),
            Ptr(0, true, true),
        ], 
        vec![ // 199 
            Prm(Add,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp106
        vec![ // 200 
            Com(4,3),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 201 
            Com(1,198),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp107
        vec![ // 202 
            Prm(LE,false),
            Arg(0, false),
            Arg(1, false),
            Com(2,197),
            Com(2,200),
            Arg(0, false),
            Arg(1, false),
        ], 
        // AExp108
        vec![ // 203 
            Arg(0, true),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 204 
            Com(3,0),
            Arg(2, true),
        ], 
    ],

}});