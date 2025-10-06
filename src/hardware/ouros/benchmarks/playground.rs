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
    Program{
        heap_img: vec![
            vec![
                COM(2, 2),
                COM(2, 0),
                COM(2, 1),
            ]
        ],
        comb_img: vec![
            // True
            vec![ARG(0)],
            // False
            vec![ARG(1)],
            // tfAnd
            vec![ARG(0), ARG(1), COM(2, 1)]
        ],        
    }
});

/**
-- the computation in `a` should be shared
main = let a = tfAnd T
           b = ftAnd (a T) (a F)
       in tfOr b b
 */
#[rustfmt::skip]
pub static BOOL_NEST: LazyLock<Program> = LazyLock::new(|| {
    Program {
        heap_img: vec![
            vec![ // main
                COM(1, 2),
                PTR(1, false, false),
                PTR(1, false, false)
            ],
            vec![ // b
                COM(1, 4),
                PTR(2, false, false)
            ],
            vec![ // a
                COM(2, 3),
                COM(2, 0)
            ]
        ],
        comb_img: vec![
            // True 0
            vec![ARG(0)],
            // False 1
            vec![ARG(1)],
            // tfOr 2
            vec![ARG(0), COM(2, 0)],
            // tfAnd 3
            vec![ARG(0), ARG(1) ,COM(2, 1)],
            // b-body 4
            vec![COM(2, 3), PTR(0, true, true), PTR(1, true, true)],
            vec![ARG(0), COM(2, 0)],
            vec![ARG(0), COM(2, 1)]
        ]
    }
});

/**
main = let a = 4 + 5
           b = a * 2
       in
          if 42 == 42 then a * b else a - b
*/
#[rustfmt::skip]
pub static ALU_OP: LazyLock<Program> = LazyLock::new(|| {
    Program {
        heap_img: vec![
            vec![ // main
                PRM(EQ, false),
                INT(42),
                INT(42),
                PTR(1, false, false),
                PTR(2, false, false),
            ],
            vec![ // a - b
                PRM(SUB, false),
                PTR(3, false, false),
                PTR(4, false, false),
            ],
            vec![ // a * b
                PRM(MUL, false),
                PTR(3, false, false),
                PTR(4, false, false),
            ],
            vec![ // a
                PRM(ADD, false),
                INT(4),
                INT(5)
            ],
            vec![ // b
                PRM(MUL, false),
                PTR(3, false, false),
                INT(2)
            ],            
        ],
        comb_img: vec![
            // True 0
            vec![ARG(1)],
            // False 1
            vec![ARG(0)],
        ]
    }
});

#[rustfmt::skip]
pub static EQLIST: LazyLock<Program> = LazyLock::new(|| {
    Program {
        heap_img: vec![
            vec![
                COM(3, 4),
                PRM(LT, false),
                PTR(1, false, false),
                PTR(2, false, false)
            ],
            vec![
                COM(2, 15), // enumFromTo
                INT(1),
                INT(100)
            ],
            vec![
                COM(2, 15), // enumFromTo
                INT(101),
                INT(200)
            ]
        ],
        comb_img: vec![
            // True 0
            vec![ARG(1)],
            // False 1
            vec![ARG(0)],
            // [] 2 (arity 2)
            vec![ARG(0)],
            // : 3 (arity 4)
            vec![ARG(3), ARG(0), ARG(1)],
            // eqList 4
            vec![
                ARG(1),
                PTR(0, true, true),
                PTR(1, true, true),
            ],
            vec![COM(1, 7), ARG(2)],
            vec![COM(4, 8), ARG(0), ARG(2)],
            // 7
            vec![ARG(0), COM(2, 0), COM(2, 10)],
            // 8
            vec![ARG(1), COM(2, 1), PTR(0, true, true)],
            vec![COM(5, 11), ARG(0), ARG(2), ARG(3)],
            // 10
            vec![COM(2, 1)],
            // 11
            vec![COM(2, 14), PTR(0, true, true), PTR(1, true, true)],
            vec![ARG(0), ARG(1), ARG(3)],
            vec![COM(3, 4), ARG(0), ARG(2), ARG(4)],
            // and 14
            vec![ARG(0), COM(2, 1), ARG(1)],
            // enumFromTo 15
            vec![PRM(LE, false), ARG(0), ARG(1), COM(2, 2), PTR(0, true, true)],
            vec![COM(2, 17), ARG(0), ARG(1)],
            // 17
            vec![COM(4, 3), ARG(0), PTR(0, true, true)],
            vec![COM(2, 19), ARG(0), ARG(1)],
            // 19
            vec![COM(2, 15), PTR(0, true, true), ARG(1)],
            vec![PRM(ADD, false), ARG(0), INT(1)]
        ]
    }
});

