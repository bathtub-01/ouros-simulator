use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 31
// Apps in this file: 136
// Combinators in this file: 199
#[rustfmt::skip]
pub static WHILEX: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0Whilex.main
        vec![ // 0 
            PTR(55, false),
            PTR(54, false),
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
            PTR(1, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 3 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(4),
            INT(0),
        ], 
        vec![ // 4 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(3, false),
            PTR(2, false),
        ], 
        vec![ // 5 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(3),
            INT(17),
        ], 
        vec![ // 6 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(5, false),
            PTR(4, false),
        ], 
        vec![ // 7 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(2),
            INT(0),
        ], 
        vec![ // 8 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(7, false),
            PTR(6, false),
        ], 
        vec![ // 9 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(1),
            INT(0),
        ], 
        vec![ // 10 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(9, false),
            PTR(8, false),
        ], 
        vec![ // 11 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(0),
            INT(0),
        ], 
        vec![ // 12 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(11, false),
            PTR(10, false),
        ], 
        vec![ // 13 
            PTR(132, false),
            INT(1),
        ], 
        vec![ // 14 
            COM(5,1,[4,0,0,0,0,0]), //XX
            INT(4),
        ], 
        vec![ // 15 
            PTR(134, false),
            PTR(14, false),
            PTR(13, false),
        ], 
        vec![ // 16 
            PTR(128, false),
            INT(4),
            PTR(15, false),
        ], 
        vec![ // 17 
            PTR(132, false),
            INT(1),
        ], 
        vec![ // 18 
            COM(5,1,[4,0,0,0,0,0]), //XX
            INT(5),
        ], 
        vec![ // 19 
            PTR(135, false),
            PTR(18, false),
            PTR(17, false),
        ], 
        vec![ // 20 
            PTR(128, false),
            INT(5),
            PTR(19, false),
        ], 
        vec![ // 21 
            PTR(132, false),
            INT(0),
        ], 
        vec![ // 22 
            COM(5,1,[4,0,0,0,0,0]), //XX
            INT(0),
        ], 
        vec![ // 23 
            PTR(131, false),
            PTR(22, false),
            PTR(21, false),
        ], 
        vec![ // 24 
            PTR(127, false),
            PTR(23, false),
            PTR(20, false),
            COM(5,0,[3,0,0,0,0,0]), //X
        ], 
        vec![ // 25 
            PTR(132, false),
            INT(1),
        ], 
        vec![ // 26 
            COM(5,1,[4,0,0,0,0,0]), //XX
            INT(2),
        ], 
        vec![ // 27 
            PTR(135, false),
            PTR(26, false),
            PTR(25, false),
        ], 
        vec![ // 28 
            PTR(128, false),
            INT(2),
            PTR(27, false),
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
            PTR(134, false),
            PTR(30, false),
            PTR(29, false),
        ], 
        vec![ // 32 
            PTR(128, false),
            INT(0),
            PTR(31, false),
        ], 
        vec![ // 33 
            PTR(93, false),
            PTR(32, false),
            PTR(28, false),
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
            PTR(133, false),
            PTR(35, false),
            PTR(34, false),
        ], 
        vec![ // 37 
            COM(7,2,[6,0,1,0,0,0]), //XXX
            PTR(36, false),
            PTR(33, false),
        ], 
        vec![ // 38 
            COM(5,1,[4,0,0,0,0,0]), //XX
            INT(4),
        ], 
        vec![ // 39 
            PTR(128, false),
            INT(1),
            PTR(38, false),
        ], 
        vec![ // 40 
            COM(5,1,[4,0,0,0,0,0]), //XX
            INT(3),
        ], 
        vec![ // 41 
            PTR(128, false),
            INT(0),
            PTR(40, false),
        ], 
        vec![ // 42 
            PTR(93, false),
            PTR(41, false),
            PTR(39, false),
        ], 
        vec![ // 43 
            PTR(93, false),
            PTR(42, false),
            PTR(37, false),
        ], 
        vec![ // 44 
            PTR(93, false),
            PTR(43, false),
            PTR(24, false),
        ], 
        vec![ // 45 
            PTR(93, false),
            PTR(44, false),
            PTR(16, false),
        ], 
        vec![ // 46 
            PTR(132, false),
            INT(0),
        ], 
        vec![ // 47 
            COM(5,1,[4,0,0,0,0,0]), //XX
            INT(4),
        ], 
        vec![ // 48 
            PTR(131, false),
            PTR(47, false),
            PTR(46, false),
        ], 
        vec![ // 49 
            PTR(130, false),
            PTR(48, false),
        ], 
        vec![ // 50 
            COM(7,2,[6,0,1,0,0,0]), //XXX
            PTR(49, false),
            PTR(45, false),
        ], 
        vec![ // 51 
            COM(5,1,[4,0,0,0,0,0]), //XX
            INT(3),
        ], 
        vec![ // 52 
            PTR(128, false),
            INT(4),
            PTR(51, false),
        ], 
        vec![ // 53 
            PTR(93, false),
            PTR(52, false),
            PTR(50, false),
        ], 
        vec![ // 54 
            PTR(58, false),
            PTR(53, false),
            PTR(12, false),
        ], 
         // FUN1Whilex.value
        vec![ // 55 
            COM(6,25,[0,1,5,2,3,4]), //XX(XX)XX
            COM(5,16,[0,1,2,4,3,0]), //XX(XXX)
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            ERR(42),
            PTR(57, false),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 56 
            COM(6,32,[0,1,5,2,3,4]), //X(XXXX)X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PRM(EQ,false),
        ], 
        vec![ // 57 
            COM(5,52,[0,1,2,4,3,4]), //X(X(XX)XX)
            COM(5,19,[3,0,1,2,4,0]), //X(X(XX)X)
            COM(5,32,[0,1,4,2,3,3]), //X(XXXX)X
            PTR(56, false),
            PTR(55, false),
        ], 
         // FUN2Whilex.ssos
        vec![ // 58 
            COM(4,6,[0,1,2,3,0,0]), //XX(XX)
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(59, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
         // FUN3Whilex.run
        vec![ // 59 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(1,0,[0,0,0,0,0,0]), //X
            PTR(60, false),
        ], 
        vec![ // 60 
            COM(4,6,[0,1,2,3,0,0]), //XX(XX)
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(59, false),
            PTR(61, false),
        ], 
         // FUN4Whilex.sosstm
        vec![ // 61 
            COM(5,10,[0,1,4,2,3,0]), //X(XX)XX
            COM(4,39,[0,3,1,3,2,3]), //XX(XX)(XX)
            PTR(69, false),
            PTR(92, false),
            PTR(63, false),
        ], 
        vec![ // 62 
            COM(5,38,[0,1,4,2,3,4]), //X(XX)X(XX)
            COM(5,34,[0,1,4,2,4,3]), //X(XX(XX))X
            PTR(127, false),
            PTR(93, false),
            COM(7,2,[6,0,1,0,0,0]), //XXX
        ], 
        vec![ // 63 
            COM(6,30,[0,1,2,5,3,4]), //XX(XXX)X
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(62, false),
            COM(5,0,[3,0,0,0,0,0]), //X
        ], 
        vec![ // 64 
            COM(6,30,[0,1,2,5,4,3]), //XX(XXX)X
            COM(4,6,[0,1,2,3,0,0]), //XX(XX)
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(94, false),
        ], 
        vec![ // 65 
            COM(4,7,[0,1,3,2,0,0]), //X(XXX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(93, false),
        ], 
        vec![ // 66 
            COM(6,26,[0,1,5,4,2,3]), //X(XXX)XX
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            PTR(61, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(65, false),
        ], 
        vec![ // 67 
            COM(5,40,[0,1,2,4,3,4]), //X(XXX)(XX)
            COM(4,7,[0,1,3,2,0,0]), //X(XXX)
            COM(4,4,[0,3,1,2,0,0]), //XXXX
            PTR(70, false),
            PTR(82, false),
        ], 
        vec![ // 68 
            COM(5,40,[3,0,4,1,2,4]), //X(XXX)(XX)
            PTR(67, false),
            PTR(92, false),
            PTR(66, false),
        ], 
        vec![ // 69 
            COM(5,10,[0,1,4,2,3,0]), //X(XX)XX
            COM(4,47,[0,3,1,2,3,3]), //XX(X(XX)X)
            PTR(68, false),
            PTR(64, false),
            PTR(125, false),
        ], 
         // FUN5Whilex.aval
        vec![ // 70 
            COM(4,14,[0,2,3,1,3,0]), //XXX(XX)
            PTR(77, false),
            PTR(55, false),
        ], 
        vec![ // 71 
            COM(5,35,[0,1,2,4,3,2]), //X(X(XXX))X
            COM(5,12,[0,1,4,2,3,0]), //X(XXX)X
            PTR(78, false),
            PTR(70, false),
        ], 
        vec![ // 72 
            COM(3,5,[0,1,2,2,0,0]), //X(XX)X
            COM(4,4,[0,3,1,2,0,0]), //XXXX
            PTR(71, false),
        ], 
        vec![ // 73 
            COM(5,35,[0,1,2,4,3,2]), //X(X(XXX))X
            COM(5,12,[0,1,4,2,3,0]), //X(XXX)X
            PTR(78, false),
            PTR(70, false),
        ], 
        vec![ // 74 
            COM(5,47,[0,3,1,2,4,4]), //XX(X(XX)X)
            COM(4,13,[0,1,2,3,3,0]), //X(X(XX))X
            COM(4,4,[0,3,1,2,0,0]), //XXXX
            PTR(73, false),
        ], 
        vec![ // 75 
            COM(5,40,[0,1,4,2,3,4]), //X(XXX)(XX)
            COM(4,17,[0,3,1,2,3,0]), //XX(X(XX))
        ], 
        vec![ // 76 
            COM(5,10,[0,1,4,2,3,0]), //X(XX)XX
            PTR(75, false),
            PTR(74, false),
            PTR(79, false),
            PTR(72, false),
        ], 
        vec![ // 77 
            COM(6,32,[0,1,4,5,2,3]), //X(XXXX)X
            COM(4,16,[0,3,1,2,3,0]), //XX(XXX)
            PTR(76, false),
            PTR(81, false),
            COM(3,2,[0,2,1,0,0,0]), //XXX
        ], 
         // FUN6Whilex.seqq
        vec![ // 78 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
        ], 
         // FUN7Whilex.add
        vec![ // 79 
            COM(5,11,[0,1,2,4,3,0]), //XX(XX)X
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(80, false),
            PRM(ADD,false),
        ], 
         // FUN8Whilex.int
        vec![ // 80 
            COM(5,26,[0,1,4,2,3,4]), //X(XXX)XX
            COM(4,15,[0,3,2,3,1,0]), //X(XX)(XX)
            PRM(EQ,false),
            INT(0),
            INT(0),
        ], 
         // FUN9Whilex.sub
        vec![ // 81 
            COM(5,11,[0,1,2,4,3,0]), //XX(XX)X
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(80, false),
            PRM(SUB,false),
        ], 
         // FUN10Whilex.update
        vec![ // 82 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            PTR(90, false),
            PTR(89, false),
        ], 
        vec![ // 83 
            COM(5,9,[0,4,1,2,3,0]), //XXXXX
            PTR(82, false),
        ], 
        vec![ // 84 
            COM(5,12,[0,1,4,2,3,0]), //X(XXX)X
            COM(6,33,[0,1,2,5,3,4]), //X(X(XX)X)X
            PRM(EQ,false),
        ], 
        vec![ // 85 
            COM(6,35,[0,1,2,5,3,4]), //X(X(XXX))X
            COM(5,25,[0,4,1,4,2,3]), //XX(XX)XX
        ], 
        vec![ // 86 
            COM(4,10,[0,1,3,2,3,0]), //X(XX)XX
            PTR(85, false),
            PTR(84, false),
            PTR(82, false),
        ], 
        vec![ // 87 
            COM(6,31,[0,1,2,3,5,4]), //XX(X(XX))X
            COM(5,47,[0,1,2,3,4,4]), //XX(X(XX)X)
            COM(5,56,[3,0,4,2,1,4]), //X(XXX(XX))
        ], 
        vec![ // 88 
            COM(5,38,[0,1,4,2,3,4]), //X(XX)X(XX)
            PTR(87, false),
            PTR(86, false),
            PTR(91, false),
            PTR(83, false),
        ], 
        vec![ // 89 
            COM(4,42,[0,2,3,1,3,2]), //XXX(XXX)
            PTR(88, false),
            PTR(91, false),
        ], 
        vec![ // 90 
            COM(5,41,[0,2,4,1,3,4]), //X(X(XX))(XX)
            COM(3,7,[0,1,2,2,0,0]), //X(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN11Whilex.upd
        vec![ // 91 
            COM(6,54,[2,0,1,3,4,5]), //X(X(XXX)X)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(3,2,[2,0,1,0,0,0]), //XXX
        ], 
         // FUN12Whilex.Final
        vec![ // 92 
            COM(3,3,[0,2,1,0,0,0]), //X(XX)
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN13Whilex.Comp
        vec![ // 93 
            COM(6,58,[0,1,0,5,2,3]), //X(XX(XXX))
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(3,1,[0,1,0,0,0,0]), //XX
        ], 
         // FUN14Whilex.bval
        vec![ // 94 
            COM(6,24,[0,1,5,2,3,4]), //X(XX)XXX
            COM(5,29,[0,1,4,2,4,3]), //X(XX)(XX)X
            COM(5,59,[0,1,4,2,3,4]), //X(XX(X(XX)))
            PTR(117, false),
            PTR(96, false),
            PTR(123, false),
        ], 
        vec![ // 95 
            COM(5,35,[0,1,2,4,3,2]), //X(X(XXX))X
            COM(5,12,[0,1,4,2,3,0]), //X(XXX)X
            PTR(78, false),
            PTR(94, false),
        ], 
        vec![ // 96 
            COM(4,9,[0,1,3,1,2,0]), //XXXXX
            PTR(95, false),
        ], 
        vec![ // 97 
            COM(5,35,[0,1,2,4,3,2]), //X(X(XXX))X
            COM(5,12,[0,1,4,2,3,0]), //X(XXX)X
            PTR(78, false),
            PTR(70, false),
        ], 
        vec![ // 98 
            COM(4,9,[0,1,3,1,2,0]), //XXXXX
            PTR(97, false),
        ], 
        vec![ // 99 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            PTR(98, false),
            PTR(122, false),
        ], 
        vec![ // 100 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PRM(LT,false),
            INT(2),
        ], 
        vec![ // 101 
            COM(5,40,[0,1,4,2,3,4]), //X(XXX)(XX)
            COM(5,57,[0,1,4,2,4,3]), //X(X(XX)(XX))
            PRM(LT,false),
            INT(1),
            PTR(100, false),
        ], 
        vec![ // 102 
            COM(5,39,[0,1,4,2,3,4]), //XX(XX)(XX)
            COM(4,4,[0,3,1,2,0,0]), //XXXX
            PTR(101, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 103 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(102, false),
            PTR(99, false),
        ], 
        vec![ // 104 
            COM(5,35,[0,1,2,4,3,2]), //X(X(XXX))X
            COM(5,12,[0,1,4,2,3,0]), //X(XXX)X
            PTR(78, false),
            PTR(70, false),
        ], 
        vec![ // 105 
            COM(4,9,[0,1,3,1,2,0]), //XXXXX
            PTR(104, false),
        ], 
        vec![ // 106 
            COM(4,4,[0,3,1,2,0,0]), //XXXX
            PTR(94, false),
        ], 
        vec![ // 107 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            PTR(106, false),
            PTR(118, false),
        ], 
        vec![ // 108 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PRM(LT,false),
            INT(5),
        ], 
        vec![ // 109 
            COM(5,40,[0,1,4,2,3,4]), //X(XXX)(XX)
            COM(5,57,[0,1,4,2,4,3]), //X(X(XX)(XX))
            PRM(LT,false),
            INT(4),
            PTR(108, false),
        ], 
        vec![ // 110 
            COM(5,39,[0,1,4,2,3,4]), //XX(XX)(XX)
            COM(4,4,[0,3,1,2,0,0]), //XXXX
            PTR(109, false),
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
        vec![ // 111 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(110, false),
            PTR(107, false),
        ], 
        vec![ // 112 
            COM(5,40,[0,1,4,2,3,4]), //X(XXX)(XX)
            COM(4,20,[0,1,3,3,2,0]), //X(XX(XX))
            PRM(LT,false),
            INT(3),
        ], 
        vec![ // 113 
            COM(6,49,[0,1,2,3,4,5]), //XX(X(XXX))
            COM(5,40,[0,1,4,2,3,4]), //X(XXX)(XX)
            COM(4,48,[0,3,1,3,3,2]), //XX(XX(XX))
            PTR(112, false),
            PTR(111, false),
        ], 
        vec![ // 114 
            COM(5,43,[0,1,4,2,3,4]), //XXX(X(XX))
            PTR(113, false),
        ], 
        vec![ // 115 
            COM(4,11,[0,3,1,3,2,0]), //XX(XX)X
            PTR(114, false),
            PTR(105, false),
            PTR(121, false),
        ], 
        vec![ // 116 
            COM(5,56,[0,1,2,4,3,4]), //X(XXX(XX))
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(115, false),
        ], 
        vec![ // 117 
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(116, false),
            PTR(103, false),
        ], 
         // FUN15Whilex.notk
        vec![ // 118 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(119, false),
            PTR(120, false),
        ], 
         // FUN16Whilex.bool
        vec![ // 119 
            COM(4,15,[2,3,0,3,1,0]), //X(XX)(XX)
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
         // FUN17Data.Bool.not
        vec![ // 120 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN18Whilex.leq
        vec![ // 121 
            COM(5,11,[0,1,2,4,3,0]), //XX(XX)X
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(119, false),
            PRM(LE,false),
        ], 
         // FUN19Whilex.eq
        vec![ // 122 
            COM(5,11,[0,1,2,4,3,0]), //XX(XX)X
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(119, false),
            PRM(EQ,false),
        ], 
         // FUN20Whilex.andk
        vec![ // 123 
            COM(5,11,[0,1,2,4,3,0]), //XX(XX)X
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(119, false),
            PTR(124, false),
        ], 
         // FUN21Data.Bool.&&
        vec![ // 124 
            COM(2,1,[1,0,0,0,0,0]), //XX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN22Whilex.cond
        vec![ // 125 
            COM(4,16,[0,2,1,3,2,0]), //XX(XXX)
            PTR(126, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 126 
            COM(5,12,[0,1,4,2,3,0]), //X(XXX)X
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
         // FUN23Whilex.If
        vec![ // 127 
            COM(7,23,[0,1,2,3,4,5]), //XXXXXX
            COM(7,46,[0,1,6,2,3,4]), //XX(XXXX)
            COM(3,1,[0,1,0,0,0,0]), //XX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN24Whilex.Ass
        vec![ // 128 
            COM(6,46,[0,1,2,3,4,5]), //XX(XXXX)
            COM(3,1,[0,1,0,0,0,0]), //XX
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(129, false),
        ], 
        vec![ // 129 
            COM(5,16,[0,1,4,2,3,0]), //XX(XXX)
            COM(3,1,[0,1,0,0,0,0]), //XX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN25Whilex.Neg
        vec![ // 130 
            COM(4,6,[3,0,1,2,0,0]), //XX(XX)
            INT(4),
            COM(2,1,[1,0,0,0,0,0]), //XX
        ], 
         // FUN26Whilex.Eq
        vec![ // 131 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            INT(1),
            COM(3,2,[2,0,1,0,0,0]), //XXX
        ], 
         // FUN27Whilex.N
        vec![ // 132 
            COM(5,6,[0,1,4,2,0,0]), //XX(XX)
            COM(3,1,[0,1,0,0,0,0]), //XX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN28Whilex.Le
        vec![ // 133 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            INT(3),
            COM(3,2,[2,0,1,0,0,0]), //XXX
        ], 
         // FUN29Whilex.Sub
        vec![ // 134 
            COM(6,7,[0,5,1,2,0,0]), //X(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN30Whilex.Add
        vec![ // 135 
            COM(5,58,[0,1,0,4,2,3]), //X(XX(XXX))
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(3,1,[0,1,0,0,0,0]), //XX
        ], 
    ]
});