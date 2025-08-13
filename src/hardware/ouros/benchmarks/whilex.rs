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
            COM(5,1,[2,0,0,0,0,0]), //XX
            INT(1),
        ], 
        vec![ // 14 
            COM(5,1,[4,0,0,0,0,0]), //XX
            INT(4),
        ], 
        vec![ // 15 
            COM(6,2,[4,0,1,0,0,0]), //XXX
            PTR(14, false),
            PTR(13, false),
        ], 
        vec![ // 16 
            COM(7,2,[2,0,1,0,0,0]), //XXX
            INT(4),
            PTR(15, false),
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
            PTR(18, false),
            PTR(17, false),
        ], 
        vec![ // 20 
            COM(7,2,[2,0,1,0,0,0]), //XXX
            INT(5),
            PTR(19, false),
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
            PTR(147, false),
            PTR(22, false),
            PTR(21, false),
        ], 
        vec![ // 24 
            PTR(145, false),
            PTR(23, false),
            PTR(20, false),
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
            PTR(26, false),
            PTR(25, false),
        ], 
        vec![ // 28 
            COM(7,2,[2,0,1,0,0,0]), //XXX
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
            COM(6,2,[4,0,1,0,0,0]), //XXX
            PTR(30, false),
            PTR(29, false),
        ], 
        vec![ // 32 
            COM(7,2,[2,0,1,0,0,0]), //XXX
            INT(0),
            PTR(31, false),
        ], 
        vec![ // 33 
            COM(7,2,[3,0,1,0,0,0]), //XXX
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
            PTR(148, false),
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
            COM(7,2,[2,0,1,0,0,0]), //XXX
            INT(1),
            PTR(38, false),
        ], 
        vec![ // 40 
            COM(5,1,[4,0,0,0,0,0]), //XX
            INT(3),
        ], 
        vec![ // 41 
            COM(7,2,[2,0,1,0,0,0]), //XXX
            INT(0),
            PTR(40, false),
        ], 
        vec![ // 42 
            COM(7,2,[3,0,1,0,0,0]), //XXX
            PTR(41, false),
            PTR(39, false),
        ], 
        vec![ // 43 
            COM(7,2,[3,0,1,0,0,0]), //XXX
            PTR(42, false),
            PTR(37, false),
        ], 
        vec![ // 44 
            COM(7,2,[3,0,1,0,0,0]), //XXX
            PTR(43, false),
            PTR(24, false),
        ], 
        vec![ // 45 
            COM(7,2,[3,0,1,0,0,0]), //XXX
            PTR(44, false),
            PTR(16, false),
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
            PTR(147, false),
            PTR(47, false),
            PTR(46, false),
        ], 
        vec![ // 49 
            PTR(146, false),
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
            COM(7,2,[2,0,1,0,0,0]), //XXX
            INT(4),
            PTR(51, false),
        ], 
        vec![ // 53 
            COM(7,2,[3,0,1,0,0,0]), //XXX
            PTR(52, false),
            PTR(50, false),
        ], 
        vec![ // 54 
            PTR(59, false),
            PTR(53, false),
            PTR(12, false),
        ], 
         // FUN1Whilex.value
        vec![ // 55 
            COM(5,16,[2,0,1,3,4,0]), //XX(XXX)
            ERR(42),
            PTR(58, false),
        ], 
        vec![ // 56 
            COM(4,4,[0,3,1,2,0,0]), //XXXX
            PTR(55, false),
        ], 
        vec![ // 57 
            COM(6,37,[0,4,1,2,3,5]), //XXXX(XX)
            PRM(EQ,false),
        ], 
        vec![ // 58 
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            COM(5,54,[3,0,1,2,4,2]), //X(X(XXX)X)
            PTR(57, false),
            PTR(56, false),
        ], 
         // FUN2Whilex.ssos
        vec![ // 59 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            PTR(60, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
         // FUN3Whilex.run
        vec![ // 60 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(1,0,[0,0,0,0,0,0]), //X
            PTR(61, false),
        ], 
        vec![ // 61 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            PTR(60, false),
            PTR(62, false),
        ], 
         // FUN4Whilex.sosstm
        vec![ // 62 
            COM(5,10,[0,1,4,2,3,0]), //X(XX)XX
            COM(4,39,[0,3,1,3,2,3]), //XX(XX)(XX)
            PTR(74, false),
            COM(3,1,[1,0,0,0,0,0]), //XX
            PTR(66, false),
        ], 
        vec![ // 63 
            COM(4,16,[0,3,1,2,3,0]), //XX(XXX)
            COM(7,2,[3,0,1,0,0,0]), //XXX
            COM(7,2,[6,0,1,0,0,0]), //XXX
        ], 
        vec![ // 64 
            COM(6,53,[0,1,4,2,5,3]), //X(XX(XX)X)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(145, false),
        ], 
        vec![ // 65 
            COM(4,10,[0,1,3,2,3,0]), //X(XX)XX
            PTR(64, false),
            PTR(63, false),
            COM(5,0,[3,0,0,0,0,0]), //X
        ], 
        vec![ // 66 
            COM(4,4,[0,2,3,1,0,0]), //XXXX
            PTR(65, false),
        ], 
        vec![ // 67 
            COM(6,42,[0,3,2,1,4,5]), //XXX(XXX)
            PTR(105, false),
        ], 
        vec![ // 68 
            COM(4,7,[0,1,3,2,0,0]), //X(XXX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(7,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 69 
            COM(5,14,[0,3,2,1,4,0]), //XXX(XX)
            PTR(62, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 70 
            COM(5,37,[0,2,3,4,1,4]), //XXXX(XX)
            PTR(69, false),
            PTR(68, false),
        ], 
        vec![ // 71 
            COM(4,4,[0,2,3,1,0,0]), //XXXX
            PTR(91, false),
            COM(3,1,[1,0,0,0,0,0]), //XX
        ], 
        vec![ // 72 
            COM(5,42,[0,4,2,1,2,3]), //XXX(XXX)
            PTR(75, false),
            PTR(71, false),
        ], 
        vec![ // 73 
            COM(4,15,[2,0,3,1,3,0]), //X(XX)(XX)
            PTR(72, false),
            PTR(70, false),
        ], 
        vec![ // 74 
            COM(5,10,[0,1,4,2,3,0]), //X(XX)XX
            COM(4,47,[0,3,1,2,3,3]), //XX(X(XX)X)
            PTR(73, false),
            PTR(67, false),
            PTR(142, false),
        ], 
         // FUN5Whilex.aval
        vec![ // 75 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
            PTR(85, false),
            PTR(76, false),
        ], 
        vec![ // 76 
            COM(4,4,[0,1,3,2,0,0]), //XXXX
            PTR(55, false),
        ], 
        vec![ // 77 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(75, false),
        ], 
        vec![ // 78 
            COM(6,40,[0,1,4,3,2,5]), //X(XXX)(XX)
            PTR(86, false),
            PTR(75, false),
        ], 
        vec![ // 79 
            COM(3,5,[0,1,2,2,0,0]), //X(XX)X
            PTR(78, false),
            PTR(77, false),
        ], 
        vec![ // 80 
            COM(6,37,[0,2,4,5,1,3]), //XXXX(XX)
            PTR(79, false),
            PTR(90, false),
        ], 
        vec![ // 81 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(75, false),
        ], 
        vec![ // 82 
            COM(6,40,[0,1,4,3,2,5]), //X(XXX)(XX)
            PTR(86, false),
            PTR(75, false),
        ], 
        vec![ // 83 
            COM(3,5,[0,1,2,2,0,0]), //X(XX)X
            PTR(82, false),
            PTR(81, false),
        ], 
        vec![ // 84 
            COM(6,37,[0,2,4,5,1,3]), //XXXX(XX)
            PTR(83, false),
            PTR(87, false),
        ], 
        vec![ // 85 
            COM(5,39,[0,3,1,4,2,4]), //XX(XX)(XX)
            COM(4,38,[0,1,3,3,2,3]), //X(XX)X(XX)
            PTR(84, false),
            PTR(80, false),
        ], 
         // FUN6Whilex.seqq
        vec![ // 86 
            COM(4,7,[1,0,2,3,0,0]), //X(XXX)
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
        ], 
         // FUN7Whilex.add
        vec![ // 87 
            COM(5,12,[0,1,3,4,2,0]), //X(XXX)X
            PTR(88, false),
            PRM(ADD,false),
        ], 
         // FUN8Whilex.int
        vec![ // 88 
            COM(5,38,[0,1,3,4,4,2]), //X(XX)X(XX)
            PTR(89, false),
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
            PTR(88, false),
            PRM(SUB,false),
        ], 
         // FUN10Whilex.update
        vec![ // 91 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            PTR(103, false),
            PTR(102, false),
        ], 
        vec![ // 92 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(104, false),
        ], 
        vec![ // 93 
            COM(5,28,[0,4,1,2,3,3]), //XXX(XX)X
            PTR(91, false),
        ], 
        vec![ // 94 
            COM(4,16,[0,2,1,2,3,0]), //XX(XXX)
            PTR(93, false),
            PTR(92, false),
        ], 
        vec![ // 95 
            COM(6,28,[0,2,1,3,5,4]), //XXX(XX)X
            PTR(91, false),
        ], 
        vec![ // 96 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(6,30,[0,4,1,2,5,3]), //XX(XXX)X
            PTR(95, false),
            PTR(104, false),
        ], 
        vec![ // 97 
            COM(6,28,[0,4,1,2,5,3]), //XXX(XX)X
            PRM(EQ,false),
        ], 
        vec![ // 98 
            COM(6,44,[0,1,2,3,4,5]), //X(XX)(XXX)
            COM(5,26,[0,1,2,4,3,4]), //X(XXX)XX
            PTR(97, false),
        ], 
        vec![ // 99 
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(98, false),
            PTR(96, false),
        ], 
        vec![ // 100 
            COM(5,38,[0,1,2,4,3,4]), //X(XX)X(XX)
            COM(5,56,[3,0,1,4,2,4]), //X(XXX(XX))
        ], 
        vec![ // 101 
            COM(5,38,[0,1,2,4,3,4]), //X(XX)X(XX)
            PTR(100, false),
            PTR(99, false),
        ], 
        vec![ // 102 
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(101, false),
            PTR(94, false),
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
            PTR(134, false),
            PTR(109, false),
        ], 
        vec![ // 106 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(105, false),
        ], 
        vec![ // 107 
            COM(6,40,[0,1,4,3,2,5]), //X(XXX)(XX)
            PTR(86, false),
            PTR(105, false),
        ], 
        vec![ // 108 
            COM(3,5,[0,1,2,2,0,0]), //X(XX)X
            PTR(107, false),
            PTR(106, false),
        ], 
        vec![ // 109 
            COM(6,37,[0,2,4,5,1,3]), //XXXX(XX)
            PTR(108, false),
            PTR(140, false),
        ], 
        vec![ // 110 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(75, false),
        ], 
        vec![ // 111 
            COM(6,40,[0,1,4,3,2,5]), //X(XXX)(XX)
            PTR(86, false),
            PTR(75, false),
        ], 
        vec![ // 112 
            COM(3,5,[0,1,2,2,0,0]), //X(XX)X
            PTR(111, false),
            PTR(110, false),
        ], 
        vec![ // 113 
            COM(6,37,[0,2,4,5,1,3]), //XXXX(XX)
            PTR(112, false),
            PTR(139, false),
        ], 
        vec![ // 114 
            COM(6,43,[0,4,1,5,3,2]), //XXX(X(XX))
            PRM(LT,false),
            INT(2),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 115 
            COM(5,37,[0,1,3,4,4,2]), //XXXX(XX)
            PTR(114, false),
        ], 
        vec![ // 116 
            COM(4,16,[0,3,1,2,3,0]), //XX(XXX)
            PTR(115, false),
            PTR(113, false),
        ], 
        vec![ // 117 
            COM(5,42,[0,3,1,2,3,4]), //XXX(XXX)
            PRM(LT,false),
            INT(1),
        ], 
        vec![ // 118 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(75, false),
        ], 
        vec![ // 119 
            COM(6,40,[0,1,4,3,2,5]), //X(XXX)(XX)
            PTR(86, false),
            PTR(75, false),
        ], 
        vec![ // 120 
            COM(3,5,[0,1,2,2,0,0]), //X(XX)X
            PTR(119, false),
            PTR(118, false),
        ], 
        vec![ // 121 
            COM(6,37,[0,2,4,5,1,3]), //XXXX(XX)
            PTR(120, false),
            PTR(138, false),
        ], 
        vec![ // 122 
            COM(5,14,[0,4,2,1,3,0]), //XXX(XX)
            PTR(105, false),
            PTR(135, false),
        ], 
        vec![ // 123 
            COM(6,43,[0,4,1,5,3,2]), //XXX(X(XX))
            PRM(LT,false),
            INT(5),
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
        vec![ // 124 
            COM(5,37,[0,1,3,4,4,2]), //XXXX(XX)
            PTR(123, false),
        ], 
        vec![ // 125 
            COM(4,16,[0,3,1,2,3,0]), //XX(XXX)
            PTR(124, false),
            PTR(122, false),
        ], 
        vec![ // 126 
            COM(5,42,[0,3,1,2,3,4]), //XXX(XXX)
            PRM(LT,false),
            INT(4),
        ], 
        vec![ // 127 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            PTR(126, false),
            PTR(125, false),
        ], 
        vec![ // 128 
            COM(5,38,[0,1,2,4,3,4]), //X(XX)X(XX)
            COM(5,37,[0,1,3,4,4,2]), //XXXX(XX)
            PTR(127, false),
        ], 
        vec![ // 129 
            COM(5,42,[0,3,1,2,3,4]), //XXX(XXX)
            PRM(LT,false),
            INT(3),
        ], 
        vec![ // 130 
            COM(5,53,[0,1,3,2,3,4]), //X(XX(XX)X)
            PTR(129, false),
            PTR(128, false),
            PTR(121, false),
        ], 
        vec![ // 131 
            COM(5,29,[0,1,4,2,4,3]), //X(XX)(XX)X
            COM(4,48,[0,3,1,3,3,2]), //XX(XX(XX))
        ], 
        vec![ // 132 
            COM(5,45,[0,1,4,2,3,4]), //X(XX)(X(XX))
            PTR(131, false),
        ], 
        vec![ // 133 
            COM(5,38,[0,1,4,2,3,4]), //X(XX)X(XX)
            PTR(132, false),
            PTR(130, false),
            PTR(117, false),
            PTR(116, false),
        ], 
        vec![ // 134 
            COM(5,56,[1,0,2,4,3,4]), //X(XXX(XX))
            PTR(133, false),
        ], 
         // FUN13Whilex.notk
        vec![ // 135 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(136, false),
            PTR(137, false),
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
            PTR(136, false),
            PRM(LE,false),
        ], 
         // FUN17Whilex.eq
        vec![ // 139 
            COM(5,12,[0,1,3,4,2,0]), //X(XXX)X
            PTR(136, false),
            PRM(EQ,false),
        ], 
         // FUN18Whilex.andk
        vec![ // 140 
            COM(5,12,[0,1,3,4,2,0]), //X(XXX)X
            PTR(136, false),
            PTR(141, false),
        ], 
         // FUN19Data.Bool.&&
        vec![ // 141 
            COM(2,1,[1,0,0,0,0,0]), //XX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN20Whilex.cond
        vec![ // 142 
            COM(3,5,[0,1,2,2,0,0]), //X(XX)X
            PTR(144, false),
            PTR(143, false),
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