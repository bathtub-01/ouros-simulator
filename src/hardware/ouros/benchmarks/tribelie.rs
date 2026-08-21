use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 83
#[rustfmt::skip]
pub static TRIBELIE: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            Com(1,5),
            Com(1,27),
            Ptr(1, false, false),
        ], 
        // AExp1
        vec![ // 1 
            Y,
            Ptr(29, false, false),
            Ptr(3, false, false),
        ], 
        vec![ // 2 
            Com(4,19),
            Com(2,1),
            Com(2,0),
        ], 
        vec![ // 3 
            Com(4,19),
            Com(2,0),
            Ptr(2, false, false),
        ], 
        vec![ // 4 
            Com(4,19),
            Com(2,1),
            Com(2,0),
        ], 
        vec![ // 5 
            Com(4,19),
            Com(2,0),
            Ptr(4, false, false),
        ], 
        vec![ // 6 
            Com(4,19),
            Com(2,1),
            Com(2,0),
        ], 
        vec![ // 7 
            Com(4,19),
            Com(2,0),
            Ptr(6, false, false),
        ], 
        vec![ // 8 
            Com(4,19),
            Com(2,1),
            Com(2,0),
        ], 
        vec![ // 9 
            Com(4,19),
            Com(2,0),
            Ptr(8, false, false),
        ], 
        vec![ // 10 
            Com(4,19),
            Com(2,1),
            Com(2,0),
        ], 
        vec![ // 11 
            Com(4,19),
            Com(2,0),
            Ptr(10, false, false),
        ], 
        vec![ // 12 
            Com(4,19),
            Com(2,1),
            Com(2,0),
        ], 
        vec![ // 13 
            Com(4,19),
            Com(2,0),
            Ptr(12, false, false),
        ], 
        vec![ // 14 
            Com(4,19),
            Com(2,1),
            Com(2,0),
        ], 
        vec![ // 15 
            Com(4,19),
            Com(2,0),
            Ptr(14, false, false),
        ], 
        vec![ // 16 
            Com(4,19),
            Com(2,1),
            Com(2,0),
        ], 
        vec![ // 17 
            Com(4,19),
            Com(2,0),
            Ptr(16, false, false),
        ], 
        vec![ // 18 
            Com(3,125),
            Ptr(17, false, false),
        ], 
        vec![ // 19 
            Com(2,106),
            Ptr(18, false, false),
            Ptr(15, false, false),
        ], 
        vec![ // 20 
            Com(4,84),
            Ptr(19, false, false),
        ], 
        vec![ // 21 
            Com(5,77),
            Ptr(20, false, false),
            Ptr(13, false, false),
        ], 
        vec![ // 22 
            Com(4,71),
            Ptr(21, false, false),
        ], 
        vec![ // 23 
            Com(5,68),
            Ptr(22, false, false),
            Ptr(11, false, false),
        ], 
        vec![ // 24 
            Com(3,64),
            Ptr(23, false, false),
        ], 
        vec![ // 25 
            Com(4,59),
            Ptr(24, false, false),
            Ptr(9, false, false),
        ], 
        vec![ // 26 
            Com(1,55),
            Ptr(25, false, false),
        ], 
        vec![ // 27 
            Com(3,49),
            Ptr(26, false, false),
            Ptr(7, false, false),
        ], 
        vec![ // 28 
            Com(5,45),
            Ptr(27, false, false),
            Ptr(5, false, false),
        ], 
        vec![ // 29 
            Com(3,42),
            Ptr(28, false, false),
        ], 
        // AExp2
        vec![ // 30 
            Com(2,10),
            Prm(Add,false),
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
            Com(1,0),
            Ptr(30, false, false),
            Ptr(0, true, true),
        ], 
        vec![ // 3 
            Com(1,17),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp3
        vec![ // 4 
            Arg(0, true),
            Arg(1, true),
            Int(0),
            Int(1),
        ], 
        // AExp4
        vec![ // 5 
            Com(2,2),
            Ptr(0, true, true),
        ], 
        vec![ // 6 
            Com(2,4),
            Arg(0, true),
        ], 
        // AExp5
        vec![ // 7 
            Arg(3, true),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 8 
            Arg(4, true),
            Arg(1, true),
        ], 
        // AExp6
        vec![ // 9 
            Arg(3, true),
            Com(3,0),
            Com(5,7),
            Arg(1, true),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp7
        vec![ // 10 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 11 
            Com(4,9),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp8
        vec![ // 12 
            Com(2,0),
        ], 
        // AExp9
        vec![ // 13 
            Com(4,19),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 14 
            Arg(3, true),
            Arg(1, true),
        ], 
        vec![ // 15 
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp10
        vec![ // 16 
            Arg(2, true),
            Com(2,12),
            Com(4,13),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp11
        vec![ // 17 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 18 
            Com(3,16),
            Arg(0, true),
        ], 
        // AExp12
        vec![ // 19 
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp13
        vec![ // 20 
            Com(1,29),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 21 
            Com(1,32),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp14
        vec![ // 22 
            Com(1,29),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 23 
            Com(1,40),
            Arg(1, true),
            Arg(2, true),
            Arg(3, false),
        ], 
        vec![ // 24 
            Com(1,35),
            Arg(0, true),
            Arg(3, false),
        ], 
        // AExp15
        vec![ // 25 
            Com(7,20),
            Arg(2, false),
            Ptr(0, true, true),
            Arg(3, false),
        ], 
        vec![ // 26 
            Com(4,22),
            Arg(0, true),
            Arg(1, true),
            Arg(2, false),
            Arg(3, false),
        ], 
        // AExp16
        vec![ // 27 
            Arg(0, true),
            Com(4,25),
        ], 
        // AExp17
        vec![ // 28 
            Com(2,0),
        ], 
        // AExp18
        vec![ // 29 
            Arg(0, true),
            Com(1,28),
            Com(1,0),
        ], 
        // AExp19
        vec![ // 30 
            Arg(0, true),
            Com(2,1),
            Com(2,0),
        ], 
        // AExp20
        vec![ // 31 
            Com(2,1),
        ], 
        // AExp21
        vec![ // 32 
            Arg(0, true),
            Com(1,30),
            Com(1,31),
        ], 
        // AExp22
        vec![ // 33 
            Arg(0, true),
            Com(2,1),
            Com(2,0),
        ], 
        // AExp23
        vec![ // 34 
            Com(2,1),
        ], 
        // AExp24
        vec![ // 35 
            Arg(0, true),
            Com(1,33),
            Com(1,34),
        ], 
        // AExp25
        vec![ // 36 
            Com(2,0),
        ], 
        // AExp26
        vec![ // 37 
            Arg(0, true),
            Com(2,1),
            Com(2,0),
        ], 
        // AExp27
        vec![ // 38 
            Arg(0, true),
            Com(1,36),
            Com(1,37),
        ], 
        // AExp28
        vec![ // 39 
            Arg(1, true),
            Com(2,0),
            Com(2,1),
        ], 
        // AExp29
        vec![ // 40 
            Arg(0, true),
            Com(1,38),
            Com(2,39),
        ], 
        // AExp30
        vec![ // 41 
            Com(2,0),
        ], 
        // AExp31
        vec![ // 42 
            Arg(2, true),
            Com(1,41),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp32
        vec![ // 43 
            Arg(6, true),
            Arg(0, true),
            Arg(1, true),
            Arg(4, true),
            Arg(2, true),
            Arg(5, true),
            Arg(3, true),
        ], 
        // AExp33
        vec![ // 44 
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp34
        vec![ // 45 
            Y,
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 46 
            Com(7,43),
            Com(4,44),
            Arg(0, true),
            Arg(3, true),
            Arg(2, true),
            Arg(4, true),
        ], 
        // AExp35
        vec![ // 47 
            Y,
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 48 
            Arg(0, true),
            Arg(2, true),
            Arg(5, true),
            Arg(6, true),
        ], 
        // AExp36
        vec![ // 49 
            Com(7,47),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 50 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp37
        vec![ // 51 
            Arg(0, true),
            Ptr(0, true, true),
            Arg(2, true),
        ], 
        vec![ // 52 
            Arg(1, true),
            Arg(3, true),
            Arg(4, true),
            Arg(5, true),
            Arg(6, true),
        ], 
        // AExp38
        vec![ // 53 
            Arg(0, true),
            Arg(2, true),
            Arg(1, true),
        ], 
        // AExp39
        vec![ // 54 
            Arg(6, true),
            Arg(0, true),
            Arg(1, true),
            Arg(3, true),
            Arg(2, true),
            Arg(5, true),
            Arg(4, true),
        ], 
        // AExp40
        vec![ // 55 
            Com(7,51),
            Com(3,53),
            Ptr(0, true, true),
        ], 
        vec![ // 56 
            Com(7,54),
            Com(5,44),
            Arg(0, true),
        ], 
        // AExp41
        vec![ // 57 
            Y,
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 58 
            Arg(0, true),
            Arg(4, true),
            Arg(5, true),
            Arg(6, true),
        ], 
        // AExp42
        vec![ // 59 
            Com(7,57),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 60 
            Arg(0, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        // AExp43
        vec![ // 61 
            Arg(0, true),
            Ptr(0, true, true),
            Arg(5, true),
            Arg(2, true),
        ], 
        vec![ // 62 
            Arg(1, true),
            Arg(3, true),
            Arg(4, true),
            Arg(6, true),
        ], 
        // AExp44
        vec![ // 63 
            Arg(0, true),
            Arg(3, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp45
        vec![ // 64 
            Com(7,61),
            Com(4,63),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 65 
            Com(7,54),
            Com(6,44),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp46
        vec![ // 66 
            Y,
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 67 
            Arg(0, true),
            Arg(3, true),
            Arg(4, true),
            Arg(5, true),
            Arg(6, true),
        ], 
        // AExp47
        vec![ // 68 
            Com(7,66),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 69 
            Arg(0, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        // AExp48
        vec![ // 70 
            Arg(6, true),
            Com(7,44),
            Arg(0, true),
            Arg(2, true),
            Arg(1, true),
            Arg(5, true),
            Arg(3, true),
            Arg(4, true),
        ], 
        // AExp49
        vec![ // 71 
            Com(7,61),
            Com(4,63),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 72 
            Com(7,70),
            Arg(0, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        // AExp50
        vec![ // 73 
            Arg(0, true),
            Ptr(0, true, true),
            Arg(2, true),
        ], 
        vec![ // 74 
            Arg(1, true),
            Arg(4, true),
            Arg(5, true),
            Arg(6, true),
        ], 
        // AExp51
        vec![ // 75 
            Y,
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 76 
            Arg(0, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        // AExp52
        vec![ // 77 
            Com(7,73),
            Com(4,75),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 78 
            Arg(0, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        // AExp53
        vec![ // 79 
            Arg(0, true),
            Ptr(0, true, true),
            Arg(6, true),
            Arg(3, true),
        ], 
        vec![ // 80 
            Arg(1, true),
            Arg(2, true),
            Arg(4, true),
            Arg(5, true),
        ], 
        // AExp54
        vec![ // 81 
            Arg(0, true),
            Arg(4, true),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        // AExp55
        vec![ // 82 
            Com(5,81),
            Ptr(0, true, true),
            Arg(1, true),
            Arg(3, true),
            Arg(2, true),
        ], 
        vec![ // 83 
            Arg(0, true),
            Arg(4, true),
        ], 
        // AExp56
        vec![ // 84 
            Com(7,79),
            Com(5,82),
            Ptr(0, true, true),
            Arg(2, true),
            Arg(1, true),
        ], 
        vec![ // 85 
            Arg(0, true),
            Arg(3, true),
        ], 
        // AExp57
        vec![ // 86 
            Arg(0, true),
            Arg(6, true),
            Arg(1, true),
            Arg(2, true),
            Arg(5, true),
            Arg(3, true),
            Arg(4, true),
        ], 
        // AExp58
        vec![ // 87 
            Arg(0, true),
            Ptr(0, true, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        vec![ // 88 
            Arg(1, true),
            Arg(6, true),
        ], 
        // AExp59
        vec![ // 89 
            Arg(0, true),
            Arg(6, true),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
            Arg(4, true),
            Arg(5, true),
        ], 
        // AExp60
        vec![ // 90 
            Arg(6, true),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
            Arg(5, true),
            Arg(4, true),
        ], 
        // AExp61
        vec![ // 91 
            Com(7,89),
            Ptr(0, true, true),
            Arg(5, true),
        ], 
        vec![ // 92 
            Com(7,90),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
            Arg(4, true),
            Arg(6, true),
        ], 
        // AExp62
        vec![ // 93 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 94 
            Com(7,91),
            Arg(1, true),
            Arg(2, true),
            Arg(4, true),
            Arg(3, true),
            Arg(5, true),
            Arg(6, true),
        ], 
        // AExp63
        vec![ // 95 
            Com(7,87),
            Arg(0, true),
            Ptr(0, true, true),
            Arg(5, true),
            Arg(4, true),
        ], 
        vec![ // 96 
            Com(7,93),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
            Arg(6, true),
        ], 
        // AExp64
        vec![ // 97 
            Arg(6, true),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 98 
            Com(7,95),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
            Arg(4, true),
            Arg(5, true),
        ], 
        // AExp65
        vec![ // 99 
            Com(7,44),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp66
        vec![ // 100 
            Y,
            Ptr(0, true, true),
            Arg(2, true),
        ], 
        vec![ // 101 
            Arg(0, true),
            Arg(3, true),
            Arg(4, true),
            Arg(5, true),
            Arg(1, true),
            Arg(6, true),
        ], 
        // AExp67
        vec![ // 102 
            Com(7,100),
            Ptr(0, true, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        vec![ // 103 
            Arg(0, true),
            Arg(3, true),
        ], 
        // AExp68
        vec![ // 104 
            Arg(0, true),
            Arg(5, true),
            Arg(1, true),
            Arg(2, true),
            Arg(4, true),
            Arg(3, true),
        ], 
        // AExp69
        vec![ // 105 
            Com(7,44),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp70
        vec![ // 106 
            Com(7,86),
            Ptr(0, true, true),
        ], 
        vec![ // 107 
            Com(7,97),
            Com(3,99),
            Com(4,102),
            Com(6,104),
            Com(4,105),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp71
        vec![ // 108 
            Arg(0, true),
            Ptr(0, true, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        vec![ // 109 
            Arg(1, true),
            Arg(4, true),
            Arg(5, true),
            Arg(6, true),
        ], 
        // AExp72
        vec![ // 110 
            Com(7,108),
            Arg(0, true),
            Ptr(0, true, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        vec![ // 111 
            Arg(1, true),
            Arg(6, true),
        ], 
        // AExp73
        vec![ // 112 
            Arg(0, true),
            Arg(1, true),
            Arg(3, true),
            Arg(4, true),
            Arg(2, true),
            Arg(6, true),
            Arg(5, true),
        ], 
        // AExp74
        vec![ // 113 
            Y,
            Ptr(0, true, true),
            Arg(2, true),
        ], 
        vec![ // 114 
            Com(7,112),
            Arg(0, true),
            Arg(3, true),
            Arg(1, true),
            Arg(4, true),
            Arg(5, true),
        ], 
        // AExp75
        vec![ // 115 
            Arg(6, true),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
            Arg(4, true),
            Arg(5, true),
        ], 
        // AExp76
        vec![ // 116 
            Com(7,44),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp77
        vec![ // 117 
            Com(7,108),
            Arg(0, true),
            Ptr(0, true, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        vec![ // 118 
            Com(6,127),
            Arg(3, true),
            Arg(4, true),
            Arg(5, true),
            Arg(6, true),
        ], 
        // AExp78
        vec![ // 119 
            Com(4,19),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 120 
            Arg(3, true),
            Arg(2, true),
        ], 
        vec![ // 121 
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp79
        vec![ // 122 
            Com(7,117),
            Com(4,119),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp80
        vec![ // 123 
            Com(7,89),
            Ptr(0, true, true),
        ], 
        vec![ // 124 
            Com(7,115),
            Com(5,116),
            Com(4,122),
            Arg(1, true),
            Arg(0, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        // AExp81
        vec![ // 125 
            Com(7,110),
            Com(6,113),
            Ptr(0, true, true),
            Arg(1, true),
            Arg(0, true),
        ], 
        vec![ // 126 
            Com(4,123),
            Arg(2, true),
        ], 
        // AExp82
        vec![ // 127 
            Com(4,63),
            Ptr(0, true, true),
        ], 
        vec![ // 128 
            Com(7,115),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
            Arg(4, true),
            Arg(5, true),
        ], 
    ],

}});