use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 76
#[rustfmt::skip]
pub static WHILEX: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            COM(3,14),
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
            COM(5,155),
            INT(1),
        ], 
        vec![ // 8 
            COM(5,150),
            INT(4),
        ], 
        vec![ // 9 
            COM(6,158),
            PTR(8, false, false),
            PTR(7, false, false),
        ], 
        vec![ // 10 
            COM(7,84),
            INT(4),
            PTR(9, false, false),
        ], 
        vec![ // 11 
            COM(5,155),
            INT(1),
        ], 
        vec![ // 12 
            COM(5,150),
            INT(5),
        ], 
        vec![ // 13 
            COM(6,84),
            PTR(12, false, false),
            PTR(11, false, false),
        ], 
        vec![ // 14 
            COM(7,84),
            INT(5),
            PTR(13, false, false),
        ], 
        vec![ // 15 
            COM(5,155),
            INT(0),
        ], 
        vec![ // 16 
            COM(5,150),
            INT(0),
        ], 
        vec![ // 17 
            COM(3,153),
            PTR(16, false, false),
            PTR(15, false, false),
        ], 
        vec![ // 18 
            COM(4,147),
            PTR(17, false, false),
            PTR(14, false, false),
            COM(5,148),
        ], 
        vec![ // 19 
            COM(5,155),
            INT(1),
        ], 
        vec![ // 20 
            COM(5,150),
            INT(2),
        ], 
        vec![ // 21 
            COM(6,84),
            PTR(20, false, false),
            PTR(19, false, false),
        ], 
        vec![ // 22 
            COM(7,84),
            INT(2),
            PTR(21, false, false),
        ], 
        vec![ // 23 
            COM(5,150),
            INT(1),
        ], 
        vec![ // 24 
            COM(5,150),
            INT(0),
        ], 
        vec![ // 25 
            COM(6,158),
            PTR(24, false, false),
            PTR(23, false, false),
        ], 
        vec![ // 26 
            COM(7,84),
            INT(0),
            PTR(25, false, false),
        ], 
        vec![ // 27 
            COM(7,2),
            PTR(26, false, false),
            PTR(22, false, false),
        ], 
        vec![ // 28 
            COM(5,150),
            INT(0),
        ], 
        vec![ // 29 
            COM(5,150),
            INT(1),
        ], 
        vec![ // 30 
            COM(3,156),
            PTR(29, false, false),
            PTR(28, false, false),
        ], 
        vec![ // 31 
            COM(7,149),
            PTR(30, false, false),
        ], 
        vec![ // 32 
            COM(1,0),
            PTR(31, false, false),
            PTR(27, false, false),
        ], 
        vec![ // 33 
            COM(5,150),
            INT(4),
        ], 
        vec![ // 34 
            COM(7,84),
            INT(1),
            PTR(33, false, false),
        ], 
        vec![ // 35 
            COM(5,150),
            INT(3),
        ], 
        vec![ // 36 
            COM(7,84),
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
            COM(5,155),
            INT(0),
        ], 
        vec![ // 43 
            COM(5,150),
            INT(4),
        ], 
        vec![ // 44 
            COM(3,153),
            PTR(43, false, false),
            PTR(42, false, false),
        ], 
        vec![ // 45 
            COM(2,151),
            PTR(44, false, false),
        ], 
        vec![ // 46 
            COM(7,149),
            PTR(45, false, false),
        ], 
        vec![ // 47 
            COM(1,0),
            PTR(46, false, false),
            PTR(41, false, false),
        ], 
        vec![ // 48 
            COM(5,150),
            INT(3),
        ], 
        vec![ // 49 
            COM(7,84),
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
    ],
    comb_img: vec![
        // AExp0
        vec![ // 0 
            ARG(0, true),
        ], 
        // AExp1
        vec![ // 1 
            ARG(1, true),
        ], 
        // AExp2
        vec![ // 2 
            ARG(3, true),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp3
        vec![ // 3 
            ARG(0, true),
            INT(0),
            INT(0),
        ], 
        // AExp4
        vec![ // 4 
            ARG(0, true),
            INT(1),
            INT(0),
        ], 
        // AExp5
        vec![ // 5 
            ARG(0, true),
            INT(2),
            INT(0),
        ], 
        // AExp6
        vec![ // 6 
            ARG(0, true),
            INT(3),
            INT(14000),
        ], 
        // AExp7
        vec![ // 7 
            ARG(0, true),
            INT(4),
            INT(0),
        ], 
        // AExp8
        vec![ // 8 
            ARG(0, true),
            INT(5),
            INT(0),
        ], 
        // AExp9
        vec![ // 9 
            PRM(EQ,false),
            ARG(3, true),
            ARG(0, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 10 
            ARG(1, false),
            ARG(4, true),
        ], 
        vec![ // 11 
            COM(3,14),
            ARG(2, true),
            ARG(0, false),
            ARG(1, false),
        ], 
        // AExp10
        vec![ // 12 
            ARG(2, true),
            PTR(0, true, true),
        ], 
        vec![ // 13 
            COM(5,9),
            ARG(0, true),
            ARG(1, true),
            ARG(3, true),
        ], 
        // AExp11
        vec![ // 14 
            ARG(0, true),
            ERR(42),
            PTR(0, true, true),
        ], 
        vec![ // 15 
            COM(4,12),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp12
        vec![ // 16 
            COM(1,20),
            PTR(0, true, true),
        ], 
        vec![ // 17 
            COM(4,2),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp13
        vec![ // 18 
            COM(1,20),
            PTR(0, true, true),
        ], 
        vec![ // 19 
            COM(2,36),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp14
        vec![ // 20 
            ARG(0, true),
            COM(1,0),
            COM(2,18),
        ], 
        // AExp15
        vec![ // 21 
            COM(3,52),
            ARG(2, true),
            ARG(0, false),
            PTR(0, true, true),
        ], 
        vec![ // 22 
            COM(4,79),
            ARG(0, false),
            ARG(1, true),
            COM(3,42),
        ], 
        // AExp16
        vec![ // 23 
            COM(4,2),
            PTR(0, true, true),
        ], 
        vec![ // 24 
            COM(7,2),
            ARG(1, true),
            ARG(0, true),
        ], 
        // AExp17
        vec![ // 25 
            COM(2,36),
            ARG(1, true),
            ARG(0, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 26 
            COM(2,23),
            ARG(2, false),
        ], 
        vec![ // 27 
            COM(4,2),
            ARG(2, false),
        ], 
        // AExp18
        vec![ // 28 
            COM(3,128),
            ARG(1, true),
            ARG(0, false),
            PTR(0, true, true),
        ], 
        vec![ // 29 
            COM(4,143),
            ARG(0, false),
            ARG(2, true),
            ARG(3, true),
        ], 
        // AExp19
        vec![ // 30 
            COM(7,2),
            ARG(1, false),
            PTR(0, true, true),
        ], 
        vec![ // 31 
            COM(7,149),
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp20
        vec![ // 32 
            COM(4,147),
            ARG(0, false),
            PTR(0, true, true),
            COM(5,148),
        ], 
        vec![ // 33 
            COM(2,30),
            ARG(0, false),
            ARG(1, true),
        ], 
        // AExp21
        vec![ // 34 
            COM(4,2),
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 35 
            COM(2,32),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp22
        vec![ // 36 
            ARG(0, true),
            PTR(4, true, true),
            PTR(3, true, true),
            PTR(2, true, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 37 
            COM(3,34),
            ARG(1, false),
        ], 
        vec![ // 38 
            COM(3,42),
            ARG(1, false),
        ], 
        vec![ // 39 
            COM(4,28),
            ARG(1, false),
        ], 
        vec![ // 40 
            COM(3,25),
            ARG(1, false),
        ], 
        vec![ // 41 
            COM(3,21),
            ARG(1, false),
        ], 
        // AExp23
        vec![ // 42 
            ARG(1, true),
            ARG(0, true),
        ], 
        // AExp24
        vec![ // 43 
            COM(3,56),
            PTR(2, true, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 44 
            COM(3,60),
            ARG(1, true),
        ], 
        vec![ // 45 
            COM(3,52),
            ARG(3, true),
            ARG(0, false),
        ], 
        vec![ // 46 
            COM(3,52),
            ARG(2, true),
            ARG(0, false),
        ], 
        // AExp25
        vec![ // 47 
            COM(3,56),
            PTR(2, true, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 48 
            COM(3,65),
            ARG(1, true),
        ], 
        vec![ // 49 
            COM(3,52),
            ARG(3, true),
            ARG(0, false),
        ], 
        vec![ // 50 
            COM(3,52),
            ARG(2, true),
            ARG(0, false),
        ], 
        // AExp26
        vec![ // 51 
            COM(3,14),
            ARG(0, true),
            ARG(2, true),
            ARG(1, true),
        ], 
        // AExp27
        vec![ // 52 
            ARG(0, true),
            PTR(2, true, true),
            ARG(2, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 53 
            COM(3,51),
            ARG(1, false),
            ARG(2, false),
        ], 
        vec![ // 54 
            COM(4,47),
            ARG(1, false),
            ARG(2, false),
        ], 
        vec![ // 55 
            COM(4,43),
            ARG(1, false),
            ARG(2, false),
        ], 
        // AExp28
        vec![ // 56 
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 57 
            COM(3,58),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp29
        vec![ // 58 
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 59 
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp30
        vec![ // 60 
            COM(2,62),
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 61 
            PRM(ADD,false),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp31
        vec![ // 62 
            PRM(EQ,false),
            ARG(0, false),
            INT(0),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 63 
            ARG(1, false),
            INT(0),
        ], 
        vec![ // 64 
            ARG(1, false),
            ARG(0, false),
        ], 
        // AExp32
        vec![ // 65 
            COM(2,62),
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 66 
            PRM(SUB,false),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp33
        vec![ // 67 
            PRM(EQ,false),
            ARG(5, false),
            ARG(0, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 68 
            ARG(2, true),
            ARG(3, false),
            ARG(4, false),
        ], 
        vec![ // 69 
            ARG(1, true),
            ARG(3, false),
            ARG(4, false),
            ARG(5, false),
            ARG(6, true),
        ], 
        // AExp34
        vec![ // 70 
            COM(4,79),
            ARG(3, true),
            ARG(0, true),
            PTR(0, true, true),
            ARG(2, true),
        ], 
        vec![ // 71 
            COM(3,85),
            ARG(1, true),
            ARG(4, true),
            ARG(5, true),
        ], 
        // AExp35
        vec![ // 72 
            COM(4,79),
            ARG(3, true),
            ARG(0, false),
            PTR(0, true, true),
            ARG(2, false),
        ], 
        vec![ // 73 
            COM(3,85),
            ARG(1, true),
            ARG(0, false),
            ARG(2, false),
        ], 
        // AExp36
        vec![ // 74 
            COM(7,67),
            ARG(0, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 75 
            COM(4,72),
            ARG(0, false),
            ARG(1, false),
        ], 
        vec![ // 76 
            COM(6,70),
            ARG(0, false),
            ARG(1, false),
        ], 
        // AExp37
        vec![ // 77 
            ARG(3, true),
            PTR(0, true, true),
        ], 
        vec![ // 78 
            COM(2,74),
            ARG(0, true),
            ARG(1, true),
            ARG(2, true),
            ARG(4, true),
        ], 
        // AExp38
        vec![ // 79 
            ARG(0, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 80 
            COM(5,77),
            ARG(1, true),
            ARG(2, false),
            ARG(3, true),
        ], 
        vec![ // 81 
            ARG(2, false),
            COM(2,0),
        ], 
        // AExp39
        vec![ // 82 
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 83 
            COM(4,2),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp40
        vec![ // 84 
            ARG(2, true),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp41
        vec![ // 85 
            COM(3,82),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 86 
            COM(3,84),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp42
        vec![ // 87 
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 88 
            ARG(0, true),
            COM(2,1),
        ], 
        // AExp43
        vec![ // 89 
            PRM(LT,false),
            ARG(2, true),
            INT(5),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 90 
            ARG(3, false),
            ARG(1, true),
        ], 
        vec![ // 91 
            COM(2,87),
            ARG(0, true),
            ARG(3, false),
        ], 
        // AExp44
        vec![ // 92 
            COM(3,128),
            ARG(2, true),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 93 
            COM(2,130),
            ARG(1, true),
        ], 
        // AExp45
        vec![ // 94 
            COM(4,89),
            ARG(1, false),
            PTR(0, true, true),
        ], 
        vec![ // 95 
            COM(3,92),
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp46
        vec![ // 96 
            PRM(LT,false),
            ARG(3, false),
            INT(4),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 97 
            ARG(4, false),
            ARG(2, true),
        ], 
        vec![ // 98 
            COM(2,94),
            ARG(0, true),
            ARG(1, true),
            ARG(3, false),
            ARG(4, false),
        ], 
        // AExp47
        vec![ // 99 
            COM(3,56),
            PTR(2, true, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 100 
            COM(3,136),
            ARG(1, true),
        ], 
        vec![ // 101 
            COM(3,52),
            ARG(3, true),
            ARG(0, false),
        ], 
        vec![ // 102 
            COM(3,52),
            ARG(2, true),
            ARG(0, false),
        ], 
        // AExp48
        vec![ // 103 
            COM(5,96),
            ARG(0, false),
            ARG(1, false),
            PTR(0, true, true),
        ], 
        vec![ // 104 
            COM(4,99),
            ARG(0, false),
            ARG(1, false),
        ], 
        // AExp49
        vec![ // 105 
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 106 
            ARG(0, true),
            COM(2,0),
        ], 
        // AExp50
        vec![ // 107 
            PRM(LT,false),
            ARG(2, true),
            INT(2),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 108 
            ARG(3, false),
            ARG(1, true),
        ], 
        vec![ // 109 
            COM(2,105),
            ARG(0, true),
            ARG(3, false),
        ], 
        // AExp51
        vec![ // 110 
            COM(3,56),
            PTR(2, true, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 111 
            COM(3,138),
            ARG(1, true),
        ], 
        vec![ // 112 
            COM(3,52),
            ARG(3, true),
            ARG(0, false),
        ], 
        vec![ // 113 
            COM(3,52),
            ARG(2, true),
            ARG(0, false),
        ], 
        // AExp52
        vec![ // 114 
            COM(4,107),
            ARG(1, false),
            PTR(0, true, true),
        ], 
        vec![ // 115 
            COM(4,110),
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp53
        vec![ // 116 
            PRM(LT,false),
            ARG(3, false),
            INT(1),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 117 
            ARG(4, false),
            ARG(2, true),
        ], 
        vec![ // 118 
            COM(2,114),
            ARG(0, true),
            ARG(1, true),
            ARG(3, false),
            ARG(4, false),
        ], 
        // AExp54
        vec![ // 119 
            COM(3,56),
            PTR(2, true, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 120 
            COM(3,140),
            ARG(1, true),
        ], 
        vec![ // 121 
            COM(3,128),
            ARG(3, true),
            ARG(0, false),
        ], 
        vec![ // 122 
            COM(3,128),
            ARG(2, true),
            ARG(0, false),
        ], 
        // AExp55
        vec![ // 123 
            COM(5,116),
            ARG(0, false),
            ARG(1, false),
            PTR(0, true, true),
        ], 
        vec![ // 124 
            COM(4,119),
            ARG(0, false),
            ARG(1, false),
        ], 
        // AExp56
        vec![ // 125 
            PRM(LT,false),
            ARG(2, false),
            INT(3),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 126 
            COM(2,123),
            ARG(0, false),
            ARG(1, false),
            ARG(2, false),
            ARG(3, false),
        ], 
        vec![ // 127 
            COM(2,103),
            ARG(0, false),
            ARG(1, false),
            ARG(2, false),
            ARG(3, false),
        ], 
        // AExp57
        vec![ // 128 
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 129 
            COM(4,125),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp58
        vec![ // 130 
            COM(2,132),
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 131 
            COM(1,135),
            ARG(1, true),
        ], 
        // AExp59
        vec![ // 132 
            ARG(0, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 133 
            ARG(1, false),
            COM(2,1),
        ], 
        vec![ // 134 
            ARG(1, false),
            COM(2,0),
        ], 
        // AExp60
        vec![ // 135 
            ARG(0, true),
            COM(2,1),
            COM(2,0),
        ], 
        // AExp61
        vec![ // 136 
            COM(2,132),
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 137 
            PRM(LE,false),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp62
        vec![ // 138 
            COM(2,132),
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 139 
            PRM(EQ,false),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp63
        vec![ // 140 
            COM(2,132),
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 141 
            COM(1,142),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp64
        vec![ // 142 
            ARG(0, true),
            COM(2,0),
        ], 
        // AExp65
        vec![ // 143 
            ARG(3, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 144 
            COM(4,2),
            ARG(1, true),
            ARG(0, false),
        ], 
        vec![ // 145 
            COM(4,2),
            ARG(2, true),
            ARG(0, false),
        ], 
        // AExp66
        vec![ // 146 
            ARG(4, true),
            ARG(0, true),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp67
        vec![ // 147 
            COM(7,146),
            ARG(0, true),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp68
        vec![ // 148 
            ARG(3, true),
        ], 
        // AExp69
        vec![ // 149 
            ARG(6, true),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp70
        vec![ // 150 
            ARG(4, true),
            ARG(0, true),
        ], 
        // AExp71
        vec![ // 151 
            ARG(1, true),
            INT(4),
            PTR(0, true, true),
        ], 
        vec![ // 152 
            COM(2,42),
            ARG(0, true),
        ], 
        // AExp72
        vec![ // 153 
            ARG(2, true),
            INT(1),
            PTR(0, true, true),
        ], 
        vec![ // 154 
            COM(3,84),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp73
        vec![ // 155 
            ARG(2, true),
            ARG(0, true),
        ], 
        // AExp74
        vec![ // 156 
            ARG(2, true),
            INT(3),
            PTR(0, true, true),
        ], 
        vec![ // 157 
            COM(3,84),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp75
        vec![ // 158 
            ARG(4, true),
            ARG(0, true),
            ARG(1, true),
        ], 
    ],

}});