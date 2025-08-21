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
            PTR(1, false),
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
-- the computation in `a` should be shared
main = let a = tfAnd T
           b = ftAnd (a T) (a F)
       in tfOr b b
 */
#[rustfmt::skip]
pub static BOOL_NEST: LazyLock<Program> = LazyLock::new(|| {
    vec![
        // main
        vec![ // 0
            PTR(4, false),
            PTR(2, false),
            PTR(2, false)
        ],
        // tfAnd
        vec![ // 1
            COM(3,2,[1,2,0,0,0,0]), // XXX
            COM(2,0,[1,0,0,0,0,0]), // X
        ],
        // b
        vec![ // 2
            COM(4,15,[0,1,2,1,3,0]), // X(XX)(XX)
            PTR(1, false),
            PTR(3, false), // a
            COM(2,0,[0,0,0,0,0,0]), // T
            COM(2,0,[1,0,0,0,0,0]), // F
        ],
        // a
        vec![ // 3
            PTR(1, false),
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
            PTR(2, false),
            PTR(1, false)
        ],
        // a * b
        vec![ // 1
            PRM(MUL, false),
            PTR(3, false),
            PTR(4, false)
        ],
        // a - b
        vec![ // 2
            PRM(SUB, false),
            PTR(3, false),
            PTR(4, false)
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
            PTR(3, false),
            INT(2)
        ],
    ]
});

#[rustfmt::skip]
pub static MAP_Y: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0Map.main
        vec![ // 0 
            PTR(4, false),
            PRM(ADD,false),
            INT(0),
            PTR(3, false),
        ], 
        vec![ // 1 
            PTR(9, false),
            INT(0),
            INT(49),
        ], 
        vec![ // 2 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PRM(ADD,false),
            INT(1),
        ], 
        vec![ // 3 
            PTR(6, false),
            PTR(2, false),
            PTR(1, false),
        ], 
         // FUN1NanoPrelude.foldr'
        vec![ // 4 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            Y,
            PTR(5, false),
        ], 
        vec![ // 5 
            COM(5,16,[4,2,0,1,3,0]), //XX(XXX)
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
        ], 
         // FUN2NanoPrelude.map
        vec![ // 6 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(8, false),
        ], 
        vec![ // 7 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 8 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(7, false),
        ], 
         // FUN3NanoPrelude.enumFromTo
        vec![ // 9 
            COM(5,15,[0,1,4,2,3,0]), //X(XX)(XX)
            PTR(11, false),
            PTR(10, false),
            PTR(15, false),
        ], 
        vec![ // 10 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PRM(LE,false),
        ], 
         // FUN4NanoPrelude.takeWhile
        vec![ // 11 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(14, false),
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
            PTR(13, false),
            PTR(12, false),
        ], 
         // FUN5NanoPrelude.enumFrom
        vec![ // 15 
            COM(5,49,[0,4,1,2,4,3]), //XX(X(XXX))
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(15, false),
            PRM(ADD,false),
            INT(1),
        ], 
    ]
});

/**
A deadlock example:
main = let a = sum [1 .. 50]
           b = a * 2
           c = sum (map (+ b) [1 .. 10])
       in a + (b + c)
 */
#[rustfmt::skip]
pub static DEADLOCK: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0Playground.main
        vec![ // 0 
            COM(5,49,[0,4,1,2,4,3]), //XX(X(XXX))
            PRM(ADD,false),
            PTR(5, false),
            PRM(MUL,false),
            INT(2),
            PTR(2, false),
        ], 
        vec![ // 1 
            PTR(12, false),
            INT(1),
            INT(50),
        ], 
        vec![ // 2 
            PTR(6, false),
            PTR(1, false),
        ], 
        vec![ // 3 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PRM(ADD,false),
        ], 
        vec![ // 4 
            COM(6,44,[0,1,5,2,3,4]), //X(XX)(XXX)
            PTR(9, false),
            PTR(3, false),
            PTR(12, false),
            INT(1),
            INT(10),
        ], 
        vec![ // 5 
            COM(4,17,[0,3,1,2,3,0]), //XX(X(XX))
            PRM(ADD,false),
            PTR(6, false),
            PTR(4, false),
        ], 
         // FUN1NanoPrelude.sum
        vec![ // 6 
            PTR(7, false),
            PRM(ADD,false),
            INT(0),
        ], 
         // FUN2NanoPrelude.foldr
        vec![ // 7 
            COM(4,7,[0,1,2,3,0,0]), //X(XXX)
            Y,
            PTR(8, false),
        ], 
        vec![ // 8 
            COM(5,16,[4,2,0,1,3,0]), //XX(XXX)
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
        ], 
         // FUN3NanoPrelude.map
        vec![ // 9 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(11, false),
        ], 
        vec![ // 10 
            COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 11 
            COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(10, false),
        ], 
         // FUN4NanoPrelude.enumFromTo
        vec![ // 12 
            COM(5,15,[0,1,4,2,3,0]), //X(XX)(XX)
            PTR(14, false),
            PTR(13, false),
            PTR(18, false),
        ], 
        vec![ // 13 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PRM(LE,false),
        ], 
         // FUN5NanoPrelude.takeWhile
        vec![ // 14 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(17, false),
        ], 
        vec![ // 15 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 16 
            COM(5,42,[1,3,0,2,3,4]), //XXX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 17 
            COM(6,48,[5,0,1,3,2,4]), //XX(XX(XX))
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(16, false),
            PTR(15, false),
        ], 
         // FUN6NanoPrelude.enumFrom
        vec![ // 18 
            COM(5,49,[0,4,1,2,4,3]), //XX(X(XXX))
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(18, false),
            PRM(ADD,false),
            INT(1),
        ], 
    ]
});

