use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 38
// Apps in this file: 118
// Combinators in this file: 219
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
            COM(5,11,[0,1,2,4,3,0]), //XX(XX)X
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(11, false),
            PTR(15, false),
        ], 
         // FUN2Sumpuz.sumMap
        vec![ // 11 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(3,2,[0,2,1,0,0,0]), //XXX
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
            COM(6,47,[0,3,1,2,5,4]), //XX(X(XX)X)
            COM(3,2,[0,2,1,0,0,0]), //XXX
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
            COM(4,6,[0,1,2,3,0,0]), //XX(XX)
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(11, false),
            PTR(17, false),
        ], 
         // FUN6Sumpuz.fz
        vec![ // 17 
            COM(6,25,[0,1,2,5,3,4]), //XX(XX)XX
            COM(5,10,[0,1,4,2,3,0]), //X(XX)XX
            COM(4,4,[0,3,1,2,0,0]), //XXXX
            PTR(18, false),
            INT(0),
            INT(1),
        ], 
         // FUN7Sumpuz.valid
        vec![ // 18 
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(24, false),
            PTR(22, false),
        ], 
        vec![ // 19 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(0),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 20 
            COM(6,31,[0,1,2,3,5,4]), //XX(X(XX))X
            COM(5,41,[0,1,2,4,3,4]), //X(X(XX))(XX)
            PTR(25, false),
            PRM(EQ,false),
            PTR(26, false),
            PTR(26, false),
        ], 
        vec![ // 21 
            COM(5,10,[0,1,4,2,3,0]), //X(XX)XX
            COM(6,53,[0,1,2,3,5,4]), //X(XX(XX)X)
            PTR(20, false),
            COM(4,7,[0,1,3,2,0,0]), //X(XXX)
            PTR(29, false),
        ], 
        vec![ // 22 
            COM(4,11,[0,3,1,3,2,0]), //XX(XX)X
            PTR(21, false),
            PTR(32, false),
            PTR(19, false),
        ], 
        vec![ // 23 
            COM(5,22,[0,1,2,3,4,0]), //X(X(X(XX)))
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(25, false),
        ], 
        vec![ // 24 
            COM(6,55,[0,1,2,3,5,4]), //X(X(X(XX))X)
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(23, false),
            PRM(EQ,false),
            PTR(26, false),
            PTR(26, false),
        ], 
         // FUN8Data.Bool.&&
        vec![ // 25 
            COM(2,1,[1,0,0,0,0,0]), //XX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN9NanoPrelude.length
        vec![ // 26 
            Y,
            PTR(28, false),
            INT(0),
        ], 
        vec![ // 27 
            COM(6,49,[0,1,4,2,5,3]), //XX(X(XXX))
            COM(4,6,[3,2,0,1,0,0]), //XX(XX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PRM(ADD,false),
            INT(1),
        ], 
        vec![ // 28 
            COM(3,4,[0,1,2,2,0,0]), //XXXX
            PTR(27, false),
        ], 
         // FUN10Sumpuz.isSingleton
        vec![ // 29 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(31, false),
        ], 
        vec![ // 30 
            COM(3,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 31 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(30, false),
        ], 
         // FUN11Sumpuz.solutions
        vec![ // 32 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(51, false),
            PTR(42, false),
        ], 
        vec![ // 33 
            COM(6,32,[0,1,5,2,3,4]), //X(XXXX)X
            PTR(109, false),
            PTR(117, false),
            INT(1),
            INT(0),
            INT(9),
        ], 
        vec![ // 34 
            COM(6,52,[0,1,2,5,3,4]), //X(X(XX)XX)
            PTR(109, false),
            PTR(117, false),
            PTR(108, false),
            INT(1),
            INT(0),
        ], 
        vec![ // 35 
            COM(5,44,[0,1,4,2,4,3]), //X(XX)(XXX)
            PTR(54, false),
            PTR(107, false),
            PTR(34, false),
            INT(9),
        ], 
        vec![ // 36 
            COM(6,31,[0,1,2,3,5,4]), //XX(X(XX))X
            COM(6,25,[0,1,2,5,3,4]), //XX(XX)XX
            COM(5,19,[0,1,2,4,3,0]), //X(X(XX)X)
            PTR(84, false),
            PTR(35, false),
            PTR(54, false),
        ], 
        vec![ // 37 
            COM(5,12,[0,1,4,2,3,0]), //X(XXX)X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(36, false),
            PTR(33, false),
            PTR(53, false),
        ], 
        vec![ // 38 
            COM(6,36,[0,1,2,3,5,4]), //X(X(X(XX)))X
            COM(1,0,[0,0,0,0,0,0]), //X
            PTR(84, false),
        ], 
        vec![ // 39 
            COM(6,58,[0,1,2,3,5,4]), //X(XX(XXX))
            COM(5,40,[0,1,4,2,3,4]), //X(XXX)(XX)
            COM(5,10,[0,1,4,2,3,0]), //X(XX)XX
            PTR(38, false),
        ], 
        vec![ // 40 
            COM(6,55,[0,1,2,3,5,4]), //X(X(X(XX))X)
            COM(5,37,[0,4,1,2,3,4]), //XXXX(XX)
            PTR(39, false),
            PTR(90, false),
            PTR(52, false),
        ], 
        vec![ // 41 
            COM(5,13,[0,1,2,4,3,0]), //X(X(XX))X
            COM(5,37,[0,4,1,2,3,4]), //XXXX(XX)
            PTR(40, false),
            PTR(107, false),
            PTR(32, false),
        ], 
        vec![ // 42 
            COM(4,39,[0,3,1,3,2,3]), //XX(XX)(XX)
            PTR(41, false),
            PTR(108, false),
            PTR(37, false),
        ], 
        vec![ // 43 
            COM(4,6,[0,1,3,2,0,0]), //XX(XX)
            COM(4,6,[0,1,2,3,0,0]), //XX(XX)
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 44 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(1),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 45 
            COM(5,10,[0,1,4,2,3,0]), //X(XX)XX
            PRM(EQ,false),
            PTR(52, false),
            INT(1),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 46 
            COM(6,25,[0,1,2,5,3,4]), //XX(XX)XX
            COM(6,60,[0,1,2,5,3,4]), //X(X(XXXX))
            COM(3,2,[2,0,1,0,0,0]), //XXX
            PTR(45, false),
            PTR(54, false),
            PTR(44, false),
        ], 
        vec![ // 47 
            COM(5,34,[0,1,4,2,4,3]), //X(XX(XX))X
            COM(4,6,[0,3,1,2,0,0]), //XX(XX)
            PTR(46, false),
            PTR(53, false),
            COM(3,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 48 
            COM(5,10,[0,1,4,2,3,0]), //X(XX)XX
            PRM(EQ,false),
            PTR(52, false),
            INT(0),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 49 
            COM(5,47,[0,4,1,2,4,3]), //XX(X(XX)X)
            PTR(48, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(53, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 50 
            COM(5,44,[3,0,4,1,4,2]), //X(XX)(XXX)
            PTR(49, false),
            PTR(47, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 51 
            COM(5,39,[0,3,1,4,2,4]), //XX(XX)(XX)
            COM(5,45,[0,1,4,2,3,4]), //X(XX)(X(XX))
            PTR(50, false),
            PTR(43, false),
        ], 
         // FUN12NanoPrelude.fst
        vec![ // 52 
            COM(2,1,[1,0,0,0,0,0]), //XX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN13NanoPrelude.snd
        vec![ // 53 
            COM(2,1,[1,0,0,0,0,0]), //XX
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
         // FUN14Sumpuz.bindings
        vec![ // 54 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(61, false),
            PTR(56, false),
        ], 
        vec![ // 55 
            COM(4,2,[3,1,0,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 56 
            COM(6,37,[0,5,3,1,2,4]), //XXXX(XX)
            PTR(81, false),
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(55, false),
        ], 
        vec![ // 57 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(68, false),
        ], 
        vec![ // 58 
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            PTR(65, false),
            PTR(57, false),
        ], 
        vec![ // 59 
            COM(6,53,[0,1,2,3,5,4]), //X(XX(XX)X)
            PTR(58, false),
            COM(4,8,[0,1,2,3,0,0]), //X(X(XX))
        ], 
        vec![ // 60 
            COM(6,27,[0,1,2,5,3,4]), //X(X(XX))XX
            PTR(59, false),
            PTR(69, false),
            PTR(72, false),
            PTR(73, false),
            PTR(80, false),
        ], 
        vec![ // 61 
            COM(5,39,[0,1,2,4,3,4]), //XX(XX)(XX)
            COM(5,39,[0,1,2,4,3,4]), //XX(XX)(XX)
            COM(4,39,[0,3,1,3,2,3]), //XX(XX)(XX)
            PTR(62, false),
            PTR(60, false),
        ], 
         // FUN15NanoPrelude.lookup
        vec![ // 62 
            COM(4,8,[0,1,2,3,0,0]), //X(X(XX))
            Y,
            PTR(64, false),
            PTR(63, false),
        ], 
        vec![ // 63 
            COM(6,34,[0,1,2,3,5,4]), //X(XX(XX))X
            COM(5,19,[3,0,2,4,1,0]), //X(X(XX)X)
            COM(5,12,[0,1,4,2,3,0]), //X(XXX)X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PRM(EQ,false),
            COM(3,1,[2,0,0,0,0,0]), //XX
        ], 
        vec![ // 64 
            COM(4,6,[0,1,2,3,0,0]), //XX(XX)
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN16NanoPrelude.map
        vec![ // 65 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(67, false),
        ], 
        vec![ // 66 
            COM(5,13,[0,1,2,4,3,0]), //X(X(XX))X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 67 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(66, false),
        ], 
         // FUN17Data.List_Type.:
        vec![ // 68 
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
         // FUN18NanoPrelude.zip
        vec![ // 69 
            COM(4,5,[0,3,1,2,0,0]), //X(XX)X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(71, false),
        ], 
        vec![ // 70 
            COM(6,30,[0,1,2,3,5,4]), //XX(XXX)X
            COM(4,6,[0,1,2,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(3,2,[2,0,1,0,0,0]), //XXX
        ], 
        vec![ // 71 
            COM(6,25,[0,1,5,2,3,4]), //XX(XX)XX
            COM(5,11,[0,1,2,4,3,0]), //XX(XX)X
            COM(4,8,[0,1,2,3,0,0]), //X(X(XX))
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(70, false),
            PTR(69, false),
        ], 
         // FUN19NanoPrelude.repeat
        vec![ // 72 
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(72, false),
        ], 
         // FUN20Sumpuz.diff
        vec![ // 73 
            PTR(75, false),
            PTR(74, false),
        ], 
        vec![ // 74 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(77, false),
        ], 
         // FUN21NanoPrelude.foldl
        vec![ // 75 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(76, false),
        ], 
        vec![ // 76 
            COM(5,34,[0,1,3,2,4,4]), //X(XX(XX))X
            COM(3,2,[2,1,0,0,0,0]), //XXX
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
        ], 
         // FUN22Sumpuz.del
        vec![ // 77 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(79, false),
        ], 
        vec![ // 78 
            COM(4,6,[0,1,2,3,0,0]), //XX(XX)
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 79 
            COM(6,53,[0,1,2,3,5,4]), //X(XX(XX)X)
            PTR(78, false),
            COM(5,29,[0,1,4,2,4,3]), //X(XX)(XX)X
            COM(4,13,[0,1,2,3,3,0]), //X(X(XX))X
            PRM(EQ,false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
         // FUN23Sumpuz.rng
        vec![ // 80 
            PTR(65, false),
            PTR(53, false),
        ], 
         // FUN24NanoPrelude.elem
        vec![ // 81 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(83, false),
        ], 
        vec![ // 82 
            COM(4,6,[0,1,2,3,0,0]), //XX(XX)
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 83 
            COM(6,53,[0,1,2,3,5,4]), //X(XX(XX)X)
            PTR(82, false),
            COM(5,10,[0,1,4,3,2,0]), //X(XX)XX
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PRM(EQ,false),
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
         // FUN25Sumpuz.ofAll
        vec![ // 84 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(86, false),
        ], 
        vec![ // 85 
            COM(5,13,[0,1,2,4,3,0]), //X(X(XX))X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(87, false),
        ], 
        vec![ // 86 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(85, false),
        ], 
         // FUN26Data.List_Type.++
        vec![ // 87 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            Y,
            PTR(89, false),
        ], 
        vec![ // 88 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 89 
            COM(4,6,[3,1,0,2,0,0]), //XX(XX)
            PTR(88, false),
        ], 
         // FUN27Sumpuz.solns
        vec![ // 90 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            PTR(98, false),
            PTR(95, false),
        ], 
        vec![ // 91 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(105, false),
        ], 
        vec![ // 92 
            COM(5,40,[0,1,4,2,3,4]), //X(XXX)(XX)
            PRM(ADD,false),
            PTR(105, false),
        ], 
        vec![ // 93 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            PTR(92, false),
            PTR(91, false),
        ], 
        vec![ // 94 
            COM(6,54,[0,1,2,4,5,3]), //X(X(XXX)X)
            PTR(100, false),
            PRM(ADD,false),
        ], 
        vec![ // 95 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(94, false),
            PTR(93, false),
        ], 
        vec![ // 96 
            COM(6,24,[0,1,5,2,3,4]), //X(XX)XXX
            COM(6,33,[0,1,2,5,3,4]), //X(X(XX)X)X
            PTR(54, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(53, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 97 
            COM(6,53,[0,1,2,3,5,4]), //X(XX(XX)X)
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            COM(5,41,[0,1,2,4,3,4]), //X(X(XX))(XX)
            PTR(84, false),
            PTR(99, false),
            PTR(52, false),
        ], 
        vec![ // 98 
            COM(6,39,[0,1,2,5,3,4]), //XX(XX)(XX)
            COM(4,4,[0,3,1,2,0,0]), //XXXX
            PTR(97, false),
            PTR(96, false),
        ], 
         // FUN28NanoPrelude.curry
        vec![ // 99 
            COM(4,7,[1,0,2,3,0,0]), //X(XXX)
            COM(3,2,[2,0,1,0,0,0]), //XXX
        ], 
         // FUN29Sumpuz.divMod10
        vec![ // 100 
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(104, false),
            PTR(101, false),
        ], 
        vec![ // 101 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(0),
        ], 
        vec![ // 102 
            COM(4,7,[0,1,3,2,0,0]), //X(XXX)
            COM(3,2,[2,0,1,0,0,0]), //XXX
            PRM(ADD,false),
            INT(1),
        ], 
        vec![ // 103 
            COM(5,12,[0,1,4,2,3,0]), //X(XXX)X
            PTR(100, false),
            PRM(SUB,false),
            INT(10),
            PTR(102, false),
        ], 
        vec![ // 104 
            COM(4,14,[0,3,1,2,3,0]), //XXX(XX)
            PRM(LE,false),
            INT(9),
            PTR(103, false),
        ], 
         // FUN30Sumpuz.img
        vec![ // 105 
            COM(4,7,[0,1,3,2,0,0]), //X(XXX)
            PTR(106, false),
            PTR(62, false),
        ], 
         // FUN31NanoPrelude.fromJust
        vec![ // 106 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            ERR(4),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN32NanoPrelude.head
        vec![ // 107 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            ERR(3),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN33NanoPrelude.tail
        vec![ // 108 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
         // FUN34NanoPrelude.enumFromTo
        vec![ // 109 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(111, false),
            PTR(116, false),
        ], 
        vec![ // 110 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PRM(LE,false),
        ], 
        vec![ // 111 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(112, false),
            PTR(110, false),
        ], 
         // FUN35NanoPrelude.takeWhile
        vec![ // 112 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(115, false),
        ], 
        vec![ // 113 
            COM(5,40,[0,3,4,1,2,4]), //X(XXX)(XX)
            COM(4,8,[0,1,2,3,0,0]), //X(X(XX))
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 114 
            COM(4,4,[0,1,3,2,0,0]), //XXXX
            PTR(113, false),
        ], 
        vec![ // 115 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(114, false),
        ], 
         // FUN36NanoPrelude.enumFrom
        vec![ // 116 
            COM(5,49,[0,4,1,2,4,3]), //XX(X(XXX))
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(116, false),
            PRM(ADD,false),
            INT(1),
        ], 
         // FUN37Sumpuz.ifNull
        vec![ // 117 
            COM(4,5,[0,2,3,1,0,0]), //X(XX)X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            COM(3,0,[0,0,0,0,0,0]), //X
        ], 
    ]
});