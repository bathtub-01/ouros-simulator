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
            PTR(5),
            PTR(4),
            PTR(2),
        ], 
        vec![ // 1 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(5),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 2 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(2),
            PTR(1),
        ], 
        vec![ // 3 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(4),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 4 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(1),
            PTR(3),
        ], 
         // FUN1Adjoxo.adjudicate
        vec![ // 5 
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
            PTR(15),
            PTR(6),
        ], 
        vec![ // 6 
            COM(5,12,[0,1,4,3,2,0]), //X(XXX)X
            PTR(69),
            PTR(73),
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
        vec![ // 7 
            COM(5,12,[0,1,3,4,2,0]), //X(XXX)X
            PTR(69),
            PTR(73),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 8 
            PTR(69),
            COM(3,0,[2,0,0,0,0,0]), //X
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
        vec![ // 9 
            PTR(69),
            COM(3,0,[2,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 10 
            COM(5,12,[0,1,4,3,2,0]), //X(XXX)X
            PTR(69),
            PTR(73),
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
        vec![ // 11 
            COM(5,30,[0,3,1,3,4,2]), //XX(XXX)X
            PTR(22),
            PTR(10),
            PTR(9),
        ], 
        vec![ // 12 
            COM(5,30,[0,4,1,3,4,2]), //XX(XXX)X
            PTR(22),
            PTR(11),
            PTR(8),
        ], 
        vec![ // 13 
            COM(4,15,[0,1,2,1,3,0]), //X(XX)(XX)
            PTR(16),
            PTR(19),
        ], 
        vec![ // 14 
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            COM(4,39,[0,3,1,3,2,3]), //XX(XX)(XX)
            PTR(13),
            PTR(12),
        ], 
        vec![ // 15 
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(14),
            PTR(7),
        ], 
         // FUN2Adjoxo.cmp
        vec![ // 16 
            COM(4,10,[0,1,3,2,3,0]), //X(XX)XX
            PTR(18),
            PTR(17),
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
            PTR(21),
            INT(0),
        ], 
        vec![ // 20 
            COM(5,7,[2,0,3,1,0,0]), //X(XXX)
            PRM(ADD,false),
            INT(1),
        ], 
        vec![ // 21 
            COM(4,16,[3,2,0,1,2,0]), //XX(XXX)
            PTR(20),
        ], 
         // FUN4Adjoxo.hasLine
        vec![ // 22 
            COM(5,40,[0,1,2,4,3,4]), //X(XXX)(XX)
            PTR(54),
            PTR(55),
            PTR(53),
            PTR(50),
        ], 
        vec![ // 23 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(7),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 24 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(5),
            PTR(23),
        ], 
        vec![ // 25 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(3),
            PTR(24),
        ], 
        vec![ // 26 
            PTR(55),
            PTR(25),
        ], 
        vec![ // 27 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(9),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 28 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(5),
            PTR(27),
        ], 
        vec![ // 29 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(1),
            PTR(28),
        ], 
        vec![ // 30 
            COM(5,40,[0,1,2,4,3,4]), //X(XXX)(XX)
            PTR(54),
            PTR(55),
            PTR(29),
            PTR(26),
        ], 
        vec![ // 31 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(9),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 32 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(6),
            PTR(31),
        ], 
        vec![ // 33 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(3),
            PTR(32),
        ], 
        vec![ // 34 
            COM(5,40,[0,1,2,4,3,4]), //X(XXX)(XX)
            PTR(54),
            PTR(55),
            PTR(33),
            PTR(30),
        ], 
        vec![ // 35 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(8),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 36 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(5),
            PTR(35),
        ], 
        vec![ // 37 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(2),
            PTR(36),
        ], 
        vec![ // 38 
            COM(5,40,[0,1,2,4,3,4]), //X(XXX)(XX)
            PTR(54),
            PTR(55),
            PTR(37),
            PTR(34),
        ], 
        vec![ // 39 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(7),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 40 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(4),
            PTR(39),
        ], 
        vec![ // 41 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(1),
            PTR(40),
        ], 
        vec![ // 42 
            COM(5,40,[0,1,2,4,3,4]), //X(XXX)(XX)
            PTR(54),
            PTR(55),
            PTR(41),
            PTR(38),
        ], 
        vec![ // 43 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(9),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 44 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(8),
            PTR(43),
        ], 
        vec![ // 45 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(7),
            PTR(44),
        ], 
        vec![ // 46 
            COM(5,40,[0,1,2,4,3,4]), //X(XXX)(XX)
            PTR(54),
            PTR(55),
            PTR(45),
            PTR(42),
        ], 
        vec![ // 47 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(6),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 48 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(5),
            PTR(47),
        ], 
        vec![ // 49 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(4),
            PTR(48),
        ], 
        vec![ // 50 
            COM(5,40,[0,1,2,4,3,4]), //X(XXX)(XX)
            PTR(54),
            PTR(55),
            PTR(49),
            PTR(46),
        ], 
        vec![ // 51 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(3),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 52 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(2),
            PTR(51),
        ], 
        vec![ // 53 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(1),
            PTR(52),
        ], 
         // FUN5Data.Bool.||
        vec![ // 54 
            COM(3,2,[1,2,0,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
         // FUN6Adjoxo.subset
        vec![ // 55 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            PTR(56),
            PTR(58),
        ], 
         // FUN7NanoPrelude.null
        vec![ // 56 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(57),
        ], 
        vec![ // 57 
            COM(3,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN8Adjoxo.diff
        vec![ // 58 
            COM(4,6,[2,0,1,3,0,0]), //XX(XX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(68),
        ], 
        vec![ // 59 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            PTR(58),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 60 
            COM(5,16,[0,2,1,3,4,0]), //XX(XXX)
            PTR(58),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 61 
            COM(6,42,[0,2,4,1,3,5]), //XXX(XXX)
            PTR(16),
            PTR(58),
        ], 
        vec![ // 62 
            COM(6,26,[0,1,4,5,2,3]), //X(XXX)XX
            COM(5,48,[0,4,1,2,3,4]), //XX(XX(XX))
        ], 
        vec![ // 63 
            COM(4,10,[0,1,3,2,3,0]), //X(XX)XX
            PTR(62),
            PTR(61),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 64 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
            PTR(63),
            PTR(60),
        ], 
        vec![ // 65 
            COM(5,38,[0,1,2,4,3,4]), //X(XX)X(XX)
            COM(5,37,[0,1,3,4,2,4]), //XXXX(XX)
            PTR(64),
        ], 
        vec![ // 66 
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(65),
            PTR(59),
        ], 
        vec![ // 67 
            COM(5,40,[1,0,3,4,2,4]), //X(XXX)(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 68 
            COM(4,11,[0,2,1,3,3,0]), //XX(XX)X
            PTR(67),
            PTR(66),
        ], 
         // FUN9Adjoxo.report
        vec![ // 69 
            COM(4,14,[0,2,3,1,3,0]), //XXX(XX)
            PTR(70),
            PTR(71),
        ], 
        vec![ // 70 
            COM(5,17,[3,0,1,2,4,0]), //XX(X(XX))
            INT(3),
            PTR(71),
            PTR(72),
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
            PTR(22),
            PTR(79),
            COM(3,0,[1,0,0,0,0,0]), //X
        ], 
        vec![ // 74 
            COM(6,54,[0,1,2,3,4,5]), //X(X(XXX)X)
            PTR(58),
            PTR(58),
            PTR(98),
            INT(1),
            INT(9),
        ], 
        vec![ // 75 
            COM(5,40,[0,1,3,4,2,4]), //X(XXX)(XX)
            PTR(86),
            PTR(89),
        ], 
        vec![ // 76 
            COM(3,5,[0,1,2,2,0,0]), //X(XX)X
            PTR(75),
            PTR(74),
        ], 
        vec![ // 77 
            COM(5,16,[0,1,2,3,4,0]), //XX(XXX)
            PTR(82),
            PTR(84),
            PTR(76),
        ], 
        vec![ // 78 
            COM(5,28,[0,3,4,1,4,2]), //XXX(XX)X
            PTR(80),
        ], 
        vec![ // 79 
            COM(4,10,[0,1,3,2,3,0]), //X(XX)XX
            PTR(78),
            PTR(77),
            COM(3,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN13Adjoxo.gridFull
        vec![ // 80 
            COM(4,4,[0,2,3,1,0,0]), //XXXX
            PTR(81),
            INT(9),
        ], 
        vec![ // 81 
            COM(5,57,[0,1,2,3,2,4]), //X(X(XX)(XX))
            PRM(EQ,false),
            PRM(ADD,false),
            PTR(19),
        ], 
         // FUN14NanoPrelude.foldr1
        vec![ // 82 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(83),
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
            PTR(85),
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
            PTR(88),
        ], 
        vec![ // 87 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 88 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(87),
        ], 
         // FUN17Adjoxo.moveval
        vec![ // 89 
            COM(6,58,[0,1,4,2,5,3]), //X(XX(XXX))
            PTR(90),
            PTR(73),
            PTR(91),
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
            PTR(97),
            PTR(96),
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
            PTR(94),
            PTR(93),
        ], 
        vec![ // 96 
            COM(5,40,[0,1,3,4,2,3]), //X(XXX)(XX)
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
            PTR(95),
            PTR(92),
        ], 
        vec![ // 97 
            COM(6,40,[5,0,3,1,2,4]), //X(XXX)(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN20NanoPrelude.enumFromTo
        vec![ // 98 
            COM(5,15,[0,1,4,2,3,0]), //X(XX)(XX)
            PTR(100),
            PTR(99),
            PTR(104),
        ], 
        vec![ // 99 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PRM(LE,false),
        ], 
         // FUN21NanoPrelude.takeWhile
        vec![ // 100 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(103),
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
            PTR(102),
            PTR(101),
        ], 
         // FUN22NanoPrelude.enumFrom
        vec![ // 104 
            COM(5,49,[0,4,1,2,4,3]), //XX(X(XXX))
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(104),
            PRM(ADD,false),
            INT(1),
        ], 
    ]
});