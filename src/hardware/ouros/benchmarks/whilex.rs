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
            Com(3,17),
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
            Com(5,131),
            Int(1),
        ], 
        vec![ // 8 
            Com(5,128),
            Int(4),
        ], 
        vec![ // 9 
            Com(6,132),
            Ptr(8, false, false),
            Ptr(7, false, false),
        ], 
        vec![ // 10 
            Com(7,84),
            Int(4),
            Ptr(9, false, false),
        ], 
        vec![ // 11 
            Com(5,131),
            Int(1),
        ], 
        vec![ // 12 
            Com(5,128),
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
            Com(5,131),
            Int(0),
        ], 
        vec![ // 16 
            Com(5,128),
            Int(0),
        ], 
        vec![ // 17 
            Com(3,130),
            Ptr(16, false, false),
            Ptr(15, false, false),
        ], 
        vec![ // 18 
            Com(4,125),
            Ptr(17, false, false),
            Ptr(14, false, false),
            Com(5,126),
        ], 
        vec![ // 19 
            Com(5,131),
            Int(1),
        ], 
        vec![ // 20 
            Com(5,128),
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
            Com(5,128),
            Int(1),
        ], 
        vec![ // 24 
            Com(5,128),
            Int(0),
        ], 
        vec![ // 25 
            Com(6,132),
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
            Com(5,128),
            Int(0),
        ], 
        vec![ // 29 
            Com(5,128),
            Int(1),
        ], 
        vec![ // 30 
            Com(3,133),
            Ptr(29, false, false),
            Ptr(28, false, false),
        ], 
        vec![ // 31 
            Com(7,127),
            Ptr(30, false, false),
        ], 
        vec![ // 32 
            Com(1,0),
            Ptr(31, false, false),
            Ptr(27, false, false),
        ], 
        vec![ // 33 
            Com(5,128),
            Int(4),
        ], 
        vec![ // 34 
            Com(7,84),
            Int(1),
            Ptr(33, false, false),
        ], 
        vec![ // 35 
            Com(5,128),
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
            Com(5,131),
            Int(0),
        ], 
        vec![ // 43 
            Com(5,128),
            Int(4),
        ], 
        vec![ // 44 
            Com(3,130),
            Ptr(43, false, false),
            Ptr(42, false, false),
        ], 
        vec![ // 45 
            Com(7,129),
            Ptr(44, false, false),
        ], 
        vec![ // 46 
            Com(7,127),
            Ptr(45, false, false),
        ], 
        vec![ // 47 
            Com(1,0),
            Ptr(46, false, false),
            Ptr(41, false, false),
        ], 
        vec![ // 48 
            Com(5,128),
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
            Arg(0, true),
            Arg(2, true),
            Arg(1, true),
        ], 
        // AExp10
        vec![ // 10 
            Com(3,17),
            Arg(2, true),
            Arg(3, true),
            Arg(0, true),
        ], 
        // AExp11
        vec![ // 11 
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp12
        vec![ // 12 
            Prm(EQ,false),
            Arg(3, true),
            Arg(0, true),
            Com(4,10),
            Com(4,11),
            Arg(1, true),
            Arg(4, true),
            Arg(2, true),
        ], 
        // AExp13
        vec![ // 13 
            Com(3,9),
            Ptr(0, true, true),
            Arg(0, false),
        ], 
        vec![ // 14 
            Com(5,12),
            Arg(0, false),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        // AExp14
        vec![ // 15 
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 16 
            Com(4,13),
            Arg(0, true),
            Arg(1, true),
            Arg(3, true),
        ], 
        // AExp15
        vec![ // 17 
            Arg(0, true),
            Err(42),
            Ptr(0, true, true),
        ], 
        vec![ // 18 
            Com(4,15),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp16
        vec![ // 19 
            Com(1,23),
            Ptr(0, true, true),
        ], 
        vec![ // 20 
            Com(4,2),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp17
        vec![ // 21 
            Com(1,23),
            Ptr(0, true, true),
        ], 
        vec![ // 22 
            Com(2,39),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp18
        vec![ // 23 
            Arg(0, true),
            Com(1,0),
            Com(2,21),
        ], 
        // AExp19
        vec![ // 24 
            Com(3,55),
            Arg(2, true),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 25 
            Com(4,79),
            Arg(0, false),
            Arg(1, true),
            Com(3,45),
        ], 
        // AExp20
        vec![ // 26 
            Com(4,2),
            Ptr(0, true, true),
        ], 
        vec![ // 27 
            Com(7,2),
            Arg(1, true),
            Arg(0, true),
        ], 
        // AExp21
        vec![ // 28 
            Com(2,39),
            Arg(1, true),
            Arg(0, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 29 
            Com(2,26),
            Arg(2, false),
        ], 
        vec![ // 30 
            Com(4,2),
            Arg(2, false),
        ], 
        // AExp22
        vec![ // 31 
            Com(3,101),
            Arg(1, true),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 32 
            Com(4,123),
            Arg(0, false),
            Arg(2, true),
            Arg(3, true),
        ], 
        // AExp23
        vec![ // 33 
            Com(7,2),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 34 
            Com(7,127),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp24
        vec![ // 35 
            Com(4,125),
            Arg(0, false),
            Ptr(0, true, true),
            Com(5,126),
        ], 
        vec![ // 36 
            Com(2,33),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp25
        vec![ // 37 
            Com(4,2),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 38 
            Com(2,35),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp26
        vec![ // 39 
            Arg(0, true),
            Ptr(4, true, true),
            Ptr(3, true, true),
            Ptr(2, true, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 40 
            Com(3,37),
            Arg(1, false),
        ], 
        vec![ // 41 
            Com(3,45),
            Arg(1, false),
        ], 
        vec![ // 42 
            Com(4,31),
            Arg(1, false),
        ], 
        vec![ // 43 
            Com(3,28),
            Arg(1, false),
        ], 
        vec![ // 44 
            Com(3,24),
            Arg(1, false),
        ], 
        // AExp27
        vec![ // 45 
            Arg(1, true),
            Arg(0, true),
        ], 
        // AExp28
        vec![ // 46 
            Com(3,59),
            Ptr(2, true, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 47 
            Com(3,63),
            Arg(1, true),
        ], 
        vec![ // 48 
            Com(3,55),
            Arg(3, true),
            Arg(0, false),
        ], 
        vec![ // 49 
            Com(3,55),
            Arg(2, true),
            Arg(0, false),
        ], 
        // AExp29
        vec![ // 50 
            Com(3,59),
            Ptr(2, true, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 51 
            Com(3,67),
            Arg(1, true),
        ], 
        vec![ // 52 
            Com(3,55),
            Arg(3, true),
            Arg(0, false),
        ], 
        vec![ // 53 
            Com(3,55),
            Arg(2, true),
            Arg(0, false),
        ], 
        // AExp30
        vec![ // 54 
            Com(3,17),
            Arg(0, true),
            Arg(2, true),
            Arg(1, true),
        ], 
        // AExp31
        vec![ // 55 
            Arg(0, true),
            Ptr(2, true, true),
            Arg(2, false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 56 
            Com(3,54),
            Arg(1, false),
            Arg(2, false),
        ], 
        vec![ // 57 
            Com(4,50),
            Arg(1, false),
            Arg(2, false),
        ], 
        vec![ // 58 
            Com(4,46),
            Arg(1, false),
            Arg(2, false),
        ], 
        // AExp32
        vec![ // 59 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 60 
            Com(3,61),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp33
        vec![ // 61 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 62 
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp34
        vec![ // 63 
            Com(2,66),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 64 
            Prm(Add,false),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp35
        vec![ // 65 
            Arg(0, true),
            Int(0),
        ], 
        // AExp36
        vec![ // 66 
            Prm(EQ,false),
            Arg(0, false),
            Int(0),
            Com(1,0),
            Com(2,65),
            Arg(1, true),
            Arg(0, false),
        ], 
        // AExp37
        vec![ // 67 
            Com(2,66),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 68 
            Prm(Sub,false),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp38
        vec![ // 69 
            Arg(0, true),
            Arg(2, false),
            Arg(1, true),
            Arg(2, false),
        ], 
        // AExp39
        vec![ // 70 
            Com(4,79),
            Arg(0, true),
            Arg(1, true),
            Ptr(0, true, true),
            Arg(3, true),
        ], 
        vec![ // 71 
            Com(3,85),
            Arg(2, true),
            Arg(4, true),
            Arg(5, true),
        ], 
        // AExp40
        vec![ // 72 
            Com(4,79),
            Arg(0, true),
            Arg(1, false),
            Ptr(0, true, true),
            Arg(3, false),
        ], 
        vec![ // 73 
            Com(3,85),
            Arg(2, true),
            Arg(1, false),
            Arg(3, false),
        ], 
        // AExp41
        vec![ // 74 
            Prm(EQ,false),
            Arg(3, true),
            Arg(0, false),
            Com(6,70),
            Com(6,72),
            Arg(2, true),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp42
        vec![ // 75 
            Com(3,69),
            Ptr(0, true, true),
            Arg(2, true),
        ], 
        vec![ // 76 
            Com(4,74),
            Arg(0, true),
            Arg(1, true),
            Arg(3, true),
        ], 
        // AExp43
        vec![ // 77 
            Arg(3, true),
            Ptr(0, true, true),
        ], 
        vec![ // 78 
            Com(4,75),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
            Arg(4, true),
        ], 
        // AExp44
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
        // AExp45
        vec![ // 82 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 83 
            Com(4,2),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp46
        vec![ // 84 
            Arg(2, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp47
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
        // AExp48
        vec![ // 87 
            Com(3,59),
            Ptr(2, true, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 88 
            Com(3,108),
            Arg(1, true),
        ], 
        vec![ // 89 
            Com(3,101),
            Arg(3, true),
            Arg(0, false),
        ], 
        vec![ // 90 
            Com(3,101),
            Arg(2, true),
            Arg(0, false),
        ], 
        // AExp49
        vec![ // 91 
            Com(3,59),
            Ptr(2, true, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 92 
            Com(3,114),
            Arg(1, true),
        ], 
        vec![ // 93 
            Com(3,55),
            Arg(3, true),
            Arg(0, false),
        ], 
        vec![ // 94 
            Com(3,55),
            Arg(2, true),
            Arg(0, false),
        ], 
        // AExp50
        vec![ // 95 
            Com(3,59),
            Ptr(2, true, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 96 
            Com(3,116),
            Arg(1, true),
        ], 
        vec![ // 97 
            Com(3,55),
            Arg(3, true),
            Arg(0, false),
        ], 
        vec![ // 98 
            Com(3,55),
            Arg(2, true),
            Arg(0, false),
        ], 
        // AExp51
        vec![ // 99 
            Com(3,101),
            Arg(2, true),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 100 
            Com(2,118),
            Arg(1, true),
        ], 
        // AExp52
        vec![ // 101 
            Arg(0, true),
            Ptr(5, true, true),
            Ptr(4, true, true),
            Ptr(3, true, true),
            Ptr(2, true, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 102 
            Arg(2, false),
            Com(2,1),
        ], 
        vec![ // 103 
            Com(3,99),
            Arg(1, false),
            Arg(2, false),
        ], 
        vec![ // 104 
            Com(4,95),
            Arg(1, false),
            Arg(2, false),
        ], 
        vec![ // 105 
            Arg(2, false),
            Com(2,0),
        ], 
        vec![ // 106 
            Com(4,91),
            Arg(1, false),
            Arg(2, false),
        ], 
        vec![ // 107 
            Com(4,87),
            Arg(1, false),
            Arg(2, false),
        ], 
        // AExp53
        vec![ // 108 
            Com(2,110),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 109 
            Com(1,113),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp54
        vec![ // 110 
            Arg(0, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 111 
            Arg(1, false),
            Com(2,1),
        ], 
        vec![ // 112 
            Arg(1, false),
            Com(2,0),
        ], 
        // AExp55
        vec![ // 113 
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp56
        vec![ // 114 
            Com(2,110),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 115 
            Prm(EQ,false),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp57
        vec![ // 116 
            Com(2,110),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 117 
            Prm(LE,false),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp58
        vec![ // 118 
            Com(2,110),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 119 
            Com(1,120),
            Arg(1, true),
        ], 
        // AExp59
        vec![ // 120 
            Arg(0, true),
            Com(2,1),
            Com(2,0),
        ], 
        // AExp60
        vec![ // 121 
            Com(4,2),
            Arg(2, true),
            Arg(1, true),
        ], 
        // AExp61
        vec![ // 122 
            Com(4,2),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp62
        vec![ // 123 
            Arg(3, true),
            Com(3,121),
            Com(3,122),
            Arg(1, true),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp63
        vec![ // 124 
            Arg(4, true),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp64
        vec![ // 125 
            Com(7,124),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp65
        vec![ // 126 
            Arg(3, true),
        ], 
        // AExp66
        vec![ // 127 
            Arg(6, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp67
        vec![ // 128 
            Arg(4, true),
            Arg(0, true),
        ], 
        // AExp68
        vec![ // 129 
            Arg(5, true),
            Arg(0, true),
        ], 
        // AExp69
        vec![ // 130 
            Com(7,84),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp70
        vec![ // 131 
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp71
        vec![ // 132 
            Arg(4, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp72
        vec![ // 133 
            Com(7,132),
            Arg(0, true),
            Arg(1, true),
        ], 
    ],

}});