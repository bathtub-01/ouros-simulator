use crate::hardware::ouros::program::{AluOp, Atom, Program, SpeCell};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*;

#[rustfmt::skip]
pub static EQLIST: LazyLock<Program> = LazyLock::new(|| { Program {
    heap_img: vec![
        // AExp0
        vec![ // 0 
            COM(1,2),
            PTR(3, false, false),
        ], 
        vec![ // 1 
            PTR(7, false, false),
            INT(101),
            INT(200),
        ], 
        vec![ // 2 
            PTR(7, false, false),
            INT(1),
            INT(100),
        ], 
        vec![ // 3 
            PTR(4, false, false),
            PRM(LT,false),
            PTR(2, false, false),
            PTR(1, false, false),
        ], 
        // AExp1
        vec![ // 4 
            COM(2,3),
            PTR(6, false, false),
        ], 
        vec![ // 5 
            COM(6,9),
            COM(6,11),
        ], 
        vec![ // 6 
            COM(6,5),
            COM(2,8),
            PTR(5, false, false),
        ], 
        // AExp2
        vec![ // 7 
            COM(3,15),
            PTR(8, false, false),
        ], 
        vec![ // 8 
            COM(3,17),
            COM(1,19),
        ], 
    ],
    comb_img: vec![
        // AExp0
        vec![ // 0 
            ARG(0),
        ], 
        // AExp1
        vec![ // 1 
            ARG(1),
        ], 
        // AExp2
        vec![ // 2 
            ARG(0),
            INT(0),
            INT(42),
        ], 
        // AExp3
        vec![ // 3 
            Y,
            PTR(0, true, true),
        ], 
        vec![ // 4 
            ARG(0),
            ARG(1),
        ], 
        // AExp4
        vec![ // 5 
            ARG(4),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 6 
            ARG(1),
            ARG(2),
            ARG(3),
            ARG(5),
        ], 
        vec![ // 7 
            ARG(5),
            COM(2,1),
            ARG(0),
        ], 
        // AExp5
        vec![ // 8 
            COM(2,0),
        ], 
        // AExp6
        vec![ // 9 
            ARG(3),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 10 
            ARG(0),
            ARG(1),
            ARG(2),
            ARG(4),
            ARG(5),
        ], 
        // AExp7
        vec![ // 11 
            COM(1,14),
            PTR(1, true, true),
            PTR(0, true, true),
        ], 
        vec![ // 12 
            ARG(1),
            ARG(3),
            ARG(5),
        ], 
        vec![ // 13 
            ARG(0),
            ARG(2),
            ARG(4),
        ], 
        // AExp8
        vec![ // 14 
            ARG(0),
            COM(2,0),
        ], 
        // AExp9
        vec![ // 15 
            PRM(LE,false),
            ARG(1),
            ARG(2),
            COM(2,0),
            PTR(0, true, true),
        ], 
        vec![ // 16 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp10
        vec![ // 17 
            COM(4,21),
            ARG(1),
            PTR(0, true, true),
        ], 
        vec![ // 18 
            ARG(0),
            ARG(1),
            ARG(2),
        ], 
        // AExp11
        vec![ // 19 
            PTR(7, false, false),
            PTR(0, true, true),
        ], 
        vec![ // 20 
            PRM(ADD,false),
            ARG(0),
            INT(1),
        ], 
        // AExp12
        vec![ // 21 
            ARG(3),
            ARG(0),
            ARG(1),
        ], 
    ],

}});

// #[rustfmt::skip]
// pub static EQLIST: LazyLock<Program> = LazyLock::new(|| {
//     Program {
//         heap_img: vec![
//             vec![
//                 COM(2, 4),
//                 PTR(1, false, false),
//                 PTR(2, false, false)
//             ],
//             vec![
//                 COM(2, 15), // enumFromTo
//                 INT(1),
//                 INT(100)
//             ],
//             vec![
//                 COM(2, 15), // enumFromTo
//                 INT(101),
//                 INT(200)
//             ]
//         ],
//         comb_img: vec![
//             // True 0
//             vec![ARG(1)],
//             // False 1
//             vec![ARG(0)],
//             // [] 2 (arity 2)
//             vec![ARG(0)],
//             // : 3 (arity 4)
//             vec![ARG(3), ARG(0), ARG(1)],
//             // eqList 4
//             vec![ARG(0), TAB(5, 1), ARG(1)],
//             // 5 C2
//             vec![ARG(1), TAB(7, 0)],
//             // 6 C3
//             vec![ARG(3), TAB(9, 2), ARG(0), ARG(1)],
//             // 7 C4
//             vec![CON(1, 0, 1)],
//             // 8 C5
//             vec![CON(1, 0, 0)],
//             // 9 C6
//             vec![CON(1, 0, 0)],
//             // 10
//             vec![COM(2, 23),
//                  SPE(LE, false, SpeCell::ARG(3), SpeCell::ARG(0), 0),
//                  // PTR(0, true, true),
//                  PTR(1, true, true)
//             ],
//             vec![PRM(LE, false), ARG(3), ARG(0)],
//             vec![COM(2, 4), ARG(4), ARG(1)],
//             vec![],
//             // and 14
//             vec![ARG(0), COM(2, 1), ARG(1)],
//             // enumFromTo 15
//             vec![PRM(LE, false), ARG(0), ARG(1), TAB(16, 2), ARG(0), ARG(1)],
//             // 16
//             vec![CON(1, 0, 0)],
//             // 17
//             vec![COM(2, 19), ARG(1), PTR(0, true, true)],
//             vec![COM(2, 21), ARG(1), ARG(2)],
//             // 19
//             vec![TRY, PTR(0, true, true), ARG(1)],
//             vec![CON(3, 2, 1), ARG(0), ARG(1)],
//             // 21
//             vec![COM(2, 15),
//                  SPE(ADD, false, SpeCell::ARG(0), SpeCell::LIT(1), 0),
//                  // PTR(0, true, true),
//                  ARG(1)
//             ],
//             vec![PRM(ADD, false), ARG(0), INT(1)],
//             // and 23
//             vec![ARG(0), TAB(24, 1), ARG(1)],
//             vec![CON(1, 0, 0)],
//             vec![ARG(1)]
//         ]
//     }
// });
