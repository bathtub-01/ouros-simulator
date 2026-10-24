use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 180
#[rustfmt::skip]
pub static UART: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            Com(1,4),
            Ptr(18, false, false),
            Ptr(17, false, false),
            Ptr(1, false, false),
            Int(1993),
        ], 
        vec![ // 1 
            Ptr(20, false, false),
            Com(2,0),
        ], 
        vec![ // 2 
            Com(4,3),
            Int(111),
            Com(2,0),
        ], 
        vec![ // 3 
            Com(4,3),
            Int(109),
            Ptr(2, false, false),
        ], 
        vec![ // 4 
            Com(4,3),
            Int(101),
            Ptr(3, false, false),
        ], 
        vec![ // 5 
            Com(4,3),
            Int(100),
            Ptr(4, false, false),
        ], 
        vec![ // 6 
            Com(4,3),
            Int(32),
            Ptr(5, false, false),
        ], 
        vec![ // 7 
            Com(4,3),
            Int(84),
            Ptr(6, false, false),
        ], 
        vec![ // 8 
            Com(4,3),
            Int(82),
            Ptr(7, false, false),
        ], 
        vec![ // 9 
            Com(4,3),
            Int(65),
            Ptr(8, false, false),
        ], 
        vec![ // 10 
            Com(4,3),
            Int(85),
            Ptr(9, false, false),
        ], 
        vec![ // 11 
            Com(4,3),
            Int(32),
            Ptr(10, false, false),
        ], 
        vec![ // 12 
            Com(4,3),
            Int(115),
            Ptr(11, false, false),
        ], 
        vec![ // 13 
            Com(4,3),
            Int(111),
            Ptr(12, false, false),
        ], 
        vec![ // 14 
            Com(4,3),
            Int(114),
            Ptr(13, false, false),
        ], 
        vec![ // 15 
            Com(4,3),
            Int(117),
            Ptr(14, false, false),
        ], 
        vec![ // 16 
            Com(4,3),
            Int(79),
            Ptr(15, false, false),
        ], 
        vec![ // 17 
            Com(1,17),
            Ptr(16, false, false),
        ], 
        // AExp1
        vec![ // 18 
            Com(4,13),
            Com(2,9),
            Ptr(19, false, false),
            Com(1,11),
        ], 
        vec![ // 19 
            Com(3,14),
            Ptr(18, false, false),
        ], 
        // AExp2
        vec![ // 20 
            Com(2,32),
            Ptr(26, false, false),
        ], 
        vec![ // 21 
            Com(4,3),
            Int(32),
            Com(2,0),
        ], 
        vec![ // 22 
            Com(4,3),
            Int(62),
            Ptr(21, false, false),
        ], 
        vec![ // 23 
            Com(1,40),
            Ptr(22, false, false),
        ], 
        vec![ // 24 
            Com(1,38),
            Ptr(23, false, false),
        ], 
        vec![ // 25 
            Com(1,36),
            Ptr(24, false, false),
        ], 
        vec![ // 26 
            Com(2,34),
            Ptr(25, false, false),
        ], 
        // AExp3
        vec![ // 27 
            Com(2,102),
            Ptr(33, false, false),
        ], 
        vec![ // 28 
            Com(4,3),
            Int(33),
            Com(2,0),
        ], 
        vec![ // 29 
            Com(4,3),
            Int(119),
            Ptr(28, false, false),
        ], 
        vec![ // 30 
            Com(1,110),
            Ptr(29, false, false),
        ], 
        vec![ // 31 
            Com(1,108),
            Ptr(30, false, false),
        ], 
        vec![ // 32 
            Com(1,106),
            Ptr(31, false, false),
        ], 
        vec![ // 33 
            Com(1,104),
            Ptr(32, false, false),
        ], 
        // AExp4
        vec![ // 34 
            Com(2,142),
            Ptr(73, false, false),
            Ptr(62, false, false),
        ], 
        vec![ // 35 
            Com(1,4),
            Ptr(74, false, false),
            Ptr(76, false, false),
            Ptr(80, false, false),
        ], 
        vec![ // 36 
            Com(1,16),
            Ptr(74, false, false),
            Ptr(35, false, false),
            Com(1,176),
        ], 
        vec![ // 37 
            Com(4,3),
            Int(114),
            Com(2,0),
        ], 
        vec![ // 38 
            Com(4,3),
            Int(97),
            Ptr(37, false, false),
        ], 
        vec![ // 39 
            Com(4,3),
            Int(115),
            Ptr(38, false, false),
        ], 
        vec![ // 40 
            Com(4,3),
            Int(101),
            Ptr(39, false, false),
        ], 
        vec![ // 41 
            Com(4,3),
            Int(97),
            Ptr(40, false, false),
        ], 
        vec![ // 42 
            Com(4,3),
            Int(99),
            Ptr(41, false, false),
        ], 
        vec![ // 43 
            Com(1,179),
            Ptr(42, false, false),
            Ptr(36, false, false),
        ], 
        vec![ // 44 
            Com(1,4),
            Ptr(74, false, false),
            Ptr(76, false, false),
            Ptr(80, false, false),
        ], 
        vec![ // 45 
            Com(1,16),
            Ptr(74, false, false),
            Ptr(44, false, false),
            Com(1,171),
        ], 
        vec![ // 46 
            Com(4,3),
            Int(98),
            Com(2,0),
        ], 
        vec![ // 47 
            Com(4,3),
            Int(105),
            Ptr(46, false, false),
        ], 
        vec![ // 48 
            Com(4,3),
            Int(102),
            Ptr(47, false, false),
        ], 
        vec![ // 49 
            Com(1,179),
            Ptr(48, false, false),
            Ptr(45, false, false),
        ], 
        vec![ // 50 
            Com(2,142),
            Ptr(49, false, false),
            Ptr(43, false, false),
        ], 
        vec![ // 51 
            Com(4,3),
            Int(33),
            Com(2,0),
        ], 
        vec![ // 52 
            Com(4,3),
            Int(111),
            Ptr(51, false, false),
        ], 
        vec![ // 53 
            Com(4,3),
            Int(108),
            Ptr(52, false, false),
        ], 
        vec![ // 54 
            Com(4,3),
            Int(108),
            Ptr(53, false, false),
        ], 
        vec![ // 55 
            Com(4,3),
            Int(101),
            Ptr(54, false, false),
        ], 
        vec![ // 56 
            Com(4,3),
            Int(104),
            Ptr(55, false, false),
        ], 
        vec![ // 57 
            Com(1,0),
            Ptr(56, false, false),
        ], 
        vec![ // 58 
            Com(1,25),
            Ptr(74, false, false),
            Ptr(57, false, false),
        ], 
        vec![ // 59 
            Com(4,3),
            Int(105),
            Com(2,0),
        ], 
        vec![ // 60 
            Com(4,3),
            Int(104),
            Ptr(59, false, false),
        ], 
        vec![ // 61 
            Com(1,179),
            Ptr(60, false, false),
            Ptr(58, false, false),
        ], 
        vec![ // 62 
            Com(2,142),
            Ptr(61, false, false),
            Ptr(50, false, false),
        ], 
        vec![ // 63 
            Com(4,3),
            Int(33),
            Com(2,0),
        ], 
        vec![ // 64 
            Com(4,3),
            Int(105),
            Ptr(63, false, false),
        ], 
        vec![ // 65 
            Com(4,3),
            Int(104),
            Ptr(64, false, false),
        ], 
        vec![ // 66 
            Com(1,0),
            Ptr(65, false, false),
        ], 
        vec![ // 67 
            Com(1,25),
            Ptr(74, false, false),
            Ptr(66, false, false),
        ], 
        vec![ // 68 
            Com(4,3),
            Int(111),
            Com(2,0),
        ], 
        vec![ // 69 
            Com(4,3),
            Int(108),
            Ptr(68, false, false),
        ], 
        vec![ // 70 
            Com(4,3),
            Int(108),
            Ptr(69, false, false),
        ], 
        vec![ // 71 
            Com(4,3),
            Int(101),
            Ptr(70, false, false),
        ], 
        vec![ // 72 
            Com(4,3),
            Int(104),
            Ptr(71, false, false),
        ], 
        vec![ // 73 
            Com(1,179),
            Ptr(72, false, false),
            Ptr(67, false, false),
        ], 
        // AExp5
        vec![ // 74 
            Com(4,13),
            Com(2,127),
            Ptr(75, false, false),
            Com(1,131),
        ], 
        vec![ // 75 
            Com(3,14),
            Ptr(74, false, false),
        ], 
        // AExp6
        vec![ // 76 
            Com(1,4),
            Ptr(74, false, false),
            Ptr(79, false, false),
            Ptr(77, false, false),
        ], 
        vec![ // 77 
            Com(1,25),
            Ptr(74, false, false),
            Com(1,0),
        ], 
        vec![ // 78 
            Com(1,157),
            Com(1,159),
        ], 
        vec![ // 79 
            Com(1,149),
            Ptr(78, false, false),
        ], 
        // AExp7
        vec![ // 80 
            Com(2,142),
            Ptr(90, false, false),
            Ptr(86, false, false),
        ], 
        vec![ // 81 
            Com(1,196),
            Com(2,0),
        ], 
        vec![ // 82 
            Com(1,196),
            Com(2,0),
        ], 
        vec![ // 83 
            Com(1,0),
            Int(43),
        ], 
        vec![ // 84 
            Com(1,192),
            Ptr(83, false, false),
        ], 
        vec![ // 85 
            Com(1,4),
            Ptr(74, false, false),
            Ptr(84, false, false),
            Ptr(82, false, false),
        ], 
        vec![ // 86 
            Com(2,142),
            Ptr(85, false, false),
            Ptr(81, false, false),
        ], 
        vec![ // 87 
            Com(1,196),
            Com(2,1),
        ], 
        vec![ // 88 
            Com(1,0),
            Int(45),
        ], 
        vec![ // 89 
            Com(1,192),
            Ptr(88, false, false),
        ], 
        vec![ // 90 
            Com(1,4),
            Ptr(74, false, false),
            Ptr(89, false, false),
            Ptr(87, false, false),
        ], 
        // AExp8
        vec![ // 91 
            Com(1,0),
            Com(1,241),
        ], 
        // AExp9
        vec![ // 92 
            Com(1,149),
            Ptr(93, false, false),
        ], 
        vec![ // 93 
            Com(1,157),
            Com(1,282),
        ], 
        // AExp10
        vec![ // 94 
            Com(3,42),
            Ptr(104, false, false),
        ], 
        vec![ // 95 
            Com(4,3),
            Int(8),
            Com(2,0),
        ], 
        vec![ // 96 
            Com(4,3),
            Int(32),
            Ptr(95, false, false),
        ], 
        vec![ // 97 
            Com(1,90),
            Ptr(96, false, false),
        ], 
        vec![ // 98 
            Com(3,88),
            Ptr(97, false, false),
        ], 
        vec![ // 99 
            Com(4,86),
            Ptr(98, false, false),
        ], 
        vec![ // 100 
            Com(5,83),
            Ptr(99, false, false),
        ], 
        vec![ // 101 
            Com(5,69),
            Ptr(100, false, false),
        ], 
        vec![ // 102 
            Com(4,61),
            Ptr(101, false, false),
        ], 
        vec![ // 103 
            Com(4,48),
            Ptr(102, false, false),
        ], 
        vec![ // 104 
            Com(3,45),
            Ptr(103, false, false),
        ], 
        // AExp11
        vec![ // 105 
            Com(1,16),
            Ptr(18, false, false),
            Ptr(106, false, false),
            Com(1,92),
        ], 
        vec![ // 106 
            IO(false),
            Int(0),
        ], 
        // AExp12
        vec![ // 107 
            Com(1,4),
            Ptr(18, false, false),
            Ptr(111, false, false),
            Ptr(109, false, false),
        ], 
        vec![ // 108 
            Com(1,0),
            Int(10),
        ], 
        vec![ // 109 
            Ptr(112, false, false),
            Ptr(108, false, false),
        ], 
        vec![ // 110 
            Com(1,0),
            Int(13),
        ], 
        vec![ // 111 
            Ptr(112, false, false),
            Ptr(110, false, false),
        ], 
        // AExp13
        vec![ // 112 
            IO(true),
            Int(4),
        ], 
        // AExp14
        vec![ // 113 
            Com(1,4),
            Ptr(74, false, false),
            Ptr(116, false, false),
            Ptr(114, false, false),
        ], 
        vec![ // 114 
            Com(1,25),
            Ptr(74, false, false),
            Com(1,0),
        ], 
        vec![ // 115 
            Com(1,157),
            Com(1,159),
        ], 
        vec![ // 116 
            Com(1,134),
            Ptr(115, false, false),
        ], 
        // AExp15
        vec![ // 117 
            Com(1,0),
            Com(1,164),
        ], 
        // AExp16
        vec![ // 118 
            Com(1,16),
            Ptr(74, false, false),
            Ptr(119, false, false),
            Com(1,202),
        ], 
        vec![ // 119 
            Com(1,157),
            Com(1,199),
        ], 
        // AExp17
        vec![ // 120 
            Com(2,204),
            Ptr(123, false, false),
        ], 
        vec![ // 121 
            Com(1,0),
            Com(5,221),
            Com(5,226),
            Com(1,229),
            Com(2,233),
            Com(3,238),
            Com(4,240),
        ], 
        vec![ // 122 
            Com(4,208),
            Ptr(121, false, false),
        ], 
        vec![ // 123 
            Com(5,207),
            Ptr(122, false, false),
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
            Arg(2, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp3
        vec![ // 3 
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp4
        vec![ // 4 
            Arg(0, true),
            Com(3,1),
        ], 
        // AExp5
        vec![ // 5 
            Com(1,0),
            Ptr(0, true, true),
        ], 
        vec![ // 6 
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp6
        vec![ // 7 
            Arg(0, true),
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 8 
            Com(2,5),
            Arg(1, true),
        ], 
        // AExp7
        vec![ // 9 
            Com(1,0),
            Ptr(0, true, true),
        ], 
        vec![ // 10 
            Com(3,7),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp8
        vec![ // 11 
            Com(1,0),
            Ptr(0, true, true),
        ], 
        vec![ // 12 
            Com(3,2),
            Arg(0, true),
        ], 
        // AExp9
        vec![ // 13 
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp10
        vec![ // 14 
            Com(1,16),
            Arg(0, true),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 15 
            Com(2,0),
            Arg(2, true),
        ], 
        // AExp11
        vec![ // 16 
            Arg(0, true),
            Com(3,0),
        ], 
        // AExp12
        vec![ // 17 
            Com(1,4),
            Ptr(18, false, false),
            Ptr(0, true, true),
            Ptr(107, false, false),
        ], 
        vec![ // 18 
            Com(1,22),
            Arg(0, true),
        ], 
        // AExp13
        vec![ // 19 
            Com(1,4),
            Ptr(18, false, false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 20 
            Com(1,22),
            Arg(1, true),
        ], 
        vec![ // 21 
            Ptr(112, false, false),
            Arg(0, true),
        ], 
        // AExp14
        vec![ // 22 
            Arg(0, true),
            Ptr(0, true, true),
            Com(2,19),
        ], 
        vec![ // 23 
            Com(1,25),
            Ptr(18, false, false),
            Com(1,0),
        ], 
        // AExp15
        vec![ // 24 
            Arg(2, true),
        ], 
        // AExp16
        vec![ // 25 
            Arg(0, true),
            Com(3,24),
        ], 
        // AExp17
        vec![ // 26 
            Com(1,17),
            Ptr(0, true, true),
        ], 
        vec![ // 27 
            Ptr(27, false, false),
            Arg(0, true),
        ], 
        // AExp18
        vec![ // 28 
            Com(1,4),
            Ptr(18, false, false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 29 
            Ptr(20, false, false),
            Arg(1, true),
        ], 
        vec![ // 30 
            Com(1,26),
            Arg(0, true),
        ], 
        // AExp19
        vec![ // 31 
            Arg(0, true),
            Com(2,28),
        ], 
        // AExp20
        vec![ // 32 
            Com(1,16),
            Ptr(18, false, false),
            Ptr(0, true, true),
            Com(1,31),
        ], 
        vec![ // 33 
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp21
        vec![ // 34 
            Com(1,4),
            Ptr(18, false, false),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 35 
            Ptr(94, false, false),
            Arg(1, true),
            Com(2,0),
        ], 
        // AExp22
        vec![ // 36 
            Com(1,22),
            Ptr(0, true, true),
        ], 
        vec![ // 37 
            Com(4,3),
            Int(111),
            Arg(0, true),
        ], 
        // AExp23
        vec![ // 38 
            Com(4,3),
            Int(117),
            Ptr(0, true, true),
        ], 
        vec![ // 39 
            Com(4,3),
            Int(114),
            Arg(0, true),
        ], 
        // AExp24
        vec![ // 40 
            Com(4,3),
            Int(111),
            Ptr(0, true, true),
        ], 
        vec![ // 41 
            Com(4,3),
            Int(115),
            Arg(0, true),
        ], 
        // AExp25
        vec![ // 42 
            Com(1,16),
            Ptr(18, false, false),
            Ptr(105, false, false),
            Ptr(0, true, true),
        ], 
        vec![ // 43 
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp26
        vec![ // 44 
            Arg(0, true),
            Arg(2, true),
            Arg(1, true),
            Ptr(18, false, false),
            Ptr(18, false, false),
            Ptr(18, false, false),
        ], 
        // AExp27
        vec![ // 45 
            Com(3,44),
            Ptr(0, true, true),
            Ptr(18, false, false),
        ], 
        vec![ // 46 
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp28
        vec![ // 47 
            Ptr(94, false, false),
            Com(2,0),
            Arg(0, true),
        ], 
        // AExp29
        vec![ // 48 
            Com(1,95),
            Arg(1, true),
            Ptr(0, true, true),
            Arg(0, true),
            Com(7,47),
            Arg(2, true),
            Arg(3, false),
            Ptr(18, false, false),
        ], 
        vec![ // 49 
            Prm(EQ,false),
            Arg(3, false),
            Int(10),
        ], 
        // AExp30
        vec![ // 50 
            Arg(0, true),
            Arg(2, true),
            Arg(1, true),
        ], 
        // AExp31
        vec![ // 51 
            Arg(0, true),
            Ptr(0, true, true),
            Arg(5, false),
        ], 
        vec![ // 52 
            Arg(1, true),
            Arg(4, true),
            Arg(5, false),
            Arg(6, true),
        ], 
        // AExp32
        vec![ // 53 
            Prm(EQ,false),
            Arg(4, true),
            Int(13),
            Ptr(0, true, true),
            Arg(2, true),
            Arg(5, true),
            Arg(6, true),
            Arg(3, true),
        ], 
        vec![ // 54 
            Com(7,51),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp33
        vec![ // 55 
            Com(1,4),
            Arg(0, true),
            Ptr(107, false, false),
            Ptr(0, true, true),
        ], 
        vec![ // 56 
            Com(1,25),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp34
        vec![ // 57 
            Arg(1, true),
            Ptr(0, true, true),
            Com(2,1),
        ], 
        vec![ // 58 
            Com(1,101),
            Arg(0, true),
        ], 
        // AExp35
        vec![ // 59 
            Com(7,55),
            Arg(0, true),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 60 
            Com(2,57),
            Arg(2, true),
        ], 
        // AExp36
        vec![ // 61 
            Com(3,50),
            Ptr(0, true, true),
            Arg(2, false),
        ], 
        vec![ // 62 
            Com(7,53),
            Com(3,50),
            Arg(0, true),
            Com(3,59),
            Arg(1, true),
            Arg(2, false),
            Arg(3, true),
        ], 
        // AExp37
        vec![ // 63 
            Com(1,4),
            Arg(0, true),
            Ptr(107, false, false),
            Ptr(0, true, true),
        ], 
        vec![ // 64 
            Com(1,25),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp38
        vec![ // 65 
            Arg(1, true),
            Ptr(0, true, true),
            Com(2,0),
        ], 
        vec![ // 66 
            Com(1,101),
            Arg(0, true),
        ], 
        // AExp39
        vec![ // 67 
            Com(5,63),
            Arg(0, true),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 68 
            Com(2,65),
            Arg(2, true),
        ], 
        // AExp40
        vec![ // 69 
            Prm(EQ,false),
            Arg(2, true),
            Int(10),
            Arg(0, true),
            Com(3,67),
            Arg(3, true),
            Arg(4, true),
            Arg(1, true),
        ], 
        // AExp41
        vec![ // 70 
            Ptr(94, false, false),
            Com(2,0),
        ], 
        // AExp42
        vec![ // 71 
            Ptr(112, false, false),
            Ptr(0, true, true),
        ], 
        vec![ // 72 
            Com(1,0),
            Arg(0, true),
        ], 
        // AExp43
        vec![ // 73 
            Com(4,3),
            Ptr(0, true, true),
        ], 
        vec![ // 74 
            Com(1,0),
            Arg(0, true),
        ], 
        // AExp44
        vec![ // 75 
            Ptr(94, false, false),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 76 
            Com(1,73),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp45
        vec![ // 77 
            Com(1,4),
            Arg(0, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 78 
            Com(2,75),
            Arg(1, false),
            Arg(2, true),
        ], 
        vec![ // 79 
            Com(1,71),
            Arg(1, false),
        ], 
        // AExp46
        vec![ // 80 
            Com(1,95),
            Ptr(1, true, true),
            Ptr(0, true, true),
            Com(2,70),
            Com(3,77),
            Arg(2, true),
            Arg(1, false),
            Arg(0, true),
        ], 
        vec![ // 81 
            Prm(LE,false),
            Arg(1, false),
            Int(126),
        ], 
        vec![ // 82 
            Prm(LT,true),
            Arg(1, false),
            Int(32),
        ], 
        // AExp47
        vec![ // 83 
            Com(1,97),
            Ptr(1, true, true),
            Ptr(0, true, true),
            Com(3,80),
            Arg(0, true),
            Arg(3, true),
            Arg(4, false),
        ], 
        vec![ // 84 
            Prm(EQ,false),
            Arg(4, false),
            Int(127),
        ], 
        vec![ // 85 
            Prm(EQ,false),
            Arg(4, false),
            Int(8),
        ], 
        // AExp48
        vec![ // 86 
            Arg(1, true),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 87 
            Ptr(94, false, false),
            Com(2,0),
            Com(2,0),
        ], 
        // AExp49
        vec![ // 88 
            Com(1,4),
            Ptr(18, false, false),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 89 
            Ptr(94, false, false),
            Com(2,0),
            Arg(2, true),
        ], 
        // AExp50
        vec![ // 90 
            Com(1,22),
            Ptr(0, true, true),
        ], 
        vec![ // 91 
            Com(4,3),
            Int(8),
            Arg(0, true),
        ], 
        // AExp51
        vec![ // 92 
            Com(1,25),
            Ptr(18, false, false),
            Ptr(0, true, true),
        ], 
        vec![ // 93 
            Com(1,0),
            Arg(0, true),
        ], 
        // AExp52
        vec![ // 94 
            Com(2,0),
        ], 
        // AExp53
        vec![ // 95 
            Arg(0, true),
            Com(1,94),
            Com(1,0),
        ], 
        // AExp54
        vec![ // 96 
            Com(2,1),
        ], 
        // AExp55
        vec![ // 97 
            Arg(0, true),
            Com(1,0),
            Com(1,96),
        ], 
        // AExp56
        vec![ // 98 
            Arg(3, true),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 99 
            Com(4,3),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp57
        vec![ // 100 
            Arg(1, true),
            Com(2,0),
            Com(4,98),
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp58
        vec![ // 101 
            Y,
            Com(3,100),
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp59
        vec![ // 102 
            Com(2,119),
            Ptr(34, false, false),
            Arg(1, true),
            Ptr(0, true, true),
            Com(1,0),
        ], 
        vec![ // 103 
            Com(4,3),
            Int(100),
            Arg(0, true),
        ], 
        // AExp60
        vec![ // 104 
            Com(4,3),
            Int(111),
            Ptr(0, true, true),
        ], 
        vec![ // 105 
            Com(4,3),
            Int(110),
            Arg(0, true),
        ], 
        // AExp61
        vec![ // 106 
            Com(4,3),
            Int(39),
            Ptr(0, true, true),
        ], 
        vec![ // 107 
            Com(4,3),
            Int(116),
            Arg(0, true),
        ], 
        // AExp62
        vec![ // 108 
            Com(4,3),
            Int(32),
            Ptr(0, true, true),
        ], 
        vec![ // 109 
            Com(4,3),
            Int(107),
            Arg(0, true),
        ], 
        // AExp63
        vec![ // 110 
            Com(4,3),
            Int(110),
            Ptr(0, true, true),
        ], 
        vec![ // 111 
            Com(4,3),
            Int(111),
            Arg(0, true),
        ], 
        // AExp64
        vec![ // 112 
            Com(1,4),
            Ptr(74, false, false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 113 
            Com(1,25),
            Ptr(74, false, false),
            Arg(0, true),
        ], 
        vec![ // 114 
            Com(1,4),
            Ptr(74, false, false),
            Ptr(113, false, false),
            Ptr(117, false, false),
        ], 
        // AExp65
        vec![ // 115 
            Com(1,16),
            Ptr(74, false, false),
            Ptr(0, true, true),
            Com(1,112),
        ], 
        vec![ // 116 
            Com(1,4),
            Ptr(74, false, false),
            Ptr(113, false, false),
            Arg(0, true),
        ], 
        // AExp66
        vec![ // 117 
            Com(3,133),
            Arg(0, true),
        ], 
        // AExp67
        vec![ // 118 
            Arg(0, true),
            Com(2,117),
        ], 
        // AExp68
        vec![ // 119 
            Com(1,0),
            Ptr(0, true, true),
            Arg(1, true),
            Com(2,0),
            Com(1,118),
        ], 
        vec![ // 120 
            Com(1,115),
            Arg(0, true),
        ], 
        // AExp69
        vec![ // 121 
            Com(2,0),
        ], 
        // AExp70
        vec![ // 122 
            Com(1,0),
            Ptr(0, true, true),
        ], 
        vec![ // 123 
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp71
        vec![ // 124 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 125 
            Com(2,122),
            Arg(1, true),
        ], 
        // AExp72
        vec![ // 126 
            Arg(0, true),
            Arg(2, true),
            Com(1,121),
            Com(2,124),
            Arg(1, true),
        ], 
        // AExp73
        vec![ // 127 
            Com(1,0),
            Ptr(0, true, true),
        ], 
        vec![ // 128 
            Com(3,126),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp74
        vec![ // 129 
            Com(3,133),
            Ptr(0, true, true),
        ], 
        vec![ // 130 
            Com(3,2),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp75
        vec![ // 131 
            Com(1,0),
            Ptr(0, true, true),
        ], 
        vec![ // 132 
            Com(2,129),
            Arg(0, true),
        ], 
        // AExp76
        vec![ // 133 
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp77
        vec![ // 134 
            Com(2,142),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 135 
            Com(1,25),
            Ptr(74, false, false),
            Com(2,0),
        ], 
        vec![ // 136 
            Com(1,149),
            Arg(0, true),
        ], 
        // AExp78
        vec![ // 137 
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp79
        vec![ // 138 
            Arg(3, true),
        ], 
        // AExp80
        vec![ // 139 
            Arg(2, false),
            Com(3,137),
            Com(4,138),
            Arg(0, true),
            Arg(1, true),
            Arg(2, false),
        ], 
        // AExp81
        vec![ // 140 
            Com(3,139),
            Arg(1, true),
            Arg(2, false),
            Ptr(0, true, true),
        ], 
        vec![ // 141 
            Arg(0, true),
            Arg(2, false),
        ], 
        // AExp82
        vec![ // 142 
            Com(1,0),
            Ptr(0, true, true),
        ], 
        vec![ // 143 
            Com(3,140),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp83
        vec![ // 144 
            Com(1,25),
            Ptr(74, false, false),
            Ptr(0, true, true),
        ], 
        vec![ // 145 
            Com(4,3),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp84
        vec![ // 146 
            Com(1,16),
            Ptr(74, false, false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 147 
            Com(2,144),
            Arg(1, true),
        ], 
        vec![ // 148 
            Com(1,134),
            Arg(0, true),
        ], 
        // AExp85
        vec![ // 149 
            Com(1,16),
            Ptr(74, false, false),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 150 
            Com(2,146),
            Arg(0, false),
        ], 
        // AExp86
        vec![ // 151 
            Com(2,0),
        ], 
        // AExp87
        vec![ // 152 
            Com(2,0),
        ], 
        // AExp88
        vec![ // 153 
            Com(3,133),
            Ptr(0, true, true),
        ], 
        vec![ // 154 
            Com(3,2),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp89
        vec![ // 155 
            Arg(2, true),
            Arg(0, false),
            Com(2,152),
            Com(2,153),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp90
        vec![ // 156 
            Arg(1, true),
            Com(1,151),
            Com(3,155),
            Arg(0, true),
        ], 
        // AExp91
        vec![ // 157 
            Com(1,0),
            Ptr(0, true, true),
        ], 
        vec![ // 158 
            Com(2,156),
            Arg(0, true),
        ], 
        // AExp92
        vec![ // 159 
            Com(1,97),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 160 
            Prm(EQ,false),
            Arg(0, false),
            Int(9),
        ], 
        vec![ // 161 
            Prm(EQ,false),
            Arg(0, false),
            Int(32),
        ], 
        // AExp93
        vec![ // 162 
            Arg(0, true),
            Com(1,0),
            Com(2,0),
        ], 
        // AExp94
        vec![ // 163 
            Com(2,0),
        ], 
        // AExp95
        vec![ // 164 
            Arg(0, true),
            Ptr(0, true, true),
            Com(2,163),
        ], 
        vec![ // 165 
            Com(3,133),
            Com(1,162),
        ], 
        // AExp96
        vec![ // 166 
            Ptr(91, false, false),
        ], 
        // AExp97
        vec![ // 167 
            Com(1,257),
            Ptr(0, true, true),
        ], 
        vec![ // 168 
            Com(1,281),
            Arg(0, true),
        ], 
        // AExp98
        vec![ // 169 
            Com(1,25),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 170 
            Com(1,167),
            Arg(1, true),
        ], 
        // AExp99
        vec![ // 171 
            Com(1,95),
            Ptr(1, true, true),
            Ptr(0, true, true),
            Com(2,166),
            Com(2,169),
            Ptr(74, false, false),
            Arg(0, false),
        ], 
        vec![ // 172 
            Prm(LE,false),
            Arg(0, false),
            Int(40),
        ], 
        vec![ // 173 
            Prm(LT,true),
            Arg(0, false),
            Int(0),
        ], 
        // AExp100
        vec![ // 174 
            Com(1,25),
            Ptr(74, false, false),
            Ptr(0, true, true),
        ], 
        vec![ // 175 
            Com(1,285),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp101
        vec![ // 176 
            Com(1,16),
            Ptr(74, false, false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 177 
            Com(2,174),
            Arg(0, true),
        ], 
        vec![ // 178 
            Com(1,4),
            Ptr(74, false, false),
            Ptr(76, false, false),
            Ptr(92, false, false),
        ], 
        // AExp102
        vec![ // 179 
            Com(1,4),
            Ptr(74, false, false),
            Ptr(0, true, true),
        ], 
        vec![ // 180 
            Com(1,189),
            Arg(0, true),
        ], 
        // AExp103
        vec![ // 181 
            Com(1,4),
            Ptr(74, false, false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 182 
            Com(1,189),
            Arg(1, true),
        ], 
        vec![ // 183 
            Com(1,192),
            Arg(0, true),
        ], 
        // AExp104
        vec![ // 184 
            Com(1,25),
            Ptr(74, false, false),
            Ptr(0, true, true),
        ], 
        vec![ // 185 
            Com(4,3),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp105
        vec![ // 186 
            Com(1,4),
            Ptr(74, false, false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 187 
            Com(2,184),
            Arg(0, false),
            Arg(1, false),
        ], 
        vec![ // 188 
            Com(2,181),
            Arg(0, false),
            Arg(1, false),
        ], 
        // AExp106
        vec![ // 189 
            Arg(0, true),
            Ptr(0, true, true),
            Com(2,186),
        ], 
        vec![ // 190 
            Com(1,25),
            Ptr(74, false, false),
            Com(2,0),
        ], 
        // AExp107
        vec![ // 191 
            Prm(EQ,false),
            Arg(1, true),
            Arg(0, true),
        ], 
        // AExp108
        vec![ // 192 
            Com(1,157),
            Ptr(0, true, true),
        ], 
        vec![ // 193 
            Com(2,191),
            Arg(0, true),
        ], 
        // AExp109
        vec![ // 194 
            Ptr(120, false, false),
            Arg(0, true),
            Arg(1, true),
            Ptr(91, false, false),
            Ptr(0, true, true),
        ], 
        vec![ // 195 
            Com(1,25),
            Ptr(74, false, false),
        ], 
        // AExp110
        vec![ // 196 
            Com(1,16),
            Ptr(74, false, false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 197 
            Com(2,194),
            Arg(0, true),
        ], 
        vec![ // 198 
            Com(1,149),
            Ptr(118, false, false),
        ], 
        // AExp111
        vec![ // 199 
            Com(1,95),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 200 
            Prm(LE,false),
            Arg(0, false),
            Int(57),
        ], 
        vec![ // 201 
            Prm(LT,true),
            Arg(0, false),
            Int(48),
        ], 
        // AExp112
        vec![ // 202 
            Com(1,25),
            Ptr(74, false, false),
            Ptr(0, true, true),
        ], 
        vec![ // 203 
            Prm(Sub,false),
            Arg(0, true),
            Int(48),
        ], 
        // AExp113
        vec![ // 204 
            Y,
            Ptr(0, true, true),
            Int(0),
        ], 
        vec![ // 205 
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp114
        vec![ // 206 
            Com(3,133),
            Arg(0, true),
        ], 
        // AExp115
        vec![ // 207 
            Arg(4, true),
            Com(3,206),
            Arg(0, true),
            Arg(3, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp116
        vec![ // 208 
            Arg(0, true),
            Arg(3, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp117
        vec![ // 209 
            Arg(5, true),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
            Arg(6, true),
            Arg(4, true),
        ], 
        // AExp118
        vec![ // 210 
            Com(1,95),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 211 
            Prm(LE,true),
            Arg(1, true),
            Int(5),
        ], 
        vec![ // 212 
            Prm(EQ,false),
            Arg(0, true),
            Int(26843545),
        ], 
        // AExp119
        vec![ // 213 
            Prm(Add,false),
            Ptr(0, true, true),
        ], 
        vec![ // 214 
            Prm(Mul,false),
            Arg(0, true),
            Int(10),
        ], 
        // AExp120
        vec![ // 215 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 216 
            Com(1,213),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp121
        vec![ // 217 
            Com(2,0),
        ], 
        // AExp122
        vec![ // 218 
            Com(1,97),
            Ptr(1, true, true),
            Ptr(0, true, true),
            Com(3,215),
            Com(4,217),
            Arg(2, true),
            Arg(0, false),
            Arg(1, false),
        ], 
        vec![ // 219 
            Com(2,210),
            Arg(0, false),
            Arg(1, false),
        ], 
        vec![ // 220 
            Prm(LE,true),
            Arg(0, false),
            Int(26843545),
        ], 
        // AExp123
        vec![ // 221 
            Com(7,209),
            Com(3,218),
            Ptr(0, true, true),
        ], 
        vec![ // 222 
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
            Arg(4, true),
        ], 
        // AExp124
        vec![ // 223 
            Com(1,97),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
            Arg(6, true),
            Arg(4, true),
            Arg(5, true),
        ], 
        // AExp125
        vec![ // 224 
            Com(7,223),
            Arg(0, true),
            Ptr(0, true, true),
            Arg(2, true),
            Arg(3, true),
            Arg(4, true),
            Arg(5, false),
        ], 
        vec![ // 225 
            Arg(1, true),
            Arg(5, false),
        ], 
        // AExp126
        vec![ // 226 
            Com(6,224),
            Ptr(1, true, true),
            Ptr(0, true, true),
            Arg(2, true),
            Arg(3, true),
            Arg(4, false),
        ], 
        vec![ // 227 
            Arg(1, true),
            Arg(4, false),
        ], 
        vec![ // 228 
            Arg(0, true),
            Arg(4, false),
        ], 
        // AExp127
        vec![ // 229 
            Prm(LT,false),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 230 
            Prm(Sub,false),
            Int(0),
            Int(26843545),
        ], 
        // AExp128
        vec![ // 231 
            Prm(EQ,false),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 232 
            Prm(Sub,false),
            Int(0),
            Int(26843545),
        ], 
        // AExp129
        vec![ // 233 
            Com(1,95),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 234 
            Prm(LE,true),
            Arg(1, true),
            Int(6),
        ], 
        vec![ // 235 
            Com(1,231),
            Arg(0, true),
        ], 
        // AExp130
        vec![ // 236 
            Prm(Sub,false),
            Ptr(0, true, true),
        ], 
        vec![ // 237 
            Prm(Mul,false),
            Arg(0, true),
            Int(10),
        ], 
        // AExp131
        vec![ // 238 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 239 
            Com(1,236),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp132
        vec![ // 240 
            Com(2,0),
        ], 
        // AExp133
        vec![ // 241 
            Com(2,0),
        ], 
        // AExp134
        vec![ // 242 
            Arg(0, true),
            Arg(2, false),
            Ptr(0, true, true),
        ], 
        vec![ // 243 
            Arg(1, true),
            Arg(2, false),
        ], 
        // AExp135
        vec![ // 244 
            Arg(1, true),
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp136
        vec![ // 245 
            Com(4,3),
            Ptr(0, true, true),
        ], 
        vec![ // 246 
            Com(1,0),
            Arg(0, true),
        ], 
        // AExp137
        vec![ // 247 
            Prm(Add,false),
            Int(48),
            Ptr(0, true, true),
        ], 
        vec![ // 248 
            Arg(0, true),
            Com(2,1),
        ], 
        // AExp138
        vec![ // 249 
            Com(1,245),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 250 
            Com(1,247),
            Arg(1, true),
        ], 
        // AExp139
        vec![ // 251 
            Prm(EQ,false),
            Arg(3, false),
            Int(0),
            Com(3,244),
            Com(3,0),
            Ptr(0, true, true),
            Arg(0, true),
            Arg(3, false),
        ], 
        vec![ // 252 
            Com(2,249),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp140
        vec![ // 253 
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp141
        vec![ // 254 
            Com(3,242),
            Ptr(1, true, true),
            Com(1,253),
            Ptr(0, true, true),
        ], 
        vec![ // 255 
            Com(1,271),
            Arg(1, true),
            Int(10),
        ], 
        vec![ // 256 
            Com(4,251),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp142
        vec![ // 257 
            Y,
            Com(3,254),
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp143
        vec![ // 258 
            Arg(2, true),
            Int(0),
            Arg(0, true),
        ], 
        // AExp144
        vec![ // 259 
            Arg(2, true),
            Int(1),
            Ptr(0, true, true),
        ], 
        vec![ // 260 
            Prm(Sub,false),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp145
        vec![ // 261 
            Prm(LE,false),
            Arg(2, false),
            Arg(0, false),
            Com(3,258),
            Com(3,259),
            Arg(0, false),
            Arg(2, false),
        ], 
        // AExp146
        vec![ // 262 
            Arg(3, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 263 
            Prm(Sub,false),
            Arg(1, true),
            Arg(2, true),
        ], 
        vec![ // 264 
            Prm(Add,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp147
        vec![ // 265 
            Prm(LE,false),
            Arg(0, false),
            Arg(2, false),
            Com(4,3),
            Com(4,262),
            Ptr(0, true, true),
            Arg(2, false),
            Arg(0, false),
        ], 
        vec![ // 266 
            Prm(Add,false),
            Arg(1, false),
            Arg(1, false),
        ], 
        // AExp148
        vec![ // 267 
            Com(1,271),
            Arg(0, true),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 268 
            Com(3,265),
            Arg(2, true),
        ], 
        // AExp149
        vec![ // 269 
            Prm(LE,false),
            Arg(2, false),
            Arg(0, false),
            Com(3,261),
            Com(3,267),
            Arg(0, false),
            Arg(2, false),
            Arg(1, true),
        ], 
        // AExp150
        vec![ // 270 
            Prm(Add,false),
            Arg(0, false),
            Arg(0, false),
        ], 
        // AExp151
        vec![ // 271 
            Com(3,242),
            Ptr(0, true, true),
            Com(1,270),
        ], 
        vec![ // 272 
            Com(3,269),
            Arg(0, true),
        ], 
        // AExp152
        vec![ // 273 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 274 
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp153
        vec![ // 275 
            Arg(1, true),
            Ptr(1, true, true),
            Arg(3, false),
            Ptr(0, true, true),
        ], 
        vec![ // 276 
            Prm(Add,false),
            Arg(0, true),
            Arg(3, false),
        ], 
        vec![ // 277 
            Prm(Sub,false),
            Arg(2, true),
            Int(1),
        ], 
        // AExp154
        vec![ // 278 
            Prm(EQ,false),
            Arg(1, false),
            Int(0),
            Com(4,275),
            Com(4,0),
            Arg(2, true),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp155
        vec![ // 279 
            Com(3,273),
            Ptr(0, true, true),
            Com(1,0),
        ], 
        vec![ // 280 
            Com(3,278),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp156
        vec![ // 281 
            Y,
            Com(3,279),
            Arg(0, true),
            Int(0),
            Int(1),
        ], 
        // AExp157
        vec![ // 282 
            Prm(LE,true),
            Arg(0, true),
            Int(32),
        ], 
        // AExp158
        vec![ // 283 
            Com(2,314),
            Ptr(0, true, true),
        ], 
        vec![ // 284 
            Com(1,326),
            Arg(0, true),
        ], 
        // AExp159
        vec![ // 285 
            Com(1,292),
            Ptr(0, true, true),
        ], 
        vec![ // 286 
            Com(1,283),
            Arg(0, true),
        ], 
        // AExp160
        vec![ // 287 
            Com(2,0),
        ], 
        // AExp161
        vec![ // 288 
            Com(4,3),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 289 
            Arg(3, true),
            Arg(1, true),
        ], 
        vec![ // 290 
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp162
        vec![ // 291 
            Arg(2, true),
            Com(2,287),
            Com(4,288),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp163
        vec![ // 292 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 293 
            Com(3,291),
            Arg(0, true),
        ], 
        // AExp164
        vec![ // 294 
            Prm(Add,false),
            Int(65),
            Ptr(0, true, true),
        ], 
        vec![ // 295 
            Com(2,318),
            Arg(0, true),
            Int(26),
        ], 
        // AExp165
        vec![ // 296 
            Prm(Add,false),
            Ptr(0, true, true),
        ], 
        vec![ // 297 
            Prm(Sub,false),
            Arg(0, true),
            Int(65),
        ], 
        // AExp166
        vec![ // 298 
            Com(1,294),
            Ptr(0, true, true),
        ], 
        vec![ // 299 
            Com(1,296),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp167
        vec![ // 300 
            Com(1,0),
            Ptr(0, true, true),
        ], 
        vec![ // 301 
            Com(2,298),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp168
        vec![ // 302 
            Prm(Add,false),
            Int(97),
            Ptr(0, true, true),
        ], 
        vec![ // 303 
            Com(2,318),
            Arg(0, true),
            Int(26),
        ], 
        // AExp169
        vec![ // 304 
            Prm(Add,false),
            Ptr(0, true, true),
        ], 
        vec![ // 305 
            Prm(Sub,false),
            Arg(0, true),
            Int(97),
        ], 
        // AExp170
        vec![ // 306 
            Com(1,302),
            Ptr(0, true, true),
        ], 
        vec![ // 307 
            Com(1,304),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp171
        vec![ // 308 
            Com(1,0),
            Ptr(0, true, true),
        ], 
        vec![ // 309 
            Com(2,306),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp172
        vec![ // 310 
            Com(1,95),
            Ptr(2, true, true),
            Ptr(1, true, true),
            Com(3,24),
            Com(3,308),
            Arg(1, false),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 311 
            Com(1,0),
            Arg(1, false),
        ], 
        vec![ // 312 
            Prm(LE,false),
            Arg(1, false),
            Int(122),
        ], 
        vec![ // 313 
            Prm(LT,true),
            Arg(1, false),
            Int(97),
        ], 
        // AExp173
        vec![ // 314 
            Com(1,95),
            Ptr(2, true, true),
            Ptr(1, true, true),
            Com(3,24),
            Com(3,300),
            Arg(1, false),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 315 
            Com(2,310),
            Arg(0, false),
            Arg(1, false),
        ], 
        vec![ // 316 
            Prm(LE,false),
            Arg(1, false),
            Int(90),
        ], 
        vec![ // 317 
            Prm(LT,true),
            Arg(1, false),
            Int(65),
        ], 
        // AExp174
        vec![ // 318 
            Com(1,271),
            Arg(0, true),
            Arg(1, true),
            Com(2,1),
        ], 
        // AExp175
        vec![ // 319 
            Prm(Sub,false),
            Int(0),
            Ptr(0, true, true),
        ], 
        vec![ // 320 
            Prm(Add,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp176
        vec![ // 321 
            Com(2,318),
            Ptr(0, true, true),
            Int(26),
        ], 
        vec![ // 322 
            Com(1,319),
            Arg(0, true),
        ], 
        // AExp177
        vec![ // 323 
            Prm(Sub,false),
            Int(25),
            Ptr(0, true, true),
        ], 
        vec![ // 324 
            Com(1,321),
            Arg(0, true),
        ], 
        // AExp178
        vec![ // 325 
            Com(2,318),
            Arg(0, true),
            Int(26),
        ], 
        // AExp179
        vec![ // 326 
            Prm(LT,true),
            Arg(0, false),
            Int(0),
            Com(1,323),
            Com(1,325),
            Arg(0, false),
        ], 
    ],

}});