// #[rustfmt::skip]
// pub static MAP_Y: LazyLock<Program> = LazyLock::new(|| {
//     vec![
//          // FUN0Map.main
//         vec![ // 0
//             PTR(4, false),
//             PRM(ADD,false),
//             INT(0),
//             PTR(3, false),
//         ],
//         vec![ // 1
//             PTR(9, false),
//             INT(0),
//             INT(49),
//         ],
//         vec![ // 2
//             COM(3,2,[0,2,1,0,0,0]), //XXX
//             PRM(ADD,false),
//             INT(1),
//         ],
//         vec![ // 3
//             PTR(6, false),
//             PTR(2, false),
//             PTR(1, false),
//         ],
//          // FUN1NanoPrelude.foldr'
//         vec![ // 4
//             COM(4,7,[0,1,2,3,0,0]), //X(XXX)
//             Y,
//             PTR(5, false),
//         ],
//         vec![ // 5
//             COM(5,16,[4,2,0,1,3,0]), //XX(XXX)
//             COM(4,6,[0,2,1,3,0,0]), //XX(XX)
//         ],
//          // FUN2NanoPrelude.map
//         vec![ // 6
//             COM(3,3,[0,1,2,0,0,0]), //X(XX)
//             Y,
//             PTR(8, false),
//         ],
//         vec![ // 7
//             COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
//             COM(4,2,[3,0,1,0,0,0]), //XXX
//         ],
//         vec![ // 8
//             COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
//             COM(2,0,[0,0,0,0,0,0]), //X
//             PTR(7, false),
//         ],
//          // FUN3NanoPrelude.enumFromTo
//         vec![ // 9
//             COM(5,15,[0,1,4,2,3,0]), //X(XX)(XX)
//             PTR(11, false),
//             PTR(10, false),
//             PTR(15, false),
//         ],
//         vec![ // 10
//             COM(3,2,[0,2,1,0,0,0]), //XXX
//             PRM(LE,false),
//         ],
//          // FUN4NanoPrelude.takeWhile
//         vec![ // 11
//             COM(3,3,[0,1,2,0,0,0]), //X(XX)
//             Y,
//             PTR(14, false),
//         ],
//         vec![ // 12
//             COM(4,6,[0,2,1,3,0,0]), //XX(XX)
//             COM(4,2,[3,0,1,0,0,0]), //XXX
//         ],
//         vec![ // 13
//             COM(5,42,[1,3,0,2,3,4]), //XXX(XXX)
//             COM(2,0,[0,0,0,0,0,0]), //X
//         ],
//         vec![ // 14
//             COM(6,48,[5,0,1,3,2,4]), //XX(XX(XX))
//             COM(2,0,[0,0,0,0,0,0]), //X
//             PTR(13, false),
//             PTR(12, false),
//         ],
//          // FUN5NanoPrelude.enumFrom
//         vec![ // 15
//             COM(5,49,[0,4,1,2,4,3]), //XX(X(XXX))
//             COM(4,2,[3,0,1,0,0,0]), //XXX
//             PTR(15, false),
//             PRM(ADD,false),
//             INT(1),
//         ],
//     ]
// });

/**
A deadlock example:
main = let a = sum [1 .. 50]
           b = a * 2
           c = sum (map (+ b) [1 .. 10])
       in a + (b + c)
 */
