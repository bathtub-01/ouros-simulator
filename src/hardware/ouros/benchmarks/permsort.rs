use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 38
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
            Err(3),
            Com(2,0),
        ], 
        // AExp4
        vec![ // 4 
            Com(1,16),
            Com(1,25),
            Ptr(0, true, true),
        ], 
        vec![ // 5 
            Com(1,31),
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
            Com(2,0),
        ], 
        // AExp7
        vec![ // 9 
            Arg(3, true),
        ], 
        // AExp8
        vec![ // 10 
            Com(4,2),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 11 
            Com(1,16),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp9
        vec![ // 12 
            Arg(3, false),
            Arg(1, false),
            Com(4,9),
            Com(4,10),
            Arg(1, false),
            Arg(3, false),
            Arg(2, false),
            Ptr(0, true, true),
        ], 
        vec![ // 13 
            Arg(0, true),
            Arg(2, false),
        ], 
        // AExp10
        vec![ // 14 
            Arg(2, true),
            Com(1,8),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 15 
            Com(4,12),
            Arg(1, true),
        ], 
        // AExp11
        vec![ // 16 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 17 
            Com(3,14),
            Arg(0, true),
        ], 
        // AExp12
        vec![ // 18 
            Com(2,1),
        ], 
        // AExp13
        vec![ // 19 
            Com(1,25),
            Ptr(0, true, true),
        ], 
        vec![ // 20 
            Com(4,2),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp14
        vec![ // 21 
            Com(1,27),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 22 
            Com(2,19),
            Arg(0, false),
            Arg(1, true),
        ], 
        vec![ // 23 
            Prm(LE,false),
            Arg(2, true),
            Arg(0, false),
        ], 
        // AExp15
        vec![ // 24 
            Arg(1, true),
            Com(1,18),
            Com(3,21),
            Arg(0, true),
        ], 
        // AExp16
        vec![ // 25 
            Arg(0, true),
            Com(2,1),
            Com(2,24),
        ], 
        // AExp17
        vec![ // 26 
            Com(2,0),
        ], 
        // AExp18
        vec![ // 27 
            Arg(0, true),
            Com(1,26),
            Com(1,0),
        ], 
        // AExp19
        vec![ // 28 
            Com(1,38),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 29 
            Com(1,31),
            Arg(1, true),
        ], 
        vec![ // 30 
            Com(1,57),
            Arg(0, true),
        ], 
        // AExp20
        vec![ // 31 
            Arg(0, true),
            Ptr(0, true, true),
            Com(2,28),
        ], 
        vec![ // 32 
            Com(4,2),
            Com(2,0),
            Com(2,0),
        ], 
        // AExp21
        vec![ // 33 
            Com(2,0),
        ], 
        // AExp22
        vec![ // 34 
            Com(2,43),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 35 
            Arg(3, true),
            Arg(1, true),
        ], 
        vec![ // 36 
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp23
        vec![ // 37 
            Arg(2, true),
            Com(2,33),
            Com(4,34),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp24
        vec![ // 38 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 39 
            Com(3,37),
            Arg(0, true),
        ], 
        // AExp25
        vec![ // 40 
            Com(4,2),
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 41 
            Arg(3, true),
            Arg(1, true),
        ], 
        // AExp26
        vec![ // 42 
            Arg(2, true),
            Com(2,0),
            Com(4,40),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp27
        vec![ // 43 
            Y,
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 44 
            Com(3,42),
            Arg(1, true),
        ], 
        // AExp28
        vec![ // 45 
            Com(4,2),
            Ptr(0, true, true),
            Com(2,0),
        ], 
        vec![ // 46 
            Com(4,2),
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp29
        vec![ // 47 
            Com(4,2),
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 48 
            Com(4,2),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp30
        vec![ // 49 
            Com(1,64),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 50 
            Arg(0, true),
            Arg(2, true),
        ], 
        vec![ // 51 
            Com(4,2),
            Arg(1, true),
        ], 
        // AExp31
        vec![ // 52 
            Com(4,2),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 53 
            Com(3,49),
            Arg(0, true),
            Arg(1, false),
            Arg(2, false),
        ], 
        vec![ // 54 
            Com(3,47),
            Arg(1, false),
            Arg(2, false),
            Arg(3, true),
        ], 
        // AExp32
        vec![ // 55 
            Arg(2, true),
            Com(1,45),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 56 
            Com(4,52),
            Arg(1, true),
        ], 
        // AExp33
        vec![ // 57 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 58 
            Com(3,55),
            Arg(0, true),
        ], 
        // AExp34
        vec![ // 59 
            Com(2,0),
        ], 
        // AExp35
        vec![ // 60 
            Com(4,2),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 61 
            Arg(3, true),
            Arg(1, true),
        ], 
        vec![ // 62 
            Arg(2, true),
            Arg(0, true),
        ], 
        // AExp36
        vec![ // 63 
            Arg(2, true),
            Com(2,59),
            Com(4,60),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp37
        vec![ // 64 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 65 
            Com(3,63),
            Arg(0, true),
        ], 
    ],

}});