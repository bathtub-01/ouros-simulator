use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 23
// Apps in this file: 78
// Combinators in this file: 128
#[rustfmt::skip]
pub static TAUT: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0Taut.main
        vec![ // 0 
            PTR(1, false),
            PTR(55, false),
            INT(0),
            INT(1),
        ], 
         // FUN1Taut.isTaut
        vec![ // 1 
            COM(5,57,[0,1,2,4,3,4]), //X(X(XX)(XX))
            PTR(3, false),
            PTR(5, false),
            PTR(2, false),
            PTR(22, false),
        ], 
        vec![ // 2 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(8, false),
        ], 
         // FUN2NanoPrelude.and
        vec![ // 3 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(4, false),
        ], 
        vec![ // 4 
            COM(4,5,[0,3,1,2,0,0]), //X(XX)X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
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
            COM(5,13,[0,1,2,4,3,0]), //X(X(XX))X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
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
            COM(4,12,[0,2,3,1,2,0]), //X(XXX)X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
        vec![ // 12 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(11, false),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 13 
            COM(4,12,[0,2,3,1,2,0]), //X(XXX)X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
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
            COM(4,6,[0,1,2,3,0,0]), //XX(XX)
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
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
            COM(4,8,[0,1,2,3,0,0]), //X(X(XX))
            Y,
            PTR(21, false),
            PTR(20, false),
        ], 
        vec![ // 20 
            COM(6,34,[0,1,2,3,5,4]), //X(XX(XX))X
            COM(5,19,[3,0,2,4,1,0]), //X(X(XX)X)
            COM(5,12,[0,1,4,2,3,0]), //X(XXX)X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PRM(EQ,false),
            COM(3,1,[2,0,0,0,0,0]), //XX
        ], 
        vec![ // 21 
            COM(4,6,[0,1,2,3,0,0]), //XX(XX)
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN8Taut.substs
        vec![ // 22 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(24, false),
            PTR(23, false),
        ], 
        vec![ // 23 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(44, false),
            PTR(50, false),
        ], 
        vec![ // 24 
            COM(5,45,[0,1,4,2,3,4]), //X(XX)(X(XX))
            PTR(5, false),
            PTR(25, false),
            PTR(28, false),
            PTR(41, false),
        ], 
         // FUN9NanoPrelude.zip
        vec![ // 25 
            COM(4,5,[0,3,1,2,0,0]), //X(XX)X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(27, false),
        ], 
        vec![ // 26 
            COM(6,30,[0,1,2,3,5,4]), //XX(XXX)X
            COM(4,6,[0,1,2,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(3,2,[2,0,1,0,0,0]), //XXX
        ], 
        vec![ // 27 
            COM(6,25,[0,1,5,2,3,4]), //XX(XX)XX
            COM(5,11,[0,1,2,4,3,0]), //XX(XX)X
            COM(4,8,[0,1,2,3,0,0]), //X(X(XX))
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(26, false),
            PTR(25, false),
        ], 
         // FUN10Taut.bools
        vec![ // 28 
            COM(5,28,[0,4,1,2,4,3]), //XXX(XX)X
            PRM(EQ,false),
            INT(0),
            PTR(36, false),
            PTR(29, false),
        ], 
        vec![ // 29 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 30 
            PTR(40, false),
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
        vec![ // 31 
            PTR(5, false),
            PTR(30, false),
        ], 
        vec![ // 32 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(31, false),
            PTR(28, false),
        ], 
        vec![ // 33 
            PTR(40, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 34 
            PTR(5, false),
            PTR(33, false),
        ], 
        vec![ // 35 
            COM(5,41,[0,1,2,4,3,4]), //X(X(XX))(XX)
            PTR(37, false),
            PTR(34, false),
            PTR(28, false),
            PTR(32, false),
        ], 
        vec![ // 36 
            COM(4,7,[0,1,3,2,0,0]), //X(XXX)
            PTR(35, false),
            PRM(SUB,false),
            INT(1),
        ], 
         // FUN11Data.List_Type.++
        vec![ // 37 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            Y,
            PTR(39, false),
        ], 
        vec![ // 38 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 39 
            COM(4,6,[3,1,0,2,0,0]), //XX(XX)
            PTR(38, false),
        ], 
         // FUN12Data.List_Type.:
        vec![ // 40 
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
         // FUN13NanoPrelude.length
        vec![ // 41 
            Y,
            PTR(43, false),
            INT(0),
        ], 
        vec![ // 42 
            COM(6,49,[0,1,4,2,5,3]), //XX(X(XXX))
            COM(4,6,[3,2,0,1,0,0]), //XX(XX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PRM(ADD,false),
            INT(1),
        ], 
        vec![ // 43 
            COM(3,4,[0,1,2,2,0,0]), //XXXX
            PTR(42, false),
        ], 
         // FUN14Taut.rmdups
        vec![ // 44 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(46, false),
        ], 
        vec![ // 45 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(47, false),
            PRM(EQ,true),
        ], 
        vec![ // 46 
            COM(5,38,[0,1,4,2,3,4]), //X(XX)X(XX)
            COM(4,8,[0,1,2,3,0,0]), //X(X(XX))
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(44, false),
            PTR(45, false),
        ], 
         // FUN15NanoPrelude.filter
        vec![ // 47 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(49, false),
        ], 
        vec![ // 48 
            COM(5,38,[0,2,4,3,1,4]), //X(XX)X(XX)
            COM(4,45,[0,1,3,2,1,3]), //X(XX)(X(XX))
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 49 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(48, false),
        ], 
         // FUN16Taut.vars
        vec![ // 50 
            COM(6,23,[5,0,1,2,3,4]), //XXXXXX
            PTR(54, false),
            PTR(53, false),
            PTR(52, false),
            PTR(50, false),
            PTR(51, false),
        ], 
        vec![ // 51 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 52 
            COM(4,13,[0,1,2,3,2,0]), //X(X(XX))X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(37, false),
            PTR(50, false),
        ], 
        vec![ // 53 
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 54 
            COM(4,13,[0,1,2,3,2,0]), //X(X(XX))X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(37, false),
            PTR(50, false),
        ], 
         // FUN17Taut.testProp
        vec![ // 55 
            PTR(62, false),
            PTR(61, false),
            PTR(59, false),
        ], 
        vec![ // 56 
            PTR(5, false),
            COM(6,1,[5,0,0,0,0,0]), //XX
            PTR(72, false),
        ], 
        vec![ // 57 
            PTR(63, false),
            PTR(67, false),
            PTR(56, false),
        ], 
        vec![ // 58 
            COM(6,1,[5,0,0,0,0,0]), //XX
            INT(42),
        ], 
        vec![ // 59 
            PTR(62, false),
            PTR(58, false),
            PTR(57, false),
        ], 
        vec![ // 60 
            PTR(5, false),
            PTR(69, false),
            PTR(72, false),
        ], 
        vec![ // 61 
            PTR(63, false),
            PTR(67, false),
            PTR(60, false),
        ], 
         // FUN18Taut.Implies
        vec![ // 62 
            COM(7,16,[0,1,6,2,3,0]), //XX(XXX)
            COM(3,1,[0,1,0,0,0,0]), //XX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN19NanoPrelude.foldr1
        vec![ // 63 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(66, false),
        ], 
        vec![ // 64 
            COM(5,17,[0,1,2,3,4,0]), //XX(X(XX))
            COM(3,1,[0,1,0,0,0,0]), //XX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 65 
            COM(5,33,[0,1,2,4,3,4]), //X(X(XX)X)X
            COM(3,6,[2,1,0,2,0,0]), //XX(XX)
            PTR(64, false),
        ], 
        vec![ // 66 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            ERR(0),
            PTR(65, false),
        ], 
         // FUN20Taut.And
        vec![ // 67 
            COM(6,46,[0,1,2,3,4,5]), //XX(XXXX)
            COM(3,1,[0,1,0,0,0,0]), //XX
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(68, false),
        ], 
        vec![ // 68 
            COM(5,16,[0,1,4,2,3,0]), //XX(XXX)
            COM(3,1,[0,1,0,0,0,0]), //XX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN21Taut.imp
        vec![ // 69 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(71, false),
            COM(6,1,[5,0,0,0,0,0]), //XX
        ], 
        vec![ // 70 
            COM(6,1,[5,0,0,0,0,0]), //XX
            INT(42),
        ], 
        vec![ // 71 
            PTR(62, false),
            PTR(70, false),
        ], 
         // FUN22Taut.names
        vec![ // 72 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(0),
            PTR(77, false),
        ], 
        vec![ // 73 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(5),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 74 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(4),
            PTR(73, false),
        ], 
        vec![ // 75 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(3),
            PTR(74, false),
        ], 
        vec![ // 76 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(2),
            PTR(75, false),
        ], 
        vec![ // 77 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(1),
            PTR(76, false),
        ], 
    ]
});