use crate::hardware::ouros::program::{Atom, Program};
use std::sync::LazyLock;
use Atom::*;

pub static A: i32 = 5;
pub static BOOL_AND: LazyLock<Program> = LazyLock::new(|| {
    vec![
        [
            PTR(11),
            PTR(22),
            PTR(33),
            PTR(44),
            PTR(55),
            INT(1),
            INT(2),
            INT(3),
        ],
        [PTR(0), INT(1), INT(2), INT(3), NOP, NOP, NOP, NOP],
    ]
});
