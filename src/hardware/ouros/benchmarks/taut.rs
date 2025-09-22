use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 21
// Apps in this file: 73
// Combinators in this file: 111
#[rustfmt::skip]
pub static TAUT: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0Taut.main
        vec![ // 0 
            PTR(1, false),
            PTR(57, false),
            INT(0),
            INT(1),
        ], 
         // FUN1Taut.isTaut
        vec![ // 1 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(3, false),
            PTR(2, false),
        ], 
        vec![ // 2 
            COM(5,40,[0,1,2,4,3,4]), //X(XXX)(XX)
            PTR(5, false),
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(8, false),
            PTR(24, false),
        ], 
         // FUN2NanoPrelude.and
        vec![ // 3 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(4, false),
        ], 
        vec![ // 4 
            COM(4,6,[2,0,1,3,0,0]), //XX(XX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(3, false),
        ], 
         // FUN3NanoPrelude.map
        vec![ // 5 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(7, false),
        ], 
        vec![ // 6 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 7 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(6, false),
        ], 
         // FUN4Taut.eval
        vec![ // 8 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(16, false),
        ], 
        vec![ // 9 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(17, false),
        ], 
        vec![ // 10 
            COM(4,4,[2,3,0,1,0,0]), //XXXX
            COM(2,0,[1,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 11 
            COM(4,14,[1,2,0,1,3,0]), //XXX(XX)
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
        vec![ // 12 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(11, false),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 13 
            COM(4,14,[1,2,0,1,3,0]), //XXX(XX)
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 14 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(13, false),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 15 
            COM(5,38,[0,1,4,2,3,4]), //X(XX)X(XX)
            COM(6,23,[5,0,1,2,3,4]), //XXXXXX
            PTR(14, false),
            COM(1,0,[0,0,0,0,0,0]), //X
            PTR(12, false),
        ], 
        vec![ // 16 
            COM(5,39,[0,4,1,4,2,3]), //XX(XX)(XX)
            PTR(15, false),
            PTR(10, false),
            PTR(9, false),
        ], 
         // FUN5Taut.find
        vec![ // 17 
            COM(5,16,[0,1,2,3,4,0]), //XX(XXX)
            COM(1,0,[0,0,0,0,0,0]), //X
            PTR(18, false),
            PTR(19, false),
        ], 
         // FUN6NanoPrelude.fromJust
        vec![ // 18 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            ERR(4),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN7NanoPrelude.lookup
        vec![ // 19 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(23, false),
        ], 
        vec![ // 20 
            COM(5,14,[0,1,4,2,3,0]), //XXX(XX)
            PRM(EQ,false),
        ], 
        vec![ // 21 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(6,37,[0,2,3,4,1,5]), //XXXX(XX)
            PTR(20, false),
            COM(3,1,[2,0,0,0,0,0]), //XX
        ], 
        vec![ // 22 
            COM(5,18,[3,0,1,2,4,0]), //X(XXXX)
            PTR(21, false),
        ], 
        vec![ // 23 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(22, false),
        ], 
         // FUN8Taut.substs
        vec![ // 24 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(26, false),
            PTR(25, false),
        ], 
        vec![ // 25 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(45, false),
            PTR(52, false),
        ], 
        vec![ // 26 
            COM(5,45,[0,1,4,2,3,4]), //X(XX)(X(XX))
            PTR(5, false),
            PTR(27, false),
            PTR(31, false),
            PTR(42, false),
        ], 
         // FUN9NanoPrelude.zip
        vec![ // 27 
            COM(4,6,[2,0,1,3,0,0]), //XX(XX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(30, false),
        ], 
        vec![ // 28 
            COM(6,44,[0,1,4,2,3,5]), //X(XX)(XXX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 29 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(28, false),
            COM(3,2,[2,0,1,0,0,0]), //XXX
            PTR(27, false),
        ], 
        vec![ // 30 
            COM(5,16,[2,0,1,3,4,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(29, false),
        ], 
         // FUN10Taut.bools
        vec![ // 31 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(37, false),
            PTR(32, false),
        ], 
        vec![ // 32 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 33 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PRM(SUB,false),
            INT(1),
        ], 
        vec![ // 34 
            COM(5,15,[0,1,2,3,4,0]), //X(XX)(XX)
            PTR(5, false),
            PTR(41, false),
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(31, false),
        ], 
        vec![ // 35 
            COM(6,57,[0,1,2,3,4,5]), //X(X(XX)(XX))
            PTR(38, false),
            PTR(5, false),
            PTR(41, false),
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(31, false),
        ], 
        vec![ // 36 
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(35, false),
            PTR(34, false),
        ], 
        vec![ // 37 
            COM(5,43,[0,4,1,2,3,4]), //XXX(X(XX))
            PRM(EQ,false),
            INT(0),
            PTR(36, false),
            PTR(33, false),
        ], 
         // FUN11Data.List_Type.++
        vec![ // 38 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            Y,
            PTR(40, false),
        ], 
        vec![ // 39 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 40 
            COM(4,6,[3,1,0,2,0,0]), //XX(XX)
            PTR(39, false),
        ], 
         // FUN12Data.List_Type.:
        vec![ // 41 
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
         // FUN13NanoPrelude.length
        vec![ // 42 
            Y,
            PTR(44, false),
            INT(0),
        ], 
        vec![ // 43 
            COM(5,7,[2,0,3,1,0,0]), //X(XXX)
            PRM(ADD,false),
            INT(1),
        ], 
        vec![ // 44 
            COM(4,16,[3,2,0,1,2,0]), //XX(XXX)
            PTR(43, false),
        ], 
         // FUN14Taut.rmdups
        vec![ // 45 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(47, false),
        ], 
        vec![ // 46 
            COM(5,19,[0,1,2,3,4,0]), //X(X(XX)X)
            PTR(45, false),
            PTR(48, false),
            PRM(EQ,true),
        ], 
        vec![ // 47 
            COM(4,16,[0,2,1,2,3,0]), //XX(XXX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(46, false),
        ], 
         // FUN15NanoPrelude.filter
        vec![ // 48 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(51, false),
        ], 
        vec![ // 49 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 50 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(4,30,[0,2,1,2,3,3]), //XX(XXX)X
            COM(5,39,[0,3,1,4,2,4]), //XX(XX)(XX)
            PTR(49, false),
        ], 
        vec![ // 51 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(50, false),
        ], 
         // FUN16Taut.vars
        vec![ // 52 
            COM(6,23,[5,0,1,2,3,4]), //XXXXXX
            PTR(56, false),
            PTR(55, false),
            PTR(54, false),
            PTR(52, false),
            PTR(53, false),
        ], 
        vec![ // 53 
            COM(4,2,[3,1,0,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 54 
            COM(4,15,[0,1,2,1,3,0]), //X(XX)(XX)
            PTR(38, false),
            PTR(52, false),
        ], 
        vec![ // 55 
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 56 
            COM(4,15,[0,1,2,1,3,0]), //X(XX)(XX)
            PTR(38, false),
            PTR(52, false),
        ], 
         // FUN17Taut.testProp
        vec![ // 57 
            COM(7,2,[4,0,1,0,0,0]), //XXX
            PTR(63, false),
            PTR(61, false),
        ], 
        vec![ // 58 
            PTR(5, false),
            COM(6,1,[5,0,0,0,0,0]), //XX
            PTR(67, false),
        ], 
        vec![ // 59 
            PTR(64, false),
            COM(7,2,[2,0,1,0,0,0]), //XXX
            PTR(58, false),
        ], 
        vec![ // 60 
            COM(6,1,[5,0,0,0,0,0]), //XX
            INT(42),
        ], 
        vec![ // 61 
            COM(7,2,[4,0,1,0,0,0]), //XXX
            PTR(60, false),
            PTR(59, false),
        ], 
        vec![ // 62 
            PTR(5, false),
            PTR(66, false),
            PTR(67, false),
        ], 
        vec![ // 63 
            PTR(64, false),
            COM(7,2,[2,0,1,0,0,0]), //XXX
            PTR(62, false),
        ], 
         // FUN18NanoPrelude.foldr1
        vec![ // 64 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(65, false),
        ], 
        vec![ // 65 
            COM(6,47,[5,0,1,2,3,4]), //XX(X(XX)X)
            ERR(0),
            COM(4,46,[3,2,0,1,2,3]), //XX(XXXX)
            COM(6,6,[0,2,1,3,0,0]), //XX(XX)
        ], 
         // FUN19Taut.imp
        vec![ // 66 
            COM(5,15,[0,1,2,3,4,0]), //X(XX)(XX)
            COM(7,2,[4,0,1,0,0,0]), //XXX
            COM(6,1,[5,0,0,0,0,0]), //XX
            INT(42),
            COM(6,1,[5,0,0,0,0,0]), //XX
        ], 
         // FUN20Taut.names
        vec![ // 67 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(0),
            PTR(72, false),
        ], 
        vec![ // 68 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(5),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 69 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(4),
            PTR(68, false),
        ], 
        vec![ // 70 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(3),
            PTR(69, false),
        ], 
        vec![ // 71 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(2),
            PTR(70, false),
        ], 
        vec![ // 72 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(1),
            PTR(71, false),
        ], 
    ]
});