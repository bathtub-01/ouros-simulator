use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 15
// Apps in this file: 46
// Combinators in this file: 59
#[rustfmt::skip]
pub static SUMEULER: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0SumEuler.main
        vec![ // 0 
            PTR(2, false),
            PTR(1, false),
        ], 
        vec![ // 1 
            PTR(5, false),
            INT(1),
            INT(30),
        ], 
         // FUN1NanoPrelude.sum
        vec![ // 2 
            PTR(3, false),
            PRM(ADD,false),
            INT(0),
        ], 
         // FUN2NanoPrelude.foldr
        vec![ // 3 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            Y,
            PTR(4, false),
        ], 
        vec![ // 4 
            COM(5,16,[4,2,0,1,3,0]), //XX(XXX)
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
        ], 
         // FUN3SumEuler.totients
        vec![ // 5 
            COM(3,5,[0,1,2,2,0,0]), //X(XX)X
            PTR(7, false),
            PTR(6, false),
        ], 
        vec![ // 6 
            COM(4,7,[0,1,3,2,0,0]), //X(XXX)
            PTR(39, false),
            PRM(ADD,false),
            INT(1),
        ], 
        vec![ // 7 
            COM(6,48,[0,1,2,4,3,5]), //XX(XX(XX))
            PTR(8, false),
            PTR(11, false),
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
         // FUN4NanoPrelude.map
        vec![ // 8 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(10, false),
        ], 
        vec![ // 9 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 10 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(9, false),
        ], 
         // FUN5SumEuler.euler
        vec![ // 11 
            COM(5,57,[0,1,2,4,3,4]), //X(X(XX)(XX))
            PTR(13, false),
            PTR(16, false),
            PTR(20, false),
            PTR(12, false),
        ], 
        vec![ // 12 
            COM(5,16,[0,1,2,4,3,0]), //XX(XXX)
            PTR(39, false),
            INT(1),
            PRM(SUB,false),
            INT(1),
        ], 
         // FUN6NanoPrelude.length
        vec![ // 13 
            Y,
            PTR(15, false),
            INT(0),
        ], 
        vec![ // 14 
            COM(5,7,[2,0,3,1,0,0]), //X(XXX)
            PRM(ADD,false),
            INT(1),
        ], 
        vec![ // 15 
            COM(4,16,[3,2,0,1,2,0]), //XX(XXX)
            PTR(14, false),
        ], 
         // FUN7NanoPrelude.filter
        vec![ // 16 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(19, false),
        ], 
        vec![ // 17 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 18 
            COM(4,5,[0,1,3,2,0,0]), //X(XX)X
            COM(4,30,[0,2,1,2,3,3]), //XX(XXX)X
            COM(5,39,[0,3,1,4,2,4]), //XX(XX)(XX)
            PTR(17, false),
        ], 
        vec![ // 19 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(18, false),
        ], 
         // FUN8SumEuler.relprime
        vec![ // 20 
            COM(5,12,[0,1,3,4,2,0]), //X(XXX)X
            PRM(EQ,false),
            PTR(21, false),
            INT(1),
        ], 
         // FUN9SumEuler.hcf
        vec![ // 21 
            COM(3,5,[0,1,2,2,0,0]), //X(XX)X
            PTR(23, false),
            PTR(22, false),
        ], 
        vec![ // 22 
            COM(4,16,[0,3,1,2,3,0]), //XX(XXX)
            PTR(21, false),
            PTR(24, false),
        ], 
        vec![ // 23 
            COM(5,28,[0,1,4,2,4,3]), //XXX(XX)X
            PRM(EQ,false),
            INT(0),
        ], 
         // FUN10NanoPrelude.mod
        vec![ // 24 
            COM(4,4,[0,2,3,1,0,0]), //XXXX
            PTR(25, false),
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
         // FUN11NanoPrelude.divMod
        vec![ // 25 
            COM(5,19,[0,1,2,4,3,0]), //X(X(XX)X)
            Y,
            COM(4,42,[0,2,3,1,3,3]), //XXX(XXX)
            PTR(38, false),
            PRM(ADD,false),
        ], 
        vec![ // 26 
            COM(2,2,[0,1,1,0,0,0]), //XXX
            PRM(ADD,false),
        ], 
        vec![ // 27 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PRM(SUB,false),
        ], 
        vec![ // 28 
            COM(6,40,[5,0,4,1,2,3]), //X(XXX)(XX)
            PRM(ADD,false),
            INT(1),
        ], 
        vec![ // 29 
            COM(5,42,[0,2,3,1,3,4]), //XXX(XXX)
            PRM(LE,false),
            COM(3,2,[2,1,0,0,0,0]), //XXX
        ], 
        vec![ // 30 
            COM(5,45,[0,1,4,2,3,4]), //X(XX)(X(XX))
            COM(4,42,[0,3,2,1,3,2]), //XXX(XXX)
            PTR(29, false),
            PTR(28, false),
            PTR(27, false),
        ], 
        vec![ // 31 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            PTR(30, false),
            PTR(26, false),
        ], 
        vec![ // 32 
            COM(4,6,[1,3,0,2,0,0]), //XX(XX)
            PTR(31, false),
        ], 
        vec![ // 33 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            INT(1),
            PRM(SUB,false),
        ], 
        vec![ // 34 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(0),
        ], 
        vec![ // 35 
            COM(5,37,[0,4,3,1,2,4]), //XXXX(XX)
            PRM(LE,false),
        ], 
        vec![ // 36 
            COM(4,29,[0,1,3,2,3,3]), //X(XX)(XX)X
            PTR(35, false),
            PTR(34, false),
            PTR(33, false),
        ], 
        vec![ // 37 
            COM(5,37,[0,4,1,2,3,4]), //XXXX(XX)
            PRM(LE,false),
        ], 
        vec![ // 38 
            COM(5,29,[0,1,4,2,4,3]), //X(XX)(XX)X
            COM(5,44,[0,1,4,2,3,4]), //X(XX)(XXX)
            PTR(37, false),
            PTR(36, false),
            PTR(32, false),
        ], 
         // FUN12NanoPrelude.enumFromTo
        vec![ // 39 
            COM(5,15,[0,1,4,2,3,0]), //X(XX)(XX)
            PTR(41, false),
            PTR(40, false),
            PTR(45, false),
        ], 
        vec![ // 40 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PRM(LE,false),
        ], 
         // FUN13NanoPrelude.takeWhile
        vec![ // 41 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(44, false),
        ], 
        vec![ // 42 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 43 
            COM(5,42,[1,3,0,2,3,4]), //XXX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 44 
            COM(6,48,[5,0,1,3,2,4]), //XX(XX(XX))
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(43, false),
            PTR(42, false),
        ], 
         // FUN14NanoPrelude.enumFrom
        vec![ // 45 
            COM(5,49,[0,4,1,2,4,3]), //XX(X(XXX))
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(45, false),
            PRM(ADD,false),
            INT(1),
        ], 
    ]
});