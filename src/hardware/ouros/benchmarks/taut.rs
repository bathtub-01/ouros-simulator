use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 75
#[rustfmt::skip]
pub static TAUT: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            Com(1,5),
            Ptr(1, false, false),
            Int(0),
            Int(1),
        ], 
        // AExp1
        vec![ // 1 
            Com(7,106),
            Ptr(7, false, false),
            Ptr(5, false, false),
        ], 
        vec![ // 2 
            Com(1,15),
            Com(6,120),
            Ptr(8, false, false),
        ], 
        vec![ // 3 
            Com(1,114),
            Com(7,116),
            Ptr(2, false, false),
        ], 
        vec![ // 4 
            Com(6,120),
            Int(42),
        ], 
        vec![ // 5 
            Com(7,106),
            Ptr(4, false, false),
            Ptr(3, false, false),
        ], 
        vec![ // 6 
            Com(1,15),
            Com(1,117),
            Ptr(8, false, false),
        ], 
        vec![ // 7 
            Com(1,114),
            Com(7,116),
            Ptr(6, false, false),
        ], 
        // AExp2
        vec![ // 8 
            Com(4,17),
            Int(0),
            Ptr(13, false, false),
        ], 
        vec![ // 9 
            Com(4,17),
            Int(5),
            Com(2,0),
        ], 
        vec![ // 10 
            Com(4,17),
            Int(4),
            Ptr(9, false, false),
        ], 
        vec![ // 11 
            Com(4,17),
            Int(3),
            Ptr(10, false, false),
        ], 
        vec![ // 12 
            Com(4,17),
            Int(2),
            Ptr(11, false, false),
        ], 
        vec![ // 13 
            Com(4,17),
            Int(1),
            Ptr(12, false, false),
        ], 
        // AExp3
        vec![ // 14 
            Y,
            Com(3,79),
            Int(0),
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
            Com(1,15),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 3 
            Com(1,48),
            Arg(0, false),
        ], 
        vec![ // 4 
            Com(3,18),
            Com(1,28),
            Arg(0, false),
        ], 
        // AExp3
        vec![ // 5 
            Com(1,9),
            Ptr(0, true, true),
        ], 
        vec![ // 6 
            Com(1,2),
            Arg(0, true),
        ], 
        // AExp4
        vec![ // 7 
            Com(2,0),
        ], 
        // AExp5
        vec![ // 8 
            Arg(0, true),
            Com(1,7),
            Com(1,9),
        ], 
        // AExp6
        vec![ // 9 
            Arg(0, true),
            Com(2,1),
            Com(1,8),
        ], 
        // AExp7
        vec![ // 10 
            Com(2,0),
        ], 
        // AExp8
        vec![ // 11 
            Com(4,17),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 12 
            Arg(3, true),
            Arg(1, true),
        ], 
        vec![ // 13 
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp9
        vec![ // 14 
            Arg(2, true),
            Com(2,10),
            Com(4,11),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp10
        vec![ // 15 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 16 
            Com(3,14),
            Arg(0, true),
        ], 
        // AExp11
        vec![ // 17 
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp12
        vec![ // 18 
            Arg(0, true),
            Arg(2, true),
            Arg(1, true),
        ], 
        // AExp13
        vec![ // 19 
            Arg(6, true),
            Ptr(2, true, true),
            Arg(1, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
            Com(2,30),
            Arg(4, true),
        ], 
        vec![ // 20 
            Arg(3, true),
            Arg(5, false),
        ], 
        vec![ // 21 
            Arg(2, true),
            Arg(5, false),
        ], 
        vec![ // 22 
            Arg(0, true),
            Arg(5, false),
        ], 
        // AExp14
        vec![ // 23 
            Com(2,0),
        ], 
        // AExp15
        vec![ // 24 
            Arg(0, true),
            Arg(1, true),
            Com(2,23),
            Com(1,28),
            Arg(3, true),
            Arg(2, true),
        ], 
        // AExp16
        vec![ // 25 
            Com(2,1),
        ], 
        // AExp17
        vec![ // 26 
            Arg(0, true),
            Arg(1, true),
            Com(2,25),
            Com(1,28),
            Arg(3, true),
            Arg(2, true),
        ], 
        // AExp18
        vec![ // 27 
            Arg(0, true),
            Arg(1, true),
            Com(2,1),
            Com(2,0),
        ], 
        // AExp19
        vec![ // 28 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 29 
            Com(7,19),
            Com(4,24),
            Com(2,0),
            Com(4,26),
            Com(3,27),
            Arg(0, true),
        ], 
        // AExp20
        vec![ // 30 
            Com(1,0),
            Com(1,32),
            Ptr(0, true, true),
        ], 
        vec![ // 31 
            Com(2,39),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp21
        vec![ // 32 
            Arg(0, true),
            Err(4),
            Com(1,0),
        ], 
        // AExp22
        vec![ // 33 
            Com(2,0),
        ], 
        // AExp23
        vec![ // 34 
            Com(2,39),
        ], 
        // AExp24
        vec![ // 35 
            Com(3,40),
            Arg(0, true),
        ], 
        // AExp25
        vec![ // 36 
            Prm(EQ,false),
            Arg(1, false),
            Arg(2, true),
            Com(1,34),
            Com(3,35),
            Arg(3, true),
            Arg(1, false),
            Arg(0, true),
        ], 
        // AExp26
        vec![ // 37 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 38 
            Com(4,36),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp27
        vec![ // 39 
            Arg(1, true),
            Com(1,33),
            Com(3,37),
            Arg(0, true),
        ], 
        // AExp28
        vec![ // 40 
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp29
        vec![ // 41 
            Com(1,71),
            Ptr(0, true, true),
        ], 
        vec![ // 42 
            Ptr(14, false, false),
            Arg(0, true),
        ], 
        // AExp30
        vec![ // 43 
            Com(1,15),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 44 
            Com(1,41),
            Arg(0, false),
        ], 
        vec![ // 45 
            Com(1,57),
            Arg(0, false),
        ], 
        // AExp31
        vec![ // 46 
            Com(1,86),
            Ptr(0, true, true),
        ], 
        vec![ // 47 
            Com(1,105),
            Arg(0, true),
        ], 
        // AExp32
        vec![ // 48 
            Com(1,43),
            Ptr(0, true, true),
        ], 
        vec![ // 49 
            Com(1,46),
            Arg(0, true),
        ], 
        // AExp33
        vec![ // 50 
            Com(2,0),
        ], 
        // AExp34
        vec![ // 51 
            Com(2,0),
        ], 
        // AExp35
        vec![ // 52 
            Arg(2, true),
            Arg(1, true),
            Arg(0, true),
        ], 
        // AExp36
        vec![ // 53 
            Com(4,17),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 54 
            Com(1,57),
            Arg(3, true),
            Arg(1, true),
        ], 
        vec![ // 55 
            Com(3,52),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp37
        vec![ // 56 
            Arg(2, true),
            Com(2,51),
            Com(4,53),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp38
        vec![ // 57 
            Arg(0, true),
            Com(1,50),
            Com(3,56),
        ], 
        // AExp39
        vec![ // 58 
            Com(1,15),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 59 
            Com(1,71),
            Arg(0, true),
        ], 
        vec![ // 60 
            Com(4,17),
            Com(2,0),
        ], 
        // AExp40
        vec![ // 61 
            Com(1,15),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 62 
            Com(1,71),
            Arg(0, true),
        ], 
        vec![ // 63 
            Com(4,17),
            Com(2,1),
        ], 
        // AExp41
        vec![ // 64 
            Com(2,75),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 65 
            Com(1,61),
            Arg(0, false),
        ], 
        vec![ // 66 
            Com(1,58),
            Arg(0, false),
        ], 
        // AExp42
        vec![ // 67 
            Prm(Sub,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp43
        vec![ // 68 
            Com(1,64),
            Ptr(0, true, true),
        ], 
        vec![ // 69 
            Com(1,67),
            Arg(0, true),
        ], 
        // AExp44
        vec![ // 70 
            Com(4,17),
            Com(2,0),
            Com(2,0),
        ], 
        // AExp45
        vec![ // 71 
            Prm(EQ,false),
            Arg(0, false),
            Int(0),
            Com(1,68),
            Com(1,70),
            Arg(0, false),
        ], 
        // AExp46
        vec![ // 72 
            Com(4,17),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 73 
            Arg(3, true),
            Arg(1, true),
        ], 
        // AExp47
        vec![ // 74 
            Arg(2, true),
            Com(2,0),
            Com(4,72),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp48
        vec![ // 75 
            Y,
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 76 
            Com(3,74),
            Arg(1, true),
        ], 
        // AExp49
        vec![ // 77 
            Arg(3, true),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 78 
            Prm(Add,false),
            Arg(2, true),
            Int(1),
        ], 
        // AExp50
        vec![ // 79 
            Arg(2, true),
            Com(2,0),
            Com(4,77),
            Arg(1, true),
            Arg(0, true),
        ], 
        // AExp51
        vec![ // 80 
            Com(1,95),
            Ptr(0, true, true),
        ], 
        vec![ // 81 
            Prm(EQ,true),
            Arg(0, true),
        ], 
        // AExp52
        vec![ // 82 
            Com(1,86),
            Ptr(0, true, true),
        ], 
        vec![ // 83 
            Com(1,80),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp53
        vec![ // 84 
            Com(4,17),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 85 
            Com(2,82),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp54
        vec![ // 86 
            Arg(0, true),
            Com(2,0),
            Com(2,84),
        ], 
        // AExp55
        vec![ // 87 
            Com(2,0),
        ], 
        // AExp56
        vec![ // 88 
            Arg(3, true),
        ], 
        // AExp57
        vec![ // 89 
            Com(4,17),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 90 
            Com(1,95),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp58
        vec![ // 91 
            Arg(3, false),
            Arg(1, false),
            Com(4,88),
            Com(4,89),
            Arg(1, false),
            Arg(3, false),
            Arg(2, false),
            Ptr(0, true, true),
        ], 
        vec![ // 92 
            Arg(0, true),
            Arg(2, false),
        ], 
        // AExp59
        vec![ // 93 
            Arg(2, true),
            Com(1,87),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 94 
            Com(4,91),
            Arg(1, true),
        ], 
        // AExp60
        vec![ // 95 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 96 
            Com(3,93),
            Arg(0, true),
        ], 
        // AExp61
        vec![ // 97 
            Com(2,75),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 98 
            Com(1,105),
            Arg(1, true),
        ], 
        vec![ // 99 
            Com(1,105),
            Arg(0, true),
        ], 
        // AExp62
        vec![ // 100 
            Com(2,0),
        ], 
        // AExp63
        vec![ // 101 
            Com(2,75),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 102 
            Com(1,105),
            Arg(1, true),
        ], 
        vec![ // 103 
            Com(1,105),
            Arg(0, true),
        ], 
        // AExp64
        vec![ // 104 
            Com(4,17),
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp65
        vec![ // 105 
            Arg(0, true),
            Com(2,97),
            Com(1,100),
            Com(2,101),
            Com(1,105),
            Com(1,104),
        ], 
        // AExp66
        vec![ // 106 
            Arg(4, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp67
        vec![ // 107 
            Err(0),
        ], 
        // AExp68
        vec![ // 108 
            Arg(3, true),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 109 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp69
        vec![ // 110 
            Arg(2, false),
            Com(2,0),
            Com(4,88),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 111 
            Com(4,108),
            Arg(0, true),
            Arg(1, false),
            Arg(2, false),
            Arg(3, true),
        ], 
        // AExp70
        vec![ // 112 
            Arg(2, true),
            Com(1,107),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 113 
            Com(4,110),
            Arg(1, true),
        ], 
        // AExp71
        vec![ // 114 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 115 
            Com(3,112),
            Arg(0, true),
        ], 
        // AExp72
        vec![ // 116 
            Arg(2, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp73
        vec![ // 117 
            Com(7,106),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 118 
            Com(6,120),
            Arg(0, true),
        ], 
        vec![ // 119 
            Com(6,120),
            Int(42),
        ], 
        // AExp74
        vec![ // 120 
            Arg(5, true),
            Arg(0, true),
        ], 
    ],

}});