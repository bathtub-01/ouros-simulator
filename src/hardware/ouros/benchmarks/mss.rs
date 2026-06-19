use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 31
#[rustfmt::skip]
pub static MSS: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            Com(1,4),
            Ptr(2, false, false),
        ], 
        vec![ // 1 
            Prm(Sub,false),
            Int(0),
            Int(20),
        ], 
        vec![ // 2 
            Com(2,58),
            Ptr(1, false, false),
            Int(20),
        ], 
        // AExp1
        vec![ // 3 
            Com(2,13),
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
            Com(1,20),
            Ptr(3, false, false),
            Ptr(0, true, true),
        ], 
        vec![ // 3 
            Com(1,23),
            Arg(0, true),
        ], 
        // AExp3
        vec![ // 4 
            Com(1,7),
            Ptr(0, true, true),
        ], 
        vec![ // 5 
            Com(1,2),
            Arg(0, true),
        ], 
        // AExp4
        vec![ // 6 
            Prm(LE,true),
            Arg(0, false),
            Arg(1, false),
            Arg(1, false),
            Arg(0, false),
        ], 
        // AExp5
        vec![ // 7 
            Arg(0, true),
            Int(0),
            Ptr(0, true, true),
        ], 
        vec![ // 8 
            Com(2,13),
            Com(2,6),
        ], 
        // AExp6
        vec![ // 9 
            Arg(0, true),
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 10 
            Arg(1, true),
            Arg(3, true),
        ], 
        // AExp7
        vec![ // 11 
            Arg(3, true),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 12 
            Com(4,9),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp8
        vec![ // 13 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 14 
            Com(4,11),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp9
        vec![ // 15 
            Com(4,22),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 16 
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 17 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp10
        vec![ // 18 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 19 
            Com(4,15),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp11
        vec![ // 20 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 21 
            Com(3,18),
            Arg(0, true),
        ], 
        // AExp12
        vec![ // 22 
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp13
        vec![ // 23 
            Com(1,30),
            Com(1,41),
            Ptr(0, true, true),
        ], 
        vec![ // 24 
            Com(1,46),
            Arg(0, true),
        ], 
        // AExp14
        vec![ // 25 
            Com(2,36),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 26 
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 27 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp15
        vec![ // 28 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 29 
            Com(4,25),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp16
        vec![ // 30 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 31 
            Com(3,28),
            Arg(0, true),
        ], 
        // AExp17
        vec![ // 32 
            Com(4,22),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 33 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp18
        vec![ // 34 
            Arg(2, true),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 35 
            Com(3,32),
            Arg(1, true),
        ], 
        // AExp19
        vec![ // 36 
            Y,
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 37 
            Com(3,34),
            Arg(1, true),
        ], 
        // AExp20
        vec![ // 38 
            Com(4,22),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 39 
            Com(1,41),
            Arg(1, false),
        ], 
        vec![ // 40 
            Com(4,22),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp21
        vec![ // 41 
            Arg(0, true),
            Com(2,0),
            Com(2,38),
        ], 
        // AExp22
        vec![ // 42 
            Com(1,46),
            Ptr(0, true, true),
        ], 
        vec![ // 43 
            Com(1,53),
            Arg(0, true),
        ], 
        // AExp23
        vec![ // 44 
            Com(4,22),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 45 
            Com(1,42),
            Arg(0, false),
        ], 
        // AExp24
        vec![ // 46 
            Arg(0, false),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 47 
            Com(3,44),
            Arg(0, false),
        ], 
        vec![ // 48 
            Com(4,22),
            Com(2,0),
            Com(2,0),
        ], 
        // AExp25
        vec![ // 49 
            Com(4,22),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 50 
            Com(1,53),
            Arg(1, true),
        ], 
        // AExp26
        vec![ // 51 
            Arg(1, false),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 52 
            Com(4,49),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp27
        vec![ // 53 
            Arg(0, true),
            Err(1),
            Com(2,51),
        ], 
        // AExp28
        vec![ // 54 
            Com(2,58),
            Ptr(0, true, true),
        ], 
        vec![ // 55 
            Prm(Add,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp29
        vec![ // 56 
            Com(4,22),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 57 
            Com(1,54),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp30
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