// #[rustfmt::skip]
// pub static DEADLOCK: LazyLock<Program> = LazyLock::new(|| {
//     vec![
//          // FUN0Playground.main
//         vec![ // 0
//             COM(5,49,[0,4,1,2,4,3]), //XX(X(XXX))
//             PRM(ADD,false),
//             PTR(5, false),
//             PRM(MUL,false),
//             INT(2),
//             PTR(2, false),
//         ],
//         vec![ // 1
//             PTR(12, false),
//             INT(1),
//             INT(50),
//         ],
//         vec![ // 2
//             PTR(6, false),
//             PTR(1, false),
//         ],
//         vec![ // 3
//             COM(3,2,[0,2,1,0,0,0]), //XXX
//             PRM(ADD,false),
//         ],
//         vec![ // 4
//             COM(6,44,[0,1,5,2,3,4]), //X(XX)(XXX)
//             PTR(9, false),
//             PTR(3, false),
//             PTR(12, false),
//             INT(1),
//             INT(10),
//         ],
//         vec![ // 5
//             COM(4,17,[0,3,1,2,3,0]), //XX(X(XX))
//             PRM(ADD,false),
//             PTR(6, false),
//             PTR(4, false),
//         ],
//          // FUN1NanoPrelude.sum
//         vec![ // 6
//             PTR(7, false),
//             PRM(ADD,false),
//             INT(0),
//         ],
//          // FUN2NanoPrelude.foldr
//         vec![ // 7
//             COM(4,7,[0,1,2,3,0,0]), //X(XXX)
//             Y,
//             PTR(8, false),
//         ],
//         vec![ // 8
//             COM(5,16,[4,2,0,1,3,0]), //XX(XXX)
//             COM(4,6,[0,2,1,3,0,0]), //XX(XX)
//         ],
//          // FUN3NanoPrelude.map
//         vec![ // 9
//             COM(3,3,[0,1,2,0,0,0]), //X(XX)
//             Y,
//             PTR(11, false),
//         ],
//         vec![ // 10
//             COM(5,15,[0,1,3,2,4,0]), //X(XX)(XX)
//             COM(4,2,[3,0,1,0,0,0]), //XXX
//         ],
//         vec![ // 11
//             COM(5,16,[4,0,1,2,3,0]), //XX(XXX)
//             COM(2,0,[0,0,0,0,0,0]), //X
//             PTR(10, false),
//         ],
//          // FUN4NanoPrelude.enumFromTo
//         vec![ // 12
//             COM(5,15,[0,1,4,2,3,0]), //X(XX)(XX)
//             PTR(14, false),
//             PTR(13, false),
//             PTR(18, false),
//         ],
//         vec![ // 13
//             COM(3,2,[0,2,1,0,0,0]), //XXX
//             PRM(LE,false),
//         ],
//          // FUN5NanoPrelude.takeWhile
//         vec![ // 14
//             COM(3,3,[0,1,2,0,0,0]), //X(XX)
//             Y,
//             PTR(17, false),
//         ],
//         vec![ // 15
//             COM(4,6,[0,2,1,3,0,0]), //XX(XX)
//             COM(4,2,[3,0,1,0,0,0]), //XXX
//         ],
//         vec![ // 16
//             COM(5,42,[1,3,0,2,3,4]), //XXX(XXX)
//             COM(2,0,[0,0,0,0,0,0]), //X
//         ],
//         vec![ // 17
//             COM(6,48,[5,0,1,3,2,4]), //XX(XX(XX))
//             COM(2,0,[0,0,0,0,0,0]), //X
//             PTR(16, false),
//             PTR(15, false),
//         ],
//          // FUN6NanoPrelude.enumFrom
//         vec![ // 18
//             COM(5,49,[0,4,1,2,4,3]), //XX(X(XXX))
//             COM(4,2,[3,0,1,0,0,0]), //XXX
//             PTR(18, false),
//             PRM(ADD,false),
//             INT(1),
//         ],
//     ]
// });

/**
Use `seq` to create new threads:
main = let a = tfAnd T
           b = ftAnd (a T) (a F)
       in seq a (tfOr b b)
 */
fn foo() {
    todo!()
}
// #[rustfmt::skip]
// pub static USE_SEQ: LazyLock<Program> = LazyLock::new(|| {
//     vec![
//          // FUN0Playground.main
//         vec![ // 0
//             PTR(2, false),
//             INT(1),
//             PTR(1, false),
//         ],
//         vec![ // 1
//             PRM(MUL,false),
//             INT(3),
//             INT(3),
//         ],
//          // FUN1Playground.sumPair
//         vec![ // 2
//             COM(4,16,[0,2,1,2,3,0]), //XX(XXX)
//             SEQ(false),
//             PTR(3, false),
//         ],
//         vec![ // 3
//             COM(4,16,[0,3,1,2,3,0]), //XX(XXX)
//             SEQ(false),
//             PRM(ADD,false),
//         ],
//     ]
// });

