use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 154
#[rustfmt::skip]
pub static SKIABSEVAL: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            Com(1,7),
            Ptr(5, false, false),
        ], 
        vec![ // 1 
            Com(2,154),
            Int(5),
        ], 
        vec![ // 2 
            Com(2,154),
            Int(1),
        ], 
        vec![ // 3 
            Com(1,184),
            Ptr(6, false, false),
        ], 
        vec![ // 4 
            Com(3,21),
            Ptr(3, false, false),
            Ptr(2, false, false),
        ], 
        vec![ // 5 
            Com(3,21),
            Ptr(4, false, false),
            Ptr(1, false, false),
        ], 
        // AExp1
        vec![ // 6 
            Com(3,21),
            Com(1,173),
            Ptr(27, false, false),
        ], 
        vec![ // 7 
            Com(2,261),
            Int(2),
        ], 
        vec![ // 8 
            Com(2,154),
            Int(1),
        ], 
        vec![ // 9 
            Com(2,261),
            Int(1),
        ], 
        vec![ // 10 
            Com(3,21),
            Com(1,263),
            Ptr(9, false, false),
        ], 
        vec![ // 11 
            Com(3,21),
            Ptr(10, false, false),
            Ptr(8, false, false),
        ], 
        vec![ // 12 
            Com(2,261),
            Int(0),
        ], 
        vec![ // 13 
            Com(3,21),
            Ptr(12, false, false),
            Ptr(11, false, false),
        ], 
        vec![ // 14 
            Com(3,21),
            Ptr(13, false, false),
            Ptr(7, false, false),
        ], 
        vec![ // 15 
            Com(2,261),
            Int(1),
        ], 
        vec![ // 16 
            Com(3,21),
            Com(1,263),
            Ptr(15, false, false),
        ], 
        vec![ // 17 
            Com(3,21),
            Ptr(16, false, false),
            Ptr(14, false, false),
        ], 
        vec![ // 18 
            Com(2,261),
            Int(1),
        ], 
        vec![ // 19 
            Com(2,261),
            Int(2),
        ], 
        vec![ // 20 
            Com(2,261),
            Int(1),
        ], 
        vec![ // 21 
            Com(3,21),
            Com(1,260),
            Ptr(20, false, false),
        ], 
        vec![ // 22 
            Com(3,21),
            Ptr(21, false, false),
            Ptr(19, false, false),
        ], 
        vec![ // 23 
            Com(3,21),
            Ptr(22, false, false),
            Ptr(18, false, false),
        ], 
        vec![ // 24 
            Com(3,21),
            Ptr(23, false, false),
            Ptr(17, false, false),
        ], 
        vec![ // 25 
            Com(3,258),
            Int(2),
            Ptr(24, false, false),
        ], 
        vec![ // 26 
            Com(3,258),
            Int(1),
            Ptr(25, false, false),
        ], 
        vec![ // 27 
            Com(3,258),
            Int(0),
            Ptr(26, false, false),
        ], 
        // AExp2
        vec![ // 28 
            Com(3,9),
            Com(1,11),
            Ptr(29, false, false),
        ], 
        vec![ // 29 
            Com(3,9),
            Com(1,145),
            Com(1,172),
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
            Arg(0, true),
            Com(1,0),
        ], 
        // AExp3
        vec![ // 3 
            Com(1,7),
            Ptr(0, true, true),
        ], 
        vec![ // 4 
            Ptr(28, false, false),
            Arg(0, true),
        ], 
        // AExp4
        vec![ // 5 
            Prm(EQ,false),
            Arg(1, true),
            Int(10),
            Com(2,1),
            Com(2,2),
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 6 
            Com(1,3),
            Arg(0, true),
        ], 
        // AExp5
        vec![ // 7 
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 8 
            Com(3,5),
            Arg(0, false),
        ], 
        // AExp6
        vec![ // 9 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 10 
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp7
        vec![ // 11 
            Arg(0, true),
            Com(1,13),
            Ptr(0, true, true),
        ], 
        vec![ // 12 
            Com(1,18),
            Com(3,21),
        ], 
        // AExp8
        vec![ // 13 
            Arg(0, true),
            Int(5),
            Com(1,0),
        ], 
        // AExp9
        vec![ // 14 
            Arg(0, true),
            Ptr(0, true, true),
            Arg(2, true),
        ], 
        vec![ // 15 
            Arg(4, true),
            Arg(3, true),
            Arg(1, true),
        ], 
        // AExp10
        vec![ // 16 
            Arg(3, true),
            Com(2,0),
            Ptr(0, true, true),
            Arg(2, true),
            Arg(0, true),
        ], 
        vec![ // 17 
            Com(5,14),
            Arg(1, true),
        ], 
        // AExp11
        vec![ // 18 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 19 
            Com(4,16),
            Arg(0, true),
        ], 
        // AExp12
        vec![ // 20 
            Arg(2, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp13
        vec![ // 21 
            Arg(2, true),
            Int(2),
            Ptr(0, true, true),
        ], 
        vec![ // 22 
            Com(3,20),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp14
        vec![ // 23 
            Arg(0, true),
            Arg(2, true),
            Arg(1, true),
        ], 
        // AExp15
        vec![ // 24 
            Arg(0, true),
            Ptr(0, true, true),
            Arg(3, true),
        ], 
        vec![ // 25 
            Prm(LT,false),
            Arg(4, false),
            Int(11),
            Arg(1, true),
            Arg(2, true),
            Arg(4, false),
            Arg(5, true),
            Arg(6, true),
        ], 
        // AExp16
        vec![ // 26 
            Com(1,152),
            Ptr(0, true, true),
        ], 
        vec![ // 27 
            Ptr(28, false, false),
            Arg(0, true),
        ], 
        // AExp17
        vec![ // 28 
            Com(1,152),
            Ptr(0, true, true),
        ], 
        vec![ // 29 
            Ptr(28, false, false),
            Arg(0, true),
        ], 
        // AExp18
        vec![ // 30 
            Prm(EQ,false),
            Ptr(1, true, true),
            Ptr(0, true, true),
            Com(1,147),
            Com(1,148),
        ], 
        vec![ // 31 
            Com(1,28),
            Arg(0, true),
        ], 
        vec![ // 32 
            Com(1,26),
            Arg(1, true),
        ], 
        // AExp19
        vec![ // 33 
            Com(4,146),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 34 
            Com(2,30),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp20
        vec![ // 35 
            Com(1,145),
            Ptr(0, true, true),
        ], 
        vec![ // 36 
            Com(3,33),
            Arg(0, true),
            Arg(1, true),
            Arg(3, true),
        ], 
        // AExp21
        vec![ // 37 
            Arg(1, true),
            Com(2,0),
            Com(4,35),
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp22
        vec![ // 38 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 39 
            Arg(1, true),
            Com(1,0),
            Com(3,37),
            Arg(2, true),
        ], 
        // AExp23
        vec![ // 40 
            Com(1,152),
            Ptr(0, true, true),
        ], 
        vec![ // 41 
            Ptr(28, false, false),
            Arg(0, true),
        ], 
        // AExp24
        vec![ // 42 
            Com(1,152),
            Ptr(0, true, true),
        ], 
        vec![ // 43 
            Ptr(28, false, false),
            Arg(0, true),
        ], 
        // AExp25
        vec![ // 44 
            Prm(Add,false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 45 
            Com(1,42),
            Arg(0, true),
        ], 
        vec![ // 46 
            Com(1,40),
            Arg(1, true),
        ], 
        // AExp26
        vec![ // 47 
            Com(2,154),
            Ptr(0, true, true),
        ], 
        vec![ // 48 
            Com(2,44),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp27
        vec![ // 49 
            Com(4,146),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 50 
            Com(2,47),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp28
        vec![ // 51 
            Com(1,145),
            Ptr(0, true, true),
        ], 
        vec![ // 52 
            Com(3,49),
            Arg(0, true),
            Arg(1, true),
            Arg(3, true),
        ], 
        // AExp29
        vec![ // 53 
            Arg(1, true),
            Com(2,0),
            Com(4,51),
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp30
        vec![ // 54 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 55 
            Arg(1, true),
            Com(1,0),
            Com(3,53),
            Arg(2, true),
        ], 
        // AExp31
        vec![ // 56 
            Prm(LT,false),
            Arg(0, true),
            Int(12),
            Com(3,38),
            Com(3,54),
        ], 
        // AExp32
        vec![ // 57 
            Arg(2, true),
        ], 
        // AExp33
        vec![ // 58 
            Com(4,146),
            Ptr(0, true, true),
        ], 
        vec![ // 59 
            Com(3,21),
            Com(1,173),
            Arg(0, true),
        ], 
        // AExp34
        vec![ // 60 
            Com(2,159),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 61 
            Com(1,58),
            Arg(0, false),
            Arg(1, true),
        ], 
        vec![ // 62 
            Com(1,172),
            Arg(0, false),
        ], 
        // AExp35
        vec![ // 63 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 64 
            Arg(1, true),
            Com(1,0),
            Com(3,60),
            Arg(2, true),
        ], 
        // AExp36
        vec![ // 65 
            Prm(EQ,false),
            Arg(0, true),
            Int(9),
            Com(3,57),
            Com(3,63),
        ], 
        // AExp37
        vec![ // 66 
            Com(2,159),
            Ptr(0, true, true),
        ], 
        vec![ // 67 
            Com(1,172),
            Arg(0, true),
        ], 
        // AExp38
        vec![ // 68 
            Com(1,145),
            Ptr(0, true, true),
        ], 
        vec![ // 69 
            Com(1,66),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp39
        vec![ // 70 
            Arg(1, true),
            Com(1,0),
            Com(3,68),
        ], 
        // AExp40
        vec![ // 71 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 72 
            Arg(1, true),
            Com(1,0),
            Com(2,70),
            Arg(2, true),
        ], 
        // AExp41
        vec![ // 73 
            Com(4,146),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 74 
            Ptr(28, false, false),
            Arg(1, true),
        ], 
        // AExp42
        vec![ // 75 
            Com(4,146),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 76 
            Com(2,73),
            Arg(1, true),
            Arg(2, true),
        ], 
        vec![ // 77 
            Ptr(28, false, false),
            Arg(0, true),
        ], 
        // AExp43
        vec![ // 78 
            Com(2,159),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 79 
            Com(3,75),
            Arg(0, true),
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 80 
            Com(1,172),
            Arg(2, true),
        ], 
        // AExp44
        vec![ // 81 
            Com(1,145),
            Ptr(0, true, true),
        ], 
        vec![ // 82 
            Com(4,78),
            Arg(0, true),
            Arg(1, true),
            Arg(3, true),
            Arg(4, true),
        ], 
        // AExp45
        vec![ // 83 
            Arg(1, true),
            Com(3,0),
            Com(5,81),
            Arg(2, true),
            Arg(3, true),
            Arg(0, true),
        ], 
        // AExp46
        vec![ // 84 
            Arg(1, true),
            Com(2,0),
            Com(4,83),
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp47
        vec![ // 85 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 86 
            Arg(1, true),
            Com(1,0),
            Com(3,84),
            Arg(2, true),
        ], 
        // AExp48
        vec![ // 87 
            Prm(LT,false),
            Arg(0, true),
            Int(8),
            Com(3,71),
            Com(3,85),
        ], 
        // AExp49
        vec![ // 88 
            Prm(LT,false),
            Arg(0, false),
            Int(9),
            Ptr(0, true, true),
            Com(1,87),
            Arg(0, false),
        ], 
        vec![ // 89 
            Com(7,24),
            Com(3,9),
            Com(1,56),
            Com(1,65),
            Com(1,0),
        ], 
        // AExp50
        vec![ // 90 
            Com(3,21),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 91 
            Ptr(28, false, false),
            Arg(0, true),
        ], 
        vec![ // 92 
            Ptr(28, false, false),
            Arg(1, true),
        ], 
        // AExp51
        vec![ // 93 
            Com(4,146),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 94 
            Com(2,90),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp52
        vec![ // 95 
            Com(2,159),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 96 
            Com(3,93),
            Arg(0, true),
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 97 
            Com(1,172),
            Arg(2, true),
        ], 
        // AExp53
        vec![ // 98 
            Com(1,145),
            Ptr(0, true, true),
        ], 
        vec![ // 99 
            Com(4,95),
            Arg(0, true),
            Arg(1, true),
            Arg(3, true),
            Arg(4, true),
        ], 
        // AExp54
        vec![ // 100 
            Arg(1, true),
            Com(3,0),
            Com(5,98),
            Arg(2, true),
            Arg(3, true),
            Arg(0, true),
        ], 
        // AExp55
        vec![ // 101 
            Arg(1, true),
            Com(2,0),
            Com(4,100),
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp56
        vec![ // 102 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 103 
            Arg(1, true),
            Com(1,0),
            Com(3,101),
            Arg(2, true),
        ], 
        // AExp57
        vec![ // 104 
            Com(2,159),
            Ptr(0, true, true),
        ], 
        vec![ // 105 
            Com(1,172),
            Arg(0, true),
        ], 
        // AExp58
        vec![ // 106 
            Com(1,145),
            Ptr(0, true, true),
        ], 
        vec![ // 107 
            Com(1,104),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp59
        vec![ // 108 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 109 
            Arg(1, true),
            Com(1,0),
            Com(3,106),
            Arg(2, true),
        ], 
        // AExp60
        vec![ // 110 
            Prm(LT,false),
            Arg(0, true),
            Int(6),
            Com(3,102),
            Com(3,108),
        ], 
        // AExp61
        vec![ // 111 
            Arg(0, true),
            Ptr(0, true, true),
            Arg(3, true),
        ], 
        vec![ // 112 
            Prm(LT,false),
            Arg(4, false),
            Int(4),
            Arg(1, true),
            Arg(2, true),
            Arg(4, false),
            Arg(5, true),
            Arg(6, true),
        ], 
        // AExp62
        vec![ // 113 
            Com(2,159),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 114 
            Com(1,172),
            Arg(1, true),
        ], 
        // AExp63
        vec![ // 115 
            Com(1,145),
            Ptr(0, true, true),
        ], 
        vec![ // 116 
            Com(2,113),
            Arg(1, true),
            Arg(3, true),
        ], 
        // AExp64
        vec![ // 117 
            Arg(1, true),
            Com(2,0),
            Com(4,115),
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp65
        vec![ // 118 
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 119 
            Arg(2, true),
            Com(1,0),
            Com(3,117),
            Arg(3, true),
        ], 
        // AExp66
        vec![ // 120 
            Com(3,21),
            Ptr(0, true, true),
        ], 
        vec![ // 121 
            Ptr(28, false, false),
            Arg(0, true),
        ], 
        // AExp67
        vec![ // 122 
            Com(4,146),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 123 
            Com(1,120),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp68
        vec![ // 124 
            Com(4,146),
            Arg(2, false),
            Ptr(0, true, true),
        ], 
        vec![ // 125 
            Com(3,122),
            Arg(0, true),
            Arg(1, true),
            Arg(2, false),
        ], 
        // AExp69
        vec![ // 126 
            Com(2,159),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 127 
            Com(3,124),
            Arg(0, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        vec![ // 128 
            Com(1,172),
            Arg(1, true),
        ], 
        // AExp70
        vec![ // 129 
            Com(1,145),
            Ptr(0, true, true),
        ], 
        vec![ // 130 
            Com(4,126),
            Arg(1, true),
            Arg(3, true),
            Arg(4, true),
            Arg(0, true),
        ], 
        // AExp71
        vec![ // 131 
            Com(5,129),
            Ptr(0, true, true),
        ], 
        vec![ // 132 
            Ptr(28, false, false),
            Arg(0, true),
        ], 
        // AExp72
        vec![ // 133 
            Arg(1, true),
            Com(3,0),
            Com(1,131),
            Arg(2, true),
            Arg(3, true),
            Arg(0, true),
        ], 
        // AExp73
        vec![ // 134 
            Arg(1, true),
            Com(2,0),
            Com(4,133),
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp74
        vec![ // 135 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 136 
            Arg(1, true),
            Com(1,0),
            Com(3,134),
            Arg(2, true),
        ], 
        // AExp75
        vec![ // 137 
            Prm(EQ,false),
            Arg(0, true),
            Int(3),
            Com(3,57),
            Com(3,135),
        ], 
        // AExp76
        vec![ // 138 
            Prm(LT,false),
            Arg(0, false),
            Int(5),
            Com(1,110),
            Ptr(0, true, true),
            Arg(0, false),
        ], 
        vec![ // 139 
            Com(7,111),
            Com(3,9),
            Com(4,118),
            Com(1,137),
            Com(1,0),
        ], 
        // AExp77
        vec![ // 140 
            Prm(LT,false),
            Arg(1, false),
            Int(7),
            Com(1,88),
            Com(1,138),
            Arg(1, false),
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp78
        vec![ // 141 
            Com(3,23),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 142 
            Com(3,140),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp79
        vec![ // 143 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 144 
            Com(3,141),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp80
        vec![ // 145 
            Arg(0, false),
            Com(1,0),
            Com(3,143),
            Arg(0, false),
        ], 
        // AExp81
        vec![ // 146 
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp82
        vec![ // 147 
            Arg(0, true),
            Int(8),
            Com(1,0),
        ], 
        // AExp83
        vec![ // 148 
            Arg(0, true),
            Int(4),
            Com(1,0),
        ], 
        // AExp84
        vec![ // 149 
            Int(0),
        ], 
        // AExp85
        vec![ // 150 
            Arg(0, true),
            Com(1,0),
        ], 
        // AExp86
        vec![ // 151 
            Prm(EQ,false),
            Arg(0, true),
            Int(10),
            Com(1,149),
            Com(1,150),
        ], 
        // AExp87
        vec![ // 152 
            Arg(0, true),
            Com(1,151),
        ], 
        // AExp88
        vec![ // 153 
            Arg(1, true),
            Arg(0, true),
        ], 
        // AExp89
        vec![ // 154 
            Arg(1, true),
            Int(10),
            Ptr(0, true, true),
        ], 
        vec![ // 155 
            Com(2,153),
            Arg(0, true),
        ], 
        // AExp90
        vec![ // 156 
            Com(4,146),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 157 
            Arg(3, true),
            Arg(1, true),
        ], 
        // AExp91
        vec![ // 158 
            Arg(2, true),
            Com(2,0),
            Com(4,156),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp92
        vec![ // 159 
            Y,
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 160 
            Com(3,158),
            Arg(1, true),
        ], 
        // AExp93
        vec![ // 161 
            Arg(3, true),
        ], 
        // AExp94
        vec![ // 162 
            Arg(0, true),
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 163 
            Com(4,146),
            Arg(3, true),
            Arg(1, true),
        ], 
        // AExp95
        vec![ // 164 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 165 
            Com(4,162),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp96
        vec![ // 166 
            Prm(EQ,false),
            Arg(2, true),
            Int(2),
            Com(4,161),
            Com(4,164),
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp97
        vec![ // 167 
            Com(3,23),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 168 
            Com(4,146),
            Arg(1, true),
            Arg(2, false),
        ], 
        vec![ // 169 
            Com(4,166),
            Arg(0, true),
            Arg(2, false),
            Arg(3, true),
        ], 
        // AExp98
        vec![ // 170 
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 171 
            Com(4,167),
            Arg(0, true),
            Arg(1, false),
            Arg(2, true),
        ], 
        // AExp99
        vec![ // 172 
            Y,
            Com(3,170),
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp100
        vec![ // 173 
            Arg(0, true),
            Int(9),
            Com(1,0),
        ], 
        // AExp101
        vec![ // 174 
            Com(3,21),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 175 
            Com(1,184),
            Arg(1, true),
        ], 
        vec![ // 176 
            Com(1,184),
            Arg(0, true),
        ], 
        // AExp102
        vec![ // 177 
            Arg(0, true),
            Com(2,174),
        ], 
        // AExp103
        vec![ // 178 
            Prm(EQ,false),
            Arg(0, true),
            Int(2),
            Com(2,1),
            Com(2,177),
        ], 
        // AExp104
        vec![ // 179 
            Com(2,201),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 180 
            Com(1,184),
            Arg(1, true),
        ], 
        // AExp105
        vec![ // 181 
            Arg(0, true),
            Com(2,179),
        ], 
        // AExp106
        vec![ // 182 
            Prm(EQ,false),
            Arg(0, true),
            Int(1),
            Com(2,1),
            Com(2,181),
        ], 
        // AExp107
        vec![ // 183 
            Prm(LT,false),
            Arg(1, false),
            Int(2),
            Com(1,178),
            Com(1,182),
            Arg(1, false),
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp108
        vec![ // 184 
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 185 
            Com(3,183),
            Arg(0, false),
        ], 
        // AExp109
        vec![ // 186 
            Com(0,254),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 187 
            Com(2,201),
            Arg(0, false),
            Arg(2, true),
        ], 
        vec![ // 188 
            Com(2,201),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp110
        vec![ // 189 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 190 
            Com(3,186),
            Arg(1, true),
        ], 
        // AExp111
        vec![ // 191 
            Prm(EQ,false),
            Arg(0, true),
            Int(2),
            Com(3,57),
            Com(3,189),
        ], 
        // AExp112
        vec![ // 192 
            Com(1,13),
        ], 
        // AExp113
        vec![ // 193 
            Prm(EQ,false),
            Arg(0, true),
            Arg(2, true),
            Com(1,0),
            Com(1,192),
            Arg(1, true),
        ], 
        // AExp114
        vec![ // 194 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 195 
            Com(3,193),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp115
        vec![ // 196 
            Prm(EQ,false),
            Arg(0, true),
            Int(0),
            Com(3,57),
            Com(3,194),
        ], 
        // AExp116
        vec![ // 197 
            Prm(LT,false),
            Arg(1, false),
            Int(2),
            Com(1,191),
            Com(1,196),
            Arg(1, false),
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp117
        vec![ // 198 
            Com(3,23),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 199 
            Com(3,21),
            Com(1,148),
            Arg(1, true),
        ], 
        vec![ // 200 
            Com(3,197),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp118
        vec![ // 201 
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 202 
            Com(3,198),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp119
        vec![ // 203 
            Arg(0, true),
            Arg(4, true),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        // AExp120
        vec![ // 204 
            Com(5,203),
            Ptr(0, true, true),
            Arg(1, true),
            Arg(3, true),
            Arg(2, true),
        ], 
        vec![ // 205 
            Arg(0, true),
            Arg(4, true),
        ], 
        // AExp121
        vec![ // 206 
            Prm(EQ,false),
            Arg(5, true),
            Int(2),
            Arg(0, true),
            Arg(1, true),
            Arg(6, true),
            Arg(4, false),
            Ptr(0, true, true),
        ], 
        vec![ // 207 
            Arg(2, true),
            Arg(3, true),
            Arg(4, false),
        ], 
        // AExp122
        vec![ // 208 
            Arg(0, true),
            Arg(4, true),
            Arg(5, false),
            Ptr(0, true, true),
        ], 
        vec![ // 209 
            Com(7,206),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
            Arg(5, false),
            Arg(6, true),
        ], 
        // AExp123
        vec![ // 210 
            Com(3,21),
            Com(1,148),
            Ptr(0, true, true),
        ], 
        vec![ // 211 
            Com(3,21),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp124
        vec![ // 212 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 213 
            Com(2,210),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp125
        vec![ // 214 
            Prm(EQ,false),
            Arg(2, true),
            Int(4),
            Com(4,161),
            Com(4,212),
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp126
        vec![ // 215 
            Com(3,23),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 216 
            Com(4,214),
            Arg(0, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        // AExp127
        vec![ // 217 
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 218 
            Com(4,215),
            Arg(0, true),
            Arg(1, true),
            Arg(3, true),
        ], 
        // AExp128
        vec![ // 219 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 220 
            Com(4,217),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp129
        vec![ // 221 
            Com(3,21),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 222 
            Com(3,21),
            Com(1,255),
            Arg(1, true),
        ], 
        // AExp130
        vec![ // 223 
            Prm(EQ,false),
            Arg(0, true),
            Int(4),
            Com(4,161),
            Ptr(0, true, true),
        ], 
        vec![ // 224 
            Com(7,208),
            Com(4,9),
            Com(3,57),
            Com(3,219),
            Com(2,221),
        ], 
        // AExp131
        vec![ // 225 
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 226 
            Com(5,204),
            Com(1,223),
            Arg(0, true),
            Arg(1, true),
            Arg(3, true),
        ], 
        // AExp132
        vec![ // 227 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 228 
            Com(4,225),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp133
        vec![ // 229 
            Prm(EQ,false),
            Arg(2, true),
            Int(2),
            Com(3,57),
            Com(3,227),
            Arg(3, true),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 230 
            Arg(1, false),
            Arg(0, true),
        ], 
        // AExp134
        vec![ // 231 
            Com(4,229),
            Ptr(0, true, true),
            Arg(1, false),
        ], 
        vec![ // 232 
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp135
        vec![ // 233 
            Prm(EQ,false),
            Arg(5, true),
            Int(2),
            Arg(0, true),
            Arg(1, true),
            Arg(6, true),
            Arg(3, true),
            Ptr(0, true, true),
        ], 
        vec![ // 234 
            Arg(2, true),
            Arg(4, true),
        ], 
        // AExp136
        vec![ // 235 
            Com(3,21),
            Ptr(0, true, true),
        ], 
        vec![ // 236 
            Com(3,21),
            Com(1,256),
            Arg(0, true),
        ], 
        // AExp137
        vec![ // 237 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 238 
            Com(1,235),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp138
        vec![ // 239 
            Prm(EQ,false),
            Arg(2, true),
            Int(4),
            Com(4,161),
            Com(4,237),
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp139
        vec![ // 240 
            Com(3,23),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 241 
            Com(4,239),
            Arg(0, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        // AExp140
        vec![ // 242 
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 243 
            Com(4,240),
            Arg(0, true),
            Arg(1, true),
            Arg(3, true),
        ], 
        // AExp141
        vec![ // 244 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 245 
            Com(4,242),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp142
        vec![ // 246 
            Com(3,21),
            Ptr(0, true, true),
        ], 
        vec![ // 247 
            Com(3,21),
            Com(1,257),
            Arg(0, true),
        ], 
        // AExp143
        vec![ // 248 
            Com(7,233),
            Com(3,57),
            Com(3,244),
            Ptr(0, true, true),
            Arg(0, false),
        ], 
        vec![ // 249 
            Com(1,246),
            Arg(0, false),
        ], 
        // AExp144
        vec![ // 250 
            Com(2,231),
            Ptr(0, true, true),
        ], 
        vec![ // 251 
            Com(1,248),
            Arg(0, true),
        ], 
        // AExp145
        vec![ // 252 
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 253 
            Com(1,250),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp146
        vec![ // 254 
            Com(2,252),
        ], 
        // AExp147
        vec![ // 255 
            Arg(0, true),
            Int(6),
            Com(1,0),
        ], 
        // AExp148
        vec![ // 256 
            Arg(0, true),
            Int(7),
            Com(1,0),
        ], 
        // AExp149
        vec![ // 257 
            Arg(0, true),
            Int(3),
            Com(1,0),
        ], 
        // AExp150
        vec![ // 258 
            Arg(2, true),
            Int(1),
            Ptr(0, true, true),
        ], 
        vec![ // 259 
            Com(3,20),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp151
        vec![ // 260 
            Arg(0, true),
            Int(12),
            Com(1,0),
        ], 
        // AExp152
        vec![ // 261 
            Arg(1, true),
            Int(0),
            Ptr(0, true, true),
        ], 
        vec![ // 262 
            Com(2,153),
            Arg(0, true),
        ], 
        // AExp153
        vec![ // 263 
            Arg(0, true),
            Int(11),
            Com(1,0),
        ], 
    ],

}});