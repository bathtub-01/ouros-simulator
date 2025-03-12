use crate::hardware::ouros::combinator::{Arity, Idx, Pat};

pub const HOLES: usize = 6;
pub const APP_LENGTH: usize = 8;

pub enum Atom {
    NOP,
    PTR(usize),
    COM(Arity, Pat, [Idx; HOLES]),
    INT(i32),
    PRM(u8),
    Y,
    ERR(u8),
}

impl Default for Atom {
    fn default() -> Self {
        Atom::NOP
    }
}

pub type App = [Atom; APP_LENGTH];
