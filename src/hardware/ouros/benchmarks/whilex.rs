use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 73
#[rustfmt::skip]
pub static WHILEX: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            PTR(53, false, false),
            PTR(52, false, false),
            INT(5),
            COM(1,0),
        ], 
        vec![ // 1 
            COM(4,2),
            COM(1,8),
            COM(2,0),
        ], 
        vec![ // 2 
            COM(4,2),
            COM(1,7),
            PTR(1, false, false),
        ], 
        vec![ // 3 
            COM(4,2),
            COM(1,6),
            PTR(2, false, false),
        ], 
        vec![ // 4 
            COM(4,2),
            COM(1,5),
            PTR(3, false, false),
        ], 
        vec![ // 5 
            COM(4,2),
            COM(1,4),
            PTR(4, false, false),
        ], 
        vec![ // 6 
            COM(4,2),
            COM(1,3),
            PTR(5, false, false),
        ], 
        vec![ // 7 
            COM(5,150),
            INT(1),
        ], 
        vec![ // 8 
            COM(5,145),
            INT(4),
        ], 
        vec![ // 9 
            COM(6,153),
            PTR(8, false, false),
            PTR(7, false, false),
        ], 
        vec![ // 10 
            COM(7,18),
            INT(4),
            PTR(9, false, false),
        ], 
        vec![ // 11 
            COM(5,150),
            INT(1),
        ], 
        vec![ // 12 
            COM(5,145),
            INT(5),
        ], 
        vec![ // 13 
            COM(6,18),
            PTR(12, false, false),
            PTR(11, false, false),
        ], 
        vec![ // 14 
            COM(7,18),
            INT(5),
            PTR(13, false, false),
        ], 
        vec![ // 15 
            COM(5,150),
            INT(0),
        ], 
        vec![ // 16 
            COM(5,145),
            INT(0),
        ], 
        vec![ // 17 
            PTR(56, false, false),
            PTR(16, false, false),
            PTR(15, false, false),
        ], 
        vec![ // 18 
            COM(4,142),
            PTR(17, false, false),
            PTR(14, false, false),
            COM(5,143),
        ], 
        vec![ // 19 
            COM(5,150),
            INT(1),
        ], 
        vec![ // 20 
            COM(5,145),
            INT(2),
        ], 
        vec![ // 21 
            COM(6,18),
            PTR(20, false, false),
            PTR(19, false, false),
        ], 
        vec![ // 22 
            COM(7,18),
            INT(2),
            PTR(21, false, false),
        ], 
        vec![ // 23 
            COM(5,145),
            INT(1),
        ], 
        vec![ // 24 
            COM(5,145),
            INT(0),
        ], 
        vec![ // 25 
            COM(6,153),
            PTR(24, false, false),
            PTR(23, false, false),
        ], 
        vec![ // 26 
            COM(7,18),
            INT(0),
            PTR(25, false, false),
        ], 
        vec![ // 27 
            COM(7,2),
            PTR(26, false, false),
            PTR(22, false, false),
        ], 
        vec![ // 28 
            COM(5,145),
            INT(0),
        ], 
        vec![ // 29 
            COM(5,145),
            INT(1),
        ], 
        vec![ // 30 
            PTR(57, false, false),
            PTR(29, false, false),
            PTR(28, false, false),
        ], 
        vec![ // 31 
            COM(7,144),
            PTR(30, false, false),
        ], 
        vec![ // 32 
            COM(1,0),
            PTR(31, false, false),
            PTR(27, false, false),
        ], 
        vec![ // 33 
            COM(5,145),
            INT(4),
        ], 
        vec![ // 34 
            COM(7,18),
            INT(1),
            PTR(33, false, false),
        ], 
        vec![ // 35 
            COM(5,145),
            INT(3),
        ], 
        vec![ // 36 
            COM(7,18),
            INT(0),
            PTR(35, false, false),
        ], 
        vec![ // 37 
            COM(7,2),
            PTR(36, false, false),
            PTR(34, false, false),
        ], 
        vec![ // 38 
            COM(7,2),
            PTR(37, false, false),
            PTR(32, false, false),
        ], 
        vec![ // 39 
            COM(7,2),
            PTR(38, false, false),
        ], 
        vec![ // 40 
            COM(1,0),
            PTR(39, false, false),
            PTR(18, false, false),
        ], 
        vec![ // 41 
            COM(7,2),
            PTR(40, false, false),
            PTR(10, false, false),
        ], 
        vec![ // 42 
            COM(5,150),
            INT(0),
        ], 
        vec![ // 43 
            COM(5,145),
            INT(4),
        ], 
        vec![ // 44 
            PTR(56, false, false),
            PTR(43, false, false),
            PTR(42, false, false),
        ], 
        vec![ // 45 
            PTR(55, false, false),
            PTR(44, false, false),
        ], 
        vec![ // 46 
            COM(7,144),
            PTR(45, false, false),
        ], 
        vec![ // 47 
            COM(1,0),
            PTR(46, false, false),
            PTR(41, false, false),
        ], 
        vec![ // 48 
            COM(5,145),
            INT(3),
        ], 
        vec![ // 49 
            COM(7,18),
            INT(4),
            PTR(48, false, false),
        ], 
        vec![ // 50 
            COM(7,2),
            PTR(49, false, false),
        ], 
        vec![ // 51 
            COM(1,0),
            PTR(50, false, false),
            PTR(47, false, false),
        ], 
        vec![ // 52 
            COM(2,16),
            PTR(51, false, false),
            PTR(6, false, false),
        ], 
        // AExp1
        vec![ // 53 
            COM(4,9),
            PTR(54, false, false),
        ], 
        vec![ // 54 
            COM(5,11),
            COM(5,13),
        ], 
        // AExp2
        vec![ // 55 
            COM(3,146),
            COM(2,42),
        ], 
        // AExp3
        vec![ // 56 
            COM(4,148),
            COM(3,18),
        ], 
        // AExp4
        vec![ // 57 
            COM(4,151),
            COM(3,18),
        ], 
        // AExp5
        vec![ // 58 
            COM(3,18),
            COM(1,0),
            COM(2,19),
        ], 
        // AExp6
        vec![ // 59 
            COM(6,21),
            COM(3,27),
            PTR(62, false, false),
            COM(4,34),
            PTR(61, false, false),
        ], 
        vec![ // 60 
            COM(3,38),
            COM(2,40),
        ], 
        vec![ // 61 
            COM(4,36),
            PTR(60, false, false),
        ], 
        vec![ // 62 
            COM(4,29),
            COM(2,32),
        ], 
        // AExp7
        vec![ // 63 
            COM(6,43),
            COM(4,47),
            COM(4,51),
            COM(3,55),
        ], 
        // AExp8
        vec![ // 64 
            COM(5,67),
            PTR(66, false, false),
        ], 
        vec![ // 65 
            COM(4,75),
            COM(6,78),
            COM(4,80),
        ], 
        vec![ // 66 
            COM(6,70),
            PTR(65, false, false),
        ], 
        // AExp9
        vec![ // 67 
            COM(5,82),
            COM(3,84),
            COM(3,18),
        ], 
        // AExp10
        vec![ // 68 
            COM(4,86),
            PTR(77, false, false),
        ], 
        vec![ // 69 
            COM(5,112),
            COM(2,115),
        ], 
        vec![ // 70 
            COM(4,96),
            PTR(69, false, false),
            COM(4,117),
        ], 
        vec![ // 71 
            COM(6,109),
            PTR(70, false, false),
        ], 
        vec![ // 72 
            COM(4,91),
            PTR(71, false, false),
            COM(4,121),
        ], 
        vec![ // 73 
            COM(5,98),
            COM(2,101),
        ], 
        vec![ // 74 
            COM(4,96),
            PTR(73, false, false),
            COM(3,103),
        ], 
        vec![ // 75 
            COM(6,93),
            PTR(74, false, false),
        ], 
        vec![ // 76 
            COM(4,91),
            PTR(75, false, false),
            COM(4,105),
        ], 
        vec![ // 77 
            COM(6,88),
            PTR(76, false, false),
            PTR(72, false, false),
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
            ARG(0),
            INT(0),
            INT(0),
        ], 
        // AExp4
        vec![ // 4 
            ARG(0),
            INT(1),
            INT(0),
        ], 
        // AExp5
        vec![ // 5 
            ARG(0),
            INT(2),
            INT(0),
        ], 
        // AExp6
        vec![ // 6 
            ARG(0),
            INT(3),
            INT(17),
        ], 
        // AExp7
        vec![ // 7 
            ARG(0),
            INT(4),
            INT(0),
        ], 
        // AExp8
        vec![ // 8 
            ARG(0),
            INT(5),
            INT(0),
        ], 
        // AExp9
        vec![ // 9 
            ARG(1),
            ERR(42),
            PTR(0, true, true),
        ], 
        vec![ // 10 
            ARG(0),
            ARG(2),
            ARG(3),
        ], 
        // AExp10
        vec![ // 11 
            ARG(3),
            PTR(0, true, true),
        ], 
        vec![ // 12 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(4),
        ], 
        // AExp11
        vec![ // 13 
            PRM(EQ,false),
            ARG(3),
            ARG(0),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 14 
            ARG(1),
            ARG(4),
        ], 
        vec![ // 15 
            PTR(53, false, false),
            ARG(2),
            ARG(0),
            ARG(1),
        ], 
        // AExp12
        vec![ // 16 
            PTR(58, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 17 
            COM(4,2),
            ARG(0),
            ARG(1),
        ], 
        // AExp13
        vec![ // 18 
            ARG(2),
            ARG(0),
            ARG(1),
        ], 
        // AExp14
        vec![ // 19 
            PTR(58, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 20 
            PTR(59, false, false),
            ARG(0),
            ARG(1),
        ], 
        // AExp15
        vec![ // 21 
            ARG(4),
            PTR(4, true, true),
            PTR(3, true, true),
            PTR(2, true, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 22 
            ARG(3),
            ARG(5),
        ], 
        vec![ // 23 
            COM(3,42),
            ARG(5),
        ], 
        vec![ // 24 
            ARG(2),
            ARG(5),
        ], 
        vec![ // 25 
            ARG(1),
            ARG(5),
        ], 
        vec![ // 26 
            ARG(0),
            ARG(5),
        ], 
        // AExp16
        vec![ // 27 
            PTR(63, false, false),
            ARG(2),
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 28 
            PTR(64, false, false),
            ARG(0),
            ARG(1),
            COM(3,42),
        ], 
        // AExp17
        vec![ // 29 
            PTR(59, false, false),
            ARG(2),
            ARG(1),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 30 
            ARG(0),
            ARG(3),
        ], 
        vec![ // 31 
            COM(4,2),
            ARG(3),
        ], 
        // AExp18
        vec![ // 32 
            COM(4,2),
            PTR(0, true, true),
        ], 
        vec![ // 33 
            COM(7,2),
            ARG(1),
            ARG(0),
        ], 
        // AExp19
        vec![ // 34 
            PTR(68, false, false),
            ARG(1),
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 35 
            COM(4,138),
            ARG(0),
            ARG(2),
            ARG(3),
        ], 
        // AExp20
        vec![ // 36 
            COM(4,2),
            PTR(0, true, true),
            ARG(1),
        ], 
        vec![ // 37 
            ARG(0),
            ARG(2),
            ARG(3),
        ], 
        // AExp21
        vec![ // 38 
            COM(4,142),
            ARG(1),
            PTR(0, true, true),
            COM(5,143),
        ], 
        vec![ // 39 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp22
        vec![ // 40 
            COM(7,2),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 41 
            COM(7,144),
            ARG(0),
            ARG(1),
        ], 
        // AExp23
        vec![ // 42 
            ARG(1),
            ARG(0),
        ], 
        // AExp24
        vec![ // 43 
            ARG(3),
            PTR(2, true, true),
            ARG(5),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 44 
            ARG(2),
            ARG(4),
            ARG(5),
        ], 
        vec![ // 45 
            ARG(1),
            ARG(4),
            ARG(5),
        ], 
        vec![ // 46 
            ARG(0),
            ARG(4),
            ARG(5),
        ], 
        // AExp25
        vec![ // 47 
            COM(3,56),
            PTR(2, true, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 48 
            COM(3,60),
            ARG(1),
        ], 
        vec![ // 49 
            PTR(63, false, false),
            ARG(3),
            ARG(0),
        ], 
        vec![ // 50 
            PTR(63, false, false),
            ARG(2),
            ARG(0),
        ], 
        // AExp26
        vec![ // 51 
            COM(3,56),
            PTR(2, true, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 52 
            COM(3,65),
            ARG(1),
        ], 
        vec![ // 53 
            PTR(63, false, false),
            ARG(3),
            ARG(0),
        ], 
        vec![ // 54 
            PTR(63, false, false),
            ARG(2),
            ARG(0),
        ], 
        // AExp27
        vec![ // 55 
            PTR(53, false, false),
            ARG(0),
            ARG(2),
            ARG(1),
        ], 
        // AExp28
        vec![ // 56 
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 57 
            COM(3,58),
            ARG(1),
            ARG(2),
        ], 
        // AExp29
        vec![ // 58 
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 59 
            ARG(1),
            ARG(2),
        ], 
        // AExp30
        vec![ // 60 
            COM(2,62),
            PTR(0, true, true),
            ARG(0),
        ], 
        vec![ // 61 
            PRM(ADD,false),
            ARG(1),
            ARG(2),
        ], 
        // AExp31
        vec![ // 62 
            PRM(EQ,false),
            ARG(0),
            INT(0),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 63 
            ARG(1),
            INT(0),
        ], 
        vec![ // 64 
            ARG(1),
            ARG(0),
        ], 
        // AExp32
        vec![ // 65 
            COM(2,62),
            PTR(0, true, true),
            ARG(0),
        ], 
        vec![ // 66 
            PRM(SUB,false),
            ARG(1),
            ARG(2),
        ], 
        // AExp33
        vec![ // 67 
            ARG(1),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 68 
            ARG(0),
            ARG(2),
            ARG(3),
            ARG(4),
        ], 
        vec![ // 69 
            ARG(3),
            COM(2,0),
        ], 
        // AExp34
        vec![ // 70 
            ARG(4),
            PTR(0, true, true),
        ], 
        vec![ // 71 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(5),
        ], 
        // AExp35
        vec![ // 72 
            PRM(EQ,false),
            ARG(5),
            ARG(0),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 73 
            ARG(2),
            ARG(3),
            ARG(4),
        ], 
        vec![ // 74 
            ARG(1),
            ARG(3),
            ARG(4),
            ARG(5),
            ARG(6),
        ], 
        // AExp36
        vec![ // 75 
            COM(7,72),
            ARG(2),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 76 
            ARG(1),
            ARG(2),
            ARG(3),
        ], 
        vec![ // 77 
            ARG(0),
            ARG(2),
            ARG(3),
        ], 
        // AExp37
        vec![ // 78 
            PTR(64, false, false),
            ARG(3),
            ARG(0),
            PTR(0, true, true),
            ARG(2),
        ], 
        vec![ // 79 
            PTR(67, false, false),
            ARG(1),
            ARG(4),
            ARG(5),
        ], 
        // AExp38
        vec![ // 80 
            PTR(64, false, false),
            ARG(3),
            ARG(0),
            PTR(0, true, true),
            ARG(2),
        ], 
        vec![ // 81 
            PTR(67, false, false),
            ARG(1),
            ARG(0),
            ARG(2),
        ], 
        // AExp39
        vec![ // 82 
            ARG(0),
            ARG(2),
            PTR(0, true, true),
        ], 
        vec![ // 83 
            ARG(1),
            ARG(3),
            ARG(4),
        ], 
        // AExp40
        vec![ // 84 
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 85 
            COM(4,2),
            ARG(1),
            ARG(2),
        ], 
        // AExp41
        vec![ // 86 
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 87 
            ARG(0),
            ARG(2),
            ARG(3),
        ], 
        // AExp42
        vec![ // 88 
            PRM(LT,false),
            ARG(4),
            INT(3),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 89 
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(4),
            ARG(5),
        ], 
        vec![ // 90 
            ARG(0),
            ARG(2),
            ARG(3),
            ARG(4),
            ARG(5),
        ], 
        // AExp43
        vec![ // 91 
            ARG(0),
            ARG(2),
            ARG(3),
            PTR(0, true, true),
        ], 
        vec![ // 92 
            ARG(1),
            ARG(2),
            ARG(3),
        ], 
        // AExp44
        vec![ // 93 
            PRM(LT,false),
            ARG(4),
            INT(4),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 94 
            ARG(5),
            ARG(3),
        ], 
        vec![ // 95 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(4),
            ARG(5),
        ], 
        // AExp45
        vec![ // 96 
            ARG(0),
            ARG(3),
            PTR(0, true, true),
        ], 
        vec![ // 97 
            ARG(1),
            ARG(2),
            ARG(3),
        ], 
        // AExp46
        vec![ // 98 
            PRM(LT,false),
            ARG(3),
            INT(5),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 99 
            ARG(4),
            ARG(2),
        ], 
        vec![ // 100 
            ARG(0),
            ARG(1),
            ARG(4),
        ], 
        // AExp47
        vec![ // 101 
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 102 
            ARG(0),
            COM(2,1),
        ], 
        // AExp48
        vec![ // 103 
            PTR(68, false, false),
            ARG(2),
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 104 
            COM(2,125),
            ARG(1),
        ], 
        // AExp49
        vec![ // 105 
            COM(3,56),
            PTR(2, true, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 106 
            COM(3,131),
            ARG(1),
        ], 
        vec![ // 107 
            PTR(63, false, false),
            ARG(3),
            ARG(0),
        ], 
        vec![ // 108 
            PTR(63, false, false),
            ARG(2),
            ARG(0),
        ], 
        // AExp50
        vec![ // 109 
            PRM(LT,false),
            ARG(4),
            INT(1),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 110 
            ARG(5),
            ARG(3),
        ], 
        vec![ // 111 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(4),
            ARG(5),
        ], 
        // AExp51
        vec![ // 112 
            PRM(LT,false),
            ARG(3),
            INT(2),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 113 
            ARG(4),
            ARG(2),
        ], 
        vec![ // 114 
            ARG(0),
            ARG(1),
            ARG(4),
        ], 
        // AExp52
        vec![ // 115 
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 116 
            ARG(0),
            COM(2,0),
        ], 
        // AExp53
        vec![ // 117 
            COM(3,56),
            PTR(2, true, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 118 
            COM(3,133),
            ARG(1),
        ], 
        vec![ // 119 
            PTR(63, false, false),
            ARG(3),
            ARG(0),
        ], 
        vec![ // 120 
            PTR(63, false, false),
            ARG(2),
            ARG(0),
        ], 
        // AExp54
        vec![ // 121 
            COM(3,56),
            PTR(2, true, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 122 
            COM(3,135),
            ARG(1),
        ], 
        vec![ // 123 
            PTR(68, false, false),
            ARG(3),
            ARG(0),
        ], 
        vec![ // 124 
            PTR(68, false, false),
            ARG(2),
            ARG(0),
        ], 
        // AExp55
        vec![ // 125 
            COM(2,127),
            PTR(0, true, true),
            ARG(0),
        ], 
        vec![ // 126 
            COM(1,130),
            ARG(1),
        ], 
        // AExp56
        vec![ // 127 
            ARG(0),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 128 
            ARG(1),
            COM(2,1),
        ], 
        vec![ // 129 
            ARG(1),
            COM(2,0),
        ], 
        // AExp57
        vec![ // 130 
            ARG(0),
            COM(2,1),
            COM(2,0),
        ], 
        // AExp58
        vec![ // 131 
            COM(2,127),
            PTR(0, true, true),
            ARG(0),
        ], 
        vec![ // 132 
            PRM(LE,false),
            ARG(1),
            ARG(2),
        ], 
        // AExp59
        vec![ // 133 
            COM(2,127),
            PTR(0, true, true),
            ARG(0),
        ], 
        vec![ // 134 
            PRM(EQ,false),
            ARG(1),
            ARG(2),
        ], 
        // AExp60
        vec![ // 135 
            COM(2,127),
            PTR(0, true, true),
            ARG(0),
        ], 
        vec![ // 136 
            COM(1,137),
            ARG(1),
            ARG(2),
        ], 
        // AExp61
        vec![ // 137 
            ARG(0),
            COM(2,0),
        ], 
        // AExp62
        vec![ // 138 
            ARG(3),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 139 
            COM(4,2),
            ARG(1),
            ARG(0),
        ], 
        vec![ // 140 
            COM(4,2),
            ARG(2),
            ARG(0),
        ], 
        // AExp63
        vec![ // 141 
            ARG(4),
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp64
        vec![ // 142 
            COM(7,141),
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp65
        vec![ // 143 
            ARG(3),
        ], 
        // AExp66
        vec![ // 144 
            ARG(6),
            ARG(0),
            ARG(1),
        ], 
        // AExp67
        vec![ // 145 
            ARG(4),
            ARG(0),
        ], 
        // AExp68
        vec![ // 146 
            ARG(2),
            INT(4),
            PTR(0, true, true),
        ], 
        vec![ // 147 
            ARG(0),
            ARG(1),
        ], 
        // AExp69
        vec![ // 148 
            ARG(3),
            INT(1),
            PTR(0, true, true),
        ], 
        vec![ // 149 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp70
        vec![ // 150 
            ARG(2),
            ARG(0),
        ], 
        // AExp71
        vec![ // 151 
            ARG(3),
            INT(3),
            PTR(0, true, true),
        ], 
        vec![ // 152 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp72
        vec![ // 153 
            ARG(4),
            ARG(0),
            ARG(1),
        ], 
    ],

}});