use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 23
// Apps in this file: 106
// Combinators in this file: 175
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
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(13, false),
            PTR(6, false),
        ], 
        vec![ // 6 
            COM(5,12,[0,1,4,3,2,0]), //X(XXX)X
            PTR(70, false),
            PTR(73, false),
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
        vec![ // 7 
            PTR(70, false),
            COM(3,0,[2,0,0,0,0,0]), //X
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
        vec![ // 8 
            PTR(70, false),
            COM(3,0,[2,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 9 
            COM(6,24,[0,1,5,2,3,4]), //X(XX)XXX
            COM(6,54,[0,1,2,5,4,3]), //X(X(XXX)X)
            PTR(19, false),
            PTR(70, false),
            PTR(73, false),
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
        vec![ // 10 
            COM(5,30,[0,1,2,4,4,3]), //XX(XXX)X
            COM(5,30,[0,4,1,4,2,3]), //XX(XXX)X
            PTR(19, false),
            PTR(9, false),
            PTR(8, false),
        ], 
        vec![ // 11 
            COM(5,13,[0,1,2,4,3,0]), //X(X(XX))X
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            PTR(14, false),
            PTR(16, false),
            PTR(16, false),
        ], 
        vec![ // 12 
            COM(5,58,[0,1,4,2,4,3]), //X(XX(XXX))
            COM(5,47,[0,4,1,2,4,3]), //XX(X(XX)X)
            PTR(11, false),
            PTR(10, false),
            PTR(7, false),
        ], 
        vec![ // 13 
            COM(5,28,[0,4,1,2,4,3]), //XXX(XX)X
            PTR(12, false),
            PTR(70, false),
            PTR(73, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN2Adjoxo.cmp
        vec![ // 14 
            COM(5,12,[0,1,4,2,3,0]), //X(XXX)X
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(15, false),
            COM(3,0,[1,0,0,0,0,0]), //X
            COM(3,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 15 
            COM(5,29,[0,1,4,2,4,3]), //X(XX)(XX)X
            COM(5,46,[0,4,1,4,2,3]), //XX(XXXX)
            PRM(EQ,false),
            PRM(LE,false),
            COM(3,0,[2,0,0,0,0,0]), //X
        ], 
         // FUN3NanoPrelude.length
        vec![ // 16 
            Y,
            PTR(18, false),
            INT(0),
        ], 
        vec![ // 17 
            COM(6,49,[0,1,4,2,5,3]), //XX(X(XXX))
            COM(4,6,[3,2,0,1,0,0]), //XX(XX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PRM(ADD,false),
            INT(1),
        ], 
        vec![ // 18 
            COM(3,4,[0,1,2,2,0,0]), //XXXX
            PTR(17, false),
        ], 
         // FUN4Adjoxo.hasLine
        vec![ // 19 
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            PTR(58, false),
            PTR(57, false),
            PTR(53, false),
        ], 
        vec![ // 20 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(7),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 21 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(5),
            PTR(20, false),
        ], 
        vec![ // 22 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(3),
            PTR(21, false),
        ], 
        vec![ // 23 
            PTR(59, false),
            PTR(22, false),
        ], 
        vec![ // 24 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(9),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 25 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(5),
            PTR(24, false),
        ], 
        vec![ // 26 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(1),
            PTR(25, false),
        ], 
        vec![ // 27 
            PTR(59, false),
            PTR(26, false),
        ], 
        vec![ // 28 
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            PTR(58, false),
            PTR(27, false),
            PTR(23, false),
        ], 
        vec![ // 29 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(9),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 30 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(6),
            PTR(29, false),
        ], 
        vec![ // 31 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(3),
            PTR(30, false),
        ], 
        vec![ // 32 
            PTR(59, false),
            PTR(31, false),
        ], 
        vec![ // 33 
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            PTR(58, false),
            PTR(32, false),
            PTR(28, false),
        ], 
        vec![ // 34 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(8),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 35 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(5),
            PTR(34, false),
        ], 
        vec![ // 36 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(2),
            PTR(35, false),
        ], 
        vec![ // 37 
            PTR(59, false),
            PTR(36, false),
        ], 
        vec![ // 38 
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            PTR(58, false),
            PTR(37, false),
            PTR(33, false),
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
            PTR(59, false),
            PTR(41, false),
        ], 
        vec![ // 43 
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            PTR(58, false),
            PTR(42, false),
            PTR(38, false),
        ], 
        vec![ // 44 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(9),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 45 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(8),
            PTR(44, false),
        ], 
        vec![ // 46 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(7),
            PTR(45, false),
        ], 
        vec![ // 47 
            PTR(59, false),
            PTR(46, false),
        ], 
        vec![ // 48 
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            PTR(58, false),
            PTR(47, false),
            PTR(43, false),
        ], 
        vec![ // 49 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(6),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 50 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(5),
            PTR(49, false),
        ], 
        vec![ // 51 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(4),
            PTR(50, false),
        ], 
        vec![ // 52 
            PTR(59, false),
            PTR(51, false),
        ], 
        vec![ // 53 
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            PTR(58, false),
            PTR(52, false),
            PTR(48, false),
        ], 
        vec![ // 54 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(3),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 55 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(2),
            PTR(54, false),
        ], 
        vec![ // 56 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(1),
            PTR(55, false),
        ], 
        vec![ // 57 
            PTR(59, false),
            PTR(56, false),
        ], 
         // FUN5Data.Bool.||
        vec![ // 58 
            COM(3,2,[1,2,0,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
         // FUN6Adjoxo.subset
        vec![ // 59 
            COM(4,6,[0,1,2,3,0,0]), //XX(XX)
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(60, false),
            PTR(62, false),
        ], 
         // FUN7NanoPrelude.null
        vec![ // 60 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(61, false),
        ], 
        vec![ // 61 
            COM(3,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN8Adjoxo.diff
        vec![ // 62 
            COM(4,5,[0,3,1,2,0,0]), //X(XX)X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(69, false),
        ], 
        vec![ // 63 
            COM(6,34,[0,1,2,3,5,4]), //X(XX(XX))X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            COM(4,8,[0,1,2,3,0,0]), //X(X(XX))
        ], 
        vec![ // 64 
            COM(4,10,[0,1,3,2,1,0]), //X(XX)XX
            PTR(63, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(62, false),
        ], 
        vec![ // 65 
            COM(5,34,[0,1,0,2,4,3]), //X(XX(XX))X
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            COM(5,38,[0,1,4,2,3,4]), //X(XX)X(XX)
            PTR(14, false),
            PTR(62, false),
        ], 
        vec![ // 66 
            COM(5,39,[0,1,2,4,3,4]), //XX(XX)(XX)
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
        ], 
        vec![ // 67 
            COM(4,20,[0,1,3,2,3,0]), //X(XX(XX))
            PTR(66, false),
            PTR(65, false),
            PTR(64, false),
        ], 
        vec![ // 68 
            COM(5,48,[0,4,1,2,3,4]), //XX(XX(XX))
            PTR(67, false),
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(62, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 69 
            COM(5,39,[0,3,1,4,2,4]), //XX(XX)(XX)
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(68, false),
        ], 
         // FUN9Adjoxo.report
        vec![ // 70 
            COM(5,10,[0,4,1,2,3,0]), //X(XX)XX
            COM(4,41,[0,1,2,3,1,3]), //X(X(XX))(XX)
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
            COM(5,11,[0,1,2,4,3,0]), //XX(XX)X
            COM(4,11,[0,3,1,3,2,0]), //XX(XX)X
            PTR(19, false),
            PTR(79, false),
            COM(3,0,[1,0,0,0,0,0]), //X
        ], 
        vec![ // 74 
            PTR(98, false),
            INT(1),
            INT(9),
        ], 
        vec![ // 75 
            PTR(62, false),
            PTR(74, false),
        ], 
        vec![ // 76 
            PTR(82, false),
            PTR(86, false),
        ], 
        vec![ // 77 
            COM(5,57,[0,1,2,4,3,4]), //X(X(XX)(XX))
            PTR(76, false),
            PTR(88, false),
        ], 
        vec![ // 78 
            COM(5,45,[0,1,4,2,3,4]), //X(XX)(X(XX))
            PTR(77, false),
            PTR(91, false),
            PTR(62, false),
            PTR(75, false),
        ], 
        vec![ // 79 
            COM(5,29,[0,1,4,2,4,3]), //X(XX)(XX)X
            COM(4,11,[0,3,1,3,2,0]), //XX(XX)X
            PTR(80, false),
            PTR(78, false),
            COM(3,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN13Adjoxo.gridFull
        vec![ // 80 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(81, false),
            INT(9),
        ], 
        vec![ // 81 
            COM(6,31,[0,1,2,3,5,4]), //XX(X(XX))X
            COM(5,13,[0,1,2,4,3,0]), //X(X(XX))X
            PRM(EQ,false),
            PRM(ADD,false),
            PTR(16, false),
            PTR(16, false),
        ], 
         // FUN14NanoPrelude.foldr1
        vec![ // 82 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(85, false),
        ], 
        vec![ // 83 
            COM(5,17,[0,1,2,3,4,0]), //XX(X(XX))
            COM(3,1,[0,1,0,0,0,0]), //XX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 84 
            COM(5,33,[0,1,2,4,3,4]), //X(X(XX)X)X
            COM(3,6,[2,1,0,2,0,0]), //XX(XX)
            PTR(83, false),
        ], 
        vec![ // 85 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            ERR(0),
            PTR(84, false),
        ], 
         // FUN15Adjoxo.bestOf
        vec![ // 86 
            COM(4,4,[0,2,3,1,0,0]), //XXXX
            PTR(87, false),
            COM(3,0,[2,0,0,0,0,0]), //X
        ], 
        vec![ // 87 
            COM(4,32,[2,3,0,0,1,3]), //X(XXXX)X
            COM(3,0,[0,0,0,0,0,0]), //X
            COM(3,0,[2,0,0,0,0,0]), //X
        ], 
         // FUN16NanoPrelude.map
        vec![ // 88 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(90, false),
        ], 
        vec![ // 89 
            COM(5,13,[0,1,2,4,3,0]), //X(X(XX))X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 90 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(89, false),
        ], 
         // FUN17Adjoxo.moveval
        vec![ // 91 
            COM(6,25,[0,1,2,5,3,4]), //XX(XX)XX
            COM(5,21,[0,1,2,4,3,0]), //X(X(XXX))
            PTR(92, false),
            PTR(73, false),
            PTR(93, false),
        ], 
         // FUN18Adjoxo.inverse
        vec![ // 92 
            COM(4,4,[3,0,1,2,0,0]), //XXXX
            COM(3,0,[0,0,0,0,0,0]), //X
            COM(3,0,[2,0,0,0,0,0]), //X
            COM(3,0,[1,0,0,0,0,0]), //X
        ], 
         // FUN19Adjoxo.insert
        vec![ // 93 
            COM(4,20,[0,1,3,2,3,0]), //X(XX(XX))
            Y,
            PTR(97, false),
            PTR(96, false),
        ], 
        vec![ // 94 
            COM(5,11,[0,1,2,4,3,0]), //XX(XX)X
            COM(5,29,[0,1,4,2,4,3]), //X(XX)(XX)X
            COM(5,41,[0,1,2,4,3,4]), //X(X(XX))(XX)
            PRM(LE,false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 95 
            COM(5,11,[0,1,2,4,3,0]), //XX(XX)X
            COM(6,24,[0,1,5,2,3,4]), //X(XX)XXX
            COM(5,48,[0,4,1,2,3,4]), //XX(XX(XX))
            PTR(94, false),
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
        ], 
        vec![ // 96 
            COM(4,11,[0,3,1,3,2,0]), //XX(XX)X
            PTR(95, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 97 
            COM(5,16,[0,1,2,4,3,0]), //XX(XXX)
            COM(4,6,[0,1,2,3,0,0]), //XX(XX)
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN20NanoPrelude.enumFromTo
        vec![ // 98 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(100, false),
            PTR(105, false),
        ], 
        vec![ // 99 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PRM(LE,false),
        ], 
        vec![ // 100 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(101, false),
            PTR(99, false),
        ], 
         // FUN21NanoPrelude.takeWhile
        vec![ // 101 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(104, false),
        ], 
        vec![ // 102 
            COM(5,40,[0,3,4,1,2,4]), //X(XXX)(XX)
            COM(4,8,[0,1,2,3,0,0]), //X(X(XX))
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 103 
            COM(4,4,[0,1,3,2,0,0]), //XXXX
            PTR(102, false),
        ], 
        vec![ // 104 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(103, false),
        ], 
         // FUN22NanoPrelude.enumFrom
        vec![ // 105 
            COM(5,49,[0,4,1,2,4,3]), //XX(X(XXX))
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(105, false),
            PRM(ADD,false),
            INT(1),
        ], 
    ]
});