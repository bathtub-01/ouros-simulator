use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Combinators in this file: 43
#[rustfmt::skip]
pub static TRIBELIE: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            Com(1,5),
            Com(1,28),
            Ptr(1, false, false),
        ], 
        // AExp1
        vec![ // 1 
            Y,
            Ptr(26, false, false),
            Ptr(3, false, false),
        ], 
        vec![ // 2 
            Com(4,20),
            Com(2,1),
            Com(2,0),
        ], 
        vec![ // 3 
            Com(4,20),
            Com(2,0),
            Ptr(2, false, false),
        ], 
        vec![ // 4 
            Com(4,20),
            Com(2,1),
            Com(2,0),
        ], 
        vec![ // 5 
            Com(4,20),
            Com(2,0),
            Ptr(4, false, false),
        ], 
        vec![ // 6 
            Com(4,20),
            Com(2,1),
            Com(2,0),
        ], 
        vec![ // 7 
            Com(4,20),
            Com(2,0),
            Ptr(6, false, false),
        ], 
        vec![ // 8 
            Com(4,20),
            Com(2,1),
            Com(2,0),
        ], 
        vec![ // 9 
            Com(4,20),
            Com(2,0),
            Ptr(8, false, false),
        ], 
        vec![ // 10 
            Com(4,20),
            Com(2,1),
            Com(2,0),
        ], 
        vec![ // 11 
            Com(4,20),
            Com(2,0),
            Ptr(10, false, false),
        ], 
        vec![ // 12 
            Com(4,20),
            Com(2,1),
            Com(2,0),
        ], 
        vec![ // 13 
            Com(4,20),
            Com(2,0),
            Ptr(12, false, false),
        ], 
        vec![ // 14 
            Com(4,20),
            Com(2,1),
            Com(2,0),
        ], 
        vec![ // 15 
            Com(4,20),
            Com(2,0),
            Ptr(14, false, false),
        ], 
        vec![ // 16 
            Com(4,20),
            Com(2,1),
            Com(2,0),
        ], 
        vec![ // 17 
            Com(4,20),
            Com(2,0),
            Ptr(16, false, false),
        ], 
        vec![ // 18 
            Com(2,78),
            Ptr(17, false, false),
        ], 
        vec![ // 19 
            Com(6,65),
            Ptr(18, false, false),
            Ptr(15, false, false),
        ], 
        vec![ // 20 
            Com(5,61),
            Ptr(19, false, false),
            Ptr(13, false, false),
        ], 
        vec![ // 21 
            Com(4,57),
            Ptr(20, false, false),
            Ptr(11, false, false),
        ], 
        vec![ // 22 
            Com(3,53),
            Ptr(21, false, false),
            Ptr(9, false, false),
        ], 
        vec![ // 23 
            Com(6,49),
            Ptr(22, false, false),
            Ptr(7, false, false),
        ], 
        vec![ // 24 
            Com(6,43),
            Ptr(23, false, false),
        ], 
        vec![ // 25 
            Com(5,41),
            Ptr(24, false, false),
            Ptr(5, false, false),
        ], 
        vec![ // 26 
            Com(3,39),
            Ptr(25, false, false),
        ], 
        // AExp2
        vec![ // 27 
            Com(2,11),
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
            Com(1,0),
            Ptr(27, false, false),
            Ptr(0, true, true),
        ], 
        vec![ // 3 
            Com(1,18),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp3
        vec![ // 4 
            Arg(0, true),
            Arg(1, true),
            Int(0),
            Int(1),
        ], 
        // AExp4
        vec![ // 5 
            Com(2,2),
            Ptr(0, true, true),
        ], 
        vec![ // 6 
            Com(2,4),
            Arg(0, true),
        ], 
        // AExp5
        vec![ // 7 
            Arg(0, true),
            Arg(2, true),
            Ptr(0, true, true),
        ], 
        vec![ // 8 
            Arg(1, true),
            Arg(3, true),
        ], 
        // AExp6
        vec![ // 9 
            Arg(3, true),
            Arg(1, true),
            Ptr(0, true, true),
        ], 
        vec![ // 10 
            Com(4,7),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp7
        vec![ // 11 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 12 
            Com(4,9),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp8
        vec![ // 13 
            Com(4,20),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 14 
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 15 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp9
        vec![ // 16 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 17 
            Com(4,13),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp10
        vec![ // 18 
            Y,
            Ptr(0, true, true),
        ], 
        vec![ // 19 
            Com(3,16),
            Arg(0, true),
        ], 
        // AExp11
        vec![ // 20 
            Arg(3, true),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp12
        vec![ // 21 
            Com(1,29),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 22 
            Com(2,30),
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp13
        vec![ // 23 
            Com(1,29),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 24 
            Com(3,36),
            Arg(1, true),
            Arg(2, true),
            Arg(3, false),
        ], 
        vec![ // 25 
            Com(2,32),
            Arg(0, true),
            Arg(3, false),
        ], 
        // AExp14
        vec![ // 26 
            Com(7,21),
            Arg(2, false),
            Ptr(0, true, true),
            Arg(3, false),
        ], 
        vec![ // 27 
            Com(4,23),
            Arg(0, true),
            Arg(1, true),
            Arg(2, false),
            Arg(3, false),
        ], 
        // AExp15
        vec![ // 28 
            Arg(0, true),
            Com(4,26),
        ], 
        // AExp16
        vec![ // 29 
            Arg(0, true),
            Com(2,0),
        ], 
        // AExp17
        vec![ // 30 
            Arg(0, true),
            Ptr(0, true, true),
            Com(2,1),
        ], 
        vec![ // 31 
            Arg(1, true),
            Com(2,1),
            Com(2,0),
        ], 
        // AExp18
        vec![ // 32 
            Arg(0, true),
            Ptr(0, true, true),
            Com(2,1),
        ], 
        vec![ // 33 
            Arg(1, true),
            Com(2,1),
            Com(2,0),
        ], 
        // AExp19
        vec![ // 34 
            Arg(0, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 35 
            Arg(1, true),
            Com(2,1),
            Com(2,0),
        ], 
        // AExp20
        vec![ // 36 
            Arg(0, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 37 
            Arg(2, false),
            Com(2,0),
            Com(2,1),
        ], 
        vec![ // 38 
            Com(2,34),
            Arg(1, true),
            Arg(2, false),
        ], 
        // AExp21
        vec![ // 39 
            Arg(2, true),
            Com(2,0),
            Ptr(0, true, true),
        ], 
        vec![ // 40 
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp22
        vec![ // 41 
            Y,
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 42 
            Arg(0, true),
            Arg(2, true),
            Arg(3, true),
            Arg(4, true),
        ], 
        // AExp23
        vec![ // 43 
            Arg(5, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 44 
            Arg(0, true),
            Arg(2, true),
            Arg(4, true),
        ], 
        vec![ // 45 
            Arg(1, true),
            Arg(3, true),
        ], 
        // AExp24
        vec![ // 46 
            Arg(6, true),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 47 
            Arg(0, true),
            Arg(1, true),
            Arg(3, true),
            Arg(5, true),
        ], 
        vec![ // 48 
            Arg(2, true),
            Arg(4, true),
        ], 
        // AExp25
        vec![ // 49 
            Y,
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 50 
            Com(7,46),
            Arg(0, true),
            Arg(2, true),
            Arg(3, true),
            Arg(4, true),
            Arg(5, true),
        ], 
        // AExp26
        vec![ // 51 
            Y,
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 52 
            Com(7,46),
            Arg(0, true),
            Arg(2, true),
            Arg(3, true),
            Arg(4, true),
            Arg(5, true),
        ], 
        // AExp27
        vec![ // 53 
            Com(6,51),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 54 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp28
        vec![ // 55 
            Y,
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 56 
            Com(7,46),
            Arg(0, true),
            Arg(2, true),
            Arg(3, true),
            Arg(4, true),
            Arg(5, true),
        ], 
        // AExp29
        vec![ // 57 
            Com(6,55),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 58 
            Arg(0, true),
            Arg(2, true),
            Arg(3, true),
        ], 
        // AExp30
        vec![ // 59 
            Y,
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 60 
            Com(7,46),
            Arg(0, true),
            Arg(2, true),
            Arg(3, true),
            Arg(4, true),
            Arg(5, true),
        ], 
        // AExp31
        vec![ // 61 
            Com(6,59),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 62 
            Arg(0, true),
            Arg(2, true),
            Arg(3, true),
            Arg(4, true),
        ], 
        // AExp32
        vec![ // 63 
            Y,
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 64 
            Com(7,46),
            Arg(0, true),
            Arg(2, true),
            Arg(3, true),
            Arg(4, true),
            Arg(5, true),
        ], 
        // AExp33
        vec![ // 65 
            Com(6,63),
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 66 
            Arg(0, true),
            Arg(2, true),
            Arg(3, true),
            Arg(4, true),
            Arg(5, true),
        ], 
        // AExp34
        vec![ // 67 
            Arg(0, true),
            Ptr(0, true, true),
            Arg(2, true),
        ], 
        vec![ // 68 
            Arg(1, true),
            Arg(3, true),
            Arg(4, true),
            Arg(5, true),
            Arg(6, true),
        ], 
        // AExp35
        vec![ // 69 
            Y,
            Ptr(0, true, true),
            Arg(1, true),
        ], 
        vec![ // 70 
            Com(7,46),
            Arg(0, true),
            Arg(2, true),
            Arg(3, true),
            Arg(4, true),
            Arg(5, true),
        ], 
        // AExp36
        vec![ // 71 
            Arg(0, true),
            Ptr(0, true, true),
        ], 
        vec![ // 72 
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
            Arg(4, true),
            Arg(5, true),
            Arg(6, true),
        ], 
        // AExp37
        vec![ // 73 
            Com(4,20),
            Ptr(1, true, true),
            Ptr(0, true, true),
        ], 
        vec![ // 74 
            Arg(1, true),
            Arg(3, true),
        ], 
        vec![ // 75 
            Arg(0, true),
            Arg(2, true),
        ], 
        // AExp38
        vec![ // 76 
            Com(7,71),
            Com(4,73),
            Ptr(0, true, true),
        ], 
        vec![ // 77 
            Com(6,82),
            Arg(0, true),
            Arg(1, true),
        ], 
        // AExp39
        vec![ // 78 
            Com(7,67),
            Com(6,69),
            Ptr(0, true, true),
            Arg(0, true),
        ], 
        vec![ // 79 
            Com(2,76),
            Arg(1, true),
        ], 
        // AExp40
        vec![ // 80 
            Arg(0, true),
            Arg(3, true),
            Arg(1, true),
            Arg(2, true),
        ], 
        // AExp41
        vec![ // 81 
            Arg(6, true),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
            Arg(4, true),
            Arg(5, true),
        ], 
        // AExp42
        vec![ // 82 
            Com(4,80),
            Ptr(0, true, true),
        ], 
        vec![ // 83 
            Com(7,81),
            Arg(0, true),
            Arg(1, true),
            Arg(2, true),
            Arg(3, true),
            Arg(4, true),
            Arg(5, true),
        ], 
    ],

}});