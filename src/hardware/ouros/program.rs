use crate::hardware::ouros::combinator::{Arity, Idx, Pat};

const HOLES: usize = 6;

pub enum Atom {
    NOP,
    PTR(usize),
    COM(Arity, Pat, [Idx; HOLES]),
    INT(i32),
    PRM(u8),
    Y,
    ERR(u8),
}
