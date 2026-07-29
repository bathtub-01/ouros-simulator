use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 73
#[rustfmt::skip]
pub static ADJOXO: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            Com(2,12),
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
            Com(3,24),
            Int(0),
        ], 
        // AExp2
        vec![ // 6 
            Com(3,26),
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
            Com(1,61),
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
            Com(1,59),
            Ptr(11, false, false),
        ], 
        vec![ // 13 
            Com(3,56),
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
            Com(1,54),
            Ptr(15, false, false),
        ], 
        vec![ // 17 
            Com(3,51),
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
            Com(1,49),
            Ptr(19, false, false),
        ], 
        vec![ // 21 
            Com(3,46),
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
            Com(1,44),
            Ptr(23, false, false),
        ], 
        vec![ // 25 
            Com(3,41),
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
            Com(1,39),
            Ptr(27, false, false),
        ], 
        vec![ // 29 
            Com(3,36),
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
            Com(1,34),
            Ptr(31, false, false),
        ], 
        vec![ // 33 
            Com(3,31),
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
            Com(1,29),
            Ptr(35, false, false),
        ], 
        // AExp3
        vec![ // 37 
            Com(3,91),
            Ptr(42, false, false),
        ], 
        vec![ // 38 
            Com(2,141),
            Int(1),
            Int(9),
        ], 
        vec![ // 39 
            Com(2,81),
            Ptr(38, false, false),
        ], 
        vec![ // 40 
            Com(2,99),
            Ptr(39, false, false),
        ], 
        vec![ // 41 
            Com(3,94),
            Ptr(40, false, false),
        ], 
        vec![ // 42 
            Com(3,93),
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
            Com(2,85),
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
            Com(2,85),
            Com(3,21),
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
            Com(2,85),
            Com(3,21),
            Com(2,1),
        ], 
        // AExp7
        vec![ // 8 
            Com(2,85),
            Ptr(0, true, true),
            Com(2,0),
        ], 
        vec![ // 9 
            Ptr(37, false, false),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp8
        vec![ // 10 
            Com(2,85),
            Ptr(0, true, true),
            Com(2,1),
        ], 
        vec![ // 11 
            Ptr(37, false, false),
            Arg(1, true),
            Arg(0, true),
        ], 
        // AExp9
        vec![ // 12 
            Com(2,20),
            Ptr(4, true, true),
            Ptr(3, true, true),
            Ptr(2, true, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 13 
            Com(2,10),
            Arg(0, false),
            Arg(1, false),
        ], 
        vec![ // 14 
            Com(2,8),
            Arg(0, false),
            Arg(1, false),
        ], 
        vec![ // 15 
            Ptr(6, false, false),
            Arg(1, false),
            Com(2,6),
            Com(2,7),
            Arg(0, false),
            Arg(1, false),
        ], 
        vec![ // 16 
            Ptr(5, false, false),
            Arg(1, false),
        ], 
        vec![ // 17 
            Ptr(5, false, false),
            Arg(0, false),
        ], 
        // AExp10
        vec![ // 18 
            Prm(LE,false),
            Arg(0, true),
            Arg(1, true),
            Com(3,21),
            Com(3,1),
        ], 
        // AExp11
        vec![ // 19 
            Com(3,0),
        ], 
        // AExp12
        vec![ // 20 
            Prm(EQ,false),
            Arg(0, false),
            Arg(1, false),
            Com(2,18),
            Com(2,19),
            Arg(0, false),
            Arg(1, false),
        ], 
        // AExp13
        vec![ // 21 
            Arg(2, true),
        ], 
        // AExp14
        vec![ // 22 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 23 
            Prm(Add,false),
            Arg(1, true),
            Int(1),
        ], 
        // AExp15
        vec![ // 24 
            Arg(2, true),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 25 
            Com(3,22),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp16
        vec![ // 26 
            Com(2,63),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 27 
            Arg(1, true),
            Arg(2, false),
        ], 
        vec![ // 28 
            Arg(0, true),
            Arg(2, false),
        ], 
        // AExp17
        vec![ // 29 
            Com(2,64),
            Ptr(0, true, true),
        ], 
        vec![ // 30 
            Com(4,2),
            Int(1),
            Arg(0, true),
        ], 
        // AExp18
        vec![ // 31 
            Com(2,63),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 32 
            Arg(1, true),
            Arg(2, false),
        ], 
        vec![ // 33 
            Arg(0, true),
            Arg(2, false),
        ], 
        // AExp19
        vec![ // 34 
            Com(2,64),
            Ptr(0, true, true),
        ], 
        vec![ // 35 
            Com(4,2),
            Int(4),
            Arg(0, true),
        ], 
        // AExp20
        vec![ // 36 
            Com(2,63),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 37 
            Arg(1, true),
            Arg(2, false),
        ], 
        vec![ // 38 
            Arg(0, true),
            Arg(2, false),
        ], 
        // AExp21
        vec![ // 39 
            Com(2,64),
            Ptr(0, true, true),
        ], 
        vec![ // 40 
            Com(4,2),
            Int(7),
            Arg(0, true),
        ], 
        // AExp22
        vec![ // 41 
            Com(2,63),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 42 
            Arg(1, true),
            Arg(2, false),
        ], 
        vec![ // 43 
            Arg(0, true),
            Arg(2, false),
        ], 
        // AExp23
        vec![ // 44 
            Com(2,64),
            Ptr(0, true, true),
        ], 
        vec![ // 45 
            Com(4,2),
            Int(1),
            Arg(0, true),
        ], 
        // AExp24
        vec![ // 46 
            Com(2,63),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 47 
            Arg(1, true),
            Arg(2, false),
        ], 
        vec![ // 48 
            Arg(0, true),
            Arg(2, false),
        ], 
        // AExp25
        vec![ // 49 
            Com(2,64),
            Ptr(0, true, true),
        ], 
        vec![ // 50 
            Com(4,2),
            Int(2),
            Arg(0, true),
        ], 
        // AExp26
        vec![ // 51 
            Com(2,63),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 52 
            Arg(1, true),
            Arg(2, false),
        ], 
        vec![ // 53 
            Arg(0, true),
            Arg(2, false),
        ], 
        // AExp27
        vec![ // 54 
            Com(2,64),
            Ptr(0, true, true),
        ], 
        vec![ // 55 
            Com(4,2),
            Int(3),
            Arg(0, true),
        ], 
        // AExp28
        vec![ // 56 
            Com(2,63),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 57 
            Arg(1, true),
            Arg(2, false),
        ], 
        vec![ // 58 
            Arg(0, true),
            Arg(2, false),
        ], 
        // AExp29
        vec![ // 59 
            Com(2,64),
            Ptr(0, true, true),
        ], 
        vec![ // 60 
            Com(4,2),
            Int(1),
            Arg(0, true),
        ], 
        // AExp30
        vec![ // 61 
            Com(2,64),
            Ptr(0, true, true),
        ], 
        vec![ // 62 
            Com(4,2),
            Int(3),
            Arg(0, true),
        ], 
        // AExp31
        vec![ // 63 
            Arg(0, true),
            Arg(1, true),
            Com(2,1),
        ], 
        // AExp32
        vec![ // 64 
            Com(1,67),
            Ptr(0, true, true),
        ], 
        vec![ // 65 
            Com(2,81),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp33
        vec![ // 66 
            Com(2,0),
        ], 
        // AExp34
        vec![ // 67 
            Arg(0, true),
            Com(2,1),
            Com(2,66),
        ], 
        // AExp35
        vec![ // 68 
            Com(2,81),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 69 
            Com(4,2),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp36
        vec![ // 70 
            Com(4,2),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 71 
            Com(3,68),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        // AExp37
        vec![ // 72 
            Com(2,81),
            Ptr(0, true, true),
        ], 
        vec![ // 73 
            Com(4,2),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp38
        vec![ // 74 
            Com(2,20),
            Arg(0, false),
            Arg(2, false),
            Ptr(2, true, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 75 
            Com(2,72),
            Arg(0, false),
            Arg(1, false),
            Arg(3, false),
        ], 
        vec![ // 76 
            Com(4,70),
            Arg(0, false),
            Arg(1, false),
            Arg(2, false),
            Arg(3, false),
        ], 
        vec![ // 77 
            Com(2,81),
            Arg(1, false),
            Arg(3, false),
        ], 
        // AExp39
        vec![ // 78 
            Arg(0, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 79 
            Com(4,74),
            Arg(1, false),
            Arg(2, false),
        ], 
        vec![ // 80 
            Com(4,2),
            Arg(1, false),
            Arg(2, false),
        ], 
        // AExp40
        vec![ // 81 
            Arg(0, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 82 
            Com(3,78),
            Arg(1, true),
        ], 
        // AExp41
        vec![ // 83 
            Com(1,88),
            Ptr(0, true, true),
        ], 
        vec![ // 84 
            Com(1,89),
            Arg(0, true),
        ], 
        // AExp42
        vec![ // 85 
            Arg(0, true),
            Int(3),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 86 
            Com(1,88),
            Arg(1, false),
        ], 
        vec![ // 87 
            Com(1,83),
            Arg(1, false),
        ], 
        // AExp43
        vec![ // 88 
            Arg(0, true),
            Int(0),
            Int(88),
        ], 
        // AExp44
        vec![ // 89 
            Arg(0, true),
            Com(2,1),
            Com(2,0),
        ], 
        // AExp45
        vec![ // 90 
            Com(3,1),
        ], 
        // AExp46
        vec![ // 91 
            Ptr(6, false, false),
            Arg(2, false),
            Arg(0, true),
            Com(2,90),
            Arg(1, true),
            Arg(2, false),
        ], 
        // AExp47
        vec![ // 92 
            Com(3,0),
        ], 
        // AExp48
        vec![ // 93 
            Com(2,104),
            Arg(1, false),
            Arg(2, false),
            Arg(0, true),
            Com(2,92),
            Arg(1, false),
            Arg(2, false),
        ], 
        // AExp49
        vec![ // 94 
            Com(1,112),
            Com(2,114),
            Ptr(0, true, true),
        ], 
        vec![ // 95 
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp50
        vec![ // 96 
            Com(1,121),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 97 
            Com(2,81),
            Arg(0, true),
            Arg(2, false),
        ], 
        vec![ // 98 
            Com(3,125),
            Arg(1, true),
            Arg(2, false),
        ], 
        // AExp51
        vec![ // 99 
            Com(3,96),
            Ptr(0, true, true),
            Arg(1, false),
        ], 
        vec![ // 100 
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp52
        vec![ // 101 
            Prm(Add,false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 102 
            Ptr(5, false, false),
            Arg(1, true),
        ], 
        vec![ // 103 
            Ptr(5, false, false),
            Arg(0, true),
        ], 
        // AExp53
        vec![ // 104 
            Prm(EQ,false),
            Ptr(0, true, true),
            Int(9),
        ], 
        vec![ // 105 
            Com(2,101),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp54
        vec![ // 106 
            Arg(0, true),
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 107 
            Arg(1, true),
            Arg(3, true),
        ], 
        // AExp55
        vec![ // 108 
            Arg(3, false),
            Arg(2, false),
            Ptr(0, true, true),
        ], 
        vec![ // 109 
            Com(6,106),
            Arg(0, true),
            Arg(1, true),
            Arg(2, false),
            Arg(3, false),
        ], 
        // AExp56
        vec![ // 110 
            Arg(2, true),
            Err(0),
            Ptr(0, true, true),
        ], 
        vec![ // 111 
            Com(4,108),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp57
        vec![ // 112 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 113 
            Com(3,110),
            Arg(0, true),
        ], 
        // AExp58
        vec![ // 114 
            Arg(0, true),
            Ptr(0, true, true),
            Arg(1, false),
            Com(3,21),
        ], 
        vec![ // 115 
            Arg(1, false),
            Com(3,0),
            Com(3,0),
            Com(3,21),
        ], 
        // AExp59
        vec![ // 116 
            Com(4,2),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 117 
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 118 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp60
        vec![ // 119 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 120 
            Com(4,116),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp61
        vec![ // 121 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 122 
            Com(3,119),
            Arg(0, true),
        ], 
        // AExp62
        vec![ // 123 
            Ptr(37, false, false),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 124 
            Com(2,133),
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp63
        vec![ // 125 
            Com(1,127),
            Ptr(0, true, true),
        ], 
        vec![ // 126 
            Com(3,123),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp64
        vec![ // 127 
            Arg(0, true),
            Com(3,0),
            Com(3,21),
            Com(3,1),
        ], 
        // AExp65
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
        // AExp66
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
        // AExp67
        vec![ // 132 
            Prm(LE,false),
            Arg(0, false),
            Arg(1, false),
            Com(3,128),
            Com(3,130),
            Arg(0, false),
            Arg(1, false),
        ], 
        // AExp68
        vec![ // 133 
            Arg(1, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 134 
            Com(2,132),
            Arg(0, false),
        ], 
        vec![ // 135 
            Com(4,2),
            Arg(0, false),
            Com(2,0),
        ], 
        // AExp69
        vec![ // 136 
            Com(2,0),
        ], 
        // AExp70
        vec![ // 137 
            Com(2,141),
            Ptr(0, true, true),
        ], 
        vec![ // 138 
            Prm(Add,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp71
        vec![ // 139 
            Com(4,2),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 140 
            Com(1,137),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp72
        vec![ // 141 
            Prm(LE,false),
            Arg(0, false),
            Arg(1, false),
            Com(2,136),
            Com(2,139),
            Arg(0, false),
            Arg(1, false),
        ], 
    ],

}});