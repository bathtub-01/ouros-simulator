use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 102
#[rustfmt::skip]
pub static COUNTDOWN: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            PTR(5, false, false),
            PTR(4, false, false),
        ], 
        vec![ // 1 
            COM(4,2),
            INT(10),
            COM(2,0),
        ], 
        vec![ // 2 
            COM(4,2),
            INT(4),
            PTR(1, false, false),
        ], 
        vec![ // 3 
            COM(4,2),
            INT(3),
            PTR(2, false, false),
        ], 
        vec![ // 4 
            COM(2,7),
            PTR(3, false, false),
            INT(70),
        ], 
        // AExp1
        vec![ // 5 
            Y,
            PTR(6, false, false),
            INT(0),
        ], 
        vec![ // 6 
            COM(4,3),
            COM(3,5),
        ], 
        // AExp2
        vec![ // 7 
            COM(2,10),
            PTR(8, false, false),
        ], 
        vec![ // 8 
            COM(4,12),
            COM(4,14),
        ], 
        // AExp3
        vec![ // 9 
            COM(3,17),
            PTR(10, false, false),
        ], 
        vec![ // 10 
            COM(4,19),
            COM(3,21),
        ], 
        // AExp4
        vec![ // 11 
            COM(2,185),
            COM(2,187),
        ], 
        // AExp5
        vec![ // 12 
            COM(2,190),
            PTR(14, false, false),
        ], 
        vec![ // 13 
            COM(6,197),
            COM(3,200),
            COM(3,202),
        ], 
        vec![ // 14 
            COM(5,192),
            COM(1,195),
            PTR(13, false, false),
        ], 
        // AExp6
        vec![ // 15 
            COM(2,171),
            PTR(16, false, false),
        ], 
        vec![ // 16 
            COM(4,173),
            COM(4,175),
        ], 
        // AExp7
        vec![ // 17 
            COM(2,205),
            PTR(19, false, false),
        ], 
        vec![ // 18 
            COM(3,209),
            COM(1,211),
        ], 
        vec![ // 19 
            COM(3,207),
            PTR(18, false, false),
        ], 
        // AExp8
        vec![ // 20 
            COM(2,25),
            PTR(23, false, false),
        ], 
        vec![ // 21 
            COM(6,31),
            COM(3,34),
        ], 
        vec![ // 22 
            COM(5,29),
            PTR(21, false, false),
        ], 
        vec![ // 23 
            COM(4,27),
            PTR(22, false, false),
        ], 
        // AExp9
        vec![ // 24 
            COM(2,36),
            PTR(27, false, false),
        ], 
        vec![ // 25 
            COM(3,42),
            COM(2,44),
        ], 
        vec![ // 26 
            COM(4,39),
            PTR(25, false, false),
        ], 
        vec![ // 27 
            COM(3,37),
            PTR(26, false, false),
            COM(2,46),
        ], 
        // AExp10
        vec![ // 28 
            COM(2,48),
            COM(2,49),
        ], 
        // AExp11
        vec![ // 29 
            COM(2,50),
            COM(2,51),
        ], 
        // AExp12
        vec![ // 30 
            COM(2,54),
            PTR(32, false, false),
        ], 
        vec![ // 31 
            COM(6,58),
            COM(3,61),
        ], 
        vec![ // 32 
            COM(5,56),
            PTR(31, false, false),
        ], 
        // AExp13
        vec![ // 33 
            COM(3,63),
            PTR(38, false, false),
        ], 
        vec![ // 34 
            COM(4,2),
            PTR(63, false, false),
            COM(2,0),
        ], 
        vec![ // 35 
            COM(4,2),
            PTR(62, false, false),
            PTR(34, false, false),
        ], 
        vec![ // 36 
            COM(1,69),
            PTR(35, false, false),
        ], 
        vec![ // 37 
            COM(5,67),
            PTR(36, false, false),
        ], 
        vec![ // 38 
            COM(4,65),
            PTR(37, false, false),
        ], 
        // AExp14
        vec![ // 39 
            COM(6,71),
            PTR(40, false, false),
        ], 
        vec![ // 40 
            COM(6,73),
            COM(6,75),
        ], 
        // AExp15
        vec![ // 41 
            COM(4,65),
            PTR(45, false, false),
        ], 
        vec![ // 42 
            COM(4,84),
            COM(3,90),
            COM(2,93),
        ], 
        vec![ // 43 
            COM(4,84),
            COM(3,86),
            COM(2,88),
        ], 
        vec![ // 44 
            COM(5,81),
            PTR(43, false, false),
        ], 
        vec![ // 45 
            COM(6,78),
            PTR(44, false, false),
            PTR(42, false, false),
        ], 
        // AExp16
        vec![ // 46 
            COM(3,96),
            COM(2,1),
        ], 
        // AExp17
        vec![ // 47 
            COM(2,97),
            PTR(50, false, false),
        ], 
        vec![ // 48 
            COM(5,112),
            COM(3,115),
            COM(4,116),
        ], 
        vec![ // 49 
            COM(4,110),
            PTR(48, false, false),
            COM(1,119),
        ], 
        vec![ // 50 
            COM(7,99),
            COM(7,104),
            COM(2,107),
            COM(3,108),
            PTR(49, false, false),
        ], 
        // AExp18
        vec![ // 51 
            COM(5,120),
            COM(4,122),
        ], 
        // AExp19
        vec![ // 52 
            COM(4,65),
            PTR(56, false, false),
        ], 
        vec![ // 53 
            COM(6,135),
            COM(3,138),
            COM(3,140),
        ], 
        vec![ // 54 
            COM(5,129),
            COM(3,131),
        ], 
        vec![ // 55 
            COM(6,126),
            PTR(54, false, false),
            COM(3,133),
        ], 
        vec![ // 56 
            COM(6,123),
            PTR(55, false, false),
            PTR(53, false, false),
        ], 
        // AExp20
        vec![ // 57 
            COM(3,142),
            COM(3,144),
            PTR(58, false, false),
        ], 
        vec![ // 58 
            COM(4,146),
            COM(1,149),
        ], 
        // AExp21
        vec![ // 59 
            COM(3,151),
            COM(2,0),
        ], 
        // AExp22
        vec![ // 60 
            COM(2,152),
            COM(1,0),
        ], 
        // AExp23
        vec![ // 61 
            COM(2,153),
            COM(1,0),
        ], 
        // AExp24
        vec![ // 62 
            COM(2,154),
            COM(1,0),
        ], 
        // AExp25
        vec![ // 63 
            COM(2,155),
            COM(1,0),
        ], 
        // AExp26
        vec![ // 64 
            COM(2,156),
            PTR(67, false, false),
        ], 
        vec![ // 65 
            COM(3,164),
            COM(2,166),
            COM(2,169),
        ], 
        vec![ // 66 
            COM(4,159),
            COM(3,162),
            PTR(65, false, false),
        ], 
        vec![ // 67 
            COM(3,157),
            PTR(66, false, false),
        ], 
        // AExp27
        vec![ // 68 
            COM(3,63),
            PTR(69, false, false),
        ], 
        vec![ // 69 
            COM(4,65),
            COM(5,178),
        ], 
        // AExp28
        vec![ // 70 
            COM(3,181),
            COM(2,50),
        ], 
    ],
    comb_img: vec![
        // AExp0
        vec![ // 0 
            ARG(0),
        ], 
        // AExp1
        vec![ // 1 
            ARG(1),
        ], 
        // AExp2
        vec![ // 2 
            ARG(3),
            ARG(0),
            ARG(1),
        ], 
        // AExp3
        vec![ // 3 
            ARG(3),
            ARG(2),
            PTR(0, true, true),
        ], 
        vec![ // 4 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp4
        vec![ // 5 
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 6 
            PRM(ADD,false),
            ARG(1),
            INT(1),
        ], 
        // AExp5
        vec![ // 7 
            PTR(7, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 8 
            COM(1,183),
            ARG(0),
        ], 
        vec![ // 9 
            COM(2,23),
            ARG(1),
        ], 
        // AExp6
        vec![ // 10 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 11 
            ARG(0),
            ARG(1),
        ], 
        // AExp7
        vec![ // 12 
            ARG(3),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 13 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp8
        vec![ // 14 
            PTR(9, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 15 
            ARG(1),
            ARG(3),
        ], 
        vec![ // 16 
            ARG(0),
            ARG(2),
        ], 
        // AExp9
        vec![ // 17 
            Y,
            PTR(0, true, true),
            ARG(1),
        ], 
        vec![ // 18 
            ARG(0),
            ARG(2),
        ], 
        // AExp10
        vec![ // 19 
            ARG(3),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 20 
            ARG(0),
            ARG(2),
        ], 
        // AExp11
        vec![ // 21 
            COM(4,2),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 22 
            ARG(0),
            ARG(2),
        ], 
        // AExp12
        vec![ // 23 
            PTR(20, false, false),
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 24 
            PTR(24, false, false),
            ARG(1),
        ], 
        // AExp13
        vec![ // 25 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 26 
            ARG(0),
            ARG(1),
        ], 
        // AExp14
        vec![ // 27 
            ARG(3),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 28 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp15
        vec![ // 29 
            ARG(3),
            PTR(0, true, true),
        ], 
        vec![ // 30 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(4),
        ], 
        // AExp16
        vec![ // 31 
            PRM(EQ,false),
            ARG(5),
            ARG(1),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 32 
            ARG(0),
            ARG(2),
            ARG(3),
            ARG(4),
        ], 
        vec![ // 33 
            ARG(2),
            ARG(3),
        ], 
        // AExp17
        vec![ // 34 
            COM(4,2),
            ARG(2),
            PTR(0, true, true),
        ], 
        vec![ // 35 
            ARG(0),
            ARG(1),
        ], 
        // AExp18
        vec![ // 36 
            ARG(1),
            COM(2,0),
            ARG(0),
        ], 
        // AExp19
        vec![ // 37 
            ARG(0),
            ARG(2),
            PTR(0, true, true),
        ], 
        vec![ // 38 
            ARG(1),
            ARG(2),
        ], 
        // AExp20
        vec![ // 39 
            PTR(28, false, false),
            ARG(3),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 40 
            COM(4,2),
            ARG(2),
            COM(2,0),
        ], 
        vec![ // 41 
            ARG(0),
            ARG(1),
            ARG(3),
        ], 
        // AExp21
        vec![ // 42 
            PTR(7, false, false),
            PTR(29, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 43 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp22
        vec![ // 44 
            PTR(64, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 45 
            COM(4,2),
            ARG(0),
            ARG(1),
        ], 
        // AExp23
        vec![ // 46 
            ARG(1),
            PTR(0, true, true),
            ARG(0),
        ], 
        vec![ // 47 
            PTR(70, false, false),
            ARG(0),
        ], 
        // AExp24
        vec![ // 48 
            ARG(1),
            COM(2,1),
            ARG(0),
        ], 
        // AExp25
        vec![ // 49 
            COM(2,0),
        ], 
        // AExp26
        vec![ // 50 
            ARG(1),
            ARG(0),
        ], 
        // AExp27
        vec![ // 51 
            PTR(30, false, false),
            PTR(33, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 52 
            PTR(24, false, false),
            ARG(1),
        ], 
        vec![ // 53 
            PTR(24, false, false),
            ARG(0),
        ], 
        // AExp28
        vec![ // 54 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 55 
            ARG(0),
            ARG(1),
        ], 
        // AExp29
        vec![ // 56 
            ARG(3),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 57 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(4),
        ], 
        // AExp30
        vec![ // 58 
            PTR(9, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 59 
            ARG(2),
            ARG(5),
            ARG(3),
        ], 
        vec![ // 60 
            ARG(0),
            ARG(1),
            ARG(3),
            ARG(4),
        ], 
        // AExp31
        vec![ // 61 
            PTR(7, false, false),
            PTR(0, true, true),
            ARG(1),
        ], 
        vec![ // 62 
            ARG(0),
            ARG(2),
        ], 
        // AExp32
        vec![ // 63 
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 64 
            ARG(0),
            ARG(2),
        ], 
        // AExp33
        vec![ // 65 
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 66 
            ARG(0),
            ARG(2),
            ARG(3),
        ], 
        // AExp34
        vec![ // 67 
            PTR(7, false, false),
            PTR(0, true, true),
            ARG(0),
        ], 
        vec![ // 68 
            PTR(39, false, false),
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(4),
        ], 
        // AExp35
        vec![ // 69 
            COM(4,2),
            PTR(60, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 70 
            COM(4,2),
            PTR(61, false, false),
            ARG(0),
        ], 
        // AExp36
        vec![ // 71 
            PTR(41, false, false),
            ARG(5),
            ARG(2),
            ARG(4),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 72 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(4),
            ARG(5),
        ], 
        // AExp37
        vec![ // 73 
            COM(4,2),
            PTR(0, true, true),
            COM(2,0),
        ], 
        vec![ // 74 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(4),
            ARG(5),
        ], 
        // AExp38
        vec![ // 75 
            ARG(5),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 76 
            PTR(52, false, false),
            ARG(4),
            ARG(1),
            ARG(3),
        ], 
        vec![ // 77 
            PTR(51, false, false),
            ARG(4),
            ARG(0),
            ARG(2),
        ], 
        // AExp39
        vec![ // 78 
            PRM(LT,false),
            ARG(4),
            INT(2),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 79 
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(4),
            ARG(5),
        ], 
        vec![ // 80 
            ARG(0),
            ARG(2),
            ARG(3),
            ARG(4),
            ARG(5),
        ], 
        // AExp40
        vec![ // 81 
            PRM(LT,false),
            ARG(3),
            INT(3),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 82 
            ARG(4),
            COM(2,1),
        ], 
        vec![ // 83 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(4),
        ], 
        // AExp41
        vec![ // 84 
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 85 
            ARG(1),
            ARG(2),
            ARG(3),
        ], 
        // AExp42
        vec![ // 86 
            PRM(EQ,false),
            ARG(1),
            INT(3),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 87 
            ARG(2),
            ARG(0),
        ], 
        // AExp43
        vec![ // 88 
            COM(1,95),
            PTR(0, true, true),
        ], 
        vec![ // 89 
            PRM(LE,false),
            ARG(0),
            ARG(1),
        ], 
        // AExp44
        vec![ // 90 
            PRM(LT,false),
            ARG(1),
            INT(1),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 91 
            ARG(2),
            COM(2,1),
        ], 
        vec![ // 92 
            ARG(2),
            ARG(0),
        ], 
        // AExp45
        vec![ // 93 
            PRM(EQ,false),
            PTR(0, true, true),
            INT(0),
        ], 
        vec![ // 94 
            PTR(46, false, false),
            ARG(0),
            ARG(1),
        ], 
        // AExp46
        vec![ // 95 
            ARG(0),
            COM(2,1),
            COM(2,0),
        ], 
        // AExp47
        vec![ // 96 
            PTR(47, false, false),
            ARG(1),
            ARG(2),
            ARG(0),
        ], 
        // AExp48
        vec![ // 97 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 98 
            ARG(0),
            ARG(1),
        ], 
        // AExp49
        vec![ // 99 
            ARG(0),
            PTR(3, true, true),
            ARG(4),
            PTR(2, true, true),
            ARG(5),
            PTR(1, true, true),
            ARG(6),
            PTR(0, true, true),
        ], 
        vec![ // 100 
            PRM(ADD,false),
            ARG(6),
            ARG(6),
        ], 
        vec![ // 101 
            ARG(3),
            ARG(6),
        ], 
        vec![ // 102 
            ARG(2),
            ARG(4),
            ARG(6),
        ], 
        vec![ // 103 
            ARG(1),
            ARG(4),
        ], 
        // AExp50
        vec![ // 104 
            PRM(LE,false),
            ARG(6),
            ARG(1),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 105 
            ARG(3),
            ARG(6),
            ARG(4),
        ], 
        vec![ // 106 
            PRM(LE,false),
            ARG(5),
            ARG(1),
            ARG(0),
            ARG(2),
        ], 
        // AExp51
        vec![ // 107 
            ARG(1),
            INT(0),
            ARG(0),
        ], 
        // AExp52
        vec![ // 108 
            ARG(2),
            INT(1),
            PTR(0, true, true),
        ], 
        vec![ // 109 
            PRM(SUB,false),
            ARG(0),
            ARG(1),
        ], 
        // AExp53
        vec![ // 110 
            ARG(0),
            ARG(2),
            PTR(0, true, true),
        ], 
        vec![ // 111 
            ARG(1),
            ARG(3),
        ], 
        // AExp54
        vec![ // 112 
            PRM(LE,false),
            ARG(2),
            ARG(4),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 113 
            ARG(1),
            ARG(2),
            ARG(4),
            ARG(3),
        ], 
        vec![ // 114 
            ARG(0),
            ARG(4),
            ARG(3),
        ], 
        // AExp55
        vec![ // 115 
            ARG(2),
            ARG(1),
            ARG(0),
        ], 
        // AExp56
        vec![ // 116 
            ARG(3),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 117 
            PRM(SUB,false),
            ARG(1),
            ARG(0),
        ], 
        vec![ // 118 
            PRM(ADD,false),
            ARG(2),
            INT(1),
        ], 
        // AExp57
        vec![ // 119 
            PRM(ADD,false),
            ARG(0),
            ARG(0),
        ], 
        // AExp58
        vec![ // 120 
            ARG(4),
            INT(5),
            PTR(0, true, true),
        ], 
        vec![ // 121 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(3),
        ], 
        // AExp59
        vec![ // 122 
            ARG(3),
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp60
        vec![ // 123 
            PRM(LT,false),
            ARG(4),
            INT(2),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 124 
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(4),
            ARG(5),
        ], 
        vec![ // 125 
            ARG(0),
            ARG(2),
            ARG(3),
            ARG(4),
            ARG(5),
        ], 
        // AExp61
        vec![ // 126 
            PRM(LT,false),
            ARG(4),
            INT(3),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 127 
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(5),
        ], 
        vec![ // 128 
            ARG(0),
            ARG(2),
            ARG(3),
            ARG(4),
            ARG(5),
        ], 
        // AExp62
        vec![ // 129 
            PRM(EQ,false),
            ARG(3),
            INT(3),
            INT(0),
            PTR(0, true, true),
        ], 
        vec![ // 130 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(4),
        ], 
        // AExp63
        vec![ // 131 
            ARG(2),
            PTR(0, true, true),
        ], 
        vec![ // 132 
            PRM(SUB,false),
            ARG(0),
            ARG(1),
        ], 
        // AExp64
        vec![ // 133 
            ARG(2),
            PTR(0, true, true),
        ], 
        vec![ // 134 
            PTR(57, false, false),
            ARG(0),
            ARG(1),
        ], 
        // AExp65
        vec![ // 135 
            PRM(LT,false),
            ARG(4),
            INT(1),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 136 
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(5),
        ], 
        vec![ // 137 
            ARG(0),
            ARG(2),
            ARG(3),
            ARG(5),
        ], 
        // AExp66
        vec![ // 138 
            ARG(2),
            PTR(0, true, true),
        ], 
        vec![ // 139 
            PTR(59, false, false),
            ARG(0),
            ARG(1),
        ], 
        // AExp67
        vec![ // 140 
            ARG(2),
            PTR(0, true, true),
        ], 
        vec![ // 141 
            PRM(ADD,false),
            ARG(0),
            ARG(1),
        ], 
        // AExp68
        vec![ // 142 
            ARG(0),
            PTR(0, true, true),
            ARG(2),
        ], 
        vec![ // 143 
            ARG(1),
            ARG(2),
        ], 
        // AExp69
        vec![ // 144 
            PRM(EQ,false),
            ARG(2),
            INT(1),
            PTR(0, true, true),
            ARG(1),
        ], 
        vec![ // 145 
            PTR(47, false, false),
            ARG(2),
            INT(2),
            ARG(0),
        ], 
        // AExp70
        vec![ // 146 
            PRM(ADD,false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 147 
            PRM(EQ,false),
            ARG(3),
            INT(0),
            ARG(1),
            INT(0),
        ], 
        vec![ // 148 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp71
        vec![ // 149 
            PTR(57, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 150 
            PRM(ADD,false),
            ARG(0),
            ARG(0),
        ], 
        // AExp72
        vec![ // 151 
            PTR(47, false, false),
            ARG(1),
            ARG(2),
            ARG(0),
        ], 
        // AExp73
        vec![ // 152 
            ARG(1),
            INT(0),
            ARG(0),
        ], 
        // AExp74
        vec![ // 153 
            ARG(1),
            INT(3),
            ARG(0),
        ], 
        // AExp75
        vec![ // 154 
            ARG(1),
            INT(2),
            ARG(0),
        ], 
        // AExp76
        vec![ // 155 
            ARG(1),
            INT(1),
            ARG(0),
        ], 
        // AExp77
        vec![ // 156 
            ARG(1),
            COM(2,0),
            ARG(0),
        ], 
        // AExp78
        vec![ // 157 
            PTR(28, false, false),
            ARG(2),
            PTR(0, true, true),
            COM(2,0),
        ], 
        vec![ // 158 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp79
        vec![ // 159 
            COM(4,2),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 160 
            ARG(1),
            ARG(2),
            ARG(3),
        ], 
        vec![ // 161 
            ARG(0),
            ARG(2),
            ARG(3),
        ], 
        // AExp80
        vec![ // 162 
            ARG(2),
            PTR(0, true, true),
            ARG(1),
        ], 
        vec![ // 163 
            COM(4,2),
            ARG(0),
            COM(2,0),
        ], 
        // AExp81
        vec![ // 164 
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 165 
            ARG(1),
            ARG(2),
        ], 
        // AExp82
        vec![ // 166 
            PTR(15, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 167 
            PTR(64, false, false),
            ARG(1),
        ], 
        vec![ // 168 
            PTR(68, false, false),
            ARG(0),
        ], 
        // AExp83
        vec![ // 169 
            ARG(1),
            PTR(0, true, true),
            COM(1,0),
        ], 
        vec![ // 170 
            COM(4,2),
            ARG(0),
        ], 
        // AExp84
        vec![ // 171 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 172 
            ARG(0),
            ARG(1),
        ], 
        // AExp85
        vec![ // 173 
            ARG(3),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 174 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp86
        vec![ // 175 
            COM(4,2),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 176 
            ARG(1),
            ARG(3),
        ], 
        vec![ // 177 
            ARG(0),
            ARG(2),
        ], 
        // AExp87
        vec![ // 178 
            ARG(4),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 179 
            ARG(1),
            ARG(3),
        ], 
        vec![ // 180 
            ARG(0),
            ARG(2),
        ], 
        // AExp88
        vec![ // 181 
            ARG(2),
            INT(4),
            PTR(0, true, true),
        ], 
        vec![ // 182 
            ARG(0),
            ARG(1),
        ], 
        // AExp89
        vec![ // 183 
            PTR(7, false, false),
            PTR(11, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 184 
            PTR(17, false, false),
            ARG(0),
        ], 
        // AExp90
        vec![ // 185 
            ARG(1),
            PTR(0, true, true),
            ARG(0),
        ], 
        vec![ // 186 
            COM(4,2),
            COM(2,0),
            COM(2,0),
        ], 
        // AExp91
        vec![ // 187 
            PTR(7, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 188 
            PTR(11, false, false),
            ARG(1),
        ], 
        vec![ // 189 
            PTR(12, false, false),
            ARG(0),
        ], 
        // AExp92
        vec![ // 190 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 191 
            ARG(0),
            ARG(1),
        ], 
        // AExp93
        vec![ // 192 
            ARG(4),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 193 
            ARG(1),
            ARG(2),
            ARG(3),
        ], 
        vec![ // 194 
            ARG(0),
            ARG(2),
        ], 
        // AExp94
        vec![ // 195 
            COM(4,2),
            PTR(0, true, true),
            COM(2,0),
        ], 
        vec![ // 196 
            COM(4,2),
            ARG(0),
            COM(2,0),
        ], 
        // AExp95
        vec![ // 197 
            COM(4,2),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 198 
            ARG(1),
            ARG(3),
            ARG(4),
            ARG(5),
        ], 
        vec![ // 199 
            ARG(0),
            ARG(2),
            ARG(4),
            ARG(5),
        ], 
        // AExp96
        vec![ // 200 
            COM(4,2),
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 201 
            COM(4,2),
            ARG(1),
            ARG(2),
        ], 
        // AExp97
        vec![ // 202 
            PTR(15, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 203 
            ARG(0),
            ARG(2),
        ], 
        vec![ // 204 
            COM(4,2),
            ARG(1),
        ], 
        // AExp98
        vec![ // 205 
            ARG(1),
            PTR(0, true, true),
            ARG(0),
        ], 
        vec![ // 206 
            COM(4,2),
            COM(2,0),
            COM(2,0),
        ], 
        // AExp99
        vec![ // 207 
            ARG(0),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 208 
            PTR(17, false, false),
            ARG(2),
        ], 
        // AExp100
        vec![ // 209 
            PTR(9, false, false),
            ARG(2),
            PTR(0, true, true),
        ], 
        vec![ // 210 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp101
        vec![ // 211 
            PTR(15, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 212 
            COM(4,2),
            ARG(0),
        ], 
    ],

}});