use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 64
#[rustfmt::skip]
pub static TAUT: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            COM(1,5),
            PTR(1, false, false),
            INT(0),
            INT(1),
        ], 
        // AExp1
        vec![ // 1 
            COM(7,113),
            PTR(7, false, false),
            PTR(5, false, false),
        ], 
        vec![ // 2 
            COM(1,15),
            COM(6,125),
            PTR(8, false, false),
        ], 
        vec![ // 3 
            COM(1,120),
            COM(7,55),
            PTR(2, false, false),
        ], 
        vec![ // 4 
            COM(6,125),
            INT(42),
        ], 
        vec![ // 5 
            COM(7,113),
            PTR(4, false, false),
            PTR(3, false, false),
        ], 
        vec![ // 6 
            COM(1,15),
            COM(1,122),
            PTR(8, false, false),
        ], 
        vec![ // 7 
            COM(1,120),
            COM(7,55),
            PTR(6, false, false),
        ], 
        // AExp2
        vec![ // 8 
            COM(4,17),
            INT(0),
            PTR(13, false, false),
        ], 
        vec![ // 9 
            COM(4,17),
            INT(5),
            COM(2,0),
        ], 
        vec![ // 10 
            COM(4,17),
            INT(4),
            PTR(9, false, false),
        ], 
        vec![ // 11 
            COM(4,17),
            INT(3),
            PTR(10, false, false),
        ], 
        vec![ // 12 
            COM(4,17),
            INT(2),
            PTR(11, false, false),
        ], 
        vec![ // 13 
            COM(4,17),
            INT(1),
            PTR(12, false, false),
        ], 
        // AExp3
        vec![ // 14 
            Y,
            COM(3,86),
            INT(0),
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
            COM(1,15),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 3 
            COM(1,53),
            ARG(0, false),
        ], 
        vec![ // 4 
            COM(3,18),
            COM(1,31),
            ARG(0, false),
        ], 
        // AExp3
        vec![ // 5 
            COM(1,9),
            PTR(0, true, true),
        ], 
        vec![ // 6 
            COM(1,2),
            ARG(0, true),
        ], 
        // AExp4
        vec![ // 7 
            ARG(0, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 8 
            COM(1,9),
            ARG(1, true),
        ], 
        // AExp5
        vec![ // 9 
            ARG(0, true),
            COM(2,1),
            COM(2,7),
        ], 
        // AExp6
        vec![ // 10 
            COM(4,17),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 11 
            ARG(1, true),
            ARG(3, true),
        ], 
        vec![ // 12 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp7
        vec![ // 13 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 14 
            COM(4,10),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp8
        vec![ // 15 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 16 
            COM(3,13),
            ARG(0, true),
        ], 
        // AExp9
        vec![ // 17 
            ARG(3, true),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp10
        vec![ // 18 
            ARG(0, true),
            ARG(2, true),
            ARG(1, true),
        ], 
        // AExp11
        vec![ // 19 
            ARG(6, true),
            PTR(2, true, true),
            ARG(1, true),
            PTR(1, true, true),
            PTR(0, true, true),
            ARG(4, true),
        ], 
        vec![ // 20 
            ARG(3, true),
            ARG(5, false),
        ], 
        vec![ // 21 
            ARG(2, true),
            ARG(5, false),
        ], 
        vec![ // 22 
            ARG(0, true),
            ARG(5, false),
        ], 
        // AExp12
        vec![ // 23 
            ARG(0, false),
            ARG(1, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 24 
            ARG(0, false),
            ARG(2, true),
        ], 
        // AExp13
        vec![ // 25 
            ARG(0, false),
            ARG(1, true),
            COM(2,1),
            PTR(0, true, true),
        ], 
        vec![ // 26 
            ARG(0, false),
            ARG(2, true),
        ], 
        // AExp14
        vec![ // 27 
            ARG(0, true),
            ARG(1, true),
            COM(2,1),
            COM(2,0),
        ], 
        // AExp15
        vec![ // 28 
            COM(2,33),
            ARG(1, true),
            ARG(0, true),
        ], 
        // AExp16
        vec![ // 29 
            COM(7,19),
            COM(3,23),
            COM(1,0),
            COM(3,25),
            COM(2,27),
            PTR(0, true, true),
        ], 
        vec![ // 30 
            COM(2,28),
            ARG(0, true),
        ], 
        // AExp17
        vec![ // 31 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 32 
            COM(1,29),
            ARG(0, true),
        ], 
        // AExp18
        vec![ // 33 
            COM(1,0),
            COM(1,35),
            PTR(0, true, true),
        ], 
        vec![ // 34 
            COM(1,43),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp19
        vec![ // 35 
            ARG(0, true),
            ERR(4),
            COM(1,0),
        ], 
        // AExp20
        vec![ // 36 
            PRM(EQ,false),
            ARG(0, true),
            ARG(3, true),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 37 
            COM(3,45),
            ARG(4, true),
        ], 
        vec![ // 38 
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp21
        vec![ // 39 
            ARG(2, true),
            PTR(0, true, true),
        ], 
        vec![ // 40 
            COM(5,36),
            ARG(0, true),
            ARG(1, true),
            ARG(3, true),
        ], 
        // AExp22
        vec![ // 41 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 42 
            COM(4,39),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp23
        vec![ // 43 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 44 
            COM(3,41),
            ARG(0, true),
        ], 
        // AExp24
        vec![ // 45 
            ARG(2, true),
            ARG(0, true),
        ], 
        // AExp25
        vec![ // 46 
            COM(1,75),
            PTR(0, true, true),
        ], 
        vec![ // 47 
            PTR(14, false, false),
            ARG(0, true),
        ], 
        // AExp26
        vec![ // 48 
            COM(1,15),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 49 
            COM(1,46),
            ARG(0, false),
        ], 
        vec![ // 50 
            COM(2,61),
            ARG(0, false),
        ], 
        // AExp27
        vec![ // 51 
            COM(1,94),
            PTR(0, true, true),
        ], 
        vec![ // 52 
            COM(1,112),
            ARG(0, true),
        ], 
        // AExp28
        vec![ // 53 
            COM(1,48),
            PTR(0, true, true),
        ], 
        vec![ // 54 
            COM(1,51),
            ARG(0, true),
        ], 
        // AExp29
        vec![ // 55 
            ARG(2, true),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp30
        vec![ // 56 
            COM(4,17),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 57 
            COM(2,61),
            ARG(1, true),
            ARG(3, true),
        ], 
        vec![ // 58 
            COM(3,55),
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp31
        vec![ // 59 
            ARG(0, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 60 
            COM(4,56),
            ARG(1, true),
            ARG(2, true),
        ], 
        // AExp32
        vec![ // 61 
            ARG(0, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 62 
            COM(3,59),
            ARG(1, true),
        ], 
        // AExp33
        vec![ // 63 
            COM(1,15),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 64 
            COM(1,75),
            ARG(0, true),
        ], 
        vec![ // 65 
            COM(4,17),
            COM(2,0),
        ], 
        // AExp34
        vec![ // 66 
            COM(1,15),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 67 
            COM(1,75),
            ARG(0, true),
        ], 
        vec![ // 68 
            COM(4,17),
            COM(2,1),
        ], 
        // AExp35
        vec![ // 69 
            COM(2,82),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 70 
            COM(1,66),
            ARG(0, false),
        ], 
        vec![ // 71 
            COM(1,63),
            ARG(0, false),
        ], 
        // AExp36
        vec![ // 72 
            PRM(SUB,false),
            ARG(0, true),
            INT(1),
        ], 
        // AExp37
        vec![ // 73 
            COM(1,69),
            PTR(0, true, true),
        ], 
        vec![ // 74 
            COM(1,72),
            ARG(0, true),
        ], 
        // AExp38
        vec![ // 75 
            PRM(EQ,false),
            ARG(0, false),
            INT(0),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 76 
            COM(4,17),
            COM(2,0),
            COM(2,0),
        ], 
        vec![ // 77 
            COM(1,73),
            ARG(0, false),
        ], 
        // AExp39
        vec![ // 78 
            COM(4,17),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 79 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp40
        vec![ // 80 
            ARG(2, true),
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 81 
            COM(3,78),
            ARG(1, true),
        ], 
        // AExp41
        vec![ // 82 
            Y,
            PTR(0, true, true),
            ARG(0, true),
        ], 
        vec![ // 83 
            COM(3,80),
            ARG(1, true),
        ], 
        // AExp42
        vec![ // 84 
            ARG(0, true),
            PTR(0, true, true),
        ], 
        vec![ // 85 
            PRM(ADD,false),
            ARG(1, true),
            INT(1),
        ], 
        // AExp43
        vec![ // 86 
            ARG(2, true),
            ARG(1, false),
            PTR(0, true, true),
        ], 
        vec![ // 87 
            COM(3,84),
            ARG(0, true),
            ARG(1, false),
        ], 
        // AExp44
        vec![ // 88 
            COM(1,102),
            PTR(0, true, true),
        ], 
        vec![ // 89 
            PRM(EQ,true),
            ARG(0, true),
        ], 
        // AExp45
        vec![ // 90 
            COM(1,94),
            PTR(0, true, true),
        ], 
        vec![ // 91 
            COM(1,88),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp46
        vec![ // 92 
            COM(4,17),
            ARG(0, false),
            PTR(0, true, true),
        ], 
        vec![ // 93 
            COM(2,90),
            ARG(0, false),
            ARG(1, true),
        ], 
        // AExp47
        vec![ // 94 
            ARG(0, true),
            COM(2,0),
            COM(2,92),
        ], 
        // AExp48
        vec![ // 95 
            COM(4,17),
            ARG(1, true),
            PTR(0, true, true),
        ], 
        vec![ // 96 
            ARG(0, true),
            ARG(2, true),
        ], 
        // AExp49
        vec![ // 97 
            ARG(0, true),
            ARG(2, false),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 98 
            COM(3,95),
            ARG(1, false),
            ARG(2, false),
            ARG(3, false),
        ], 
        vec![ // 99 
            ARG(1, false),
            ARG(3, false),
        ], 
        // AExp50
        vec![ // 100 
            ARG(2, true),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 101 
            COM(4,97),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp51
        vec![ // 102 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 103 
            COM(3,100),
            ARG(0, true),
        ], 
        // AExp52
        vec![ // 104 
            COM(2,82),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 105 
            COM(1,112),
            ARG(1, true),
        ], 
        vec![ // 106 
            COM(1,112),
            ARG(0, true),
        ], 
        // AExp53
        vec![ // 107 
            COM(2,0),
        ], 
        // AExp54
        vec![ // 108 
            COM(2,82),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 109 
            COM(1,112),
            ARG(1, true),
        ], 
        vec![ // 110 
            COM(1,112),
            ARG(0, true),
        ], 
        // AExp55
        vec![ // 111 
            COM(4,17),
            ARG(0, true),
            COM(2,0),
        ], 
        // AExp56
        vec![ // 112 
            ARG(0, true),
            COM(2,104),
            COM(1,107),
            COM(2,108),
            COM(1,112),
            COM(1,111),
        ], 
        // AExp57
        vec![ // 113 
            ARG(4, true),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp58
        vec![ // 114 
            ARG(0, true),
            ARG(2, true),
            PTR(0, true, true),
        ], 
        vec![ // 115 
            ARG(1, true),
            ARG(3, true),
        ], 
        // AExp59
        vec![ // 116 
            ARG(3, false),
            ARG(2, false),
            PTR(0, true, true),
        ], 
        vec![ // 117 
            COM(6,114),
            ARG(0, true),
            ARG(1, true),
            ARG(2, false),
            ARG(3, false),
        ], 
        // AExp60
        vec![ // 118 
            ARG(2, true),
            ERR(0),
            PTR(0, true, true),
        ], 
        vec![ // 119 
            COM(4,116),
            ARG(0, true),
            ARG(1, true),
        ], 
        // AExp61
        vec![ // 120 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 121 
            COM(3,118),
            ARG(0, true),
        ], 
        // AExp62
        vec![ // 122 
            COM(7,113),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 123 
            COM(6,125),
            ARG(0, true),
        ], 
        vec![ // 124 
            COM(6,125),
            INT(42),
        ], 
        // AExp63
        vec![ // 125 
            ARG(5, true),
            ARG(0, true),
        ], 
    ],

}});