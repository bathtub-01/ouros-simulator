use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 32
#[rustfmt::skip]
pub static BRAUN: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            Com(1,2),
            Ptr(3, false, false),
        ], 
        vec![ // 1 
            Com(2,58),
            Int(0),
            Int(255),
        ], 
        vec![ // 2 
            Com(2,52),
            Int(2),
            Ptr(1, false, false),
        ], 
        vec![ // 3 
            Com(1,8),
            Com(1,13),
            Ptr(2, false, false),
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
            Arg(0, true),
            Int(0),
            Int(1),
        ], 
        // AExp3
        vec![ // 3 
            Com(1,10),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 4 
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 5 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp4
        vec![ // 6 
            Arg(2, true),
            Com(2,1),
            Ptr(0, true, true),
        ], 
        vec![ // 7 
            Com(4,3),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp5
        vec![ // 8 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 9 
            Com(3,6),
            Arg(0, true),
        ], 
        // AExp6
        vec![ // 10 
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp7
        vec![ // 11 
            Com(1,28),
            Ptr(0, true, true),
        ], 
        vec![ // 12 
            Com(1,41),
            Arg(0, true),
        ], 
        // AExp8
        vec![ // 13 
            Com(2,20),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 14 
            Com(1,11),
            Arg(0, false),
        ], 
        // AExp9
        vec![ // 15 
            Com(2,0),
        ], 
        // AExp10
        vec![ // 16 
            Prm(EQ,false),
            Arg(0, true),
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 17 
            Com(2,20),
            Arg(1, true),
            Arg(3, true),
        ], 
        // AExp11
        vec![ // 18 
            Arg(0, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 19 
            Com(4,16),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp12
        vec![ // 20 
            Arg(0, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 21 
            Com(3,18),
            Arg(1, false),
        ], 
        vec![ // 22 
            Arg(1, false),
            Com(2,1),
            Com(2,15),
        ], 
        // AExp13
        vec![ // 23 
            Com(2,37),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 24 
            Com(1,28),
            Arg(1, true),
        ], 
        vec![ // 25 
            Com(1,28),
            Arg(0, true),
        ], 
        // AExp14
        vec![ // 26 
            Com(4,29),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 27 
            Com(2,23),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp15
        vec![ // 28 
            Arg(0, true),
            Com(3,26),
            Com(2,0),
        ], 
        // AExp16
        vec![ // 29 
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp17
        vec![ // 30 
            Com(4,29),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 31 
            Com(2,37),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp18
        vec![ // 32 
            Com(4,29),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 33 
            Com(3,30),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        // AExp19
        vec![ // 34 
            Arg(0, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 35 
            Com(4,32),
            Arg(1, false),
            Arg(2, false),
        ], 
        vec![ // 36 
            Com(4,29),
            Arg(1, false),
            Arg(2, false),
        ], 
        // AExp20
        vec![ // 37 
            Arg(0, true),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 38 
            Com(3,34),
            Arg(1, false),
        ], 
        // AExp21
        vec![ // 39 
            Com(2,44),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 40 
            Com(1,41),
            Arg(1, true),
        ], 
        // AExp22
        vec![ // 41 
            Arg(0, true),
            Com(2,1),
            Com(2,39),
        ], 
        // AExp23
        vec![ // 42 
            Com(5,47),
            Arg(0, true),
            Ptr(0, true, true),
            Arg(2, true),
        ], 
        vec![ // 43 
            Com(2,44),
            Arg(1, true),
            Arg(3, true),
        ], 
        // AExp24
        vec![ // 44 
            Arg(1, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 45 
            Com(5,47),
            Arg(0, false),
            Com(2,1),
            Com(2,1),
        ], 
        vec![ // 46 
            Com(4,42),
            Arg(0, false),
        ], 
        // AExp25
        vec![ // 47 
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp26
        vec![ // 48 
            Com(2,52),
            Ptr(0, true, true),
        ], 
        vec![ // 49 
            Prm(Sub,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp27
        vec![ // 50 
            Com(4,29),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 51 
            Com(1,48),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp28
        vec![ // 52 
            Prm(LE,false),
            Arg(0, false),
            Int(0),
            Ptr(0, true, true),
            Com(2,0),
        ], 
        vec![ // 53 
            Com(2,50),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp29
        vec![ // 54 
            Com(2,58),
            Ptr(0, true, true),
        ], 
        vec![ // 55 
            Prm(Add,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp30
        vec![ // 56 
            Com(4,29),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 57 
            Com(1,54),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp31
        vec![ // 58 
            Prm(LE,false),
            Arg(0, false),
            Arg(1, false),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 59 
            Com(2,56),
            Arg(0, false),
            Arg(1, false),
        ], 
    ],

}});
