use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 18
// Apps in this file: 55
// Combinators in this file: 72
#[rustfmt::skip]
pub static QUEENS2: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0Queens2.main
        vec![ // 0 
            PTR(1),
            INT(5),
        ], 
         // FUN1Queens2.nqueens
        vec![ // 1 
            COM(5,58,[0,1,4,2,4,3]), //X(XX(XXX))
            PTR(2),
            PTR(5),
            PTR(51),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN2NanoPrelude.length
        vec![ // 2 
            Y,
            PTR(4),
            INT(0),
        ], 
        vec![ // 3 
            COM(5,7,[2,0,3,1,0,0]), //X(XXX)
            PRM(ADD,false),
            INT(1),
        ], 
        vec![ // 4 
            COM(4,16,[3,2,0,1,2,0]), //XX(XXX)
            PTR(3),
        ], 
         // FUN3Queens2.solve
        vec![ // 5 
            COM(4,4,[0,2,3,1,0,0]), //XXXX
            PTR(9),
            PTR(6),
        ], 
        vec![ // 6 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 7 
            COM(5,21,[0,1,2,4,3,0]), //X(X(XXX))
            PTR(10),
            PTR(16),
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
            PTR(8),
            PTR(7),
            PTR(41),
        ], 
         // FUN4Data.List_Type.concatMap
        vec![ // 10 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(12),
        ], 
        vec![ // 11 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            PTR(13),
        ], 
        vec![ // 12 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(11),
        ], 
         // FUN5Data.List_Type.++
        vec![ // 13 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            Y,
            PTR(15),
        ], 
        vec![ // 14 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 15 
            COM(4,6,[3,1,0,2,0,0]), //XX(XX)
            PTR(14),
        ], 
         // FUN6Queens2.sol
        vec![ // 16 
            COM(5,44,[0,1,4,2,3,4]), //X(XX)(XXX)
            PTR(18),
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(17),
        ], 
        vec![ // 17 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            PTR(5),
            PTR(21),
        ], 
         // FUN7NanoPrelude.map
        vec![ // 18 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(20),
        ], 
        vec![ // 19 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 20 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(19),
        ], 
         // FUN8Queens2.next
        vec![ // 21 
            COM(3,6,[0,2,1,2,0,0]), //XX(XX)
            PTR(22),
            PTR(37),
        ], 
        vec![ // 22 
            COM(5,57,[0,1,2,4,3,4]), //X(X(XX)(XX))
            PTR(23),
            PTR(23),
            PTR(28),
            PTR(35),
        ], 
         // FUN9Queens2.merge
        vec![ // 23 
            COM(4,6,[2,0,1,3,0,0]), //XX(XX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(27),
        ], 
        vec![ // 24 
            COM(6,40,[0,1,2,4,3,5]), //X(XXX)(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(13),
        ], 
        vec![ // 25 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            PTR(24),
            PTR(23),
        ], 
        vec![ // 26 
            COM(5,40,[1,0,3,4,2,4]), //X(XXX)(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 27 
            COM(4,11,[0,2,1,3,3,0]), //XX(XX)X
            PTR(26),
            PTR(25),
        ], 
         // FUN10Queens2.down
        vec![ // 28 
            PTR(18),
            PTR(30),
        ], 
        vec![ // 29 
            PRM(EQ,false),
            INT(2),
        ], 
        vec![ // 30 
            PTR(31),
            PTR(29),
        ], 
         // FUN11Queens2.one
        vec![ // 31 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(34),
        ], 
        vec![ // 32 
            COM(4,2,[3,1,0,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 33 
            COM(5,39,[1,3,2,4,0,3]), //XX(XX)(XX)
            PTR(32),
        ], 
        vec![ // 34 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(33),
        ], 
         // FUN12Queens2.left
        vec![ // 35 
            COM(6,41,[0,1,2,3,4,5]), //X(X(XX))(XX)
            PTR(18),
            PTR(31),
            PRM(EQ,false),
            INT(0),
            PTR(36),
        ], 
         // FUN13NanoPrelude.tail
        vec![ // 36 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
         // FUN14Queens2.right
        vec![ // 37 
            COM(4,6,[0,1,2,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(40),
        ], 
        vec![ // 38 
            PRM(EQ,false),
            INT(1),
        ], 
        vec![ // 39 
            PTR(31),
            PTR(38),
        ], 
        vec![ // 40 
            PTR(18),
            PTR(39),
        ], 
         // FUN15Queens2.fill
        vec![ // 41 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(44),
        ], 
        vec![ // 42 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            PTR(18),
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(41),
        ], 
        vec![ // 43 
            COM(5,40,[0,1,3,4,2,4]), //X(XXX)(XX)
            PTR(13),
            PTR(45),
        ], 
        vec![ // 44 
            COM(3,5,[0,1,2,2,0,0]), //X(XX)X
            PTR(43),
            PTR(42),
        ], 
         // FUN16Queens2.lrd
        vec![ // 45 
            COM(4,5,[2,0,3,1,0,0]), //X(XX)X
            PTR(50),
            PTR(46),
        ], 
        vec![ // 46 
            COM(3,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 47 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(2),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 48 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(1),
            PTR(47),
        ], 
        vec![ // 49 
            COM(4,2,[3,0,1,0,0,0]), //XXX
            INT(0),
            PTR(48),
        ], 
        vec![ // 50 
            COM(6,12,[5,0,1,3,2,0]), //X(XXX)X
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(49),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
         // FUN17NanoPrelude.replicate
        vec![ // 51 
            COM(4,10,[0,1,3,2,3,0]), //X(XX)XX
            PTR(54),
            PTR(53),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 52 
            COM(4,7,[0,1,3,2,0,0]), //X(XXX)
            PTR(51),
            PRM(SUB,false),
            INT(1),
        ], 
        vec![ // 53 
            COM(4,16,[0,3,1,2,3,0]), //XX(XXX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(52),
        ], 
        vec![ // 54 
            COM(6,28,[0,4,1,2,5,3]), //XXX(XX)X
            PRM(LE,false),
            INT(0),
        ], 
    ]
});