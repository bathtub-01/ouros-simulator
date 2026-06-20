use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 143
#[rustfmt::skip]
pub static SKIABSEVAL: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            Com(1,7),
            Ptr(5, false, false),
        ], 
        vec![ // 1 
            Com(2,175),
            Int(5),
        ], 
        vec![ // 2 
            Com(2,175),
            Int(1),
        ], 
        vec![ // 3 
            Com(1,206),
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
            Com(1,193),
            Ptr(27, false, false),
        ], 
        vec![ // 7 
            Com(2,286),
            Int(2),
        ], 
        vec![ // 8 
            Com(2,175),
            Int(1),
        ], 
        vec![ // 9 
            Com(2,286),
            Int(1),
        ], 
        vec![ // 10 
            Com(3,21),
            Com(1,288),
            Ptr(9, false, false),
        ], 
        vec![ // 11 
            Com(3,21),
            Ptr(10, false, false),
            Ptr(8, false, false),
        ], 
        vec![ // 12 
            Com(2,286),
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
            Com(2,286),
            Int(1),
        ], 
        vec![ // 16 
            Com(3,21),
            Com(1,288),
            Ptr(15, false, false),
        ], 
        vec![ // 17 
            Com(3,21),
            Ptr(16, false, false),
            Ptr(14, false, false),
        ], 
        vec![ // 18 
            Com(2,286),
            Int(1),
        ], 
        vec![ // 19 
            Com(2,286),
            Int(2),
        ], 
        vec![ // 20 
            Com(2,286),
            Int(1),
        ], 
        vec![ // 21 
            Com(3,21),
            Com(1,285),
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
            Com(3,283),
            Int(2),
            Ptr(24, false, false),
        ], 
        vec![ // 26 
            Com(3,283),
            Int(1),
            Ptr(25, false, false),
        ], 
        vec![ // 27 
            Com(3,283),
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
            Com(1,166),
            Com(1,192),
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
            Com(1,7),
            Ptr(0, true, true),
        ], 
        vec![ // 3 
            Ptr(28, false, false),
            Arg(0, true),
        ], 
        // AExp3
        vec![ // 4 
            Prm(EQ,false),
            Arg(1, true),
            Int(10),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 5 
            Arg(2, true),
            Com(1,0),
        ], 
        vec![ // 6 
            Com(1,2),
            Arg(0, true),
        ], 
        // AExp4
        vec![ // 7 
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 8 
            Com(3,4),
            Arg(0, false),
        ], 
        // AExp5
        vec![ // 9 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 10 
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp6
        vec![ // 11 
            Arg(0, true),
            Com(1,13),
            Ptr(0, true, true),
        ], 
        vec![ // 12 
            Com(1,18),
            Com(3,21),
        ], 
        // AExp7
        vec![ // 13 
            Arg(0, true),
            Int(5),
            Com(1,0),
        ], 
        // AExp8
        vec![ // 14 
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 15 
            Arg(0, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        // AExp9
        vec![ // 16 
            Arg(3, true),
            Arg(2, false),
            Ptr(0, true, true),
        ], 
        vec![ // 17 
            Com(4,14),
            Arg(0, true),
            Arg(1, true),
            Arg(2, false),
        ], 
        // AExp10
        vec![ // 18 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 19 
            Com(4,16),
            Arg(0, true),
        ], 
        // AExp11
        vec![ // 20 
            Arg(2, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp12
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
        // AExp13
        vec![ // 23 
            Arg(3, true),
            Ptr(0, true, true),
        ], 
        vec![ // 24 
            Arg(2, true),
            Arg(1, true),
            Arg(0, true),
        ], 
        // AExp14
        vec![ // 25 
            Com(4,168),
            Ptr(0, true, true),
        ], 
        vec![ // 26 
            Prm(EQ,false),
            Arg(0, true),
            Arg(1, true),
            Com(1,169),
            Com(1,170),
        ], 
        // AExp15
        vec![ // 27 
            Com(1,173),
            Ptr(0, true, true),
        ], 
        vec![ // 28 
            Ptr(28, false, false),
            Arg(0, true),
        ], 
        // AExp16
        vec![ // 29 
            Com(1,173),
            Ptr(0, true, true),
        ], 
        vec![ // 30 
            Ptr(28, false, false),
            Arg(0, true),
        ], 
        // AExp17
        vec![ // 31 
            Com(2,25),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 32 
            Com(1,29),
            Arg(1, true),
        ], 
        vec![ // 33 
            Com(1,27),
            Arg(0, true),
        ], 
        // AExp18
        vec![ // 34 
            Com(1,166),
            Ptr(0, true, true),
        ], 
        vec![ // 35 
            Com(2,31),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp19
        vec![ // 36 
            Arg(2, true),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 37 
            Com(3,34),
            Arg(1, true),
        ], 
        // AExp20
        vec![ // 38 
            Com(4,23),
            Ptr(0, true, true),
            Arg(0, false),
        ], 
        vec![ // 39 
            Com(3,36),
            Arg(0, false),
        ], 
        // AExp21
        vec![ // 40 
            Com(1,166),
            Ptr(0, true, true),
        ], 
        vec![ // 41 
            Com(4,168),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp22
        vec![ // 42 
            Com(2,175),
            Ptr(0, true, true),
        ], 
        vec![ // 43 
            Prm(Add,false),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp23
        vec![ // 44 
            Com(1,173),
            Ptr(0, true, true),
        ], 
        vec![ // 45 
            Ptr(28, false, false),
            Arg(0, true),
        ], 
        // AExp24
        vec![ // 46 
            Com(1,173),
            Ptr(0, true, true),
        ], 
        vec![ // 47 
            Ptr(28, false, false),
            Arg(0, true),
        ], 
        // AExp25
        vec![ // 48 
            Com(2,42),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 49 
            Com(1,46),
            Arg(1, true),
        ], 
        vec![ // 50 
            Com(1,44),
            Arg(0, true),
        ], 
        // AExp26
        vec![ // 51 
            Com(2,40),
            Ptr(0, true, true),
        ], 
        vec![ // 52 
            Com(2,48),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp27
        vec![ // 53 
            Arg(2, true),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 54 
            Com(2,51),
            Arg(1, true),
        ], 
        // AExp28
        vec![ // 55 
            Com(4,23),
            Ptr(0, true, true),
            Arg(0, false),
        ], 
        vec![ // 56 
            Com(3,53),
            Arg(0, false),
        ], 
        // AExp29
        vec![ // 57 
            Prm(LT,false),
            Arg(2, true),
            Int(12),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 58 
            Com(1,55),
            Arg(0, false),
            Arg(1, false),
            Arg(3, false),
        ], 
        vec![ // 59 
            Com(1,38),
            Arg(0, false),
            Arg(1, false),
            Arg(3, false),
        ], 
        // AExp30
        vec![ // 60 
            Com(4,168),
            Ptr(0, true, true),
        ], 
        vec![ // 61 
            Com(3,21),
            Com(1,193),
            Arg(0, true),
        ], 
        // AExp31
        vec![ // 62 
            Com(2,181),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 63 
            Com(1,60),
            Arg(0, false),
            Arg(1, true),
        ], 
        vec![ // 64 
            Com(1,192),
            Arg(0, false),
        ], 
        // AExp32
        vec![ // 65 
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 66 
            Arg(1, true),
            Arg(0, true),
            Com(2,62),
        ], 
        // AExp33
        vec![ // 67 
            Prm(EQ,false),
            Arg(2, true),
            Int(9),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 68 
            Com(3,65),
            Arg(0, false),
            Arg(1, true),
            Arg(3, true),
        ], 
        // AExp34
        vec![ // 69 
            Prm(LT,false),
            Arg(2, false),
            Int(11),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 70 
            Com(4,67),
            Arg(0, false),
            Arg(1, false),
            Arg(2, false),
            Arg(3, false),
        ], 
        vec![ // 71 
            Com(4,57),
            Arg(0, false),
            Arg(1, false),
            Arg(2, false),
            Arg(3, false),
        ], 
        // AExp35
        vec![ // 72 
            Com(2,181),
            Ptr(0, true, true),
        ], 
        vec![ // 73 
            Com(1,192),
            Arg(0, true),
        ], 
        // AExp36
        vec![ // 74 
            Com(1,166),
            Ptr(0, true, true),
        ], 
        vec![ // 75 
            Com(1,72),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp37
        vec![ // 76 
            Arg(2, true),
            Arg(0, true),
            Com(2,74),
        ], 
        // AExp38
        vec![ // 77 
            Com(4,23),
            Ptr(0, true, true),
            Arg(0, false),
        ], 
        vec![ // 78 
            Com(3,76),
            Arg(0, false),
        ], 
        // AExp39
        vec![ // 79 
            Com(4,168),
            Ptr(0, true, true),
        ], 
        vec![ // 80 
            Ptr(28, false, false),
            Arg(0, true),
        ], 
        // AExp40
        vec![ // 81 
            Com(4,168),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 82 
            Com(1,79),
            Arg(0, true),
            Arg(2, true),
        ], 
        vec![ // 83 
            Ptr(28, false, false),
            Arg(1, true),
        ], 
        // AExp41
        vec![ // 84 
            Com(2,181),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 85 
            Com(3,81),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        vec![ // 86 
            Com(1,192),
            Arg(0, true),
        ], 
        // AExp42
        vec![ // 87 
            Com(1,166),
            Ptr(0, true, true),
        ], 
        vec![ // 88 
            Com(4,84),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        // AExp43
        vec![ // 89 
            Arg(3, true),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 90 
            Com(4,87),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp44
        vec![ // 91 
            Arg(2, true),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 92 
            Com(4,89),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp45
        vec![ // 93 
            Com(4,23),
            Ptr(0, true, true),
            Arg(0, false),
        ], 
        vec![ // 94 
            Com(3,91),
            Arg(0, false),
        ], 
        // AExp46
        vec![ // 95 
            Prm(LT,false),
            Arg(2, true),
            Int(8),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 96 
            Com(1,93),
            Arg(0, false),
            Arg(1, false),
            Arg(3, false),
        ], 
        vec![ // 97 
            Com(1,77),
            Arg(0, false),
            Arg(1, false),
            Arg(3, false),
        ], 
        // AExp47
        vec![ // 98 
            Prm(LT,false),
            Arg(2, false),
            Int(9),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 99 
            Com(4,95),
            Arg(0, false),
            Arg(1, false),
            Arg(2, false),
            Arg(3, false),
        ], 
        vec![ // 100 
            Com(4,69),
            Arg(0, false),
            Arg(1, false),
            Arg(2, false),
            Arg(3, false),
        ], 
        // AExp48
        vec![ // 101 
            Com(2,181),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 102 
            Com(4,168),
            Arg(1, true),
            Arg(2, true),
        ], 
        vec![ // 103 
            Com(1,192),
            Arg(0, true),
        ], 
        // AExp49
        vec![ // 104 
            Com(3,21),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 105 
            Ptr(28, false, false),
            Arg(1, true),
        ], 
        vec![ // 106 
            Ptr(28, false, false),
            Arg(0, true),
        ], 
        // AExp50
        vec![ // 107 
            Com(3,101),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 108 
            Com(2,104),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp51
        vec![ // 109 
            Com(1,166),
            Ptr(0, true, true),
        ], 
        vec![ // 110 
            Com(3,107),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        // AExp52
        vec![ // 111 
            Arg(3, true),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 112 
            Com(4,109),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp53
        vec![ // 113 
            Arg(2, true),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 114 
            Com(4,111),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp54
        vec![ // 115 
            Com(4,23),
            Ptr(0, true, true),
            Arg(0, false),
        ], 
        vec![ // 116 
            Com(3,113),
            Arg(0, false),
        ], 
        // AExp55
        vec![ // 117 
            Com(2,181),
            Ptr(0, true, true),
        ], 
        vec![ // 118 
            Com(1,192),
            Arg(0, true),
        ], 
        // AExp56
        vec![ // 119 
            Com(1,166),
            Ptr(0, true, true),
        ], 
        vec![ // 120 
            Com(1,117),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp57
        vec![ // 121 
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 122 
            Arg(1, true),
            Arg(0, true),
            Com(2,119),
        ], 
        // AExp58
        vec![ // 123 
            Prm(LT,false),
            Arg(2, true),
            Int(6),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 124 
            Com(3,121),
            Arg(0, false),
            Arg(1, false),
            Arg(3, false),
        ], 
        vec![ // 125 
            Com(1,115),
            Arg(0, false),
            Arg(1, false),
            Arg(3, false),
        ], 
        // AExp59
        vec![ // 126 
            Com(2,181),
            Ptr(0, true, true),
        ], 
        vec![ // 127 
            Com(1,192),
            Arg(0, true),
        ], 
        // AExp60
        vec![ // 128 
            Com(1,166),
            Ptr(0, true, true),
        ], 
        vec![ // 129 
            Com(1,126),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp61
        vec![ // 130 
            Arg(2, true),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 131 
            Com(3,128),
            Arg(1, true),
        ], 
        // AExp62
        vec![ // 132 
            Com(4,23),
            Ptr(0, true, true),
            Arg(0, false),
        ], 
        vec![ // 133 
            Com(3,130),
            Arg(0, false),
        ], 
        // AExp63
        vec![ // 134 
            Com(3,21),
            Ptr(0, true, true),
        ], 
        vec![ // 135 
            Ptr(28, false, false),
            Arg(0, true),
        ], 
        // AExp64
        vec![ // 136 
            Com(4,168),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 137 
            Com(1,134),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp65
        vec![ // 138 
            Com(4,168),
            Arg(2, false),
            Ptr(0, true, true),
        ], 
        vec![ // 139 
            Com(3,136),
            Arg(0, true),
            Arg(1, true),
            Arg(2, false),
        ], 
        // AExp66
        vec![ // 140 
            Com(2,181),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 141 
            Com(3,138),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        vec![ // 142 
            Com(1,192),
            Arg(0, true),
        ], 
        // AExp67
        vec![ // 143 
            Com(1,166),
            Ptr(0, true, true),
        ], 
        vec![ // 144 
            Com(4,140),
            Arg(0, true),
            Arg(1, true),
            Arg(3, true),
            Arg(2, true),
        ], 
        // AExp68
        vec![ // 145 
            Com(4,143),
            Arg(0, true),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 146 
            Ptr(28, false, false),
            Arg(2, true),
        ], 
        // AExp69
        vec![ // 147 
            Arg(3, true),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 148 
            Com(3,145),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp70
        vec![ // 149 
            Arg(2, true),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 150 
            Com(4,147),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp71
        vec![ // 151 
            Com(4,23),
            Ptr(0, true, true),
            Arg(0, false),
        ], 
        vec![ // 152 
            Com(3,149),
            Arg(0, false),
        ], 
        // AExp72
        vec![ // 153 
            Prm(EQ,false),
            Arg(2, true),
            Int(3),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 154 
            Com(1,151),
            Arg(0, false),
            Arg(1, true),
            Arg(3, true),
        ], 
        // AExp73
        vec![ // 155 
            Prm(LT,false),
            Arg(2, false),
            Int(4),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 156 
            Com(4,153),
            Arg(0, false),
            Arg(1, false),
            Arg(2, false),
            Arg(3, false),
        ], 
        vec![ // 157 
            Com(1,132),
            Arg(0, false),
            Arg(1, false),
            Arg(3, false),
        ], 
        // AExp74
        vec![ // 158 
            Prm(LT,false),
            Arg(2, false),
            Int(5),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 159 
            Com(4,155),
            Arg(0, false),
            Arg(1, false),
            Arg(2, false),
            Arg(3, false),
        ], 
        vec![ // 160 
            Com(4,123),
            Arg(0, false),
            Arg(1, false),
            Arg(2, false),
            Arg(3, false),
        ], 
        // AExp75
        vec![ // 161 
            Prm(LT,false),
            Arg(2, false),
            Int(7),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 162 
            Com(4,158),
            Arg(0, false),
            Arg(1, false),
            Arg(2, false),
            Arg(3, false),
        ], 
        vec![ // 163 
            Com(4,98),
            Arg(0, false),
            Arg(1, false),
            Arg(2, false),
            Arg(3, false),
        ], 
        // AExp76
        vec![ // 164 
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 165 
            Com(4,161),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp77
        vec![ // 166 
            Arg(0, false),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 167 
            Com(3,164),
            Arg(0, false),
        ], 
        // AExp78
        vec![ // 168 
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp79
        vec![ // 169 
            Arg(0, true),
            Int(8),
            Com(1,0),
        ], 
        // AExp80
        vec![ // 170 
            Arg(0, true),
            Int(4),
            Com(1,0),
        ], 
        // AExp81
        vec![ // 171 
            Prm(EQ,false),
            Arg(0, true),
            Int(10),
            Int(0),
            Ptr(0, true, true),
        ], 
        vec![ // 172 
            Arg(1, true),
            Com(1,0),
        ], 
        // AExp82
        vec![ // 173 
            Arg(0, true),
            Com(2,171),
        ], 
        // AExp83
        vec![ // 174 
            Arg(1, true),
            Arg(0, true),
        ], 
        // AExp84
        vec![ // 175 
            Arg(1, true),
            Int(10),
            Ptr(0, true, true),
        ], 
        vec![ // 176 
            Com(2,174),
            Arg(0, true),
        ], 
        // AExp85
        vec![ // 177 
            Com(4,168),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 178 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp86
        vec![ // 179 
            Arg(2, true),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 180 
            Com(3,177),
            Arg(1, true),
        ], 
        // AExp87
        vec![ // 181 
            Y,
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 182 
            Com(3,179),
            Arg(1, true),
        ], 
        // AExp88
        vec![ // 183 
            Prm(EQ,false),
            Arg(3, true),
            Int(2),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 184 
            Arg(4, true),
            Arg(1, true),
        ], 
        vec![ // 185 
            Com(4,168),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp89
        vec![ // 186 
            Arg(0, true),
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 187 
            Com(4,168),
            Arg(3, true),
            Arg(1, true),
        ], 
        // AExp90
        vec![ // 188 
            Com(5,183),
            Arg(1, true),
            Ptr(0, true, true),
            Arg(2, false),
        ], 
        vec![ // 189 
            Com(4,186),
            Arg(0, true),
            Arg(2, false),
        ], 
        // AExp91
        vec![ // 190 
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 191 
            Com(3,188),
            Arg(0, true),
            Arg(1, false),
            Arg(2, true),
        ], 
        // AExp92
        vec![ // 192 
            Y,
            Com(3,190),
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp93
        vec![ // 193 
            Arg(0, true),
            Int(9),
            Com(1,0),
        ], 
        // AExp94
        vec![ // 194 
            Com(3,21),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 195 
            Com(1,206),
            Arg(1, true),
        ], 
        vec![ // 196 
            Com(1,206),
            Arg(0, true),
        ], 
        // AExp95
        vec![ // 197 
            Prm(EQ,false),
            Arg(1, true),
            Int(2),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 198 
            Arg(2, true),
            Com(2,194),
        ], 
        // AExp96
        vec![ // 199 
            Com(1,227),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 200 
            Com(1,206),
            Arg(1, true),
        ], 
        // AExp97
        vec![ // 201 
            Prm(EQ,false),
            Arg(1, true),
            Int(1),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 202 
            Arg(2, true),
            Com(2,199),
        ], 
        // AExp98
        vec![ // 203 
            Prm(LT,false),
            Arg(1, false),
            Int(2),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 204 
            Com(3,201),
            Arg(0, false),
            Arg(1, false),
            Arg(2, false),
        ], 
        vec![ // 205 
            Com(3,197),
            Arg(0, false),
            Arg(1, false),
            Arg(2, false),
        ], 
        // AExp99
        vec![ // 206 
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 207 
            Com(3,203),
            Arg(0, false),
        ], 
        // AExp100
        vec![ // 208 
            Prm(EQ,false),
            Arg(2, true),
            Int(2),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 209 
            Arg(3, true),
            Arg(0, true),
        ], 
        // AExp101
        vec![ // 210 
            Com(0,279),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 211 
            Arg(0, false),
            Arg(2, true),
        ], 
        vec![ // 212 
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp102
        vec![ // 213 
            Com(4,208),
            Ptr(0, true, true),
        ], 
        vec![ // 214 
            Com(3,210),
            Arg(0, true),
        ], 
        // AExp103
        vec![ // 215 
            Prm(EQ,false),
            Arg(2, true),
            Int(0),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 216 
            Arg(3, true),
            Arg(0, true),
        ], 
        // AExp104
        vec![ // 217 
            Prm(EQ,false),
            Arg(0, true),
            Arg(2, true),
            Arg(1, true),
            Com(1,13),
        ], 
        // AExp105
        vec![ // 218 
            Com(4,215),
            Ptr(0, true, true),
            Arg(1, false),
        ], 
        vec![ // 219 
            Com(3,217),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp106
        vec![ // 220 
            Prm(LT,false),
            Arg(3, false),
            Int(2),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 221 
            Com(2,218),
            Arg(0, true),
            Arg(2, false),
            Arg(3, false),
            Arg(4, false),
        ], 
        vec![ // 222 
            Com(1,213),
            Arg(1, true),
            Arg(2, false),
            Arg(3, false),
            Arg(4, false),
        ], 
        // AExp107
        vec![ // 223 
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 224 
            Com(5,220),
            Arg(0, true),
            Arg(1, true),
            Arg(3, true),
        ], 
        // AExp108
        vec![ // 225 
            Com(4,223),
            Arg(0, true),
            Arg(1, true),
            Arg(2, false),
            Ptr(0, true, true),
        ], 
        vec![ // 226 
            Com(3,21),
            Com(1,170),
            Arg(2, false),
        ], 
        // AExp109
        vec![ // 227 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 228 
            Com(3,225),
            Arg(0, true),
        ], 
        // AExp110
        vec![ // 229 
            Prm(EQ,false),
            Arg(2, true),
            Int(2),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 230 
            Arg(3, true),
            Arg(0, true),
        ], 
        // AExp111
        vec![ // 231 
            Prm(EQ,false),
            Arg(2, true),
            Int(4),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 232 
            Arg(3, true),
            Arg(1, true),
        ], 
        // AExp112
        vec![ // 233 
            Prm(EQ,false),
            Arg(2, true),
            Int(2),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 234 
            Arg(3, true),
            Arg(0, true),
        ], 
        // AExp113
        vec![ // 235 
            Prm(EQ,false),
            Arg(2, true),
            Int(4),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 236 
            Arg(3, true),
            Arg(1, true),
        ], 
        // AExp114
        vec![ // 237 
            Com(3,21),
            Com(1,170),
            Ptr(0, true, true),
        ], 
        vec![ // 238 
            Com(3,21),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp115
        vec![ // 239 
            Com(4,235),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 240 
            Com(2,237),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp116
        vec![ // 241 
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 242 
            Com(3,239),
            Arg(0, true),
            Arg(1, true),
            Arg(3, true),
        ], 
        // AExp117
        vec![ // 243 
            Com(4,233),
            Ptr(0, true, true),
            Arg(1, false),
        ], 
        vec![ // 244 
            Com(4,241),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp118
        vec![ // 245 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 246 
            Com(2,243),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp119
        vec![ // 247 
            Com(3,21),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 248 
            Com(3,21),
            Com(1,280),
            Arg(1, true),
        ], 
        // AExp120
        vec![ // 249 
            Com(3,245),
            Arg(0, false),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 250 
            Com(2,247),
            Arg(0, false),
            Arg(1, false),
        ], 
        // AExp121
        vec![ // 251 
            Com(4,231),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 252 
            Com(2,249),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp122
        vec![ // 253 
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 254 
            Com(3,251),
            Arg(0, true),
            Arg(1, true),
            Arg(3, true),
        ], 
        // AExp123
        vec![ // 255 
            Com(4,229),
            Ptr(0, true, true),
            Arg(1, false),
        ], 
        vec![ // 256 
            Com(4,253),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp124
        vec![ // 257 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 258 
            Com(2,255),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp125
        vec![ // 259 
            Prm(EQ,false),
            Arg(2, true),
            Int(2),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 260 
            Arg(3, true),
            Arg(0, true),
        ], 
        // AExp126
        vec![ // 261 
            Prm(EQ,false),
            Arg(2, true),
            Int(4),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 262 
            Arg(3, true),
            Arg(1, true),
        ], 
        // AExp127
        vec![ // 263 
            Com(3,21),
            Ptr(0, true, true),
        ], 
        vec![ // 264 
            Com(3,21),
            Com(1,281),
            Arg(0, true),
        ], 
        // AExp128
        vec![ // 265 
            Com(4,261),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 266 
            Com(1,263),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp129
        vec![ // 267 
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 268 
            Com(3,265),
            Arg(0, true),
            Arg(1, true),
            Arg(3, true),
        ], 
        // AExp130
        vec![ // 269 
            Com(4,259),
            Ptr(0, true, true),
            Arg(1, false),
        ], 
        vec![ // 270 
            Com(4,267),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp131
        vec![ // 271 
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 272 
            Com(2,269),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp132
        vec![ // 273 
            Com(3,21),
            Ptr(0, true, true),
        ], 
        vec![ // 274 
            Com(3,21),
            Com(1,282),
            Arg(0, true),
        ], 
        // AExp133
        vec![ // 275 
            Com(3,271),
            Arg(0, false),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 276 
            Com(1,273),
            Arg(0, false),
            Arg(1, false),
        ], 
        // AExp134
        vec![ // 277 
            Com(3,257),
            Arg(0, false),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 278 
            Com(2,275),
            Arg(0, false),
            Arg(1, false),
        ], 
        // AExp135
        vec![ // 279 
            Com(2,277),
        ], 
        // AExp136
        vec![ // 280 
            Arg(0, true),
            Int(6),
            Com(1,0),
        ], 
        // AExp137
        vec![ // 281 
            Arg(0, true),
            Int(7),
            Com(1,0),
        ], 
        // AExp138
        vec![ // 282 
            Arg(0, true),
            Int(3),
            Com(1,0),
        ], 
        // AExp139
        vec![ // 283 
            Arg(2, true),
            Int(1),
            Ptr(0, true, true),
        ], 
        vec![ // 284 
            Com(3,20),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp140
        vec![ // 285 
            Arg(0, true),
            Int(12),
            Com(1,0),
        ], 
        // AExp141
        vec![ // 286 
            Arg(1, true),
            Int(0),
            Ptr(0, true, true),
        ], 
        vec![ // 287 
            Com(2,174),
            Arg(0, true),
        ], 
        // AExp142
        vec![ // 288 
            Arg(0, true),
            Int(11),
            Com(1,0),
        ], 
    ],

}});
