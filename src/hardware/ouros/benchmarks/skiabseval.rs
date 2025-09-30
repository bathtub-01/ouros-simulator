use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 26
// Apps in this file: 189
// Combinators in this file: 201
#[rustfmt::skip]
pub static SKIABSEVAL: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0SkiAbsEval.main
        vec![ // 0 
            PTR(6, false),
            PTR(5, false),
        ], 
        vec![ // 1 
            PTR(111, false),
            INT(5),
        ], 
        vec![ // 2 
            PTR(111, false),
            INT(1),
        ], 
        vec![ // 3 
            PTR(121, false),
            PTR(163, false),
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
            PTR(115, false),
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
            PTR(106, false),
        ], 
        vec![ // 17 
            COM(5,12,[0,1,4,3,2,0]), //X(XXX)X
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(15, false),
        ], 
        vec![ // 18 
            COM(5,46,[0,3,1,2,3,4]), //XX(XXXX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(17, false),
        ], 
        vec![ // 19 
            COM(6,57,[0,1,2,3,4,5]), //X(X(XX)(XX))
            PTR(16, false),
            PTR(112, false),
            PTR(115, false),
        ], 
        vec![ // 20 
            COM(5,16,[0,2,1,3,4,0]), //XX(XXX)
            PTR(19, false),
            PTR(18, false),
        ], 
        vec![ // 21 
            COM(4,40,[0,0,3,2,1,3]), //X(XXX)(XX)
            SEQ(false),
        ], 
        vec![ // 22 
            COM(5,38,[0,1,4,4,2,3]), //X(XX)X(XX)
            PTR(21, false),
        ], 
        vec![ // 23 
            COM(6,24,[0,1,5,2,3,4]), //X(XX)XXX
            PTR(22, false),
        ], 
        vec![ // 24 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(23, false),
            PTR(20, false),
            PTR(9, false),
        ], 
        vec![ // 25 
            COM(5,14,[0,2,3,1,4,0]), //XXX(XX)
            PTR(24, false),
            PTR(9, false),
        ], 
        vec![ // 26 
            COM(5,16,[4,1,0,2,3,0]), //XX(XXX)
            PTR(25, false),
        ], 
        vec![ // 27 
            COM(4,16,[3,1,0,1,2,0]), //XX(XXX)
            PTR(26, false),
        ], 
        vec![ // 28 
            COM(4,20,[3,2,1,0,1,0]), //X(XX(XX))
            PTR(27, false),
        ], 
        vec![ // 29 
            COM(6,37,[0,4,1,2,3,5]), //XXXX(XX)
            PRM(EQ,false),
            INT(3),
        ], 
        vec![ // 30 
            COM(4,16,[0,2,1,2,3,0]), //XX(XXX)
            PTR(29, false),
            PTR(28, false),
        ], 
        vec![ // 31 
            COM(6,19,[0,1,2,3,5,0]), //X(X(XX)X)
            PTR(16, false),
            PTR(112, false),
            PTR(115, false),
        ], 
        vec![ // 32 
            COM(4,6,[3,1,0,2,0,0]), //XX(XX)
            PTR(31, false),
        ], 
        vec![ // 33 
            COM(4,20,[3,2,1,0,1,0]), //X(XX(XX))
            PTR(32, false),
        ], 
        vec![ // 34 
            COM(6,42,[0,4,1,2,3,5]), //XXX(XXX)
            PRM(LT,false),
            INT(4),
        ], 
        vec![ // 35 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(34, false),
            PTR(33, false),
        ], 
        vec![ // 36 
            COM(5,19,[0,1,2,3,4,0]), //X(X(XX)X)
            PTR(16, false),
            PTR(112, false),
            PTR(115, false),
        ], 
        vec![ // 37 
            COM(5,12,[0,1,4,3,2,0]), //X(XXX)X
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(15, false),
        ], 
        vec![ // 38 
            COM(6,57,[0,1,2,3,4,5]), //X(X(XX)(XX))
            PTR(16, false),
            PTR(112, false),
            PTR(115, false),
        ], 
        vec![ // 39 
            COM(5,16,[0,2,1,3,4,0]), //XX(XXX)
            PTR(38, false),
            PTR(37, false),
        ], 
        vec![ // 40 
            COM(4,40,[0,0,3,2,1,3]), //X(XXX)(XX)
            SEQ(false),
        ], 
        vec![ // 41 
            COM(5,38,[0,1,4,4,2,3]), //X(XX)X(XX)
            PTR(40, false),
        ], 
        vec![ // 42 
            COM(6,24,[0,1,5,2,3,4]), //X(XX)XXX
            PTR(41, false),
        ], 
        vec![ // 43 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(42, false),
            PTR(39, false),
            PTR(9, false),
        ], 
        vec![ // 44 
            COM(5,14,[0,2,3,1,4,0]), //XXX(XX)
            PTR(43, false),
            PTR(9, false),
        ], 
        vec![ // 45 
            COM(5,16,[4,1,0,2,3,0]), //XX(XXX)
            PTR(44, false),
        ], 
        vec![ // 46 
            COM(4,16,[3,1,0,1,2,0]), //XX(XXX)
            PTR(45, false),
        ], 
        vec![ // 47 
            COM(4,20,[3,2,1,0,1,0]), //X(XX(XX))
            PTR(46, false),
        ], 
        vec![ // 48 
            COM(6,42,[0,4,1,2,3,5]), //XXX(XXX)
            PRM(LT,false),
            INT(6),
        ], 
        vec![ // 49 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(48, false),
            PTR(47, false),
        ], 
        vec![ // 50 
            COM(6,24,[0,1,5,4,2,3]), //X(XX)XXX
            COM(5,49,[0,4,4,1,2,3]), //XX(X(XXX))
        ], 
        vec![ // 51 
            COM(5,24,[0,1,4,3,2,4]), //X(XX)XXX
            PTR(50, false),
        ], 
        vec![ // 52 
            COM(4,10,[0,1,3,2,3,0]), //X(XX)XX
            PTR(51, false),
            PTR(49, false),
            PTR(36, false),
        ], 
        vec![ // 53 
            COM(5,42,[0,3,1,2,3,4]), //XXX(XXX)
            PRM(LT,false),
            INT(5),
        ], 
        vec![ // 54 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            PTR(53, false),
            PTR(52, false),
        ], 
        vec![ // 55 
            COM(5,40,[0,1,2,4,3,4]), //X(XXX)(XX)
            COM(4,48,[0,3,1,3,2,3]), //XX(XX(XX))
        ], 
        vec![ // 56 
            COM(5,38,[0,1,2,4,3,4]), //X(XX)X(XX)
            PTR(55, false),
            PTR(54, false),
        ], 
        vec![ // 57 
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(56, false),
            PTR(35, false),
        ], 
        vec![ // 58 
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
            PTR(57, false),
            PTR(30, false),
        ], 
        vec![ // 59 
            COM(4,16,[0,2,0,3,1,0]), //XX(XXX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 60 
            COM(6,57,[0,1,2,3,4,5]), //X(X(XX)(XX))
            PTR(16, false),
            PTR(112, false),
            PTR(115, false),
        ], 
        vec![ // 61 
            COM(5,16,[0,2,1,3,4,0]), //XX(XXX)
            PTR(60, false),
            PTR(59, false),
        ], 
        vec![ // 62 
            COM(4,40,[0,0,3,2,1,3]), //X(XXX)(XX)
            SEQ(false),
        ], 
        vec![ // 63 
            COM(5,38,[0,1,4,4,2,3]), //X(XX)X(XX)
            PTR(62, false),
        ], 
        vec![ // 64 
            COM(6,24,[0,1,5,2,3,4]), //X(XX)XXX
            PTR(63, false),
        ], 
        vec![ // 65 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(64, false),
            PTR(61, false),
            PTR(9, false),
        ], 
        vec![ // 66 
            COM(5,14,[0,2,3,1,4,0]), //XXX(XX)
            PTR(65, false),
            PTR(9, false),
        ], 
        vec![ // 67 
            COM(5,16,[4,1,0,2,3,0]), //XX(XXX)
            PTR(66, false),
        ], 
        vec![ // 68 
            COM(4,16,[3,1,0,1,2,0]), //XX(XXX)
            PTR(67, false),
        ], 
        vec![ // 69 
            COM(5,19,[0,1,2,3,4,0]), //X(X(XX)X)
            PTR(16, false),
            PTR(112, false),
            PTR(115, false),
        ], 
        vec![ // 70 
            COM(4,2,[3,1,0,0,0,0]), //XXX
            PTR(69, false),
        ], 
        vec![ // 71 
            COM(4,20,[3,2,1,0,1,0]), //X(XX(XX))
            PTR(70, false),
        ], 
        vec![ // 72 
            COM(6,42,[0,4,1,2,3,5]), //XXX(XXX)
            PRM(LT,false),
            INT(8),
        ], 
        vec![ // 73 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(72, false),
            PTR(71, false),
        ], 
        vec![ // 74 
            COM(6,24,[0,1,5,4,2,3]), //X(XX)XXX
            COM(5,49,[0,4,4,1,2,3]), //XX(X(XXX))
        ], 
        vec![ // 75 
            COM(5,24,[0,1,4,3,2,4]), //X(XX)XXX
            PTR(74, false),
        ], 
        vec![ // 76 
            COM(4,29,[0,1,3,2,3,3]), //X(XX)(XX)X
            PTR(75, false),
            PTR(73, false),
            PTR(68, false),
        ], 
        vec![ // 77 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(15, false),
            PTR(120, false),
        ], 
        vec![ // 78 
            COM(5,44,[0,1,3,2,3,4]), //X(XX)(XXX)
            PTR(112, false),
            PTR(115, false),
            PTR(77, false),
        ], 
        vec![ // 79 
            COM(4,7,[3,2,1,0,0,0]), //X(XXX)
            PTR(78, false),
        ], 
        vec![ // 80 
            COM(6,37,[0,4,1,2,3,5]), //XXXX(XX)
            PRM(EQ,false),
            INT(9),
        ], 
        vec![ // 81 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(107, false),
            PTR(9, false),
        ], 
        vec![ // 82 
            COM(6,41,[0,1,2,4,3,5]), //X(X(XX))(XX)
            PRM(ADD,false),
            PTR(107, false),
            PTR(9, false),
            PTR(81, false),
        ], 
        vec![ // 83 
            COM(6,55,[0,1,2,3,4,5]), //X(X(X(XX))X)
            PTR(16, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(111, false),
        ], 
        vec![ // 84 
            COM(5,17,[4,2,0,1,3,0]), //XX(X(XX))
            PTR(83, false),
            PTR(82, false),
        ], 
        vec![ // 85 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(107, false),
            PTR(9, false),
        ], 
        vec![ // 86 
            COM(6,41,[0,1,2,4,3,5]), //X(X(XX))(XX)
            PRM(EQ,false),
            PTR(107, false),
            PTR(9, false),
            PTR(85, false),
        ], 
        vec![ // 87 
            COM(4,4,[0,2,3,1,0,0]), //XXXX
            PTR(86, false),
            PTR(109, false),
        ], 
        vec![ // 88 
            COM(6,54,[0,1,2,4,3,5]), //X(X(XXX)X)
            PTR(16, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 89 
            COM(6,47,[5,3,0,1,4,2]), //XX(X(XX)X)
            PTR(88, false),
            PTR(87, false),
            PTR(110, false),
        ], 
        vec![ // 90 
            COM(4,20,[3,2,1,0,1,0]), //X(XX(XX))
            PTR(89, false),
        ], 
        vec![ // 91 
            COM(6,42,[0,4,1,2,3,5]), //XXX(XXX)
            PRM(LT,false),
            INT(12),
        ], 
        vec![ // 92 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(91, false),
            PTR(90, false),
        ], 
        vec![ // 93 
            COM(6,24,[0,1,5,4,2,3]), //X(XX)XXX
            COM(5,49,[0,4,4,1,2,3]), //XX(X(XXX))
        ], 
        vec![ // 94 
            COM(5,24,[0,1,4,3,2,4]), //X(XX)XXX
            PTR(93, false),
        ], 
        vec![ // 95 
            COM(4,29,[0,1,3,2,3,3]), //X(XX)(XX)X
            PTR(94, false),
            PTR(92, false),
            PTR(84, false),
        ], 
        vec![ // 96 
            COM(5,42,[0,3,1,2,3,4]), //XXX(XXX)
            PRM(LT,false),
            INT(11),
        ], 
        vec![ // 97 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            PTR(96, false),
            PTR(95, false),
        ], 
        vec![ // 98 
            COM(5,45,[0,1,4,2,3,4]), //X(XX)(X(XX))
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
        ], 
        vec![ // 99 
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            PTR(98, false),
            PTR(97, false),
            PTR(80, false),
        ], 
        vec![ // 100 
            COM(5,42,[0,3,1,2,3,4]), //XXX(XXX)
            PRM(LT,false),
            INT(9),
        ], 
        vec![ // 101 
            COM(5,53,[0,1,3,2,3,4]), //X(XX(XX)X)
            PTR(100, false),
            PTR(99, false),
            PTR(79, false),
        ], 
        vec![ // 102 
            COM(5,40,[0,1,2,4,3,4]), //X(XXX)(XX)
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
            PTR(101, false),
        ], 
        vec![ // 103 
            COM(5,42,[0,3,1,2,3,4]), //XXX(XXX)
            PRM(LT,false),
            INT(7),
        ], 
        vec![ // 104 
            COM(5,53,[0,1,3,2,3,4]), //X(XX(XX)X)
            PTR(103, false),
            PTR(102, false),
            PTR(76, false),
        ], 
        vec![ // 105 
            COM(5,57,[3,0,1,4,2,4]), //X(X(XX)(XX))
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
        ], 
        vec![ // 106 
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            PTR(105, false),
            PTR(104, false),
            PTR(58, false),
        ], 
         // FUN8SkiAbsEval.peek
        vec![ // 107 
            COM(2,1,[1,0,0,0,0,0]), //XX
            PTR(108, false),
        ], 
        vec![ // 108 
            COM(6,37,[0,4,1,2,5,3]), //XXXX(XX)
            PRM(EQ,false),
            INT(10),
            INT(0),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN9SkiAbsEval.A
        vec![ // 109 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(8),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN10SkiAbsEval.K
        vec![ // 110 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(4),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN11SkiAbsEval.Int
        vec![ // 111 
            COM(4,6,[3,0,1,2,0,0]), //XX(XX)
            INT(10),
            COM(2,1,[1,0,0,0,0,0]), //XX
        ], 
         // FUN12Data.List_Type.++
        vec![ // 112 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            Y,
            PTR(114, false),
        ], 
        vec![ // 113 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 114 
            COM(4,6,[3,1,0,2,0,0]), //XX(XX)
            PTR(113, false),
        ], 
         // FUN13SkiAbsEval.peel
        vec![ // 115 
            COM(4,4,[0,1,3,2,0,0]), //XXXX
            Y,
            PTR(119, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 116 
            COM(5,16,[1,3,0,4,2,0]), //XX(XXX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 117 
            COM(6,42,[0,5,1,2,3,4]), //XXX(XXX)
            PRM(EQ,false),
            INT(2),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 118 
            COM(5,52,[3,0,1,3,2,4]), //X(X(XX)XX)
            COM(5,43,[0,2,3,4,1,2]), //XXX(X(XX))
            PTR(117, false),
        ], 
        vec![ // 119 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(118, false),
            PTR(116, false),
        ], 
         // FUN14SkiAbsEval.Y
        vec![ // 120 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(9),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN15SkiAbsEval.compile
        vec![ // 121 
            COM(4,57,[3,0,1,3,2,3]), //X(X(XX)(XX))
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
            PTR(127, false),
            PTR(123, false),
        ], 
        vec![ // 122 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            PTR(128, false),
            PTR(121, false),
        ], 
        vec![ // 123 
            COM(6,37,[0,4,1,3,5,2]), //XXXX(XX)
            PRM(EQ,false),
            INT(1),
            PTR(122, false),
        ], 
        vec![ // 124 
            COM(4,15,[0,1,2,1,3,0]), //X(XX)(XX)
            PTR(15, false),
            PTR(121, false),
        ], 
        vec![ // 125 
            COM(6,37,[0,4,1,3,5,2]), //XXXX(XX)
            PRM(EQ,false),
            INT(2),
            PTR(124, false),
        ], 
        vec![ // 126 
            COM(5,42,[0,3,1,2,3,4]), //XXX(XXX)
            PRM(LT,false),
            INT(2),
        ], 
        vec![ // 127 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(126, false),
            PTR(125, false),
        ], 
         // FUN16SkiAbsEval.abstract
        vec![ // 128 
            COM(6,52,[0,1,2,5,3,4]), //X(X(XX)XX)
            Y,
            COM(5,42,[0,3,4,1,2,4]), //XXX(XXX)
            PTR(138, false),
            PTR(15, false),
            PTR(110, false),
        ], 
        vec![ // 129 
            COM(5,9,[0,2,4,3,1,0]), //XXXXX
            PRM(EQ,false),
            PTR(12, false),
        ], 
        vec![ // 130 
            COM(6,37,[0,4,1,3,5,2]), //XXXX(XX)
            PRM(EQ,false),
            INT(0),
        ], 
        vec![ // 131 
            COM(4,12,[0,1,2,3,3,0]), //X(XXX)X
            PTR(130, false),
            PTR(129, false),
        ], 
        vec![ // 132 
            COM(4,15,[0,1,2,1,3,0]), //X(XX)(XX)
            PTR(139, false),
        ], 
        vec![ // 133 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(132, false),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 134 
            COM(6,37,[0,4,1,3,5,2]), //XXXX(XX)
            PRM(EQ,false),
            INT(2),
        ], 
        vec![ // 135 
            COM(5,42,[0,3,1,2,3,4]), //XXX(XXX)
            PRM(LT,false),
            INT(2),
        ], 
        vec![ // 136 
            COM(5,19,[0,1,2,3,4,0]), //X(X(XX)X)
            PTR(135, false),
            PTR(134, false),
            PTR(133, false),
        ], 
        vec![ // 137 
            COM(5,57,[3,0,1,4,2,4]), //X(X(XX)(XX))
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
        ], 
        vec![ // 138 
            COM(5,15,[0,1,4,2,3,0]), //X(XX)(XX)
            PTR(137, false),
            PTR(136, false),
            PTR(131, false),
        ], 
         // FUN17SkiAbsEval.mkS
        vec![ // 139 
            COM(4,16,[0,3,1,2,3,0]), //XX(XXX)
            SEQ(false),
            PTR(159, false),
        ], 
        vec![ // 140 
            COM(3,7,[0,0,1,2,0,0]), //X(XXX)
            PTR(15, false),
            PTR(162, false),
        ], 
        vec![ // 141 
            COM(5,54,[4,0,0,1,2,3]), //X(X(XXX)X)
            PTR(15, false),
            PTR(161, false),
        ], 
        vec![ // 142 
            COM(6,37,[0,4,1,2,3,5]), //XXXX(XX)
            PRM(EQ,false),
            INT(4),
        ], 
        vec![ // 143 
            COM(6,58,[4,0,3,1,2,5]), //X(XX(XXX))
            PTR(142, false),
            PTR(141, false),
        ], 
        vec![ // 144 
            COM(6,37,[0,4,1,3,5,2]), //XXXX(XX)
            PRM(EQ,false),
            INT(2),
        ], 
        vec![ // 145 
            COM(5,54,[3,0,1,2,4,4]), //X(X(XXX)X)
            PTR(144, false),
            PTR(143, false),
        ], 
        vec![ // 146 
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
            PTR(145, false),
            PTR(140, false),
        ], 
        vec![ // 147 
            COM(4,12,[0,0,1,3,2,0]), //X(XXX)X
            PTR(15, false),
            PTR(160, false),
        ], 
        vec![ // 148 
            COM(5,58,[4,0,1,0,2,3]), //X(XX(XXX))
            PTR(15, false),
            PTR(110, false),
        ], 
        vec![ // 149 
            COM(6,37,[0,4,1,2,3,5]), //XXXX(XX)
            PRM(EQ,false),
            INT(4),
        ], 
        vec![ // 150 
            COM(6,58,[4,0,3,1,2,5]), //X(XX(XXX))
            PTR(149, false),
            PTR(148, false),
        ], 
        vec![ // 151 
            COM(6,37,[0,4,1,3,5,2]), //XXXX(XX)
            PRM(EQ,false),
            INT(2),
        ], 
        vec![ // 152 
            COM(5,61,[4,1,0,2,3,3]), //X(X(X(XX)X))
            PTR(151, false),
        ], 
        vec![ // 153 
            COM(5,39,[0,2,1,4,3,4]), //XX(XX)(XX)
            PTR(152, false),
            PTR(150, false),
        ], 
        vec![ // 154 
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(153, false),
            PTR(147, false),
        ], 
        vec![ // 155 
            COM(6,37,[0,4,1,2,3,5]), //XXXX(XX)
            PRM(EQ,false),
            INT(4),
        ], 
        vec![ // 156 
            COM(6,58,[4,0,3,1,2,5]), //X(XX(XXX))
            PTR(155, false),
            PTR(154, false),
        ], 
        vec![ // 157 
            COM(6,37,[0,4,1,3,5,2]), //XXXX(XX)
            PRM(EQ,false),
            INT(2),
        ], 
        vec![ // 158 
            COM(5,54,[2,0,1,3,4,4]), //X(X(XXX)X)
            PTR(157, false),
            PTR(156, false),
        ], 
        vec![ // 159 
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
            PTR(158, false),
            PTR(146, false),
        ], 
         // FUN18SkiAbsEval.B
        vec![ // 160 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(6),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN19SkiAbsEval.C
        vec![ // 161 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(7),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN20SkiAbsEval.S
        vec![ // 162 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(3),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN21SkiAbsEval.sumRange
        vec![ // 163 
            PTR(15, false),
            PTR(120, false),
            PTR(184, false),
        ], 
        vec![ // 164 
            PTR(187, false),
            INT(2),
        ], 
        vec![ // 165 
            PTR(111, false),
            INT(1),
        ], 
        vec![ // 166 
            PTR(187, false),
            INT(1),
        ], 
        vec![ // 167 
            PTR(15, false),
            PTR(188, false),
            PTR(166, false),
        ], 
        vec![ // 168 
            PTR(15, false),
            PTR(167, false),
            PTR(165, false),
        ], 
        vec![ // 169 
            PTR(187, false),
            INT(0),
        ], 
        vec![ // 170 
            PTR(15, false),
            PTR(169, false),
            PTR(168, false),
        ], 
        vec![ // 171 
            PTR(15, false),
            PTR(170, false),
            PTR(164, false),
        ], 
        vec![ // 172 
            PTR(187, false),
            INT(1),
        ], 
        vec![ // 173 
            PTR(15, false),
            PTR(188, false),
            PTR(172, false),
        ], 
        vec![ // 174 
            PTR(15, false),
            PTR(173, false),
            PTR(171, false),
        ], 
        vec![ // 175 
            PTR(187, false),
            INT(1),
        ], 
        vec![ // 176 
            PTR(187, false),
            INT(2),
        ], 
        vec![ // 177 
            PTR(187, false),
            INT(1),
        ], 
        vec![ // 178 
            PTR(15, false),
            PTR(186, false),
            PTR(177, false),
        ], 
        vec![ // 179 
            PTR(15, false),
            PTR(178, false),
            PTR(176, false),
        ], 
        vec![ // 180 
            PTR(15, false),
            PTR(179, false),
            PTR(175, false),
        ], 
        vec![ // 181 
            PTR(15, false),
            PTR(180, false),
            PTR(174, false),
        ], 
        vec![ // 182 
            PTR(185, false),
            INT(2),
            PTR(181, false),
        ], 
        vec![ // 183 
            PTR(185, false),
            INT(1),
            PTR(182, false),
        ], 
        vec![ // 184 
            PTR(185, false),
            INT(0),
            PTR(183, false),
        ], 
         // FUN22SkiAbsEval.Lam
        vec![ // 185 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            INT(1),
            COM(3,2,[2,0,1,0,0,0]), //XXX
        ], 
         // FUN23SkiAbsEval.Eq
        vec![ // 186 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(12),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN24SkiAbsEval.Idx
        vec![ // 187 
            COM(4,6,[3,0,1,2,0,0]), //XX(XX)
            INT(0),
            COM(2,1,[1,0,0,0,0,0]), //XX
        ], 
         // FUN25SkiAbsEval.Add
        vec![ // 188 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(11),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
    ]
});