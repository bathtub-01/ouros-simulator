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
            PTR(1),
            PTR(57),
            INT(0),
            INT(1),
        ], 
         // FUN1Taut.isTaut
        vec![ // 1 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(3),
            PTR(2),
        ], 
        vec![ // 2 
            COM(5,40,[0,1,2,4,3,4]), //X(XXX)(XX)
            PTR(5),
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(8),
            PTR(24),
        ], 
         // FUN2NanoPrelude.and
        vec![ // 3 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(4),
        ], 
        vec![ // 4 
            COM(4,6,[2,0,1,3,0,0]), //XX(XX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(3),
        ], 
         // FUN3NanoPrelude.map
        vec![ // 5 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(7),
        ], 
        vec![ // 6 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 7 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(6),
        ], 
         // FUN4Taut.eval
        vec![ // 8 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(16),
        ], 
        vec![ // 9 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(17),
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
            PTR(11),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 13 
            COM(4,14,[1,2,0,1,3,0]), //XXX(XX)
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 14 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(13),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 15 
            COM(5,38,[0,1,4,2,3,4]), //X(XX)X(XX)
            COM(6,23,[5,0,1,2,3,4]), //XXXXXX
            PTR(14),
            COM(1,0,[0,0,0,0,0,0]), //X
            PTR(12),
        ], 
        vec![ // 16 
            COM(5,39,[0,4,1,4,2,3]), //XX(XX)(XX)
            PTR(15),
            PTR(10),
            PTR(9),
        ], 
         // FUN5Taut.find
        vec![ // 17 
            COM(5,16,[0,1,2,3,4,0]), //XX(XXX)
            COM(1,0,[0,0,0,0,0,0]), //X
            PTR(18),
            PTR(19),
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
            PTR(23),
        ], 
        vec![ // 20 
            COM(5,14,[0,1,4,2,3,0]), //XXX(XX)
            PRM(EQ,false),
        ], 
        vec![ // 21 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(6,37,[0,2,3,4,1,5]), //XXXX(XX)
            PTR(20),
            COM(3,1,[2,0,0,0,0,0]), //XX
        ], 
        vec![ // 22 
            COM(5,18,[3,0,1,2,4,0]), //X(XXXX)
            PTR(21),
        ], 
        vec![ // 23 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(22),
        ], 
         // FUN8Taut.substs
        vec![ // 24 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(26),
            PTR(25),
        ], 
        vec![ // 25 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(45),
            PTR(52),
        ], 
        vec![ // 26 
            COM(5,45,[0,1,4,2,3,4]), //X(XX)(X(XX))
            PTR(5),
            PTR(27),
            PTR(31),
            PTR(42),
        ], 
         // FUN9NanoPrelude.zip
        vec![ // 27 
            COM(4,6,[2,0,1,3,0,0]), //XX(XX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(30),
        ], 
        vec![ // 28 
            COM(6,44,[0,1,4,2,3,5]), //X(XX)(XXX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 29 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(28),
            COM(3,2,[2,0,1,0,0,0]), //XXX
            PTR(27),
        ], 
        vec![ // 30 
            COM(5,16,[2,0,1,3,4,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(29),
        ], 
         // FUN10Taut.bools
        vec![ // 31 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(37),
            PTR(32),
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
            PTR(5),
            PTR(41),
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(31),
        ], 
        vec![ // 35 
            COM(6,57,[0,1,2,3,4,5]), //X(X(XX)(XX))
            PTR(38),
            PTR(5),
            PTR(41),
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(31),
        ], 
        vec![ // 36 
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(35),
            PTR(34),
        ], 
        vec![ // 37 
            COM(5,43,[0,4,1,2,3,4]), //XXX(X(XX))
            PRM(EQ,false),
            INT(0),
            PTR(36),
            PTR(33),
        ], 
         // FUN11Data.List_Type.++
        vec![ // 38 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            Y,
            PTR(40),
        ], 
        vec![ // 39 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 40 
            COM(4,6,[3,1,0,2,0,0]), //XX(XX)
            PTR(39),
        ], 
         // FUN12Data.List_Type.:
        vec![ // 41 
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
         // FUN13NanoPrelude.length
        vec![ // 42 
            Y,
            PTR(44),
            INT(0),
        ], 
        vec![ // 43 
            COM(5,7,[2,0,3,1,0,0]), //X(XXX)
            PRM(ADD,false),
            INT(1),
        ], 
        vec![ // 44 
            COM(4,16,[3,2,0,1,2,0]), //XX(XXX)
            PTR(43),
        ], 
         // FUN14Taut.rmdups
        vec![ // 45 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(47),
        ], 
        vec![ // 46 
            COM(5,19,[0,1,2,3,4,0]), //X(X(XX)X)
            PTR(45),
            PTR(48),
            PRM(EQ,true),
        ], 
        vec![ // 47 
            COM(4,16,[0,2,1,2,3,0]), //XX(XXX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(46),
        ], 
         // FUN15NanoPrelude.filter
        vec![ // 48 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(51),
        ], 
        vec![ // 49 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 50 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(4,30,[0,2,1,2,3,3]), //XX(XXX)X
            COM(5,39,[0,3,1,4,2,4]), //XX(XX)(XX)
            PTR(49),
        ], 
        vec![ // 51 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(50),
        ], 
         // FUN16Taut.vars
        vec![ // 52 
            COM(6,23,[5,0,1,2,3,4]), //XXXXXX
            PTR(56),
            PTR(55),
            PTR(54),
            PTR(52),
            PTR(53),
        ], 
        vec![ // 53 
            COM(4,2,[3,1,0,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 54 
            COM(4,15,[0,1,2,1,3,0]), //X(XX)(XX)
            PTR(38),
            PTR(52),
        ], 
        vec![ // 55 
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 56 
            COM(4,15,[0,1,2,1,3,0]), //X(XX)(XX)
            PTR(38),
            PTR(52),
        ], 
         // FUN17Taut.testProp
        vec![ // 57 
            COM(7,2,[4,0,1,0,0,0]), //XXX
            PTR(63),
            PTR(61),
        ], 
        vec![ // 58 
            PTR(5),
            COM(6,1,[5,0,0,0,0,0]), //XX
            PTR(67),
        ], 
        vec![ // 59 
            PTR(64),
            COM(7,2,[2,0,1,0,0,0]), //XXX
            PTR(58),
        ], 
        vec![ // 60 
            COM(6,1,[5,0,0,0,0,0]), //XX
            INT(42),
        ], 
        vec![ // 61 
            COM(7,2,[4,0,1,0,0,0]), //XXX
            PTR(60),
            PTR(59),
        ], 
        vec![ // 62 
            PTR(5),
            PTR(66),
            PTR(67),
        ], 
        vec![ // 63 
            PTR(64),
            COM(7,2,[2,0,1,0,0,0]), //XXX
            PTR(62),
        ], 
         // FUN18NanoPrelude.foldr1
        vec![ // 64 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(65),
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
            PTR(72),
        ], 
        vec![ // 68 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(5),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 69 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(4),
            PTR(68),
        ], 
        vec![ // 70 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(3),
            PTR(69),
        ], 
        vec![ // 71 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(2),
            PTR(70),
        ], 
        vec![ // 72 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(1),
            PTR(71),
        ], 
    ]
});