use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 81
#[rustfmt::skip]
pub static WHILEX: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            Com(1,18),
            Ptr(52, false, false),
            Int(5),
            Com(1,0),
        ], 
        vec![ // 1 
            Com(4,2),
            Com(1,8),
            Com(2,0),
        ], 
        vec![ // 2 
            Com(4,2),
            Com(1,7),
            Ptr(1, false, false),
        ], 
        vec![ // 3 
            Com(4,2),
            Com(1,6),
            Ptr(2, false, false),
        ], 
        vec![ // 4 
            Com(4,2),
            Com(1,5),
            Ptr(3, false, false),
        ], 
        vec![ // 5 
            Com(4,2),
            Com(1,4),
            Ptr(4, false, false),
        ], 
        vec![ // 6 
            Com(4,2),
            Com(1,3),
            Ptr(5, false, false),
        ], 
        vec![ // 7 
            Com(5,44),
            Int(1),
        ], 
        vec![ // 8 
            Com(5,116),
            Int(4),
        ], 
        vec![ // 9 
            Com(6,119),
            Ptr(8, false, false),
            Ptr(7, false, false),
        ], 
        vec![ // 10 
            Com(7,75),
            Int(4),
            Ptr(9, false, false),
        ], 
        vec![ // 11 
            Com(5,44),
            Int(1),
        ], 
        vec![ // 12 
            Com(5,116),
            Int(5),
        ], 
        vec![ // 13 
            Com(6,75),
            Ptr(12, false, false),
            Ptr(11, false, false),
        ], 
        vec![ // 14 
            Com(7,75),
            Int(5),
            Ptr(13, false, false),
        ], 
        vec![ // 15 
            Com(5,44),
            Int(0),
        ], 
        vec![ // 16 
            Com(5,116),
            Int(0),
        ], 
        vec![ // 17 
            Com(3,118),
            Ptr(16, false, false),
            Ptr(15, false, false),
        ], 
        vec![ // 18 
            Com(4,113),
            Ptr(17, false, false),
            Ptr(14, false, false),
            Com(5,114),
        ], 
        vec![ // 19 
            Com(5,44),
            Int(1),
        ], 
        vec![ // 20 
            Com(5,116),
            Int(2),
        ], 
        vec![ // 21 
            Com(6,75),
            Ptr(20, false, false),
            Ptr(19, false, false),
        ], 
        vec![ // 22 
            Com(7,75),
            Int(2),
            Ptr(21, false, false),
        ], 
        vec![ // 23 
            Com(5,116),
            Int(1),
        ], 
        vec![ // 24 
            Com(5,116),
            Int(0),
        ], 
        vec![ // 25 
            Com(6,119),
            Ptr(24, false, false),
            Ptr(23, false, false),
        ], 
        vec![ // 26 
            Com(7,75),
            Int(0),
            Ptr(25, false, false),
        ], 
        vec![ // 27 
            Com(7,2),
            Ptr(26, false, false),
            Ptr(22, false, false),
        ], 
        vec![ // 28 
            Com(5,116),
            Int(0),
        ], 
        vec![ // 29 
            Com(5,116),
            Int(1),
        ], 
        vec![ // 30 
            Com(3,120),
            Ptr(29, false, false),
            Ptr(28, false, false),
        ], 
        vec![ // 31 
            Com(7,115),
            Ptr(30, false, false),
        ], 
        vec![ // 32 
            Com(1,0),
            Ptr(31, false, false),
            Ptr(27, false, false),
        ], 
        vec![ // 33 
            Com(5,116),
            Int(4),
        ], 
        vec![ // 34 
            Com(7,75),
            Int(1),
            Ptr(33, false, false),
        ], 
        vec![ // 35 
            Com(5,116),
            Int(3),
        ], 
        vec![ // 36 
            Com(7,75),
            Int(0),
            Ptr(35, false, false),
        ], 
        vec![ // 37 
            Com(7,2),
            Ptr(36, false, false),
            Ptr(34, false, false),
        ], 
        vec![ // 38 
            Com(7,2),
            Ptr(37, false, false),
            Ptr(32, false, false),
        ], 
        vec![ // 39 
            Com(7,2),
            Ptr(38, false, false),
        ], 
        vec![ // 40 
            Com(1,0),
            Ptr(39, false, false),
            Ptr(18, false, false),
        ], 
        vec![ // 41 
            Com(7,2),
            Ptr(40, false, false),
            Ptr(10, false, false),
        ], 
        vec![ // 42 
            Com(5,44),
            Int(0),
        ], 
        vec![ // 43 
            Com(5,116),
            Int(4),
        ], 
        vec![ // 44 
            Com(3,118),
            Ptr(43, false, false),
            Ptr(42, false, false),
        ], 
        vec![ // 45 
            Com(7,117),
            Ptr(44, false, false),
        ], 
        vec![ // 46 
            Com(7,115),
            Ptr(45, false, false),
        ], 
        vec![ // 47 
            Com(1,0),
            Ptr(46, false, false),
            Ptr(41, false, false),
        ], 
        vec![ // 48 
            Com(5,116),
            Int(3),
        ], 
        vec![ // 49 
            Com(7,75),
            Int(4),
            Ptr(48, false, false),
        ], 
        vec![ // 50 
            Com(7,2),
            Ptr(49, false, false),
        ], 
        vec![ // 51 
            Com(1,0),
            Ptr(50, false, false),
            Ptr(47, false, false),
        ], 
        vec![ // 52 
            Com(2,19),
            Ptr(51, false, false),
            Ptr(6, false, false),
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
            Arg(0, true),
            Int(0),
            Int(0),
        ], 
        // AExp4
        vec![ // 4 
            Arg(0, true),
            Int(1),
            Int(0),
        ], 
        // AExp5
        vec![ // 5 
            Arg(0, true),
            Int(2),
            Int(0),
        ], 
        // AExp6
        vec![ // 6 
            Arg(0, true),
            Int(3),
            Int(17),
        ], 
        // AExp7
        vec![ // 7 
            Arg(0, true),
            Int(4),
            Int(0),
        ], 
        // AExp8
        vec![ // 8 
            Arg(0, true),
            Int(5),
            Int(0),
        ], 
        // AExp9
        vec![ // 9 
            E(42),
        ], 
        // AExp10
        vec![ // 10 
            Arg(0, true),
            Arg(2, true),
            Arg(1, true),
        ], 
        // AExp11
        vec![ // 11 
            Prm(EQ,false),
            Arg(5, true),
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
            Arg(4, true),
            Arg(6, true),
            Arg(2, true),
        ], 
        // AExp12
        vec![ // 12 
            Com(1,18),
            Arg(2, true),
            Arg(3, true),
            Arg(0, true),
        ], 
        // AExp13
        vec![ // 13 
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp14
        vec![ // 14 
            Com(3,10),
            Ptr(0, true, true),
            Arg(1, false),
        ], 
        vec![ // 15 
            Com(7,11),
            Com(4,12),
            Com(4,13),
            Arg(0, true),
            Arg(1, false),
            Arg(2, true),
            Arg(3, true),
        ], 
        // AExp15
        vec![ // 16 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 17 
            Com(4,14),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        // AExp16
        vec![ // 18 
            Arg(0, true),
            Com(2,9),
            Com(4,16),
        ], 
        // AExp17
        vec![ // 19 
            Com(1,23),
            Ptr(0, true, true),
        ], 
        vec![ // 20 
            Com(4,2),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp18
        vec![ // 21 
            Com(1,23),
            Ptr(0, true, true),
        ], 
        vec![ // 22 
            Com(1,38),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp19
        vec![ // 23 
            Arg(0, true),
            Com(1,0),
            Com(2,21),
        ], 
        // AExp20
        vec![ // 24 
            Com(1,50),
            Arg(1, true),
            Arg(2, false),
            Ptr(0, true, true),
        ], 
        vec![ // 25 
            Com(3,72),
            Arg(2, false),
            Arg(0, true),
            Com(3,39),
        ], 
        // AExp21
        vec![ // 26 
            Com(4,2),
            Arg(1, true),
            Arg(0, true),
        ], 
        // AExp22
        vec![ // 27 
            Com(4,2),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 28 
            Com(7,2),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp23
        vec![ // 29 
            Com(1,38),
            Arg(0, true),
            Arg(2, true),
            Com(2,26),
            Com(3,27),
            Arg(1, true),
        ], 
        // AExp24
        vec![ // 30 
            Com(1,94),
            Arg(0, true),
            Arg(3, false),
            Ptr(0, true, true),
        ], 
        vec![ // 31 
            Com(4,111),
            Arg(3, false),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp25
        vec![ // 32 
            Com(4,2),
            Ptr(0, true, true),
        ], 
        vec![ // 33 
            Com(4,113),
            Arg(0, true),
            Arg(1, true),
            Com(5,114),
        ], 
        // AExp26
        vec![ // 34 
            Com(7,2),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 35 
            Com(7,115),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp27
        vec![ // 36 
            Com(2,32),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 37 
            Com(2,34),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp28
        vec![ // 38 
            Arg(0, true),
            Com(3,24),
            Com(3,29),
            Com(4,30),
            Com(3,39),
            Com(2,36),
        ], 
        // AExp29
        vec![ // 39 
            Arg(1, true),
            Arg(0, true),
        ], 
        // AExp30
        vec![ // 40 
            Com(3,51),
            Ptr(2, true, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 41 
            Com(3,55),
            Arg(3, true),
        ], 
        vec![ // 42 
            Com(1,50),
            Arg(1, true),
            Arg(2, false),
        ], 
        vec![ // 43 
            Com(1,50),
            Arg(0, true),
            Arg(2, false),
        ], 
        // AExp31
        vec![ // 44 
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp32
        vec![ // 45 
            Com(3,51),
            Ptr(2, true, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 46 
            Com(3,59),
            Arg(3, true),
        ], 
        vec![ // 47 
            Com(1,50),
            Arg(1, true),
            Arg(2, false),
        ], 
        vec![ // 48 
            Com(1,50),
            Arg(0, true),
            Arg(2, false),
        ], 
        // AExp33
        vec![ // 49 
            Com(1,18),
            Arg(1, true),
            Arg(0, true),
        ], 
        // AExp34
        vec![ // 50 
            Arg(0, true),
            Com(4,40),
            Com(3,44),
            Com(4,45),
            Com(2,49),
        ], 
        // AExp35
        vec![ // 51 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 52 
            Com(3,53),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp36
        vec![ // 53 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 54 
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp37
        vec![ // 55 
            Com(2,58),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 56 
            Prm(Add,false),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp38
        vec![ // 57 
            Arg(0, true),
            Int(0),
        ], 
        // AExp39
        vec![ // 58 
            Prm(EQ,false),
            Arg(0, false),
            Int(0),
            Com(1,0),
            Com(2,57),
            Arg(1, true),
            Arg(0, false),
        ], 
        // AExp40
        vec![ // 59 
            Com(2,58),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 60 
            Prm(Sub,false),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp41
        vec![ // 61 
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp42
        vec![ // 62 
            Arg(0, true),
            Arg(2, false),
            Arg(1, true),
            Arg(2, false),
        ], 
        // AExp43
        vec![ // 63 
            Com(3,72),
            Arg(0, true),
            Arg(1, true),
            Ptr(0, true, true),
            Arg(3, true),
        ], 
        vec![ // 64 
            Com(3,76),
            Arg(2, true),
            Arg(4, true),
            Arg(5, true),
        ], 
        // AExp44
        vec![ // 65 
            Com(3,72),
            Arg(0, true),
            Arg(1, false),
            Ptr(0, true, true),
            Arg(3, false),
        ], 
        vec![ // 66 
            Com(3,76),
            Arg(2, true),
            Arg(1, false),
            Arg(3, false),
        ], 
        // AExp45
        vec![ // 67 
            Prm(EQ,false),
            Arg(3, true),
            Arg(2, false),
            Com(6,63),
            Com(6,65),
            Arg(0, true),
            Arg(2, false),
            Arg(1, true),
        ], 
        // AExp46
        vec![ // 68 
            Com(3,62),
            Ptr(0, true, true),
        ], 
        vec![ // 69 
            Com(4,67),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp47
        vec![ // 70 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 71 
            Com(3,68),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
            Arg(4, true),
        ], 
        // AExp48
        vec![ // 72 
            Arg(0, true),
            Com(3,61),
            Com(5,70),
            Arg(2, true),
            Arg(1, true),
        ], 
        // AExp49
        vec![ // 73 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 74 
            Com(4,2),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp50
        vec![ // 75 
            Arg(2, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp51
        vec![ // 76 
            Com(3,73),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 77 
            Com(3,75),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp52
        vec![ // 78 
            Com(3,51),
            Ptr(2, true, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 79 
            Com(3,95),
            Arg(3, true),
        ], 
        vec![ // 80 
            Com(1,94),
            Arg(1, true),
            Arg(2, false),
        ], 
        vec![ // 81 
            Com(1,94),
            Arg(0, true),
            Arg(2, false),
        ], 
        // AExp53
        vec![ // 82 
            Com(3,51),
            Ptr(2, true, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 83 
            Com(3,102),
            Arg(3, true),
        ], 
        vec![ // 84 
            Com(1,50),
            Arg(1, true),
            Arg(2, false),
        ], 
        vec![ // 85 
            Com(1,50),
            Arg(0, true),
            Arg(2, false),
        ], 
        // AExp54
        vec![ // 86 
            Arg(1, true),
            Com(2,0),
        ], 
        // AExp55
        vec![ // 87 
            Com(3,51),
            Ptr(2, true, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 88 
            Com(3,104),
            Arg(3, true),
        ], 
        vec![ // 89 
            Com(1,50),
            Arg(1, true),
            Arg(2, false),
        ], 
        vec![ // 90 
            Com(1,50),
            Arg(0, true),
            Arg(2, false),
        ], 
        // AExp56
        vec![ // 91 
            Com(1,94),
            Arg(0, true),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 92 
            Com(2,106),
            Arg(2, true),
        ], 
        // AExp57
        vec![ // 93 
            Arg(1, true),
            Com(2,1),
        ], 
        // AExp58
        vec![ // 94 
            Arg(0, true),
            Com(4,78),
            Com(4,82),
            Com(2,86),
            Com(4,87),
            Com(3,91),
            Com(2,93),
        ], 
        // AExp59
        vec![ // 95 
            Com(1,99),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 96 
            Com(1,101),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp60
        vec![ // 97 
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp61
        vec![ // 98 
            Arg(0, true),
            Com(2,1),
        ], 
        // AExp62
        vec![ // 99 
            Arg(0, true),
            Com(1,97),
            Com(1,98),
        ], 
        // AExp63
        vec![ // 100 
            Com(2,0),
        ], 
        // AExp64
        vec![ // 101 
            Arg(0, true),
            Com(1,100),
            Com(1,0),
        ], 
        // AExp65
        vec![ // 102 
            Com(1,99),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 103 
            Prm(EQ,false),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp66
        vec![ // 104 
            Com(1,99),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 105 
            Prm(LE,false),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp67
        vec![ // 106 
            Com(1,99),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 107 
            Com(1,108),
            Arg(1, true),
        ], 
        // AExp68
        vec![ // 108 
            Arg(0, true),
            Com(2,1),
            Com(2,0),
        ], 
        // AExp69
        vec![ // 109 
            Com(4,2),
            Arg(2, true),
            Arg(1, true),
        ], 
        // AExp70
        vec![ // 110 
            Com(4,2),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp71
        vec![ // 111 
            Arg(3, true),
            Com(3,109),
            Com(3,110),
            Arg(1, true),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp72
        vec![ // 112 
            Arg(4, true),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp73
        vec![ // 113 
            Com(7,112),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp74
        vec![ // 114 
            Arg(3, true),
        ], 
        // AExp75
        vec![ // 115 
            Arg(6, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp76
        vec![ // 116 
            Arg(4, true),
            Arg(0, true),
        ], 
        // AExp77
        vec![ // 117 
            Arg(5, true),
            Arg(0, true),
        ], 
        // AExp78
        vec![ // 118 
            Com(7,75),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp79
        vec![ // 119 
            Arg(4, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp80
        vec![ // 120 
            Com(7,119),
            Arg(0, true),
            Arg(1, true),
        ], 
    ],

}});
