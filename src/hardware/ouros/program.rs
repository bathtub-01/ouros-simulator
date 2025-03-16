use crate::hardware::ouros::combinator::{Arity, Idx};
use crate::hardware::ouros::config::{APP_LENGTH, HOLES};

#[derive(Clone, PartialEq, Debug)]
pub enum Atom {
    NOP,
    PTR(usize),
    COM(Arity, u8, [Idx; HOLES]), // represent Pat with an u8
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
pub type DataFlowLink<T> = (u8, T);
