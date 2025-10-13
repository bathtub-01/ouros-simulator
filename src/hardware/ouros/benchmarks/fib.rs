use crate::hardware::ouros::program::{AluOp, Atom, Program, SpeCell};
use std::sync::LazyLock;
use AluOp::*;
use Atom::*;

#[rustfmt::skip]
pub static FIB: LazyLock<Program> = LazyLock::new(|| {
    Program {
        heap_img: vec![
            vec![COM(1, 2), INT(17)]
        ],
        comb_img: vec![
            // True 0
            vec![ARG(1)],
            // False 1
            vec![ARG(0)],
            // fib 2
            vec![
                PRM(LE, true),
                ARG(0),
                INT(1),
                TAB(3, 1),
                ARG(0)
                // PTR(0, true, true),
                // INT(1),
            ],
            // 3
            vec![
                INT(1)
            ],
            // 4
            vec![
                PRM(ADD, false),
                PTR(0, true, true),
                PTR(1, true, true),
            ],
            vec![
                COM(1, 7),
                ARG(1)
            ],
            vec![
                COM(1, 9),
                ARG(1)
            ],
            // 7
            vec![
                COM(1, 2),
                SPE(SUB, false, SpeCell::ARG(0), SpeCell::LIT(1), 0),
                // PTR(0, true, true),
            ],            
            vec![
                PRM(SUB, false),
                ARG(0),
                INT(1)
            ],
            // 9
            vec![
                COM(1, 2),
                SPE(SUB, false, SpeCell::ARG(0), SpeCell::LIT(2), 0),
                // PTR(0, true, true),
            ],            
            vec![
                PRM(SUB, false),
                ARG(0),
                INT(2)
            ],
        ]
    }
});
