use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 105
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
            Com(5,147),
            Int(0),
        ], 
        vec![ // 2 
            Com(5,147),
            Int(0),
        ], 
        vec![ // 3 
            Com(2,217),
            Ptr(2, false, false),
            Ptr(1, false, false),
        ], 
        vec![ // 4 
            Com(5,147),
            Int(0),
        ], 
        vec![ // 5 
            Com(2,217),
            Ptr(4, false, false),
        ], 
        vec![ // 6 
            Com(1,0),
            Ptr(5, false, false),
            Ptr(3, false, false),
        ], 
        vec![ // 7 
            Com(5,147),
            Int(0),
        ], 
        vec![ // 8 
            Com(5,147),
            Int(0),
        ], 
        vec![ // 9 
            Com(2,217),
            Ptr(8, false, false),
            Ptr(7, false, false),
        ], 
        vec![ // 10 
            Com(5,147),
            Int(0),
        ], 
        vec![ // 11 
            Com(2,217),
            Ptr(10, false, false),
        ], 
        vec![ // 12 
            Com(1,0),
            Ptr(11, false, false),
            Ptr(9, false, false),
        ], 
        vec![ // 13 
            Com(2,217),
            Ptr(12, false, false),
            Ptr(6, false, false),
        ], 
        vec![ // 14 
            Com(2,211),
            Int(2),
            Ptr(13, false, false),
        ], 
        vec![ // 15 
            Com(5,147),
            Int(0),
        ], 
        vec![ // 16 
            Com(2,14),
            Com(6,101),
            Ptr(15, false, false),
        ], 
        vec![ // 17 
            Com(1,0),
            Ptr(16, false, false),
            Ptr(14, false, false),
        ], 
        vec![ // 18 
            Com(1,0),
            Com(1,24),
            Ptr(17, false, false),
        ], 
        // AExp1
        vec![ // 19 
            Com(1,26),
            Ptr(21, false, false),
        ], 
        vec![ // 20 
            Com(2,33),
            Com(2,65),
        ], 
        vec![ // 21 
            Com(3,27),
            Ptr(20, false, false),
            Com(1,79),
        ], 
        // AExp2
        vec![ // 22 
            Com(1,49),
            Com(1,82),
        ], 
        // AExp3
        vec![ // 23 
            Com(1,93),
            Ptr(24, false, false),
        ], 
        vec![ // 24 
            Com(2,117),
            Com(1,87),
        ], 
        // AExp4
        vec![ // 25 
            Com(1,144),
            Com(2,0),
        ], 
        // AExp5
        vec![ // 26 
            Com(2,14),
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
            Arg(0, true),
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 11 
            Arg(1, true),
            Arg(3, true),
        ], 
        // AExp7
        vec![ // 12 
            Arg(3, true),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 13 
            Com(4,10),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp8
        vec![ // 14 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 15 
            Com(4,12),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp9
        vec![ // 16 
            Com(1,0),
            Com(1,154),
            Ptr(0, true, true),
        ], 
        vec![ // 17 
            Com(1,206),
            Arg(0, true),
        ], 
        // AExp10
        vec![ // 18 
            Com(1,0),
            Ptr(25, false, false),
            Ptr(0, true, true),
        ], 
        vec![ // 19 
            Com(1,16),
            Arg(0, true),
        ], 
        // AExp11
        vec![ // 20 
            Com(1,0),
            Ptr(23, false, false),
            Ptr(0, true, true),
        ], 
        vec![ // 21 
            Com(1,18),
            Arg(0, true),
        ], 
        // AExp12
        vec![ // 22 
            Com(1,0),
            Ptr(22, false, false),
            Ptr(0, true, true),
        ], 
        vec![ // 23 
            Com(1,20),
            Arg(0, true),
        ], 
        // AExp13
        vec![ // 24 
            Com(1,0),
            Ptr(19, false, false),
            Ptr(0, true, true),
        ], 
        vec![ // 25 
            Com(1,22),
            Arg(0, true),
        ], 
        // AExp14
        vec![ // 26 
            Com(2,14),
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp15
        vec![ // 27 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 28 
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp16
        vec![ // 29 
            Com(2,39),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 30 
            Com(1,49),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp17
        vec![ // 31 
            Com(3,27),
            Com(1,51),
            Ptr(0, true, true),
        ], 
        vec![ // 32 
            Com(1,57),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp18
        vec![ // 33 
            Com(3,29),
            Ptr(0, true, true),
            Arg(1, false),
        ], 
        vec![ // 34 
            Com(2,31),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp19
        vec![ // 35 
            Com(4,41),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 36 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp20
        vec![ // 37 
            Arg(2, true),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 38 
            Com(3,35),
            Arg(1, true),
        ], 
        // AExp21
        vec![ // 39 
            Y,
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 40 
            Com(3,37),
            Arg(1, true),
        ], 
        // AExp22
        vec![ // 41 
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp23
        vec![ // 42 
            Com(4,41),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 43 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp24
        vec![ // 44 
            Arg(0, true),
            Arg(2, false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 45 
            Com(3,42),
            Arg(1, false),
            Arg(2, false),
            Arg(3, false),
        ], 
        vec![ // 46 
            Arg(1, false),
            Arg(3, false),
        ], 
        // AExp25
        vec![ // 47 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 48 
            Com(4,44),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp26
        vec![ // 49 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 50 
            Com(3,47),
            Arg(0, true),
        ], 
        // AExp27
        vec![ // 51 
            Arg(0, true),
            Com(2,1),
            Com(2,0),
        ], 
        // AExp28
        vec![ // 52 
            Com(2,59),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 53 
            Arg(1, true),
            Arg(4, true),
            Arg(2, false),
        ], 
        vec![ // 54 
            Arg(0, true),
            Arg(3, true),
            Arg(2, false),
        ], 
        // AExp29
        vec![ // 55 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 56 
            Com(5,52),
            Arg(0, true),
            Arg(1, true),
            Arg(3, true),
        ], 
        // AExp30
        vec![ // 57 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 58 
            Com(4,55),
            Arg(0, true),
        ], 
        // AExp31
        vec![ // 59 
            Arg(0, true),
            Arg(1, true),
            Com(2,1),
        ], 
        // AExp32
        vec![ // 60 
            Com(1,67),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 61 
            Com(1,77),
            Prm(EQ,false),
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 62 
            Com(1,77),
            Prm(EQ,false),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp33
        vec![ // 63 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 64 
            Com(4,60),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp34
        vec![ // 65 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 66 
            Com(3,63),
            Arg(1, true),
        ], 
        // AExp35
        vec![ // 67 
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp36
        vec![ // 68 
            Com(2,0),
        ], 
        // AExp37
        vec![ // 69 
            Com(1,67),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 70 
            Arg(1, true),
            Arg(3, true),
            Arg(5, true),
        ], 
        vec![ // 71 
            Arg(0, true),
            Arg(2, true),
            Arg(4, true),
        ], 
        // AExp38
        vec![ // 72 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 73 
            Com(6,69),
            Arg(0, true),
            Arg(1, true),
            Arg(3, true),
            Arg(4, true),
        ], 
        // AExp39
        vec![ // 74 
            Arg(2, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 75 
            Com(5,72),
            Arg(0, true),
            Arg(1, true),
            Arg(3, false),
        ], 
        vec![ // 76 
            Arg(3, false),
            Com(2,1),
            Com(2,68),
        ], 
        // AExp40
        vec![ // 77 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 78 
            Com(4,74),
            Arg(0, true),
        ], 
        // AExp41
        vec![ // 79 
            Com(4,41),
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp42
        vec![ // 80 
            Com(1,84),
            Ptr(0, true, true),
        ], 
        vec![ // 81 
            Com(2,85),
            Prm(EQ,false),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp43
        vec![ // 82 
            Arg(0, true),
            Com(2,80),
        ], 
        // AExp44
        vec![ // 83 
            Com(2,0),
        ], 
        // AExp45
        vec![ // 84 
            Arg(0, true),
            Com(2,1),
            Com(2,83),
        ], 
        // AExp46
        vec![ // 85 
            Com(1,49),
            Ptr(0, true, true),
        ], 
        vec![ // 86 
            Com(1,57),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp47
        vec![ // 87 
            Arg(0, true),
            Com(2,0),
            Com(2,0),
        ], 
        // AExp48
        vec![ // 88 
            Com(4,41),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 89 
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 90 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp49
        vec![ // 91 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 92 
            Com(4,88),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp50
        vec![ // 93 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 94 
            Com(3,91),
            Arg(0, true),
        ], 
        // AExp51
        vec![ // 95 
            Arg(0, true),
            Arg(1, true),
            Ptr(2, true, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 96 
            Arg(4, true),
            Arg(5, false),
            Arg(6, false),
        ], 
        vec![ // 97 
            Arg(3, true),
            Arg(5, false),
            Arg(6, false),
        ], 
        vec![ // 98 
            Arg(2, true),
            Arg(5, false),
            Arg(6, false),
        ], 
        // AExp52
        vec![ // 99 
            Com(2,117),
            Ptr(0, true, true),
        ], 
        vec![ // 100 
            Com(2,117),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp53
        vec![ // 101 
            Arg(2, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp54
        vec![ // 102 
            Com(2,99),
            Ptr(0, true, true),
        ], 
        vec![ // 103 
            Com(3,101),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp55
        vec![ // 104 
            Arg(5, true),
            Arg(0, false),
            Arg(0, false),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 105 
            Arg(2, true),
            Arg(3, true),
            Arg(4, true),
        ], 
        // AExp56
        vec![ // 106 
            Arg(3, true),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 107 
            Com(1,129),
            Arg(2, true),
            Arg(1, true),
        ], 
        // AExp57
        vec![ // 108 
            Com(6,104),
            Ptr(1, true, true),
            Ptr(0, true, true),
            Com(4,106),
        ], 
        vec![ // 109 
            Com(2,0),
            Arg(0, false),
        ], 
        vec![ // 110 
            Com(3,0),
            Arg(0, false),
        ], 
        // AExp58
        vec![ // 111 
            Arg(3, true),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 112 
            Com(1,129),
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp59
        vec![ // 113 
            Com(7,95),
            Arg(0, true),
            Ptr(1, true, true),
            Com(2,102),
            Ptr(0, true, true),
            Com(4,111),
        ], 
        vec![ // 114 
            Com(1,108),
            Arg(1, false),
        ], 
        vec![ // 115 
            Com(3,0),
            Arg(1, false),
        ], 
        // AExp60
        vec![ // 116 
            Arg(0, true),
            Com(2,0),
            Com(2,0),
        ], 
        // AExp61
        vec![ // 117 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 118 
            Com(2,113),
            Arg(1, true),
            Com(1,116),
        ], 
        // AExp62
        vec![ // 119 
            Com(4,41),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 120 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp63
        vec![ // 121 
            Com(4,41),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 122 
            Com(4,41),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp64
        vec![ // 123 
            Prm(LE,false),
            Arg(0, false),
            Arg(2, false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 124 
            Com(3,121),
            Arg(0, false),
            Arg(2, false),
            Arg(3, false),
        ], 
        vec![ // 125 
            Com(3,119),
            Arg(1, true),
            Arg(2, false),
            Arg(3, false),
        ], 
        // AExp65
        vec![ // 126 
            Arg(2, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 127 
            Com(4,123),
            Arg(0, false),
            Arg(1, true),
        ], 
        vec![ // 128 
            Com(4,41),
            Arg(0, false),
            Com(2,0),
        ], 
        // AExp66
        vec![ // 129 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 130 
            Com(3,126),
            Arg(0, true),
        ], 
        // AExp67
        vec![ // 131 
            Com(1,144),
            Ptr(0, true, true),
        ], 
        vec![ // 132 
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp68
        vec![ // 133 
            Com(4,41),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 134 
            Com(6,41),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp69
        vec![ // 135 
            Com(4,41),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 136 
            Com(5,146),
            Arg(1, true),
        ], 
        // AExp70
        vec![ // 137 
            Com(4,41),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 138 
            Com(5,147),
            Arg(1, true),
        ], 
        // AExp71
        vec![ // 139 
            Arg(2, true),
            Ptr(3, true, true),
            Ptr(2, true, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 140 
            Com(2,137),
            Arg(0, false),
        ], 
        vec![ // 141 
            Com(2,135),
            Arg(0, false),
        ], 
        vec![ // 142 
            Com(3,133),
            Arg(0, false),
        ], 
        vec![ // 143 
            Com(2,131),
            Arg(1, true),
        ], 
        // AExp72
        vec![ // 144 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 145 
            Com(3,139),
            Arg(0, true),
        ], 
        // AExp73
        vec![ // 146 
            Arg(3, true),
            Arg(0, true),
        ], 
        // AExp74
        vec![ // 147 
            Arg(4, true),
            Arg(0, true),
        ], 
        // AExp75
        vec![ // 148 
            Com(6,101),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 149 
            Com(1,154),
            Arg(1, true),
        ], 
        vec![ // 150 
            Com(1,154),
            Arg(0, true),
        ], 
        // AExp76
        vec![ // 151 
            Com(2,164),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 152 
            Com(1,154),
            Arg(1, true),
        ], 
        vec![ // 153 
            Com(1,154),
            Arg(0, true),
        ], 
        // AExp77
        vec![ // 154 
            Arg(0, true),
            Com(2,148),
            Com(2,151),
            Com(5,146),
            Com(5,147),
        ], 
        // AExp78
        vec![ // 155 
            Com(6,101),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 156 
            Com(2,164),
            Arg(2, true),
            Arg(0, false),
        ], 
        vec![ // 157 
            Com(2,164),
            Arg(1, true),
            Arg(0, false),
        ], 
        // AExp79
        vec![ // 158 
            Com(2,178),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 159 
            Com(6,41),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp80
        vec![ // 160 
            Com(2,178),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 161 
            Com(5,146),
            Arg(1, true),
        ], 
        // AExp81
        vec![ // 162 
            Com(2,178),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 163 
            Com(5,147),
            Arg(1, true),
        ], 
        // AExp82
        vec![ // 164 
            Arg(0, true),
            Ptr(3, true, true),
            Ptr(2, true, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 165 
            Com(2,162),
            Arg(1, false),
        ], 
        vec![ // 166 
            Com(2,160),
            Arg(1, false),
        ], 
        vec![ // 167 
            Com(3,158),
            Arg(1, false),
        ], 
        vec![ // 168 
            Com(3,155),
            Arg(1, false),
        ], 
        // AExp83
        vec![ // 169 
            Com(6,101),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 170 
            Com(2,164),
            Arg(0, false),
            Arg(2, true),
        ], 
        vec![ // 171 
            Com(2,164),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp84
        vec![ // 172 
            Com(6,41),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 173 
            Com(6,41),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp85
        vec![ // 174 
            Com(6,41),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 175 
            Com(5,146),
            Arg(1, true),
        ], 
        // AExp86
        vec![ // 176 
            Com(6,41),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 177 
            Com(5,147),
            Arg(1, true),
        ], 
        // AExp87
        vec![ // 178 
            Arg(1, true),
            Ptr(3, true, true),
            Ptr(2, true, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 179 
            Com(2,176),
            Arg(0, false),
        ], 
        vec![ // 180 
            Com(2,174),
            Arg(0, false),
        ], 
        vec![ // 181 
            Com(3,172),
            Arg(0, false),
        ], 
        vec![ // 182 
            Com(3,169),
            Arg(0, false),
        ], 
        // AExp88
        vec![ // 183 
            Com(6,101),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 184 
            Com(1,206),
            Arg(1, true),
        ], 
        vec![ // 185 
            Com(1,206),
            Arg(0, true),
        ], 
        // AExp89
        vec![ // 186 
            Com(6,41),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 187 
            Com(1,206),
            Arg(1, true),
        ], 
        vec![ // 188 
            Com(1,206),
            Arg(0, true),
        ], 
        // AExp90
        vec![ // 189 
            Com(1,206),
            Ptr(0, true, true),
        ], 
        vec![ // 190 
            Com(5,146),
            Arg(0, true),
        ], 
        // AExp91
        vec![ // 191 
            Com(1,206),
            Ptr(0, true, true),
        ], 
        vec![ // 192 
            Com(5,146),
            Arg(0, true),
        ], 
        // AExp92
        vec![ // 193 
            Com(6,41),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 194 
            Com(1,191),
            Arg(1, true),
        ], 
        vec![ // 195 
            Com(1,189),
            Arg(0, true),
        ], 
        // AExp93
        vec![ // 196 
            Com(1,206),
            Ptr(0, true, true),
        ], 
        vec![ // 197 
            Com(5,146),
            Arg(0, true),
        ], 
        // AExp94
        vec![ // 198 
            Com(1,206),
            Ptr(0, true, true),
        ], 
        vec![ // 199 
            Com(5,146),
            Arg(0, true),
        ], 
        // AExp95
        vec![ // 200 
            Com(6,101),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 201 
            Com(1,198),
            Arg(1, true),
        ], 
        vec![ // 202 
            Com(1,196),
            Arg(0, true),
        ], 
        // AExp96
        vec![ // 203 
            Com(5,146),
            Ptr(0, true, true),
        ], 
        vec![ // 204 
            Com(5,147),
            Arg(0, true),
        ], 
        // AExp97
        vec![ // 205 
            Arg(0, true),
            Com(2,193),
            Com(2,200),
            Com(1,206),
            Com(1,203),
        ], 
        // AExp98
        vec![ // 206 
            Arg(0, true),
            Com(2,183),
            Com(2,186),
            Com(1,205),
            Com(5,147),
        ], 
        // AExp99
        vec![ // 207 
            Com(2,211),
            Ptr(0, true, true),
        ], 
        vec![ // 208 
            Prm(Sub,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp100
        vec![ // 209 
            Com(4,41),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 210 
            Com(1,207),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp101
        vec![ // 211 
            Prm(LE,false),
            Arg(0, false),
            Int(0),
            Ptr(0, true, true),
            Com(2,0),
        ], 
        vec![ // 212 
            Com(2,209),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp102
        vec![ // 213 
            Com(6,41),
            Ptr(0, true, true),
        ], 
        vec![ // 214 
            Com(5,146),
            Arg(0, true),
        ], 
        // AExp103
        vec![ // 215 
            Com(6,41),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 216 
            Com(5,146),
            Arg(1, true),
        ], 
        // AExp104
        vec![ // 217 
            Com(6,101),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 218 
            Com(2,215),
            Arg(0, false),
            Arg(1, false),
        ], 
        vec![ // 219 
            Com(1,213),
            Arg(0, false),
            Arg(1, false),
        ], 
    ],

}});
