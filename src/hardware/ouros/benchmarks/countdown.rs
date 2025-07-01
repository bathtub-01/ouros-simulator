use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*;

// Functions in this file: 33
// Apps in this file: 140
// Combinators in this file: 200
#[rustfmt::skip]
pub static COUNTDOWN: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0Countdown.main
        vec![ // 0 
            PTR(5),
            PTR(4),
        ], 
        vec![ // 1 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(10),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 2 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(4),
            PTR(1),
        ], 
        vec![ // 3 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(3),
            PTR(2),
        ], 
        vec![ // 4 
            PTR(8),
            PTR(3),
            INT(70),
        ], 
         // FUN1NanoPrelude.length
        vec![ // 5 
            Y,
            PTR(7),
            INT(0),
        ], 
        vec![ // 6 
            COM(5,7,[2,0,3,1,0,0]), //X(XXX)
            PRM(ADD,false),
            INT(1),
        ], 
        vec![ // 7 
            COM(4,16,[3,2,0,1,2,0]), //XX(XXX)
            PTR(6),
        ], 
         // FUN2Countdown.solutions
        vec![ // 8 
            COM(5,15,[0,1,4,2,3,0]), //X(XX)(XX)
            PTR(9),
            PTR(15),
            PTR(125),
        ], 
         // FUN3Data.List_Type.concatMap
        vec![ // 9 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(11),
        ], 
        vec![ // 10 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            PTR(12),
        ], 
        vec![ // 11 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(10),
        ], 
         // FUN4Data.List_Type.++
        vec![ // 12 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            Y,
            PTR(14),
        ], 
        vec![ // 13 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 14 
            COM(4,6,[3,1,0,2,0,0]), //XX(XX)
            PTR(13),
        ], 
         // FUN5Countdown.solns
        vec![ // 15 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            PTR(16),
            PTR(22),
        ], 
         // FUN6Countdown.preImage
        vec![ // 16 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(21),
        ], 
        vec![ // 17 
            COM(4,6,[0,3,1,2,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 18 
            COM(6,28,[0,5,1,2,3,4]), //XXX(XX)X
            PRM(EQ,false),
        ], 
        vec![ // 19 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            COM(5,42,[0,1,3,2,3,4]), //XXX(XXX)
            PTR(18),
        ], 
        vec![ // 20 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(5,53,[3,0,2,1,2,4]), //X(XX(XX)X)
            PTR(19),
            PTR(17),
        ], 
        vec![ // 21 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(20),
        ], 
         // FUN7Countdown.results
        vec![ // 22 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(27),
        ], 
        vec![ // 23 
            COM(3,5,[2,0,1,1,0,0]), //X(XX)X
            PTR(124),
        ], 
        vec![ // 24 
            COM(6,13,[5,0,1,3,2,0]), //X(X(XX))X
            PTR(23),
            COM(1,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 25 
            COM(6,49,[0,1,2,3,4,5]), //XX(X(XXX))
            PTR(9),
            PTR(30),
            PTR(111),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 26 
            COM(5,30,[0,4,1,2,4,3]), //XX(XXX)X
            PTR(28),
            PTR(25),
        ], 
        vec![ // 27 
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(26),
            PTR(24),
        ], 
         // FUN8NanoPrelude.null
        vec![ // 28 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(29),
        ], 
        vec![ // 29 
            COM(3,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN9Countdown.combinedResults
        vec![ // 30 
            COM(2,1,[1,0,0,0,0,0]), //XX
            PTR(31),
        ], 
        vec![ // 31 
            COM(5,39,[0,1,2,3,2,4]), //XX(XX)(XX)
            PTR(32),
            PTR(36),
            PTR(22),
        ], 
         // FUN10Countdown.concatProdWith
        vec![ // 32 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(35),
        ], 
        vec![ // 33 
            COM(5,19,[0,1,2,4,3,0]), //X(X(XX)X)
            PTR(12),
            PTR(9),
        ], 
        vec![ // 34 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            COM(5,42,[0,2,3,1,4,2]), //XXX(XXX)
            PTR(33),
        ], 
        vec![ // 35 
            COM(6,46,[4,0,1,2,3,5]), //XX(XXXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(34),
        ], 
         // FUN11Countdown.combine
        vec![ // 36 
            COM(3,3,[1,0,2,0,0,0]), //X(XX)
            PTR(43),
        ], 
        vec![ // 37 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(110),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 38 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(109),
            PTR(37),
        ], 
        vec![ // 39 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(108),
            PTR(38),
        ], 
        vec![ // 40 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(107),
            PTR(39),
        ], 
        vec![ // 41 
            COM(6,51,[0,1,2,3,4,5]), //X(XXXXX)
            PTR(9),
            PTR(44),
        ], 
        vec![ // 42 
            COM(6,23,[0,2,3,4,5,1]), //XXXXXX
            PTR(41),
            PTR(40),
        ], 
        vec![ // 43 
            COM(4,7,[1,0,2,3,0,0]), //X(XXX)
            PTR(42),
        ], 
         // FUN12Countdown.combi
        vec![ // 44 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(50),
            PTR(48),
        ], 
        vec![ // 45 
            COM(4,4,[0,3,1,2,0,0]), //XXXX
            PTR(81),
        ], 
        vec![ // 46 
            COM(6,32,[5,0,4,1,2,3]), //X(XXXX)X
            PTR(80),
        ], 
        vec![ // 47 
            COM(6,29,[0,1,4,2,5,3]), //X(XX)(XX)X
            COM(6,33,[5,0,1,3,3,2]), //X(X(XX)X)X
        ], 
        vec![ // 48 
            COM(6,29,[0,1,4,2,5,3]), //X(XX)(XX)X
            PTR(47),
            PTR(46),
            PTR(45),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 49 
            COM(5,9,[0,4,2,3,1,0]), //XXXXX
            PTR(51),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 50 
            COM(5,44,[0,1,3,2,3,4]), //X(XX)(XXX)
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
            PTR(49),
        ], 
         // FUN13Countdown.valid
        vec![ // 51 
            COM(5,18,[2,0,3,1,4,0]), //X(XXXX)
            PTR(63),
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
        vec![ // 52 
            COM(6,54,[5,0,1,3,4,2]), //X(X(XXX)X)
            PRM(EQ,false),
            PTR(65),
            INT(0),
        ], 
        vec![ // 53 
            COM(6,42,[0,4,1,2,3,5]), //XXX(XXX)
            PRM(LT,false),
            INT(1),
        ], 
        vec![ // 54 
            COM(5,21,[4,0,1,2,3,0]), //X(X(XXX))
            PTR(64),
            PRM(LE,false),
        ], 
        vec![ // 55 
            COM(6,37,[0,4,1,2,3,5]), //XXXX(XX)
            PRM(EQ,false),
            INT(3),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 56 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            PTR(55),
            PTR(54),
        ], 
        vec![ // 57 
            COM(5,42,[0,3,1,2,3,4]), //XXX(XXX)
            PRM(LT,false),
            INT(3),
        ], 
        vec![ // 58 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            PTR(57),
            PTR(56),
        ], 
        vec![ // 59 
            COM(5,42,[0,3,1,2,3,4]), //XXX(XXX)
            PRM(LT,false),
            INT(2),
        ], 
        vec![ // 60 
            COM(6,52,[0,1,2,4,3,5]), //X(X(XX)XX)
            PTR(59),
            COM(5,37,[0,2,3,4,4,1]), //XXXX(XX)
            PTR(58),
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
        vec![ // 61 
            COM(5,29,[0,1,4,2,4,3]), //X(XX)(XX)X
            COM(4,48,[0,3,1,3,3,2]), //XX(XX(XX))
        ], 
        vec![ // 62 
            COM(5,29,[0,1,4,2,4,3]), //X(XX)(XX)X
            PTR(61),
        ], 
        vec![ // 63 
            COM(5,45,[0,1,4,2,3,4]), //X(XX)(X(XX))
            PTR(62),
            PTR(60),
            PTR(53),
            PTR(52),
        ], 
         // FUN14Data.Bool.not
        vec![ // 64 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN15NanoPrelude.mod
        vec![ // 65 
            COM(4,4,[0,2,3,1,0,0]), //XXXX
            PTR(66),
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
         // FUN16NanoPrelude.divMod
        vec![ // 66 
            COM(5,19,[0,1,2,4,3,0]), //X(X(XX)X)
            Y,
            COM(4,42,[0,2,3,1,3,3]), //XXX(XXX)
            PTR(79),
            PRM(ADD,false),
        ], 
        vec![ // 67 
            COM(2,2,[0,1,1,0,0,0]), //XXX
            PRM(ADD,false),
        ], 
        vec![ // 68 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PRM(SUB,false),
        ], 
        vec![ // 69 
            COM(6,40,[5,0,4,1,2,3]), //X(XXX)(XX)
            PRM(ADD,false),
            INT(1),
        ], 
        vec![ // 70 
            COM(5,42,[0,2,3,1,3,4]), //XXX(XXX)
            PRM(LE,false),
            COM(3,2,[2,1,0,0,0,0]), //XXX
        ], 
        vec![ // 71 
            COM(5,45,[0,1,4,2,3,4]), //X(XX)(X(XX))
            COM(4,42,[0,3,2,1,3,2]), //XXX(XXX)
            PTR(70),
            PTR(69),
            PTR(68),
        ], 
        vec![ // 72 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            PTR(71),
            PTR(67),
        ], 
        vec![ // 73 
            COM(4,6,[1,3,0,2,0,0]), //XX(XX)
            PTR(72),
        ], 
        vec![ // 74 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            INT(1),
            PRM(SUB,false),
        ], 
        vec![ // 75 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(0),
        ], 
        vec![ // 76 
            COM(5,37,[0,4,3,1,2,4]), //XXXX(XX)
            PRM(LE,false),
        ], 
        vec![ // 77 
            COM(4,29,[0,1,3,2,3,3]), //X(XX)(XX)X
            PTR(76),
            PTR(75),
            PTR(74),
        ], 
        vec![ // 78 
            COM(5,37,[0,4,1,2,3,4]), //XXXX(XX)
            PRM(LE,false),
        ], 
        vec![ // 79 
            COM(5,29,[0,1,4,2,4,3]), //X(XX)(XX)X
            COM(5,44,[0,1,4,2,3,4]), //X(XX)(XXX)
            PTR(78),
            PTR(77),
            PTR(73),
        ], 
         // FUN17Countdown.App
        vec![ // 80 
            COM(6,46,[5,0,1,2,3,4]), //XX(XXXX)
            INT(5),
            COM(4,4,[3,0,1,2,0,0]), //XXXX
        ], 
         // FUN18Countdown.apply
        vec![ // 81 
            COM(5,39,[0,3,1,4,2,4]), //XX(XX)(XX)
            PTR(98),
            PTR(97),
            PTR(87),
        ], 
        vec![ // 82 
            COM(4,7,[3,0,1,2,0,0]), //X(XXX)
            PTR(106),
        ], 
        vec![ // 83 
            COM(6,42,[0,4,1,2,3,5]), //XXX(XXX)
            PRM(LT,false),
            INT(1),
        ], 
        vec![ // 84 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(83),
            PTR(82),
        ], 
        vec![ // 85 
            COM(6,24,[0,1,5,2,3,4]), //X(XX)XXX
            COM(5,49,[0,4,4,1,2,3]), //XX(X(XXX))
        ], 
        vec![ // 86 
            COM(5,24,[0,1,4,2,3,4]), //X(XX)XXX
            PTR(85),
        ], 
        vec![ // 87 
            COM(4,10,[0,1,3,2,3,0]), //X(XX)XX
            PTR(86),
            PTR(84),
            PRM(ADD,false),
        ], 
        vec![ // 88 
            COM(4,7,[3,0,1,2,0,0]), //X(XXX)
            PRM(SUB,false),
        ], 
        vec![ // 89 
            COM(6,37,[0,4,1,2,3,5]), //XXXX(XX)
            PRM(EQ,false),
            INT(3),
            INT(0),
        ], 
        vec![ // 90 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            PTR(89),
            PTR(88),
        ], 
        vec![ // 91 
            COM(5,42,[0,3,1,2,3,4]), //XXX(XXX)
            PRM(LT,false),
            INT(3),
        ], 
        vec![ // 92 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            PTR(91),
            PTR(90),
        ], 
        vec![ // 93 
            COM(6,24,[0,1,5,2,3,4]), //X(XX)XXX
            COM(5,49,[0,4,4,1,2,3]), //XX(X(XXX))
        ], 
        vec![ // 94 
            COM(5,24,[0,1,4,2,3,4]), //X(XX)XXX
            PTR(93),
        ], 
        vec![ // 95 
            COM(4,10,[0,1,3,2,3,0]), //X(XX)XX
            PTR(94),
            PTR(92),
            PTR(99),
        ], 
        vec![ // 96 
            COM(5,42,[0,3,1,2,3,4]), //XXX(XXX)
            PRM(LT,false),
            INT(2),
        ], 
        vec![ // 97 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            PTR(96),
            PTR(95),
        ], 
        vec![ // 98 
            COM(5,57,[1,0,2,4,3,4]), //X(X(XX)(XX))
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
        ], 
         // FUN19Countdown.mul
        vec![ // 99 
            COM(3,5,[0,1,2,2,0,0]), //X(XX)X
            PTR(105),
            PTR(104),
        ], 
        vec![ // 100 
            COM(4,54,[0,1,0,2,2,3]), //X(X(XXX)X)
            PRM(ADD,false),
            PTR(99),
        ], 
        vec![ // 101 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(100),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 102 
            COM(6,24,[0,1,5,2,3,4]), //X(XX)XXX
            COM(5,51,[0,1,4,2,3,2]), //X(XXXXX)
        ], 
        vec![ // 103 
            COM(5,24,[0,1,4,2,3,4]), //X(XX)XXX
            PTR(102),
            PTR(101),
            PRM(EQ,false),
            INT(0),
        ], 
        vec![ // 104 
            COM(5,14,[0,4,1,2,3,0]), //XXX(XX)
            PTR(66),
            INT(2),
            PTR(103),
        ], 
        vec![ // 105 
            COM(5,28,[0,4,1,2,4,3]), //XXX(XX)X
            PRM(EQ,false),
            INT(1),
        ], 
         // FUN20NanoPrelude.div
        vec![ // 106 
            COM(4,4,[0,2,3,1,0,0]), //XXXX
            PTR(66),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN21Countdown.Add
        vec![ // 107 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(0),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN22Countdown.Sub
        vec![ // 108 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(3),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN23Countdown.Mul
        vec![ // 109 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(2),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN24Countdown.Div
        vec![ // 110 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(1),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN25Countdown.split
        vec![ // 111 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(118),
        ], 
        vec![ // 112 
            COM(4,5,[3,0,2,1,0,0]), //X(XX)X
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 113 
            COM(6,41,[0,1,2,4,3,5]), //X(X(XX))(XX)
            PTR(119),
            PTR(122),
            PTR(112),
            PTR(111),
        ], 
        vec![ // 114 
            COM(4,2,[3,1,0,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 115 
            COM(4,5,[3,0,1,2,0,0]), //X(XX)X
            PTR(114),
        ], 
        vec![ // 116 
            COM(5,40,[0,1,2,4,3,4]), //X(XXX)(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(115),
        ], 
        vec![ // 117 
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(116),
            PTR(113),
        ], 
        vec![ // 118 
            COM(5,30,[0,4,1,3,4,2]), //XX(XXX)X
            PTR(28),
            PTR(117),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN26NanoPrelude.map
        vec![ // 119 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(121),
        ], 
        vec![ // 120 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 121 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(120),
        ], 
         // FUN27Countdown.cross
        vec![ // 122 
            COM(3,3,[1,0,2,0,0,0]), //X(XX)
            PTR(123),
        ], 
        vec![ // 123 
            COM(4,7,[1,0,2,3,0,0]), //X(XXX)
            COM(5,15,[4,0,2,1,3,0]), //X(XX)(XX)
        ], 
         // FUN28Countdown.Val
        vec![ // 124 
            COM(4,6,[3,0,1,2,0,0]), //XX(XX)
            INT(4),
            COM(2,1,[1,0,0,0,0,0]), //XX
        ], 
         // FUN29Countdown.choices
        vec![ // 125 
            COM(4,6,[0,1,2,3,0,0]), //XX(XX)
            PTR(9),
            PTR(126),
            PTR(136),
        ], 
         // FUN30Countdown.perms
        vec![ // 126 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            PTR(128),
            PTR(127),
        ], 
        vec![ // 127 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            PTR(9),
            PTR(129),
            PTR(126),
        ], 
        vec![ // 128 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN31Countdown.interleave
        vec![ // 129 
            COM(5,53,[0,1,4,2,4,3]), //X(XX(XX)X)
            Y,
            PTR(135),
            PTR(132),
            PTR(130),
        ], 
        vec![ // 130 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            PTR(119),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 131 
            COM(4,58,[0,0,1,0,2,3]), //X(XX(XXX))
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 132 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
            PTR(131),
        ], 
        vec![ // 133 
            COM(4,2,[3,1,0,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 134 
            COM(5,5,[4,0,2,1,0,0]), //X(XX)X
            PTR(133),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 135 
            COM(6,45,[5,0,1,2,3,4]), //X(XX)(X(XX))
            PTR(134),
        ], 
         // FUN32Countdown.subs
        vec![ // 136 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            PTR(139),
            PTR(138),
        ], 
        vec![ // 137 
            COM(5,47,[0,4,1,2,3,4]), //XX(X(XX)X)
            PTR(12),
            PTR(119),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 138 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            PTR(137),
            PTR(136),
        ], 
        vec![ // 139 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
    ]
});
