use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 108
#[rustfmt::skip]
pub static SKIABSEVAL: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            PTR(6, false, false),
            PTR(5, false, false),
        ], 
        vec![ // 1 
            PTR(63, false, false),
            INT(5),
        ], 
        vec![ // 2 
            PTR(63, false, false),
            INT(1),
        ], 
        vec![ // 3 
            PTR(71, false, false),
            PTR(101, false, false),
        ], 
        vec![ // 4 
            PTR(69, false, false),
            PTR(3, false, false),
            PTR(2, false, false),
        ], 
        vec![ // 5 
            PTR(69, false, false),
            PTR(4, false, false),
            PTR(1, false, false),
        ], 
        // AExp1
        vec![ // 6 
            COM(2,2),
            PTR(7, false, false),
        ], 
        vec![ // 7 
            COM(5,4),
            COM(1,7),
            COM(1,0),
        ], 
        // AExp2
        vec![ // 8 
            COM(3,9),
            COM(1,11),
            PTR(9, false, false),
        ], 
        vec![ // 9 
            COM(3,9),
            PTR(10, false, false),
            PTR(66, false, false),
        ], 
        // AExp3
        vec![ // 10 
            COM(2,23),
            PTR(58, false, false),
        ], 
        vec![ // 11 
            COM(4,129),
            COM(1,131),
        ], 
        vec![ // 12 
            COM(4,127),
            PTR(11, false, false),
        ], 
        vec![ // 13 
            COM(5,124),
            PTR(12, false, false),
        ], 
        vec![ // 14 
            COM(5,122),
            PTR(13, false, false),
        ], 
        vec![ // 15 
            COM(4,120),
            PTR(14, false, false),
        ], 
        vec![ // 16 
            COM(5,79),
            PTR(15, false, false),
        ], 
        vec![ // 17 
            COM(4,77),
            PTR(16, false, false),
        ], 
        vec![ // 18 
            COM(3,39),
            COM(4,41),
            PTR(17, false, false),
        ], 
        vec![ // 19 
            COM(5,118),
            PTR(18, false, false),
        ], 
        vec![ // 20 
            COM(4,114),
            COM(1,116),
        ], 
        vec![ // 21 
            COM(4,43),
            PTR(20, false, false),
        ], 
        vec![ // 22 
            COM(3,39),
            COM(4,41),
            PTR(21, false, false),
        ], 
        vec![ // 23 
            COM(6,111),
            PTR(22, false, false),
            PTR(19, false, false),
        ], 
        vec![ // 24 
            COM(3,107),
            COM(1,109),
        ], 
        vec![ // 25 
            COM(4,41),
            PTR(24, false, false),
        ], 
        vec![ // 26 
            COM(5,99),
            COM(3,101),
            COM(2,104),
        ], 
        vec![ // 27 
            COM(5,97),
            PTR(26, false, false),
        ], 
        vec![ // 28 
            COM(5,79),
            PTR(27, false, false),
        ], 
        vec![ // 29 
            COM(4,77),
            PTR(28, false, false),
        ], 
        vec![ // 30 
            COM(3,39),
            COM(4,41),
            PTR(29, false, false),
        ], 
        vec![ // 31 
            COM(6,94),
            PTR(30, false, false),
            PTR(25, false, false),
        ], 
        vec![ // 32 
            COM(6,91),
            PTR(31, false, false),
            PTR(23, false, false),
        ], 
        vec![ // 33 
            COM(4,86),
            COM(1,89),
        ], 
        vec![ // 34 
            COM(5,83),
            PTR(33, false, false),
        ], 
        vec![ // 35 
            COM(5,81),
            PTR(34, false, false),
        ], 
        vec![ // 36 
            COM(5,79),
            PTR(35, false, false),
        ], 
        vec![ // 37 
            COM(4,77),
            PTR(36, false, false),
        ], 
        vec![ // 38 
            COM(3,39),
            COM(4,41),
            PTR(37, false, false),
        ], 
        vec![ // 39 
            COM(3,73),
            COM(1,75),
        ], 
        vec![ // 40 
            COM(4,72),
            PTR(39, false, false),
        ], 
        vec![ // 41 
            COM(3,39),
            COM(4,41),
            PTR(40, false, false),
        ], 
        vec![ // 42 
            COM(6,69),
            PTR(41, false, false),
            PTR(38, false, false),
        ], 
        vec![ // 43 
            COM(3,64),
            COM(1,67),
        ], 
        vec![ // 44 
            COM(4,41),
            PTR(43, false, false),
        ], 
        vec![ // 45 
            COM(5,62),
            PTR(44, false, false),
        ], 
        vec![ // 46 
            COM(4,47),
            COM(2,58),
            COM(1,60),
        ], 
        vec![ // 47 
            COM(4,54),
            COM(2,56),
            PTR(46, false, false),
        ], 
        vec![ // 48 
            COM(4,43),
            PTR(47, false, false),
        ], 
        vec![ // 49 
            COM(3,39),
            COM(4,41),
            PTR(48, false, false),
        ], 
        vec![ // 50 
            COM(4,47),
            COM(2,50),
            COM(1,52),
        ], 
        vec![ // 51 
            COM(4,45),
            PTR(50, false, false),
        ], 
        vec![ // 52 
            COM(4,43),
            PTR(51, false, false),
        ], 
        vec![ // 53 
            COM(3,39),
            COM(4,41),
            PTR(52, false, false),
        ], 
        vec![ // 54 
            COM(6,36),
            PTR(53, false, false),
            PTR(49, false, false),
        ], 
        vec![ // 55 
            COM(6,33),
            PTR(54, false, false),
            PTR(45, false, false),
        ], 
        vec![ // 56 
            COM(6,30),
            PTR(55, false, false),
            PTR(42, false, false),
        ], 
        vec![ // 57 
            COM(6,27),
            PTR(56, false, false),
            PTR(32, false, false),
        ], 
        vec![ // 58 
            COM(4,25),
            PTR(57, false, false),
        ], 
        // AExp4
        vec![ // 59 
            COM(2,134),
            COM(1,0),
        ], 
        // AExp5
        vec![ // 60 
            COM(2,135),
            COM(1,0),
        ], 
        // AExp6
        vec![ // 61 
            COM(2,136),
            PTR(62, false, false),
        ], 
        vec![ // 62 
            COM(3,137),
            COM(1,0),
        ], 
        // AExp7
        vec![ // 63 
            COM(3,139),
            COM(2,136),
        ], 
        // AExp8
        vec![ // 64 
            COM(3,141),
            PTR(65, false, false),
        ], 
        vec![ // 65 
            COM(4,43),
            COM(3,143),
        ], 
        // AExp9
        vec![ // 66 
            COM(2,145),
            PTR(68, false, false),
        ], 
        vec![ // 67 
            COM(5,148),
            COM(5,150),
            COM(4,153),
        ], 
        vec![ // 68 
            COM(4,146),
            PTR(67, false, false),
        ], 
        // AExp10
        vec![ // 69 
            COM(4,20),
            COM(3,22),
        ], 
        // AExp11
        vec![ // 70 
            COM(2,155),
            COM(1,0),
        ], 
        // AExp12
        vec![ // 71 
            COM(2,2),
            PTR(74, false, false),
        ], 
        vec![ // 72 
            COM(4,164),
            COM(2,166),
        ], 
        vec![ // 73 
            COM(4,159),
            COM(2,161),
        ], 
        vec![ // 74 
            COM(5,156),
            PTR(73, false, false),
            PTR(72, false, false),
        ], 
        // AExp13
        vec![ // 75 
            COM(2,168),
            PTR(80, false, false),
        ], 
        vec![ // 76 
            COM(4,182),
            COM(4,184),
            COM(3,186),
        ], 
        vec![ // 77 
            COM(3,9),
            COM(4,177),
            COM(3,179),
        ], 
        vec![ // 78 
            COM(7,174),
            PTR(77, false, false),
            PTR(76, false, false),
        ], 
        vec![ // 79 
            COM(5,172),
            PTR(78, false, false),
        ], 
        vec![ // 80 
            COM(4,170),
            PTR(79, false, false),
        ], 
        // AExp14
        vec![ // 81 
            COM(1,0),
            PTR(96, false, false),
        ], 
        vec![ // 82 
            COM(5,191),
            COM(4,205),
            COM(1,207),
        ], 
        vec![ // 83 
            COM(5,172),
            PTR(82, false, false),
        ], 
        vec![ // 84 
            COM(4,182),
            COM(4,203),
            PTR(83, false, false),
        ], 
        vec![ // 85 
            COM(4,25),
            PTR(84, false, false),
        ], 
        vec![ // 86 
            COM(4,187),
            PTR(85, false, false),
            COM(1,209),
        ], 
        vec![ // 87 
            COM(5,191),
            COM(4,197),
            COM(2,199),
        ], 
        vec![ // 88 
            COM(5,172),
            PTR(87, false, false),
        ], 
        vec![ // 89 
            COM(4,182),
            COM(4,195),
            PTR(88, false, false),
        ], 
        vec![ // 90 
            COM(4,18),
            PTR(89, false, false),
        ], 
        vec![ // 91 
            COM(4,187),
            PTR(90, false, false),
            COM(2,201),
        ], 
        vec![ // 92 
            COM(5,191),
            COM(4,193),
            PTR(91, false, false),
        ], 
        vec![ // 93 
            COM(5,172),
            PTR(92, false, false),
        ], 
        vec![ // 94 
            COM(4,182),
            COM(4,189),
            PTR(93, false, false),
        ], 
        vec![ // 95 
            COM(4,18),
            PTR(94, false, false),
        ], 
        vec![ // 96 
            COM(4,187),
            PTR(95, false, false),
            PTR(86, false, false),
        ], 
        // AExp15
        vec![ // 97 
            COM(2,211),
            COM(1,0),
        ], 
        // AExp16
        vec![ // 98 
            COM(2,212),
            COM(1,0),
        ], 
        // AExp17
        vec![ // 99 
            COM(2,213),
            COM(1,0),
        ], 
        // AExp18
        vec![ // 100 
            COM(2,13),
            COM(1,0),
        ], 
        // AExp19
        vec![ // 101 
            PTR(69, false, false),
            PTR(70, false, false),
            PTR(122, false, false),
        ], 
        vec![ // 102 
            PTR(125, false, false),
            INT(2),
        ], 
        vec![ // 103 
            PTR(63, false, false),
            INT(1),
        ], 
        vec![ // 104 
            PTR(125, false, false),
            INT(1),
        ], 
        vec![ // 105 
            PTR(69, false, false),
            PTR(126, false, false),
            PTR(104, false, false),
        ], 
        vec![ // 106 
            PTR(69, false, false),
            PTR(105, false, false),
            PTR(103, false, false),
        ], 
        vec![ // 107 
            PTR(125, false, false),
            INT(0),
        ], 
        vec![ // 108 
            PTR(69, false, false),
            PTR(107, false, false),
            PTR(106, false, false),
        ], 
        vec![ // 109 
            PTR(69, false, false),
            PTR(108, false, false),
            PTR(102, false, false),
        ], 
        vec![ // 110 
            PTR(125, false, false),
            INT(1),
        ], 
        vec![ // 111 
            PTR(69, false, false),
            PTR(126, false, false),
            PTR(110, false, false),
        ], 
        vec![ // 112 
            PTR(69, false, false),
            PTR(111, false, false),
            PTR(109, false, false),
        ], 
        vec![ // 113 
            PTR(125, false, false),
            INT(1),
        ], 
        vec![ // 114 
            PTR(125, false, false),
            INT(2),
        ], 
        vec![ // 115 
            PTR(125, false, false),
            INT(1),
        ], 
        vec![ // 116 
            PTR(69, false, false),
            PTR(124, false, false),
            PTR(115, false, false),
        ], 
        vec![ // 117 
            PTR(69, false, false),
            PTR(116, false, false),
            PTR(114, false, false),
        ], 
        vec![ // 118 
            PTR(69, false, false),
            PTR(117, false, false),
            PTR(113, false, false),
        ], 
        vec![ // 119 
            PTR(69, false, false),
            PTR(118, false, false),
            PTR(112, false, false),
        ], 
        vec![ // 120 
            PTR(123, false, false),
            INT(2),
            PTR(119, false, false),
        ], 
        vec![ // 121 
            PTR(123, false, false),
            INT(1),
            PTR(120, false, false),
        ], 
        vec![ // 122 
            PTR(123, false, false),
            INT(0),
            PTR(121, false, false),
        ], 
        // AExp20
        vec![ // 123 
            COM(4,214),
            COM(3,22),
        ], 
        // AExp21
        vec![ // 124 
            COM(2,216),
            COM(1,0),
        ], 
        // AExp22
        vec![ // 125 
            COM(3,217),
            COM(2,136),
        ], 
        // AExp23
        vec![ // 126 
            COM(2,219),
            COM(1,0),
        ], 
        // AExp24
        vec![ // 127 
            COM(2,14),
            PTR(128, false, false),
        ], 
        vec![ // 128 
            COM(5,16),
            COM(4,18),
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
            PTR(0, true, true),
        ], 
        vec![ // 3 
            ARG(0),
            ARG(1),
        ], 
        // AExp3
        vec![ // 4 
            PRM(EQ,false),
            ARG(3),
            INT(10),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 5 
            ARG(4),
            ARG(1),
        ], 
        vec![ // 6 
            ARG(0),
            ARG(2),
        ], 
        // AExp4
        vec![ // 7 
            PTR(6, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 8 
            PTR(8, false, false),
            ARG(0),
        ], 
        // AExp5
        vec![ // 9 
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 10 
            ARG(1),
            ARG(2),
        ], 
        // AExp6
        vec![ // 11 
            ARG(0),
            PTR(100, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 12 
            PTR(127, false, false),
            PTR(69, false, false),
        ], 
        // AExp7
        vec![ // 13 
            ARG(1),
            INT(5),
            ARG(0),
        ], 
        // AExp8
        vec![ // 14 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 15 
            ARG(0),
            ARG(1),
        ], 
        // AExp9
        vec![ // 16 
            ARG(4),
            ARG(3),
            PTR(0, true, true),
        ], 
        vec![ // 17 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(3),
        ], 
        // AExp10
        vec![ // 18 
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 19 
            ARG(0),
            ARG(2),
            ARG(3),
        ], 
        // AExp11
        vec![ // 20 
            ARG(3),
            INT(2),
            PTR(0, true, true),
        ], 
        vec![ // 21 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp12
        vec![ // 22 
            ARG(2),
            ARG(0),
            ARG(1),
        ], 
        // AExp13
        vec![ // 23 
            ARG(1),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 24 
            ARG(0),
            ARG(1),
        ], 
        // AExp14
        vec![ // 25 
            ARG(2),
            PTR(0, true, true),
        ], 
        vec![ // 26 
            ARG(0),
            ARG(1),
            ARG(3),
        ], 
        // AExp15
        vec![ // 27 
            PRM(LT,false),
            ARG(4),
            INT(7),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 28 
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(4),
            ARG(5),
        ], 
        vec![ // 29 
            ARG(0),
            ARG(2),
            ARG(3),
            ARG(4),
            ARG(5),
        ], 
        // AExp16
        vec![ // 30 
            PRM(LT,false),
            ARG(4),
            INT(9),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 31 
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(4),
            ARG(5),
        ], 
        vec![ // 32 
            ARG(0),
            ARG(2),
            ARG(3),
            ARG(4),
            ARG(5),
        ], 
        // AExp17
        vec![ // 33 
            PRM(LT,false),
            ARG(4),
            INT(11),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 34 
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(4),
            ARG(5),
        ], 
        vec![ // 35 
            ARG(0),
            ARG(2),
            ARG(3),
            ARG(4),
            ARG(5),
        ], 
        // AExp18
        vec![ // 36 
            PRM(LT,false),
            ARG(4),
            INT(12),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 37 
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(5),
        ], 
        vec![ // 38 
            ARG(0),
            ARG(2),
            ARG(3),
            ARG(5),
        ], 
        // AExp19
        vec![ // 39 
            ARG(0),
            PTR(0, true, true),
            ARG(2),
        ], 
        vec![ // 40 
            ARG(1),
            ARG(2),
        ], 
        // AExp20
        vec![ // 41 
            ARG(3),
            PTR(0, true, true),
        ], 
        vec![ // 42 
            ARG(2),
            ARG(1),
            ARG(0),
        ], 
        // AExp21
        vec![ // 43 
            ARG(3),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 44 
            ARG(0),
            ARG(2),
        ], 
        // AExp22
        vec![ // 45 
            PTR(10, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 46 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(3),
        ], 
        // AExp23
        vec![ // 47 
            ARG(0),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 48 
            ARG(1),
            ARG(3),
        ], 
        vec![ // 49 
            ARG(1),
            ARG(2),
        ], 
        // AExp24
        vec![ // 50 
            COM(4,133),
            PTR(0, true, true),
        ], 
        vec![ // 51 
            PRM(EQ,false),
            ARG(0),
            ARG(1),
            PTR(59, false, false),
            PTR(60, false, false),
        ], 
        // AExp25
        vec![ // 52 
            PTR(61, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 53 
            PTR(8, false, false),
            ARG(0),
        ], 
        // AExp26
        vec![ // 54 
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 55 
            ARG(1),
            ARG(2),
            ARG(3),
        ], 
        // AExp27
        vec![ // 56 
            PTR(10, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 57 
            COM(4,133),
            ARG(0),
            ARG(1),
        ], 
        // AExp28
        vec![ // 58 
            PTR(63, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 59 
            PRM(ADD,false),
            ARG(0),
            ARG(1),
        ], 
        // AExp29
        vec![ // 60 
            PTR(61, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 61 
            PTR(8, false, false),
            ARG(0),
        ], 
        // AExp30
        vec![ // 62 
            PRM(EQ,false),
            ARG(3),
            INT(9),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 63 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(4),
        ], 
        // AExp31
        vec![ // 64 
            PTR(64, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 65 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        vec![ // 66 
            PTR(66, false, false),
            ARG(1),
        ], 
        // AExp32
        vec![ // 67 
            COM(4,133),
            PTR(0, true, true),
        ], 
        vec![ // 68 
            PTR(69, false, false),
            PTR(70, false, false),
            ARG(0),
        ], 
        // AExp33
        vec![ // 69 
            PRM(LT,false),
            ARG(4),
            INT(8),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 70 
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(5),
        ], 
        vec![ // 71 
            ARG(0),
            ARG(2),
            ARG(3),
            ARG(5),
        ], 
        // AExp34
        vec![ // 72 
            ARG(3),
            ARG(1),
            ARG(0),
        ], 
        // AExp35
        vec![ // 73 
            PTR(10, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 74 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp36
        vec![ // 75 
            PTR(64, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 76 
            PTR(66, false, false),
            ARG(0),
        ], 
        // AExp37
        vec![ // 77 
            ARG(3),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 78 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp38
        vec![ // 79 
            ARG(4),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 80 
            ARG(0),
            ARG(2),
            ARG(3),
        ], 
        // AExp39
        vec![ // 81 
            PTR(10, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 82 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(4),
        ], 
        // AExp40
        vec![ // 83 
            PTR(64, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 84 
            ARG(0),
            ARG(2),
            ARG(3),
            ARG(4),
        ], 
        vec![ // 85 
            PTR(66, false, false),
            ARG(1),
        ], 
        // AExp41
        vec![ // 86 
            COM(4,133),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 87 
            ARG(0),
            ARG(1),
            ARG(3),
        ], 
        vec![ // 88 
            PTR(8, false, false),
            ARG(2),
        ], 
        // AExp42
        vec![ // 89 
            COM(4,133),
            PTR(0, true, true),
        ], 
        vec![ // 90 
            PTR(8, false, false),
            ARG(0),
        ], 
        // AExp43
        vec![ // 91 
            PRM(LT,false),
            ARG(4),
            INT(5),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 92 
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(4),
            ARG(5),
        ], 
        vec![ // 93 
            ARG(0),
            ARG(2),
            ARG(3),
            ARG(4),
            ARG(5),
        ], 
        // AExp44
        vec![ // 94 
            PRM(LT,false),
            ARG(4),
            INT(6),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 95 
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(5),
        ], 
        vec![ // 96 
            ARG(0),
            ARG(2),
            ARG(3),
            ARG(5),
        ], 
        // AExp45
        vec![ // 97 
            PTR(10, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 98 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(4),
        ], 
        // AExp46
        vec![ // 99 
            ARG(0),
            ARG(2),
            PTR(0, true, true),
        ], 
        vec![ // 100 
            ARG(1),
            ARG(3),
            ARG(4),
        ], 
        // AExp47
        vec![ // 101 
            PTR(64, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 102 
            COM(4,133),
            ARG(1),
            ARG(2),
        ], 
        vec![ // 103 
            PTR(66, false, false),
            ARG(0),
        ], 
        // AExp48
        vec![ // 104 
            PTR(69, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 105 
            PTR(8, false, false),
            ARG(1),
        ], 
        vec![ // 106 
            PTR(8, false, false),
            ARG(0),
        ], 
        // AExp49
        vec![ // 107 
            PTR(10, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 108 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp50
        vec![ // 109 
            PTR(64, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 110 
            PTR(66, false, false),
            ARG(0),
        ], 
        // AExp51
        vec![ // 111 
            PRM(LT,false),
            ARG(4),
            INT(4),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 112 
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(4),
            ARG(5),
        ], 
        vec![ // 113 
            ARG(0),
            ARG(2),
            ARG(3),
            ARG(5),
        ], 
        // AExp52
        vec![ // 114 
            PTR(10, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 115 
            ARG(0),
            ARG(1),
            ARG(3),
        ], 
        // AExp53
        vec![ // 116 
            PTR(64, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 117 
            PTR(66, false, false),
            ARG(0),
        ], 
        // AExp54
        vec![ // 118 
            PRM(EQ,false),
            ARG(3),
            INT(3),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 119 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(4),
        ], 
        // AExp55
        vec![ // 120 
            ARG(0),
            ARG(1),
            ARG(2),
            PTR(0, true, true),
        ], 
        vec![ // 121 
            PTR(8, false, false),
            ARG(3),
        ], 
        // AExp56
        vec![ // 122 
            PTR(10, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 123 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(4),
            ARG(3),
        ], 
        // AExp57
        vec![ // 124 
            PTR(64, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 125 
            ARG(0),
            ARG(2),
            ARG(3),
            ARG(4),
        ], 
        vec![ // 126 
            PTR(66, false, false),
            ARG(1),
        ], 
        // AExp58
        vec![ // 127 
            COM(4,133),
            ARG(3),
            PTR(0, true, true),
        ], 
        vec![ // 128 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(3),
        ], 
        // AExp59
        vec![ // 129 
            COM(4,133),
            PTR(0, true, true),
            ARG(2),
        ], 
        vec![ // 130 
            ARG(0),
            ARG(1),
            ARG(3),
        ], 
        // AExp60
        vec![ // 131 
            PTR(69, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 132 
            PTR(8, false, false),
            ARG(0),
        ], 
        // AExp61
        vec![ // 133 
            ARG(3),
            ARG(0),
            ARG(1),
        ], 
        // AExp62
        vec![ // 134 
            ARG(1),
            INT(8),
            ARG(0),
        ], 
        // AExp63
        vec![ // 135 
            ARG(1),
            INT(4),
            ARG(0),
        ], 
        // AExp64
        vec![ // 136 
            ARG(1),
            ARG(0),
        ], 
        // AExp65
        vec![ // 137 
            PRM(EQ,false),
            ARG(1),
            INT(10),
            INT(0),
            PTR(0, true, true),
        ], 
        vec![ // 138 
            ARG(2),
            ARG(0),
        ], 
        // AExp66
        vec![ // 139 
            ARG(2),
            INT(10),
            PTR(0, true, true),
        ], 
        vec![ // 140 
            ARG(0),
            ARG(1),
        ], 
        // AExp67
        vec![ // 141 
            Y,
            PTR(0, true, true),
            ARG(1),
        ], 
        vec![ // 142 
            ARG(0),
            ARG(2),
        ], 
        // AExp68
        vec![ // 143 
            COM(4,133),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 144 
            ARG(0),
            ARG(2),
        ], 
        // AExp69
        vec![ // 145 
            Y,
            ARG(0),
            ARG(1),
            COM(2,0),
        ], 
        // AExp70
        vec![ // 146 
            ARG(2),
            PTR(0, true, true),
        ], 
        vec![ // 147 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(3),
        ], 
        // AExp71
        vec![ // 148 
            ARG(0),
            ARG(3),
            PTR(0, true, true),
            ARG(4),
        ], 
        vec![ // 149 
            ARG(1),
            ARG(2),
            ARG(4),
        ], 
        // AExp72
        vec![ // 150 
            PRM(EQ,false),
            ARG(3),
            INT(2),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 151 
            ARG(4),
            ARG(1),
        ], 
        vec![ // 152 
            COM(4,133),
            ARG(0),
            ARG(2),
        ], 
        // AExp73
        vec![ // 153 
            ARG(0),
            ARG(2),
            PTR(0, true, true),
        ], 
        vec![ // 154 
            COM(4,133),
            ARG(3),
            ARG(1),
        ], 
        // AExp74
        vec![ // 155 
            ARG(1),
            INT(9),
            ARG(0),
        ], 
        // AExp75
        vec![ // 156 
            PRM(LT,false),
            ARG(3),
            INT(2),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 157 
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(4),
        ], 
        vec![ // 158 
            ARG(0),
            ARG(2),
            ARG(3),
            ARG(4),
        ], 
        // AExp76
        vec![ // 159 
            PRM(EQ,false),
            ARG(2),
            INT(2),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 160 
            ARG(3),
            ARG(0),
        ], 
        // AExp77
        vec![ // 161 
            PTR(69, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 162 
            PTR(71, false, false),
            ARG(1),
        ], 
        vec![ // 163 
            PTR(71, false, false),
            ARG(0),
        ], 
        // AExp78
        vec![ // 164 
            PRM(EQ,false),
            ARG(2),
            INT(1),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 165 
            ARG(3),
            ARG(0),
        ], 
        // AExp79
        vec![ // 166 
            PTR(75, false, false),
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 167 
            PTR(71, false, false),
            ARG(1),
        ], 
        // AExp80
        vec![ // 168 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 169 
            ARG(0),
            ARG(1),
        ], 
        // AExp81
        vec![ // 170 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(3),
            PTR(0, true, true),
        ], 
        vec![ // 171 
            PTR(69, false, false),
            PTR(60, false, false),
            ARG(3),
        ], 
        // AExp82
        vec![ // 172 
            ARG(3),
            PTR(0, true, true),
        ], 
        vec![ // 173 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(4),
        ], 
        // AExp83
        vec![ // 174 
            PRM(LT,false),
            ARG(5),
            INT(2),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 175 
            ARG(1),
            ARG(2),
            ARG(4),
            ARG(5),
            ARG(6),
        ], 
        vec![ // 176 
            ARG(0),
            ARG(3),
            ARG(4),
            ARG(5),
            ARG(6),
        ], 
        // AExp84
        vec![ // 177 
            PRM(EQ,false),
            ARG(2),
            INT(2),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 178 
            ARG(3),
            ARG(0),
        ], 
        // AExp85
        vec![ // 179 
            PTR(81, false, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 180 
            ARG(0),
            ARG(2),
        ], 
        vec![ // 181 
            ARG(0),
            ARG(1),
        ], 
        // AExp86
        vec![ // 182 
            ARG(0),
            PTR(0, true, true),
            ARG(3),
        ], 
        vec![ // 183 
            ARG(1),
            ARG(2),
            ARG(3),
        ], 
        // AExp87
        vec![ // 184 
            PRM(EQ,false),
            ARG(2),
            INT(0),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 185 
            ARG(3),
            ARG(0),
        ], 
        // AExp88
        vec![ // 186 
            PRM(EQ,false),
            ARG(0),
            ARG(2),
            ARG(1),
            PTR(100, false, false),
        ], 
        // AExp89
        vec![ // 187 
            ARG(0),
            ARG(2),
            ARG(3),
            PTR(0, true, true),
        ], 
        vec![ // 188 
            ARG(1),
            ARG(2),
            ARG(3),
        ], 
        // AExp90
        vec![ // 189 
            PRM(EQ,false),
            ARG(2),
            INT(2),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 190 
            ARG(3),
            ARG(0),
        ], 
        // AExp91
        vec![ // 191 
            ARG(0),
            ARG(3),
            PTR(0, true, true),
        ], 
        vec![ // 192 
            ARG(1),
            ARG(2),
            ARG(4),
        ], 
        // AExp92
        vec![ // 193 
            PRM(EQ,false),
            ARG(2),
            INT(4),
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 194 
            ARG(3),
            ARG(1),
        ], 
        // AExp93
        vec![ // 195 
            PRM(EQ,false),
            ARG(2),
            INT(2),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 196 
            ARG(3),
            ARG(0),
        ], 
        // AExp94
        vec![ // 197 
            PRM(EQ,false),
            ARG(2),
            INT(4),
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 198 
            ARG(3),
            ARG(1),
        ], 
        // AExp95
        vec![ // 199 
            PTR(69, false, false),
            PTR(60, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 200 
            PTR(69, false, false),
            ARG(0),
            ARG(1),
        ], 
        // AExp96
        vec![ // 201 
            PTR(69, false, false),
            PTR(0, true, true),
            ARG(0),
        ], 
        vec![ // 202 
            PTR(69, false, false),
            PTR(97, false, false),
            ARG(1),
        ], 
        // AExp97
        vec![ // 203 
            PRM(EQ,false),
            ARG(2),
            INT(2),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 204 
            ARG(3),
            ARG(0),
        ], 
        // AExp98
        vec![ // 205 
            PRM(EQ,false),
            ARG(2),
            INT(4),
            ARG(0),
            PTR(0, true, true),
        ], 
        vec![ // 206 
            ARG(3),
            ARG(1),
        ], 
        // AExp99
        vec![ // 207 
            PTR(69, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 208 
            PTR(69, false, false),
            PTR(98, false, false),
            ARG(0),
        ], 
        // AExp100
        vec![ // 209 
            PTR(69, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 210 
            PTR(69, false, false),
            PTR(99, false, false),
            ARG(0),
        ], 
        // AExp101
        vec![ // 211 
            ARG(1),
            INT(6),
            ARG(0),
        ], 
        // AExp102
        vec![ // 212 
            ARG(1),
            INT(7),
            ARG(0),
        ], 
        // AExp103
        vec![ // 213 
            ARG(1),
            INT(3),
            ARG(0),
        ], 
        // AExp104
        vec![ // 214 
            ARG(3),
            INT(1),
            PTR(0, true, true),
        ], 
        vec![ // 215 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp105
        vec![ // 216 
            ARG(1),
            INT(12),
            ARG(0),
        ], 
        // AExp106
        vec![ // 217 
            ARG(2),
            INT(0),
            PTR(0, true, true),
        ], 
        vec![ // 218 
            ARG(0),
            ARG(1),
        ], 
        // AExp107
        vec![ // 219 
            ARG(1),
            INT(11),
            ARG(0),
        ], 
    ],

}});