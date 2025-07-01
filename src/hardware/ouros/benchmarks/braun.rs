use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*;

// Functions in this file: 14
// Apps in this file: 37
// Combinators in this file: 55
#[rustfmt::skip]
pub static BRAUN: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0Braun.main
        vec![ // 0 
            PTR(4),
            PTR(3),
        ], 
        vec![ // 1 
            PTR(30),
            INT(0),
            INT(255),
        ], 
        vec![ // 2 
            PTR(26),
            INT(2),
            PTR(1),
        ], 
        vec![ // 3 
            PTR(5),
            PTR(9),
            PTR(2),
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
            PTR(7),
        ], 
        vec![ // 6 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            PTR(8),
        ], 
        vec![ // 7 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(6),
        ], 
         // FUN3Data.Bool.&&
        vec![ // 8 
            COM(2,1,[1,0,0,0,0,0]), //XX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN4Braun.prop
        vec![ // 9 
            COM(4,17,[0,3,1,2,3,0]), //XX(X(XX))
            PTR(10),
            PTR(14),
            PTR(22),
        ], 
         // FUN5Braun.equal
        vec![ // 10 
            COM(5,40,[3,4,0,1,2,4]), //X(XXX)(XX)
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(13),
            PTR(12),
        ], 
        vec![ // 11 
            COM(6,37,[0,2,4,1,3,5]), //XXXX(XX)
            PRM(EQ,false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 12 
            COM(6,48,[3,0,1,4,2,5]), //XX(XX(XX))
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(11),
            PTR(10),
        ], 
        vec![ // 13 
            COM(3,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN6Braun.toList
        vec![ // 14 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            PTR(16),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 15 
            COM(4,15,[0,1,2,1,3,0]), //X(XX)(XX)
            PTR(17),
            PTR(14),
        ], 
        vec![ // 16 
            COM(5,16,[0,2,1,3,4,0]), //XX(XXX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(15),
        ], 
         // FUN7Braun.ilv
        vec![ // 17 
            COM(3,6,[1,2,0,2,0,0]), //XX(XX)
            PTR(21),
        ], 
        vec![ // 18 
            COM(5,16,[0,3,1,2,4,0]), //XX(XXX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(17),
        ], 
        vec![ // 19 
            COM(6,46,[0,2,1,3,4,5]), //XX(XXXX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(18),
        ], 
        vec![ // 20 
            COM(5,40,[1,0,3,4,2,4]), //X(XXX)(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 21 
            COM(4,11,[0,2,1,3,3,0]), //XX(XX)X
            PTR(20),
            PTR(19),
        ], 
         // FUN8Braun.fromList
        vec![ // 22 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(23),
        ], 
        vec![ // 23 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            PTR(24),
            PTR(22),
        ], 
         // FUN9Braun.insertTree
        vec![ // 24 
            COM(5,24,[0,1,4,2,3,4]), //X(XX)XXX
            COM(5,46,[4,0,1,3,2,2]), //XX(XXXX)
            PTR(25),
            COM(5,4,[3,0,1,2,0,0]), //XXXX
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
        vec![ // 25 
            COM(6,30,[0,2,1,3,5,4]), //XX(XXX)X
            COM(5,4,[3,0,1,2,0,0]), //XXXX
            PTR(24),
        ], 
         // FUN10NanoPrelude.replicate
        vec![ // 26 
            COM(4,10,[0,1,3,2,3,0]), //X(XX)XX
            PTR(29),
            PTR(28),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 27 
            COM(4,7,[0,1,3,2,0,0]), //X(XXX)
            PTR(26),
            PRM(SUB,false),
            INT(1),
        ], 
        vec![ // 28 
            COM(4,16,[0,3,1,2,3,0]), //XX(XXX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(27),
        ], 
        vec![ // 29 
            COM(6,28,[0,4,1,2,5,3]), //XXX(XX)X
            PRM(LE,false),
            INT(0),
        ], 
         // FUN11NanoPrelude.enumFromTo
        vec![ // 30 
            COM(5,15,[0,1,4,2,3,0]), //X(XX)(XX)
            PTR(32),
            PTR(31),
            PTR(36),
        ], 
        vec![ // 31 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PRM(LE,false),
        ], 
         // FUN12NanoPrelude.takeWhile
        vec![ // 32 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(35),
        ], 
        vec![ // 33 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 34 
            COM(5,42,[1,3,0,2,3,4]), //XXX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 35 
            COM(6,48,[5,0,1,3,2,4]), //XX(XX(XX))
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(34),
            PTR(33),
        ], 
         // FUN13NanoPrelude.enumFrom
        vec![ // 36 
            COM(5,49,[0,4,1,2,4,3]), //XX(X(XXX))
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(36),
            PRM(ADD,false),
            INT(1),
        ], 
    ]
});
