use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*;

// Functions in this file: 15
// Apps in this file: 35
// Combinators in this file: 49
#[rustfmt::skip]
pub static MSS: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0Mss.main
        vec![ // 0 
            PTR(3),
            PTR(2),
        ], 
        vec![ // 1 
            PRM(SUB,false),
            INT(0),
            INT(20),
        ], 
        vec![ // 2 
            PTR(28),
            PTR(1),
            INT(20),
        ], 
         // FUN1Mss.mss
        vec![ // 3 
            COM(5,20,[0,1,2,3,4,0]), //X(XX(XX))
            PTR(4),
            PTR(9),
            PTR(12),
            PTR(13),
        ], 
         // FUN2NanoPrelude.maximum
        vec![ // 4 
            COM(4,6,[3,0,1,2,0,0]), //XX(XX)
            INT(0),
            PTR(7),
            PTR(6),
        ], 
        vec![ // 5 
            COM(3,9,[0,1,2,2,1,0]), //XXXXX
            PRM(LE,true),
        ], 
        vec![ // 6 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(5),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN3NanoPrelude.foldr
        vec![ // 7 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            Y,
            PTR(8),
        ], 
        vec![ // 8 
            COM(5,16,[4,2,0,1,3,0]), //XX(XXX)
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
        ], 
         // FUN4NanoPrelude.map
        vec![ // 9 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(11),
        ], 
        vec![ // 10 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 11 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(10),
        ], 
         // FUN5NanoPrelude.sum
        vec![ // 12 
            PTR(7),
            PRM(ADD,false),
            INT(0),
        ], 
         // FUN6Mss.segments
        vec![ // 13 
            COM(4,6,[0,1,2,3,0,0]), //XX(XX)
            PTR(14),
            PTR(20),
            PTR(22),
        ], 
         // FUN7Data.List_Type.concatMap
        vec![ // 14 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(16),
        ], 
        vec![ // 15 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            PTR(17),
        ], 
        vec![ // 16 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(15),
        ], 
         // FUN8Data.List_Type.++
        vec![ // 17 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            Y,
            PTR(19),
        ], 
        vec![ // 18 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 19 
            COM(4,6,[3,1,0,2,0,0]), //XX(XX)
            PTR(18),
        ], 
         // FUN9Mss.tails
        vec![ // 20 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(21),
        ], 
        vec![ // 21 
            COM(4,40,[0,0,2,3,1,3]), //X(XXX)(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(20),
        ], 
         // FUN10Mss.inits
        vec![ // 22 
            COM(4,17,[3,0,1,2,3,0]), //XX(X(XX))
            PTR(24),
            PTR(23),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 23 
            COM(6,17,[0,3,1,2,3,0]), //XX(X(XX))
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(22),
            PTR(25),
        ], 
        vec![ // 24 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN11NanoPrelude.init
        vec![ // 25 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            ERR(1),
            PTR(27),
        ], 
        vec![ // 26 
            COM(6,6,[0,2,1,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(25),
        ], 
        vec![ // 27 
            COM(4,16,[3,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(26),
        ], 
         // FUN12NanoPrelude.enumFromTo
        vec![ // 28 
            COM(5,15,[0,1,4,2,3,0]), //X(XX)(XX)
            PTR(30),
            PTR(29),
            PTR(34),
        ], 
        vec![ // 29 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PRM(LE,false),
        ], 
         // FUN13NanoPrelude.takeWhile
        vec![ // 30 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(33),
        ], 
        vec![ // 31 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 32 
            COM(5,42,[1,3,0,2,3,4]), //XXX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 33 
            COM(6,48,[5,0,1,3,2,4]), //XX(XX(XX))
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(32),
            PTR(31),
        ], 
         // FUN14NanoPrelude.enumFrom
        vec![ // 34 
            COM(5,49,[0,4,1,2,4,3]), //XX(X(XXX))
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(34),
            PRM(ADD,false),
            INT(1),
        ], 
    ]
});
