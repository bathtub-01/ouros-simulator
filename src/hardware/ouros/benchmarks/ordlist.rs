use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 12
// Apps in this file: 43
// Combinators in this file: 68
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
            PTR(42, false),
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
        vec![ // 2 
            PTR(42, false),
            PTR(1, false),
        ], 
        vec![ // 3 
            PTR(42, false),
            PTR(2, false),
        ], 
        vec![ // 4 
            PTR(42, false),
            PTR(3, false),
        ], 
         // FUN1Ordlist.top
        vec![ // 5 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(12, false),
            PTR(11, false),
        ], 
        vec![ // 6 
            PTR(20, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 7 
            PTR(17, false),
            PTR(6, false),
        ], 
        vec![ // 8 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(7, false),
            PTR(33, false),
        ], 
        vec![ // 9 
            PTR(20, false),
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
        vec![ // 10 
            PTR(17, false),
            PTR(9, false),
        ], 
        vec![ // 11 
            COM(5,41,[0,1,2,4,3,4]), //X(X(XX))(XX)
            PTR(14, false),
            PTR(10, false),
            PTR(33, false),
            PTR(8, false),
        ], 
         // FUN2NanoPrelude.and
        vec![ // 12 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(13, false),
        ], 
        vec![ // 13 
            COM(4,5,[0,3,1,2,0,0]), //X(XX)X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(12, false),
        ], 
         // FUN3Data.List_Type.++
        vec![ // 14 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            Y,
            PTR(16, false),
        ], 
        vec![ // 15 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 16 
            COM(4,6,[3,1,0,2,0,0]), //XX(XX)
            PTR(15, false),
        ], 
         // FUN4NanoPrelude.map
        vec![ // 17 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(19, false),
        ], 
        vec![ // 18 
            COM(5,13,[0,1,2,4,3,0]), //X(X(XX))X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 19 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(18, false),
        ], 
         // FUN5Ordlist.prop
        vec![ // 20 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(21, false),
            PTR(28, false),
        ], 
        vec![ // 21 
            COM(4,45,[0,1,3,1,2,3]), //X(XX)(X(XX))
            PTR(22, false),
            PTR(23, false),
        ], 
         // FUN6Ordlist.implies
        vec![ // 22 
            COM(2,1,[1,0,0,0,0,0]), //XX
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
         // FUN7Ordlist.ord
        vec![ // 23 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(26, false),
        ], 
        vec![ // 24 
            COM(5,13,[0,1,2,4,3,0]), //X(X(XX))X
            COM(4,8,[0,1,2,3,0,0]), //X(X(XX))
            PTR(27, false),
        ], 
        vec![ // 25 
            COM(6,33,[0,1,2,5,3,4]), //X(X(XX)X)X
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(24, false),
            PTR(22, false),
            PTR(23, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 26 
            COM(4,6,[0,1,2,3,0,0]), //XX(XX)
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(25, false),
        ], 
         // FUN8Data.Bool.&&
        vec![ // 27 
            COM(2,1,[1,0,0,0,0,0]), //XX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN9Ordlist.ins
        vec![ // 28 
            COM(4,20,[0,1,3,2,3,0]), //X(XX(XX))
            Y,
            PTR(32, false),
            PTR(31, false),
        ], 
        vec![ // 29 
            COM(5,11,[0,1,2,4,3,0]), //XX(XX)X
            COM(5,29,[0,1,4,2,4,3]), //X(XX)(XX)X
            COM(5,41,[0,1,2,4,3,4]), //X(X(XX))(XX)
            PTR(22, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 30 
            COM(5,11,[0,1,2,4,3,0]), //XX(XX)X
            COM(6,24,[0,1,5,2,3,4]), //X(XX)XXX
            COM(5,48,[0,4,1,2,3,4]), //XX(XX(XX))
            PTR(29, false),
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
        ], 
        vec![ // 31 
            COM(4,11,[0,3,1,3,2,0]), //XX(XX)X
            PTR(30, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 32 
            COM(5,16,[0,1,2,3,4,0]), //XX(XXX)
            COM(4,6,[0,1,2,3,0,0]), //XX(XX)
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(4,2,[3,1,0,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN10Ordlist.boolList
        vec![ // 33 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            PTR(41, false),
            PTR(34, false),
        ], 
        vec![ // 34 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 35 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
        vec![ // 36 
            PTR(17, false),
            PTR(35, false),
        ], 
        vec![ // 37 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(36, false),
            PTR(33, false),
        ], 
        vec![ // 38 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 39 
            PTR(17, false),
            PTR(38, false),
        ], 
        vec![ // 40 
            COM(5,41,[0,1,2,4,3,4]), //X(X(XX))(XX)
            PTR(14, false),
            PTR(39, false),
            PTR(33, false),
            PTR(37, false),
        ], 
        vec![ // 41 
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            PTR(14, false),
            PTR(33, false),
            PTR(40, false),
        ], 
         // FUN11Ordlist.S
        vec![ // 42 
            COM(3,3,[0,2,1,0,0,0]), //X(XX)
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
    ]
});