// #[rustfmt::skip]
// pub static BANG: LazyLock<Program> = LazyLock::new(|| {
//     vec![
//          // FUN0Playground.main
//         vec![ // 0
//             PTR(8, false),
//             PTR(7, false),
//             PTR(4, false),
//             PTR(2, false),
//         ],
//         vec![ // 1
//             PTR(22, false),
//             INT(30),
//             INT(200),
//         ],
//         vec![ // 2
//             PTR(12, false),
//             PTR(1, false),
//         ],
//         vec![ // 3
//             PTR(22, false),
//             INT(30),
//             INT(100),
//         ],
//         vec![ // 4
//             PTR(12, false),
//             PTR(3, false),
//         ],
//         vec![ // 5
//             PTR(15, false),
//             INT(1),
//             INT(50),
//         ],
//         vec![ // 6
//             PTR(12, false),
//             PTR(5, false),
//         ],
//         vec![ // 7
//             PRM(EQ,false),
//             PTR(6, false),
//             INT(50),
//         ],
//          // FUN1Playground.sumOrMul
//         vec![ // 8
//             COM(4,12,[0,1,2,3,3,0]), //X(XXX)X
//             PTR(11, false),
//             PTR(10, false),
//         ],
//         vec![ // 9
//             COM(5,40,[1,0,3,4,2,4]), //X(XXX)(XX)
//             PRM(SUB,false),
//         ],
//         vec![ // 10
//             COM(4,11,[0,2,1,3,3,0]), //XX(XX)X
//             PTR(9, false),
//             PRM(ADD,false),
//         ],
//         vec![ // 11
//             COM(4,48,[0,2,0,3,1,3]), //XX(XX(XX))
//             SEQ(false),
//         ],
//          // FUN2Playground.lastEle
//         vec![ // 12
//             COM(3,2,[2,0,1,0,0,0]), //XXX
//             ERR(0),
//             PTR(14, false),
//         ],
//         vec![ // 13
//             COM(4,1,[0,1,0,0,0,0]), //XX
//             PTR(12, false),
//         ],
//         vec![ // 14
//             COM(3,6,[2,1,0,2,0,0]), //XX(XX)
//             PTR(13, false),
//         ],
//          // FUN3NanoPrelude.enumFromTo
//         vec![ // 15
//             COM(5,15,[0,1,4,2,3,0]), //X(XX)(XX)
//             PTR(17, false),
//             PTR(16, false),
//             PTR(21, false),
//         ],
//         vec![ // 16
//             COM(3,2,[0,2,1,0,0,0]), //XXX
//             PRM(LE,false),
//         ],
//          // FUN4NanoPrelude.takeWhile
//         vec![ // 17
//             COM(3,3,[0,1,2,0,0,0]), //X(XX)
//             Y,
//             PTR(20, false),
//         ],
//         vec![ // 18
//             COM(4,6,[0,2,1,3,0,0]), //XX(XX)
//             COM(4,2,[3,0,1,0,0,0]), //XXX
//         ],
//         vec![ // 19
//             COM(5,42,[1,3,0,2,3,4]), //XXX(XXX)
//             COM(2,0,[0,0,0,0,0,0]), //X
//         ],
//         vec![ // 20
//             COM(6,48,[5,0,1,3,2,4]), //XX(XX(XX))
//             COM(2,0,[0,0,0,0,0,0]), //X
//             PTR(19, false),
//             PTR(18, false),
//         ],
//          // FUN5NanoPrelude.enumFrom
//         vec![ // 21
//             COM(5,49,[0,4,1,2,4,3]), //XX(X(XXX))
//             COM(4,2,[3,0,1,0,0,0]), //XXX
//             PTR(21, false),
//             PRM(ADD,false),
//             INT(1),
//         ],
//          // FUN6NanoPrelude.replicate
//         vec![ // 22
//             COM(4,10,[0,1,3,2,3,0]), //X(XX)XX
//             PTR(25, false),
//             PTR(24, false),
//             COM(2,0,[0,0,0,0,0,0]), //X
//         ],
//         vec![ // 23
//             COM(4,7,[0,1,3,2,0,0]), //X(XXX)
//             PTR(22, false),
//             PRM(SUB,false),
//             INT(1),
//         ],
//         vec![ // 24
//             COM(4,16,[0,3,1,2,3,0]), //XX(XXX)
//             COM(4,2,[3,0,1,0,0,0]), //XXX
//             PTR(23, false),
//         ],
//         vec![ // 25
//             COM(6,28,[0,4,1,2,5,3]), //XXX(XX)X
//             PRM(LE,false),
//             INT(0),
//         ],
//     ]
// });
