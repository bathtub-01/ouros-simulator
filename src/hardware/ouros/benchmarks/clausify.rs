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
            PTR(125, false),
            PTR(2, false),
            PTR(1, false),
        ], 
        vec![ // 4 
            COM(5,1,[4,0,0,0,0,0]), //XX
            INT(0),
        ], 
        vec![ // 5 
            PTR(125, false),
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
            PTR(125, false),
            PTR(7, false),
            PTR(6, false),
        ], 
        vec![ // 9 
            COM(5,1,[4,0,0,0,0,0]), //XX
            INT(0),
        ], 
        vec![ // 10 
            PTR(125, false),
            PTR(9, false),
            PTR(8, false),
        ], 
        vec![ // 11 
            PTR(125, false),
            PTR(10, false),
            PTR(5, false),
        ], 
        vec![ // 12 
            PTR(121, false),
            INT(2),
            PTR(11, false),
        ], 
        vec![ // 13 
            COM(5,1,[4,0,0,0,0,0]), //XX
            INT(0),
        ], 
        vec![ // 14 
            PTR(21, false),
            COM(6,2,[2,0,1,0,0,0]), //XXX
            PTR(13, false),
            PTR(12, false),
        ], 
        vec![ // 15 
            PTR(23, false),
            PTR(14, false),
        ], 
         // FUN1Clausify.display
        vec![ // 16 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(0),
            PTR(17, false),
        ], 
        vec![ // 17 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
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
            COM(4,15,[0,1,2,1,3,0]), //X(XX)(XX)
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
            PTR(22, false),
        ], 
        vec![ // 22 
            COM(5,16,[4,2,0,1,3,0]), //XX(XXX)
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
        ], 
         // FUN5Clausify.clausify
        vec![ // 23 
            COM(4,6,[0,1,2,3,0,0]), //XX(XX)
            COM(1,0,[0,0,0,0,0,0]), //X
            PTR(26, false),
            PTR(25, false),
        ], 
        vec![ // 24 
            COM(6,48,[0,1,2,3,4,5]), //XX(XX(XX))
            COM(1,0,[0,0,0,0,0,0]), //X
            PTR(85, false),
            COM(1,0,[0,0,0,0,0,0]), //X
            PTR(93, false),
            PTR(112, false),
        ], 
        vec![ // 25 
            COM(6,48,[0,1,2,3,4,5]), //XX(XX(XX))
            COM(1,0,[0,0,0,0,0,0]), //X
            PTR(56, false),
            COM(1,0,[0,0,0,0,0,0]), //X
            PTR(62, false),
            PTR(24, false),
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
            PTR(55, false),
        ], 
         // FUN7Clausify.union
        vec![ // 29 
            COM(5,46,[0,3,1,2,3,4]), //XX(XXXX)
            PTR(31, false),
            PTR(30, false),
        ], 
        vec![ // 30 
            COM(6,58,[0,1,2,3,4,5]), //X(XX(XXX))
            PTR(34, false),
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(38, false),
            PTR(39, false),
        ], 
         // FUN8Data.List_Type.++
        vec![ // 31 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            Y,
            PTR(33, false),
        ], 
        vec![ // 32 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 33 
            COM(4,6,[3,1,0,2,0,0]), //XX(XX)
            PTR(32, false),
        ], 
         // FUN9NanoPrelude.filter
        vec![ // 34 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(37, false),
        ], 
        vec![ // 35 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 36 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(4,30,[0,2,1,2,3,3]), //XX(XXX)X
            COM(5,39,[0,3,1,4,2,4]), //XX(XX)(XX)
            PTR(35, false),
        ], 
        vec![ // 37 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(36, false),
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
            PTR(42, false),
        ], 
        vec![ // 40 
            COM(6,40,[0,1,4,3,2,5]), //X(XXX)(XX)
            PTR(43, false),
        ], 
        vec![ // 41 
            COM(5,47,[3,0,1,2,4,4]), //XX(X(XX)X)
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 42 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            PTR(41, false),
            PTR(40, false),
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
            PTR(47, false),
        ], 
        vec![ // 45 
            COM(5,18,[0,1,2,3,4,0]), //X(XXXX)
            PTR(48, false),
            PTR(49, false),
            PRM(EQ,false),
        ], 
        vec![ // 46 
            COM(5,10,[0,1,4,2,3,0]), //X(XX)XX
            COM(6,46,[0,4,1,2,3,5]), //XX(XXXX)
            PTR(45, false),
            PTR(49, false),
            PRM(EQ,false),
        ], 
        vec![ // 47 
            COM(4,7,[1,0,2,3,0,0]), //X(XXX)
            PTR(46, false),
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
            PTR(54, false),
        ], 
        vec![ // 50 
            COM(6,40,[0,1,2,4,3,5]), //X(XXX)(XX)
            PTR(48, false),
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
            PTR(52, false),
        ], 
        vec![ // 54 
            COM(5,19,[0,1,2,3,4,0]), //X(X(XX)X)
            PTR(53, false),
            PTR(51, false),
            PTR(50, false),
        ], 
         // FUN16Clausify.singleton
        vec![ // 55 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN17Clausify.nonTaut
        vec![ // 56 
            PTR(34, false),
            PTR(57, false),
        ], 
         // FUN18Clausify.notTaut
        vec![ // 57 
            COM(2,1,[1,0,0,0,0,0]), //XX
            PTR(58, false),
        ], 
        vec![ // 58 
            COM(5,18,[0,1,2,3,4,0]), //X(XXXX)
            PTR(59, false),
            PTR(61, false),
            PRM(EQ,false),
        ], 
         // FUN19NanoPrelude.null
        vec![ // 59 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(60, false),
        ], 
        vec![ // 60 
            COM(3,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN20Clausify.inter
        vec![ // 61 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            PTR(34, false),
            PTR(39, false),
        ], 
         // FUN21Clausify.clauses
        vec![ // 62 
            PTR(65, false),
            PTR(64, false),
        ], 
        vec![ // 63 
            COM(2,2,[1,0,0,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 64 
            PTR(68, false),
            PTR(63, false),
        ], 
         // FUN22NanoPrelude.map
        vec![ // 65 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(67, false),
        ], 
        vec![ // 66 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 67 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(66, false),
        ], 
         // FUN23Clausify.clause
        vec![ // 68 
            COM(6,54,[4,0,1,5,3,2]), //X(X(XXX)X)
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
            PTR(77, false),
            PTR(70, false),
            PTR(69, false),
        ], 
        vec![ // 69 
            COM(2,2,[1,0,0,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 70 
            COM(5,12,[4,0,3,1,2,0]), //X(XXX)X
            PTR(78, false),
        ], 
        vec![ // 71 
            COM(5,16,[4,1,0,3,2,0]), //XX(XXX)
            PTR(78, false),
        ], 
        vec![ // 72 
            COM(4,15,[0,1,3,2,3,0]), //X(XX)(XX)
            COM(5,37,[4,0,0,1,2,3]), //XXXX(XX)
            COM(3,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 73 
            COM(5,54,[0,0,1,2,3,4]), //X(X(XXX)X)
            PTR(68, false),
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
            PTR(75, false),
            PTR(74, false),
            PTR(73, false),
            PTR(72, false),
        ], 
        vec![ // 77 
            COM(4,4,[0,2,3,1,0,0]), //XXXX
            PTR(76, false),
            PTR(71, false),
        ], 
         // FUN24Clausify.insert
        vec![ // 78 
            COM(4,19,[0,1,2,3,3,0]), //X(X(XX)X)
            Y,
            PTR(84, false),
            PTR(83, false),
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
            PTR(81, false),
            PTR(80, false),
        ], 
        vec![ // 83 
            COM(5,40,[0,1,3,4,2,3]), //X(XXX)(XX)
            COM(4,42,[0,2,3,1,2,3]), //XXX(XXX)
            PTR(82, false),
            PTR(79, false),
        ], 
        vec![ // 84 
            COM(6,40,[5,0,3,1,2,4]), //X(XXX)(XX)
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
            COM(5,1,[3,0,0,0,0,0]), //XX
        ], 
        vec![ // 89 
            COM(5,12,[0,1,3,4,2,0]), //X(XXX)X
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(6,2,[3,0,1,0,0,0]), //XXX
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
         // FUN27Clausify.disin
        vec![ // 93 
            COM(5,9,[4,0,1,2,3,0]), //XXXXX
            PTR(95, false),
            PTR(94, false),
            COM(5,1,[3,0,0,0,0,0]), //XX
            COM(5,1,[4,0,0,0,0,0]), //XX
        ], 
        vec![ // 94 
            COM(4,15,[0,1,2,1,3,0]), //X(XX)(XX)
            PTR(96, false),
            PTR(93, false),
        ], 
        vec![ // 95 
            COM(4,15,[0,1,2,1,3,0]), //X(XX)(XX)
            COM(6,2,[2,0,1,0,0,0]), //XXX
            PTR(93, false),
        ], 
         // FUN28Clausify.din
        vec![ // 96 
            COM(4,14,[0,2,3,1,3,0]), //XXX(XX)
            PTR(103, false),
            PTR(97, false),
        ], 
        vec![ // 97 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(104, false),
            COM(5,1,[4,0,0,0,0,0]), //XX
        ], 
        vec![ // 98 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(104, false),
            COM(5,1,[3,0,0,0,0,0]), //XX
        ], 
        vec![ // 99 
            COM(5,12,[0,1,3,4,2,0]), //X(XXX)X
            PTR(104, false),
            COM(6,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 100 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(96, false),
        ], 
        vec![ // 101 
            COM(6,40,[0,1,4,3,2,5]), //X(XXX)(XX)
            COM(6,2,[2,0,1,0,0,0]), //XXX
            PTR(96, false),
        ], 
        vec![ // 102 
            COM(4,19,[2,0,1,3,3,0]), //X(X(XX)X)
            PTR(101, false),
            PTR(100, false),
        ], 
        vec![ // 103 
            COM(5,10,[0,1,4,2,3,0]), //X(XX)XX
            COM(4,39,[0,3,1,3,2,3]), //XX(XX)(XX)
            PTR(102, false),
            PTR(99, false),
            PTR(98, false),
        ], 
         // FUN29Clausify.din2
        vec![ // 104 
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(111, false),
            PTR(105, false),
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
            PTR(96, false),
        ], 
        vec![ // 109 
            COM(3,5,[0,1,2,2,0,0]), //X(XX)X
            PTR(108, false),
            PTR(96, false),
        ], 
        vec![ // 110 
            COM(6,24,[5,0,1,2,3,4]), //X(XX)XXX
            PTR(109, false),
        ], 
        vec![ // 111 
            COM(4,39,[0,3,1,3,2,3]), //XX(XX)(XX)
            PTR(110, false),
            PTR(107, false),
            PTR(106, false),
        ], 
         // FUN30Clausify.negin
        vec![ // 112 
            COM(5,9,[4,0,1,2,3,0]), //XXXXX
            PTR(120, false),
            PTR(119, false),
            PTR(118, false),
            COM(5,1,[4,0,0,0,0,0]), //XX
        ], 
        vec![ // 113 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            COM(5,1,[3,0,0,0,0,0]), //XX
            COM(5,1,[4,0,0,0,0,0]), //XX
        ], 
        vec![ // 114 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(112, false),
            COM(5,1,[3,0,0,0,0,0]), //XX
        ], 
        vec![ // 115 
            COM(6,41,[0,1,2,4,3,5]), //X(X(XX))(XX)
            COM(6,2,[2,0,1,0,0,0]), //XXX
            PTR(112, false),
            COM(5,1,[3,0,0,0,0,0]), //XX
            PTR(114, false),
        ], 
        vec![ // 116 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(112, false),
            COM(5,1,[3,0,0,0,0,0]), //XX
        ], 
        vec![ // 117 
            COM(6,41,[0,1,2,4,3,5]), //X(X(XX))(XX)
            COM(6,2,[3,0,1,0,0,0]), //XXX
            PTR(112, false),
            COM(5,1,[3,0,0,0,0,0]), //XX
            PTR(116, false),
        ], 
        vec![ // 118 
            COM(5,9,[4,0,1,2,3,0]), //XXXXX
            PTR(117, false),
            PTR(115, false),
            PTR(112, false),
            PTR(113, false),
        ], 
        vec![ // 119 
            COM(4,15,[0,1,2,1,3,0]), //X(XX)(XX)
            COM(6,2,[3,0,1,0,0,0]), //XXX
            PTR(112, false),
        ], 
        vec![ // 120 
            COM(4,15,[0,1,2,1,3,0]), //X(XX)(XX)
            COM(6,2,[2,0,1,0,0,0]), //XXX
            PTR(112, false),
        ], 
         // FUN31NanoPrelude.replicate
        vec![ // 121 
            COM(4,10,[0,1,3,2,3,0]), //X(XX)XX
            PTR(124, false),
            PTR(123, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 122 
            COM(4,7,[0,1,3,2,0,0]), //X(XXX)
            PTR(121, false),
            PRM(SUB,false),
            INT(1),
        ], 
        vec![ // 123 
            COM(4,16,[0,3,1,2,3,0]), //XX(XXX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(122, false),
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
            PTR(126, false),
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