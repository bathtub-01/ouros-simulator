use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 15
// Apps in this file: 36
// Combinators in this file: 68
#[rustfmt::skip]
pub static BRAUN: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0Braun.main
        vec![ // 0 
            PTR(4, false),
            PTR(3, false),
        ], 
        vec![ // 1 
            PTR(28, false),
            INT(0),
            INT(255),
        ], 
        vec![ // 2 
            PTR(25, false),
            INT(2),
            PTR(1, false),
        ], 
        vec![ // 3 
            PTR(5, false),
            PTR(9, false),
            PTR(2, false),
        ], 
         // FUN1Braun.int
        vec![ // 4 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(0),
            INT(1),
        ], 
         // FUN2NanoPrelude.all
        vec![ // 5 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(7, false),
        ], 
        vec![ // 6 
            COM(5,13,[0,1,2,4,3,0]), //X(X(XX))X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(8, false),
        ], 
        vec![ // 7 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(6, false),
        ], 
         // FUN3Data.Bool.&&
        vec![ // 8 
            COM(2,1,[1,0,0,0,0,0]), //XX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN4Braun.prop
        vec![ // 9 
            COM(4,17,[0,3,1,2,3,0]), //XX(X(XX))
            PTR(10, false),
            PTR(14, false),
            PTR(20, false),
        ], 
         // FUN5Braun.equal
        vec![ // 10 
            COM(5,40,[3,4,0,1,2,4]), //X(XXX)(XX)
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(13, false),
            PTR(12, false),
        ], 
        vec![ // 11 
            COM(5,11,[0,1,2,4,3,0]), //XX(XX)X
            COM(5,12,[0,1,4,2,3,0]), //X(XXX)X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PRM(EQ,false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 12 
            COM(6,25,[0,1,5,2,3,4]), //XX(XX)XX
            COM(5,11,[0,1,2,4,3,0]), //XX(XX)X
            COM(4,8,[0,1,2,3,0,0]), //X(X(XX))
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(11, false),
            PTR(10, false),
        ], 
        vec![ // 13 
            COM(3,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN6Braun.toList
        vec![ // 14 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            PTR(15, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 15 
            COM(6,25,[0,1,2,5,3,4]), //XX(XX)XX
            COM(5,31,[0,1,2,3,4,3]), //XX(X(XX))X
            COM(4,8,[0,1,2,3,0,0]), //X(X(XX))
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(16, false),
            PTR(14, false),
        ], 
         // FUN7Braun.ilv
        vec![ // 16 
            COM(3,6,[1,2,0,2,0,0]), //XX(XX)
            PTR(19, false),
        ], 
        vec![ // 17 
            COM(4,11,[0,1,2,3,2,0]), //XX(XX)X
            COM(5,11,[0,1,2,4,3,0]), //XX(XX)X
            COM(4,8,[0,1,2,3,0,0]), //X(X(XX))
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 18 
            COM(5,39,[0,3,1,4,2,4]), //XX(XX)(XX)
            COM(5,45,[0,1,4,2,3,4]), //X(XX)(X(XX))
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(17, false),
        ], 
        vec![ // 19 
            COM(4,4,[0,2,3,1,0,0]), //XXXX
            PTR(18, false),
            PTR(16, false),
        ], 
         // FUN8Braun.fromList
        vec![ // 20 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(21, false),
        ], 
        vec![ // 21 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(22, false),
            PTR(20, false),
        ], 
         // FUN9Braun.insertTree
        vec![ // 22 
            COM(5,46,[0,4,1,4,2,3]), //XX(XXXX)
            PTR(23, false),
            PTR(24, false),
            COM(2,0,[1,0,0,0,0,0]), //X
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
        vec![ // 23 
            COM(6,53,[0,1,2,3,5,4]), //X(XX(XX)X)
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(4,6,[0,1,2,3,0,0]), //XX(XX)
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(24, false),
            PTR(22, false),
        ], 
         // FUN10Braun.Branch
        vec![ // 24 
            COM(5,18,[0,4,1,2,3,0]), //X(XXXX)
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN11NanoPrelude.replicate
        vec![ // 25 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(27, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 26 
            COM(5,12,[0,1,4,2,3,0]), //X(XXX)X
            COM(5,34,[0,1,4,2,4,3]), //X(XX(XX))X
            PRM(LE,false),
            INT(0),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 27 
            COM(5,49,[0,4,1,2,4,3]), //XX(X(XXX))
            PTR(26, false),
            PTR(25, false),
            PRM(SUB,false),
            INT(1),
        ], 
         // FUN12NanoPrelude.enumFromTo
        vec![ // 28 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(30, false),
            PTR(35, false),
        ], 
        vec![ // 29 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PRM(LE,false),
        ], 
        vec![ // 30 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(31, false),
            PTR(29, false),
        ], 
         // FUN13NanoPrelude.takeWhile
        vec![ // 31 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(34, false),
        ], 
        vec![ // 32 
            COM(5,40,[0,3,4,1,2,4]), //X(XXX)(XX)
            COM(4,8,[0,1,2,3,0,0]), //X(X(XX))
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 33 
            COM(4,4,[0,1,3,2,0,0]), //XXXX
            PTR(32, false),
        ], 
        vec![ // 34 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(33, false),
        ], 
         // FUN14NanoPrelude.enumFrom
        vec![ // 35 
            COM(5,49,[0,4,1,2,4,3]), //XX(X(XXX))
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(35, false),
            PRM(ADD,false),
            INT(1),
        ], 
    ]
});