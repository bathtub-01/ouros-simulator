use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 12
// Apps in this file: 66
// Combinators in this file: 115
#[rustfmt::skip]
pub static TRIBELIE: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0TribeLie.main
        vec![ // 0 
            PTR(1, false),
            PTR(9, false),
            PTR(21, false),
        ], 
         // FUN1TribeLie.count
        vec![ // 1 
            COM(6,47,[0,1,2,3,4,5]), //XX(X(XX)X)
            COM(1,0,[0,0,0,0,0,0]), //X
            PTR(3, false),
            PTR(6, false),
            PTR(2, false),
        ], 
        vec![ // 2 
            COM(4,4,[2,3,0,1,0,0]), //XXXX
            INT(0),
            INT(1),
        ], 
         // FUN2NanoPrelude.sum
        vec![ // 3 
            PTR(4, false),
            PRM(ADD,false),
            INT(0),
        ], 
         // FUN3NanoPrelude.foldr
        vec![ // 4 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            Y,
            PTR(5, false),
        ], 
        vec![ // 5 
            COM(5,16,[4,2,0,1,3,0]), //XX(XXX)
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
        ], 
         // FUN4NanoPrelude.map
        vec![ // 6 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(8, false),
        ], 
        vec![ // 7 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 8 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(7, false),
        ], 
         // FUN5TribeLie.predicate
        vec![ // 9 
            COM(2,1,[1,0,0,0,0,0]), //XX
            PTR(15, false),
        ], 
        vec![ // 10 
            COM(7,23,[0,1,2,3,4,5]), //XXXXXX
            COM(7,12,[0,1,2,3,4,0]), //X(XXX)X
            PTR(16, false),
            PTR(17, false),
        ], 
        vec![ // 11 
            COM(6,24,[0,1,5,2,3,4]), //X(XX)XXX
            COM(7,54,[0,1,2,3,5,4]), //X(X(XXX)X)
        ], 
        vec![ // 12 
            COM(6,24,[0,1,5,2,3,4]), //X(XX)XXX
            PTR(11, false),
            PTR(10, false),
            PTR(16, false),
            PTR(18, false),
        ], 
        vec![ // 13 
            COM(5,24,[0,1,4,2,3,4]), //X(XX)XXX
            COM(5,46,[0,4,1,2,3,4]), //XX(XXXX)
        ], 
        vec![ // 14 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(13, false),
            PTR(12, false),
            PTR(19, false),
        ], 
        vec![ // 15 
            COM(5,23,[0,1,2,3,4,4]), //XXXXXX
            PTR(14, false),
        ], 
         // FUN6Data.Bool.&&
        vec![ // 16 
            COM(2,1,[1,0,0,0,0,0]), //XX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN7TribeLie.childs_statement_is_valid
        vec![ // 17 
            COM(4,12,[2,3,0,1,0,0]), //X(XXX)X
            COM(2,0,[1,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN8TribeLie.parent1_statement_is_valid
        vec![ // 18 
            COM(4,12,[2,3,0,1,0,0]), //X(XXX)X
            COM(2,0,[1,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN9TribeLie.parent2_statement_is_valid
        vec![ // 19 
            COM(5,10,[0,1,4,2,3,0]), //X(XX)XX
            COM(5,42,[0,3,4,4,1,2]), //XXX(XXX)
            PTR(20, false),
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
        vec![ // 20 
            COM(5,58,[2,3,0,4,1,0]), //X(XX(XXX))
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
         // FUN10TribeLie.allCombis
        vec![ // 21 
            Y,
            PTR(64, false),
            PTR(23, false),
        ], 
        vec![ // 22 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 23 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(22, false),
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
            COM(6,51,[0,1,2,3,4,5]), //X(XXXXX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 39 
            COM(6,51,[0,1,2,3,4,5]), //X(XXXXX)
            PTR(38, false),
            PTR(65, false),
        ], 
        vec![ // 40 
            COM(6,54,[0,1,2,3,4,5]), //X(X(XXX)X)
            COM(6,44,[5,1,3,0,2,4]), //X(XX)(XXX)
            COM(6,37,[0,1,2,4,3,5]), //XXXX(XX)
        ], 
        vec![ // 41 
            COM(5,18,[0,1,2,3,4,0]), //X(XXXX)
            PTR(40, false),
            PTR(39, false),
        ], 
        vec![ // 42 
            COM(6,32,[0,1,3,4,5,2]), //X(XXXX)X
            Y,
        ], 
        vec![ // 43 
            COM(6,32,[0,1,3,4,5,2]), //X(XXXX)X
            PTR(42, false),
        ], 
        vec![ // 44 
            COM(6,32,[0,1,3,4,5,2]), //X(XXXX)X
            PTR(43, false),
            PTR(41, false),
            PTR(37, false),
        ], 
        vec![ // 45 
            COM(6,51,[0,1,2,3,4,5]), //X(XXXXX)
            COM(6,44,[5,1,3,0,2,4]), //X(XX)(XXX)
        ], 
        vec![ // 46 
            COM(6,32,[0,1,3,4,5,2]), //X(XXXX)X
            Y,
        ], 
        vec![ // 47 
            COM(6,32,[0,1,3,4,5,2]), //X(XXXX)X
            PTR(46, false),
        ], 
        vec![ // 48 
            COM(6,33,[0,1,2,4,5,3]), //X(X(XX)X)X
            PTR(47, false),
            PTR(45, false),
            PTR(44, false),
            PTR(35, false),
        ], 
        vec![ // 49 
            COM(6,51,[0,1,2,3,4,5]), //X(XXXXX)
            COM(6,44,[5,1,3,0,2,4]), //X(XX)(XXX)
            PTR(48, false),
        ], 
        vec![ // 50 
            COM(6,32,[0,1,3,4,5,2]), //X(XXXX)X
            Y,
        ], 
        vec![ // 51 
            COM(6,32,[0,1,3,4,5,2]), //X(XXXX)X
            PTR(50, false),
        ], 
        vec![ // 52 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(51, false),
            PTR(49, false),
            PTR(33, false),
        ], 
        vec![ // 53 
            COM(5,18,[0,1,2,3,4,0]), //X(XXXX)
            COM(6,44,[5,1,3,0,2,4]), //X(XX)(XXX)
            PTR(52, false),
        ], 
        vec![ // 54 
            COM(6,32,[0,1,3,4,5,2]), //X(XXXX)X
            Y,
        ], 
        vec![ // 55 
            COM(6,32,[0,1,3,4,5,2]), //X(XXXX)X
            PTR(54, false),
            PTR(53, false),
            PTR(31, false),
        ], 
        vec![ // 56 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            COM(6,44,[5,1,3,0,2,4]), //X(XX)(XXX)
            PTR(55, false),
        ], 
        vec![ // 57 
            COM(6,32,[0,1,3,4,5,2]), //X(XXXX)X
            Y,
        ], 
        vec![ // 58 
            COM(5,12,[0,1,3,4,2,0]), //X(XXX)X
            PTR(57, false),
            PTR(56, false),
            PTR(29, false),
        ], 
        vec![ // 59 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            COM(6,44,[5,1,3,0,2,4]), //X(XX)(XXX)
            PTR(58, false),
        ], 
        vec![ // 60 
            COM(6,32,[0,1,3,4,5,2]), //X(XXXX)X
            Y,
        ], 
        vec![ // 61 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(60, false),
            PTR(59, false),
            PTR(27, false),
        ], 
        vec![ // 62 
            COM(6,44,[5,1,3,0,2,4]), //X(XX)(XXX)
            PTR(61, false),
        ], 
        vec![ // 63 
            COM(6,32,[0,1,3,4,5,2]), //X(XXXX)X
            Y,
            PTR(62, false),
            PTR(25, false),
        ], 
        vec![ // 64 
            COM(4,6,[3,0,1,2,0,0]), //XX(XX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(63, false),
        ], 
         // FUN11TribeLie.PuzzleAnswer
        vec![ // 65 
            COM(6,51,[0,1,2,3,4,5]), //X(XXXXX)
            COM(6,23,[0,1,5,2,3,4]), //XXXXXX
            COM(6,23,[5,0,1,2,3,4]), //XXXXXX
        ], 
    ]
});