use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 10
// Apps in this file: 68
// Combinators in this file: 115
#[rustfmt::skip]
pub static TRIBELIE: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0TribeLie.main
        vec![ // 0 
            PTR(2, false),
            PTR(1, false),
        ], 
        vec![ // 1 
            PTR(5, false),
            PTR(11, false),
            PTR(23, false),
        ], 
         // FUN1NanoPrelude.length
        vec![ // 2 
            Y,
            PTR(4, false),
            INT(0),
        ], 
        vec![ // 3 
            COM(5,7,[2,0,3,1,0,0]), //X(XXX)
            PRM(ADD,false),
            INT(1),
        ], 
        vec![ // 4 
            COM(4,16,[3,2,0,1,2,0]), //XX(XXX)
            PTR(3, false),
        ], 
         // FUN2TribeLie.filter'
        vec![ // 5 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(10, false),
        ], 
        vec![ // 6 
            COM(4,42,[1,2,3,0,2,3]), //XXX(XXX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 7 
            COM(5,33,[0,1,2,3,4,4]), //X(X(XX)X)X
            TRY,
        ], 
        vec![ // 8 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(7, false),
            PTR(6, false),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 9 
            COM(5,14,[0,1,3,2,4,0]), //XXX(XX)
            PTR(8, false),
        ], 
        vec![ // 10 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(9, false),
        ], 
         // FUN3TribeLie.predicate
        vec![ // 11 
            COM(2,1,[1,0,0,0,0,0]), //XX
            PTR(17, false),
        ], 
        vec![ // 12 
            COM(7,23,[0,1,2,3,4,5]), //XXXXXX
            COM(7,12,[0,1,2,3,4,0]), //X(XXX)X
            PTR(18, false),
            PTR(19, false),
        ], 
        vec![ // 13 
            COM(6,24,[0,1,5,2,3,4]), //X(XX)XXX
            COM(7,54,[0,1,2,3,5,4]), //X(X(XXX)X)
        ], 
        vec![ // 14 
            COM(6,24,[0,1,5,2,3,4]), //X(XX)XXX
            PTR(13, false),
            PTR(12, false),
            PTR(18, false),
            PTR(20, false),
        ], 
        vec![ // 15 
            COM(5,24,[0,1,4,2,3,4]), //X(XX)XXX
            COM(5,46,[0,4,1,2,3,4]), //XX(XXXX)
        ], 
        vec![ // 16 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(15, false),
            PTR(14, false),
            PTR(21, false),
        ], 
        vec![ // 17 
            COM(5,23,[0,1,2,3,4,4]), //XXXXXX
            PTR(16, false),
        ], 
         // FUN4Data.Bool.&&
        vec![ // 18 
            COM(2,1,[1,0,0,0,0,0]), //XX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN5TribeLie.childs_statement_is_valid
        vec![ // 19 
            COM(4,12,[2,3,0,1,0,0]), //X(XXX)X
            COM(2,0,[1,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN6TribeLie.parent1_statement_is_valid
        vec![ // 20 
            COM(4,12,[2,3,0,1,0,0]), //X(XXX)X
            COM(2,0,[1,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN7TribeLie.parent2_statement_is_valid
        vec![ // 21 
            COM(5,10,[0,1,4,2,3,0]), //X(XX)XX
            COM(5,42,[0,3,4,4,1,2]), //XXX(XXX)
            PTR(22, false),
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
        vec![ // 22 
            COM(5,58,[2,3,0,4,1,0]), //X(XX(XXX))
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
         // FUN8TribeLie.allCombis
        vec![ // 23 
            Y,
            PTR(66, false),
            PTR(25, false),
        ], 
        vec![ // 24 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 25 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(24, false),
        ], 
        vec![ // 26 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 27 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(26, false),
        ], 
        vec![ // 28 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 29 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(28, false),
        ], 
        vec![ // 30 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 31 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(30, false),
        ], 
        vec![ // 32 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 33 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(32, false),
        ], 
        vec![ // 34 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 35 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(34, false),
        ], 
        vec![ // 36 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 37 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(36, false),
        ], 
        vec![ // 38 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 39 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(38, false),
        ], 
        vec![ // 40 
            COM(6,51,[0,1,2,3,4,5]), //X(XXXXX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 41 
            COM(6,51,[0,1,2,3,4,5]), //X(XXXXX)
            PTR(40, false),
            PTR(67, false),
        ], 
        vec![ // 42 
            COM(6,54,[0,1,2,3,4,5]), //X(X(XXX)X)
            COM(6,44,[5,1,3,0,2,4]), //X(XX)(XXX)
            COM(6,37,[0,1,2,4,3,5]), //XXXX(XX)
        ], 
        vec![ // 43 
            COM(5,18,[0,1,2,3,4,0]), //X(XXXX)
            PTR(42, false),
            PTR(41, false),
        ], 
        vec![ // 44 
            COM(6,32,[0,1,3,4,5,2]), //X(XXXX)X
            Y,
        ], 
        vec![ // 45 
            COM(6,32,[0,1,3,4,5,2]), //X(XXXX)X
            PTR(44, false),
        ], 
        vec![ // 46 
            COM(6,32,[0,1,3,4,5,2]), //X(XXXX)X
            PTR(45, false),
            PTR(43, false),
            PTR(39, false),
        ], 
        vec![ // 47 
            COM(6,51,[0,1,2,3,4,5]), //X(XXXXX)
            COM(6,44,[5,1,3,0,2,4]), //X(XX)(XXX)
        ], 
        vec![ // 48 
            COM(6,32,[0,1,3,4,5,2]), //X(XXXX)X
            Y,
        ], 
        vec![ // 49 
            COM(6,32,[0,1,3,4,5,2]), //X(XXXX)X
            PTR(48, false),
        ], 
        vec![ // 50 
            COM(6,33,[0,1,2,4,5,3]), //X(X(XX)X)X
            PTR(49, false),
            PTR(47, false),
            PTR(46, false),
            PTR(37, false),
        ], 
        vec![ // 51 
            COM(6,51,[0,1,2,3,4,5]), //X(XXXXX)
            COM(6,44,[5,1,3,0,2,4]), //X(XX)(XXX)
            PTR(50, false),
        ], 
        vec![ // 52 
            COM(6,32,[0,1,3,4,5,2]), //X(XXXX)X
            Y,
        ], 
        vec![ // 53 
            COM(6,32,[0,1,3,4,5,2]), //X(XXXX)X
            PTR(52, false),
        ], 
        vec![ // 54 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(53, false),
            PTR(51, false),
            PTR(35, false),
        ], 
        vec![ // 55 
            COM(5,18,[0,1,2,3,4,0]), //X(XXXX)
            COM(6,44,[5,1,3,0,2,4]), //X(XX)(XXX)
            PTR(54, false),
        ], 
        vec![ // 56 
            COM(6,32,[0,1,3,4,5,2]), //X(XXXX)X
            Y,
        ], 
        vec![ // 57 
            COM(6,32,[0,1,3,4,5,2]), //X(XXXX)X
            PTR(56, false),
            PTR(55, false),
            PTR(33, false),
        ], 
        vec![ // 58 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            COM(6,44,[5,1,3,0,2,4]), //X(XX)(XXX)
            PTR(57, false),
        ], 
        vec![ // 59 
            COM(6,32,[0,1,3,4,5,2]), //X(XXXX)X
            Y,
        ], 
        vec![ // 60 
            COM(5,12,[0,1,3,4,2,0]), //X(XXX)X
            PTR(59, false),
            PTR(58, false),
            PTR(31, false),
        ], 
        vec![ // 61 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            COM(6,44,[5,1,3,0,2,4]), //X(XX)(XXX)
            PTR(60, false),
        ], 
        vec![ // 62 
            COM(6,32,[0,1,3,4,5,2]), //X(XXXX)X
            Y,
        ], 
        vec![ // 63 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(62, false),
            PTR(61, false),
            PTR(29, false),
        ], 
        vec![ // 64 
            COM(6,44,[5,1,3,0,2,4]), //X(XX)(XXX)
            PTR(63, false),
        ], 
        vec![ // 65 
            COM(6,32,[0,1,3,4,5,2]), //X(XXXX)X
            Y,
            PTR(64, false),
            PTR(27, false),
        ], 
        vec![ // 66 
            COM(4,6,[3,0,1,2,0,0]), //XX(XX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(65, false),
        ], 
         // FUN9TribeLie.PuzzleAnswer
        vec![ // 67 
            COM(6,51,[0,1,2,3,4,5]), //X(XXXXX)
            COM(6,23,[0,1,5,2,3,4]), //XXXXXX
            COM(6,23,[5,0,1,2,3,4]), //XXXXXX
        ], 
    ]
});