use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 26
// Apps in this file: 175
// Combinators in this file: 190
#[rustfmt::skip]
pub static SKIABSEVAL: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0SkiAbsEval.main
        vec![ // 0 
            PTR(6, false),
            PTR(5, false),
        ], 
        vec![ // 1 
            PTR(97, false),
            INT(5),
        ], 
        vec![ // 2 
            PTR(97, false),
            INT(1),
        ], 
        vec![ // 3 
            PTR(107, false),
            PTR(149, false),
        ], 
        vec![ // 4 
            PTR(15, false),
            PTR(3, false),
            PTR(2, false),
        ], 
        vec![ // 5 
            PTR(15, false),
            PTR(4, false),
            PTR(1, false),
        ], 
         // FUN1SkiAbsEval.drive
        vec![ // 6 
            COM(2,3,[1,0,1,0,0,0]), //X(XX)
            PTR(8, false),
        ], 
        vec![ // 7 
            COM(6,43,[0,5,1,2,3,4]), //XXX(X(XX))
            PRM(EQ,false),
            INT(10),
            PTR(6, false),
            PTR(9, false),
        ], 
        vec![ // 8 
            COM(5,14,[0,2,3,4,1,0]), //XXX(XX)
            PTR(7, false),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN2SkiAbsEval.eval
        vec![ // 9 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(11, false),
            PTR(10, false),
        ], 
        vec![ // 10 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(16, false),
            PTR(101, false),
        ], 
         // FUN3SkiAbsEval.suture
        vec![ // 11 
            COM(4,6,[3,0,1,2,0,0]), //XX(XX)
            PTR(12, false),
            PTR(13, false),
            PTR(15, false),
        ], 
         // FUN4SkiAbsEval.I
        vec![ // 12 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(5),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN5NanoPrelude.foldl
        vec![ // 13 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(14, false),
        ], 
        vec![ // 14 
            COM(5,46,[4,3,0,1,2,3]), //XX(XXXX)
            COM(4,7,[1,0,2,3,0,0]), //X(XXX)
        ], 
         // FUN6SkiAbsEval.App
        vec![ // 15 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            INT(2),
            COM(3,2,[2,0,1,0,0,0]), //XXX
        ], 
         // FUN7SkiAbsEval.step
        vec![ // 16 
            COM(2,6,[1,1,0,1,0,0]), //XX(XX)
            PTR(92, false),
        ], 
        vec![ // 17 
            COM(6,33,[0,1,2,3,5,4]), //X(X(XX)X)X
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(15, false),
            PTR(9, false),
        ], 
        vec![ // 18 
            COM(5,46,[0,4,1,2,3,4]), //XX(XXXX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(17, false),
        ], 
        vec![ // 19 
            COM(6,57,[0,1,2,3,4,5]), //X(X(XX)(XX))
            PTR(16, false),
            PTR(98, false),
            PTR(101, false),
        ], 
        vec![ // 20 
            COM(5,10,[0,1,4,2,3,0]), //X(XX)XX
            COM(6,40,[0,1,3,5,2,4]), //X(XXX)(XX)
            PTR(19, false),
            PTR(18, false),
            PTR(9, false),
        ], 
        vec![ // 21 
            COM(5,16,[4,1,0,2,3,0]), //XX(XXX)
            PTR(20, false),
        ], 
        vec![ // 22 
            COM(4,16,[3,1,0,1,2,0]), //XX(XXX)
            PTR(21, false),
        ], 
        vec![ // 23 
            COM(4,20,[3,2,1,0,1,0]), //X(XX(XX))
            PTR(22, false),
        ], 
        vec![ // 24 
            COM(6,37,[0,4,1,2,3,5]), //XXXX(XX)
            PRM(EQ,false),
            INT(3),
        ], 
        vec![ // 25 
            COM(4,16,[0,2,1,2,3,0]), //XX(XXX)
            PTR(24, false),
            PTR(23, false),
        ], 
        vec![ // 26 
            COM(6,19,[0,1,2,3,5,0]), //X(X(XX)X)
            PTR(16, false),
            PTR(98, false),
            PTR(101, false),
        ], 
        vec![ // 27 
            COM(4,6,[3,1,0,2,0,0]), //XX(XX)
            PTR(26, false),
        ], 
        vec![ // 28 
            COM(4,20,[3,2,1,0,1,0]), //X(XX(XX))
            PTR(27, false),
        ], 
        vec![ // 29 
            COM(6,42,[0,4,1,2,3,5]), //XXX(XXX)
            PRM(LT,false),
            INT(4),
        ], 
        vec![ // 30 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(29, false),
            PTR(28, false),
        ], 
        vec![ // 31 
            COM(5,19,[0,1,2,3,4,0]), //X(X(XX)X)
            PTR(16, false),
            PTR(98, false),
            PTR(101, false),
        ], 
        vec![ // 32 
            COM(5,57,[0,1,2,3,2,4]), //X(X(XX)(XX))
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(15, false),
            PTR(9, false),
        ], 
        vec![ // 33 
            COM(6,57,[0,1,2,3,4,5]), //X(X(XX)(XX))
            PTR(16, false),
            PTR(98, false),
            PTR(101, false),
        ], 
        vec![ // 34 
            COM(5,16,[0,2,1,3,4,0]), //XX(XXX)
            PTR(33, false),
            PTR(32, false),
        ], 
        vec![ // 35 
            COM(5,16,[4,1,0,2,3,0]), //XX(XXX)
            PTR(34, false),
        ], 
        vec![ // 36 
            COM(4,16,[3,1,0,1,2,0]), //XX(XXX)
            PTR(35, false),
        ], 
        vec![ // 37 
            COM(4,20,[3,2,1,0,1,0]), //X(XX(XX))
            PTR(36, false),
        ], 
        vec![ // 38 
            COM(6,42,[0,4,1,2,3,5]), //XXX(XXX)
            PRM(LT,false),
            INT(6),
        ], 
        vec![ // 39 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(38, false),
            PTR(37, false),
        ], 
        vec![ // 40 
            COM(6,24,[0,1,5,4,2,3]), //X(XX)XXX
            COM(5,49,[0,4,4,1,2,3]), //XX(X(XXX))
        ], 
        vec![ // 41 
            COM(5,24,[0,1,4,3,2,4]), //X(XX)XXX
            PTR(40, false),
        ], 
        vec![ // 42 
            COM(4,10,[0,1,3,2,3,0]), //X(XX)XX
            PTR(41, false),
            PTR(39, false),
            PTR(31, false),
        ], 
        vec![ // 43 
            COM(5,42,[0,3,1,2,3,4]), //XXX(XXX)
            PRM(LT,false),
            INT(5),
        ], 
        vec![ // 44 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            PTR(43, false),
            PTR(42, false),
        ], 
        vec![ // 45 
            COM(5,40,[0,1,2,4,3,4]), //X(XXX)(XX)
            COM(4,48,[0,3,1,3,2,3]), //XX(XX(XX))
        ], 
        vec![ // 46 
            COM(5,38,[0,1,2,4,3,4]), //X(XX)X(XX)
            PTR(45, false),
            PTR(44, false),
        ], 
        vec![ // 47 
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(46, false),
            PTR(30, false),
        ], 
        vec![ // 48 
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
            PTR(47, false),
            PTR(25, false),
        ], 
        vec![ // 49 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(9, false),
        ], 
        vec![ // 50 
            COM(6,44,[0,1,4,2,3,5]), //X(XX)(XXX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(9, false),
            PTR(49, false),
        ], 
        vec![ // 51 
            COM(6,57,[0,1,2,3,4,5]), //X(X(XX)(XX))
            PTR(16, false),
            PTR(98, false),
            PTR(101, false),
        ], 
        vec![ // 52 
            COM(5,16,[0,2,1,3,4,0]), //XX(XXX)
            PTR(51, false),
            PTR(50, false),
        ], 
        vec![ // 53 
            COM(5,16,[4,1,0,2,3,0]), //XX(XXX)
            PTR(52, false),
        ], 
        vec![ // 54 
            COM(4,16,[3,1,0,1,2,0]), //XX(XXX)
            PTR(53, false),
        ], 
        vec![ // 55 
            COM(5,19,[0,1,2,3,4,0]), //X(X(XX)X)
            PTR(16, false),
            PTR(98, false),
            PTR(101, false),
        ], 
        vec![ // 56 
            COM(4,2,[3,1,0,0,0,0]), //XXX
            PTR(55, false),
        ], 
        vec![ // 57 
            COM(4,20,[3,2,1,0,1,0]), //X(XX(XX))
            PTR(56, false),
        ], 
        vec![ // 58 
            COM(6,42,[0,4,1,2,3,5]), //XXX(XXX)
            PRM(LT,false),
            INT(8),
        ], 
        vec![ // 59 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(58, false),
            PTR(57, false),
        ], 
        vec![ // 60 
            COM(6,24,[0,1,5,4,2,3]), //X(XX)XXX
            COM(5,49,[0,4,4,1,2,3]), //XX(X(XXX))
        ], 
        vec![ // 61 
            COM(5,24,[0,1,4,3,2,4]), //X(XX)XXX
            PTR(60, false),
        ], 
        vec![ // 62 
            COM(4,29,[0,1,3,2,3,3]), //X(XX)(XX)X
            PTR(61, false),
            PTR(59, false),
            PTR(54, false),
        ], 
        vec![ // 63 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(15, false),
            PTR(106, false),
        ], 
        vec![ // 64 
            COM(5,44,[0,1,3,2,3,4]), //X(XX)(XXX)
            PTR(98, false),
            PTR(101, false),
            PTR(63, false),
        ], 
        vec![ // 65 
            COM(4,7,[3,2,1,0,0,0]), //X(XXX)
            PTR(64, false),
        ], 
        vec![ // 66 
            COM(6,37,[0,4,1,2,3,5]), //XXXX(XX)
            PRM(EQ,false),
            INT(9),
        ], 
        vec![ // 67 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(93, false),
            PTR(9, false),
        ], 
        vec![ // 68 
            COM(6,41,[0,1,2,4,3,5]), //X(X(XX))(XX)
            PRM(ADD,false),
            PTR(93, false),
            PTR(9, false),
            PTR(67, false),
        ], 
        vec![ // 69 
            COM(6,55,[0,1,2,3,4,5]), //X(X(X(XX))X)
            PTR(16, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(97, false),
        ], 
        vec![ // 70 
            COM(5,17,[4,2,0,1,3,0]), //XX(X(XX))
            PTR(69, false),
            PTR(68, false),
        ], 
        vec![ // 71 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(93, false),
            PTR(9, false),
        ], 
        vec![ // 72 
            COM(6,41,[0,1,2,4,3,5]), //X(X(XX))(XX)
            PRM(EQ,false),
            PTR(93, false),
            PTR(9, false),
            PTR(71, false),
        ], 
        vec![ // 73 
            COM(4,4,[0,2,3,1,0,0]), //XXXX
            PTR(72, false),
            PTR(95, false),
        ], 
        vec![ // 74 
            COM(6,54,[0,1,2,4,3,5]), //X(X(XXX)X)
            PTR(16, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 75 
            COM(6,47,[5,3,0,1,4,2]), //XX(X(XX)X)
            PTR(74, false),
            PTR(73, false),
            PTR(96, false),
        ], 
        vec![ // 76 
            COM(4,20,[3,2,1,0,1,0]), //X(XX(XX))
            PTR(75, false),
        ], 
        vec![ // 77 
            COM(6,42,[0,4,1,2,3,5]), //XXX(XXX)
            PRM(LT,false),
            INT(12),
        ], 
        vec![ // 78 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(77, false),
            PTR(76, false),
        ], 
        vec![ // 79 
            COM(6,24,[0,1,5,4,2,3]), //X(XX)XXX
            COM(5,49,[0,4,4,1,2,3]), //XX(X(XXX))
        ], 
        vec![ // 80 
            COM(5,24,[0,1,4,3,2,4]), //X(XX)XXX
            PTR(79, false),
        ], 
        vec![ // 81 
            COM(4,29,[0,1,3,2,3,3]), //X(XX)(XX)X
            PTR(80, false),
            PTR(78, false),
            PTR(70, false),
        ], 
        vec![ // 82 
            COM(5,42,[0,3,1,2,3,4]), //XXX(XXX)
            PRM(LT,false),
            INT(11),
        ], 
        vec![ // 83 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            PTR(82, false),
            PTR(81, false),
        ], 
        vec![ // 84 
            COM(5,45,[0,1,4,2,3,4]), //X(XX)(X(XX))
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
        ], 
        vec![ // 85 
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            PTR(84, false),
            PTR(83, false),
            PTR(66, false),
        ], 
        vec![ // 86 
            COM(5,42,[0,3,1,2,3,4]), //XXX(XXX)
            PRM(LT,false),
            INT(9),
        ], 
        vec![ // 87 
            COM(5,53,[0,1,3,2,3,4]), //X(XX(XX)X)
            PTR(86, false),
            PTR(85, false),
            PTR(65, false),
        ], 
        vec![ // 88 
            COM(5,40,[0,1,2,4,3,4]), //X(XXX)(XX)
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
            PTR(87, false),
        ], 
        vec![ // 89 
            COM(5,42,[0,3,1,2,3,4]), //XXX(XXX)
            PRM(LT,false),
            INT(7),
        ], 
        vec![ // 90 
            COM(5,53,[0,1,3,2,3,4]), //X(XX(XX)X)
            PTR(89, false),
            PTR(88, false),
            PTR(62, false),
        ], 
        vec![ // 91 
            COM(5,57,[3,0,1,4,2,4]), //X(X(XX)(XX))
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
        ], 
        vec![ // 92 
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            PTR(91, false),
            PTR(90, false),
            PTR(48, false),
        ], 
         // FUN8SkiAbsEval.peek
        vec![ // 93 
            COM(2,1,[1,0,0,0,0,0]), //XX
            PTR(94, false),
        ], 
        vec![ // 94 
            COM(6,37,[0,4,1,2,5,3]), //XXXX(XX)
            PRM(EQ,false),
            INT(10),
            INT(0),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN9SkiAbsEval.A
        vec![ // 95 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(8),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN10SkiAbsEval.K
        vec![ // 96 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(4),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN11SkiAbsEval.Int
        vec![ // 97 
            COM(4,6,[3,0,1,2,0,0]), //XX(XX)
            INT(10),
            COM(2,1,[1,0,0,0,0,0]), //XX
        ], 
         // FUN12Data.List_Type.++
        vec![ // 98 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            Y,
            PTR(100, false),
        ], 
        vec![ // 99 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 100 
            COM(4,6,[3,1,0,2,0,0]), //XX(XX)
            PTR(99, false),
        ], 
         // FUN13SkiAbsEval.peel
        vec![ // 101 
            COM(4,4,[0,1,3,2,0,0]), //XXXX
            Y,
            PTR(105, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 102 
            COM(5,16,[1,3,0,4,2,0]), //XX(XXX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 103 
            COM(6,42,[0,5,1,2,3,4]), //XXX(XXX)
            PRM(EQ,false),
            INT(2),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 104 
            COM(5,52,[3,0,1,3,2,4]), //X(X(XX)XX)
            COM(5,43,[0,2,3,4,1,2]), //XXX(X(XX))
            PTR(103, false),
        ], 
        vec![ // 105 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(104, false),
            PTR(102, false),
        ], 
         // FUN14SkiAbsEval.Y
        vec![ // 106 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(9),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN15SkiAbsEval.compile
        vec![ // 107 
            COM(4,57,[3,0,1,3,2,3]), //X(X(XX)(XX))
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
            PTR(113, false),
            PTR(109, false),
        ], 
        vec![ // 108 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            PTR(114, false),
            PTR(107, false),
        ], 
        vec![ // 109 
            COM(6,37,[0,4,1,3,5,2]), //XXXX(XX)
            PRM(EQ,false),
            INT(1),
            PTR(108, false),
        ], 
        vec![ // 110 
            COM(4,15,[0,1,2,1,3,0]), //X(XX)(XX)
            PTR(15, false),
            PTR(107, false),
        ], 
        vec![ // 111 
            COM(6,37,[0,4,1,3,5,2]), //XXXX(XX)
            PRM(EQ,false),
            INT(2),
            PTR(110, false),
        ], 
        vec![ // 112 
            COM(5,42,[0,3,1,2,3,4]), //XXX(XXX)
            PRM(LT,false),
            INT(2),
        ], 
        vec![ // 113 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(112, false),
            PTR(111, false),
        ], 
         // FUN16SkiAbsEval.abstract
        vec![ // 114 
            COM(6,52,[0,1,2,5,3,4]), //X(X(XX)XX)
            Y,
            COM(5,42,[0,3,4,1,2,4]), //XXX(XXX)
            PTR(124, false),
            PTR(15, false),
            PTR(96, false),
        ], 
        vec![ // 115 
            COM(5,9,[0,2,4,3,1,0]), //XXXXX
            PRM(EQ,false),
            PTR(12, false),
        ], 
        vec![ // 116 
            COM(6,37,[0,4,1,3,5,2]), //XXXX(XX)
            PRM(EQ,false),
            INT(0),
        ], 
        vec![ // 117 
            COM(4,12,[0,1,2,3,3,0]), //X(XXX)X
            PTR(116, false),
            PTR(115, false),
        ], 
        vec![ // 118 
            COM(4,15,[0,1,2,1,3,0]), //X(XX)(XX)
            PTR(125, false),
        ], 
        vec![ // 119 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(118, false),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 120 
            COM(6,37,[0,4,1,3,5,2]), //XXXX(XX)
            PRM(EQ,false),
            INT(2),
        ], 
        vec![ // 121 
            COM(5,42,[0,3,1,2,3,4]), //XXX(XXX)
            PRM(LT,false),
            INT(2),
        ], 
        vec![ // 122 
            COM(5,19,[0,1,2,3,4,0]), //X(X(XX)X)
            PTR(121, false),
            PTR(120, false),
            PTR(119, false),
        ], 
        vec![ // 123 
            COM(5,57,[3,0,1,4,2,4]), //X(X(XX)(XX))
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
        ], 
        vec![ // 124 
            COM(5,15,[0,1,4,2,3,0]), //X(XX)(XX)
            PTR(123, false),
            PTR(122, false),
            PTR(117, false),
        ], 
         // FUN17SkiAbsEval.mkS
        vec![ // 125 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(145, false),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 126 
            COM(3,7,[0,0,1,2,0,0]), //X(XXX)
            PTR(15, false),
            PTR(148, false),
        ], 
        vec![ // 127 
            COM(5,54,[4,0,0,1,2,3]), //X(X(XXX)X)
            PTR(15, false),
            PTR(147, false),
        ], 
        vec![ // 128 
            COM(6,37,[0,4,1,2,3,5]), //XXXX(XX)
            PRM(EQ,false),
            INT(4),
        ], 
        vec![ // 129 
            COM(6,58,[4,0,3,1,2,5]), //X(XX(XXX))
            PTR(128, false),
            PTR(127, false),
        ], 
        vec![ // 130 
            COM(6,37,[0,4,1,3,5,2]), //XXXX(XX)
            PRM(EQ,false),
            INT(2),
        ], 
        vec![ // 131 
            COM(5,54,[3,0,1,2,4,4]), //X(X(XXX)X)
            PTR(130, false),
            PTR(129, false),
        ], 
        vec![ // 132 
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
            PTR(131, false),
            PTR(126, false),
        ], 
        vec![ // 133 
            COM(4,12,[0,0,1,3,2,0]), //X(XXX)X
            PTR(15, false),
            PTR(146, false),
        ], 
        vec![ // 134 
            COM(5,58,[4,0,1,0,2,3]), //X(XX(XXX))
            PTR(15, false),
            PTR(96, false),
        ], 
        vec![ // 135 
            COM(6,37,[0,4,1,2,3,5]), //XXXX(XX)
            PRM(EQ,false),
            INT(4),
        ], 
        vec![ // 136 
            COM(6,58,[4,0,3,1,2,5]), //X(XX(XXX))
            PTR(135, false),
            PTR(134, false),
        ], 
        vec![ // 137 
            COM(6,37,[0,4,1,3,5,2]), //XXXX(XX)
            PRM(EQ,false),
            INT(2),
        ], 
        vec![ // 138 
            COM(5,61,[4,1,0,2,3,3]), //X(X(X(XX)X))
            PTR(137, false),
        ], 
        vec![ // 139 
            COM(5,39,[0,2,1,4,3,4]), //XX(XX)(XX)
            PTR(138, false),
            PTR(136, false),
        ], 
        vec![ // 140 
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(139, false),
            PTR(133, false),
        ], 
        vec![ // 141 
            COM(6,37,[0,4,1,2,3,5]), //XXXX(XX)
            PRM(EQ,false),
            INT(4),
        ], 
        vec![ // 142 
            COM(6,58,[4,0,3,1,2,5]), //X(XX(XXX))
            PTR(141, false),
            PTR(140, false),
        ], 
        vec![ // 143 
            COM(6,37,[0,4,1,3,5,2]), //XXXX(XX)
            PRM(EQ,false),
            INT(2),
        ], 
        vec![ // 144 
            COM(5,54,[2,0,1,3,4,4]), //X(X(XXX)X)
            PTR(143, false),
            PTR(142, false),
        ], 
        vec![ // 145 
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
            PTR(144, false),
            PTR(132, false),
        ], 
         // FUN18SkiAbsEval.B
        vec![ // 146 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(6),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN19SkiAbsEval.C
        vec![ // 147 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(7),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN20SkiAbsEval.S
        vec![ // 148 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(3),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN21SkiAbsEval.sumRange
        vec![ // 149 
            PTR(15, false),
            PTR(106, false),
            PTR(170, false),
        ], 
        vec![ // 150 
            PTR(173, false),
            INT(2),
        ], 
        vec![ // 151 
            PTR(97, false),
            INT(1),
        ], 
        vec![ // 152 
            PTR(173, false),
            INT(1),
        ], 
        vec![ // 153 
            PTR(15, false),
            PTR(174, false),
            PTR(152, false),
        ], 
        vec![ // 154 
            PTR(15, false),
            PTR(153, false),
            PTR(151, false),
        ], 
        vec![ // 155 
            PTR(173, false),
            INT(0),
        ], 
        vec![ // 156 
            PTR(15, false),
            PTR(155, false),
            PTR(154, false),
        ], 
        vec![ // 157 
            PTR(15, false),
            PTR(156, false),
            PTR(150, false),
        ], 
        vec![ // 158 
            PTR(173, false),
            INT(1),
        ], 
        vec![ // 159 
            PTR(15, false),
            PTR(174, false),
            PTR(158, false),
        ], 
        vec![ // 160 
            PTR(15, false),
            PTR(159, false),
            PTR(157, false),
        ], 
        vec![ // 161 
            PTR(173, false),
            INT(1),
        ], 
        vec![ // 162 
            PTR(173, false),
            INT(2),
        ], 
        vec![ // 163 
            PTR(173, false),
            INT(1),
        ], 
        vec![ // 164 
            PTR(15, false),
            PTR(172, false),
            PTR(163, false),
        ], 
        vec![ // 165 
            PTR(15, false),
            PTR(164, false),
            PTR(162, false),
        ], 
        vec![ // 166 
            PTR(15, false),
            PTR(165, false),
            PTR(161, false),
        ], 
        vec![ // 167 
            PTR(15, false),
            PTR(166, false),
            PTR(160, false),
        ], 
        vec![ // 168 
            PTR(171, false),
            INT(2),
            PTR(167, false),
        ], 
        vec![ // 169 
            PTR(171, false),
            INT(1),
            PTR(168, false),
        ], 
        vec![ // 170 
            PTR(171, false),
            INT(0),
            PTR(169, false),
        ], 
         // FUN22SkiAbsEval.Lam
        vec![ // 171 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            INT(1),
            COM(3,2,[2,0,1,0,0,0]), //XXX
        ], 
         // FUN23SkiAbsEval.Eq
        vec![ // 172 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(12),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN24SkiAbsEval.Idx
        vec![ // 173 
            COM(4,6,[3,0,1,2,0,0]), //XX(XX)
            INT(0),
            COM(2,1,[1,0,0,0,0,0]), //XX
        ], 
         // FUN25SkiAbsEval.Add
        vec![ // 174 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(11),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
    ]
});