use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 70
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
            Com(7,112),
            Ptr(7, false, false),
            Ptr(5, false, false),
        ], 
        vec![ // 2 
            Com(1,15),
            Com(6,124),
            Ptr(8, false, false),
        ], 
        vec![ // 3 
            Com(1,119),
            Com(7,55),
            Ptr(2, false, false),
        ], 
        vec![ // 4 
            Com(6,124),
            Int(42),
        ], 
        vec![ // 5 
            Com(7,112),
            Ptr(4, false, false),
            Ptr(3, false, false),
        ], 
        vec![ // 6 
            Com(1,15),
            Com(1,121),
            Ptr(8, false, false),
        ], 
        vec![ // 7 
            Com(1,119),
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
            Com(3,85),
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
            Com(1,33),
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
        // AExp8
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
        // AExp9
        vec![ // 15 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 16 
            Com(3,13),
            Arg(0, true),
        ], 
        // AExp10
        vec![ // 17 
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp11
        vec![ // 18 
            Arg(0, true),
            Arg(2, true),
            Arg(1, true),
        ], 
        // AExp12
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
        // AExp13
        vec![ // 23 
            Com(2,0),
        ], 
        // AExp14
        vec![ // 24 
            Arg(1, true),
            Arg(2, true),
            Com(2,23),
            Com(1,33),
            Arg(0, true),
        ], 
        // AExp15
        vec![ // 25 
            Com(2,1),
        ], 
        // AExp16
        vec![ // 26 
            Arg(1, true),
            Arg(2, true),
            Com(2,25),
            Com(1,33),
            Arg(0, true),
        ], 
        // AExp17
        vec![ // 27 
            Arg(0, true),
            Arg(1, true),
            Com(2,1),
            Com(2,0),
        ], 
        // AExp18
        vec![ // 28 
            Com(2,35),
            Arg(1, true),
            Arg(0, true),
        ], 
        // AExp19
        vec![ // 29 
            Com(7,19),
            Ptr(2, true, true),
            Com(1,0),
            Ptr(1, true, true),
            Com(2,27),
            Ptr(0, true, true),
        ], 
        vec![ // 30 
            Com(2,28),
            Arg(0, false),
        ], 
        vec![ // 31 
            Com(3,26),
            Arg(0, false),
        ], 
        vec![ // 32 
            Com(3,24),
            Arg(0, false),
        ], 
        // AExp20
        vec![ // 33 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 34 
            Com(1,29),
            Arg(0, true),
        ], 
        // AExp21
        vec![ // 35 
            Com(1,0),
            Com(1,37),
            Ptr(0, true, true),
        ], 
        vec![ // 36 
            Com(2,43),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp22
        vec![ // 37 
            Arg(0, true),
            Err(4),
            Com(1,0),
        ], 
        // AExp23
        vec![ // 38 
            Com(2,43),
        ], 
        // AExp24
        vec![ // 39 
            Com(3,45),
            Arg(0, true),
        ], 
        // AExp25
        vec![ // 40 
            Prm(EQ,false),
            Arg(0, false),
            Arg(2, true),
            Com(1,38),
            Com(3,39),
            Arg(3, true),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp26
        vec![ // 41 
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 42 
            Com(4,40),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp27
        vec![ // 43 
            Arg(1, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 44 
            Com(3,41),
            Arg(0, true),
        ], 
        // AExp28
        vec![ // 45 
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp29
        vec![ // 46 
            Com(1,76),
            Ptr(0, true, true),
        ], 
        vec![ // 47 
            Ptr(14, false, false),
            Arg(0, true),
        ], 
        // AExp30
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
        // AExp31
        vec![ // 51 
            Com(1,93),
            Ptr(0, true, true),
        ], 
        vec![ // 52 
            Com(1,111),
            Arg(0, true),
        ], 
        // AExp32
        vec![ // 53 
            Com(1,48),
            Ptr(0, true, true),
        ], 
        vec![ // 54 
            Com(1,51),
            Arg(0, true),
        ], 
        // AExp33
        vec![ // 55 
            Arg(2, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp34
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
        // AExp35
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
        // AExp36
        vec![ // 61 
            Arg(0, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 62 
            Com(3,59),
            Arg(1, true),
        ], 
        // AExp37
        vec![ // 63 
            Com(1,15),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 64 
            Com(1,76),
            Arg(0, true),
        ], 
        vec![ // 65 
            Com(4,17),
            Com(2,0),
        ], 
        // AExp38
        vec![ // 66 
            Com(1,15),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 67 
            Com(1,76),
            Arg(0, true),
        ], 
        vec![ // 68 
            Com(4,17),
            Com(2,1),
        ], 
        // AExp39
        vec![ // 69 
            Com(2,81),
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
        // AExp40
        vec![ // 72 
            Prm(Sub,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp41
        vec![ // 73 
            Com(1,69),
            Ptr(0, true, true),
        ], 
        vec![ // 74 
            Com(1,72),
            Arg(0, true),
        ], 
        // AExp42
        vec![ // 75 
            Com(4,17),
            Com(2,0),
            Com(2,0),
        ], 
        // AExp43
        vec![ // 76 
            Prm(EQ,false),
            Arg(0, false),
            Int(0),
            Com(1,73),
            Com(1,75),
            Arg(0, false),
        ], 
        // AExp44
        vec![ // 77 
            Com(4,17),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 78 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp45
        vec![ // 79 
            Arg(2, true),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 80 
            Com(3,77),
            Arg(1, true),
        ], 
        // AExp46
        vec![ // 81 
            Y,
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 82 
            Com(3,79),
            Arg(1, true),
        ], 
        // AExp47
        vec![ // 83 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 84 
            Prm(Add,false),
            Arg(1, true),
            Int(1),
        ], 
        // AExp48
        vec![ // 85 
            Arg(2, true),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 86 
            Com(3,83),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp49
        vec![ // 87 
            Com(1,101),
            Ptr(0, true, true),
        ], 
        vec![ // 88 
            Prm(EQ,true),
            Arg(0, true),
        ], 
        // AExp50
        vec![ // 89 
            Com(1,93),
            Ptr(0, true, true),
        ], 
        vec![ // 90 
            Com(1,87),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp51
        vec![ // 91 
            Com(4,17),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 92 
            Com(2,89),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp52
        vec![ // 93 
            Arg(0, true),
            Com(2,0),
            Com(2,91),
        ], 
        // AExp53
        vec![ // 94 
            Arg(3, true),
        ], 
        // AExp54
        vec![ // 95 
            Com(4,17),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 96 
            Com(1,101),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp55
        vec![ // 97 
            Arg(0, false),
            Arg(2, false),
            Com(4,94),
            Com(4,95),
            Arg(2, false),
            Arg(0, false),
            Arg(3, false),
            Ptr(0, true, true),
        ], 
        vec![ // 98 
            Arg(1, true),
            Arg(3, false),
        ], 
        // AExp56
        vec![ // 99 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 100 
            Com(4,97),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp57
        vec![ // 101 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 102 
            Com(3,99),
            Arg(0, true),
        ], 
        // AExp58
        vec![ // 103 
            Com(2,81),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 104 
            Com(1,111),
            Arg(1, true),
        ], 
        vec![ // 105 
            Com(1,111),
            Arg(0, true),
        ], 
        // AExp59
        vec![ // 106 
            Com(2,0),
        ], 
        // AExp60
        vec![ // 107 
            Com(2,81),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 108 
            Com(1,111),
            Arg(1, true),
        ], 
        vec![ // 109 
            Com(1,111),
            Arg(0, true),
        ], 
        // AExp61
        vec![ // 110 
            Com(4,17),
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp62
        vec![ // 111 
            Arg(0, true),
            Com(2,103),
            Com(1,106),
            Com(2,107),
            Com(1,111),
            Com(1,110),
        ], 
        // AExp63
        vec![ // 112 
            Arg(4, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp64
        vec![ // 113 
            Arg(0, true),
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 114 
            Arg(1, true),
            Arg(3, true),
        ], 
        // AExp65
        vec![ // 115 
            Arg(3, false),
            Arg(2, false),
            Ptr(0, true, true),
        ], 
        vec![ // 116 
            Com(6,113),
            Arg(0, true),
            Arg(1, true),
            Arg(2, false),
            Arg(3, false),
        ], 
        // AExp66
        vec![ // 117 
            Arg(2, true),
            Err(0),
            Ptr(0, true, true),
        ], 
        vec![ // 118 
            Com(4,115),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp67
        vec![ // 119 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 120 
            Com(3,117),
            Arg(0, true),
        ], 
        // AExp68
        vec![ // 121 
            Com(7,112),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 122 
            Com(6,124),
            Arg(0, true),
        ], 
        vec![ // 123 
            Com(6,124),
            Int(42),
        ], 
        // AExp69
        vec![ // 124 
            Arg(5, true),
            Arg(0, true),
        ], 
    ],

}});