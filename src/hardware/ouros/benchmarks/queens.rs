use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 36
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
            Com(2,14),
            Arg(0, false),
            Arg(0, false),
        ], 
        // AExp3
        vec![ // 4 
            Arg(3, true),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 5 
            Prm(Add,false),
            Arg(2, true),
            Int(1),
        ], 
        // AExp4
        vec![ // 6 
            Arg(2, true),
            Com(2,0),
            Com(4,4),
            Arg(1, true),
            Arg(0, true),
        ], 
        // AExp5
        vec![ // 7 
            Com(2,14),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 8 
            Prm(Sub,false),
            Arg(1, true),
            Int(1),
        ], 
        // AExp6
        vec![ // 9 
            Com(1,20),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 10 
            Com(2,7),
            Arg(0, false),
            Arg(1, true),
        ], 
        vec![ // 11 
            Com(2,27),
            Arg(0, false),
        ], 
        // AExp7
        vec![ // 12 
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp8
        vec![ // 13 
            Com(4,12),
            Com(2,0),
            Com(2,0),
        ], 
        // AExp9
        vec![ // 14 
            Prm(EQ,false),
            Arg(1, false),
            Int(0),
            Com(2,9),
            Com(2,13),
            Arg(0, true),
            Arg(1, false),
        ], 
        // AExp10
        vec![ // 15 
            Com(2,0),
        ], 
        // AExp11
        vec![ // 16 
            Com(2,25),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 17 
            Arg(3, true),
            Arg(1, true),
        ], 
        vec![ // 18 
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp12
        vec![ // 19 
            Arg(2, true),
            Com(2,15),
            Com(4,16),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp13
        vec![ // 20 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 21 
            Com(3,19),
            Arg(0, true),
        ], 
        // AExp14
        vec![ // 22 
            Com(4,12),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 23 
            Arg(3, true),
            Arg(1, true),
        ], 
        // AExp15
        vec![ // 24 
            Arg(2, true),
            Com(2,0),
            Com(4,22),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp16
        vec![ // 25 
            Y,
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 26 
            Com(3,24),
            Arg(1, true),
        ], 
        // AExp17
        vec![ // 27 
            Com(1,20),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 28 
            Com(1,61),
            Arg(0, true),
        ], 
        vec![ // 29 
            Com(2,33),
            Arg(1, true),
        ], 
        // AExp18
        vec![ // 30 
            Com(2,0),
        ], 
        // AExp19
        vec![ // 31 
            Com(4,12),
            Ptr(0, true, true),
            Com(2,0),
        ], 
        vec![ // 32 
            Com(4,12),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp20
        vec![ // 33 
            Com(1,52),
            Arg(1, false),
            Int(1),
            Arg(0, false),
            Com(2,30),
            Com(2,31),
            Arg(1, false),
            Arg(0, false),
        ], 
        // AExp21
        vec![ // 34 
            Com(2,1),
        ], 
        // AExp22
        vec![ // 35 
            Prm(EQ,true),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 36 
            Prm(Add,false),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp23
        vec![ // 37 
            Prm(EQ,true),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 38 
            Prm(Sub,false),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp24
        vec![ // 39 
            Arg(0, true),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 40 
            Prm(Add,false),
            Arg(2, true),
            Int(1),
        ], 
        // AExp25
        vec![ // 41 
            Com(1,55),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 42 
            Com(3,39),
            Arg(0, true),
            Arg(2, true),
            Arg(4, false),
        ], 
        vec![ // 43 
            Com(3,37),
            Arg(1, true),
            Arg(3, true),
            Arg(4, false),
        ], 
        // AExp26
        vec![ // 44 
            Com(1,55),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 45 
            Com(5,41),
            Arg(0, true),
            Arg(1, false),
            Arg(2, true),
            Arg(3, false),
            Arg(4, false),
        ], 
        vec![ // 46 
            Com(3,35),
            Arg(1, false),
            Arg(3, false),
            Arg(4, false),
        ], 
        // AExp27
        vec![ // 47 
            Com(1,55),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 48 
            Com(5,44),
            Arg(0, true),
            Arg(1, false),
            Arg(2, true),
            Arg(3, false),
            Arg(4, true),
        ], 
        vec![ // 49 
            Prm(EQ,true),
            Arg(3, false),
            Arg(1, false),
        ], 
        // AExp28
        vec![ // 50 
            Arg(3, true),
            Com(2,34),
            Ptr(0, true, true),
            Arg(0, true),
            Arg(2, true),
        ], 
        vec![ // 51 
            Com(5,47),
            Arg(1, true),
        ], 
        // AExp29
        vec![ // 52 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 53 
            Com(4,50),
            Arg(0, true),
        ], 
        // AExp30
        vec![ // 54 
            Com(2,0),
        ], 
        // AExp31
        vec![ // 55 
            Arg(0, true),
            Com(1,54),
            Com(1,0),
        ], 
        // AExp32
        vec![ // 56 
            Com(1,61),
            Ptr(0, true, true),
        ], 
        vec![ // 57 
            Prm(Sub,false),
            Arg(0, true),
            Int(1),
        ], 
        // AExp33
        vec![ // 58 
            Com(4,12),
            Arg(0, false),
            Ptr(0, true, true),
        ], 
        vec![ // 59 
            Com(1,56),
            Arg(0, false),
        ], 
        // AExp34
        vec![ // 60 
            Com(4,12),
            Int(1),
            Com(2,0),
        ], 
        // AExp35
        vec![ // 61 
            Prm(EQ,false),
            Arg(0, false),
            Int(1),
            Com(1,58),
            Com(1,60),
            Arg(0, false),
        ], 
    ],

}});