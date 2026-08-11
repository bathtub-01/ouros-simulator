use crate::hardware::ouros::combinator::{holes_of, Arity, Idx};
use crate::hardware::ouros::config::{APP_LENGTH, HOLES};
use std::fmt;

#[derive(Clone, Copy, PartialEq, Debug)]
// FIX: should we fix capitalisation of variant names?
pub enum AluOp {
    EQ,
    LE,
    LT,
    Add,
    Sub,
    Mul,
}

type Unique = bool;
type NewCell = bool;
type RevCond = bool;
type Index = usize;
type Fields = usize;
type FreeVars = u8;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SpeCell {
    Arg(usize),
    Lit(i32),
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum Atom {
    #[default]
    Nop,
    Ptr(usize, Unique, NewCell),
    Com(Arity, usize),
    Con(Arity, Fields, Index),
    Tab(usize, FreeVars),
    Int(i32),
    Prm(AluOp, RevCond),
    Y,
    Seq(bool),
    Arg(usize, Unique),
    Try,
    Spe(AluOp, RevCond, SpeCell, SpeCell, usize),
    E(u8),
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom::Nop => write!(f, "NOP"),
            Atom::Ptr(p, unique, _) => write!(f, "PTR({},{})", p, unique),
            Atom::Com(arity, ptr) => write!(f, "COM({}, {})", arity, ptr),
            Atom::Int(n) => write!(f, "INT({})", n),
            Atom::Prm(p, inv) => write!(f, "PRM({:?}, {})", p, inv),
            Atom::Y => write!(f, "Y"),
            Atom::Seq(evaluated) => write!(f, "SEQ({})", evaluated),
            Atom::Arg(arg, _) => write!(f, "ARG({})", arg),
            Atom::E(e) => write!(f, "ERR({})", e),
            Atom::Try => write!(f, "TRY"),
            Atom::Spe(alu_op, _, spe_cell, spe_cell1, _) => write!(f, "SPE"),
            Atom::Con(a, fields, i) => write!(f, "CON({}, {}, {})", a, fields, i),
            Atom::Tab(base, fv) => write!(f, "TAB({}, {})", base, fv),
        }
    }
}

// FIX: we should wrap this into a new type that checks the bounds of lookups
// we may trigger null pointer exceptions, if we dont do that
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

pub fn arity_of(atom: &Atom) -> u8 {
    use Atom::*;
    match atom {
        Com(a, _) => *a,
        Con(a, _, _) => *a,
        Prm(_, _) => 2,
        Int(_) => 1,
        Y => 1,
        Seq(_) => 2,
        Try => 2,
        _ => 0,
    }
}

/// The length of an application, stripping off NOPs.
pub fn app_length(app: &App) -> usize {
    let found = app.iter().enumerate().find(|&(_, atom)| *atom == Atom::Nop);
    match found {
        Some((idx, _)) => idx,
        None => APP_LENGTH,
    }
}

#[test]
fn app_length_spec() {
    use Atom::*;
    let a: App = [
        Ptr(0, false, false),
        Int(1),
        Int(2),
        Int(3),
        Nop,
        Nop,
        Nop,
        Nop,
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
    matches!(atom, Atom::Seq(_))
}

pub fn is_seq_evaluated(atom: &Atom) -> bool {
    match atom {
        Atom::Seq(evaluated) => *evaluated,
        _ => false,
    }
}

pub fn is_nop(atom: &Atom) -> bool {
    matches!(atom, Atom::Nop)
}

pub fn is_ptr(atom: &Atom) -> bool {
    matches!(atom, Atom::Ptr(_, _, _))
}

/// take pointer value out from a PTR or SPE
pub fn get_ptr(atom: &Atom) -> usize {
    match atom {
        Atom::Ptr(pt, _, _) => *pt,
        Atom::Spe(_, _, _, _, pt) => *pt,
        _ => unimplemented!(),
    }
}

pub fn is_unique_ptr(a: &Atom) -> bool {
    matches!(a, Atom::Ptr(_, true, _))
}

pub fn is_new(atom: &Atom) -> bool {
    match atom {
        Atom::Ptr(_, _, new) => *new,
        _ => false,
    }
}

pub fn is_lit_atom(atom: &Atom) -> bool {
    !matches!(atom, Atom::Nop | Atom::Ptr(_, _, _))
}

pub fn is_comb(atom: &Atom) -> bool {
    matches!(atom, Atom::Com(_, _) | Atom::Y)
}

pub fn get_comb_addr(atom: &Atom) -> usize {
    match atom {
        Atom::Com(_, addr) => *addr,
        _ => unimplemented!(),
    }
}

pub fn is_int(atom: &Atom) -> bool {
    matches!(atom, Atom::Int(_))
}

pub fn take_int(atom: &Atom) -> Result<i32, String> {
    match *atom {
        Atom::Int(i) => Ok(i),
        _ => {
            return Err("atom not an INT: {:?}".to_string());
        }
    }
}

pub fn is_prm(atom: &Atom) -> bool {
    matches!(atom, Atom::Prm(_, _))
}

pub fn is_try(atom: &Atom) -> bool {
    matches!(atom, Atom::Try)
}

pub fn is_con(atom: &Atom) -> bool {
    matches!(atom, Atom::Con(_, _, _))
}