/**
Use `seq` to create new threads:
main = let a = tfAnd T
           b = ftAnd (a T) (a F)
       in seq a (tfOr b b)
 */
#[rustfmt::skip]
pub static USE_SEQ: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0Playground.main
        vec![ // 0 
            PTR(2, false),
            INT(1),
            PTR(1, false),
        ], 
        vec![ // 1 
            PRM(MUL,false),
            INT(3),
            INT(3),
        ], 
         // FUN1Playground.sumPair
        vec![ // 2 
            COM(4,16,[0,2,1,2,3,0]), //XX(XXX)
            SEQ(false),
            PTR(3, false),
        ], 
        vec![ // 3 
            COM(4,16,[0,3,1,2,3,0]), //XX(XXX)
            SEQ(false),
            PRM(ADD,false),
        ], 
    ]
});

#[rustfmt::skip]
pub static BANG: LazyLock<Program> = LazyLock::new(|| {
    vec![
         // FUN0Playground.main
        vec![ // 0 
            PTR(8, false),
            PTR(7, false),
            PTR(4, false),
            PTR(2, false),
        ], 
        vec![ // 1 
            PTR(22, false),
            INT(30),
            INT(200),
        ], 
        vec![ // 2 
            PTR(12, false),
            PTR(1, false),
        ], 
        vec![ // 3 
            PTR(22, false),
            INT(30),
            INT(100),
        ], 
        vec![ // 4 
            PTR(12, false),
            PTR(3, false),
        ], 
        vec![ // 5 
            PTR(15, false),
            INT(1),
            INT(50),
        ], 
        vec![ // 6 
            PTR(12, false),
            PTR(5, false),
        ], 
        vec![ // 7 
            PRM(EQ,false),
            PTR(6, false),
            INT(50),
        ], 
         // FUN1Playground.sumOrMul
        vec![ // 8 
            COM(4,12,[0,1,2,3,3,0]), //X(XXX)X
            PTR(11, false),
            PTR(10, false),
        ], 
        vec![ // 9 
            COM(5,40,[1,0,3,4,2,4]), //X(XXX)(XX)
            PRM(SUB,false),
        ], 
        vec![ // 10 
            COM(4,11,[0,2,1,3,3,0]), //XX(XX)X
            PTR(9, false),
            PRM(ADD,false),
        ], 
        vec![ // 11 
            COM(4,48,[0,2,0,3,1,3]), //XX(XX(XX))
            SEQ(false),
        ], 
         // FUN2Playground.lastEle
        vec![ // 12 
            COM(3,2,[2,0,1,0,0,0]), //XXX
            ERR(0),
            PTR(14, false),
        ], 
        vec![ // 13 
            COM(4,1,[0,1,0,0,0,0]), //XX
            PTR(12, false),
        ], 
        vec![ // 14 
            COM(3,6,[2,1,0,2,0,0]), //XX(XX)
            PTR(13, false),
        ], 
         // FUN3NanoPrelude.enumFromTo
        vec![ // 15 
            COM(5,15,[0,1,4,2,3,0]), //X(XX)(XX)
            PTR(17, false),
            PTR(16, false),
            PTR(21, false),
        ], 
        vec![ // 16 
            COM(3,2,[0,2,1,0,0,0]), //XXX
            PRM(LE,false),
        ], 
         // FUN4NanoPrelude.takeWhile
        vec![ // 17 
            COM(3,3,[0,1,2,0,0,0]), //X(XX)
            Y,
            PTR(20, false),
        ], 
        vec![ // 18 
            COM(4,6,[0,2,1,3,0,0]), //XX(XX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
        ], 
        vec![ // 19 
            COM(5,42,[1,3,0,2,3,4]), //XXX(XXX)
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 20 
            COM(6,48,[5,0,1,3,2,4]), //XX(XX(XX))
            COM(2,0,[0,0,0,0,0,0]), //X
            PTR(19, false),
            PTR(18, false),
        ], 
         // FUN5NanoPrelude.enumFrom
        vec![ // 21 
            COM(5,49,[0,4,1,2,4,3]), //XX(X(XXX))
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(21, false),
            PRM(ADD,false),
            INT(1),
        ], 
         // FUN6NanoPrelude.replicate
        vec![ // 22 
            COM(4,10,[0,1,3,2,3,0]), //X(XX)XX
            PTR(25, false),
            PTR(24, false),
            COM(2,0,[0,0,0,0,0,0]), //X
        ], 
        vec![ // 23 
            COM(4,7,[0,1,3,2,0,0]), //X(XXX)
            PTR(22, false),
            PRM(SUB,false),
            INT(1),
        ], 
        vec![ // 24 
            COM(4,16,[0,3,1,2,3,0]), //XX(XXX)
            COM(4,2,[3,0,1,0,0,0]), //XXX
            PTR(23, false),
        ], 
        vec![ // 25 
            COM(6,28,[0,4,1,2,5,3]), //XXX(XX)X
            PRM(LE,false),
            INT(0),
        ], 
    ]
});
