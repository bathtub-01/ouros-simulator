use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*;

// Functions in this file: 3
// Apps in this file: 9
// Combinators in this file: 9
#[rustfmt::skip]
pub static TREESUM: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0TreeSum.main
        vec![ // 0 
            PTR(2, false),
            PTR(1, false),
        ], 
        vec![ // 1 
            PTR(5, false),
            INT(13),
        ], 
         // FUN1TreeSum.treeSum
        vec![ // 2 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            INT(1),
            PTR(4, false),
        ], 
        vec![ // 3 
            COM(4,57,[0,0,1,2,1,3]), //X(X(XX)(XX))
            PRM(ADD,false),
            PTR(2, false),
        ], 
        vec![ // 4 
            COM(4,4,[0,2,3,1,0,0]), //XXXX
            PTR(3, false),
            INT(1),
        ], 
         // FUN2TreeSum.mkTree
        vec![ // 5 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PTR(8, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 6 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PRM(SUB,false),
            INT(1),
        ], 
        vec![ // 7 
            COM(3,15,[0,1,2,1,2,0]), //X(XX)(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(5, false),
        ], 
        vec![ // 8 
            COM(5,43,[0,4,1,2,3,4]), //XXX(X(XX))
            PRM(EQ,false),
            INT(0),
            PTR(7, false),
            PTR(6, false),
        ], 
    ]
});
