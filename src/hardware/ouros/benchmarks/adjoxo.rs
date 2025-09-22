use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 23
// Apps in this file: 105
// Combinators in this file: 155
#[rustfmt::skip]
pub static ADJOXO: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0Adjoxo.main
        vec![ // 0 
            PTR(5, false),
            PTR(4, false),
            PTR(2, false),
        ], 
        vec![ // 1 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(5),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 2 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(2),
            PTR(1, false),
        ], 
        vec![ // 3 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(4),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 4 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(1),
            PTR(3, false),
        ], 
         // FUN1Adjoxo.adjudicate
        vec![ // 5 
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
            PTR(15, false),
            PTR(6, false),
        ], 
        vec![ // 6 
            COM(5,12,[0,1,4,3,2,0]), //X(XXX)X
            PTR(69, false),
            PTR(73, false),
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
        vec![ // 7 
            COM(5,12,[0,1,3,4,2,0]), //X(XXX)X
            PTR(69, false),
            PTR(73, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 8 
            PTR(69, false),
            COM(3,0,[2,0,0,0,0,0]), //X
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
        vec![ // 9 
            PTR(69, false),
            COM(3,0,[2,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 10 
            COM(5,12,[0,1,4,3,2,0]), //X(XXX)X
            PTR(69, false),
            PTR(73, false),
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
        vec![ // 11 
            COM(5,30,[0,3,1,3,4,2]), //XX(XXX)X
            PTR(22, false),
            PTR(10, false),
            PTR(9, false),
        ], 
        vec![ // 12 
            COM(5,30,[0,4,1,3,4,2]), //XX(XXX)X
            PTR(22, false),
            PTR(11, false),
            PTR(8, false),
        ], 
        vec![ // 13 
            COM(4,15,[0,1,2,1,3,0]), //X(XX)(XX)
            PTR(16, false),
            PTR(19, false),
        ], 
        vec![ // 14 
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            COM(4,39,[0,3,1,3,2,3]), //XX(XX)(XX)
            PTR(13, false),
            PTR(12, false),
        ], 
        vec![ // 15 
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(14, false),
            PTR(7, false),
        ], 
         // FUN2Adjoxo.cmp
        vec![ // 16 
            COM(4,10,[0,1,3,2,3,0]), //X(XX)XX
            PTR(18, false),
            PTR(17, false),
            COM(3,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 17 
            COM(5,9,[0,3,4,1,2,0]), //XXXXX
            PRM(LE,false),
            COM(3,0,[2,0,0,0,0,0]), //X
            COM(3,0,[1,0,0,0,0,0]), //X
        ], 
        vec![ // 18 
            COM(5,28,[0,3,4,1,4,2]), //XXX(XX)X
            PRM(EQ,false),
        ], 
         // FUN3NanoPrelude.length
        vec![ // 19 
            Y,
            PTR(21, false),
            INT(0),
        ], 
        vec![ // 20 
            COM(5,7,[2,0,3,1,0,0]), //X(XXX)
            PRM(ADD,false),
            INT(1),
        ], 
        vec![ // 21 
            COM(4,16,[3,2,0,1,2,0]), //XX(XXX)
            PTR(20, false),
        ], 
         // FUN4Adjoxo.hasLine
        vec![ // 22 
            COM(5,40,[0,1,2,4,3,4]), //X(XXX)(XX)
            PTR(54, false),
            PTR(55, false),
            PTR(53, false),
            PTR(50, false),
        ], 
        vec![ // 23 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(7),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 24 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(5),
            PTR(23, false),
        ], 
        vec![ // 25 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(3),
            PTR(24, false),
        ], 
        vec![ // 26 
            PTR(55, false),
            PTR(25, false),
        ], 
        vec![ // 27 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(9),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 28 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(5),
            PTR(27, false),
        ], 
        vec![ // 29 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(1),
            PTR(28, false),
        ], 
        vec![ // 30 
            COM(5,40,[0,1,2,4,3,4]), //X(XXX)(XX)
            PTR(54, false),
            PTR(55, false),
            PTR(29, false),
            PTR(26, false),
        ], 
        vec![ // 31 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(9),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 32 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(6),
            PTR(31, false),
        ], 
        vec![ // 33 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(3),
            PTR(32, false),
        ], 
        vec![ // 34 
            COM(5,40,[0,1,2,4,3,4]), //X(XXX)(XX)
            PTR(54, false),
            PTR(55, false),
            PTR(33, false),
            PTR(30, false),
        ], 
        vec![ // 35 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(8),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 36 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(5),
            PTR(35, false),
        ], 
        vec![ // 37 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(2),
            PTR(36, false),
        ], 
        vec![ // 38 
            COM(5,40,[0,1,2,4,3,4]), //X(XXX)(XX)
            PTR(54, false),
            PTR(55, false),
            PTR(37, false),
            PTR(34, false),
        ], 
        vec![ // 39 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(7),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 40 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(4),
            PTR(39, false),
        ], 
        vec![ // 41 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(1),
            PTR(40, false),
        ], 
        vec![ // 42 
            COM(5,40,[0,1,2,4,3,4]), //X(XXX)(XX)
            PTR(54, false),
            PTR(55, false),
            PTR(41, false),
            PTR(38, false),
        ], 
        vec![ // 43 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(9),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 44 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(8),
            PTR(43, false),
        ], 
        vec![ // 45 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(7),
            PTR(44, false),
        ], 
        vec![ // 46 
            COM(5,40,[0,1,2,4,3,4]), //X(XXX)(XX)
            PTR(54, false),
            PTR(55, false),
            PTR(45, false),
            PTR(42, false),
        ], 
        vec![ // 47 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(6),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 48 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(5),
            PTR(47, false),
        ], 
        vec![ // 49 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(4),
            PTR(48, false),
        ], 
        vec![ // 50 
            COM(5,40,[0,1,2,4,3,4]), //X(XXX)(XX)
            PTR(54, false),
            PTR(55, false),
            PTR(49, false),
            PTR(46, false),
        ], 
        vec![ // 51 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(3),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 52 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(2),
            PTR(51, false),
        ], 
        vec![ // 53 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(1),
            PTR(52, false),
        ], 
         // FUN5Data.Bool.||
        vec![ // 54 
            COM(3,2,[1,2,0,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
         // FUN6Adjoxo.subset
        vec![ // 55 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            PTR(56, false),
            PTR(58, false),
        ], 
         // FUN7NanoPrelude.null
        vec![ // 56 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(57, false),
        ], 
        vec![ // 57 
            COM(3,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN8Adjoxo.diff
        vec![ // 58 
            COM(4,6,[2,0,1,3,0,0]), //XX(XX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(68, false),
        ], 
        vec![ // 59 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            PTR(58, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 60 
            COM(5,16,[0,2,1,3,4,0]), //XX(XXX)
            PTR(58, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 61 
            COM(6,42,[0,2,4,1,3,5]), //XXX(XXX)
            PTR(16, false),
            PTR(58, false),
        ], 
        vec![ // 62 
            COM(6,26,[0,1,4,5,2,3]), //X(XXX)XX
            COM(5,48,[0,4,1,2,3,4]), //XX(XX(XX))
        ], 
        vec![ // 63 
            COM(4,10,[0,1,3,2,3,0]), //X(XX)XX
            PTR(62, false),
            PTR(61, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 64 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
            PTR(63, false),
            PTR(60, false),
        ], 
        vec![ // 65 
            COM(5,38,[0,1,2,4,3,4]), //X(XX)X(XX)
            COM(5,37,[0,1,3,4,2,4]), //XXXX(XX)
            PTR(64, false),
        ], 
        vec![ // 66 
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(65, false),
            PTR(59, false),
        ], 
        vec![ // 67 
            COM(5,40,[1,0,3,4,2,4]), //X(XXX)(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 68 
            COM(4,11,[0,2,1,3,3,0]), //XX(XX)X
            PTR(67, false),
            PTR(66, false),
        ], 
         // FUN9Adjoxo.report
        vec![ // 69 
            COM(4,14,[0,2,3,1,3,0]), //XXX(XX)
            PTR(70, false),
            PTR(71, false),
        ], 
        vec![ // 70 
            COM(5,17,[3,0,1,2,4,0]), //XX(X(XX))
            INT(3),
            PTR(71, false),
            PTR(72, false),
        ], 
         // FUN10Adjoxo.side
        vec![ // 71 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(0),
            INT(88),
        ], 
         // FUN11Adjoxo.opp
        vec![ // 72 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN12Adjoxo.analysis
        vec![ // 73 
            COM(5,30,[0,4,1,3,4,2]), //XX(XXX)X
            PTR(22, false),
            PTR(79, false),
            COM(3,0,[1,0,0,0,0,0]), //X
        ], 
        vec![ // 74 
            COM(6,54,[0,1,2,3,4,5]), //X(X(XXX)X)
            PTR(58, false),
            PTR(58, false),
            PTR(98, false),
            INT(1),
            INT(9),
        ], 
        vec![ // 75 
            COM(5,40,[0,1,3,4,2,4]), //X(XXX)(XX)
            PTR(86, false),
            PTR(89, false),
        ], 
        vec![ // 76 
            COM(3,5,[0,1,2,2,0,0]), //X(XX)X
            PTR(75, false),
            PTR(74, false),
        ], 
        vec![ // 77 
            COM(5,16,[0,1,2,3,4,0]), //XX(XXX)
            PTR(82, false),
            PTR(84, false),
            PTR(76, false),
        ], 
        vec![ // 78 
            COM(5,28,[0,3,4,1,4,2]), //XXX(XX)X
            PTR(80, false),
        ], 
        vec![ // 79 
            COM(4,10,[0,1,3,2,3,0]), //X(XX)XX
            PTR(78, false),
            PTR(77, false),
            COM(3,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN13Adjoxo.gridFull
        vec![ // 80 
            COM(4,4,[0,2,3,1,0,0]), //XXXX
            PTR(81, false),
            INT(9),
        ], 
        vec![ // 81 
            COM(5,57,[0,1,2,3,2,4]), //X(X(XX)(XX))
            PRM(EQ,false),
            PRM(ADD,false),
            PTR(19, false),
        ], 
         // FUN14NanoPrelude.foldr1
        vec![ // 82 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(83, false),
        ], 
        vec![ // 83 
            COM(6,47,[5,0,1,2,3,4]), //XX(X(XX)X)
            ERR(0),
            COM(4,46,[3,2,0,1,2,3]), //XX(XXXX)
            COM(6,6,[0,2,1,3,0,0]), //XX(XX)
        ], 
         // FUN15Adjoxo.bestOf
        vec![ // 84 
            COM(4,4,[0,2,3,1,0,0]), //XXXX
            PTR(85, false),
            COM(3,0,[2,0,0,0,0,0]), //X
        ], 
        vec![ // 85 
            COM(4,32,[2,3,0,0,1,3]), //X(XXXX)X
            COM(3,0,[0,0,0,0,0,0]), //X
            COM(3,0,[2,0,0,0,0,0]), //X
        ], 
         // FUN16NanoPrelude.map
        vec![ // 86 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(88, false),
        ], 
        vec![ // 87 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 88 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(87, false),
        ], 
         // FUN17Adjoxo.moveval
        vec![ // 89 
            COM(6,58,[0,1,4,2,5,3]), //X(XX(XXX))
            PTR(90, false),
            PTR(73, false),
            PTR(91, false),
        ], 
         // FUN18Adjoxo.inverse
        vec![ // 90 
            COM(4,4,[3,0,1,2,0,0]), //XXXX
            COM(3,0,[0,0,0,0,0,0]), //X
            COM(3,0,[2,0,0,0,0,0]), //X
            COM(3,0,[1,0,0,0,0,0]), //X
        ], 
         // FUN19Adjoxo.insert
        vec![ // 91 
            COM(4,19,[0,1,2,3,3,0]), //X(X(XX)X)
            Y,
            PTR(97, false),
            PTR(96, false),
        ], 
        vec![ // 92 
            COM(4,16,[0,1,0,2,3,0]), //XX(XXX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 93 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 94 
            COM(5,42,[0,1,3,2,3,4]), //XXX(XXX)
            PRM(LE,false),
        ], 
        vec![ // 95 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            PTR(94, false),
            PTR(93, false),
        ], 
        vec![ // 96 
            COM(5,40,[0,1,3,4,2,3]), //X(XXX)(XX)
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
            PTR(95, false),
            PTR(92, false),
        ], 
        vec![ // 97 
            COM(6,40,[5,0,3,1,2,4]), //X(XXX)(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN20NanoPrelude.enumFromTo
        vec![ // 98 
            COM(5,15,[0,1,4,2,3,0]), //X(XX)(XX)
            PTR(100, false),
            PTR(99, false),
            PTR(104, false),
        ], 
        vec![ // 99 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PRM(LE,false),
        ], 
         // FUN21NanoPrelude.takeWhile
        vec![ // 100 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(103, false),
        ], 
        vec![ // 101 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 102 
            COM(5,42,[1,3,0,2,3,4]), //XXX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 103 
            COM(6,48,[5,0,1,3,2,4]), //XX(XX(XX))
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(102, false),
            PTR(101, false),
        ], 
         // FUN22NanoPrelude.enumFrom
        vec![ // 104 
            COM(5,49,[0,4,1,2,4,3]), //XX(X(XXX))
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(104, false),
            PRM(ADD,false),
            INT(1),
        ], 
    ]
});