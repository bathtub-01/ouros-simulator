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
            Com(1,5),
            Ptr(1, false, false),
            Int(0),
            Int(1),
        ], 
        // AExp1
        vec![ // 1 
            Com(7,113),
            Ptr(7, false, false),
            Ptr(5, false, false),
        ], 
        vec![ // 2 
            Com(1,15),
            Com(6,125),
            Ptr(8, false, false),
        ], 
        vec![ // 3 
            Com(1,120),
            Com(7,55),
            Ptr(2, false, false),
        ], 
        vec![ // 4 
            Com(6,125),
            Int(42),
        ], 
        vec![ // 5 
            Com(7,113),
            Ptr(4, false, false),
            Ptr(3, false, false),
        ], 
        vec![ // 6 
            Com(1,15),
            Com(1,122),
            Ptr(8, false, false),
        ], 
        vec![ // 7 
            Com(1,120),
            Com(7,55),
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
            Com(3,86),
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
            Com(1,53),
            Arg(0, false),
        ], 
        vec![ // 4 
            Com(3,18),
            Com(1,31),
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
            Arg(0, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 8 
            Com(1,9),
            Arg(1, true),
        ], 
        // AExp5
        vec![ // 9 
            Arg(0, true),
            Com(2,1),
            Com(2,7),
        ], 
        // AExp6
        vec![ // 10 
            Com(4,17),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 11 
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 12 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp7
        vec![ // 13 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 14 
            Com(4,10),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp8
        vec![ // 15 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 16 
            Com(3,13),
            Arg(0, true),
        ], 
        // AExp9
        vec![ // 17 
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp10
        vec![ // 18 
            Arg(0, true),
            Arg(2, true),
            Arg(1, true),
        ], 
        // AExp11
        vec![ // 19 
            Arg(6, true),
            Ptr(2, true, true),
            Arg(1, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
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
        // AExp12
        vec![ // 23 
            Arg(0, false),
            Arg(1, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 24 
            Arg(0, false),
            Arg(2, true),
        ], 
        // AExp13
        vec![ // 25 
            Arg(0, false),
            Arg(1, true),
            Com(2,1),
            Ptr(0, true, true),
        ], 
        vec![ // 26 
            Arg(0, false),
            Arg(2, true),
        ], 
        // AExp14
        vec![ // 27 
            Arg(0, true),
            Arg(1, true),
            Com(2,1),
            Com(2,0),
        ], 
        // AExp15
        vec![ // 28 
            Com(2,33),
            Arg(1, true),
            Arg(0, true),
        ], 
        // AExp16
        vec![ // 29 
            Com(7,19),
            Com(3,23),
            Com(1,0),
            Com(3,25),
            Com(2,27),
            Ptr(0, true, true),
        ], 
        vec![ // 30 
            Com(2,28),
            Arg(0, true),
        ], 
        // AExp17
        vec![ // 31 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 32 
            Com(1,29),
            Arg(0, true),
        ], 
        // AExp18
        vec![ // 33 
            Com(1,0),
            Com(1,35),
            Ptr(0, true, true),
        ], 
        vec![ // 34 
            Com(1,43),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp19
        vec![ // 35 
            Arg(0, true),
            Err(4),
            Com(1,0),
        ], 
        // AExp20
        vec![ // 36 
            Prm(EQ,false),
            Arg(0, true),
            Arg(3, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 37 
            Com(3,45),
            Arg(4, true),
        ], 
        vec![ // 38 
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp21
        vec![ // 39 
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 40 
            Com(5,36),
            Arg(0, true),
            Arg(1, true),
            Arg(3, true),
        ], 
        // AExp22
        vec![ // 41 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 42 
            Com(4,39),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp23
        vec![ // 43 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 44 
            Com(3,41),
            Arg(0, true),
        ], 
        // AExp24
        vec![ // 45 
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp25
        vec![ // 46 
            Com(1,75),
            Ptr(0, true, true),
        ], 
        vec![ // 47 
            Ptr(14, false, false),
            Arg(0, true),
        ], 
        // AExp26
        vec![ // 48 
            Com(1,15),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 49 
            Com(1,46),
            Arg(0, false),
        ], 
        vec![ // 50 
            Com(2,61),
            Arg(0, false),
        ], 
        // AExp27
        vec![ // 51 
            Com(1,94),
            Ptr(0, true, true),
        ], 
        vec![ // 52 
            Com(1,112),
            Arg(0, true),
        ], 
        // AExp28
        vec![ // 53 
            Com(1,48),
            Ptr(0, true, true),
        ], 
        vec![ // 54 
            Com(1,51),
            Arg(0, true),
        ], 
        // AExp29
        vec![ // 55 
            Arg(2, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp30
        vec![ // 56 
            Com(4,17),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 57 
            Com(2,61),
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 58 
            Com(3,55),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp31
        vec![ // 59 
            Arg(0, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 60 
            Com(4,56),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp32
        vec![ // 61 
            Arg(0, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 62 
            Com(3,59),
            Arg(1, true),
        ], 
        // AExp33
        vec![ // 63 
            Com(1,15),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 64 
            Com(1,75),
            Arg(0, true),
        ], 
        vec![ // 65 
            Com(4,17),
            Com(2,0),
        ], 
        // AExp34
        vec![ // 66 
            Com(1,15),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 67 
            Com(1,75),
            Arg(0, true),
        ], 
        vec![ // 68 
            Com(4,17),
            Com(2,1),
        ], 
        // AExp35
        vec![ // 69 
            Com(2,82),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 70 
            Com(1,66),
            Arg(0, false),
        ], 
        vec![ // 71 
            Com(1,63),
            Arg(0, false),
        ], 
        // AExp36
        vec![ // 72 
            Prm(Sub,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp37
        vec![ // 73 
            Com(1,69),
            Ptr(0, true, true),
        ], 
        vec![ // 74 
            Com(1,72),
            Arg(0, true),
        ], 
        // AExp38
        vec![ // 75 
            Prm(EQ,false),
            Arg(0, false),
            Int(0),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 76 
            Com(4,17),
            Com(2,0),
            Com(2,0),
        ], 
        vec![ // 77 
            Com(1,73),
            Arg(0, false),
        ], 
        // AExp39
        vec![ // 78 
            Com(4,17),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 79 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp40
        vec![ // 80 
            Arg(2, true),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 81 
            Com(3,78),
            Arg(1, true),
        ], 
        // AExp41
        vec![ // 82 
            Y,
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 83 
            Com(3,80),
            Arg(1, true),
        ], 
        // AExp42
        vec![ // 84 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 85 
            Prm(Add,false),
            Arg(1, true),
            Int(1),
        ], 
        // AExp43
        vec![ // 86 
            Arg(2, true),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 87 
            Com(3,84),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp44
        vec![ // 88 
            Com(1,102),
            Ptr(0, true, true),
        ], 
        vec![ // 89 
            Prm(EQ,true),
            Arg(0, true),
        ], 
        // AExp45
        vec![ // 90 
            Com(1,94),
            Ptr(0, true, true),
        ], 
        vec![ // 91 
            Com(1,88),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp46
        vec![ // 92 
            Com(4,17),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 93 
            Com(2,90),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp47
        vec![ // 94 
            Arg(0, true),
            Com(2,0),
            Com(2,92),
        ], 
        // AExp48
        vec![ // 95 
            Com(4,17),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 96 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp49
        vec![ // 97 
            Arg(0, true),
            Arg(2, false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 98 
            Com(3,95),
            Arg(1, false),
            Arg(2, false),
            Arg(3, false),
        ], 
        vec![ // 99 
            Arg(1, false),
            Arg(3, false),
        ], 
        // AExp50
        vec![ // 100 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 101 
            Com(4,97),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp51
        vec![ // 102 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 103 
            Com(3,100),
            Arg(0, true),
        ], 
        // AExp52
        vec![ // 104 
            Com(2,82),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 105 
            Com(1,112),
            Arg(1, true),
        ], 
        vec![ // 106 
            Com(1,112),
            Arg(0, true),
        ], 
        // AExp53
        vec![ // 107 
            Com(2,0),
        ], 
        // AExp54
        vec![ // 108 
            Com(2,82),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 109 
            Com(1,112),
            Arg(1, true),
        ], 
        vec![ // 110 
            Com(1,112),
            Arg(0, true),
        ], 
        // AExp55
        vec![ // 111 
            Com(4,17),
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp56
        vec![ // 112 
            Arg(0, true),
            Com(2,104),
            Com(1,107),
            Com(2,108),
            Com(1,112),
            Com(1,111),
        ], 
        // AExp57
        vec![ // 113 
            Arg(4, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp58
        vec![ // 114 
            Arg(0, true),
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 115 
            Arg(1, true),
            Arg(3, true),
        ], 
        // AExp59
        vec![ // 116 
            Arg(3, false),
            Arg(2, false),
            Ptr(0, true, true),
        ], 
        vec![ // 117 
            Com(6,114),
            Arg(0, true),
            Arg(1, true),
            Arg(2, false),
            Arg(3, false),
        ], 
        // AExp60
        vec![ // 118 
            Arg(2, true),
            Err(0),
            Ptr(0, true, true),
        ], 
        vec![ // 119 
            Com(4,116),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp61
        vec![ // 120 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 121 
            Com(3,118),
            Arg(0, true),
        ], 
        // AExp62
        vec![ // 122 
            Com(7,113),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 123 
            Com(6,125),
            Arg(0, true),
        ], 
        vec![ // 124 
            Com(6,125),
            Int(42),
        ], 
        // AExp63
        vec![ // 125 
            Arg(5, true),
            Arg(0, true),
        ], 
    ],

}});
