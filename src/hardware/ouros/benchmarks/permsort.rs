use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 32
#[rustfmt::skip]
pub static PERMSORT: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            Com(1,3),
            Ptr(8, false, false),
        ], 
        vec![ // 1 
            Com(4,2),
            Int(12),
            Com(2,0),
        ], 
        vec![ // 2 
            Com(4,2),
            Int(12),
            Ptr(1, false, false),
        ], 
        vec![ // 3 
            Com(4,2),
            Int(6),
            Ptr(2, false, false),
        ], 
        vec![ // 4 
            Com(4,2),
            Int(9),
            Ptr(3, false, false),
        ], 
        vec![ // 5 
            Com(4,2),
            Int(7),
            Ptr(4, false, false),
        ], 
        vec![ // 6 
            Com(4,2),
            Int(6),
            Ptr(5, false, false),
        ], 
        vec![ // 7 
            Com(4,2),
            Int(10),
            Ptr(6, false, false),
        ], 
        vec![ // 8 
            Com(1,6),
            Ptr(7, false, false),
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
            E(3),
            Com(2,0),
        ], 
        // AExp4
        vec![ // 4 
            Com(1,15),
            Com(1,24),
            Ptr(0, true, true),
        ], 
        vec![ // 5 
            Com(1,29),
            Arg(0, true),
        ], 
        // AExp5
        vec![ // 6 
            Com(1,3),
            Ptr(0, true, true),
        ], 
        vec![ // 7 
            Com(1,4),
            Arg(0, true),
        ], 
        // AExp6
        vec![ // 8 
            Com(4,2),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 9 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp7
        vec![ // 10 
            Arg(0, true),
            Arg(2, false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 11 
            Com(3,8),
            Arg(1, false),
            Arg(2, false),
            Arg(3, false),
        ], 
        vec![ // 12 
            Arg(1, false),
            Arg(3, false),
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
            Com(1,24),
            Ptr(0, true, true),
        ], 
        vec![ // 18 
            Com(4,2),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp11
        vec![ // 19 
            Com(1,25),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 20 
            Com(2,17),
            Arg(1, false),
            Arg(2, true),
        ], 
        vec![ // 21 
            Prm(LE,false),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp12
        vec![ // 22 
            Arg(1, true),
            Com(2,1),
            Ptr(0, true, true),
        ], 
        vec![ // 23 
            Com(3,19),
            Arg(0, true),
        ], 
        // AExp13
        vec![ // 24 
            Arg(0, true),
            Com(2,1),
            Com(2,22),
        ], 
        // AExp14
        vec![ // 25 
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp15
        vec![ // 26 
            Com(1,36),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 27 
            Com(1,29),
            Arg(1, true),
        ], 
        vec![ // 28 
            Com(1,57),
            Arg(0, true),
        ], 
        // AExp16
        vec![ // 29 
            Arg(0, true),
            Ptr(0, true, true),
            Com(2,26),
        ], 
        vec![ // 30 
            Com(4,2),
            Com(2,0),
            Com(2,0),
        ], 
        // AExp17
        vec![ // 31 
            Com(2,42),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 32 
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 33 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp18
        vec![ // 34 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 35 
            Com(4,31),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp19
        vec![ // 36 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 37 
            Com(3,34),
            Arg(0, true),
        ], 
        // AExp20
        vec![ // 38 
            Com(4,2),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 39 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp21
        vec![ // 40 
            Arg(2, true),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 41 
            Com(3,38),
            Arg(1, true),
        ], 
        // AExp22
        vec![ // 42 
            Y,
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 43 
            Com(3,40),
            Arg(1, true),
        ], 
        // AExp23
        vec![ // 44 
            Com(4,2),
            Ptr(0, true, true),
            Com(2,0),
        ], 
        vec![ // 45 
            Com(4,2),
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp24
        vec![ // 46 
            Com(4,2),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 47 
            Com(4,2),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp25
        vec![ // 48 
            Com(1,64),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 49 
            Arg(0, true),
            Arg(2, true),
        ], 
        vec![ // 50 
            Com(4,2),
            Arg(1, true),
        ], 
        // AExp26
        vec![ // 51 
            Com(4,2),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 52 
            Com(3,48),
            Arg(1, true),
            Arg(2, false),
            Arg(3, false),
        ], 
        vec![ // 53 
            Com(3,46),
            Arg(0, true),
            Arg(2, false),
            Arg(3, false),
        ], 
        // AExp27
        vec![ // 54 
            Arg(2, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 55 
            Com(4,51),
            Arg(0, false),
            Arg(1, true),
        ], 
        vec![ // 56 
            Com(1,44),
            Arg(0, false),
        ], 
        // AExp28
        vec![ // 57 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 58 
            Com(3,54),
            Arg(0, true),
        ], 
        // AExp29
        vec![ // 59 
            Com(4,2),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 60 
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 61 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp30
        vec![ // 62 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 63 
            Com(4,59),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp31
        vec![ // 64 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 65 
            Com(3,62),
            Arg(0, true),
        ], 
    ],

}});
