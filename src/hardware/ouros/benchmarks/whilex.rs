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
            Com(3,14),
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
            Com(5,155),
            Int(1),
        ], 
        vec![ // 8 
            Com(5,150),
            Int(4),
        ], 
        vec![ // 9 
            Com(6,158),
            Ptr(8, false, false),
            Ptr(7, false, false),
        ], 
        vec![ // 10 
            Com(7,84),
            Int(4),
            Ptr(9, false, false),
        ], 
        vec![ // 11 
            Com(5,155),
            Int(1),
        ], 
        vec![ // 12 
            Com(5,150),
            Int(5),
        ], 
        vec![ // 13 
            Com(6,84),
            Ptr(12, false, false),
            Ptr(11, false, false),
        ], 
        vec![ // 14 
            Com(7,84),
            Int(5),
            Ptr(13, false, false),
        ], 
        vec![ // 15 
            Com(5,155),
            Int(0),
        ], 
        vec![ // 16 
            Com(5,150),
            Int(0),
        ], 
        vec![ // 17 
            Com(3,153),
            Ptr(16, false, false),
            Ptr(15, false, false),
        ], 
        vec![ // 18 
            Com(4,147),
            Ptr(17, false, false),
            Ptr(14, false, false),
            Com(5,148),
        ], 
        vec![ // 19 
            Com(5,155),
            Int(1),
        ], 
        vec![ // 20 
            Com(5,150),
            Int(2),
        ], 
        vec![ // 21 
            Com(6,84),
            Ptr(20, false, false),
            Ptr(19, false, false),
        ], 
        vec![ // 22 
            Com(7,84),
            Int(2),
            Ptr(21, false, false),
        ], 
        vec![ // 23 
            Com(5,150),
            Int(1),
        ], 
        vec![ // 24 
            Com(5,150),
            Int(0),
        ], 
        vec![ // 25 
            Com(6,158),
            Ptr(24, false, false),
            Ptr(23, false, false),
        ], 
        vec![ // 26 
            Com(7,84),
            Int(0),
            Ptr(25, false, false),
        ], 
        vec![ // 27 
            Com(7,2),
            Ptr(26, false, false),
            Ptr(22, false, false),
        ], 
        vec![ // 28 
            Com(5,150),
            Int(0),
        ], 
        vec![ // 29 
            Com(5,150),
            Int(1),
        ], 
        vec![ // 30 
            Com(3,156),
            Ptr(29, false, false),
            Ptr(28, false, false),
        ], 
        vec![ // 31 
            Com(7,149),
            Ptr(30, false, false),
        ], 
        vec![ // 32 
            Com(1,0),
            Ptr(31, false, false),
            Ptr(27, false, false),
        ], 
        vec![ // 33 
            Com(5,150),
            Int(4),
        ], 
        vec![ // 34 
            Com(7,84),
            Int(1),
            Ptr(33, false, false),
        ], 
        vec![ // 35 
            Com(5,150),
            Int(3),
        ], 
        vec![ // 36 
            Com(7,84),
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
            Com(5,155),
            Int(0),
        ], 
        vec![ // 43 
            Com(5,150),
            Int(4),
        ], 
        vec![ // 44 
            Com(3,153),
            Ptr(43, false, false),
            Ptr(42, false, false),
        ], 
        vec![ // 45 
            Com(2,151),
            Ptr(44, false, false),
        ], 
        vec![ // 46 
            Com(7,149),
            Ptr(45, false, false),
        ], 
        vec![ // 47 
            Com(1,0),
            Ptr(46, false, false),
            Ptr(41, false, false),
        ], 
        vec![ // 48 
            Com(5,150),
            Int(3),
        ], 
        vec![ // 49 
            Com(7,84),
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
            Com(2,16),
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
            Prm(EQ,false),
            Arg(3, true),
            Arg(0, false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 10 
            Arg(1, false),
            Arg(4, true),
        ], 
        vec![ // 11 
            Com(3,14),
            Arg(2, true),
            Arg(0, false),
            Arg(1, false),
        ], 
        // AExp10
        vec![ // 12 
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 13 
            Com(5,9),
            Arg(0, true),
            Arg(1, true),
            Arg(3, true),
        ], 
        // AExp11
        vec![ // 14 
            Arg(0, true),
            Err(42),
            Ptr(0, true, true),
        ], 
        vec![ // 15 
            Com(4,12),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp12
        vec![ // 16 
            Com(1,20),
            Ptr(0, true, true),
        ], 
        vec![ // 17 
            Com(4,2),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp13
        vec![ // 18 
            Com(1,20),
            Ptr(0, true, true),
        ], 
        vec![ // 19 
            Com(2,36),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp14
        vec![ // 20 
            Arg(0, true),
            Com(1,0),
            Com(2,18),
        ], 
        // AExp15
        vec![ // 21 
            Com(3,52),
            Arg(2, true),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 22 
            Com(4,79),
            Arg(0, false),
            Arg(1, true),
            Com(3,42),
        ], 
        // AExp16
        vec![ // 23 
            Com(4,2),
            Ptr(0, true, true),
        ], 
        vec![ // 24 
            Com(7,2),
            Arg(1, true),
            Arg(0, true),
        ], 
        // AExp17
        vec![ // 25 
            Com(2,36),
            Arg(1, true),
            Arg(0, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 26 
            Com(2,23),
            Arg(2, false),
        ], 
        vec![ // 27 
            Com(4,2),
            Arg(2, false),
        ], 
        // AExp18
        vec![ // 28 
            Com(3,128),
            Arg(1, true),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 29 
            Com(4,143),
            Arg(0, false),
            Arg(2, true),
            Arg(3, true),
        ], 
        // AExp19
        vec![ // 30 
            Com(7,2),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 31 
            Com(7,149),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp20
        vec![ // 32 
            Com(4,147),
            Arg(0, false),
            Ptr(0, true, true),
            Com(5,148),
        ], 
        vec![ // 33 
            Com(2,30),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp21
        vec![ // 34 
            Com(4,2),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 35 
            Com(2,32),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp22
        vec![ // 36 
            Arg(0, true),
            Ptr(4, true, true),
            Ptr(3, true, true),
            Ptr(2, true, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 37 
            Com(3,34),
            Arg(1, false),
        ], 
        vec![ // 38 
            Com(3,42),
            Arg(1, false),
        ], 
        vec![ // 39 
            Com(4,28),
            Arg(1, false),
        ], 
        vec![ // 40 
            Com(3,25),
            Arg(1, false),
        ], 
        vec![ // 41 
            Com(3,21),
            Arg(1, false),
        ], 
        // AExp23
        vec![ // 42 
            Arg(1, true),
            Arg(0, true),
        ], 
        // AExp24
        vec![ // 43 
            Com(3,56),
            Ptr(2, true, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 44 
            Com(3,60),
            Arg(1, true),
        ], 
        vec![ // 45 
            Com(3,52),
            Arg(3, true),
            Arg(0, false),
        ], 
        vec![ // 46 
            Com(3,52),
            Arg(2, true),
            Arg(0, false),
        ], 
        // AExp25
        vec![ // 47 
            Com(3,56),
            Ptr(2, true, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 48 
            Com(3,65),
            Arg(1, true),
        ], 
        vec![ // 49 
            Com(3,52),
            Arg(3, true),
            Arg(0, false),
        ], 
        vec![ // 50 
            Com(3,52),
            Arg(2, true),
            Arg(0, false),
        ], 
        // AExp26
        vec![ // 51 
            Com(3,14),
            Arg(0, true),
            Arg(2, true),
            Arg(1, true),
        ], 
        // AExp27
        vec![ // 52 
            Arg(0, true),
            Ptr(2, true, true),
            Arg(2, false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 53 
            Com(3,51),
            Arg(1, false),
            Arg(2, false),
        ], 
        vec![ // 54 
            Com(4,47),
            Arg(1, false),
            Arg(2, false),
        ], 
        vec![ // 55 
            Com(4,43),
            Arg(1, false),
            Arg(2, false),
        ], 
        // AExp28
        vec![ // 56 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 57 
            Com(3,58),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp29
        vec![ // 58 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 59 
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp30
        vec![ // 60 
            Com(2,62),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 61 
            Prm(Add,false),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp31
        vec![ // 62 
            Prm(EQ,false),
            Arg(0, false),
            Int(0),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 63 
            Arg(1, false),
            Int(0),
        ], 
        vec![ // 64 
            Arg(1, false),
            Arg(0, false),
        ], 
        // AExp32
        vec![ // 65 
            Com(2,62),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 66 
            Prm(Sub,false),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp33
        vec![ // 67 
            Prm(EQ,false),
            Arg(5, false),
            Arg(0, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 68 
            Arg(2, true),
            Arg(3, false),
            Arg(4, false),
        ], 
        vec![ // 69 
            Arg(1, true),
            Arg(3, false),
            Arg(4, false),
            Arg(5, false),
            Arg(6, true),
        ], 
        // AExp34
        vec![ // 70 
            Com(4,79),
            Arg(3, true),
            Arg(0, true),
            Ptr(0, true, true),
            Arg(2, true),
        ], 
        vec![ // 71 
            Com(3,85),
            Arg(1, true),
            Arg(4, true),
            Arg(5, true),
        ], 
        // AExp35
        vec![ // 72 
            Com(4,79),
            Arg(3, true),
            Arg(0, false),
            Ptr(0, true, true),
            Arg(2, false),
        ], 
        vec![ // 73 
            Com(3,85),
            Arg(1, true),
            Arg(0, false),
            Arg(2, false),
        ], 
        // AExp36
        vec![ // 74 
            Com(7,67),
            Arg(0, false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 75 
            Com(4,72),
            Arg(0, false),
            Arg(1, false),
        ], 
        vec![ // 76 
            Com(6,70),
            Arg(0, false),
            Arg(1, false),
        ], 
        // AExp37
        vec![ // 77 
            Arg(3, true),
            Ptr(0, true, true),
        ], 
        vec![ // 78 
            Com(2,74),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
            Arg(4, true),
        ], 
        // AExp38
        vec![ // 79 
            Arg(0, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 80 
            Com(5,77),
            Arg(1, true),
            Arg(2, false),
            Arg(3, true),
        ], 
        vec![ // 81 
            Arg(2, false),
            Com(2,0),
        ], 
        // AExp39
        vec![ // 82 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 83 
            Com(4,2),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp40
        vec![ // 84 
            Arg(2, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp41
        vec![ // 85 
            Com(3,82),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 86 
            Com(3,84),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp42
        vec![ // 87 
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 88 
            Arg(0, true),
            Com(2,1),
        ], 
        // AExp43
        vec![ // 89 
            Prm(LT,false),
            Arg(2, true),
            Int(5),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 90 
            Arg(3, false),
            Arg(1, true),
        ], 
        vec![ // 91 
            Com(2,87),
            Arg(0, true),
            Arg(3, false),
        ], 
        // AExp44
        vec![ // 92 
            Com(3,128),
            Arg(2, true),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 93 
            Com(2,130),
            Arg(1, true),
        ], 
        // AExp45
        vec![ // 94 
            Com(4,89),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 95 
            Com(3,92),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp46
        vec![ // 96 
            Prm(LT,false),
            Arg(3, false),
            Int(4),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 97 
            Arg(4, false),
            Arg(2, true),
        ], 
        vec![ // 98 
            Com(2,94),
            Arg(0, true),
            Arg(1, true),
            Arg(3, false),
            Arg(4, false),
        ], 
        // AExp47
        vec![ // 99 
            Com(3,56),
            Ptr(2, true, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 100 
            Com(3,136),
            Arg(1, true),
        ], 
        vec![ // 101 
            Com(3,52),
            Arg(3, true),
            Arg(0, false),
        ], 
        vec![ // 102 
            Com(3,52),
            Arg(2, true),
            Arg(0, false),
        ], 
        // AExp48
        vec![ // 103 
            Com(5,96),
            Arg(0, false),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 104 
            Com(4,99),
            Arg(0, false),
            Arg(1, false),
        ], 
        // AExp49
        vec![ // 105 
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 106 
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp50
        vec![ // 107 
            Prm(LT,false),
            Arg(2, true),
            Int(2),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 108 
            Arg(3, false),
            Arg(1, true),
        ], 
        vec![ // 109 
            Com(2,105),
            Arg(0, true),
            Arg(3, false),
        ], 
        // AExp51
        vec![ // 110 
            Com(3,56),
            Ptr(2, true, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 111 
            Com(3,138),
            Arg(1, true),
        ], 
        vec![ // 112 
            Com(3,52),
            Arg(3, true),
            Arg(0, false),
        ], 
        vec![ // 113 
            Com(3,52),
            Arg(2, true),
            Arg(0, false),
        ], 
        // AExp52
        vec![ // 114 
            Com(4,107),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 115 
            Com(4,110),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp53
        vec![ // 116 
            Prm(LT,false),
            Arg(3, false),
            Int(1),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 117 
            Arg(4, false),
            Arg(2, true),
        ], 
        vec![ // 118 
            Com(2,114),
            Arg(0, true),
            Arg(1, true),
            Arg(3, false),
            Arg(4, false),
        ], 
        // AExp54
        vec![ // 119 
            Com(3,56),
            Ptr(2, true, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 120 
            Com(3,140),
            Arg(1, true),
        ], 
        vec![ // 121 
            Com(3,128),
            Arg(3, true),
            Arg(0, false),
        ], 
        vec![ // 122 
            Com(3,128),
            Arg(2, true),
            Arg(0, false),
        ], 
        // AExp55
        vec![ // 123 
            Com(5,116),
            Arg(0, false),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 124 
            Com(4,119),
            Arg(0, false),
            Arg(1, false),
        ], 
        // AExp56
        vec![ // 125 
            Prm(LT,false),
            Arg(2, false),
            Int(3),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 126 
            Com(2,123),
            Arg(0, false),
            Arg(1, false),
            Arg(2, false),
            Arg(3, false),
        ], 
        vec![ // 127 
            Com(2,103),
            Arg(0, false),
            Arg(1, false),
            Arg(2, false),
            Arg(3, false),
        ], 
        // AExp57
        vec![ // 128 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 129 
            Com(4,125),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp58
        vec![ // 130 
            Com(2,132),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 131 
            Com(1,135),
            Arg(1, true),
        ], 
        // AExp59
        vec![ // 132 
            Arg(0, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 133 
            Arg(1, false),
            Com(2,1),
        ], 
        vec![ // 134 
            Arg(1, false),
            Com(2,0),
        ], 
        // AExp60
        vec![ // 135 
            Arg(0, true),
            Com(2,1),
            Com(2,0),
        ], 
        // AExp61
        vec![ // 136 
            Com(2,132),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 137 
            Prm(LE,false),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp62
        vec![ // 138 
            Com(2,132),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 139 
            Prm(EQ,false),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp63
        vec![ // 140 
            Com(2,132),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 141 
            Com(1,142),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp64
        vec![ // 142 
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp65
        vec![ // 143 
            Arg(3, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 144 
            Com(4,2),
            Arg(1, true),
            Arg(0, false),
        ], 
        vec![ // 145 
            Com(4,2),
            Arg(2, true),
            Arg(0, false),
        ], 
        // AExp66
        vec![ // 146 
            Arg(4, true),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp67
        vec![ // 147 
            Com(7,146),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp68
        vec![ // 148 
            Arg(3, true),
        ], 
        // AExp69
        vec![ // 149 
            Arg(6, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp70
        vec![ // 150 
            Arg(4, true),
            Arg(0, true),
        ], 
        // AExp71
        vec![ // 151 
            Arg(1, true),
            Int(4),
            Ptr(0, true, true),
        ], 
        vec![ // 152 
            Com(2,42),
            Arg(0, true),
        ], 
        // AExp72
        vec![ // 153 
            Arg(2, true),
            Int(1),
            Ptr(0, true, true),
        ], 
        vec![ // 154 
            Com(3,84),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp73
        vec![ // 155 
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp74
        vec![ // 156 
            Arg(2, true),
            Int(3),
            Ptr(0, true, true),
        ], 
        vec![ // 157 
            Com(3,84),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp75
        vec![ // 158 
            Arg(4, true),
            Arg(0, true),
            Arg(1, true),
        ], 
    ],

}});
