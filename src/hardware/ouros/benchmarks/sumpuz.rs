use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 38
// Apps in this file: 133
// Combinators in this file: 184
#[rustfmt::skip]
pub static SUMPUZ: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0Sumpuz.main
        vec![ // 0 
            COM(2,4,[0,1,1,1,0,0]), //XXXX
            PTR(10, false),
            PTR(9, false),
        ], 
        vec![ // 1 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(2),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 2 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(1),
            PTR(1, false),
        ], 
        vec![ // 3 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(2),
            PTR(2, false),
        ], 
        vec![ // 4 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(3, false),
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
            PTR(5, false),
        ], 
        vec![ // 7 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(1),
            PTR(6, false),
        ], 
        vec![ // 8 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(0),
            PTR(7, false),
        ], 
        vec![ // 9 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(8, false),
            PTR(4, false),
        ], 
         // FUN1Sumpuz.count
        vec![ // 10 
            COM(5,12,[0,1,3,4,2,0]), //X(XXX)X
            PTR(11, false),
            PTR(15, false),
        ], 
         // FUN2Sumpuz.sumMap
        vec![ // 11 
            COM(4,4,[0,2,3,1,0,0]), //XXXX
            PTR(12, false),
            INT(0),
        ], 
         // FUN3Sumpuz.sumMapAcc
        vec![ // 12 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(14, false),
        ], 
        vec![ // 13 
            COM(6,47,[2,5,0,1,4,3]), //XX(X(XX)X)
            PRM(ADD,false),
        ], 
        vec![ // 14 
            COM(5,46,[3,4,0,1,2,4]), //XX(XXXX)
            PTR(13, false),
        ], 
         // FUN4Sumpuz.fx
        vec![ // 15 
            COM(5,12,[0,1,4,3,2,0]), //X(XXX)X
            PTR(11, false),
            PTR(16, false),
        ], 
         // FUN5Sumpuz.fy
        vec![ // 16 
            COM(5,12,[0,1,2,4,3,0]), //X(XXX)X
            PTR(11, false),
            PTR(17, false),
        ], 
         // FUN6Sumpuz.fz
        vec![ // 17 
            COM(6,23,[0,3,4,5,1,2]), //XXXXXX
            PTR(18, false),
            INT(0),
            INT(1),
        ], 
         // FUN7Sumpuz.valid
        vec![ // 18 
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
            PTR(25, false),
            PTR(22, false),
        ], 
        vec![ // 19 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(0),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 20 
            COM(5,9,[0,2,3,4,1,0]), //XXXXX
            PTR(33, false),
            PTR(19, false),
        ], 
        vec![ // 21 
            COM(5,48,[0,1,2,4,3,4]), //XX(XX(XX))
            COM(1,0,[0,0,0,0,0,0]), //X
            PTR(30, false),
            SEQ(false),
        ], 
        vec![ // 22 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            PTR(21, false),
            PTR(20, false),
        ], 
        vec![ // 23 
            COM(5,57,[0,1,2,3,2,4]), //X(X(XX)(XX))
            PTR(26, false),
            PRM(EQ,false),
            PTR(27, false),
        ], 
        vec![ // 24 
            COM(5,57,[0,1,2,3,2,4]), //X(X(XX)(XX))
            PTR(26, false),
            PRM(EQ,false),
            PTR(27, false),
        ], 
        vec![ // 25 
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            COM(5,48,[0,2,1,4,3,4]), //XX(XX(XX))
            PTR(24, false),
            PTR(23, false),
        ], 
         // FUN8Data.Bool.&&
        vec![ // 26 
            COM(2,1,[1,0,0,0,0,0]), //XX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN9NanoPrelude.length
        vec![ // 27 
            Y,
            PTR(29, false),
            INT(0),
        ], 
        vec![ // 28 
            COM(5,7,[2,0,3,1,0,0]), //X(XXX)
            PRM(ADD,false),
            INT(1),
        ], 
        vec![ // 29 
            COM(4,16,[3,2,0,1,2,0]), //XX(XXX)
            PTR(28, false),
        ], 
         // FUN10Sumpuz.isSingleton
        vec![ // 30 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(32, false),
        ], 
        vec![ // 31 
            COM(3,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 32 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(31, false),
        ], 
         // FUN11Sumpuz.solutions
        vec![ // 33 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
            PTR(59, false),
            PTR(50, false),
        ], 
        vec![ // 34 
            COM(6,32,[0,1,5,2,3,4]), //X(XXXX)X
            PTR(125, false),
            PTR(132, false),
            INT(1),
            INT(0),
            INT(9),
        ], 
        vec![ // 35 
            COM(6,39,[0,4,1,5,2,3]), //XX(XX)(XX)
            PTR(62, false),
            PTR(34, false),
            PTR(61, false),
        ], 
        vec![ // 36 
            COM(6,52,[0,1,2,5,3,4]), //X(X(XX)XX)
            PTR(125, false),
            PTR(132, false),
            PTR(124, false),
            INT(1),
            INT(0),
        ], 
        vec![ // 37 
            COM(5,44,[0,1,4,2,4,3]), //X(XX)(XXX)
            PTR(62, false),
            PTR(123, false),
            PTR(36, false),
            INT(9),
        ], 
        vec![ // 38 
            COM(4,6,[0,3,1,2,0,0]), //XX(XX)
            PTR(33, false),
            PTR(124, false),
        ], 
        vec![ // 39 
            COM(6,38,[0,1,4,5,2,3]), //X(XX)X(XX)
            PTR(104, false),
            PTR(60, false),
            PTR(123, false),
        ], 
        vec![ // 40 
            COM(6,38,[0,1,4,2,3,5]), //X(XX)X(XX)
            COM(5,48,[0,3,1,4,2,4]), //XX(XX(XX))
        ], 
        vec![ // 41 
            COM(6,26,[0,1,3,5,2,4]), //X(XXX)XX
            PTR(40, false),
            PTR(39, false),
            SEQ(false),
        ], 
        vec![ // 42 
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(41, false),
            PTR(38, false),
        ], 
        vec![ // 43 
            COM(6,35,[0,1,2,4,5,3]), //X(X(XXX))X
            COM(1,0,[0,0,0,0,0,0]), //X
            PTR(98, false),
        ], 
        vec![ // 44 
            COM(5,45,[0,1,4,2,3,4]), //X(XX)(X(XX))
            PTR(43, false),
        ], 
        vec![ // 45 
            COM(5,38,[0,1,4,2,3,4]), //X(XX)X(XX)
            PTR(44, false),
        ], 
        vec![ // 46 
            COM(5,38,[0,1,4,2,3,4]), //X(XX)X(XX)
            PTR(45, false),
        ], 
        vec![ // 47 
            COM(5,45,[0,1,4,2,3,4]), //X(XX)(X(XX))
            PTR(46, false),
            PTR(42, false),
            PTR(98, false),
            PTR(37, false),
        ], 
        vec![ // 48 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(47, false),
            PTR(35, false),
        ], 
        vec![ // 49 
            COM(6,46,[2,0,1,3,4,5]), //XX(XXXX)
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 50 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(49, false),
            PTR(48, false),
        ], 
        vec![ // 51 
            COM(3,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 52 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(1),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 53 
            COM(5,10,[0,1,4,2,3,0]), //X(XX)XX
            PRM(EQ,false),
            PTR(60, false),
            INT(1),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 54 
            COM(6,24,[0,1,5,2,3,4]), //X(XX)XXX
            COM(6,56,[0,1,5,2,3,4]), //X(XXX(XX))
            PTR(53, false),
            PTR(62, false),
            PTR(52, false),
            PTR(61, false),
        ], 
        vec![ // 55 
            COM(5,32,[4,0,2,2,3,1]), //X(XXXX)X
            PTR(54, false),
            PTR(51, false),
        ], 
        vec![ // 56 
            COM(5,5,[4,0,2,1,0,0]), //X(XX)X
            PTR(61, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 57 
            COM(5,10,[0,1,4,2,3,0]), //X(XX)XX
            PRM(EQ,false),
            PTR(60, false),
            INT(0),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 58 
            COM(4,20,[2,0,3,1,3,0]), //X(XX(XX))
            PTR(57, false),
            PTR(56, false),
        ], 
        vec![ // 59 
            COM(5,56,[2,0,3,4,1,4]), //X(XXX(XX))
            PTR(58, false),
            PTR(55, false),
        ], 
         // FUN12NanoPrelude.fst
        vec![ // 60 
            COM(2,1,[1,0,0,0,0,0]), //XX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN13NanoPrelude.snd
        vec![ // 61 
            COM(2,1,[1,0,0,0,0,0]), //XX
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
         // FUN14Sumpuz.bindings
        vec![ // 62 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
            PTR(70, false),
            PTR(64, false),
        ], 
        vec![ // 63 
            COM(4,2,[3,1,0,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 64 
            COM(6,37,[0,5,3,1,2,4]), //XXXX(XX)
            PTR(95, false),
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(63, false),
        ], 
        vec![ // 65 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            PTR(85, false),
            PTR(94, false),
        ], 
        vec![ // 66 
            COM(6,44,[0,1,3,2,4,5]), //X(XX)(XXX)
            PTR(80, false),
            PTR(84, false),
            PTR(65, false),
        ], 
        vec![ // 67 
            COM(5,40,[0,1,2,4,3,4]), //X(XXX)(XX)
            PTR(76, false),
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(79, false),
        ], 
        vec![ // 68 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            PTR(67, false),
            PTR(66, false),
        ], 
        vec![ // 69 
            COM(5,42,[0,2,4,1,3,4]), //XXX(XXX)
            PTR(71, false),
        ], 
        vec![ // 70 
            COM(3,5,[0,1,2,2,0,0]), //X(XX)X
            PTR(69, false),
            PTR(68, false),
        ], 
         // FUN15NanoPrelude.lookup
        vec![ // 71 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(75, false),
        ], 
        vec![ // 72 
            COM(5,14,[0,1,4,2,3,0]), //XXX(XX)
            PRM(EQ,false),
        ], 
        vec![ // 73 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(6,37,[0,2,3,4,1,5]), //XXXX(XX)
            PTR(72, false),
            COM(3,1,[2,0,0,0,0,0]), //XX
        ], 
        vec![ // 74 
            COM(5,18,[3,0,1,2,4,0]), //X(XXXX)
            PTR(73, false),
        ], 
        vec![ // 75 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(74, false),
        ], 
         // FUN16NanoPrelude.map
        vec![ // 76 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(78, false),
        ], 
        vec![ // 77 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 78 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(77, false),
        ], 
         // FUN17Data.List_Type.:
        vec![ // 79 
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
         // FUN18NanoPrelude.zip
        vec![ // 80 
            COM(4,6,[2,0,1,3,0,0]), //XX(XX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(83, false),
        ], 
        vec![ // 81 
            COM(6,44,[0,1,4,2,3,5]), //X(XX)(XXX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 82 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(81, false),
            COM(3,2,[2,0,1,0,0,0]), //XXX
            PTR(80, false),
        ], 
        vec![ // 83 
            COM(5,16,[2,0,1,3,4,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(82, false),
        ], 
         // FUN19NanoPrelude.repeat
        vec![ // 84 
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(84, false),
        ], 
         // FUN20Sumpuz.diff
        vec![ // 85 
            PTR(87, false),
            PTR(86, false),
        ], 
        vec![ // 86 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(89, false),
        ], 
         // FUN21NanoPrelude.foldl
        vec![ // 87 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(88, false),
        ], 
        vec![ // 88 
            COM(5,46,[4,3,0,1,2,3]), //XX(XXXX)
            COM(4,7,[1,0,2,3,0,0]), //X(XXX)
        ], 
         // FUN22Sumpuz.del
        vec![ // 89 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(93, false),
        ], 
        vec![ // 90 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 91 
            COM(5,28,[0,1,3,2,4,4]), //XXX(XX)X
            PRM(EQ,false),
        ], 
        vec![ // 92 
            COM(5,30,[0,2,1,3,4,4]), //XX(XXX)X
            PTR(91, false),
            PTR(90, false),
        ], 
        vec![ // 93 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(92, false),
        ], 
         // FUN23Sumpuz.rng
        vec![ // 94 
            PTR(76, false),
            PTR(61, false),
        ], 
         // FUN24NanoPrelude.elem
        vec![ // 95 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(97, false),
        ], 
        vec![ // 96 
            COM(6,28,[0,2,4,3,5,1]), //XXX(XX)X
            PRM(EQ,false),
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
        vec![ // 97 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(96, false),
        ], 
         // FUN25Sumpuz.ofAll
        vec![ // 98 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(100, false),
        ], 
        vec![ // 99 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            PTR(101, false),
        ], 
        vec![ // 100 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(99, false),
        ], 
         // FUN26Data.List_Type.++
        vec![ // 101 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            Y,
            PTR(103, false),
        ], 
        vec![ // 102 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 103 
            COM(4,6,[3,1,0,2,0,0]), //XX(XX)
            PTR(102, false),
        ], 
         // FUN27Sumpuz.solns
        vec![ // 104 
            COM(6,52,[0,1,2,4,3,5]), //X(X(XX)XX)
            PTR(114, false),
            PTR(108, false),
            PTR(107, false),
        ], 
        vec![ // 105 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(121, false),
        ], 
        vec![ // 106 
            COM(5,40,[0,1,4,2,3,4]), //X(XXX)(XX)
            PRM(ADD,false),
            PTR(121, false),
        ], 
        vec![ // 107 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            PTR(106, false),
            PTR(105, false),
        ], 
        vec![ // 108 
            COM(6,54,[0,1,2,4,5,3]), //X(X(XXX)X)
            PTR(116, false),
            PRM(ADD,false),
        ], 
        vec![ // 109 
            COM(5,5,[4,0,2,1,0,0]), //X(XX)X
            PTR(61, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 110 
            COM(5,11,[0,2,1,4,3,0]), //XX(XX)X
            PTR(62, false),
            PTR(109, false),
        ], 
        vec![ // 111 
            COM(5,20,[0,1,3,2,4,0]), //X(XX(XX))
            PTR(98, false),
            PTR(115, false),
            PTR(60, false),
        ], 
        vec![ // 112 
            COM(5,42,[0,2,4,1,3,4]), //XXX(XXX)
            PTR(111, false),
        ], 
        vec![ // 113 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(112, false),
            PTR(110, false),
        ], 
        vec![ // 114 
            COM(5,37,[0,2,3,4,1,4]), //XXXX(XX)
            PTR(113, false),
        ], 
         // FUN28NanoPrelude.curry
        vec![ // 115 
            COM(4,7,[1,0,2,3,0,0]), //X(XXX)
            COM(3,2,[2,0,1,0,0,0]), //XXX
        ], 
         // FUN29Sumpuz.divMod10
        vec![ // 116 
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(120, false),
            PTR(117, false),
        ], 
        vec![ // 117 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(0),
        ], 
        vec![ // 118 
            COM(5,12,[4,0,2,1,3,0]), //X(XXX)X
            PRM(ADD,false),
            INT(1),
        ], 
        vec![ // 119 
            COM(5,12,[0,1,4,2,3,0]), //X(XXX)X
            PTR(116, false),
            PRM(SUB,false),
            INT(10),
            PTR(118, false),
        ], 
        vec![ // 120 
            COM(4,14,[0,3,1,2,3,0]), //XXX(XX)
            PRM(LE,false),
            INT(9),
            PTR(119, false),
        ], 
         // FUN30Sumpuz.img
        vec![ // 121 
            COM(4,7,[0,1,3,2,0,0]), //X(XXX)
            PTR(122, false),
            PTR(71, false),
        ], 
         // FUN31NanoPrelude.fromJust
        vec![ // 122 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            ERR(4),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN32NanoPrelude.head
        vec![ // 123 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            ERR(3),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN33NanoPrelude.tail
        vec![ // 124 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
         // FUN34NanoPrelude.enumFromTo
        vec![ // 125 
            COM(5,15,[0,1,4,2,3,0]), //X(XX)(XX)
            PTR(127, false),
            PTR(126, false),
            PTR(131, false),
        ], 
        vec![ // 126 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PRM(LE,false),
        ], 
         // FUN35NanoPrelude.takeWhile
        vec![ // 127 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(130, false),
        ], 
        vec![ // 128 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 129 
            COM(5,42,[1,3,0,2,3,4]), //XXX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 130 
            COM(6,48,[5,0,1,3,2,4]), //XX(XX(XX))
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(129, false),
            PTR(128, false),
        ], 
         // FUN36NanoPrelude.enumFrom
        vec![ // 131 
            COM(5,49,[0,4,1,2,4,3]), //XX(X(XXX))
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(131, false),
            PRM(ADD,false),
            INT(1),
        ], 
         // FUN37Sumpuz.ifNull
        vec![ // 132 
            COM(4,6,[1,2,0,3,0,0]), //XX(XX)
            COM(3,0,[0,0,0,0,0,0]), //X
        ], 
    ]
});