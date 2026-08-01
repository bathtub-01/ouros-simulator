use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 68
#[rustfmt::skip]
pub static ADJOXO: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            Com(2,15),
            Ptr(4, false, false),
            Ptr(2, false, false),
        ], 
        vec![ // 1 
            Com(4,2),
            Int(5),
            Com(2,0),
        ], 
        vec![ // 2 
            Com(4,2),
            Int(2),
            Ptr(1, false, false),
        ], 
        vec![ // 3 
            Com(4,2),
            Int(4),
            Com(2,0),
        ], 
        vec![ // 4 
            Com(4,2),
            Int(1),
            Ptr(3, false, false),
        ], 
        // AExp1
        vec![ // 5 
            Y,
            Com(3,26),
            Int(0),
        ], 
        // AExp2
        vec![ // 6 
            Com(3,28),
            Ptr(36, false, false),
            Ptr(33, false, false),
        ], 
        vec![ // 7 
            Com(4,2),
            Int(7),
            Com(2,0),
        ], 
        vec![ // 8 
            Com(4,2),
            Int(5),
            Ptr(7, false, false),
        ], 
        vec![ // 9 
            Com(1,63),
            Ptr(8, false, false),
        ], 
        vec![ // 10 
            Com(4,2),
            Int(9),
            Com(2,0),
        ], 
        vec![ // 11 
            Com(4,2),
            Int(5),
            Ptr(10, false, false),
        ], 
        vec![ // 12 
            Com(1,61),
            Ptr(11, false, false),
        ], 
        vec![ // 13 
            Com(3,58),
            Ptr(12, false, false),
            Ptr(9, false, false),
        ], 
        vec![ // 14 
            Com(4,2),
            Int(9),
            Com(2,0),
        ], 
        vec![ // 15 
            Com(4,2),
            Int(6),
            Ptr(14, false, false),
        ], 
        vec![ // 16 
            Com(1,56),
            Ptr(15, false, false),
        ], 
        vec![ // 17 
            Com(3,53),
            Ptr(16, false, false),
            Ptr(13, false, false),
        ], 
        vec![ // 18 
            Com(4,2),
            Int(8),
            Com(2,0),
        ], 
        vec![ // 19 
            Com(4,2),
            Int(5),
            Ptr(18, false, false),
        ], 
        vec![ // 20 
            Com(1,51),
            Ptr(19, false, false),
        ], 
        vec![ // 21 
            Com(3,48),
            Ptr(20, false, false),
            Ptr(17, false, false),
        ], 
        vec![ // 22 
            Com(4,2),
            Int(7),
            Com(2,0),
        ], 
        vec![ // 23 
            Com(4,2),
            Int(4),
            Ptr(22, false, false),
        ], 
        vec![ // 24 
            Com(1,46),
            Ptr(23, false, false),
        ], 
        vec![ // 25 
            Com(3,43),
            Ptr(24, false, false),
            Ptr(21, false, false),
        ], 
        vec![ // 26 
            Com(4,2),
            Int(9),
            Com(2,0),
        ], 
        vec![ // 27 
            Com(4,2),
            Int(8),
            Ptr(26, false, false),
        ], 
        vec![ // 28 
            Com(1,41),
            Ptr(27, false, false),
        ], 
        vec![ // 29 
            Com(3,38),
            Ptr(28, false, false),
            Ptr(25, false, false),
        ], 
        vec![ // 30 
            Com(4,2),
            Int(6),
            Com(2,0),
        ], 
        vec![ // 31 
            Com(4,2),
            Int(5),
            Ptr(30, false, false),
        ], 
        vec![ // 32 
            Com(1,36),
            Ptr(31, false, false),
        ], 
        vec![ // 33 
            Com(3,33),
            Ptr(32, false, false),
            Ptr(29, false, false),
        ], 
        vec![ // 34 
            Com(4,2),
            Int(3),
            Com(2,0),
        ], 
        vec![ // 35 
            Com(4,2),
            Int(2),
            Ptr(34, false, false),
        ], 
        vec![ // 36 
            Com(1,31),
            Ptr(35, false, false),
        ], 
        // AExp3
        vec![ // 37 
            Com(3,92),
            Ptr(42, false, false),
        ], 
        vec![ // 38 
            Com(2,146),
            Int(1),
            Int(9),
        ], 
        vec![ // 39 
            Com(2,83),
            Ptr(38, false, false),
        ], 
        vec![ // 40 
            Com(2,101),
            Ptr(39, false, false),
        ], 
        vec![ // 41 
            Com(3,96),
            Ptr(40, false, false),
        ], 
        vec![ // 42 
            Com(3,94),
            Ptr(41, false, false),
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
            Com(2,87),
            Ptr(0, true, true),
            Com(2,1),
        ], 
        vec![ // 4 
            Ptr(37, false, false),
            Arg(1, true),
            Arg(0, true),
        ], 
        // AExp4
        vec![ // 5 
            Ptr(6, false, false),
            Arg(0, false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 6 
            Com(2,87),
            Com(3,23),
            Com(2,0),
        ], 
        vec![ // 7 
            Com(2,3),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp5
        vec![ // 8 
            Ptr(6, false, false),
            Arg(1, false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 9 
            Com(2,87),
            Com(3,23),
            Com(2,1),
        ], 
        vec![ // 10 
            Com(2,5),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp6
        vec![ // 11 
            Com(2,87),
            Ptr(0, true, true),
            Com(2,0),
        ], 
        vec![ // 12 
            Ptr(37, false, false),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp7
        vec![ // 13 
            Com(2,87),
            Ptr(0, true, true),
            Com(2,1),
        ], 
        vec![ // 14 
            Ptr(37, false, false),
            Arg(1, true),
            Arg(0, true),
        ], 
        // AExp8
        vec![ // 15 
            Com(2,21),
            Ptr(4, true, true),
            Ptr(3, true, true),
            Ptr(2, true, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 16 
            Com(2,13),
            Arg(0, false),
            Arg(1, false),
        ], 
        vec![ // 17 
            Com(2,11),
            Arg(0, false),
            Arg(1, false),
        ], 
        vec![ // 18 
            Com(2,8),
            Arg(0, false),
            Arg(1, false),
        ], 
        vec![ // 19 
            Ptr(5, false, false),
            Arg(1, false),
        ], 
        vec![ // 20 
            Ptr(5, false, false),
            Arg(0, false),
        ], 
        // AExp9
        vec![ // 21 
            Prm(EQ,false),
            Arg(0, false),
            Arg(1, false),
            Ptr(0, true, true),
            Com(3,0),
        ], 
        vec![ // 22 
            Prm(LE,false),
            Arg(0, false),
            Arg(1, false),
            Com(3,23),
            Com(3,1),
        ], 
        // AExp10
        vec![ // 23 
            Arg(2, true),
        ], 
        // AExp11
        vec![ // 24 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 25 
            Prm(Add,false),
            Arg(1, true),
            Int(1),
        ], 
        // AExp12
        vec![ // 26 
            Arg(2, true),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 27 
            Com(3,24),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp13
        vec![ // 28 
            Com(2,65),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 29 
            Arg(1, true),
            Arg(2, false),
        ], 
        vec![ // 30 
            Arg(0, true),
            Arg(2, false),
        ], 
        // AExp14
        vec![ // 31 
            Com(2,66),
            Ptr(0, true, true),
        ], 
        vec![ // 32 
            Com(4,2),
            Int(1),
            Arg(0, true),
        ], 
        // AExp15
        vec![ // 33 
            Com(2,65),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 34 
            Arg(1, true),
            Arg(2, false),
        ], 
        vec![ // 35 
            Arg(0, true),
            Arg(2, false),
        ], 
        // AExp16
        vec![ // 36 
            Com(2,66),
            Ptr(0, true, true),
        ], 
        vec![ // 37 
            Com(4,2),
            Int(4),
            Arg(0, true),
        ], 
        // AExp17
        vec![ // 38 
            Com(2,65),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 39 
            Arg(1, true),
            Arg(2, false),
        ], 
        vec![ // 40 
            Arg(0, true),
            Arg(2, false),
        ], 
        // AExp18
        vec![ // 41 
            Com(2,66),
            Ptr(0, true, true),
        ], 
        vec![ // 42 
            Com(4,2),
            Int(7),
            Arg(0, true),
        ], 
        // AExp19
        vec![ // 43 
            Com(2,65),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 44 
            Arg(1, true),
            Arg(2, false),
        ], 
        vec![ // 45 
            Arg(0, true),
            Arg(2, false),
        ], 
        // AExp20
        vec![ // 46 
            Com(2,66),
            Ptr(0, true, true),
        ], 
        vec![ // 47 
            Com(4,2),
            Int(1),
            Arg(0, true),
        ], 
        // AExp21
        vec![ // 48 
            Com(2,65),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 49 
            Arg(1, true),
            Arg(2, false),
        ], 
        vec![ // 50 
            Arg(0, true),
            Arg(2, false),
        ], 
        // AExp22
        vec![ // 51 
            Com(2,66),
            Ptr(0, true, true),
        ], 
        vec![ // 52 
            Com(4,2),
            Int(2),
            Arg(0, true),
        ], 
        // AExp23
        vec![ // 53 
            Com(2,65),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 54 
            Arg(1, true),
            Arg(2, false),
        ], 
        vec![ // 55 
            Arg(0, true),
            Arg(2, false),
        ], 
        // AExp24
        vec![ // 56 
            Com(2,66),
            Ptr(0, true, true),
        ], 
        vec![ // 57 
            Com(4,2),
            Int(3),
            Arg(0, true),
        ], 
        // AExp25
        vec![ // 58 
            Com(2,65),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 59 
            Arg(1, true),
            Arg(2, false),
        ], 
        vec![ // 60 
            Arg(0, true),
            Arg(2, false),
        ], 
        // AExp26
        vec![ // 61 
            Com(2,66),
            Ptr(0, true, true),
        ], 
        vec![ // 62 
            Com(4,2),
            Int(1),
            Arg(0, true),
        ], 
        // AExp27
        vec![ // 63 
            Com(2,66),
            Ptr(0, true, true),
        ], 
        vec![ // 64 
            Com(4,2),
            Int(3),
            Arg(0, true),
        ], 
        // AExp28
        vec![ // 65 
            Arg(0, true),
            Arg(1, true),
            Com(2,1),
        ], 
        // AExp29
        vec![ // 66 
            Com(1,69),
            Ptr(0, true, true),
        ], 
        vec![ // 67 
            Com(2,83),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp30
        vec![ // 68 
            Com(2,0),
        ], 
        // AExp31
        vec![ // 69 
            Arg(0, true),
            Com(2,1),
            Com(2,68),
        ], 
        // AExp32
        vec![ // 70 
            Com(2,83),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 71 
            Com(4,2),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp33
        vec![ // 72 
            Com(4,2),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 73 
            Com(3,70),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        // AExp34
        vec![ // 74 
            Com(2,83),
            Ptr(0, true, true),
        ], 
        vec![ // 75 
            Com(4,2),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp35
        vec![ // 76 
            Com(2,21),
            Arg(0, false),
            Arg(2, false),
            Ptr(2, true, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 77 
            Com(2,74),
            Arg(0, false),
            Arg(1, false),
            Arg(3, false),
        ], 
        vec![ // 78 
            Com(4,72),
            Arg(0, false),
            Arg(1, false),
            Arg(2, false),
            Arg(3, false),
        ], 
        vec![ // 79 
            Com(2,83),
            Arg(1, false),
            Arg(3, false),
        ], 
        // AExp36
        vec![ // 80 
            Arg(0, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 81 
            Com(4,76),
            Arg(1, false),
            Arg(2, false),
        ], 
        vec![ // 82 
            Com(4,2),
            Arg(1, false),
            Arg(2, false),
        ], 
        // AExp37
        vec![ // 83 
            Arg(0, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 84 
            Com(3,80),
            Arg(1, true),
        ], 
        // AExp38
        vec![ // 85 
            Com(1,90),
            Ptr(0, true, true),
        ], 
        vec![ // 86 
            Com(1,91),
            Arg(0, true),
        ], 
        // AExp39
        vec![ // 87 
            Arg(0, true),
            Int(3),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 88 
            Com(1,90),
            Arg(1, false),
        ], 
        vec![ // 89 
            Com(1,85),
            Arg(1, false),
        ], 
        // AExp40
        vec![ // 90 
            Arg(0, true),
            Int(0),
            Int(88),
        ], 
        // AExp41
        vec![ // 91 
            Arg(0, true),
            Com(2,1),
            Com(2,0),
        ], 
        // AExp42
        vec![ // 92 
            Ptr(6, false, false),
            Arg(2, false),
            Ptr(0, true, true),
            Com(3,1),
        ], 
        vec![ // 93 
            Arg(0, true),
            Arg(1, true),
            Arg(2, false),
        ], 
        // AExp43
        vec![ // 94 
            Com(2,106),
            Arg(1, false),
            Arg(2, false),
            Ptr(0, true, true),
            Com(3,0),
        ], 
        vec![ // 95 
            Arg(0, true),
            Arg(1, false),
            Arg(2, false),
        ], 
        // AExp44
        vec![ // 96 
            Com(1,114),
            Com(2,116),
            Ptr(0, true, true),
        ], 
        vec![ // 97 
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp45
        vec![ // 98 
            Com(1,123),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 99 
            Com(2,83),
            Arg(0, true),
            Arg(2, false),
        ], 
        vec![ // 100 
            Com(3,127),
            Arg(1, true),
            Arg(2, false),
        ], 
        // AExp46
        vec![ // 101 
            Com(3,98),
            Ptr(0, true, true),
            Arg(1, false),
        ], 
        vec![ // 102 
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp47
        vec![ // 103 
            Prm(Add,false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 104 
            Ptr(5, false, false),
            Arg(1, true),
        ], 
        vec![ // 105 
            Ptr(5, false, false),
            Arg(0, true),
        ], 
        // AExp48
        vec![ // 106 
            Prm(EQ,false),
            Ptr(0, true, true),
            Int(9),
        ], 
        vec![ // 107 
            Com(2,103),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp49
        vec![ // 108 
            Arg(0, true),
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 109 
            Arg(1, true),
            Arg(3, true),
        ], 
        // AExp50
        vec![ // 110 
            Arg(3, false),
            Arg(2, false),
            Ptr(0, true, true),
        ], 
        vec![ // 111 
            Com(6,108),
            Arg(0, true),
            Arg(1, true),
            Arg(2, false),
            Arg(3, false),
        ], 
        // AExp51
        vec![ // 112 
            Arg(2, true),
            E(0),
            Ptr(0, true, true),
        ], 
        vec![ // 113 
            Com(4,110),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp52
        vec![ // 114 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 115 
            Com(3,112),
            Arg(0, true),
        ], 
        // AExp53
        vec![ // 116 
            Arg(0, true),
            Ptr(0, true, true),
            Arg(1, false),
            Com(3,23),
        ], 
        vec![ // 117 
            Arg(1, false),
            Com(3,0),
            Com(3,0),
            Com(3,23),
        ], 
        // AExp54
        vec![ // 118 
            Com(4,2),
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
        // AExp55
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
        // AExp56
        vec![ // 123 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 124 
            Com(3,121),
            Arg(0, true),
        ], 
        // AExp57
        vec![ // 125 
            Ptr(37, false, false),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 126 
            Com(1,140),
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp58
        vec![ // 127 
            Com(1,129),
            Ptr(0, true, true),
        ], 
        vec![ // 128 
            Com(3,125),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp59
        vec![ // 129 
            Arg(0, true),
            Com(3,0),
            Com(3,23),
            Com(3,1),
        ], 
        // AExp60
        vec![ // 130 
            Com(4,2),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 131 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp61
        vec![ // 132 
            Com(4,2),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 133 
            Com(4,2),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp62
        vec![ // 134 
            Prm(LE,false),
            Arg(0, false),
            Arg(2, false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 135 
            Com(3,132),
            Arg(0, false),
            Arg(2, false),
            Arg(3, false),
        ], 
        vec![ // 136 
            Com(3,130),
            Arg(1, true),
            Arg(2, false),
            Arg(3, false),
        ], 
        // AExp63
        vec![ // 137 
            Arg(2, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 138 
            Com(4,134),
            Arg(0, false),
            Arg(1, true),
        ], 
        vec![ // 139 
            Com(4,2),
            Arg(0, false),
            Com(2,0),
        ], 
        // AExp64
        vec![ // 140 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 141 
            Com(3,137),
            Arg(0, true),
        ], 
        // AExp65
        vec![ // 142 
            Com(2,146),
            Ptr(0, true, true),
        ], 
        vec![ // 143 
            Prm(Add,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp66
        vec![ // 144 
            Com(4,2),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 145 
            Com(1,142),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp67
        vec![ // 146 
            Prm(LE,false),
            Arg(0, false),
            Arg(1, false),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 147 
            Com(2,144),
            Arg(0, false),
            Arg(1, false),
        ], 
    ],

}});
