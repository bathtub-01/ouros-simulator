use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 18
// Apps in this file: 56
// Combinators in this file: 73
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
            PTR(52, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN2NanoPrelude.length
        vec![ // 2 
            Y,
            PTR(4, false),
            INT(0),
        ], 
        vec![ // 3 
            COM(5,7,[2,0,3,1,0,0]), //X(XXX)
            PRM(ADD,false),
            INT(1),
        ], 
        vec![ // 4 
            COM(4,16,[3,2,0,1,2,0]), //XX(XXX)
            PTR(3, false),
        ], 
         // FUN3Queens2.solve
        vec![ // 5 
            COM(4,4,[0,2,3,1,0,0]), //XXXX
            PTR(9, false),
            PTR(6, false),
        ], 
        vec![ // 6 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 7 
            COM(5,21,[0,1,2,4,3,0]), //X(X(XXX))
            PTR(10, false),
            PTR(17, false),
            PRM(SUB,false),
            INT(1),
        ], 
        vec![ // 8 
            COM(6,43,[0,4,1,2,3,5]), //XXX(X(XX))
            PRM(EQ,false),
            INT(0),
        ], 
        vec![ // 9 
            COM(4,10,[0,1,3,2,3,0]), //X(XX)XX
            PTR(8, false),
            PTR(7, false),
            PTR(42, false),
        ], 
         // FUN4Queens2.concatMap'
        vec![ // 10 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(13, false),
        ], 
        vec![ // 11 
            COM(5,33,[0,1,2,3,4,4]), //X(X(XX)X)X
            TRY,
            PTR(14, false),
        ], 
        vec![ // 12 
            COM(5,14,[0,1,3,2,4,0]), //XXX(XX)
            PTR(11, false),
        ], 
        vec![ // 13 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(12, false),
        ], 
         // FUN5Data.List_Type.++
        vec![ // 14 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            Y,
            PTR(16, false),
        ], 
        vec![ // 15 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 16 
            COM(4,6,[3,1,0,2,0,0]), //XX(XX)
            PTR(15, false),
        ], 
         // FUN6Queens2.sol
        vec![ // 17 
            COM(5,44,[0,1,4,2,3,4]), //X(XX)(XXX)
            PTR(19, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(18, false),
        ], 
        vec![ // 18 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            PTR(5, false),
            PTR(22, false),
        ], 
         // FUN7NanoPrelude.map
        vec![ // 19 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(21, false),
        ], 
        vec![ // 20 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 21 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(20, false),
        ], 
         // FUN8Queens2.next
        vec![ // 22 
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(23, false),
            PTR(38, false),
        ], 
        vec![ // 23 
            COM(5,57,[0,1,2,4,3,4]), //X(X(XX)(XX))
            PTR(24, false),
            PTR(24, false),
            PTR(29, false),
            PTR(36, false),
        ], 
         // FUN9Queens2.merge
        vec![ // 24 
            COM(4,6,[2,0,1,3,0,0]), //XX(XX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(28, false),
        ], 
        vec![ // 25 
            COM(6,40,[0,1,2,4,3,5]), //X(XXX)(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(14, false),
        ], 
        vec![ // 26 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            PTR(25, false),
            PTR(24, false),
        ], 
        vec![ // 27 
            COM(5,40,[1,0,3,4,2,4]), //X(XXX)(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 28 
            COM(4,11,[0,2,1,3,3,0]), //XX(XX)X
            PTR(27, false),
            PTR(26, false),
        ], 
         // FUN10Queens2.down
        vec![ // 29 
            PTR(19, false),
            PTR(31, false),
        ], 
        vec![ // 30 
            PRM(EQ,false),
            INT(2),
        ], 
        vec![ // 31 
            PTR(32, false),
            PTR(30, false),
        ], 
         // FUN11Queens2.one
        vec![ // 32 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(35, false),
        ], 
        vec![ // 33 
            COM(4,2,[3,1,0,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 34 
            COM(5,39,[1,3,2,4,0,3]), //XX(XX)(XX)
            PTR(33, false),
        ], 
        vec![ // 35 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(34, false),
        ], 
         // FUN12Queens2.left
        vec![ // 36 
            COM(6,41,[0,1,2,3,4,5]), //X(X(XX))(XX)
            PTR(19, false),
            PTR(32, false),
            PRM(EQ,false),
            INT(0),
            PTR(37, false),
        ], 
         // FUN13NanoPrelude.tail
        vec![ // 37 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
         // FUN14Queens2.right
        vec![ // 38 
            COM(4,6,[0,1,2,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(41, false),
        ], 
        vec![ // 39 
            PRM(EQ,false),
            INT(1),
        ], 
        vec![ // 40 
            PTR(32, false),
            PTR(39, false),
        ], 
        vec![ // 41 
            PTR(19, false),
            PTR(40, false),
        ], 
         // FUN15Queens2.fill
        vec![ // 42 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(45, false),
        ], 
        vec![ // 43 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            PTR(19, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(42, false),
        ], 
        vec![ // 44 
            COM(5,40,[0,1,3,4,2,4]), //X(XXX)(XX)
            PTR(14, false),
            PTR(46, false),
        ], 
        vec![ // 45 
            COM(3,5,[0,1,2,2,0,0]), //X(XX)X
            PTR(44, false),
            PTR(43, false),
        ], 
         // FUN16Queens2.lrd
        vec![ // 46 
            COM(4,5,[2,0,3,1,0,0]), //X(XX)X
            PTR(51, false),
            PTR(47, false),
        ], 
        vec![ // 47 
            COM(3,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 48 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(2),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 49 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(1),
            PTR(48, false),
        ], 
        vec![ // 50 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(0),
            PTR(49, false),
        ], 
        vec![ // 51 
            COM(6,12,[5,0,1,3,2,0]), //X(XXX)X
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(50, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN17NanoPrelude.replicate
        vec![ // 52 
            COM(4,10,[0,1,3,2,3,0]), //X(XX)XX
            PTR(55, false),
            PTR(54, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 53 
            COM(4,7,[0,1,3,2,0,0]), //X(XXX)
            PTR(52, false),
            PRM(SUB,false),
            INT(1),
        ], 
        vec![ // 54 
            COM(4,16,[0,3,1,2,3,0]), //XX(XXX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(53, false),
        ], 
        vec![ // 55 
            COM(6,28,[0,4,1,2,5,3]), //XXX(XX)X
            PRM(LE,false),
            INT(0),
        ], 
    ]
});