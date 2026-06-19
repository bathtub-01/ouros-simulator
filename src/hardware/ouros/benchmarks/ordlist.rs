use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 34
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
            Com(3,71),
            Com(2,1),
        ], 
        vec![ // 2 
            Com(1,0),
            Com(3,71),
            Ptr(1, false, false),
        ], 
        vec![ // 3 
            Com(1,0),
            Com(3,71),
            Ptr(2, false, false),
        ], 
        vec![ // 4 
            Com(1,0),
            Com(3,71),
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
            Com(1,28),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 3 
            Com(1,69),
            Arg(0, true),
        ], 
        vec![ // 4 
            Com(2,32),
            Com(2,1),
        ], 
        // AExp3
        vec![ // 5 
            Com(1,28),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 6 
            Com(1,69),
            Arg(0, true),
        ], 
        vec![ // 7 
            Com(2,32),
            Com(2,0),
        ], 
        // AExp4
        vec![ // 8 
            Com(2,20),
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
            Arg(0, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 14 
            Com(1,15),
            Arg(1, true),
        ], 
        // AExp7
        vec![ // 15 
            Arg(0, true),
            Com(2,1),
            Com(2,13),
        ], 
        // AExp8
        vec![ // 16 
            Com(4,22),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 17 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp9
        vec![ // 18 
            Arg(2, true),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 19 
            Com(3,16),
            Arg(1, true),
        ], 
        // AExp10
        vec![ // 20 
            Y,
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 21 
            Com(3,18),
            Arg(1, true),
        ], 
        // AExp11
        vec![ // 22 
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp12
        vec![ // 23 
            Com(4,22),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 24 
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 25 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp13
        vec![ // 26 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 27 
            Com(4,23),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp14
        vec![ // 28 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 29 
            Com(3,26),
            Arg(0, true),
        ], 
        // AExp15
        vec![ // 30 
            Com(1,43),
            Ptr(0, true, true),
        ], 
        vec![ // 31 
            Com(1,55),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp16
        vec![ // 32 
            Com(1,35),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 33 
            Com(2,30),
            Arg(0, true),
            Arg(1, false),
        ], 
        vec![ // 34 
            Com(1,43),
            Arg(1, false),
        ], 
        // AExp17
        vec![ // 35 
            Arg(0, true),
            Com(2,1),
        ], 
        // AExp18
        vec![ // 36 
            Com(1,43),
            Ptr(0, true, true),
        ], 
        vec![ // 37 
            Com(4,22),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp19
        vec![ // 38 
            Com(1,44),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 39 
            Com(2,36),
            Arg(1, false),
            Arg(2, true),
        ], 
        vec![ // 40 
            Com(1,35),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp20
        vec![ // 41 
            Arg(1, true),
            Com(2,1),
            Ptr(0, true, true),
        ], 
        vec![ // 42 
            Com(3,38),
            Arg(0, true),
        ], 
        // AExp21
        vec![ // 43 
            Arg(0, true),
            Com(2,1),
            Com(2,41),
        ], 
        // AExp22
        vec![ // 44 
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp23
        vec![ // 45 
            Com(4,22),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 46 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp24
        vec![ // 47 
            Com(4,22),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 48 
            Com(4,22),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp25
        vec![ // 49 
            Com(1,35),
            Arg(0, false),
            Arg(2, false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 50 
            Com(3,47),
            Arg(0, false),
            Arg(2, false),
            Arg(3, false),
        ], 
        vec![ // 51 
            Com(3,45),
            Arg(1, true),
            Arg(2, false),
            Arg(3, false),
        ], 
        // AExp26
        vec![ // 52 
            Arg(2, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 53 
            Com(4,49),
            Arg(0, false),
            Arg(1, true),
        ], 
        vec![ // 54 
            Com(4,22),
            Arg(0, false),
            Com(2,0),
        ], 
        // AExp27
        vec![ // 55 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 56 
            Com(3,52),
            Arg(0, true),
        ], 
        // AExp28
        vec![ // 57 
            Com(1,28),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 58 
            Com(1,69),
            Arg(0, true),
        ], 
        vec![ // 59 
            Com(4,22),
            Com(2,0),
        ], 
        // AExp29
        vec![ // 60 
            Com(1,28),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 61 
            Com(1,69),
            Arg(0, true),
        ], 
        vec![ // 62 
            Com(4,22),
            Com(2,1),
        ], 
        // AExp30
        vec![ // 63 
            Com(2,20),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 64 
            Com(1,60),
            Arg(0, false),
        ], 
        vec![ // 65 
            Com(1,57),
            Arg(0, false),
        ], 
        // AExp31
        vec![ // 66 
            Com(2,20),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 67 
            Com(1,63),
            Arg(0, false),
        ], 
        vec![ // 68 
            Com(1,69),
            Arg(0, false),
        ], 
        // AExp32
        vec![ // 69 
            Arg(0, true),
            Com(1,66),
            Ptr(0, true, true),
        ], 
        vec![ // 70 
            Com(4,22),
            Com(2,0),
            Com(2,0),
        ], 
        // AExp33
        vec![ // 71 
            Arg(1, true),
            Arg(0, true),
        ], 
    ],

}});
