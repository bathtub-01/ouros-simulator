use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 86
#[rustfmt::skip]
pub static ADJOXO: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            Com(2,13),
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
            Com(3,22),
            Int(0),
        ], 
        // AExp2
        vec![ // 6 
            Com(3,23),
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
            Com(1,58),
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
            Com(1,56),
            Ptr(11, false, false),
        ], 
        vec![ // 13 
            Com(3,53),
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
            Com(1,51),
            Ptr(15, false, false),
        ], 
        vec![ // 17 
            Com(3,48),
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
            Com(1,46),
            Ptr(19, false, false),
        ], 
        vec![ // 21 
            Com(3,43),
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
            Com(1,41),
            Ptr(23, false, false),
        ], 
        vec![ // 25 
            Com(3,38),
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
            Com(1,36),
            Ptr(27, false, false),
        ], 
        vec![ // 29 
            Com(3,33),
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
            Com(1,31),
            Ptr(31, false, false),
        ], 
        vec![ // 33 
            Com(3,28),
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
            Com(1,26),
            Ptr(35, false, false),
        ], 
        // AExp3
        vec![ // 37 
            Com(3,87),
            Ptr(42, false, false),
        ], 
        vec![ // 38 
            Com(2,139),
            Int(1),
            Int(9),
        ], 
        vec![ // 39 
            Com(1,79),
            Ptr(38, false, false),
        ], 
        vec![ // 40 
            Com(2,95),
            Ptr(39, false, false),
        ], 
        vec![ // 41 
            Com(3,90),
            Ptr(40, false, false),
        ], 
        vec![ // 42 
            Com(3,89),
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
            Com(1,83),
            Ptr(0, true, true),
            Com(2,1),
        ], 
        vec![ // 4 
            Ptr(37, false, false),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp4
        vec![ // 5 
            Com(1,83),
            Com(3,19),
            Com(2,0),
        ], 
        // AExp5
        vec![ // 6 
            Ptr(6, false, false),
            Arg(0, false),
            Com(2,3),
            Com(2,5),
            Arg(1, true),
            Arg(0, false),
        ], 
        // AExp6
        vec![ // 7 
            Com(1,83),
            Com(3,19),
            Com(2,1),
        ], 
        // AExp7
        vec![ // 8 
            Ptr(6, false, false),
            Arg(0, false),
            Com(2,6),
            Com(2,7),
            Arg(1, true),
            Arg(0, false),
        ], 
        // AExp8
        vec![ // 9 
            Com(1,83),
            Ptr(0, true, true),
            Com(2,0),
        ], 
        vec![ // 10 
            Ptr(37, false, false),
            Arg(1, true),
            Arg(0, true),
        ], 
        // AExp9
        vec![ // 11 
            Com(1,83),
            Ptr(0, true, true),
            Com(2,1),
        ], 
        vec![ // 12 
            Ptr(37, false, false),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp10
        vec![ // 13 
            Com(2,18),
            Ptr(1, true, true),
            Ptr(0, true, true),
            Com(2,8),
            Com(2,9),
            Com(2,11),
            Arg(1, false),
            Arg(0, false),
        ], 
        vec![ // 14 
            Ptr(5, false, false),
            Arg(1, false),
        ], 
        vec![ // 15 
            Ptr(5, false, false),
            Arg(0, false),
        ], 
        // AExp11
        vec![ // 16 
            Prm(LE,false),
            Arg(0, true),
            Arg(1, true),
            Com(3,19),
            Com(3,1),
        ], 
        // AExp12
        vec![ // 17 
            Com(3,0),
        ], 
        // AExp13
        vec![ // 18 
            Prm(EQ,false),
            Arg(0, false),
            Arg(1, false),
            Com(2,16),
            Com(2,17),
            Arg(0, false),
            Arg(1, false),
        ], 
        // AExp14
        vec![ // 19 
            Arg(2, true),
        ], 
        // AExp15
        vec![ // 20 
            Arg(3, true),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 21 
            Prm(Add,false),
            Arg(2, true),
            Int(1),
        ], 
        // AExp16
        vec![ // 22 
            Arg(2, true),
            Com(2,0),
            Com(4,20),
            Arg(1, true),
            Arg(0, true),
        ], 
        // AExp17
        vec![ // 23 
            Com(1,61),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 24 
            Arg(1, true),
            Arg(2, false),
        ], 
        vec![ // 25 
            Arg(0, true),
            Arg(2, false),
        ], 
        // AExp18
        vec![ // 26 
            Com(2,62),
            Ptr(0, true, true),
        ], 
        vec![ // 27 
            Com(4,2),
            Int(1),
            Arg(0, true),
        ], 
        // AExp19
        vec![ // 28 
            Com(1,61),
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
        // AExp20
        vec![ // 31 
            Com(2,62),
            Ptr(0, true, true),
        ], 
        vec![ // 32 
            Com(4,2),
            Int(4),
            Arg(0, true),
        ], 
        // AExp21
        vec![ // 33 
            Com(1,61),
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
        // AExp22
        vec![ // 36 
            Com(2,62),
            Ptr(0, true, true),
        ], 
        vec![ // 37 
            Com(4,2),
            Int(7),
            Arg(0, true),
        ], 
        // AExp23
        vec![ // 38 
            Com(1,61),
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
        // AExp24
        vec![ // 41 
            Com(2,62),
            Ptr(0, true, true),
        ], 
        vec![ // 42 
            Com(4,2),
            Int(1),
            Arg(0, true),
        ], 
        // AExp25
        vec![ // 43 
            Com(1,61),
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
        // AExp26
        vec![ // 46 
            Com(2,62),
            Ptr(0, true, true),
        ], 
        vec![ // 47 
            Com(4,2),
            Int(2),
            Arg(0, true),
        ], 
        // AExp27
        vec![ // 48 
            Com(1,61),
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
        // AExp28
        vec![ // 51 
            Com(2,62),
            Ptr(0, true, true),
        ], 
        vec![ // 52 
            Com(4,2),
            Int(3),
            Arg(0, true),
        ], 
        // AExp29
        vec![ // 53 
            Com(1,61),
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
        // AExp30
        vec![ // 56 
            Com(2,62),
            Ptr(0, true, true),
        ], 
        vec![ // 57 
            Com(4,2),
            Int(1),
            Arg(0, true),
        ], 
        // AExp31
        vec![ // 58 
            Com(2,62),
            Ptr(0, true, true),
        ], 
        vec![ // 59 
            Com(4,2),
            Int(3),
            Arg(0, true),
        ], 
        // AExp32
        vec![ // 60 
            Com(2,1),
        ], 
        // AExp33
        vec![ // 61 
            Arg(0, true),
            Com(1,0),
            Com(1,60),
        ], 
        // AExp34
        vec![ // 62 
            Com(1,65),
            Ptr(0, true, true),
        ], 
        vec![ // 63 
            Com(1,79),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp35
        vec![ // 64 
            Com(2,0),
        ], 
        // AExp36
        vec![ // 65 
            Arg(0, true),
            Com(2,1),
            Com(2,64),
        ], 
        // AExp37
        vec![ // 66 
            Com(2,0),
        ], 
        // AExp38
        vec![ // 67 
            Arg(0, true),
            Arg(3, true),
            Arg(2, true),
            Arg(1, true),
        ], 
        // AExp39
        vec![ // 68 
            Com(1,79),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp40
        vec![ // 69 
            Com(1,79),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 70 
            Com(4,2),
            Arg(2, true),
            Arg(1, true),
        ], 
        // AExp41
        vec![ // 71 
            Com(4,2),
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 72 
            Com(3,69),
            Arg(0, true),
            Arg(1, true),
            Arg(3, true),
        ], 
        // AExp42
        vec![ // 73 
            Com(1,79),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 74 
            Com(4,2),
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp43
        vec![ // 75 
            Com(2,18),
            Arg(2, true),
            Arg(0, true),
            Com(4,68),
            Com(4,71),
            Com(4,73),
            Arg(3, true),
            Arg(1, true),
        ], 
        // AExp44
        vec![ // 76 
            Com(4,67),
            Ptr(0, true, true),
            Arg(0, false),
            Arg(2, false),
        ], 
        vec![ // 77 
            Com(4,75),
            Arg(0, false),
            Arg(1, true),
            Arg(2, false),
        ], 
        // AExp45
        vec![ // 78 
            Arg(2, true),
            Com(4,2),
            Com(3,76),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp46
        vec![ // 79 
            Arg(0, true),
            Com(1,66),
            Com(3,78),
        ], 
        // AExp47
        vec![ // 80 
            Int(3),
        ], 
        // AExp48
        vec![ // 81 
            Com(1,84),
            Ptr(0, true, true),
        ], 
        vec![ // 82 
            Com(1,85),
            Arg(0, true),
        ], 
        // AExp49
        vec![ // 83 
            Arg(0, true),
            Com(1,80),
            Com(1,81),
            Com(1,84),
        ], 
        // AExp50
        vec![ // 84 
            Arg(0, true),
            Int(0),
            Int(88),
        ], 
        // AExp51
        vec![ // 85 
            Arg(0, true),
            Com(2,1),
            Com(2,0),
        ], 
        // AExp52
        vec![ // 86 
            Com(3,1),
        ], 
        // AExp53
        vec![ // 87 
            Ptr(6, false, false),
            Arg(2, false),
            Arg(0, true),
            Com(2,86),
            Arg(1, true),
            Arg(2, false),
        ], 
        // AExp54
        vec![ // 88 
            Com(3,0),
        ], 
        // AExp55
        vec![ // 89 
            Com(2,100),
            Arg(1, false),
            Arg(2, false),
            Arg(0, true),
            Com(2,88),
            Arg(1, false),
            Arg(2, false),
        ], 
        // AExp56
        vec![ // 90 
            Com(1,110),
            Com(1,114),
            Ptr(0, true, true),
        ], 
        vec![ // 91 
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp57
        vec![ // 92 
            Com(1,120),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 93 
            Com(1,79),
            Arg(0, true),
            Arg(2, false),
        ], 
        vec![ // 94 
            Com(3,124),
            Arg(1, true),
            Arg(2, false),
        ], 
        // AExp58
        vec![ // 95 
            Com(3,92),
            Ptr(0, true, true),
            Arg(1, false),
        ], 
        vec![ // 96 
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp59
        vec![ // 97 
            Prm(Add,false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 98 
            Ptr(5, false, false),
            Arg(1, true),
        ], 
        vec![ // 99 
            Ptr(5, false, false),
            Arg(0, true),
        ], 
        // AExp60
        vec![ // 100 
            Prm(EQ,false),
            Ptr(0, true, true),
            Int(9),
        ], 
        vec![ // 101 
            Com(2,97),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp61
        vec![ // 102 
            Err(0),
        ], 
        // AExp62
        vec![ // 103 
            Arg(3, true),
        ], 
        // AExp63
        vec![ // 104 
            Arg(3, true),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 105 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp64
        vec![ // 106 
            Arg(2, false),
            Com(2,0),
            Com(4,103),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 107 
            Com(4,104),
            Arg(0, true),
            Arg(1, false),
            Arg(2, false),
            Arg(3, true),
        ], 
        // AExp65
        vec![ // 108 
            Arg(2, true),
            Com(1,102),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 109 
            Com(4,106),
            Arg(1, true),
        ], 
        // AExp66
        vec![ // 110 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 111 
            Com(3,108),
            Arg(0, true),
        ], 
        // AExp67
        vec![ // 112 
            Arg(0, true),
            Com(3,0),
            Com(3,0),
            Com(3,19),
        ], 
        // AExp68
        vec![ // 113 
            Com(3,19),
        ], 
        // AExp69
        vec![ // 114 
            Arg(0, true),
            Com(1,112),
            Com(1,0),
            Com(1,113),
        ], 
        // AExp70
        vec![ // 115 
            Com(2,0),
        ], 
        // AExp71
        vec![ // 116 
            Com(4,2),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 117 
            Arg(3, true),
            Arg(1, true),
        ], 
        vec![ // 118 
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp72
        vec![ // 119 
            Arg(2, true),
            Com(2,115),
            Com(4,116),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp73
        vec![ // 120 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 121 
            Com(3,119),
            Arg(0, true),
        ], 
        // AExp74
        vec![ // 122 
            Ptr(37, false, false),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 123 
            Com(2,133),
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp75
        vec![ // 124 
            Com(1,126),
            Ptr(0, true, true),
        ], 
        vec![ // 125 
            Com(3,122),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp76
        vec![ // 126 
            Arg(0, true),
            Com(3,0),
            Com(3,19),
            Com(3,1),
        ], 
        // AExp77
        vec![ // 127 
            Com(4,2),
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp78
        vec![ // 128 
            Com(4,2),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 129 
            Com(2,133),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp79
        vec![ // 130 
            Com(4,2),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 131 
            Com(4,2),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp80
        vec![ // 132 
            Prm(LE,false),
            Arg(2, false),
            Arg(0, false),
            Com(3,128),
            Com(3,130),
            Arg(2, false),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp81
        vec![ // 133 
            Arg(1, true),
            Com(1,127),
            Com(3,132),
            Arg(0, true),
        ], 
        // AExp82
        vec![ // 134 
            Com(2,0),
        ], 
        // AExp83
        vec![ // 135 
            Com(2,139),
            Ptr(0, true, true),
        ], 
        vec![ // 136 
            Prm(Add,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp84
        vec![ // 137 
            Com(4,2),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 138 
            Com(1,135),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp85
        vec![ // 139 
            Prm(LE,false),
            Arg(0, false),
            Arg(1, false),
            Com(2,134),
            Com(2,137),
            Arg(0, false),
            Arg(1, false),
        ], 
    ],

}});