use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 11
// Apps in this file: 38
// Combinators in this file: 62
#[rustfmt::skip]
pub static ORDLIST: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0Ordlist.main
        vec![ // 0 
            PTR(5, false),
            PTR(4, false),
            INT(0),
            INT(1),
        ], 
        vec![ // 1 
            COM(3,1,[1,0,0,0,0,0]), //XX
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
        vec![ // 2 
            COM(3,1,[1,0,0,0,0,0]), //XX
            PTR(1, false),
        ], 
        vec![ // 3 
            COM(3,1,[1,0,0,0,0,0]), //XX
            PTR(2, false),
        ], 
        vec![ // 4 
            COM(3,1,[1,0,0,0,0,0]), //XX
            PTR(3, false),
        ], 
         // FUN1Ordlist.top
        vec![ // 5 
            COM(5,48,[0,1,2,4,3,4]), //XX(XX(XX))
            COM(1,0,[0,0,0,0,0,0]), //X
            PTR(8, false),
            PTR(7, false),
            PTR(6, false),
        ], 
        vec![ // 6 
            COM(5,15,[0,1,2,3,4,0]), //X(XX)(XX)
            PTR(13, false),
            PTR(16, false),
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(32, false),
        ], 
        vec![ // 7 
            COM(6,57,[0,1,2,3,4,5]), //X(X(XX)(XX))
            PTR(10, false),
            PTR(13, false),
            PTR(16, false),
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(32, false),
        ], 
         // FUN2NanoPrelude.and
        vec![ // 8 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(9, false),
        ], 
        vec![ // 9 
            COM(4,6,[2,0,1,3,0,0]), //XX(XX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(8, false),
        ], 
         // FUN3Data.List_Type.++
        vec![ // 10 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            Y,
            PTR(12, false),
        ], 
        vec![ // 11 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 12 
            COM(4,6,[3,1,0,2,0,0]), //XX(XX)
            PTR(11, false),
        ], 
         // FUN4NanoPrelude.map
        vec![ // 13 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(15, false),
        ], 
        vec![ // 14 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 15 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(14, false),
        ], 
         // FUN5Ordlist.prop
        vec![ // 16 
            COM(5,44,[0,1,4,2,3,4]), //X(XX)(XXX)
            PTR(18, false),
            PTR(19, false),
            PTR(17, false),
        ], 
        vec![ // 17 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            PTR(19, false),
            PTR(25, false),
        ], 
         // FUN6Ordlist.implies
        vec![ // 18 
            COM(2,1,[1,0,0,0,0,0]), //XX
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
         // FUN7Ordlist.ord
        vec![ // 19 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(23, false),
        ], 
        vec![ // 20 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            PTR(19, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 21 
            COM(6,40,[0,1,2,4,3,5]), //X(XXX)(XX)
            PTR(24, false),
            PTR(18, false),
        ], 
        vec![ // 22 
            COM(4,11,[0,2,1,3,3,0]), //XX(XX)X
            PTR(21, false),
            PTR(20, false),
        ], 
        vec![ // 23 
            COM(4,6,[3,0,1,2,0,0]), //XX(XX)
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(22, false),
        ], 
         // FUN8Data.Bool.&&
        vec![ // 24 
            COM(2,1,[1,0,0,0,0,0]), //XX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN9Ordlist.ins
        vec![ // 25 
            COM(4,20,[0,1,3,2,3,0]), //X(XX(XX))
            Y,
            PTR(31, false),
            PTR(26, false),
        ], 
        vec![ // 26 
            COM(4,16,[0,1,0,2,3,0]), //XX(XXX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 27 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 28 
            COM(5,42,[0,1,3,2,3,4]), //XXX(XXX)
            PTR(18, false),
        ], 
        vec![ // 29 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            PTR(28, false),
            PTR(27, false),
        ], 
        vec![ // 30 
            COM(4,2,[3,1,0,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 31 
            COM(5,38,[0,1,4,2,3,4]), //X(XX)X(XX)
            COM(6,47,[5,0,1,2,4,3]), //XX(X(XX)X)
            PTR(30, false),
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
            PTR(29, false),
        ], 
         // FUN10Ordlist.boolList
        vec![ // 32 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            PTR(37, false),
            PTR(33, false),
        ], 
        vec![ // 33 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 34 
            COM(5,15,[0,1,2,3,4,0]), //X(XX)(XX)
            PTR(13, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(32, false),
        ], 
        vec![ // 35 
            COM(6,57,[0,1,2,3,4,5]), //X(X(XX)(XX))
            PTR(10, false),
            PTR(13, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(32, false),
        ], 
        vec![ // 36 
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(35, false),
            PTR(34, false),
        ], 
        vec![ // 37 
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            PTR(10, false),
            PTR(32, false),
            PTR(36, false),
        ], 
    ]
});