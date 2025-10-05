use crate::hardware::ouros::combinator::{holes_of, Arity, Idx};
use crate::hardware::ouros::config::{APP_LENGTH, HOLES};
use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub enum AluOp {
    EQ,
    LE,
    LT,
    ADD,
    SUB,
    MUL,
}

type Unique = bool;
type NewCell = bool;
type RevCond = bool;

#[derive(Clone, PartialEq, Debug)]
pub enum Atom {
    NOP,
    PTR(usize, Unique, NewCell),
    COM(Arity, usize),
    INT(i32),
    PRM(AluOp, RevCond),
    Y,
    SEQ(bool),
    ARG(usize),
    ERR(u8),
}

impl Default for Atom {
    fn default() -> Self {
        Atom::NOP
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom::NOP => write!(f, "NOP"),
            Atom::PTR(p, unique, _) => write!(f, "PTR({},{})", p, unique),
            Atom::COM(arity, ptr) => write!(f, "COM({}, {})", arity, ptr),
            Atom::INT(n) => write!(f, "INT({})", n),
            Atom::PRM(p, inv) => write!(f, "PRM({:?}, {})", p, inv),
            Atom::Y => write!(f, "Y"),
            Atom::SEQ(evaluated) => write!(f, "SEQ({})", evaluated),
            Atom::ARG(arg) => write!(f, "ARG({})", arg),
            Atom::ERR(e) => write!(f, "ERR({})", e),
        }
    }
}

pub type App = [Atom; APP_LENGTH];

#[derive(Default, Clone, Debug)]
pub struct ActiveApp {
    pub stack_idx: u8,
    pub load: App,
}

#[derive(Default, Clone, Debug)]
pub struct FrozenApp {
    pub heap_addr: usize,
    pub load: App,
}

/// For compiler generated programs. Using `Vec<Vec<Atom>>` instead of
/// `Vec<App>` will make the compiler side easier..
// pub type Program = Vec<Vec<Atom>>;
pub struct Program {
    pub heap_img: Vec<Vec<Atom>>,
    pub comb_img: Vec<Vec<Atom>>,
}

fn arity_of(atom: &Atom) -> u8 {
    use Atom::*;
    match atom {
        COM(a, _) => *a,
        PRM(_, _) => 2,
        INT(_) => 1,
        Y => 1,
        SEQ(_) => 2,
        _ => 0,
    }
}

/// The length of an application, stripping off NOPs.
pub fn app_length(app: &App) -> usize {
    let found = app.iter().enumerate().find(|&(_, atom)| *atom == Atom::NOP);
    match found {
        Some((idx, _)) => idx,
        None => APP_LENGTH,
    }
}

#[test]
fn app_length_spec() {
    use Atom::*;
    let a: App = [
        PTR(0, false, false),
        INT(1),
        INT(2),
        INT(3),
        NOP,
        NOP,
        NOP,
        NOP,
    ];
    let b: App = [Y, Y, Y, Y, Y, Y, Y, Y];
    assert_eq!(app_length(&a), 4);
    assert_eq!(app_length(&b), APP_LENGTH);
}

/// Determine whether an Application is in Weak-Head-Normal-Form.
pub fn is_whnf(app: &App) -> bool {
    // +, a, b --- false
    // +, a    --- true
    arity_of(&app[0]) >= app_length(app) as u8
}

pub fn is_seq(atom: &Atom) -> bool {
    match atom {
        Atom::SEQ(_) => true,
        _ => false,
    }
}

pub fn is_seq_evaluated(atom: &Atom) -> bool {
    match atom {
        Atom::SEQ(evaluated) => *evaluated,
        _ => false,
    }
}

pub fn is_nop(atom: &Atom) -> bool {
    match atom {
        Atom::NOP => true,
        _ => false,
    }
}

pub fn is_ptr(atom: &Atom) -> bool {
    match atom {
        Atom::PTR(_, _, _) => true,
        _ => false,
    }
}

pub fn get_ptr(atom: &Atom) -> usize {
    match atom {
        Atom::PTR(pt, _, _) => *pt,
        _ => unimplemented!(),
    }
}

pub fn is_unique_ptr(a: &Atom) -> bool {
    match a {
        Atom::PTR(_, true, _) => true,
        _ => false,
    }
}

pub fn is_new(atom: &Atom) -> bool {
    match atom {
        Atom::PTR(_, _, new) => *new,
        _ => false,
    }
}

pub fn is_lit_atom(atom: &Atom) -> bool {
    match atom {
        Atom::NOP | Atom::PTR(_, _, _) => false,
        _ => true,
    }
}

pub fn is_comb(atom: &Atom) -> bool {
    match atom {
        Atom::COM(_, _) | Atom::Y => true,
        _ => false,
    }
}

pub fn get_comb_addr(atom: &Atom) -> usize {
    match atom {
        Atom::COM(_, addr) => *addr,
        _ => unimplemented!(),
    }
}

pub fn is_int(atom: &Atom) -> bool {
    match atom {
        Atom::INT(_) => true,
        _ => false,
    }
}

pub fn is_prm(atom: &Atom) -> bool {
    match atom {
        Atom::PRM(_, _) => true,
        _ => false,
    }
}
