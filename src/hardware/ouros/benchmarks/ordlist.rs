use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 39
#[rustfmt::skip]
pub static ORDLIST: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            Com(1,11),
            Ptr(4, false, false),
            Int(0),
            Int(1),
        ], 
        vec![ // 1 
            Com(3,67),
            Com(2,1),
        ], 
        vec![ // 2 
            Com(1,0),
            Com(3,67),
            Ptr(1, false, false),
        ], 
        vec![ // 3 
            Com(1,0),
            Com(3,67),
            Ptr(2, false, false),
        ], 
        vec![ // 4 
            Com(1,0),
            Com(3,67),
            Ptr(3, false, false),
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
            Com(1,27),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 3 
            Com(1,65),
            Arg(0, true),
        ], 
        vec![ // 4 
            Com(2,31),
            Com(2,1),
        ], 
        // AExp3
        vec![ // 5 
            Com(1,27),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 6 
            Com(1,65),
            Arg(0, true),
        ], 
        vec![ // 7 
            Com(2,31),
            Com(2,0),
        ], 
        // AExp4
        vec![ // 8 
            Com(2,19),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 9 
            Com(1,5),
            Arg(0, false),
        ], 
        vec![ // 10 
            Com(1,2),
            Arg(0, false),
        ], 
        // AExp5
        vec![ // 11 
            Com(1,0),
            Com(1,15),
            Ptr(0, true, true),
        ], 
        vec![ // 12 
            Com(1,8),
            Arg(0, true),
        ], 
        // AExp6
        vec![ // 13 
            Com(2,0),
        ], 
        // AExp7
        vec![ // 14 
            Arg(0, true),
            Com(1,13),
            Com(1,15),
        ], 
        // AExp8
        vec![ // 15 
            Arg(0, true),
            Com(2,1),
            Com(1,14),
        ], 
        // AExp9
        vec![ // 16 
            Com(4,21),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 17 
            Arg(3, true),
            Arg(1, true),
        ], 
        // AExp10
        vec![ // 18 
            Arg(2, true),
            Com(2,0),
            Com(4,16),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp11
        vec![ // 19 
            Y,
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 20 
            Com(3,18),
            Arg(1, true),
        ], 
        // AExp12
        vec![ // 21 
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp13
        vec![ // 22 
            Com(2,0),
        ], 
        // AExp14
        vec![ // 23 
            Com(4,21),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 24 
            Arg(3, true),
            Arg(1, true),
        ], 
        vec![ // 25 
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp15
        vec![ // 26 
            Arg(2, true),
            Com(2,22),
            Com(4,23),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp16
        vec![ // 27 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 28 
            Com(3,26),
            Arg(0, true),
        ], 
        // AExp17
        vec![ // 29 
            Com(1,43),
            Ptr(0, true, true),
        ], 
        vec![ // 30 
            Com(2,52),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp18
        vec![ // 31 
            Com(1,35),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 32 
            Com(2,29),
            Arg(0, true),
            Arg(1, false),
        ], 
        vec![ // 33 
            Com(1,43),
            Arg(1, false),
        ], 
        // AExp19
        vec![ // 34 
            Com(2,1),
        ], 
        // AExp20
        vec![ // 35 
            Arg(0, true),
            Com(1,34),
            Com(1,0),
        ], 
        // AExp21
        vec![ // 36 
            Com(2,1),
        ], 
        // AExp22
        vec![ // 37 
            Com(1,43),
            Ptr(0, true, true),
        ], 
        vec![ // 38 
            Com(4,21),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp23
        vec![ // 39 
            Com(1,45),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 40 
            Com(2,37),
            Arg(0, false),
            Arg(1, true),
        ], 
        vec![ // 41 
            Com(1,35),
            Arg(2, true),
            Arg(0, false),
        ], 
        // AExp24
        vec![ // 42 
            Arg(1, true),
            Com(1,36),
            Com(3,39),
            Arg(0, true),
        ], 
        // AExp25
        vec![ // 43 
            Arg(0, true),
            Com(2,1),
            Com(2,42),
        ], 
        // AExp26
        vec![ // 44 
            Com(2,0),
        ], 
        // AExp27
        vec![ // 45 
            Arg(0, true),
            Com(1,44),
            Com(1,0),
        ], 
        // AExp28
        vec![ // 46 
            Com(4,21),
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp29
        vec![ // 47 
            Com(4,21),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 48 
            Com(2,52),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp30
        vec![ // 49 
            Com(4,21),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 50 
            Com(4,21),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp31
        vec![ // 51 
            Com(1,35),
            Arg(2, false),
            Arg(0, false),
            Com(3,47),
            Com(3,49),
            Arg(2, false),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp32
        vec![ // 52 
            Arg(1, true),
            Com(1,46),
            Com(3,51),
            Arg(0, true),
        ], 
        // AExp33
        vec![ // 53 
            Com(1,27),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 54 
            Com(1,65),
            Arg(0, true),
        ], 
        vec![ // 55 
            Com(4,21),
            Com(2,0),
        ], 
        // AExp34
        vec![ // 56 
            Com(1,27),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 57 
            Com(1,65),
            Arg(0, true),
        ], 
        vec![ // 58 
            Com(4,21),
            Com(2,1),
        ], 
        // AExp35
        vec![ // 59 
            Com(2,19),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 60 
            Com(1,56),
            Arg(0, false),
        ], 
        vec![ // 61 
            Com(1,53),
            Arg(0, false),
        ], 
        // AExp36
        vec![ // 62 
            Com(2,19),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 63 
            Com(1,59),
            Arg(0, false),
        ], 
        vec![ // 64 
            Com(1,65),
            Arg(0, false),
        ], 
        // AExp37
        vec![ // 65 
            Arg(0, true),
            Com(1,62),
            Ptr(0, true, true),
        ], 
        vec![ // 66 
            Com(4,21),
            Com(2,0),
            Com(2,0),
        ], 
        // AExp38
        vec![ // 67 
            Arg(1, true),
            Arg(0, true),
        ], 
    ],

}});