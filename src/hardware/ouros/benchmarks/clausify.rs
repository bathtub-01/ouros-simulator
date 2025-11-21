use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 102
#[rustfmt::skip]
pub static CLAUSIFY: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            COM(1,0),
            PTR(19, false, false),
            PTR(18, false, false),
        ], 
        vec![ // 1 
            COM(5,146),
            INT(0),
        ], 
        vec![ // 2 
            COM(5,146),
            INT(0),
        ], 
        vec![ // 3 
            PTR(70, false, false),
            PTR(2, false, false),
            PTR(1, false, false),
        ], 
        vec![ // 4 
            COM(5,146),
            INT(0),
        ], 
        vec![ // 5 
            PTR(70, false, false),
            PTR(4, false, false),
        ], 
        vec![ // 6 
            COM(1,0),
            PTR(5, false, false),
            PTR(3, false, false),
        ], 
        vec![ // 7 
            COM(5,146),
            INT(0),
        ], 
        vec![ // 8 
            COM(5,146),
            INT(0),
        ], 
        vec![ // 9 
            PTR(70, false, false),
            PTR(8, false, false),
            PTR(7, false, false),
        ], 
        vec![ // 10 
            COM(5,146),
            INT(0),
        ], 
        vec![ // 11 
            PTR(70, false, false),
            PTR(10, false, false),
        ], 
        vec![ // 12 
            COM(1,0),
            PTR(11, false, false),
            PTR(9, false, false),
        ], 
        vec![ // 13 
            PTR(70, false, false),
            PTR(12, false, false),
            PTR(6, false, false),
        ], 
        vec![ // 14 
            PTR(68, false, false),
            INT(2),
            PTR(13, false, false),
        ], 
        vec![ // 15 
            COM(5,146),
            INT(0),
        ], 
        vec![ // 16 
            PTR(22, false, false),
            COM(6,107),
            PTR(15, false, false),
        ], 
        vec![ // 17 
            COM(1,0),
            PTR(16, false, false),
            PTR(14, false, false),
        ], 
        vec![ // 18 
            COM(1,0),
            PTR(24, false, false),
            PTR(17, false, false),
        ], 
        // AExp1
        vec![ // 19 
            COM(2,2),
            COM(2,3),
        ], 
        // AExp2
        vec![ // 20 
            COM(2,6),
            COM(2,7),
        ], 
        // AExp3
        vec![ // 21 
            PTR(22, false, false),
            PRM(ADD,false),
            INT(0),
        ], 
        // AExp4
        vec![ // 22 
            COM(3,10),
            PTR(23, false, false),
        ], 
        vec![ // 23 
            COM(5,12),
            COM(4,14),
        ], 
        // AExp5
        vec![ // 24 
            COM(2,16),
            PTR(27, false, false),
        ], 
        vec![ // 25 
            COM(2,22),
            COM(1,24),
        ], 
        vec![ // 26 
            COM(2,20),
            PTR(25, false, false),
        ], 
        vec![ // 27 
            COM(2,18),
            PTR(26, false, false),
        ], 
        // AExp6
        vec![ // 28 
            COM(1,26),
            PTR(30, false, false),
        ], 
        vec![ // 29 
            PTR(31, false, false),
            PTR(39, false, false),
        ], 
        vec![ // 30 
            COM(3,27),
            PTR(29, false, false),
            COM(1,79),
        ], 
        // AExp7
        vec![ // 31 
            COM(4,29),
            COM(3,31),
            COM(2,33),
        ], 
        // AExp8
        vec![ // 32 
            COM(3,35),
            PTR(33, false, false),
        ], 
        vec![ // 33 
            COM(4,37),
            COM(3,39),
        ], 
        // AExp9
        vec![ // 34 
            COM(2,42),
            PTR(36, false, false),
        ], 
        vec![ // 35 
            COM(5,46),
            COM(3,49),
        ], 
        vec![ // 36 
            COM(4,44),
            PTR(35, false, false),
        ], 
        // AExp10
        vec![ // 37 
            COM(2,52),
            PTR(38, false, false),
        ], 
        vec![ // 38 
            COM(5,54),
            COM(5,56),
        ], 
        // AExp11
        vec![ // 39 
            COM(3,60),
            PTR(40, false, false),
        ], 
        vec![ // 40 
            COM(4,62),
            COM(4,64),
        ], 
        // AExp12
        vec![ // 41 
            COM(2,68),
            PTR(43, false, false),
        ], 
        vec![ // 42 
            COM(6,74),
            COM(6,76),
        ], 
        vec![ // 43 
            COM(6,70),
            COM(2,73),
            PTR(42, false, false),
        ], 
        // AExp13
        vec![ // 44 
            PTR(34, false, false),
            PTR(45, false, false),
        ], 
        // AExp14
        vec![ // 45 
            COM(2,6),
            COM(2,80),
        ], 
        // AExp15
        vec![ // 46 
            COM(2,82),
            COM(2,83),
        ], 
        // AExp16
        vec![ // 47 
            PTR(49, false, false),
            PTR(48, false, false),
        ], 
        vec![ // 48 
            PTR(51, false, false),
            COM(1,86),
        ], 
        // AExp17
        vec![ // 49 
            COM(2,87),
            PTR(50, false, false),
        ], 
        vec![ // 50 
            COM(4,89),
            COM(4,91),
        ], 
        // AExp18
        vec![ // 51 
            COM(4,94),
            PTR(54, false, false),
            COM(1,117),
        ], 
        vec![ // 52 
            COM(5,108),
            COM(6,111),
            COM(3,0),
            COM(2,0),
            COM(4,113),
        ], 
        vec![ // 53 
            COM(4,103),
            COM(2,105),
            COM(3,107),
        ], 
        vec![ // 54 
            COM(6,100),
            COM(3,0),
            PTR(53, false, false),
            PTR(52, false, false),
            COM(4,115),
        ], 
        // AExp19
        vec![ // 55 
            COM(2,118),
            PTR(57, false, false),
        ], 
        vec![ // 56 
            COM(6,123),
            COM(3,126),
            COM(3,128),
        ], 
        vec![ // 57 
            COM(4,120),
            PTR(56, false, false),
        ], 
        // AExp20
        vec![ // 58 
            PTR(59, false, false),
            COM(2,0),
        ], 
        // AExp21
        vec![ // 59 
            COM(2,130),
            PTR(60, false, false),
        ], 
        vec![ // 60 
            COM(7,132),
            COM(2,137),
            COM(3,139),
            COM(2,141),
            COM(2,143),
        ], 
        // AExp22
        vec![ // 61 
            COM(3,147),
            COM(2,148),
            COM(2,151),
        ], 
        // AExp23
        vec![ // 62 
            COM(6,154),
            COM(3,159),
            COM(3,162),
            COM(2,164),
            COM(2,166),
        ], 
        // AExp24
        vec![ // 63 
            COM(6,168),
            COM(3,173),
            COM(3,176),
            COM(2,178),
            COM(2,180),
        ], 
        // AExp25
        vec![ // 64 
            COM(4,182),
            COM(2,183),
            COM(2,186),
            PTR(67, false, false),
        ], 
        vec![ // 65 
            COM(3,195),
            COM(1,198),
        ], 
        vec![ // 66 
            COM(3,190),
            COM(1,193),
        ], 
        vec![ // 67 
            COM(4,189),
            PTR(66, false, false),
            PTR(65, false, false),
            COM(1,200),
        ], 
        // AExp26
        vec![ // 68 
            COM(3,202),
            PTR(69, false, false),
        ], 
        vec![ // 69 
            COM(3,204),
            COM(1,206),
        ], 
        // AExp27
        vec![ // 70 
            COM(4,208),
            COM(1,211),
            COM(2,213),
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
            ARG(1),
            INT(0),
            ARG(0),
        ], 
        // AExp3
        vec![ // 3 
            PRM(ADD,false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 4 
            PTR(19, false, false),
            ARG(1),
        ], 
        vec![ // 5 
            PTR(20, false, false),
            ARG(0),
        ], 
        // AExp4
        vec![ // 6 
            ARG(1),
            ARG(0),
        ], 
        // AExp5
        vec![ // 7 
            PRM(ADD,false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 8 
            PTR(21, false, false),
            ARG(1),
        ], 
        vec![ // 9 
            PTR(21, false, false),
            ARG(0),
        ], 
        // AExp6
        vec![ // 10 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 11 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp7
        vec![ // 12 
            ARG(4),
            ARG(2),
            PTR(0, true, true),
        ], 
        vec![ // 13 
            ARG(0),
            ARG(1),
            ARG(3),
        ], 
        // AExp8
        vec![ // 14 
            ARG(0),
            ARG(2),
            PTR(0, true, true),
        ], 
        vec![ // 15 
            ARG(1),
            ARG(3),
        ], 
        // AExp9
        vec![ // 16 
            COM(1,0),
            PTR(28, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 17 
            ARG(0),
            ARG(1),
        ], 
        // AExp10
        vec![ // 18 
            COM(1,0),
            PTR(44, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 19 
            ARG(0),
            ARG(1),
        ], 
        // AExp11
        vec![ // 20 
            COM(1,0),
            PTR(47, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 21 
            ARG(0),
            ARG(1),
        ], 
        // AExp12
        vec![ // 22 
            COM(1,0),
            PTR(58, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 23 
            ARG(0),
            ARG(1),
        ], 
        // AExp13
        vec![ // 24 
            COM(1,0),
            PTR(61, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 25 
            PTR(64, false, false),
            ARG(0),
        ], 
        // AExp14
        vec![ // 26 
            PTR(22, false, false),
            ARG(0),
            COM(2,0),
        ], 
        // AExp15
        vec![ // 27 
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 28 
            ARG(1),
            ARG(2),
        ], 
        // AExp16
        vec![ // 29 
            ARG(0),
            PTR(0, true, true),
            ARG(3),
        ], 
        vec![ // 30 
            ARG(1),
            ARG(2),
            ARG(3),
        ], 
        // AExp17
        vec![ // 31 
            PTR(32, false, false),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 32 
            PTR(34, false, false),
            ARG(0),
            ARG(2),
        ], 
        // AExp18
        vec![ // 33 
            COM(3,27),
            COM(1,51),
            PTR(0, true, true),
        ], 
        vec![ // 34 
            PTR(37, false, false),
            ARG(0),
            ARG(1),
        ], 
        // AExp19
        vec![ // 35 
            Y,
            PTR(0, true, true),
            ARG(1),
        ], 
        vec![ // 36 
            ARG(0),
            ARG(2),
        ], 
        // AExp20
        vec![ // 37 
            ARG(3),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 38 
            ARG(0),
            ARG(2),
        ], 
        // AExp21
        vec![ // 39 
            COM(4,41),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 40 
            ARG(0),
            ARG(2),
        ], 
        // AExp22
        vec![ // 41 
            ARG(3),
            ARG(0),
            ARG(1),
        ], 
        // AExp23
        vec![ // 42 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 43 
            ARG(0),
            ARG(1),
        ], 
        // AExp24
        vec![ // 44 
            ARG(3),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 45 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp25
        vec![ // 46 
            ARG(1),
            ARG(3),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 47 
            ARG(0),
            ARG(2),
            ARG(3),
            ARG(4),
        ], 
        vec![ // 48 
            ARG(2),
            ARG(4),
        ], 
        // AExp26
        vec![ // 49 
            COM(4,41),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 50 
            ARG(0),
            ARG(2),
        ], 
        // AExp27
        vec![ // 51 
            ARG(0),
            COM(2,1),
            COM(2,0),
        ], 
        // AExp28
        vec![ // 52 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 53 
            ARG(0),
            ARG(1),
        ], 
        // AExp29
        vec![ // 54 
            ARG(3),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 55 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(4),
        ], 
        // AExp30
        vec![ // 56 
            COM(2,59),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 57 
            ARG(1),
            ARG(4),
            ARG(2),
        ], 
        vec![ // 58 
            ARG(0),
            ARG(3),
            ARG(2),
        ], 
        // AExp31
        vec![ // 59 
            ARG(0),
            ARG(1),
            COM(2,1),
        ], 
        // AExp32
        vec![ // 60 
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 61 
            ARG(0),
            ARG(2),
        ], 
        // AExp33
        vec![ // 62 
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 63 
            ARG(0),
            ARG(2),
            ARG(3),
        ], 
        // AExp34
        vec![ // 64 
            COM(1,67),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 65 
            PTR(41, false, false),
            PRM(EQ,false),
            ARG(1),
            ARG(3),
        ], 
        vec![ // 66 
            PTR(41, false, false),
            PRM(EQ,false),
            ARG(0),
            ARG(2),
        ], 
        // AExp35
        vec![ // 67 
            ARG(0),
            COM(2,0),
        ], 
        // AExp36
        vec![ // 68 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 69 
            ARG(0),
            ARG(1),
        ], 
        // AExp37
        vec![ // 70 
            ARG(4),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 71 
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(5),
        ], 
        vec![ // 72 
            ARG(5),
            COM(2,1),
            ARG(0),
        ], 
        // AExp38
        vec![ // 73 
            COM(2,0),
        ], 
        // AExp39
        vec![ // 74 
            ARG(3),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 75 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(4),
            ARG(5),
        ], 
        // AExp40
        vec![ // 76 
            COM(1,67),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 77 
            ARG(1),
            ARG(3),
            ARG(5),
        ], 
        vec![ // 78 
            ARG(0),
            ARG(2),
            ARG(4),
        ], 
        // AExp41
        vec![ // 79 
            COM(4,41),
            ARG(0),
            COM(2,0),
        ], 
        // AExp42
        vec![ // 80 
            PTR(46, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 81 
            COM(2,84),
            PRM(EQ,false),
            ARG(0),
            ARG(1),
        ], 
        // AExp43
        vec![ // 82 
            ARG(1),
            COM(2,1),
            ARG(0),
        ], 
        // AExp44
        vec![ // 83 
            COM(2,0),
        ], 
        // AExp45
        vec![ // 84 
            PTR(34, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 85 
            PTR(37, false, false),
            ARG(0),
            ARG(1),
        ], 
        // AExp46
        vec![ // 86 
            ARG(0),
            COM(2,0),
            COM(2,0),
        ], 
        // AExp47
        vec![ // 87 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 88 
            ARG(0),
            ARG(1),
        ], 
        // AExp48
        vec![ // 89 
            ARG(3),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 90 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp49
        vec![ // 91 
            COM(4,41),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 92 
            ARG(1),
            ARG(3),
        ], 
        vec![ // 93 
            ARG(0),
            ARG(2),
        ], 
        // AExp50
        vec![ // 94 
            ARG(2),
            PTR(0, true, true),
        ], 
        vec![ // 95 
            ARG(0),
            ARG(3),
            ARG(1),
        ], 
        // AExp51
        vec![ // 96 
            ARG(0),
            ARG(1),
            PTR(2, true, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 97 
            ARG(4),
            ARG(5),
            ARG(6),
        ], 
        vec![ // 98 
            ARG(3),
            ARG(5),
            ARG(6),
        ], 
        vec![ // 99 
            ARG(2),
            ARG(5),
            ARG(6),
        ], 
        // AExp52
        vec![ // 100 
            COM(7,96),
            ARG(4),
            PTR(1, true, true),
            ARG(1),
            PTR(0, true, true),
            ARG(3),
        ], 
        vec![ // 101 
            ARG(2),
            ARG(5),
        ], 
        vec![ // 102 
            ARG(0),
            ARG(5),
        ], 
        // AExp53
        vec![ // 103 
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 104 
            ARG(1),
            ARG(2),
            ARG(3),
        ], 
        // AExp54
        vec![ // 105 
            PTR(51, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 106 
            PTR(51, false, false),
            ARG(0),
            ARG(1),
        ], 
        // AExp55
        vec![ // 107 
            ARG(2),
            ARG(0),
            ARG(1),
        ], 
        // AExp56
        vec![ // 108 
            ARG(0),
            PTR(1, true, true),
            PTR(0, true, true),
            ARG(3),
        ], 
        vec![ // 109 
            ARG(2),
            ARG(4),
        ], 
        vec![ // 110 
            ARG(1),
            ARG(4),
        ], 
        // AExp57
        vec![ // 111 
            ARG(5),
            ARG(0),
            ARG(0),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 112 
            ARG(2),
            ARG(3),
            ARG(4),
        ], 
        // AExp58
        vec![ // 113 
            ARG(3),
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 114 
            PTR(55, false, false),
            ARG(2),
            ARG(1),
        ], 
        // AExp59
        vec![ // 115 
            ARG(3),
            PTR(0, true, true),
            ARG(1),
        ], 
        vec![ // 116 
            PTR(55, false, false),
            ARG(2),
            ARG(0),
        ], 
        // AExp60
        vec![ // 117 
            ARG(0),
            COM(2,0),
            COM(2,0),
        ], 
        // AExp61
        vec![ // 118 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 119 
            ARG(0),
            ARG(1),
        ], 
        // AExp62
        vec![ // 120 
            ARG(3),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 121 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        vec![ // 122 
            COM(4,41),
            ARG(1),
            COM(2,0),
        ], 
        // AExp63
        vec![ // 123 
            PRM(LE,false),
            ARG(2),
            ARG(4),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 124 
            ARG(1),
            ARG(2),
            ARG(4),
            ARG(5),
        ], 
        vec![ // 125 
            ARG(0),
            ARG(3),
            ARG(4),
            ARG(5),
        ], 
        // AExp64
        vec![ // 126 
            COM(4,41),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 127 
            ARG(0),
            ARG(2),
        ], 
        // AExp65
        vec![ // 128 
            COM(4,41),
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 129 
            COM(4,41),
            ARG(1),
            ARG(2),
        ], 
        // AExp66
        vec![ // 130 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 131 
            ARG(0),
            ARG(1),
        ], 
        // AExp67
        vec![ // 132 
            ARG(6),
            PTR(3, true, true),
            PTR(2, true, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 133 
            ARG(3),
            ARG(4),
        ], 
        vec![ // 134 
            ARG(2),
            ARG(4),
        ], 
        vec![ // 135 
            ARG(1),
            ARG(4),
        ], 
        vec![ // 136 
            ARG(0),
            ARG(5),
        ], 
        // AExp68
        vec![ // 137 
            PTR(59, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 138 
            ARG(0),
            ARG(1),
        ], 
        // AExp69
        vec![ // 139 
            COM(4,41),
            PTR(0, true, true),
            ARG(0),
        ], 
        vec![ // 140 
            COM(6,41),
            ARG(1),
            ARG(2),
        ], 
        // AExp70
        vec![ // 141 
            COM(4,41),
            PTR(0, true, true),
            ARG(0),
        ], 
        vec![ // 142 
            COM(5,145),
            ARG(1),
        ], 
        // AExp71
        vec![ // 143 
            COM(4,41),
            PTR(0, true, true),
            ARG(0),
        ], 
        vec![ // 144 
            COM(5,146),
            ARG(1),
        ], 
        // AExp72
        vec![ // 145 
            ARG(3),
            ARG(0),
        ], 
        // AExp73
        vec![ // 146 
            ARG(4),
            ARG(0),
        ], 
        // AExp74
        vec![ // 147 
            ARG(2),
            ARG(0),
            ARG(1),
            COM(5,145),
            COM(5,146),
        ], 
        // AExp75
        vec![ // 148 
            COM(6,107),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 149 
            PTR(61, false, false),
            ARG(1),
        ], 
        vec![ // 150 
            PTR(61, false, false),
            ARG(0),
        ], 
        // AExp76
        vec![ // 151 
            PTR(62, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 152 
            PTR(61, false, false),
            ARG(1),
        ], 
        vec![ // 153 
            PTR(61, false, false),
            ARG(0),
        ], 
        // AExp77
        vec![ // 154 
            ARG(4),
            PTR(3, true, true),
            PTR(2, true, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 155 
            ARG(3),
            ARG(5),
        ], 
        vec![ // 156 
            ARG(2),
            ARG(5),
        ], 
        vec![ // 157 
            ARG(1),
            ARG(5),
        ], 
        vec![ // 158 
            ARG(0),
            ARG(5),
        ], 
        // AExp78
        vec![ // 159 
            COM(6,107),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 160 
            PTR(62, false, false),
            ARG(2),
            ARG(0),
        ], 
        vec![ // 161 
            PTR(62, false, false),
            ARG(1),
            ARG(0),
        ], 
        // AExp79
        vec![ // 162 
            PTR(63, false, false),
            PTR(0, true, true),
            ARG(0),
        ], 
        vec![ // 163 
            COM(6,41),
            ARG(1),
            ARG(2),
        ], 
        // AExp80
        vec![ // 164 
            PTR(63, false, false),
            PTR(0, true, true),
            ARG(0),
        ], 
        vec![ // 165 
            COM(5,145),
            ARG(1),
        ], 
        // AExp81
        vec![ // 166 
            PTR(63, false, false),
            PTR(0, true, true),
            ARG(0),
        ], 
        vec![ // 167 
            COM(5,146),
            ARG(1),
        ], 
        // AExp82
        vec![ // 168 
            ARG(5),
            PTR(3, true, true),
            PTR(2, true, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 169 
            ARG(3),
            ARG(4),
        ], 
        vec![ // 170 
            ARG(2),
            ARG(4),
        ], 
        vec![ // 171 
            ARG(1),
            ARG(4),
        ], 
        vec![ // 172 
            ARG(0),
            ARG(4),
        ], 
        // AExp83
        vec![ // 173 
            COM(6,107),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 174 
            PTR(62, false, false),
            ARG(0),
            ARG(2),
        ], 
        vec![ // 175 
            PTR(62, false, false),
            ARG(0),
            ARG(1),
        ], 
        // AExp84
        vec![ // 176 
            COM(6,41),
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 177 
            COM(6,41),
            ARG(1),
            ARG(2),
        ], 
        // AExp85
        vec![ // 178 
            COM(6,41),
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 179 
            COM(5,145),
            ARG(1),
        ], 
        // AExp86
        vec![ // 180 
            COM(6,41),
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 181 
            COM(5,146),
            ARG(1),
        ], 
        // AExp87
        vec![ // 182 
            ARG(3),
            ARG(0),
            ARG(1),
            ARG(2),
            COM(5,146),
        ], 
        // AExp88
        vec![ // 183 
            COM(6,107),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 184 
            PTR(64, false, false),
            ARG(1),
        ], 
        vec![ // 185 
            PTR(64, false, false),
            ARG(0),
        ], 
        // AExp89
        vec![ // 186 
            COM(6,41),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 187 
            PTR(64, false, false),
            ARG(1),
        ], 
        vec![ // 188 
            PTR(64, false, false),
            ARG(0),
        ], 
        // AExp90
        vec![ // 189 
            ARG(3),
            ARG(0),
            ARG(1),
            PTR(64, false, false),
            ARG(2),
        ], 
        // AExp91
        vec![ // 190 
            COM(6,41),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 191 
            ARG(0),
            ARG(2),
        ], 
        vec![ // 192 
            ARG(0),
            ARG(1),
        ], 
        // AExp92
        vec![ // 193 
            PTR(64, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 194 
            COM(5,145),
            ARG(0),
        ], 
        // AExp93
        vec![ // 195 
            COM(6,107),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 196 
            ARG(0),
            ARG(2),
        ], 
        vec![ // 197 
            ARG(0),
            ARG(1),
        ], 
        // AExp94
        vec![ // 198 
            PTR(64, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 199 
            COM(5,145),
            ARG(0),
        ], 
        // AExp95
        vec![ // 200 
            COM(5,145),
            PTR(0, true, true),
        ], 
        vec![ // 201 
            COM(5,146),
            ARG(0),
        ], 
        // AExp96
        vec![ // 202 
            PRM(LE,false),
            ARG(1),
            INT(0),
            PTR(0, true, true),
            COM(2,0),
        ], 
        vec![ // 203 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp97
        vec![ // 204 
            COM(4,41),
            ARG(2),
            PTR(0, true, true),
        ], 
        vec![ // 205 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp98
        vec![ // 206 
            PTR(68, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 207 
            PRM(SUB,false),
            ARG(0),
            INT(1),
        ], 
        // AExp99
        vec![ // 208 
            COM(6,107),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 209 
            ARG(1),
            ARG(2),
            ARG(3),
        ], 
        vec![ // 210 
            ARG(0),
            ARG(2),
            ARG(3),
        ], 
        // AExp100
        vec![ // 211 
            COM(6,41),
            PTR(0, true, true),
        ], 
        vec![ // 212 
            COM(5,145),
            ARG(0),
        ], 
        // AExp101
        vec![ // 213 
            COM(6,41),
            PTR(0, true, true),
            ARG(0),
        ], 
        vec![ // 214 
            COM(5,145),
            ARG(1),
        ], 
    ],

}});