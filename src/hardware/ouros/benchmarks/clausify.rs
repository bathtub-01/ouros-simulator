use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 36
// Apps in this file: 126
// Combinators in this file: 203
#[rustfmt::skip]
pub static CLAUSIFY: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0Clausify.main
        vec![ // 0 
            PTR(16, false),
            PTR(15, false),
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
            PTR(123, false),
            PTR(2, false),
            PTR(1, false),
        ], 
        vec![ // 4 
            COM(5,1,[4,0,0,0,0,0]), //XX
            INT(0),
        ], 
        vec![ // 5 
            PTR(123, false),
            PTR(4, false),
            PTR(3, false),
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
            PTR(123, false),
            PTR(7, false),
            PTR(6, false),
        ], 
        vec![ // 9 
            COM(5,1,[4,0,0,0,0,0]), //XX
            INT(0),
        ], 
        vec![ // 10 
            PTR(123, false),
            PTR(9, false),
            PTR(8, false),
        ], 
        vec![ // 11 
            PTR(123, false),
            PTR(10, false),
            PTR(5, false),
        ], 
        vec![ // 12 
            PTR(120, false),
            INT(2),
            PTR(11, false),
        ], 
        vec![ // 13 
            COM(5,1,[4,0,0,0,0,0]), //XX
            INT(0),
        ], 
        vec![ // 14 
            PTR(21, false),
            PTR(98, false),
            PTR(13, false),
            PTR(12, false),
        ], 
        vec![ // 15 
            PTR(24, false),
            PTR(14, false),
        ], 
         // FUN1Clausify.display
        vec![ // 16 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(0),
            PTR(17, false),
        ], 
        vec![ // 17 
            COM(5,13,[0,1,2,4,3,0]), //X(X(XX))X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PRM(ADD,false),
            PTR(18, false),
            PTR(16, false),
        ], 
         // FUN2Clausify.emitClause
        vec![ // 18 
            COM(2,1,[1,0,0,0,0,0]), //XX
            PTR(19, false),
        ], 
        vec![ // 19 
            COM(4,13,[0,1,2,3,2,0]), //X(X(XX))X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PRM(ADD,false),
            PTR(20, false),
        ], 
         // FUN3NanoPrelude.sum
        vec![ // 20 
            PTR(21, false),
            PRM(ADD,false),
            INT(0),
        ], 
         // FUN4NanoPrelude.foldr
        vec![ // 21 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            Y,
            PTR(23, false),
        ], 
        vec![ // 22 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
        ], 
        vec![ // 23 
            COM(5,16,[4,2,0,1,3,0]), //XX(XXX)
            PTR(22, false),
        ], 
         // FUN5Clausify.clausify
        vec![ // 24 
            COM(4,8,[0,1,2,3,0,0]), //X(X(XX))
            PTR(26, false),
            PTR(57, false),
            PTR(25, false),
        ], 
        vec![ // 25 
            COM(5,22,[0,1,2,3,4,0]), //X(X(X(XX)))
            PTR(64, false),
            PTR(85, false),
            PTR(95, false),
            PTR(111, false),
        ], 
         // FUN6Clausify.uniq
        vec![ // 26 
            PTR(21, false),
            PTR(28, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 27 
            PTR(29, false),
            PTR(44, false),
        ], 
        vec![ // 28 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(27, false),
            PTR(56, false),
        ], 
         // FUN7Clausify.union
        vec![ // 29 
            COM(4,8,[0,1,2,3,0,0]), //X(X(XX))
            PTR(32, false),
            PTR(31, false),
            PTR(40, false),
        ], 
        vec![ // 30 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(39, false),
        ], 
        vec![ // 31 
            COM(4,8,[0,1,2,3,0,0]), //X(X(XX))
            PTR(36, false),
            PTR(30, false),
        ], 
        vec![ // 32 
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(33, false),
        ], 
         // FUN8Data.List_Type.++
        vec![ // 33 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            Y,
            PTR(35, false),
        ], 
        vec![ // 34 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 35 
            COM(4,6,[3,1,0,2,0,0]), //XX(XX)
            PTR(34, false),
        ], 
         // FUN9NanoPrelude.filter
        vec![ // 36 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(38, false),
        ], 
        vec![ // 37 
            COM(5,38,[0,2,4,3,1,4]), //X(XX)X(XX)
            COM(4,45,[0,1,3,2,1,3]), //X(XX)(X(XX))
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 38 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(37, false),
        ], 
         // FUN10Data.Bool.not
        vec![ // 39 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN11Clausify.contains
        vec![ // 40 
            COM(4,8,[0,1,2,3,0,0]), //X(X(XX))
            Y,
            PTR(42, false),
            PTR(41, false),
        ], 
        vec![ // 41 
            COM(6,35,[0,1,2,5,4,3]), //X(X(XXX))X
            COM(4,7,[0,1,3,2,0,0]), //X(XXX)
            PTR(43, false),
        ], 
        vec![ // 42 
            COM(6,45,[0,5,1,2,3,4]), //X(XX)(X(XX))
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(3,4,[0,1,2,1,0,0]), //XXXX
        ], 
         // FUN12Data.Bool.||
        vec![ // 43 
            COM(3,2,[1,2,0,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
         // FUN13Clausify.eqClause
        vec![ // 44 
            COM(3,3,[1,0,2,0,0,0]), //X(XX)
            PTR(47, false),
        ], 
        vec![ // 45 
            PTR(49, false),
            PRM(EQ,false),
        ], 
        vec![ // 46 
            COM(5,13,[0,1,2,4,3,0]), //X(X(XX))X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(48, false),
        ], 
        vec![ // 47 
            COM(5,31,[0,3,1,2,4,2]), //XX(X(XX))X
            COM(4,8,[0,1,2,3,0,0]), //X(X(XX))
            PTR(46, false),
            PTR(45, false),
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
            PTR(55, false),
        ], 
        vec![ // 50 
            COM(5,13,[0,1,2,4,3,0]), //X(X(XX))X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(48, false),
        ], 
        vec![ // 51 
            COM(6,25,[0,1,5,2,3,4]), //XX(XX)XX
            COM(6,31,[0,1,2,3,5,4]), //XX(X(XX))X
            COM(4,8,[0,1,2,3,0,0]), //X(X(XX))
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(50, false),
        ], 
        vec![ // 52 
            COM(4,4,[0,1,3,2,0,0]), //XXXX
            PTR(51, false),
        ], 
        vec![ // 53 
            COM(3,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 54 
            COM(5,40,[3,4,0,1,2,4]), //X(XXX)(XX)
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(53, false),
        ], 
        vec![ // 55 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            PTR(54, false),
            PTR(52, false),
        ], 
         // FUN16Clausify.singleton
        vec![ // 56 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN17Clausify.nonTaut
        vec![ // 57 
            PTR(36, false),
            PTR(58, false),
        ], 
         // FUN18Clausify.notTaut
        vec![ // 58 
            COM(2,1,[1,0,0,0,0,0]), //XX
            PTR(60, false),
        ], 
        vec![ // 59 
            PTR(63, false),
            PRM(EQ,false),
        ], 
        vec![ // 60 
            COM(4,6,[0,1,2,3,0,0]), //XX(XX)
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(61, false),
            PTR(59, false),
        ], 
         // FUN19NanoPrelude.null
        vec![ // 61 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(62, false),
        ], 
        vec![ // 62 
            COM(3,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN20Clausify.inter
        vec![ // 63 
            COM(4,6,[0,1,2,3,0,0]), //XX(XX)
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(36, false),
            PTR(40, false),
        ], 
         // FUN21Clausify.clauses
        vec![ // 64 
            PTR(67, false),
            PTR(66, false),
        ], 
        vec![ // 65 
            COM(2,2,[1,0,0,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 66 
            PTR(70, false),
            PTR(65, false),
        ], 
         // FUN22NanoPrelude.map
        vec![ // 67 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(69, false),
        ], 
        vec![ // 68 
            COM(5,13,[0,1,2,4,3,0]), //X(X(XX))X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 69 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(68, false),
        ], 
         // FUN23Clausify.clause
        vec![ // 70 
            COM(6,54,[4,0,1,5,3,2]), //X(X(XXX)X)
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
            PTR(79, false),
            PTR(72, false),
            PTR(71, false),
        ], 
        vec![ // 71 
            COM(2,2,[1,0,0,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 72 
            COM(5,12,[0,1,4,2,3,0]), //X(XXX)X
            COM(3,2,[2,0,1,0,0,0]), //XXX
            PTR(80, false),
        ], 
        vec![ // 73 
            COM(5,16,[0,2,1,4,3,0]), //XX(XXX)
            COM(3,2,[2,0,1,0,0,0]), //XXX
            PTR(80, false),
        ], 
        vec![ // 74 
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            COM(5,37,[4,0,0,1,2,3]), //XXXX(XX)
            COM(3,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 75 
            COM(5,49,[0,1,1,2,3,4]), //XX(X(XXX))
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(70, false),
            COM(3,2,[2,0,1,0,0,0]), //XXX
        ], 
        vec![ // 76 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(5,39,[0,1,2,4,3,4]), //XX(XX)(XX)
            COM(3,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 77 
            COM(5,38,[0,1,4,2,3,4]), //X(XX)X(XX)
            COM(5,45,[0,1,4,2,3,4]), //X(XX)(X(XX))
        ], 
        vec![ // 78 
            COM(5,10,[0,1,4,2,3,0]), //X(XX)XX
            PTR(77, false),
            PTR(76, false),
            PTR(75, false),
            PTR(74, false),
        ], 
        vec![ // 79 
            COM(4,4,[0,2,3,1,0,0]), //XXXX
            PTR(78, false),
            PTR(73, false),
        ], 
         // FUN24Clausify.insert
        vec![ // 80 
            COM(4,20,[0,1,3,2,3,0]), //X(XX(XX))
            Y,
            PTR(84, false),
            PTR(83, false),
        ], 
        vec![ // 81 
            COM(5,11,[0,1,2,4,3,0]), //XX(XX)X
            COM(5,29,[0,1,4,2,4,3]), //X(XX)(XX)X
            COM(5,41,[0,1,2,4,3,4]), //X(X(XX))(XX)
            PRM(LE,false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 82 
            COM(5,11,[0,1,2,4,3,0]), //XX(XX)X
            COM(6,24,[0,1,5,2,3,4]), //X(XX)XXX
            COM(5,48,[0,4,1,2,3,4]), //XX(XX(XX))
            PTR(81, false),
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
        ], 
        vec![ // 83 
            COM(4,11,[0,3,1,3,2,0]), //XX(XX)X
            PTR(82, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 84 
            COM(5,16,[0,1,2,4,3,0]), //XX(XXX)
            COM(4,6,[0,1,2,3,0,0]), //XX(XX)
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN25Clausify.split
        vec![ // 85 
            PTR(86, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN26Clausify.spl
        vec![ // 86 
            COM(4,20,[0,1,3,2,3,0]), //X(XX(XX))
            Y,
            PTR(92, false),
            PTR(87, false),
        ], 
        vec![ // 87 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(5,1,[4,0,0,0,0,0]), //XX
        ], 
        vec![ // 88 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(94, false),
        ], 
        vec![ // 89 
            COM(5,11,[0,1,2,4,3,0]), //XX(XX)X
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(93, false),
        ], 
        vec![ // 90 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(86, false),
        ], 
        vec![ // 91 
            COM(6,24,[5,0,4,1,2,3]), //X(XX)XXX
            PTR(90, false),
        ], 
        vec![ // 92 
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            PTR(91, false),
            PTR(89, false),
            PTR(88, false),
        ], 
         // FUN27Clausify.Dis
        vec![ // 93 
            COM(6,16,[0,1,5,2,3,0]), //XX(XXX)
            COM(3,1,[0,1,0,0,0,0]), //XX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN28Clausify.Neg
        vec![ // 94 
            COM(5,3,[0,4,1,0,0,0]), //X(XX)
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN29Clausify.disin
        vec![ // 95 
            COM(5,9,[4,0,1,2,3,0]), //XXXXX
            PTR(97, false),
            PTR(96, false),
            PTR(94, false),
            COM(5,1,[4,0,0,0,0,0]), //XX
        ], 
        vec![ // 96 
            COM(4,13,[0,1,2,3,2,0]), //X(X(XX))X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(99, false),
            PTR(95, false),
        ], 
        vec![ // 97 
            COM(4,13,[0,1,2,3,2,0]), //X(X(XX))X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(98, false),
            PTR(95, false),
        ], 
         // FUN30Clausify.Con
        vec![ // 98 
            COM(5,58,[0,1,0,4,2,3]), //X(XX(XXX))
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(3,1,[0,1,0,0,0,0]), //XX
        ], 
         // FUN31Clausify.din
        vec![ // 99 
            COM(5,10,[0,1,4,2,3,0]), //X(XX)XX
            COM(4,39,[0,3,1,3,2,3]), //XX(XX)(XX)
            PTR(105, false),
            PTR(101, false),
            PTR(100, false),
        ], 
        vec![ // 100 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(106, false),
            COM(5,1,[4,0,0,0,0,0]), //XX
        ], 
        vec![ // 101 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(106, false),
            PTR(94, false),
        ], 
        vec![ // 102 
            COM(5,11,[0,1,2,4,3,0]), //XX(XX)X
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(106, false),
            PTR(93, false),
        ], 
        vec![ // 103 
            COM(5,35,[0,1,2,4,3,2]), //X(X(XXX))X
            COM(4,7,[0,1,3,2,0,0]), //X(XXX)
            PTR(98, false),
            PTR(99, false),
        ], 
        vec![ // 104 
            COM(3,4,[0,1,2,1,0,0]), //XXXX
            PTR(103, false),
        ], 
        vec![ // 105 
            COM(4,15,[2,0,3,1,3,0]), //X(XX)(XX)
            PTR(104, false),
            PTR(102, false),
        ], 
         // FUN32Clausify.din2
        vec![ // 106 
            COM(5,47,[0,4,1,2,4,3]), //XX(X(XX)X)
            PTR(110, false),
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(93, false),
            COM(5,1,[4,0,0,0,0,0]), //XX
        ], 
        vec![ // 107 
            COM(5,11,[0,1,2,4,3,0]), //XX(XX)X
            COM(4,6,[0,1,2,3,0,0]), //XX(XX)
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(93, false),
            PTR(93, false),
        ], 
        vec![ // 108 
            COM(4,13,[0,1,2,3,2,0]), //X(X(XX))X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(98, false),
        ], 
        vec![ // 109 
            COM(5,41,[0,1,2,4,3,4]), //X(X(XX))(XX)
            COM(5,9,[4,0,1,2,3,0]), //XXXXX
            PTR(108, false),
            PTR(99, false),
            PTR(107, false),
        ], 
        vec![ // 110 
            COM(5,47,[0,4,1,2,4,3]), //XX(X(XX)X)
            PTR(109, false),
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(93, false),
            PTR(94, false),
        ], 
         // FUN33Clausify.negin
        vec![ // 111 
            COM(5,9,[4,0,1,2,3,0]), //XXXXX
            PTR(119, false),
            PTR(118, false),
            PTR(117, false),
            COM(5,1,[4,0,0,0,0,0]), //XX
        ], 
        vec![ // 112 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(94, false),
            COM(5,1,[4,0,0,0,0,0]), //XX
        ], 
        vec![ // 113 
            COM(5,36,[0,1,2,3,4,2]), //X(X(X(XX)))X
            COM(4,8,[0,1,2,3,0,0]), //X(X(XX))
            PTR(98, false),
            PTR(111, false),
            PTR(94, false),
        ], 
        vec![ // 114 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(113, false),
            PTR(94, false),
        ], 
        vec![ // 115 
            COM(5,36,[0,1,2,3,4,2]), //X(X(X(XX)))X
            COM(4,8,[0,1,2,3,0,0]), //X(X(XX))
            PTR(93, false),
            PTR(111, false),
            PTR(94, false),
        ], 
        vec![ // 116 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(115, false),
            PTR(94, false),
        ], 
        vec![ // 117 
            COM(5,9,[4,0,1,2,3,0]), //XXXXX
            PTR(116, false),
            PTR(114, false),
            PTR(111, false),
            PTR(112, false),
        ], 
        vec![ // 118 
            COM(4,13,[0,1,2,3,2,0]), //X(X(XX))X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(93, false),
            PTR(111, false),
        ], 
        vec![ // 119 
            COM(4,13,[0,1,2,3,2,0]), //X(X(XX))X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(98, false),
            PTR(111, false),
        ], 
         // FUN34NanoPrelude.replicate
        vec![ // 120 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(122, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 121 
            COM(5,12,[0,1,4,2,3,0]), //X(XXX)X
            COM(5,34,[0,1,4,2,4,3]), //X(XX(XX))X
            PRM(LE,false),
            INT(0),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 122 
            COM(5,49,[0,4,1,2,4,3]), //XX(X(XXX))
            PTR(121, false),
            PTR(120, false),
            PRM(SUB,false),
            INT(1),
        ], 
         // FUN35Clausify.eqv
        vec![ // 123 
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(125, false),
            PTR(124, false),
        ], 
        vec![ // 124 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(93, false),
            PTR(94, false),
        ], 
        vec![ // 125 
            COM(5,17,[0,1,2,3,4,0]), //XX(X(XX))
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            PTR(98, false),
            PTR(93, false),
            PTR(94, false),
        ], 
    ]
});