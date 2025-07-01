use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 33
// Apps in this file: 127
// Combinators in this file: 196
#[rustfmt::skip]
pub static CLAUSIFY: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0Clausify.main
        vec![ // 0 
            PTR(16),
            PTR(15),
        ], 
        vec![ // 1 
            COM(5,1,[4,0,0,0,0,0]), //XX
            INT(0),
        ], 
        vec![ // 2 
            COM(5,1,[4,0,0,0,0,0]), //XX
            INT(0),
        ], 
        vec![ // 3 
            PTR(125),
            PTR(2),
            PTR(1),
        ], 
        vec![ // 4 
            COM(5,1,[4,0,0,0,0,0]), //XX
            INT(0),
        ], 
        vec![ // 5 
            PTR(125),
            PTR(4),
            PTR(3),
        ], 
        vec![ // 6 
            COM(5,1,[4,0,0,0,0,0]), //XX
            INT(0),
        ], 
        vec![ // 7 
            COM(5,1,[4,0,0,0,0,0]), //XX
            INT(0),
        ], 
        vec![ // 8 
            PTR(125),
            PTR(7),
            PTR(6),
        ], 
        vec![ // 9 
            COM(5,1,[4,0,0,0,0,0]), //XX
            INT(0),
        ], 
        vec![ // 10 
            PTR(125),
            PTR(9),
            PTR(8),
        ], 
        vec![ // 11 
            PTR(125),
            PTR(10),
            PTR(5),
        ], 
        vec![ // 12 
            PTR(121),
            INT(2),
            PTR(11),
        ], 
        vec![ // 13 
            COM(5,1,[4,0,0,0,0,0]), //XX
            INT(0),
        ], 
        vec![ // 14 
            PTR(21),
            COM(6,2,[2,0,1,0,0,0]), //XXX
            PTR(13),
            PTR(12),
        ], 
        vec![ // 15 
            PTR(23),
            PTR(14),
        ], 
         // FUN1Clausify.display
        vec![ // 16 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(0),
            PTR(17),
        ], 
        vec![ // 17 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            PRM(ADD,false),
            PTR(18),
            PTR(16),
        ], 
         // FUN2Clausify.emitClause
        vec![ // 18 
            COM(2,1,[1,0,0,0,0,0]), //XX
            PTR(19),
        ], 
        vec![ // 19 
            COM(4,15,[0,1,2,1,3,0]), //X(XX)(XX)
            PRM(ADD,false),
            PTR(20),
        ], 
         // FUN3NanoPrelude.sum
        vec![ // 20 
            PTR(21),
            PRM(ADD,false),
            INT(0),
        ], 
         // FUN4NanoPrelude.foldr
        vec![ // 21 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            Y,
            PTR(22),
        ], 
        vec![ // 22 
            COM(5,16,[4,2,0,1,3,0]), //XX(XXX)
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
        ], 
         // FUN5Clausify.clausify
        vec![ // 23 
            COM(4,6,[0,1,2,3,0,0]), //XX(XX)
            COM(1,0,[0,0,0,0,0,0]), //X
            PTR(26),
            PTR(25),
        ], 
        vec![ // 24 
            COM(6,48,[0,1,2,3,4,5]), //XX(XX(XX))
            COM(1,0,[0,0,0,0,0,0]), //X
            PTR(85),
            COM(1,0,[0,0,0,0,0,0]), //X
            PTR(93),
            PTR(112),
        ], 
        vec![ // 25 
            COM(6,48,[0,1,2,3,4,5]), //XX(XX(XX))
            COM(1,0,[0,0,0,0,0,0]), //X
            PTR(56),
            COM(1,0,[0,0,0,0,0,0]), //X
            PTR(62),
            PTR(24),
        ], 
         // FUN6Clausify.uniq
        vec![ // 26 
            PTR(21),
            PTR(28),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 27 
            PTR(29),
            PTR(44),
        ], 
        vec![ // 28 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(27),
            PTR(55),
        ], 
         // FUN7Clausify.union
        vec![ // 29 
            COM(5,46,[0,3,1,2,3,4]), //XX(XXXX)
            PTR(31),
            PTR(30),
        ], 
        vec![ // 30 
            COM(6,58,[0,1,2,3,4,5]), //X(XX(XXX))
            PTR(34),
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(38),
            PTR(39),
        ], 
         // FUN8Data.List_Type.++
        vec![ // 31 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            Y,
            PTR(33),
        ], 
        vec![ // 32 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 33 
            COM(4,6,[3,1,0,2,0,0]), //XX(XX)
            PTR(32),
        ], 
         // FUN9NanoPrelude.filter
        vec![ // 34 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(37),
        ], 
        vec![ // 35 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 36 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(4,30,[0,2,1,2,3,3]), //XX(XXX)X
            COM(5,39,[0,3,1,4,2,4]), //XX(XX)(XX)
            PTR(35),
        ], 
        vec![ // 37 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(36),
        ], 
         // FUN10Data.Bool.not
        vec![ // 38 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN11Clausify.contains
        vec![ // 39 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(42),
        ], 
        vec![ // 40 
            COM(6,40,[0,1,4,3,2,5]), //X(XXX)(XX)
            PTR(43),
        ], 
        vec![ // 41 
            COM(5,47,[3,0,1,2,4,4]), //XX(X(XX)X)
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 42 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            PTR(41),
            PTR(40),
            COM(3,2,[0,2,1,0,0,0]), //XXX
        ], 
         // FUN12Data.Bool.||
        vec![ // 43 
            COM(3,2,[1,2,0,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
         // FUN13Clausify.eqClause
        vec![ // 44 
            COM(3,3,[1,0,2,0,0,0]), //X(XX)
            PTR(47),
        ], 
        vec![ // 45 
            COM(5,18,[0,1,2,3,4,0]), //X(XXXX)
            PTR(48),
            PTR(49),
            PRM(EQ,false),
        ], 
        vec![ // 46 
            COM(5,10,[0,1,4,2,3,0]), //X(XX)XX
            COM(6,46,[0,4,1,2,3,5]), //XX(XXXX)
            PTR(45),
            PTR(49),
            PRM(EQ,false),
        ], 
        vec![ // 47 
            COM(4,7,[1,0,2,3,0,0]), //X(XXX)
            PTR(46),
        ], 
         // FUN14Data.Bool.&&
        vec![ // 48 
            COM(2,1,[1,0,0,0,0,0]), //XX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN15Clausify.eqList
        vec![ // 49 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(54),
        ], 
        vec![ // 50 
            COM(6,40,[0,1,2,4,3,5]), //X(XXX)(XX)
            PTR(48),
        ], 
        vec![ // 51 
            COM(6,48,[3,0,1,4,2,5]), //XX(XX(XX))
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 52 
            COM(3,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 53 
            COM(5,40,[3,4,0,1,2,4]), //X(XXX)(XX)
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(52),
        ], 
        vec![ // 54 
            COM(5,19,[0,1,2,3,4,0]), //X(X(XX)X)
            PTR(53),
            PTR(51),
            PTR(50),
        ], 
         // FUN16Clausify.singleton
        vec![ // 55 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN17Clausify.nonTaut
        vec![ // 56 
            PTR(34),
            PTR(57),
        ], 
         // FUN18Clausify.notTaut
        vec![ // 57 
            COM(2,1,[1,0,0,0,0,0]), //XX
            PTR(58),
        ], 
        vec![ // 58 
            COM(5,18,[0,1,2,3,4,0]), //X(XXXX)
            PTR(59),
            PTR(61),
            PRM(EQ,false),
        ], 
         // FUN19NanoPrelude.null
        vec![ // 59 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(60),
        ], 
        vec![ // 60 
            COM(3,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN20Clausify.inter
        vec![ // 61 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            PTR(34),
            PTR(39),
        ], 
         // FUN21Clausify.clauses
        vec![ // 62 
            PTR(65),
            PTR(64),
        ], 
        vec![ // 63 
            COM(2,2,[1,0,0,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 64 
            PTR(68),
            PTR(63),
        ], 
         // FUN22NanoPrelude.map
        vec![ // 65 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(67),
        ], 
        vec![ // 66 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 67 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(66),
        ], 
         // FUN23Clausify.clause
        vec![ // 68 
            COM(6,54,[4,0,1,5,3,2]), //X(X(XXX)X)
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
            PTR(77),
            PTR(70),
            PTR(69),
        ], 
        vec![ // 69 
            COM(2,2,[1,0,0,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 70 
            COM(5,12,[4,0,3,1,2,0]), //X(XXX)X
            PTR(78),
        ], 
        vec![ // 71 
            COM(5,16,[4,1,0,3,2,0]), //XX(XXX)
            PTR(78),
        ], 
        vec![ // 72 
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            COM(5,37,[4,0,0,1,2,3]), //XXXX(XX)
            COM(3,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 73 
            COM(5,54,[0,0,1,2,3,4]), //X(X(XXX)X)
            PTR(68),
            COM(3,2,[2,0,1,0,0,0]), //XXX
        ], 
        vec![ // 74 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(5,39,[0,1,2,4,3,4]), //XX(XX)(XX)
            COM(3,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 75 
            COM(5,38,[0,1,4,2,3,4]), //X(XX)X(XX)
            COM(5,45,[0,1,4,2,3,4]), //X(XX)(X(XX))
        ], 
        vec![ // 76 
            COM(5,10,[0,1,4,2,3,0]), //X(XX)XX
            PTR(75),
            PTR(74),
            PTR(73),
            PTR(72),
        ], 
        vec![ // 77 
            COM(4,4,[0,2,3,1,0,0]), //XXXX
            PTR(76),
            PTR(71),
        ], 
         // FUN24Clausify.insert
        vec![ // 78 
            COM(4,19,[0,1,2,3,3,0]), //X(X(XX)X)
            Y,
            PTR(84),
            PTR(83),
        ], 
        vec![ // 79 
            COM(4,16,[0,1,0,2,3,0]), //XX(XXX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 80 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 81 
            COM(5,42,[0,1,3,2,3,4]), //XXX(XXX)
            PRM(LE,false),
        ], 
        vec![ // 82 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            PTR(81),
            PTR(80),
        ], 
        vec![ // 83 
            COM(5,40,[0,1,3,4,2,3]), //X(XXX)(XX)
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
            PTR(82),
            PTR(79),
        ], 
        vec![ // 84 
            COM(6,40,[5,0,3,1,2,4]), //X(XXX)(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN25Clausify.split
        vec![ // 85 
            PTR(86),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN26Clausify.spl
        vec![ // 86 
            COM(4,20,[0,1,3,2,3,0]), //X(XX(XX))
            Y,
            PTR(92),
            PTR(87),
        ], 
        vec![ // 87 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(5,1,[4,0,0,0,0,0]), //XX
        ], 
        vec![ // 88 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(5,1,[3,0,0,0,0,0]), //XX
        ], 
        vec![ // 89 
            COM(5,12,[0,1,3,4,2,0]), //X(XXX)X
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(6,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 90 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(86),
        ], 
        vec![ // 91 
            COM(6,24,[5,0,4,1,2,3]), //X(XX)XXX
            PTR(90),
        ], 
        vec![ // 92 
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            PTR(91),
            PTR(89),
            PTR(88),
        ], 
         // FUN27Clausify.disin
        vec![ // 93 
            COM(5,9,[4,0,1,2,3,0]), //XXXXX
            PTR(95),
            PTR(94),
            COM(5,1,[3,0,0,0,0,0]), //XX
            COM(5,1,[4,0,0,0,0,0]), //XX
        ], 
        vec![ // 94 
            COM(4,15,[0,1,2,1,3,0]), //X(XX)(XX)
            PTR(96),
            PTR(93),
        ], 
        vec![ // 95 
            COM(4,15,[0,1,2,1,3,0]), //X(XX)(XX)
            COM(6,2,[2,0,1,0,0,0]), //XXX
            PTR(93),
        ], 
         // FUN28Clausify.din
        vec![ // 96 
            COM(4,14,[0,2,3,1,3,0]), //XXX(XX)
            PTR(103),
            PTR(97),
        ], 
        vec![ // 97 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(104),
            COM(5,1,[4,0,0,0,0,0]), //XX
        ], 
        vec![ // 98 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(104),
            COM(5,1,[3,0,0,0,0,0]), //XX
        ], 
        vec![ // 99 
            COM(5,12,[0,1,3,4,2,0]), //X(XXX)X
            PTR(104),
            COM(6,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 100 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(96),
        ], 
        vec![ // 101 
            COM(6,40,[0,1,4,3,2,5]), //X(XXX)(XX)
            COM(6,2,[2,0,1,0,0,0]), //XXX
            PTR(96),
        ], 
        vec![ // 102 
            COM(4,19,[2,0,1,3,3,0]), //X(X(XX)X)
            PTR(101),
            PTR(100),
        ], 
        vec![ // 103 
            COM(5,10,[0,1,4,2,3,0]), //X(XX)XX
            COM(4,39,[0,3,1,3,2,3]), //XX(XX)(XX)
            PTR(102),
            PTR(99),
            PTR(98),
        ], 
         // FUN29Clausify.din2
        vec![ // 104 
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(111),
            PTR(105),
        ], 
        vec![ // 105 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(6,2,[3,0,1,0,0,0]), //XXX
            COM(5,1,[4,0,0,0,0,0]), //XX
        ], 
        vec![ // 106 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(6,2,[3,0,1,0,0,0]), //XXX
            COM(5,1,[3,0,0,0,0,0]), //XX
        ], 
        vec![ // 107 
            COM(4,16,[0,1,0,2,3,0]), //XX(XXX)
            COM(6,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 108 
            COM(6,40,[0,1,3,4,2,5]), //X(XXX)(XX)
            COM(6,2,[2,0,1,0,0,0]), //XXX
            PTR(96),
        ], 
        vec![ // 109 
            COM(3,5,[0,1,2,2,0,0]), //X(XX)X
            PTR(108),
            PTR(96),
        ], 
        vec![ // 110 
            COM(6,24,[5,0,1,2,3,4]), //X(XX)XXX
            PTR(109),
        ], 
        vec![ // 111 
            COM(4,39,[0,3,1,3,2,3]), //XX(XX)(XX)
            PTR(110),
            PTR(107),
            PTR(106),
        ], 
         // FUN30Clausify.negin
        vec![ // 112 
            COM(5,9,[4,0,1,2,3,0]), //XXXXX
            PTR(120),
            PTR(119),
            PTR(118),
            COM(5,1,[4,0,0,0,0,0]), //XX
        ], 
        vec![ // 113 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            COM(5,1,[3,0,0,0,0,0]), //XX
            COM(5,1,[4,0,0,0,0,0]), //XX
        ], 
        vec![ // 114 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(112),
            COM(5,1,[3,0,0,0,0,0]), //XX
        ], 
        vec![ // 115 
            COM(6,41,[0,1,2,4,3,5]), //X(X(XX))(XX)
            COM(6,2,[2,0,1,0,0,0]), //XXX
            PTR(112),
            COM(5,1,[3,0,0,0,0,0]), //XX
            PTR(114),
        ], 
        vec![ // 116 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(112),
            COM(5,1,[3,0,0,0,0,0]), //XX
        ], 
        vec![ // 117 
            COM(6,41,[0,1,2,4,3,5]), //X(X(XX))(XX)
            COM(6,2,[3,0,1,0,0,0]), //XXX
            PTR(112),
            COM(5,1,[3,0,0,0,0,0]), //XX
            PTR(116),
        ], 
        vec![ // 118 
            COM(5,9,[4,0,1,2,3,0]), //XXXXX
            PTR(117),
            PTR(115),
            PTR(112),
            PTR(113),
        ], 
        vec![ // 119 
            COM(4,15,[0,1,2,1,3,0]), //X(XX)(XX)
            COM(6,2,[3,0,1,0,0,0]), //XXX
            PTR(112),
        ], 
        vec![ // 120 
            COM(4,15,[0,1,2,1,3,0]), //X(XX)(XX)
            COM(6,2,[2,0,1,0,0,0]), //XXX
            PTR(112),
        ], 
         // FUN31NanoPrelude.replicate
        vec![ // 121 
            COM(4,10,[0,1,3,2,3,0]), //X(XX)XX
            PTR(124),
            PTR(123),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 122 
            COM(4,7,[0,1,3,2,0,0]), //X(XXX)
            PTR(121),
            PRM(SUB,false),
            INT(1),
        ], 
        vec![ // 123 
            COM(4,16,[0,3,1,2,3,0]), //XX(XXX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(122),
        ], 
        vec![ // 124 
            COM(6,28,[0,4,1,2,5,3]), //XXX(XX)X
            PRM(LE,false),
            INT(0),
        ], 
         // FUN32Clausify.eqv
        vec![ // 125 
            COM(5,24,[0,1,4,2,3,4]), //X(XX)XXX
            COM(5,47,[0,4,1,2,4,3]), //XX(X(XX)X)
            PTR(126),
            COM(6,2,[3,0,1,0,0,0]), //XXX
            COM(5,1,[3,0,0,0,0,0]), //XX
        ], 
        vec![ // 126 
            COM(5,19,[0,1,2,3,4,0]), //X(X(XX)X)
            COM(6,2,[2,0,1,0,0,0]), //XXX
            COM(6,2,[3,0,1,0,0,0]), //XXX
            COM(5,1,[3,0,0,0,0,0]), //XX
        ], 
    ]
});