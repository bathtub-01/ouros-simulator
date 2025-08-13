use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 11
// Apps in this file: 40
// Combinators in this file: 59
#[rustfmt::skip]
pub static PERMSORT: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0Permsort.main
        vec![ // 0 
            PTR(9, false),
            PTR(8, false),
        ], 
        vec![ // 1 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(12),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 2 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(12),
            PTR(1, false),
        ], 
        vec![ // 3 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(6),
            PTR(2, false),
        ], 
        vec![ // 4 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(9),
            PTR(3, false),
        ], 
        vec![ // 5 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(7),
            PTR(4, false),
        ], 
        vec![ // 6 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(6),
            PTR(5, false),
        ], 
        vec![ // 7 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(10),
            PTR(6, false),
        ], 
        vec![ // 8 
            PTR(10, false),
            PTR(7, false),
        ], 
         // FUN1NanoPrelude.head
        vec![ // 9 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            ERR(3),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN2Permsort.permSort
        vec![ // 10 
            COM(5,20,[0,1,2,3,4,0]), //X(XX(XX))
            PTR(9, false),
            PTR(11, false),
            PTR(15, false),
            PTR(21, false),
        ], 
         // FUN3NanoPrelude.filter
        vec![ // 11 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(14, false),
        ], 
        vec![ // 12 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 13 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(4,30,[0,2,1,2,3,3]), //XX(XXX)X
            COM(5,39,[0,3,1,4,2,4]), //XX(XX)(XX)
            PTR(12, false),
        ], 
        vec![ // 14 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(13, false),
        ], 
         // FUN4Permsort.ord
        vec![ // 15 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(19, false),
        ], 
        vec![ // 16 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            PTR(15, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 17 
            COM(6,40,[0,1,2,4,3,5]), //X(XXX)(XX)
            PTR(20, false),
            PRM(LE,false),
        ], 
        vec![ // 18 
            COM(4,11,[0,2,1,3,3,0]), //XX(XX)X
            PTR(17, false),
            PTR(16, false),
        ], 
        vec![ // 19 
            COM(4,6,[3,0,1,2,0,0]), //XX(XX)
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(18, false),
        ], 
         // FUN5Data.Bool.&&
        vec![ // 20 
            COM(2,1,[1,0,0,0,0,0]), //XX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN6Permsort.perm
        vec![ // 21 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            PTR(23, false),
            PTR(22, false),
        ], 
        vec![ // 22 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            PTR(24, false),
            PTR(30, false),
            PTR(21, false),
        ], 
        vec![ // 23 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN7Data.List_Type.concatMap
        vec![ // 24 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(26, false),
        ], 
        vec![ // 25 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            PTR(27, false),
        ], 
        vec![ // 26 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(25, false),
        ], 
         // FUN8Data.List_Type.++
        vec![ // 27 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            Y,
            PTR(29, false),
        ], 
        vec![ // 28 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 29 
            COM(4,6,[3,1,0,2,0,0]), //XX(XX)
            PTR(28, false),
        ], 
         // FUN9Permsort.place
        vec![ // 30 
            COM(5,53,[0,1,4,2,4,3]), //X(XX(XX)X)
            Y,
            PTR(36, false),
            PTR(33, false),
            PTR(31, false),
        ], 
        vec![ // 31 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            PTR(37, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 32 
            COM(4,58,[0,0,1,0,2,3]), //X(XX(XXX))
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 33 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
            PTR(32, false),
        ], 
        vec![ // 34 
            COM(4,2,[3,1,0,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 35 
            COM(5,5,[4,0,2,1,0,0]), //X(XX)X
            PTR(34, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 36 
            COM(6,45,[5,0,1,2,3,4]), //X(XX)(X(XX))
            PTR(35, false),
        ], 
         // FUN10NanoPrelude.map
        vec![ // 37 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(39, false),
        ], 
        vec![ // 38 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 39 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(38, false),
        ], 
    ]
});