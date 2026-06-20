use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 31
#[rustfmt::skip]
pub static QUEENS: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            Com(1,2),
            Int(6),
        ], 
        // AExp1
        vec![ // 1 
            Y,
            Com(3,6),
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
            Ptr(1, false, false),
            Ptr(0, true, true),
        ], 
        vec![ // 3 
            Com(1,17),
            Arg(0, false),
            Arg(0, false),
        ], 
        // AExp3
        vec![ // 4 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 5 
            Prm(Add,false),
            Arg(1, true),
            Int(1),
        ], 
        // AExp4
        vec![ // 6 
            Arg(2, true),
            Arg(1, false),
            Ptr(0, true, true),
        ], 
        vec![ // 7 
            Com(3,4),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp5
        vec![ // 8 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 9 
            Prm(Sub,false),
            Arg(1, true),
            Int(1),
        ], 
        // AExp6
        vec![ // 10 
            Com(1,24),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 11 
            Com(2,8),
            Arg(1, true),
            Arg(2, true),
        ], 
        vec![ // 12 
            Com(2,32),
            Arg(0, true),
        ], 
        // AExp7
        vec![ // 13 
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp8
        vec![ // 14 
            Prm(EQ,false),
            Arg(2, false),
            Int(0),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 15 
            Com(4,13),
            Com(2,0),
            Com(2,0),
        ], 
        vec![ // 16 
            Com(3,10),
            Arg(0, true),
            Arg(1, true),
            Arg(2, false),
        ], 
        // AExp9
        vec![ // 17 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 18 
            Com(3,14),
            Arg(0, true),
        ], 
        // AExp10
        vec![ // 19 
            Com(2,30),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 20 
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 21 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp11
        vec![ // 22 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 23 
            Com(4,19),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp12
        vec![ // 24 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 25 
            Com(3,22),
            Arg(0, true),
        ], 
        // AExp13
        vec![ // 26 
            Com(4,13),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 27 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp14
        vec![ // 28 
            Arg(2, true),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 29 
            Com(3,26),
            Arg(1, true),
        ], 
        // AExp15
        vec![ // 30 
            Y,
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 31 
            Com(3,28),
            Arg(1, true),
        ], 
        // AExp16
        vec![ // 32 
            Com(1,24),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 33 
            Com(1,63),
            Arg(0, true),
        ], 
        vec![ // 34 
            Com(2,37),
            Arg(1, true),
        ], 
        // AExp17
        vec![ // 35 
            Com(4,13),
            Ptr(0, true, true),
            Com(2,0),
        ], 
        vec![ // 36 
            Com(4,13),
            Arg(1, true),
            Arg(0, true),
        ], 
        // AExp18
        vec![ // 37 
            Com(1,56),
            Arg(1, false),
            Int(1),
            Arg(0, false),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 38 
            Com(2,35),
            Arg(0, false),
            Arg(1, false),
        ], 
        // AExp19
        vec![ // 39 
            Prm(EQ,true),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 40 
            Prm(Add,false),
            Arg(2, true),
            Arg(1, true),
        ], 
        // AExp20
        vec![ // 41 
            Prm(EQ,true),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 42 
            Prm(Sub,false),
            Arg(2, true),
            Arg(1, true),
        ], 
        // AExp21
        vec![ // 43 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 44 
            Prm(Add,false),
            Arg(1, true),
            Int(1),
        ], 
        // AExp22
        vec![ // 45 
            Com(1,58),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 46 
            Com(2,43),
            Arg(1, true),
            Arg(2, false),
            Arg(4, true),
        ], 
        vec![ // 47 
            Com(3,41),
            Arg(0, true),
            Arg(2, false),
            Arg(3, true),
        ], 
        // AExp23
        vec![ // 48 
            Com(1,58),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 49 
            Com(5,45),
            Arg(0, false),
            Arg(1, true),
            Arg(2, false),
            Arg(3, false),
            Arg(4, true),
        ], 
        vec![ // 50 
            Com(3,39),
            Arg(0, false),
            Arg(2, false),
            Arg(3, false),
        ], 
        // AExp24
        vec![ // 51 
            Com(1,58),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 52 
            Com(5,48),
            Arg(0, false),
            Arg(1, true),
            Arg(2, true),
            Arg(3, false),
            Arg(4, true),
        ], 
        vec![ // 53 
            Prm(EQ,true),
            Arg(0, false),
            Arg(3, false),
        ], 
        // AExp25
        vec![ // 54 
            Arg(3, true),
            Com(2,1),
            Ptr(0, true, true),
        ], 
        vec![ // 55 
            Com(5,51),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp26
        vec![ // 56 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 57 
            Com(4,54),
            Arg(0, true),
        ], 
        // AExp27
        vec![ // 58 
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp28
        vec![ // 59 
            Com(1,63),
            Ptr(0, true, true),
        ], 
        vec![ // 60 
            Prm(Sub,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp29
        vec![ // 61 
            Com(4,13),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 62 
            Com(1,59),
            Arg(0, false),
        ], 
        // AExp30
        vec![ // 63 
            Prm(EQ,false),
            Arg(0, false),
            Int(1),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 64 
            Com(4,13),
            Int(1),
            Com(2,0),
        ], 
        vec![ // 65 
            Com(1,61),
            Arg(0, false),
        ], 
    ],

}});
