use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 18
// Apps in this file: 57
// Combinators in this file: 82
#[rustfmt::skip]
pub static QUEENS2: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0Queens2.main
        vec![ // 0 
            PTR(1, false),
            INT(5),
        ], 
         // FUN1Queens2.nqueens
        vec![ // 1 
            COM(5,58,[0,1,4,2,4,3]), //X(XX(XXX))
            PTR(2, false),
            PTR(5, false),
            PTR(54, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN2NanoPrelude.length
        vec![ // 2 
            Y,
            PTR(4, false),
            INT(0),
        ], 
        vec![ // 3 
            COM(6,49,[0,1,4,2,5,3]), //XX(X(XXX))
            COM(4,6,[3,2,0,1,0,0]), //XX(XX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PRM(ADD,false),
            INT(1),
        ], 
        vec![ // 4 
            COM(3,4,[0,1,2,2,0,0]), //XXXX
            PTR(3, false),
        ], 
         // FUN3Queens2.solve
        vec![ // 5 
            COM(4,4,[0,3,1,2,0,0]), //XXXX
            PTR(8, false),
            PTR(44, false),
            PTR(6, false),
        ], 
        vec![ // 6 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 7 
            COM(5,21,[0,1,2,4,3,0]), //X(X(XXX))
            PTR(9, false),
            PTR(15, false),
            PRM(SUB,false),
            INT(1),
        ], 
        vec![ // 8 
            COM(5,40,[0,1,4,2,3,4]), //X(XXX)(XX)
            COM(5,13,[0,1,2,4,3,0]), //X(X(XX))X
            PRM(EQ,false),
            INT(0),
            PTR(7, false),
        ], 
         // FUN4Data.List_Type.concatMap
        vec![ // 9 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(11, false),
        ], 
        vec![ // 10 
            COM(5,13,[0,1,2,4,3,0]), //X(X(XX))X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(12, false),
        ], 
        vec![ // 11 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(10, false),
        ], 
         // FUN5Data.List_Type.++
        vec![ // 12 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            Y,
            PTR(14, false),
        ], 
        vec![ // 13 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 14 
            COM(4,6,[3,1,0,2,0,0]), //XX(XX)
            PTR(13, false),
        ], 
         // FUN6Queens2.sol
        vec![ // 15 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(16, false),
            PTR(5, false),
            PTR(20, false),
        ], 
        vec![ // 16 
            COM(5,45,[0,1,4,2,3,4]), //X(XX)(X(XX))
            PTR(17, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
         // FUN7NanoPrelude.map
        vec![ // 17 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(19, false),
        ], 
        vec![ // 18 
            COM(5,13,[0,1,2,4,3,0]), //X(X(XX))X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 19 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(18, false),
        ], 
         // FUN8Queens2.next
        vec![ // 20 
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(21, false),
            PTR(39, false),
        ], 
        vec![ // 21 
            COM(5,57,[0,1,2,4,3,4]), //X(X(XX)(XX))
            PTR(22, false),
            PTR(22, false),
            PTR(27, false),
            PTR(34, false),
        ], 
         // FUN9Queens2.merge
        vec![ // 22 
            COM(4,5,[0,3,1,2,0,0]), //X(XX)X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(26, false),
        ], 
        vec![ // 23 
            COM(5,13,[0,1,2,4,3,0]), //X(X(XX))X
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 24 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(23, false),
            PTR(12, false),
        ], 
        vec![ // 25 
            COM(5,39,[0,3,1,4,2,4]), //XX(XX)(XX)
            COM(5,45,[0,1,4,2,3,4]), //X(XX)(X(XX))
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(24, false),
        ], 
        vec![ // 26 
            COM(4,4,[0,2,3,1,0,0]), //XXXX
            PTR(25, false),
            PTR(22, false),
        ], 
         // FUN10Queens2.down
        vec![ // 27 
            PTR(17, false),
            PTR(29, false),
        ], 
        vec![ // 28 
            PRM(EQ,false),
            INT(2),
        ], 
        vec![ // 29 
            PTR(30, false),
            PTR(28, false),
        ], 
         // FUN11Queens2.one
        vec![ // 30 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(33, false),
        ], 
        vec![ // 31 
            COM(4,2,[3,1,0,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 32 
            COM(5,38,[0,2,4,3,1,4]), //X(XX)X(XX)
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            PTR(31, false),
        ], 
        vec![ // 33 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(32, false),
        ], 
         // FUN12Queens2.left
        vec![ // 34 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(37, false),
            PTR(38, false),
        ], 
        vec![ // 35 
            PRM(EQ,false),
            INT(0),
        ], 
        vec![ // 36 
            PTR(30, false),
            PTR(35, false),
        ], 
        vec![ // 37 
            PTR(17, false),
            PTR(36, false),
        ], 
         // FUN13NanoPrelude.tail
        vec![ // 38 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
         // FUN14Queens2.right
        vec![ // 39 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(43, false),
            PTR(42, false),
        ], 
        vec![ // 40 
            PRM(EQ,false),
            INT(1),
        ], 
        vec![ // 41 
            PTR(30, false),
            PTR(40, false),
        ], 
        vec![ // 42 
            PTR(17, false),
            PTR(41, false),
        ], 
        vec![ // 43 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN15Queens2.fill
        vec![ // 44 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(47, false),
        ], 
        vec![ // 45 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(17, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 46 
            COM(5,39,[0,1,2,4,3,4]), //XX(XX)(XX)
            COM(5,45,[0,1,4,2,3,4]), //X(XX)(X(XX))
            PTR(12, false),
            PTR(48, false),
            PTR(45, false),
        ], 
        vec![ // 47 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(46, false),
            PTR(44, false),
        ], 
         // FUN16Queens2.lrd
        vec![ // 48 
            COM(6,33,[4,0,1,5,2,3]), //X(X(XX)X)X
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(53, false),
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(49, false),
        ], 
        vec![ // 49 
            COM(3,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 50 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(2),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 51 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(1),
            PTR(50, false),
        ], 
        vec![ // 52 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(0),
            PTR(51, false),
        ], 
        vec![ // 53 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(52, false),
        ], 
         // FUN17NanoPrelude.replicate
        vec![ // 54 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(56, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 55 
            COM(5,12,[0,1,4,2,3,0]), //X(XXX)X
            COM(5,34,[0,1,4,2,4,3]), //X(XX(XX))X
            PRM(LE,false),
            INT(0),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 56 
            COM(5,49,[0,4,1,2,4,3]), //XX(X(XXX))
            PTR(55, false),
            PTR(54, false),
            PRM(SUB,false),
            INT(1),
        ], 
    ]
});