use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 11
// Apps in this file: 37
// Combinators in this file: 49
#[rustfmt::skip]
pub static QUEENS: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0Queens.main
        vec![ // 0 
            PTR(1),
            INT(6),
        ], 
         // FUN1Queens.nsoln
        vec![ // 1 
            COM(3,7,[0,1,2,2,0,0]), //X(XXX)
            PTR(2),
            PTR(5),
        ], 
         // FUN2NanoPrelude.length
        vec![ // 2 
            Y,
            PTR(4),
            INT(0),
        ], 
        vec![ // 3 
            COM(5,7,[2,0,3,1,0,0]), //X(XXX)
            PRM(ADD,false),
            INT(1),
        ], 
        vec![ // 4 
            COM(4,16,[3,2,0,1,2,0]), //XX(XXX)
            PTR(3),
        ], 
         // FUN3Queens.gen
        vec![ // 5 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(10),
        ], 
        vec![ // 6 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 7 
            COM(4,7,[2,0,3,1,0,0]), //X(XXX)
            PRM(SUB,false),
            INT(1),
        ], 
        vec![ // 8 
            COM(6,44,[0,1,3,2,4,5]), //X(XX)(XXX)
            PTR(11),
            PTR(17),
            PTR(7),
        ], 
        vec![ // 9 
            COM(5,28,[0,4,1,2,4,3]), //XXX(XX)X
            PRM(EQ,false),
            INT(0),
        ], 
        vec![ // 10 
            COM(5,12,[0,1,3,4,2,0]), //X(XXX)X
            PTR(9),
            PTR(8),
            PTR(6),
        ], 
         // FUN4Data.List_Type.concatMap
        vec![ // 11 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(13),
        ], 
        vec![ // 12 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            PTR(14),
        ], 
        vec![ // 13 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(12),
        ], 
         // FUN5Data.List_Type.++
        vec![ // 14 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            Y,
            PTR(16),
        ], 
        vec![ // 15 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 16 
            COM(4,6,[3,1,0,2,0,0]), //XX(XX)
            PTR(15),
        ], 
         // FUN6Queens.gen1
        vec![ // 17 
            COM(5,15,[0,1,4,2,3,0]), //X(XX)(XX)
            PTR(11),
            PTR(18),
            PTR(34),
        ], 
         // FUN7Queens.gen2
        vec![ // 18 
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
            PTR(20),
            PTR(19),
        ], 
        vec![ // 19 
            COM(6,12,[5,0,3,2,1,0]), //X(XXX)X
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 20 
            COM(5,9,[0,4,1,3,2,0]), //XXXXX
            PTR(21),
            INT(1),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN8Queens.safe
        vec![ // 21 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(32),
        ], 
        vec![ // 22 
            COM(6,58,[0,1,3,2,5,4]), //X(XX(XXX))
            PTR(33),
            PRM(EQ,true),
            PRM(SUB,false),
        ], 
        vec![ // 23 
            COM(6,24,[0,1,5,2,3,4]), //X(XX)XXX
            COM(6,54,[0,1,2,3,4,5]), //X(X(XXX)X)
        ], 
        vec![ // 24 
            COM(5,24,[0,1,4,3,2,4]), //X(XX)XXX
            PTR(23),
        ], 
        vec![ // 25 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(24),
            PTR(22),
            PRM(ADD,false),
        ], 
        vec![ // 26 
            COM(6,58,[0,1,3,2,5,4]), //X(XX(XXX))
            PTR(33),
            PRM(EQ,true),
            PRM(ADD,false),
        ], 
        vec![ // 27 
            COM(5,29,[0,1,4,2,4,3]), //X(XX)(XX)X
            COM(5,46,[0,3,1,3,2,4]), //XX(XXXX)
        ], 
        vec![ // 28 
            COM(6,29,[0,1,2,3,5,4]), //X(XX)(XX)X
            PTR(27),
            PTR(26),
        ], 
        vec![ // 29 
            COM(4,11,[0,3,1,3,2,0]), //XX(XX)X
            PTR(28),
            PTR(25),
            INT(1),
        ], 
        vec![ // 30 
            COM(6,40,[0,1,2,4,3,5]), //X(XXX)(XX)
            PTR(33),
            PRM(EQ,true),
        ], 
        vec![ // 31 
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            COM(5,32,[0,1,2,3,4,4]), //X(XXXX)X
            PTR(30),
            PTR(29),
        ], 
        vec![ // 32 
            COM(6,46,[5,0,1,2,3,4]), //XX(XXXX)
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(31),
        ], 
         // FUN9Data.Bool.&&
        vec![ // 33 
            COM(2,1,[1,0,0,0,0,0]), //XX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN10Queens.toOne
        vec![ // 34 
            COM(5,28,[0,4,1,2,4,3]), //XXX(XX)X
            PRM(EQ,false),
            INT(1),
            PTR(36),
            PTR(35),
        ], 
        vec![ // 35 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(1),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 36 
            COM(5,49,[0,4,1,2,4,3]), //XX(X(XXX))
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(34),
            PRM(SUB,false),
            INT(1),
        ], 
    ]
});