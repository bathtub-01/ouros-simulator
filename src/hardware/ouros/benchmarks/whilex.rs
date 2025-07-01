use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 25
// Apps in this file: 149
// Combinators in this file: 184
#[rustfmt::skip]
pub static WHILEX: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0Whilex.main
        vec![ // 0 
            PTR(55),
            PTR(54),
            INT(5),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 1 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(5),
            INT(0),
        ], 
        vec![ // 2 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(1),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 3 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(4),
            INT(0),
        ], 
        vec![ // 4 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(3),
            PTR(2),
        ], 
        vec![ // 5 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(3),
            INT(17),
        ], 
        vec![ // 6 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(5),
            PTR(4),
        ], 
        vec![ // 7 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(2),
            INT(0),
        ], 
        vec![ // 8 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(7),
            PTR(6),
        ], 
        vec![ // 9 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(1),
            INT(0),
        ], 
        vec![ // 10 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(9),
            PTR(8),
        ], 
        vec![ // 11 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(0),
            INT(0),
        ], 
        vec![ // 12 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(11),
            PTR(10),
        ], 
        vec![ // 13 
            COM(5,1,[2,0,0,0,0,0]), //XX
            INT(1),
        ], 
        vec![ // 14 
            COM(5,1,[4,0,0,0,0,0]), //XX
            INT(4),
        ], 
        vec![ // 15 
            COM(6,2,[4,0,1,0,0,0]), //XXX
            PTR(14),
            PTR(13),
        ], 
        vec![ // 16 
            COM(7,2,[2,0,1,0,0,0]), //XXX
            INT(4),
            PTR(15),
        ], 
        vec![ // 17 
            COM(5,1,[2,0,0,0,0,0]), //XX
            INT(1),
        ], 
        vec![ // 18 
            COM(5,1,[4,0,0,0,0,0]), //XX
            INT(5),
        ], 
        vec![ // 19 
            COM(6,2,[2,0,1,0,0,0]), //XXX
            PTR(18),
            PTR(17),
        ], 
        vec![ // 20 
            COM(7,2,[2,0,1,0,0,0]), //XXX
            INT(5),
            PTR(19),
        ], 
        vec![ // 21 
            COM(5,1,[2,0,0,0,0,0]), //XX
            INT(0),
        ], 
        vec![ // 22 
            COM(5,1,[4,0,0,0,0,0]), //XX
            INT(0),
        ], 
        vec![ // 23 
            PTR(147),
            PTR(22),
            PTR(21),
        ], 
        vec![ // 24 
            PTR(145),
            PTR(23),
            PTR(20),
            COM(5,0,[3,0,0,0,0,0]), //X
        ], 
        vec![ // 25 
            COM(5,1,[2,0,0,0,0,0]), //XX
            INT(1),
        ], 
        vec![ // 26 
            COM(5,1,[4,0,0,0,0,0]), //XX
            INT(2),
        ], 
        vec![ // 27 
            COM(6,2,[2,0,1,0,0,0]), //XXX
            PTR(26),
            PTR(25),
        ], 
        vec![ // 28 
            COM(7,2,[2,0,1,0,0,0]), //XXX
            INT(2),
            PTR(27),
        ], 
        vec![ // 29 
            COM(5,1,[4,0,0,0,0,0]), //XX
            INT(1),
        ], 
        vec![ // 30 
            COM(5,1,[4,0,0,0,0,0]), //XX
            INT(0),
        ], 
        vec![ // 31 
            COM(6,2,[4,0,1,0,0,0]), //XXX
            PTR(30),
            PTR(29),
        ], 
        vec![ // 32 
            COM(7,2,[2,0,1,0,0,0]), //XXX
            INT(0),
            PTR(31),
        ], 
        vec![ // 33 
            COM(7,2,[3,0,1,0,0,0]), //XXX
            PTR(32),
            PTR(28),
        ], 
        vec![ // 34 
            COM(5,1,[4,0,0,0,0,0]), //XX
            INT(0),
        ], 
        vec![ // 35 
            COM(5,1,[4,0,0,0,0,0]), //XX
            INT(1),
        ], 
        vec![ // 36 
            PTR(148),
            PTR(35),
            PTR(34),
        ], 
        vec![ // 37 
            COM(7,2,[6,0,1,0,0,0]), //XXX
            PTR(36),
            PTR(33),
        ], 
        vec![ // 38 
            COM(5,1,[4,0,0,0,0,0]), //XX
            INT(4),
        ], 
        vec![ // 39 
            COM(7,2,[2,0,1,0,0,0]), //XXX
            INT(1),
            PTR(38),
        ], 
        vec![ // 40 
            COM(5,1,[4,0,0,0,0,0]), //XX
            INT(3),
        ], 
        vec![ // 41 
            COM(7,2,[2,0,1,0,0,0]), //XXX
            INT(0),
            PTR(40),
        ], 
        vec![ // 42 
            COM(7,2,[3,0,1,0,0,0]), //XXX
            PTR(41),
            PTR(39),
        ], 
        vec![ // 43 
            COM(7,2,[3,0,1,0,0,0]), //XXX
            PTR(42),
            PTR(37),
        ], 
        vec![ // 44 
            COM(7,2,[3,0,1,0,0,0]), //XXX
            PTR(43),
            PTR(24),
        ], 
        vec![ // 45 
            COM(7,2,[3,0,1,0,0,0]), //XXX
            PTR(44),
            PTR(16),
        ], 
        vec![ // 46 
            COM(5,1,[2,0,0,0,0,0]), //XX
            INT(0),
        ], 
        vec![ // 47 
            COM(5,1,[4,0,0,0,0,0]), //XX
            INT(4),
        ], 
        vec![ // 48 
            PTR(147),
            PTR(47),
            PTR(46),
        ], 
        vec![ // 49 
            PTR(146),
            PTR(48),
        ], 
        vec![ // 50 
            COM(7,2,[6,0,1,0,0,0]), //XXX
            PTR(49),
            PTR(45),
        ], 
        vec![ // 51 
            COM(5,1,[4,0,0,0,0,0]), //XX
            INT(3),
        ], 
        vec![ // 52 
            COM(7,2,[2,0,1,0,0,0]), //XXX
            INT(4),
            PTR(51),
        ], 
        vec![ // 53 
            COM(7,2,[3,0,1,0,0,0]), //XXX
            PTR(52),
            PTR(50),
        ], 
        vec![ // 54 
            PTR(59),
            PTR(53),
            PTR(12),
        ], 
         // FUN1Whilex.value
        vec![ // 55 
            COM(5,16,[2,0,1,3,4,0]), //XX(XXX)
            ERR(42),
            PTR(58),
        ], 
        vec![ // 56 
            COM(4,4,[0,3,1,2,0,0]), //XXXX
            PTR(55),
        ], 
        vec![ // 57 
            COM(6,37,[0,4,1,2,3,5]), //XXXX(XX)
            PRM(EQ,false),
        ], 
        vec![ // 58 
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            COM(5,54,[3,0,1,2,4,2]), //X(X(XXX)X)
            PTR(57),
            PTR(56),
        ], 
         // FUN2Whilex.ssos
        vec![ // 59 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            PTR(60),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
         // FUN3Whilex.run
        vec![ // 60 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(1,0,[0,0,0,0,0,0]), //X
            PTR(61),
        ], 
        vec![ // 61 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            PTR(60),
            PTR(62),
        ], 
         // FUN4Whilex.sosstm
        vec![ // 62 
            COM(5,10,[0,1,4,2,3,0]), //X(XX)XX
            COM(4,39,[0,3,1,3,2,3]), //XX(XX)(XX)
            PTR(74),
            COM(3,1,[1,0,0,0,0,0]), //XX
            PTR(66),
        ], 
        vec![ // 63 
            COM(4,16,[0,3,1,2,3,0]), //XX(XXX)
            COM(7,2,[3,0,1,0,0,0]), //XXX
            COM(7,2,[6,0,1,0,0,0]), //XXX
        ], 
        vec![ // 64 
            COM(6,53,[0,1,4,2,5,3]), //X(XX(XX)X)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(145),
        ], 
        vec![ // 65 
            COM(4,10,[0,1,3,2,3,0]), //X(XX)XX
            PTR(64),
            PTR(63),
            COM(5,0,[3,0,0,0,0,0]), //X
        ], 
        vec![ // 66 
            COM(4,4,[0,2,3,1,0,0]), //XXXX
            PTR(65),
        ], 
        vec![ // 67 
            COM(6,42,[0,3,2,1,4,5]), //XXX(XXX)
            PTR(105),
        ], 
        vec![ // 68 
            COM(4,7,[0,1,3,2,0,0]), //X(XXX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(7,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 69 
            COM(5,14,[0,3,2,1,4,0]), //XXX(XX)
            PTR(62),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 70 
            COM(5,37,[0,2,3,4,1,4]), //XXXX(XX)
            PTR(69),
            PTR(68),
        ], 
        vec![ // 71 
            COM(4,4,[0,2,3,1,0,0]), //XXXX
            PTR(91),
            COM(3,1,[1,0,0,0,0,0]), //XX
        ], 
        vec![ // 72 
            COM(5,42,[0,4,2,1,2,3]), //XXX(XXX)
            PTR(75),
            PTR(71),
        ], 
        vec![ // 73 
            COM(4,15,[2,0,3,1,3,0]), //X(XX)(XX)
            PTR(72),
            PTR(70),
        ], 
        vec![ // 74 
            COM(5,10,[0,1,4,2,3,0]), //X(XX)XX
            COM(4,47,[0,3,1,2,3,3]), //XX(X(XX)X)
            PTR(73),
            PTR(67),
            PTR(142),
        ], 
         // FUN5Whilex.aval
        vec![ // 75 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
            PTR(85),
            PTR(76),
        ], 
        vec![ // 76 
            COM(4,4,[0,1,3,2,0,0]), //XXXX
            PTR(55),
        ], 
        vec![ // 77 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(75),
        ], 
        vec![ // 78 
            COM(6,40,[0,1,4,3,2,5]), //X(XXX)(XX)
            PTR(86),
            PTR(75),
        ], 
        vec![ // 79 
            COM(3,5,[0,1,2,2,0,0]), //X(XX)X
            PTR(78),
            PTR(77),
        ], 
        vec![ // 80 
            COM(6,37,[0,2,4,5,1,3]), //XXXX(XX)
            PTR(79),
            PTR(90),
        ], 
        vec![ // 81 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(75),
        ], 
        vec![ // 82 
            COM(6,40,[0,1,4,3,2,5]), //X(XXX)(XX)
            PTR(86),
            PTR(75),
        ], 
        vec![ // 83 
            COM(3,5,[0,1,2,2,0,0]), //X(XX)X
            PTR(82),
            PTR(81),
        ], 
        vec![ // 84 
            COM(6,37,[0,2,4,5,1,3]), //XXXX(XX)
            PTR(83),
            PTR(87),
        ], 
        vec![ // 85 
            COM(5,39,[0,3,1,4,2,4]), //XX(XX)(XX)
            COM(4,38,[0,1,3,3,2,3]), //X(XX)X(XX)
            PTR(84),
            PTR(80),
        ], 
         // FUN6Whilex.seqq
        vec![ // 86 
            COM(4,7,[1,0,2,3,0,0]), //X(XXX)
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
        ], 
         // FUN7Whilex.add
        vec![ // 87 
            COM(5,12,[0,1,3,4,2,0]), //X(XXX)X
            PTR(88),
            PRM(ADD,false),
        ], 
         // FUN8Whilex.int
        vec![ // 88 
            COM(5,38,[0,1,3,4,4,2]), //X(XX)X(XX)
            PTR(89),
            COM(1,0,[0,0,0,0,0,0]), //X
            INT(0),
        ], 
        vec![ // 89 
            COM(4,14,[0,2,1,3,2,0]), //XXX(XX)
            PRM(EQ,false),
            INT(0),
        ], 
         // FUN9Whilex.sub
        vec![ // 90 
            COM(5,12,[0,1,3,4,2,0]), //X(XXX)X
            PTR(88),
            PRM(SUB,false),
        ], 
         // FUN10Whilex.update
        vec![ // 91 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            PTR(103),
            PTR(102),
        ], 
        vec![ // 92 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(104),
        ], 
        vec![ // 93 
            COM(5,28,[0,4,1,2,3,3]), //XXX(XX)X
            PTR(91),
        ], 
        vec![ // 94 
            COM(4,16,[0,2,1,2,3,0]), //XX(XXX)
            PTR(93),
            PTR(92),
        ], 
        vec![ // 95 
            COM(6,28,[0,2,1,3,5,4]), //XXX(XX)X
            PTR(91),
        ], 
        vec![ // 96 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(6,30,[0,4,1,2,5,3]), //XX(XXX)X
            PTR(95),
            PTR(104),
        ], 
        vec![ // 97 
            COM(6,28,[0,4,1,2,5,3]), //XXX(XX)X
            PRM(EQ,false),
        ], 
        vec![ // 98 
            COM(6,44,[0,1,2,3,4,5]), //X(XX)(XXX)
            COM(5,26,[0,1,2,4,3,4]), //X(XXX)XX
            PTR(97),
        ], 
        vec![ // 99 
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(98),
            PTR(96),
        ], 
        vec![ // 100 
            COM(5,38,[0,1,2,4,3,4]), //X(XX)X(XX)
            COM(5,56,[3,0,1,4,2,4]), //X(XXX(XX))
        ], 
        vec![ // 101 
            COM(5,38,[0,1,2,4,3,4]), //X(XX)X(XX)
            PTR(100),
            PTR(99),
        ], 
        vec![ // 102 
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(101),
            PTR(94),
        ], 
        vec![ // 103 
            COM(5,44,[1,3,0,2,3,4]), //X(XX)(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN11Whilex.upd
        vec![ // 104 
            COM(6,54,[2,0,1,3,4,5]), //X(X(XXX)X)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(3,2,[2,0,1,0,0,0]), //XXX
        ], 
         // FUN12Whilex.bval
        vec![ // 105 
            COM(4,14,[0,2,3,1,3,0]), //XXX(XX)
            PTR(134),
            PTR(109),
        ], 
        vec![ // 106 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(105),
        ], 
        vec![ // 107 
            COM(6,40,[0,1,4,3,2,5]), //X(XXX)(XX)
            PTR(86),
            PTR(105),
        ], 
        vec![ // 108 
            COM(3,5,[0,1,2,2,0,0]), //X(XX)X
            PTR(107),
            PTR(106),
        ], 
        vec![ // 109 
            COM(6,37,[0,2,4,5,1,3]), //XXXX(XX)
            PTR(108),
            PTR(140),
        ], 
        vec![ // 110 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(75),
        ], 
        vec![ // 111 
            COM(6,40,[0,1,4,3,2,5]), //X(XXX)(XX)
            PTR(86),
            PTR(75),
        ], 
        vec![ // 112 
            COM(3,5,[0,1,2,2,0,0]), //X(XX)X
            PTR(111),
            PTR(110),
        ], 
        vec![ // 113 
            COM(6,37,[0,2,4,5,1,3]), //XXXX(XX)
            PTR(112),
            PTR(139),
        ], 
        vec![ // 114 
            COM(6,43,[0,4,1,5,3,2]), //XXX(X(XX))
            PRM(LT,false),
            INT(2),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 115 
            COM(5,37,[0,1,3,4,4,2]), //XXXX(XX)
            PTR(114),
        ], 
        vec![ // 116 
            COM(4,16,[0,3,1,2,3,0]), //XX(XXX)
            PTR(115),
            PTR(113),
        ], 
        vec![ // 117 
            COM(5,42,[0,3,1,2,3,4]), //XXX(XXX)
            PRM(LT,false),
            INT(1),
        ], 
        vec![ // 118 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(75),
        ], 
        vec![ // 119 
            COM(6,40,[0,1,4,3,2,5]), //X(XXX)(XX)
            PTR(86),
            PTR(75),
        ], 
        vec![ // 120 
            COM(3,5,[0,1,2,2,0,0]), //X(XX)X
            PTR(119),
            PTR(118),
        ], 
        vec![ // 121 
            COM(6,37,[0,2,4,5,1,3]), //XXXX(XX)
            PTR(120),
            PTR(138),
        ], 
        vec![ // 122 
            COM(5,14,[0,4,2,1,3,0]), //XXX(XX)
            PTR(105),
            PTR(135),
        ], 
        vec![ // 123 
            COM(6,43,[0,4,1,5,3,2]), //XXX(X(XX))
            PRM(LT,false),
            INT(5),
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
        vec![ // 124 
            COM(5,37,[0,1,3,4,4,2]), //XXXX(XX)
            PTR(123),
        ], 
        vec![ // 125 
            COM(4,16,[0,3,1,2,3,0]), //XX(XXX)
            PTR(124),
            PTR(122),
        ], 
        vec![ // 126 
            COM(5,42,[0,3,1,2,3,4]), //XXX(XXX)
            PRM(LT,false),
            INT(4),
        ], 
        vec![ // 127 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            PTR(126),
            PTR(125),
        ], 
        vec![ // 128 
            COM(5,38,[0,1,2,4,3,4]), //X(XX)X(XX)
            COM(5,37,[0,1,3,4,4,2]), //XXXX(XX)
            PTR(127),
        ], 
        vec![ // 129 
            COM(5,42,[0,3,1,2,3,4]), //XXX(XXX)
            PRM(LT,false),
            INT(3),
        ], 
        vec![ // 130 
            COM(5,53,[0,1,3,2,3,4]), //X(XX(XX)X)
            PTR(129),
            PTR(128),
            PTR(121),
        ], 
        vec![ // 131 
            COM(5,29,[0,1,4,2,4,3]), //X(XX)(XX)X
            COM(4,48,[0,3,1,3,3,2]), //XX(XX(XX))
        ], 
        vec![ // 132 
            COM(5,45,[0,1,4,2,3,4]), //X(XX)(X(XX))
            PTR(131),
        ], 
        vec![ // 133 
            COM(5,38,[0,1,4,2,3,4]), //X(XX)X(XX)
            PTR(132),
            PTR(130),
            PTR(117),
            PTR(116),
        ], 
        vec![ // 134 
            COM(5,56,[1,0,2,4,3,4]), //X(XXX(XX))
            PTR(133),
        ], 
         // FUN13Whilex.notk
        vec![ // 135 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(136),
            PTR(137),
        ], 
         // FUN14Whilex.bool
        vec![ // 136 
            COM(4,15,[2,3,0,3,1,0]), //X(XX)(XX)
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
         // FUN15Data.Bool.not
        vec![ // 137 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN16Whilex.leq
        vec![ // 138 
            COM(5,12,[0,1,3,4,2,0]), //X(XXX)X
            PTR(136),
            PRM(LE,false),
        ], 
         // FUN17Whilex.eq
        vec![ // 139 
            COM(5,12,[0,1,3,4,2,0]), //X(XXX)X
            PTR(136),
            PRM(EQ,false),
        ], 
         // FUN18Whilex.andk
        vec![ // 140 
            COM(5,12,[0,1,3,4,2,0]), //X(XXX)X
            PTR(136),
            PTR(141),
        ], 
         // FUN19Data.Bool.&&
        vec![ // 141 
            COM(2,1,[1,0,0,0,0,0]), //XX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN20Whilex.cond
        vec![ // 142 
            COM(3,5,[0,1,2,2,0,0]), //X(XX)X
            PTR(144),
            PTR(143),
        ], 
        vec![ // 143 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 144 
            COM(6,40,[5,0,4,2,1,3]), //X(XXX)(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
         // FUN21Whilex.If
        vec![ // 145 
            COM(5,4,[0,1,2,3,0,0]), //XXXX
            COM(7,4,[4,0,1,2,0,0]), //XXXX
        ], 
         // FUN22Whilex.Neg
        vec![ // 146 
            COM(4,6,[3,0,1,2,0,0]), //XX(XX)
            INT(4),
            COM(2,1,[1,0,0,0,0,0]), //XX
        ], 
         // FUN23Whilex.Eq
        vec![ // 147 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            INT(1),
            COM(3,2,[2,0,1,0,0,0]), //XXX
        ], 
         // FUN24Whilex.Le
        vec![ // 148 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            INT(3),
            COM(3,2,[2,0,1,0,0,0]), //XXX
        ], 
    ]
});