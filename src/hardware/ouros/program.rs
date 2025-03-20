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

#[derive(Default, Clone)]
pub struct ActiveApp {
    pub stack_idx: u8,
    pub load: App,
}

#[derive(Default, Clone)]
pub struct FrozenApp {
    pub heap_addr: usize,
    pub load: [Atom; HOLES - 1],
}

/// For compiler generated programs. Using `Vec<Vec<Atom>>` instead of
/// `Vec<App>` will make the compiler side easier..
pub type Program = Vec<Vec<Atom>>;
