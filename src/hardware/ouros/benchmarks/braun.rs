use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 14
// Apps in this file: 45
// Combinators in this file: 67
#[rustfmt::skip]
pub static BRAUN: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0Braun.main
        vec![ // 0 
            PTR(4, false),
            PTR(3, false),
        ], 
        vec![ // 1 
            PTR(38, false),
            INT(0),
            INT(255),
        ], 
        vec![ // 2 
            PTR(34, false),
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
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
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
            PTR(22, false),
        ], 
         // FUN5Braun.equal
        vec![ // 10 
            COM(5,40,[3,4,0,1,2,4]), //X(XXX)(XX)
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(13, false),
            PTR(12, false),
        ], 
        vec![ // 11 
            COM(6,37,[0,2,4,1,3,5]), //XXXX(XX)
            PRM(EQ,false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 12 
            COM(6,48,[3,0,1,4,2,5]), //XX(XX(XX))
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
            PTR(16, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 15 
            COM(4,15,[0,1,2,1,3,0]), //X(XX)(XX)
            PTR(17, false),
            PTR(14, false),
        ], 
        vec![ // 16 
            COM(5,16,[0,2,1,3,4,0]), //XX(XXX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(15, false),
        ], 
         // FUN7Braun.ilv
        vec![ // 17 
            COM(3,6,[1,2,0,2,0,0]), //XX(XX)
            PTR(21, false),
        ], 
        vec![ // 18 
            COM(5,16,[0,3,1,2,4,0]), //XX(XXX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(17, false),
        ], 
        vec![ // 19 
            COM(6,46,[0,2,1,3,4,5]), //XX(XXXX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(18, false),
        ], 
        vec![ // 20 
            COM(5,40,[1,0,3,4,2,4]), //X(XXX)(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 21 
            COM(4,11,[0,2,1,3,3,0]), //XX(XX)X
            PTR(20, false),
            PTR(19, false),
        ], 
         // FUN8Braun.fromList'
        vec![ // 22 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(27, false),
        ], 
        vec![ // 23 
            COM(3,3,[0,2,1,0,0,0]), //X(XX)
            PTR(22, false),
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
        vec![ // 24 
            COM(4,40,[0,0,2,3,1,3]), //X(XXX)(XX)
            SEQ(false),
        ], 
        vec![ // 25 
            COM(5,26,[0,1,2,4,4,3]), //X(XXX)XX
            PTR(24, false),
            COM(5,4,[3,0,1,2,0,0]), //XXXX
        ], 
        vec![ // 26 
            COM(6,24,[0,1,5,2,3,4]), //X(XX)XXX
            COM(5,45,[0,1,4,2,4,3]), //X(XX)(X(XX))
            PTR(25, false),
            PTR(23, false),
            PTR(22, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 27 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            PTR(26, false),
            PTR(28, false),
        ], 
         // FUN9Braun.unravel
        vec![ // 28 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            PTR(33, false),
            PTR(32, false),
        ], 
        vec![ // 29 
            COM(2,1,[1,0,0,0,0,0]), //XX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 30 
            COM(6,34,[5,0,2,4,1,3]), //X(XX(XX))X
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
        vec![ // 31 
            COM(4,11,[0,2,1,3,3,0]), //XX(XX)X
            PTR(30, false),
            PTR(29, false),
        ], 
        vec![ // 32 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            PTR(31, false),
            PTR(28, false),
        ], 
        vec![ // 33 
            COM(2,2,[1,0,0,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN10NanoPrelude.replicate
        vec![ // 34 
            COM(4,10,[0,1,3,2,3,0]), //X(XX)XX
            PTR(37, false),
            PTR(36, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 35 
            COM(4,7,[0,1,3,2,0,0]), //X(XXX)
            PTR(34, false),
            PRM(SUB,false),
            INT(1),
        ], 
        vec![ // 36 
            COM(4,16,[0,3,1,2,3,0]), //XX(XXX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(35, false),
        ], 
        vec![ // 37 
            COM(6,28,[0,4,1,2,5,3]), //XXX(XX)X
            PRM(LE,false),
            INT(0),
        ], 
         // FUN11NanoPrelude.enumFromTo
        vec![ // 38 
            COM(5,15,[0,1,4,2,3,0]), //X(XX)(XX)
            PTR(40, false),
            PTR(39, false),
            PTR(44, false),
        ], 
        vec![ // 39 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PRM(LE,false),
        ], 
         // FUN12NanoPrelude.takeWhile
        vec![ // 40 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(43, false),
        ], 
        vec![ // 41 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 42 
            COM(5,42,[1,3,0,2,3,4]), //XXX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 43 
            COM(6,48,[5,0,1,3,2,4]), //XX(XX(XX))
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(42, false),
            PTR(41, false),
        ], 
         // FUN13NanoPrelude.enumFrom
        vec![ // 44 
            COM(5,49,[0,4,1,2,4,3]), //XX(X(XXX))
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(44, false),
            PRM(ADD,false),
            INT(1),
        ], 
    ]
});