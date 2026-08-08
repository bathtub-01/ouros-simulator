use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 37
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
            Com(2,12),
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
            Com(1,19),
            Ptr(3, false, false),
            Ptr(0, true, true),
        ], 
        vec![ // 3 
            Com(1,22),
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
            Com(2,1),
            Com(2,0),
            Arg(0, false),
            Arg(1, false),
        ], 
        // AExp5
        vec![ // 7 
            Arg(0, true),
            Int(0),
            Ptr(0, true, true),
        ], 
        vec![ // 8 
            Com(2,12),
            Com(2,6),
        ], 
        // AExp6
        vec![ // 9 
            Arg(3, true),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 10 
            Arg(4, true),
            Arg(1, true),
        ], 
        // AExp7
        vec![ // 11 
            Arg(3, true),
            Com(3,0),
            Com(5,9),
            Arg(1, true),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp8
        vec![ // 12 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 13 
            Com(4,11),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp9
        vec![ // 14 
            Com(2,0),
        ], 
        // AExp10
        vec![ // 15 
            Com(4,21),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 16 
            Arg(3, true),
            Arg(1, true),
        ], 
        vec![ // 17 
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp11
        vec![ // 18 
            Arg(2, true),
            Com(2,14),
            Com(4,15),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp12
        vec![ // 19 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 20 
            Com(3,18),
            Arg(0, true),
        ], 
        // AExp13
        vec![ // 21 
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp14
        vec![ // 22 
            Com(1,29),
            Com(1,39),
            Ptr(0, true, true),
        ], 
        vec![ // 23 
            Com(1,45),
            Arg(0, true),
        ], 
        // AExp15
        vec![ // 24 
            Com(2,0),
        ], 
        // AExp16
        vec![ // 25 
            Com(2,34),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 26 
            Arg(3, true),
            Arg(1, true),
        ], 
        vec![ // 27 
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp17
        vec![ // 28 
            Arg(2, true),
            Com(2,24),
            Com(4,25),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp18
        vec![ // 29 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 30 
            Com(3,28),
            Arg(0, true),
        ], 
        // AExp19
        vec![ // 31 
            Com(4,21),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 32 
            Arg(3, true),
            Arg(1, true),
        ], 
        // AExp20
        vec![ // 33 
            Arg(2, true),
            Com(2,0),
            Com(4,31),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp21
        vec![ // 34 
            Y,
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 35 
            Com(3,33),
            Arg(1, true),
        ], 
        // AExp22
        vec![ // 36 
            Com(4,21),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 37 
            Com(1,39),
            Arg(1, false),
        ], 
        vec![ // 38 
            Com(4,21),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp23
        vec![ // 39 
            Arg(0, true),
            Com(2,0),
            Com(2,36),
        ], 
        // AExp24
        vec![ // 40 
            Com(4,21),
            Com(2,0),
            Com(2,0),
        ], 
        // AExp25
        vec![ // 41 
            Com(1,45),
            Ptr(0, true, true),
        ], 
        vec![ // 42 
            Com(1,52),
            Arg(0, true),
        ], 
        // AExp26
        vec![ // 43 
            Com(4,21),
            Arg(2, false),
            Ptr(0, true, true),
        ], 
        vec![ // 44 
            Com(1,41),
            Arg(2, false),
        ], 
        // AExp27
        vec![ // 45 
            Arg(0, false),
            Com(1,40),
            Com(3,43),
            Arg(0, false),
        ], 
        // AExp28
        vec![ // 46 
            Com(2,0),
        ], 
        // AExp29
        vec![ // 47 
            Arg(2, true),
        ], 
        // AExp30
        vec![ // 48 
            Com(4,21),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 49 
            Com(1,52),
            Arg(1, true),
        ], 
        // AExp31
        vec![ // 50 
            Arg(1, false),
            Com(1,46),
            Com(3,47),
            Ptr(0, true, true),
        ], 
        vec![ // 51 
            Com(2,48),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp32
        vec![ // 52 
            Arg(0, true),
            Err(1),
            Com(2,50),
        ], 
        // AExp33
        vec![ // 53 
            Com(2,0),
        ], 
        // AExp34
        vec![ // 54 
            Com(2,58),
            Ptr(0, true, true),
        ], 
        vec![ // 55 
            Prm(Add,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp35
        vec![ // 56 
            Com(4,21),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 57 
            Com(1,54),
            Arg(0, false),
            Arg(1, true),
        ], 
        // AExp36
        vec![ // 58 
            Prm(LE,false),
            Arg(0, false),
            Arg(1, false),
            Com(2,53),
            Com(2,56),
            Arg(0, false),
            Arg(1, false),
        ], 
    ],

}});