use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 33
// Apps in this file: 120
// Combinators in this file: 227
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
            COM(6,49,[0,1,4,2,5,3]), //XX(X(XXX))
            COM(4,6,[3,2,0,1,0,0]), //XX(XX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PRM(ADD,false),
            INT(1),
        ], 
        vec![ // 7 
            COM(3,4,[0,1,2,2,0,0]), //XXXX
            PTR(6, false),
        ], 
         // FUN2Countdown.solutions
        vec![ // 8 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(9, false),
            PTR(106, false),
        ], 
        vec![ // 9 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(10, false),
            PTR(16, false),
        ], 
         // FUN3Data.List_Type.concatMap
        vec![ // 10 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(12, false),
        ], 
        vec![ // 11 
            COM(5,13,[0,1,2,4,3,0]), //X(X(XX))X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(13, false),
        ], 
        vec![ // 12 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(11, false),
        ], 
         // FUN4Data.List_Type.++
        vec![ // 13 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            Y,
            PTR(15, false),
        ], 
        vec![ // 14 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 15 
            COM(4,6,[3,1,0,2,0,0]), //XX(XX)
            PTR(14, false),
        ], 
         // FUN5Countdown.solns
        vec![ // 16 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(17, false),
            PTR(21, false),
        ], 
         // FUN6Countdown.preImage
        vec![ // 17 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(20, false),
        ], 
        vec![ // 18 
            COM(5,9,[0,4,1,2,3,0]), //XXXXX
            PRM(EQ,false),
        ], 
        vec![ // 19 
            COM(6,34,[0,1,3,4,5,2]), //X(XX(XX))X
            COM(4,7,[0,1,3,2,0,0]), //X(XXX)
            PTR(18, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 20 
            COM(6,47,[5,0,1,2,3,4]), //XX(X(XX)X)
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(4,56,[2,0,1,3,1,3]), //X(XXX(XX))
            PTR(19, false),
        ], 
         // FUN7Countdown.results
        vec![ // 21 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(25, false),
        ], 
        vec![ // 22 
            COM(5,33,[0,1,2,4,4,3]), //X(X(XX)X)X
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(3,2,[2,0,1,0,0,0]), //XXX
            PTR(105, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 23 
            PTR(10, false),
            PTR(28, false),
        ], 
        vec![ // 24 
            COM(5,50,[0,4,1,2,3,4]), //XX(X(X(XX)))
            PTR(26, false),
            PTR(23, false),
            PTR(94, false),
        ], 
        vec![ // 25 
            COM(5,41,[0,1,2,4,3,4]), //X(X(XX))(XX)
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(24, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(22, false),
        ], 
         // FUN8NanoPrelude.null
        vec![ // 26 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(27, false),
        ], 
        vec![ // 27 
            COM(3,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN9Countdown.combinedResults
        vec![ // 28 
            COM(2,1,[1,0,0,0,0,0]), //XX
            PTR(30, false),
        ], 
        vec![ // 29 
            PTR(31, false),
            PTR(35, false),
        ], 
        vec![ // 30 
            COM(4,13,[0,1,2,3,2,0]), //X(X(XX))X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(29, false),
            PTR(21, false),
        ], 
         // FUN10Countdown.concatProdWith
        vec![ // 31 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(34, false),
        ], 
        vec![ // 32 
            COM(6,61,[0,1,2,3,5,4]), //X(X(X(XX)X))
            COM(4,7,[0,1,3,2,0,0]), //X(XXX)
            PTR(13, false),
            PTR(10, false),
        ], 
        vec![ // 33 
            COM(5,23,[0,1,3,4,2,3]), //XXXXXX
            PTR(32, false),
        ], 
        vec![ // 34 
            COM(6,44,[0,5,1,2,3,4]), //X(XX)(XXX)
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(33, false),
        ], 
         // FUN11Countdown.combine
        vec![ // 35 
            COM(3,3,[1,0,2,0,0,0]), //X(XX)
            PTR(42, false),
        ], 
        vec![ // 36 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(93, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 37 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(92, false),
            PTR(36, false),
        ], 
        vec![ // 38 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(91, false),
            PTR(37, false),
        ], 
        vec![ // 39 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(90, false),
            PTR(38, false),
        ], 
        vec![ // 40 
            COM(5,11,[0,1,2,4,3,0]), //XX(XX)X
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(10, false),
        ], 
        vec![ // 41 
            COM(5,19,[1,0,2,4,3,0]), //X(X(XX)X)
            PTR(40, false),
        ], 
        vec![ // 42 
            COM(5,11,[0,3,1,4,2,0]), //XX(XX)X
            PTR(41, false),
            PTR(43, false),
            PTR(39, false),
        ], 
         // FUN12Countdown.combi
        vec![ // 43 
            COM(5,30,[0,4,1,3,4,2]), //XX(XXX)X
            PTR(49, false),
            PTR(45, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 44 
            COM(5,18,[0,1,4,2,3,0]), //X(XXXX)
            COM(3,2,[2,0,1,0,0,0]), //XXX
            PTR(73, false),
        ], 
        vec![ // 45 
            COM(6,26,[0,1,3,5,2,4]), //X(XXX)XX
            COM(5,46,[0,4,1,4,2,3]), //XX(XXXX)
            PTR(44, false),
            PTR(74, false),
        ], 
        vec![ // 46 
            COM(5,9,[0,4,2,3,1,0]), //XXXXX
            PTR(50, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 47 
            COM(5,38,[0,1,4,2,3,4]), //X(XX)X(XX)
            COM(5,47,[0,4,1,2,4,3]), //XX(X(XX)X)
        ], 
        vec![ // 48 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(47, false),
            PTR(46, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 49 
            COM(6,25,[0,1,2,4,5,3]), //XX(XX)XX
            PTR(48, false),
        ], 
         // FUN13Countdown.valid
        vec![ // 50 
            COM(5,48,[0,3,1,4,2,4]), //XX(XX(XX))
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(59, false),
            PTR(53, false),
        ], 
        vec![ // 51 
            COM(6,26,[0,1,5,2,3,4]), //X(XXX)XX
            COM(4,15,[0,3,1,3,2,0]), //X(XX)(XX)
            PRM(LT,false),
            INT(1),
        ], 
        vec![ // 52 
            COM(6,33,[0,1,2,5,3,4]), //X(X(XX)X)X
            PTR(51, false),
            PRM(EQ,false),
        ], 
        vec![ // 53 
            COM(5,10,[0,1,4,2,3,0]), //X(XX)XX
            PTR(52, false),
            PTR(61, false),
            INT(0),
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
        vec![ // 54 
            COM(4,4,[0,3,1,2,0,0]), //XXXX
            PRM(EQ,false),
            INT(3),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 55 
            COM(5,40,[0,1,4,2,3,4]), //X(XXX)(XX)
            COM(5,41,[0,1,4,2,4,3]), //X(X(XX))(XX)
            PRM(LT,false),
            INT(3),
            PTR(54, false),
        ], 
        vec![ // 56 
            COM(6,31,[0,1,2,3,5,4]), //XX(X(XX))X
            COM(4,4,[0,3,1,2,0,0]), //XXXX
            PTR(55, false),
            PTR(60, false),
        ], 
        vec![ // 57 
            COM(5,40,[0,1,4,2,3,4]), //X(XXX)(XX)
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            PRM(LT,false),
            INT(2),
        ], 
        vec![ // 58 
            COM(5,41,[0,1,2,4,3,4]), //X(X(XX))(XX)
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(57, false),
        ], 
        vec![ // 59 
            COM(5,19,[0,1,2,4,3,0]), //X(X(XX)X)
            PTR(58, false),
            PTR(56, false),
            PRM(LE,false),
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
         // FUN14Data.Bool.not
        vec![ // 60 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN15NanoPrelude.mod
        vec![ // 61 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(62, false),
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
         // FUN16NanoPrelude.divMod
        vec![ // 62 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(72, false),
        ], 
        vec![ // 63 
            COM(5,12,[0,1,4,2,3,0]), //X(XXX)X
            COM(3,2,[2,0,1,0,0,0]), //XXX
            PRM(ADD,false),
            INT(1),
        ], 
        vec![ // 64 
            COM(5,53,[0,1,1,2,4,3]), //X(XX(XX)X)
            COM(5,49,[0,4,1,2,4,3]), //XX(X(XXX))
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            PRM(LE,false),
            COM(3,2,[2,1,0,0,0,0]), //XXX
        ], 
        vec![ // 65 
            COM(4,9,[0,3,1,2,3,0]), //XXXXX
            PTR(64, false),
            PTR(63, false),
            PRM(SUB,false),
        ], 
        vec![ // 66 
            COM(5,11,[0,1,2,4,3,0]), //XX(XX)X
            COM(4,16,[0,1,2,3,3,0]), //XX(XXX)
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(65, false),
            PRM(ADD,false),
        ], 
        vec![ // 67 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(66, false),
        ], 
        vec![ // 68 
            COM(4,6,[0,1,2,3,0,0]), //XX(XX)
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(1),
        ], 
        vec![ // 69 
            COM(5,16,[0,1,2,3,4,0]), //XX(XXX)
            COM(5,37,[0,4,3,1,2,4]), //XXXX(XX)
            PRM(LE,false),
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(0),
        ], 
        vec![ // 70 
            COM(4,31,[0,3,1,2,3,3]), //XX(X(XX))X
            PTR(69, false),
            PTR(68, false),
            PRM(SUB,false),
        ], 
        vec![ // 71 
            COM(5,40,[0,1,2,4,3,4]), //X(XXX)(XX)
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            COM(5,37,[0,4,1,2,3,4]), //XXXX(XX)
            PRM(LE,false),
            PTR(70, false),
        ], 
        vec![ // 72 
            COM(6,25,[0,1,2,5,3,4]), //XX(XX)XX
            COM(5,13,[0,1,2,4,3,0]), //X(X(XX))X
            COM(3,16,[0,2,1,2,2,0]), //XX(XXX)
            PTR(71, false),
            PTR(67, false),
            PRM(ADD,false),
        ], 
         // FUN17Countdown.App
        vec![ // 73 
            COM(6,46,[5,0,1,2,3,4]), //XX(XXXX)
            INT(5),
            COM(4,4,[3,0,1,2,0,0]), //XXXX
        ], 
         // FUN18Countdown.apply
        vec![ // 74 
            COM(5,48,[0,3,1,4,2,4]), //XX(XX(XX))
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(82, false),
            PTR(76, false),
        ], 
        vec![ // 75 
            COM(6,26,[0,1,5,2,3,4]), //X(XXX)XX
            COM(4,15,[0,3,1,3,2,0]), //X(XX)(XX)
            PRM(LT,false),
            INT(1),
        ], 
        vec![ // 76 
            COM(5,39,[0,1,2,4,3,4]), //XX(XX)(XX)
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            PTR(75, false),
            PTR(89, false),
            PRM(ADD,false),
        ], 
        vec![ // 77 
            COM(4,4,[0,3,1,2,0,0]), //XXXX
            PRM(EQ,false),
            INT(3),
            INT(0),
        ], 
        vec![ // 78 
            COM(5,40,[0,1,4,2,3,4]), //X(XXX)(XX)
            COM(5,41,[0,1,4,2,4,3]), //X(X(XX))(XX)
            PRM(LT,false),
            INT(3),
            PTR(77, false),
        ], 
        vec![ // 79 
            COM(5,39,[0,1,2,4,3,4]), //XX(XX)(XX)
            COM(4,4,[0,3,1,2,0,0]), //XXXX
            PTR(78, false),
        ], 
        vec![ // 80 
            COM(5,40,[0,1,4,2,3,4]), //X(XXX)(XX)
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            PRM(LT,false),
            INT(2),
        ], 
        vec![ // 81 
            COM(5,41,[0,1,2,4,3,4]), //X(X(XX))(XX)
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(80, false),
        ], 
        vec![ // 82 
            COM(5,57,[0,1,2,4,3,4]), //X(X(XX)(XX))
            PTR(81, false),
            PTR(79, false),
            PRM(SUB,false),
            PTR(83, false),
        ], 
         // FUN19Countdown.mul
        vec![ // 83 
            COM(3,5,[0,1,2,2,0,0]), //X(XX)X
            PTR(88, false),
            PTR(87, false),
        ], 
        vec![ // 84 
            COM(6,27,[0,1,2,5,3,4]), //X(X(XX))XX
            COM(5,51,[0,1,4,2,3,2]), //X(XXXXX)
            PRM(ADD,false),
        ], 
        vec![ // 85 
            COM(5,35,[0,1,2,4,4,3]), //X(X(XXX))X
            PTR(84, false),
            PTR(83, false),
            PRM(ADD,false),
            PRM(EQ,false),
        ], 
        vec![ // 86 
            COM(4,4,[0,3,1,2,0,0]), //XXXX
            PTR(62, false),
            INT(2),
        ], 
        vec![ // 87 
            COM(5,54,[0,1,2,4,3,4]), //X(X(XXX)X)
            PTR(86, false),
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(85, false),
            INT(0),
        ], 
        vec![ // 88 
            COM(5,28,[0,4,1,2,4,3]), //XXX(XX)X
            PRM(EQ,false),
            INT(1),
        ], 
         // FUN20NanoPrelude.div
        vec![ // 89 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(62, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN21Countdown.Add
        vec![ // 90 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(0),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN22Countdown.Sub
        vec![ // 91 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(3),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN23Countdown.Mul
        vec![ // 92 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(2),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN24Countdown.Div
        vec![ // 93 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(1),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN25Countdown.split
        vec![ // 94 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(98, false),
        ], 
        vec![ // 95 
            COM(6,61,[0,1,2,3,5,4]), //X(X(X(XX)X))
            PTR(99, false),
            PTR(102, false),
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 96 
            COM(6,49,[0,1,2,3,4,5]), //XX(X(XXX))
            COM(5,45,[0,1,4,2,3,4]), //X(XX)(X(XX))
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(4,2,[3,1,0,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 97 
            COM(4,11,[0,3,1,3,2,0]), //XX(XX)X
            PTR(96, false),
            PTR(95, false),
            PTR(94, false),
        ], 
        vec![ // 98 
            COM(5,11,[0,1,2,4,3,0]), //XX(XX)X
            COM(4,11,[0,3,1,3,2,0]), //XX(XX)X
            PTR(26, false),
            PTR(97, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN26NanoPrelude.map
        vec![ // 99 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(101, false),
        ], 
        vec![ // 100 
            COM(5,13,[0,1,2,4,3,0]), //X(X(XX))X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 101 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(100, false),
        ], 
         // FUN27Countdown.cross
        vec![ // 102 
            COM(3,3,[1,0,2,0,0,0]), //X(XX)
            PTR(104, false),
        ], 
        vec![ // 103 
            COM(5,11,[0,1,2,4,3,0]), //XX(XX)X
            COM(4,6,[0,1,2,3,0,0]), //XX(XX)
            COM(3,2,[2,0,1,0,0,0]), //XXX
        ], 
        vec![ // 104 
            COM(4,7,[1,0,2,3,0,0]), //X(XXX)
            PTR(103, false),
        ], 
         // FUN28Countdown.Val
        vec![ // 105 
            COM(4,6,[3,0,1,2,0,0]), //XX(XX)
            INT(4),
            COM(2,1,[1,0,0,0,0,0]), //XX
        ], 
         // FUN29Countdown.choices
        vec![ // 106 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(107, false),
            PTR(116, false),
        ], 
        vec![ // 107 
            PTR(10, false),
            PTR(108, false),
        ], 
         // FUN30Countdown.perms
        vec![ // 108 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            PTR(110, false),
            PTR(109, false),
        ], 
        vec![ // 109 
            COM(5,13,[0,1,2,4,3,0]), //X(X(XX))X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(10, false),
            PTR(111, false),
            PTR(108, false),
        ], 
        vec![ // 110 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN31Countdown.interleave
        vec![ // 111 
            COM(4,7,[0,1,3,2,0,0]), //X(XXX)
            Y,
            PTR(115, false),
            PTR(112, false),
        ], 
        vec![ // 112 
            COM(5,13,[0,1,2,4,3,0]), //X(X(XX))X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(99, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 113 
            COM(6,34,[0,1,2,3,5,4]), //X(XX(XX))X
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            COM(5,41,[0,1,2,4,3,4]), //X(X(XX))(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 114 
            COM(5,12,[0,1,2,4,3,0]), //X(XXX)X
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(4,2,[3,1,0,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 115 
            COM(5,39,[0,1,2,4,3,4]), //XX(XX)(XX)
            COM(5,17,[0,1,2,3,4,0]), //XX(X(XX))
            COM(3,2,[2,0,1,0,0,0]), //XXX
            PTR(114, false),
            PTR(113, false),
        ], 
         // FUN32Countdown.subs
        vec![ // 116 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            PTR(119, false),
            PTR(118, false),
        ], 
        vec![ // 117 
            COM(6,59,[0,1,2,3,4,5]), //X(XX(X(XX)))
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(13, false),
            PTR(99, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 118 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(117, false),
            PTR(116, false),
        ], 
        vec![ // 119 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
    ]
});