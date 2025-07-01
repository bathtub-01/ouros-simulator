use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 38
// Apps in this file: 131
// Combinators in this file: 184
#[rustfmt::skip]
pub static SUMPUZ: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0Sumpuz.main
        vec![ // 0 
            COM(2,4,[0,1,1,1,0,0]), //XXXX
            PTR(10),
            PTR(9),
        ], 
        vec![ // 1 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(2),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 2 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(1),
            PTR(1),
        ], 
        vec![ // 3 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(2),
            PTR(2),
        ], 
        vec![ // 4 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(3),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 5 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(1),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 6 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(2),
            PTR(5),
        ], 
        vec![ // 7 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(1),
            PTR(6),
        ], 
        vec![ // 8 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(0),
            PTR(7),
        ], 
        vec![ // 9 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(8),
            PTR(4),
        ], 
         // FUN1Sumpuz.count
        vec![ // 10 
            COM(5,12,[0,1,3,4,2,0]), //X(XXX)X
            PTR(11),
            PTR(15),
        ], 
         // FUN2Sumpuz.sumMap
        vec![ // 11 
            COM(4,4,[0,2,3,1,0,0]), //XXXX
            PTR(12),
            INT(0),
        ], 
         // FUN3Sumpuz.sumMapAcc
        vec![ // 12 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(14),
        ], 
        vec![ // 13 
            COM(6,47,[2,5,0,1,4,3]), //XX(X(XX)X)
            PRM(ADD,false),
        ], 
        vec![ // 14 
            COM(5,46,[3,4,0,1,2,4]), //XX(XXXX)
            PTR(13),
        ], 
         // FUN4Sumpuz.fx
        vec![ // 15 
            COM(5,12,[0,1,4,3,2,0]), //X(XXX)X
            PTR(11),
            PTR(16),
        ], 
         // FUN5Sumpuz.fy
        vec![ // 16 
            COM(5,12,[0,1,2,4,3,0]), //X(XXX)X
            PTR(11),
            PTR(17),
        ], 
         // FUN6Sumpuz.fz
        vec![ // 17 
            COM(6,23,[0,3,4,5,1,2]), //XXXXXX
            PTR(18),
            INT(0),
            INT(1),
        ], 
         // FUN7Sumpuz.valid
        vec![ // 18 
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(24),
            PTR(20),
        ], 
        vec![ // 19 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(0),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 20 
            COM(5,9,[0,2,3,4,1,0]), //XXXXX
            PTR(32),
            PTR(19),
        ], 
        vec![ // 21 
            COM(5,57,[0,1,2,3,2,4]), //X(X(XX)(XX))
            PTR(25),
            PRM(EQ,false),
            PTR(26),
        ], 
        vec![ // 22 
            COM(5,10,[0,1,4,2,3,0]), //X(XX)XX
            COM(5,48,[0,4,1,2,3,4]), //XX(XX(XX))
            PTR(21),
            COM(1,0,[0,0,0,0,0,0]), //X
            PTR(29),
        ], 
        vec![ // 23 
            COM(5,57,[0,1,2,3,2,4]), //X(X(XX)(XX))
            PTR(25),
            PRM(EQ,false),
            PTR(26),
        ], 
        vec![ // 24 
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            COM(5,47,[0,3,1,2,3,4]), //XX(X(XX)X)
            PTR(23),
            PTR(22),
        ], 
         // FUN8Data.Bool.&&
        vec![ // 25 
            COM(2,1,[1,0,0,0,0,0]), //XX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN9NanoPrelude.length
        vec![ // 26 
            Y,
            PTR(28),
            INT(0),
        ], 
        vec![ // 27 
            COM(5,7,[2,0,3,1,0,0]), //X(XXX)
            PRM(ADD,false),
            INT(1),
        ], 
        vec![ // 28 
            COM(4,16,[3,2,0,1,2,0]), //XX(XXX)
            PTR(27),
        ], 
         // FUN10Sumpuz.isSingleton
        vec![ // 29 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(31),
        ], 
        vec![ // 30 
            COM(3,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 31 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(30),
        ], 
         // FUN11Sumpuz.solutions
        vec![ // 32 
            COM(5,39,[0,3,1,4,2,4]), //XX(XX)(XX)
            PTR(57),
            PTR(46),
            PTR(38),
        ], 
        vec![ // 33 
            COM(6,32,[0,1,5,2,3,4]), //X(XXXX)X
            PTR(123),
            PTR(130),
            INT(1),
            INT(0),
            INT(9),
        ], 
        vec![ // 34 
            COM(6,39,[0,4,1,5,2,3]), //XX(XX)(XX)
            PTR(60),
            PTR(33),
            PTR(59),
        ], 
        vec![ // 35 
            COM(6,52,[0,1,2,5,3,4]), //X(X(XX)XX)
            PTR(123),
            PTR(130),
            PTR(122),
            INT(1),
            INT(0),
        ], 
        vec![ // 36 
            COM(5,44,[0,1,4,2,4,3]), //X(XX)(XXX)
            PTR(60),
            PTR(121),
            PTR(35),
            INT(9),
        ], 
        vec![ // 37 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(96),
            PTR(36),
        ], 
        vec![ // 38 
            COM(6,46,[0,2,1,3,4,5]), //XX(XXXX)
            PTR(37),
            PTR(34),
        ], 
        vec![ // 39 
            COM(4,6,[0,3,1,2,0,0]), //XX(XX)
            PTR(32),
            PTR(122),
        ], 
        vec![ // 40 
            COM(6,38,[0,1,4,5,2,3]), //X(XX)X(XX)
            PTR(102),
            PTR(58),
            PTR(121),
        ], 
        vec![ // 41 
            COM(6,62,[0,1,2,4,3,5]), //X(X(XX(XX)))
            COM(1,0,[0,0,0,0,0,0]), //X
            PTR(96),
        ], 
        vec![ // 42 
            COM(5,18,[0,1,2,3,4,0]), //X(XXXX)
            PTR(41),
            PTR(40),
        ], 
        vec![ // 43 
            COM(6,32,[0,1,2,4,5,3]), //X(XXXX)X
            COM(6,24,[0,1,2,4,5,3]), //X(XX)XXX
            PTR(42),
        ], 
        vec![ // 44 
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(43),
            PTR(39),
        ], 
        vec![ // 45 
            COM(5,48,[1,0,2,4,3,4]), //XX(XX(XX))
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 46 
            COM(5,15,[0,1,4,2,3,0]), //X(XX)(XX)
            COM(5,40,[0,1,2,4,3,4]), //X(XXX)(XX)
            PTR(45),
            PTR(44),
        ], 
        vec![ // 47 
            COM(3,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 48 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(1),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 49 
            COM(5,10,[0,1,4,2,3,0]), //X(XX)XX
            PRM(EQ,false),
            PTR(58),
            INT(1),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 50 
            COM(6,24,[0,1,5,2,3,4]), //X(XX)XXX
            COM(6,56,[0,1,5,2,3,4]), //X(XXX(XX))
            PTR(49),
            PTR(60),
            PTR(48),
            PTR(59),
        ], 
        vec![ // 51 
            COM(5,32,[4,0,2,2,3,1]), //X(XXXX)X
            PTR(50),
            PTR(47),
        ], 
        vec![ // 52 
            COM(5,5,[4,0,2,1,0,0]), //X(XX)X
            PTR(59),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 53 
            COM(5,10,[0,1,4,2,3,0]), //X(XX)XX
            PRM(EQ,false),
            PTR(58),
            INT(0),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 54 
            COM(4,20,[2,0,3,1,3,0]), //X(XX(XX))
            PTR(53),
            PTR(52),
        ], 
        vec![ // 55 
            COM(5,56,[2,0,3,4,1,4]), //X(XXX(XX))
            PTR(54),
            PTR(51),
        ], 
        vec![ // 56 
            COM(5,29,[0,1,4,2,4,3]), //X(XX)(XX)X
            COM(4,48,[0,3,1,3,2,3]), //XX(XX(XX))
        ], 
        vec![ // 57 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(56),
            PTR(55),
        ], 
         // FUN12NanoPrelude.fst
        vec![ // 58 
            COM(2,1,[1,0,0,0,0,0]), //XX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN13NanoPrelude.snd
        vec![ // 59 
            COM(2,1,[1,0,0,0,0,0]), //XX
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
         // FUN14Sumpuz.bindings
        vec![ // 60 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
            PTR(68),
            PTR(62),
        ], 
        vec![ // 61 
            COM(4,2,[3,1,0,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 62 
            COM(6,37,[0,5,3,1,2,4]), //XXXX(XX)
            PTR(93),
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(61),
        ], 
        vec![ // 63 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            PTR(83),
            PTR(92),
        ], 
        vec![ // 64 
            COM(6,44,[0,1,3,2,4,5]), //X(XX)(XXX)
            PTR(78),
            PTR(82),
            PTR(63),
        ], 
        vec![ // 65 
            COM(5,40,[0,1,2,4,3,4]), //X(XXX)(XX)
            PTR(74),
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(77),
        ], 
        vec![ // 66 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            PTR(65),
            PTR(64),
        ], 
        vec![ // 67 
            COM(5,42,[0,2,4,1,3,4]), //XXX(XXX)
            PTR(69),
        ], 
        vec![ // 68 
            COM(3,5,[0,1,2,2,0,0]), //X(XX)X
            PTR(67),
            PTR(66),
        ], 
         // FUN15NanoPrelude.lookup
        vec![ // 69 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(73),
        ], 
        vec![ // 70 
            COM(5,14,[0,1,4,2,3,0]), //XXX(XX)
            PRM(EQ,false),
        ], 
        vec![ // 71 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(6,37,[0,2,3,4,1,5]), //XXXX(XX)
            PTR(70),
            COM(3,1,[2,0,0,0,0,0]), //XX
        ], 
        vec![ // 72 
            COM(5,18,[3,0,1,2,4,0]), //X(XXXX)
            PTR(71),
        ], 
        vec![ // 73 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(72),
        ], 
         // FUN16NanoPrelude.map
        vec![ // 74 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(76),
        ], 
        vec![ // 75 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 76 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(75),
        ], 
         // FUN17Data.List_Type.:
        vec![ // 77 
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
         // FUN18NanoPrelude.zip
        vec![ // 78 
            COM(4,6,[2,0,1,3,0,0]), //XX(XX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(81),
        ], 
        vec![ // 79 
            COM(6,44,[0,1,4,2,3,5]), //X(XX)(XXX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 80 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(79),
            COM(3,2,[2,0,1,0,0,0]), //XXX
            PTR(78),
        ], 
        vec![ // 81 
            COM(5,16,[2,0,1,3,4,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(80),
        ], 
         // FUN19NanoPrelude.repeat
        vec![ // 82 
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(82),
        ], 
         // FUN20Sumpuz.diff
        vec![ // 83 
            PTR(85),
            PTR(84),
        ], 
        vec![ // 84 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(87),
        ], 
         // FUN21NanoPrelude.foldl
        vec![ // 85 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(86),
        ], 
        vec![ // 86 
            COM(5,46,[4,3,0,1,2,3]), //XX(XXXX)
            COM(4,7,[1,0,2,3,0,0]), //X(XXX)
        ], 
         // FUN22Sumpuz.del
        vec![ // 87 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(91),
        ], 
        vec![ // 88 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 89 
            COM(5,28,[0,1,3,2,4,4]), //XXX(XX)X
            PRM(EQ,false),
        ], 
        vec![ // 90 
            COM(5,30,[0,2,1,3,4,4]), //XX(XXX)X
            PTR(89),
            PTR(88),
        ], 
        vec![ // 91 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(90),
        ], 
         // FUN23Sumpuz.rng
        vec![ // 92 
            PTR(74),
            PTR(59),
        ], 
         // FUN24NanoPrelude.elem
        vec![ // 93 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(95),
        ], 
        vec![ // 94 
            COM(6,28,[0,2,4,3,5,1]), //XXX(XX)X
            PRM(EQ,false),
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
        vec![ // 95 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(94),
        ], 
         // FUN25Sumpuz.ofAll
        vec![ // 96 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(98),
        ], 
        vec![ // 97 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            PTR(99),
        ], 
        vec![ // 98 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(97),
        ], 
         // FUN26Data.List_Type.++
        vec![ // 99 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            Y,
            PTR(101),
        ], 
        vec![ // 100 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 101 
            COM(4,6,[3,1,0,2,0,0]), //XX(XX)
            PTR(100),
        ], 
         // FUN27Sumpuz.solns
        vec![ // 102 
            COM(6,52,[0,1,2,4,3,5]), //X(X(XX)XX)
            PTR(112),
            PTR(106),
            PTR(105),
        ], 
        vec![ // 103 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(119),
        ], 
        vec![ // 104 
            COM(5,40,[0,1,4,2,3,4]), //X(XXX)(XX)
            PRM(ADD,false),
            PTR(119),
        ], 
        vec![ // 105 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            PTR(104),
            PTR(103),
        ], 
        vec![ // 106 
            COM(6,54,[0,1,2,4,5,3]), //X(X(XXX)X)
            PTR(114),
            PRM(ADD,false),
        ], 
        vec![ // 107 
            COM(5,5,[4,0,2,1,0,0]), //X(XX)X
            PTR(59),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 108 
            COM(5,11,[0,2,1,4,3,0]), //XX(XX)X
            PTR(60),
            PTR(107),
        ], 
        vec![ // 109 
            COM(5,20,[0,1,3,2,4,0]), //X(XX(XX))
            PTR(96),
            PTR(113),
            PTR(58),
        ], 
        vec![ // 110 
            COM(5,42,[0,2,4,1,3,4]), //XXX(XXX)
            PTR(109),
        ], 
        vec![ // 111 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(110),
            PTR(108),
        ], 
        vec![ // 112 
            COM(5,37,[0,2,3,4,1,4]), //XXXX(XX)
            PTR(111),
        ], 
         // FUN28NanoPrelude.curry
        vec![ // 113 
            COM(4,7,[1,0,2,3,0,0]), //X(XXX)
            COM(3,2,[2,0,1,0,0,0]), //XXX
        ], 
         // FUN29Sumpuz.divMod10
        vec![ // 114 
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(118),
            PTR(115),
        ], 
        vec![ // 115 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(0),
        ], 
        vec![ // 116 
            COM(5,12,[4,0,2,1,3,0]), //X(XXX)X
            PRM(ADD,false),
            INT(1),
        ], 
        vec![ // 117 
            COM(5,12,[0,1,4,2,3,0]), //X(XXX)X
            PTR(114),
            PRM(SUB,false),
            INT(10),
            PTR(116),
        ], 
        vec![ // 118 
            COM(4,14,[0,3,1,2,3,0]), //XXX(XX)
            PRM(LE,false),
            INT(9),
            PTR(117),
        ], 
         // FUN30Sumpuz.img
        vec![ // 119 
            COM(4,7,[0,1,3,2,0,0]), //X(XXX)
            PTR(120),
            PTR(69),
        ], 
         // FUN31NanoPrelude.fromJust
        vec![ // 120 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            ERR(4),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN32NanoPrelude.head
        vec![ // 121 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            ERR(3),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN33NanoPrelude.tail
        vec![ // 122 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
         // FUN34NanoPrelude.enumFromTo
        vec![ // 123 
            COM(5,15,[0,1,4,2,3,0]), //X(XX)(XX)
            PTR(125),
            PTR(124),
            PTR(129),
        ], 
        vec![ // 124 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PRM(LE,false),
        ], 
         // FUN35NanoPrelude.takeWhile
        vec![ // 125 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(128),
        ], 
        vec![ // 126 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 127 
            COM(5,42,[1,3,0,2,3,4]), //XXX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 128 
            COM(6,48,[5,0,1,3,2,4]), //XX(XX(XX))
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(127),
            PTR(126),
        ], 
         // FUN36NanoPrelude.enumFrom
        vec![ // 129 
            COM(5,49,[0,4,1,2,4,3]), //XX(X(XXX))
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(129),
            PRM(ADD,false),
            INT(1),
        ], 
         // FUN37Sumpuz.ifNull
        vec![ // 130 
            COM(4,6,[1,2,0,3,0,0]), //XX(XX)
            COM(3,0,[0,0,0,0,0,0]), //X
        ], 
    ]
});