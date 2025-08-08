use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 11
// Apps in this file: 32
// Combinators in this file: 56
#[rustfmt::skip]
pub static QUEENS: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0Queens.main
        vec![ // 0 
            PTR(1, false),
            INT(6),
        ], 
         // FUN1Queens.nsoln
        vec![ // 1 
            COM(3,7,[0,1,2,2,0,0]), //X(XXX)
            PTR(2, false),
            PTR(5, false),
        ], 
         // FUN2NanoPrelude.length
        vec![ // 2 
            Y,
            PTR(4, false),
            INT(0),
        ], 
        vec![ // 3 
            COM(6,49,[0,1,4,2,5,3]), //XX(X(XXX))
            COM(4,6,[3,2,0,1,0,0]), //XX(XX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PRM(ADD,false),
            INT(1),
        ], 
        vec![ // 4 
            COM(3,4,[0,1,2,2,0,0]), //XXXX
            PTR(3, false),
        ], 
         // FUN3Queens.gen
        vec![ // 5 
            COM(6,53,[0,1,2,3,5,4]), //X(XX(XX)X)
            Y,
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(8, false),
            PTR(7, false),
            PTR(6, false),
        ], 
        vec![ // 6 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 7 
            COM(6,27,[0,1,2,5,3,4]), //X(X(XX))XX
            COM(5,21,[0,3,1,4,2,0]), //X(X(XXX))
            PTR(9, false),
            PTR(15, false),
            PRM(SUB,false),
            INT(1),
        ], 
        vec![ // 8 
            COM(5,28,[0,4,1,2,4,3]), //XXX(XX)X
            PRM(EQ,false),
            INT(0),
        ], 
         // FUN4Data.List_Type.concatMap
        vec![ // 9 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(11, false),
        ], 
        vec![ // 10 
            COM(5,13,[0,1,2,4,3,0]), //X(X(XX))X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(12, false),
        ], 
        vec![ // 11 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(10, false),
        ], 
         // FUN5Data.List_Type.++
        vec![ // 12 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            Y,
            PTR(14, false),
        ], 
        vec![ // 13 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 14 
            COM(4,6,[3,1,0,2,0,0]), //XX(XX)
            PTR(13, false),
        ], 
         // FUN6Queens.gen1
        vec![ // 15 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(16, false),
            PTR(29, false),
        ], 
        vec![ // 16 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(9, false),
            PTR(17, false),
        ], 
         // FUN7Queens.gen2
        vec![ // 17 
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
            PTR(19, false),
            PTR(18, false),
        ], 
        vec![ // 18 
            COM(5,12,[0,1,4,3,2,0]), //X(XXX)X
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 19 
            COM(5,9,[0,4,1,3,2,0]), //XXXXX
            PTR(20, false),
            INT(1),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN8Queens.safe
        vec![ // 20 
            COM(5,20,[0,1,2,3,4,0]), //X(XX(XX))
            Y,
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(27, false),
            PTR(26, false),
        ], 
        vec![ // 21 
            COM(5,11,[0,1,2,4,3,0]), //XX(XX)X
            COM(5,21,[0,1,2,4,3,0]), //X(X(XXX))
            PTR(28, false),
            PRM(EQ,true),
            PRM(SUB,false),
        ], 
        vec![ // 22 
            COM(5,11,[0,1,2,4,3,0]), //XX(XX)X
            COM(5,21,[0,1,2,4,3,0]), //X(X(XXX))
            PTR(28, false),
            PRM(EQ,true),
            PRM(ADD,false),
        ], 
        vec![ // 23 
            COM(5,41,[0,1,2,4,3,4]), //X(X(XX))(XX)
            COM(5,22,[0,1,2,3,4,0]), //X(X(X(XX)))
            PTR(28, false),
        ], 
        vec![ // 24 
            COM(5,17,[0,1,2,3,4,0]), //XX(X(XX))
            COM(5,41,[0,1,2,4,3,4]), //X(X(XX))(XX)
            COM(4,11,[0,3,1,3,2,0]), //XX(XX)X
            PTR(23, false),
            PRM(EQ,true),
        ], 
        vec![ // 25 
            COM(4,39,[0,3,1,3,2,3]), //XX(XX)(XX)
            PTR(24, false),
            PTR(22, false),
            PTR(21, false),
        ], 
        vec![ // 26 
            COM(5,10,[0,1,4,2,3,0]), //X(XX)XX
            COM(5,49,[0,4,3,1,4,2]), //XX(X(XXX))
            PTR(25, false),
            PRM(ADD,false),
            INT(1),
        ], 
        vec![ // 27 
            COM(4,6,[0,1,2,3,0,0]), //XX(XX)
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
         // FUN9Data.Bool.&&
        vec![ // 28 
            COM(2,1,[1,0,0,0,0,0]), //XX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN10Queens.toOne
        vec![ // 29 
            COM(5,28,[0,4,1,2,4,3]), //XXX(XX)X
            PRM(EQ,false),
            INT(1),
            PTR(31, false),
            PTR(30, false),
        ], 
        vec![ // 30 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(1),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 31 
            COM(5,49,[0,4,1,2,4,3]), //XX(X(XXX))
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(29, false),
            PRM(SUB,false),
            INT(1),
        ], 
    ]
});