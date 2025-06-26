use crate::hardware::ouros::program::{AluOp, Atom, Program};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*;

/**
A minimal Haskell program:

-- T => COM(2,0,[0,0,0,0,0,0])
-- F =>  COM(2,0,[1,0,0,0,0,0])
data TF = T | F

tfAnd T b = b
tfAnd F _ = F

main = tfAnd T F
 */
#[rustfmt::skip]
pub static BOOL_AND: LazyLock<Program> = LazyLock::new(|| {
    vec![
        // main
        vec![ // 0
            PTR(1),
            COM(2,0,[0,0,0,0,0,0]), // X
            COM(2,0,[1,0,0,0,0,0]), // X
        ],
        // tfAnd
        vec![ // 1
            COM(3,2,[1,2,0,0,0,0]), // XXX
            COM(2,0,[1,0,0,0,0,0]), // X
        ],
    ]
});

/**
-- the computation in a should be shared
main = let a = tfAnd T
           b = ftAnd (a T) (a F)
       in tfOr b b
 */
#[rustfmt::skip]
pub static BOOL_NEST: LazyLock<Program> = LazyLock::new(|| {
    vec![
        // main
        vec![ // 0
            PTR(4),
            PTR(2),
            PTR(2)
        ],
        // tfAnd
        vec![ // 1
            COM(3,2,[1,2,0,0,0,0]), // XXX
            COM(2,0,[1,0,0,0,0,0]), // X
        ],
        // b
        vec![ // 2
            COM(4,15,[0,1,2,1,3,0]), // X(XX)(XX)
            PTR(1),
            PTR(3), // a
            COM(2,0,[0,0,0,0,0,0]), // T
            COM(2,0,[1,0,0,0,0,0]), // F
        ],
        // a
        vec![ // 3
            PTR(1),
            COM(2,0,[0,0,0,0,0,0]), // X
        ],
        // tfOr
        vec![ // 4
            COM(2,1,[1,0,0,0,0,0]), // XX
            COM(2,0,[0,0,0,0,0,0]), // T
        ],
    ]
});

/**
main = let a = 4 + 5
           b = a * 2
       in
          if 42 == 42 then a * b else a - b
*/
#[rustfmt::skip]
pub static ALU_OP: LazyLock<Program> = LazyLock::new(|| {
    vec![
        // main
        vec![ // 0
            PRM(EQ, false),
            INT(42),
            INT(42),
            PTR(2),
            PTR(1)
        ],
        // a * b
        vec![ // 1
            PRM(MUL, false),
            PTR(3),
            PTR(4)
        ],
        // a - b
        vec![ // 2
            PRM(SUB, false),
            PTR(3),
            PTR(4)
        ],
        // a
        vec![ // 3
            PRM(ADD, false),
            INT(4),
            INT(5)
        ],
        // b
        vec![ // 4
            PRM(MUL, false),
            PTR(3),
            INT(2)
        ],
    ]
});

#[rustfmt::skip]
pub static MAP_Y: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0Map.main
        vec![ // 0 
            PTR(4),
            PRM(ADD,false),
            INT(0),
            PTR(3),
        ], 
        vec![ // 1 
            PTR(9),
            INT(0),
            INT(49),
        ], 
        vec![ // 2 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PRM(ADD,false),
            INT(1),
        ], 
        vec![ // 3 
            PTR(6),
            PTR(2),
            PTR(1),
        ], 
         // FUN1NanoPrelude.foldr'
        vec![ // 4 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            Y,
            PTR(5),
        ], 
        vec![ // 5 
            COM(5,16,[4,2,0,1,3,0]), //XX(XXX)
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
        ], 
         // FUN2NanoPrelude.map
        vec![ // 6 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(8),
        ], 
        vec![ // 7 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 8 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(7),
        ], 
         // FUN3NanoPrelude.enumFromTo
        vec![ // 9 
            COM(5,15,[0,1,4,2,3,0]), //X(XX)(XX)
            PTR(11),
            PTR(10),
            PTR(15),
        ], 
        vec![ // 10 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PRM(LE,false),
        ], 
         // FUN4NanoPrelude.takeWhile
        vec![ // 11 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(14),
        ], 
        vec![ // 12 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 13 
            COM(5,42,[1,3,0,2,3,4]), //XXX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 14 
            COM(6,48,[5,0,1,3,2,4]), //XX(XX(XX))
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(13),
            PTR(12),
        ], 
         // FUN5NanoPrelude.enumFrom
        vec![ // 15 
            COM(5,49,[0,4,1,2,4,3]), //XX(X(XXX))
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(15),
            PRM(ADD,false),
            INT(1),
        ], 
    ]
});
