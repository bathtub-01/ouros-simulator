use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*; 
 
// Functions in this file: 6
// Apps in this file: 21
// Combinators in this file: 34
#[rustfmt::skip]
pub static TREEPARI: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0TreePari.main
        vec![ // 0 
            PTR(3, false),
            PTR(2, false),
        ], 
        vec![ // 1 
            PTR(17, false),
            INT(10),
        ], 
        vec![ // 2 
            PTR(4, false),
            PTR(14, false),
            PTR(1, false),
        ], 
         // FUN1TreePari.peek
        vec![ // 3 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(0),
            INT(42),
        ], 
         // FUN2TreePari.pariWhere
        vec![ // 4 
            COM(4,20,[0,1,3,2,3,0]), //X(XX(XX))
            Y,
            PTR(11, false),
            PTR(9, false),
        ], 
        vec![ // 5 
            COM(4,57,[0,0,1,2,1,3]), //X(X(XX)(XX))
            PTR(12, false),
        ], 
        vec![ // 6 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            PTR(5, false),
            COM(1,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 7 
            COM(6,24,[0,1,5,2,4,3]), //X(XX)XXX
            COM(6,46,[0,5,1,2,3,4]), //XX(XXXX)
        ], 
        vec![ // 8 
            COM(5,10,[0,1,4,3,2,0]), //X(XX)XX
            PTR(7, false),
            PTR(6, false),
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
        vec![ // 9 
            COM(6,23,[0,2,3,4,5,1]), //XXXXXX
            PTR(8, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 10 
            COM(5,18,[4,3,0,1,2,0]), //X(XXXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(2,0,[1,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 11 
            COM(5,42,[0,1,4,2,3,4]), //XXX(XXX)
            PTR(10, false),
        ], 
         // FUN3TreePari.xor
        vec![ // 12 
            COM(5,40,[3,4,0,1,2,4]), //X(XXX)(XX)
            COM(2,0,[1,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(13, false),
        ], 
        vec![ // 13 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            COM(2,0,[1,0,0,0,0,0]), //X
        ], 
         // FUN4TreePari.withTwoLeaf
        vec![ // 14 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(16, false),
        ], 
        vec![ // 15 
            COM(3,0,[0,0,0,0,0,0]), //X
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 16 
            COM(4,12,[2,3,0,1,1,0]), //X(XXX)X
            COM(2,0,[1,0,0,0,0,0]), //X
            PTR(15, false),
        ], 
         // FUN5TreePari.mkTree
        vec![ // 17 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(20, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 18 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PRM(SUB,false),
            INT(1),
        ], 
        vec![ // 19 
            COM(3,15,[0,1,2,1,2,0]), //X(XX)(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(17, false),
        ], 
        vec![ // 20 
            COM(5,43,[0,4,1,2,3,4]), //XXX(X(XX))
            PRM(EQ,false),
            INT(0),
            PTR(19, false),
            PTR(18, false),
        ], 
    ]
});