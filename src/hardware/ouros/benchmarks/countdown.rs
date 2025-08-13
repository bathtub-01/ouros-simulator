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
            PTR(5, false),
            PTR(4, false),
        ], 
        vec![ // 1 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(10),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 2 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(4),
            PTR(1, false),
        ], 
        vec![ // 3 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(3),
            PTR(2, false),
        ], 
        vec![ // 4 
            PTR(8, false),
            PTR(3, false),
            INT(70),
        ], 
         // FUN1NanoPrelude.length
        vec![ // 5 
            Y,
            PTR(7, false),
            INT(0),
        ], 
        vec![ // 6 
            COM(5,7,[2,0,3,1,0,0]), //X(XXX)
            PRM(ADD,false),
            INT(1),
        ], 
        vec![ // 7 
            COM(4,16,[3,2,0,1,2,0]), //XX(XXX)
            PTR(6, false),
        ], 
         // FUN2Countdown.solutions
        vec![ // 8 
            COM(5,15,[0,1,4,2,3,0]), //X(XX)(XX)
            PTR(9, false),
            PTR(15, false),
            PTR(125, false),
        ], 
         // FUN3Data.List_Type.concatMap
        vec![ // 9 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(11, false),
        ], 
        vec![ // 10 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            PTR(12, false),
        ], 
        vec![ // 11 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(10, false),
        ], 
         // FUN4Data.List_Type.++
        vec![ // 12 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            Y,
            PTR(14, false),
        ], 
        vec![ // 13 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 14 
            COM(4,6,[3,1,0,2,0,0]), //XX(XX)
            PTR(13, false),
        ], 
         // FUN5Countdown.solns
        vec![ // 15 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            PTR(16, false),
            PTR(22, false),
        ], 
         // FUN6Countdown.preImage
        vec![ // 16 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(21, false),
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
            PTR(18, false),
        ], 
        vec![ // 20 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(5,53,[3,0,2,1,2,4]), //X(XX(XX)X)
            PTR(19, false),
            PTR(17, false),
        ], 
        vec![ // 21 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(20, false),
        ], 
         // FUN7Countdown.results
        vec![ // 22 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(27, false),
        ], 
        vec![ // 23 
            COM(3,5,[2,0,1,1,0,0]), //X(XX)X
            PTR(124, false),
        ], 
        vec![ // 24 
            COM(6,13,[5,0,1,3,2,0]), //X(X(XX))X
            PTR(23, false),
            COM(1,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 25 
            COM(6,49,[0,1,2,3,4,5]), //XX(X(XXX))
            PTR(9, false),
            PTR(30, false),
            PTR(111, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 26 
            COM(5,30,[0,4,1,2,4,3]), //XX(XXX)X
            PTR(28, false),
            PTR(25, false),
        ], 
        vec![ // 27 
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(26, false),
            PTR(24, false),
        ], 
         // FUN8NanoPrelude.null
        vec![ // 28 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(29, false),
        ], 
        vec![ // 29 
            COM(3,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN9Countdown.combinedResults
        vec![ // 30 
            COM(2,1,[1,0,0,0,0,0]), //XX
            PTR(31, false),
        ], 
        vec![ // 31 
            COM(5,39,[0,1,2,3,2,4]), //XX(XX)(XX)
            PTR(32, false),
            PTR(36, false),
            PTR(22, false),
        ], 
         // FUN10Countdown.concatProdWith
        vec![ // 32 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(35, false),
        ], 
        vec![ // 33 
            COM(5,19,[0,1,2,4,3,0]), //X(X(XX)X)
            PTR(12, false),
            PTR(9, false),
        ], 
        vec![ // 34 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            COM(5,42,[0,2,3,1,4,2]), //XXX(XXX)
            PTR(33, false),
        ], 
        vec![ // 35 
            COM(6,46,[4,0,1,2,3,5]), //XX(XXXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(34, false),
        ], 
         // FUN11Countdown.combine
        vec![ // 36 
            COM(3,3,[1,0,2,0,0,0]), //X(XX)
            PTR(43, false),
        ], 
        vec![ // 37 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(110, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 38 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(109, false),
            PTR(37, false),
        ], 
        vec![ // 39 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(108, false),
            PTR(38, false),
        ], 
        vec![ // 40 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(107, false),
            PTR(39, false),
        ], 
        vec![ // 41 
            COM(6,51,[0,1,2,3,4,5]), //X(XXXXX)
            PTR(9, false),
            PTR(44, false),
        ], 
        vec![ // 42 
            COM(6,23,[0,2,3,4,5,1]), //XXXXXX
            PTR(41, false),
            PTR(40, false),
        ], 
        vec![ // 43 
            COM(4,7,[1,0,2,3,0,0]), //X(XXX)
            PTR(42, false),
        ], 
         // FUN12Countdown.combi
        vec![ // 44 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(50, false),
            PTR(48, false),
        ], 
        vec![ // 45 
            COM(4,4,[0,3,1,2,0,0]), //XXXX
            PTR(81, false),
        ], 
        vec![ // 46 
            COM(6,32,[5,0,4,1,2,3]), //X(XXXX)X
            PTR(80, false),
        ], 
        vec![ // 47 
            COM(6,29,[0,1,4,2,5,3]), //X(XX)(XX)X
            COM(6,33,[5,0,1,3,3,2]), //X(X(XX)X)X
        ], 
        vec![ // 48 
            COM(6,29,[0,1,4,2,5,3]), //X(XX)(XX)X
            PTR(47, false),
            PTR(46, false),
            PTR(45, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 49 
            COM(5,9,[0,4,2,3,1,0]), //XXXXX
            PTR(51, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 50 
            COM(5,44,[0,1,3,2,3,4]), //X(XX)(XXX)
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
            PTR(49, false),
        ], 
         // FUN13Countdown.valid
        vec![ // 51 
            COM(5,18,[2,0,3,1,4,0]), //X(XXXX)
            PTR(63, false),
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
        vec![ // 52 
            COM(6,54,[5,0,1,3,4,2]), //X(X(XXX)X)
            PRM(EQ,false),
            PTR(65, false),
            INT(0),
        ], 
        vec![ // 53 
            COM(6,42,[0,4,1,2,3,5]), //XXX(XXX)
            PRM(LT,false),
            INT(1),
        ], 
        vec![ // 54 
            COM(5,21,[4,0,1,2,3,0]), //X(X(XXX))
            PTR(64, false),
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
            PTR(55, false),
            PTR(54, false),
        ], 
        vec![ // 57 
            COM(5,42,[0,3,1,2,3,4]), //XXX(XXX)
            PRM(LT,false),
            INT(3),
        ], 
        vec![ // 58 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            PTR(57, false),
            PTR(56, false),
        ], 
        vec![ // 59 
            COM(5,42,[0,3,1,2,3,4]), //XXX(XXX)
            PRM(LT,false),
            INT(2),
        ], 
        vec![ // 60 
            COM(6,52,[0,1,2,4,3,5]), //X(X(XX)XX)
            PTR(59, false),
            COM(5,37,[0,2,3,4,4,1]), //XXXX(XX)
            PTR(58, false),
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
        vec![ // 61 
            COM(5,29,[0,1,4,2,4,3]), //X(XX)(XX)X
            COM(4,48,[0,3,1,3,3,2]), //XX(XX(XX))
        ], 
        vec![ // 62 
            COM(5,29,[0,1,4,2,4,3]), //X(XX)(XX)X
            PTR(61, false),
        ], 
        vec![ // 63 
            COM(5,45,[0,1,4,2,3,4]), //X(XX)(X(XX))
            PTR(62, false),
            PTR(60, false),
            PTR(53, false),
            PTR(52, false),
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
            PTR(66, false),
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
         // FUN16NanoPrelude.divMod
        vec![ // 66 
            COM(5,19,[0,1,2,4,3,0]), //X(X(XX)X)
            Y,
            COM(4,42,[0,2,3,1,3,3]), //XXX(XXX)
            PTR(79, false),
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
            PTR(70, false),
            PTR(69, false),
            PTR(68, false),
        ], 
        vec![ // 72 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            PTR(71, false),
            PTR(67, false),
        ], 
        vec![ // 73 
            COM(4,6,[1,3,0,2,0,0]), //XX(XX)
            PTR(72, false),
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
            PTR(76, false),
            PTR(75, false),
            PTR(74, false),
        ], 
        vec![ // 78 
            COM(5,37,[0,4,1,2,3,4]), //XXXX(XX)
            PRM(LE,false),
        ], 
        vec![ // 79 
            COM(5,29,[0,1,4,2,4,3]), //X(XX)(XX)X
            COM(5,44,[0,1,4,2,3,4]), //X(XX)(XXX)
            PTR(78, false),
            PTR(77, false),
            PTR(73, false),
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
            PTR(98, false),
            PTR(97, false),
            PTR(87, false),
        ], 
        vec![ // 82 
            COM(4,7,[3,0,1,2,0,0]), //X(XXX)
            PTR(106, false),
        ], 
        vec![ // 83 
            COM(6,42,[0,4,1,2,3,5]), //XXX(XXX)
            PRM(LT,false),
            INT(1),
        ], 
        vec![ // 84 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(83, false),
            PTR(82, false),
        ], 
        vec![ // 85 
            COM(6,24,[0,1,5,2,3,4]), //X(XX)XXX
            COM(5,49,[0,4,4,1,2,3]), //XX(X(XXX))
        ], 
        vec![ // 86 
            COM(5,24,[0,1,4,2,3,4]), //X(XX)XXX
            PTR(85, false),
        ], 
        vec![ // 87 
            COM(4,10,[0,1,3,2,3,0]), //X(XX)XX
            PTR(86, false),
            PTR(84, false),
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
            PTR(89, false),
            PTR(88, false),
        ], 
        vec![ // 91 
            COM(5,42,[0,3,1,2,3,4]), //XXX(XXX)
            PRM(LT,false),
            INT(3),
        ], 
        vec![ // 92 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            PTR(91, false),
            PTR(90, false),
        ], 
        vec![ // 93 
            COM(6,24,[0,1,5,2,3,4]), //X(XX)XXX
            COM(5,49,[0,4,4,1,2,3]), //XX(X(XXX))
        ], 
        vec![ // 94 
            COM(5,24,[0,1,4,2,3,4]), //X(XX)XXX
            PTR(93, false),
        ], 
        vec![ // 95 
            COM(4,10,[0,1,3,2,3,0]), //X(XX)XX
            PTR(94, false),
            PTR(92, false),
            PTR(99, false),
        ], 
        vec![ // 96 
            COM(5,42,[0,3,1,2,3,4]), //XXX(XXX)
            PRM(LT,false),
            INT(2),
        ], 
        vec![ // 97 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            PTR(96, false),
            PTR(95, false),
        ], 
        vec![ // 98 
            COM(5,57,[1,0,2,4,3,4]), //X(X(XX)(XX))
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
        ], 
         // FUN19Countdown.mul
        vec![ // 99 
            COM(3,5,[0,1,2,2,0,0]), //X(XX)X
            PTR(105, false),
            PTR(104, false),
        ], 
        vec![ // 100 
            COM(4,54,[0,1,0,2,2,3]), //X(X(XXX)X)
            PRM(ADD,false),
            PTR(99, false),
        ], 
        vec![ // 101 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(100, false),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 102 
            COM(6,24,[0,1,5,2,3,4]), //X(XX)XXX
            COM(5,51,[0,1,4,2,3,2]), //X(XXXXX)
        ], 
        vec![ // 103 
            COM(5,24,[0,1,4,2,3,4]), //X(XX)XXX
            PTR(102, false),
            PTR(101, false),
            PRM(EQ,false),
            INT(0),
        ], 
        vec![ // 104 
            COM(5,14,[0,4,1,2,3,0]), //XXX(XX)
            PTR(66, false),
            INT(2),
            PTR(103, false),
        ], 
        vec![ // 105 
            COM(5,28,[0,4,1,2,4,3]), //XXX(XX)X
            PRM(EQ,false),
            INT(1),
        ], 
         // FUN20NanoPrelude.div
        vec![ // 106 
            COM(4,4,[0,2,3,1,0,0]), //XXXX
            PTR(66, false),
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
            PTR(118, false),
        ], 
        vec![ // 112 
            COM(4,5,[3,0,2,1,0,0]), //X(XX)X
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 113 
            COM(6,41,[0,1,2,4,3,5]), //X(X(XX))(XX)
            PTR(119, false),
            PTR(122, false),
            PTR(112, false),
            PTR(111, false),
        ], 
        vec![ // 114 
            COM(4,2,[3,1,0,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 115 
            COM(4,5,[3,0,1,2,0,0]), //X(XX)X
            PTR(114, false),
        ], 
        vec![ // 116 
            COM(5,40,[0,1,2,4,3,4]), //X(XXX)(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(115, false),
        ], 
        vec![ // 117 
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(116, false),
            PTR(113, false),
        ], 
        vec![ // 118 
            COM(5,30,[0,4,1,3,4,2]), //XX(XXX)X
            PTR(28, false),
            PTR(117, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN26NanoPrelude.map
        vec![ // 119 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(121, false),
        ], 
        vec![ // 120 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 121 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(120, false),
        ], 
         // FUN27Countdown.cross
        vec![ // 122 
            COM(3,3,[1,0,2,0,0,0]), //X(XX)
            PTR(123, false),
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
            PTR(9, false),
            PTR(126, false),
            PTR(136, false),
        ], 
         // FUN30Countdown.perms
        vec![ // 126 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            PTR(128, false),
            PTR(127, false),
        ], 
        vec![ // 127 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            PTR(9, false),
            PTR(129, false),
            PTR(126, false),
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
            PTR(135, false),
            PTR(132, false),
            PTR(130, false),
        ], 
        vec![ // 130 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            PTR(119, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 131 
            COM(4,58,[0,0,1,0,2,3]), //X(XX(XXX))
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 132 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
            PTR(131, false),
        ], 
        vec![ // 133 
            COM(4,2,[3,1,0,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 134 
            COM(5,5,[4,0,2,1,0,0]), //X(XX)X
            PTR(133, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 135 
            COM(6,45,[5,0,1,2,3,4]), //X(XX)(X(XX))
            PTR(134, false),
        ], 
         // FUN32Countdown.subs
        vec![ // 136 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            PTR(139, false),
            PTR(138, false),
        ], 
        vec![ // 137 
            COM(5,47,[0,4,1,2,3,4]), //XX(X(XX)X)
            PTR(12, false),
            PTR(119, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 138 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            PTR(137, false),
            PTR(136, false),
        ], 
        vec![ // 139 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
    ]
});