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

#[derive(Clone, PartialEq, Debug)]
pub enum Atom {
    NOP,
    PTR(usize),
    COM(Arity, u8, [Idx; HOLES]), // represent Pat with an u8
    INT(i32),
    PRM(AluOp, bool), // (operator, conditon revert bit)
    Y,
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
            Atom::PTR(p) => write!(f, "PTR({})", p),
            Atom::COM(arity, pat, holes) => {
                // Format the array of Idx values as a comma-separated list
                let holes_str = holes
                    .iter()
                    .take(holes_of(*pat))
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "COM({}, {}, [{}])", arity, pat, holes_str)
            }
            Atom::INT(n) => write!(f, "INT({})", n),
            Atom::PRM(p, inv) => write!(f, "PRM({:?}, {})", p, inv),
            Atom::Y => write!(f, "Y"),
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
    pub load: [Atom; HOLES - 1],
}

/// For compiler generated programs. Using `Vec<Vec<Atom>>` instead of
/// `Vec<App>` will make the compiler side easier..
pub type Program = Vec<Vec<Atom>>;

fn arity_of(atom: &Atom) -> u8 {
    use Atom::*;
    match atom {
        COM(a, _, _) => *a,
        PRM(_, _) => 2,
        Y => 1,
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
    let a: App = [PTR(0), INT(1), INT(2), INT(3), NOP, NOP, NOP, NOP];
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
