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
    /// AXI4-Lite primitive: false = read, true = write.
    IO(bool),
    Y,
    Seq,
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
            Atom::IO(is_write) => write!(f, "IO({})", is_write),
            Atom::Y => write!(f, "Y"),
            Atom::Seq => write!(f, "SEQ"),
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
        IO(is_write) => io_arity(*is_write) as u8,
        Int(_) => 1,
        Y => 1,
        Seq => 2,
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
    matches!(atom, Atom::Seq)
}

// pub fn is_seq_evaluated(atom: &Atom) -> bool {
//     match atom {
//         Atom::Seq(evaluated) => *evaluated,
//         _ => false,
//     }
// }

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
            Err("atom not an INT: {:?}".to_string())
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

/// IO arguments use the existing INT representation (including World).
/// The address and data are 32-bit bit patterns carried by `i32`; consumers
/// interpret them as `u32` at the AXI4-Lite boundary. No new World Atom exists.
/// Read  [IO(false), INT(addr), INT(world)]
/// Write [IO(true),  INT(addr), INT(data), INT(world)]
pub const fn io_arity(is_write: bool) -> usize {
    if is_write { 3 } else { 2 }
}

pub fn is_io(atom: &Atom) -> bool {
    matches!(atom, Atom::IO(_))
}

pub fn io_strict_positions(atom: &Atom) -> &'static [usize] {
    match atom {
        Atom::IO(false) => &[1, 2],
        Atom::IO(true) => &[1, 2, 3],
        _ => &[],
    }
}

/// A partial IO application is a value, not a request. Over-applied requests
/// are also saturated; trailing arguments must be preserved by the IO unit.
pub fn io_is_saturated(app: &App) -> bool {
    match app[0] {
        Atom::IO(is_write) => app_length(app) > io_arity(is_write),
        _ => false,
    }
}

pub fn io_is_ready(app: &App) -> bool {
    io_is_saturated(app)
        && io_strict_positions(&app[0])
            .iter()
            .all(|&idx| is_int(&app[idx]))
}

/// The first as-yet-unboxed strict IO operand after `after`, if any.
/// Used by DHeap without confusing IO's three possible arguments with PRM's two.
pub fn io_next_pointer(app: &App, after: usize) -> Option<(usize, usize)> {
    if !io_is_saturated(app) {
        return None;
    }
    io_strict_positions(&app[0])
        .iter()
        .filter(|&&idx| idx > after)
        .find_map(|&idx| match app[idx] {
            Atom::Ptr(p, _, false) => Some((idx, p)),
            _ => None,
        })
}

#[cfg(test)]
mod io_program_tests {
    use super::*;

    #[test]
    fn io_arity_partial_and_ready() {
        let mut app = [Atom::Nop; APP_LENGTH];
        app[0] = Atom::IO(false);
        assert_eq!(arity_of(&app[0]), 2);
        assert!(is_whnf(&app));
        app[1] = Atom::Int(10);
        assert!(is_whnf(&app));
        app[2] = Atom::Int(0);
        assert!(io_is_saturated(&app));
        assert!(io_is_ready(&app));
        assert!(!is_whnf(&app));
        assert_eq!(format!("{}", app[0]), "IO(false)");

        app[0] = Atom::IO(true);
        assert_eq!(arity_of(&app[0]), 3);
        assert!(is_whnf(&app));
        app[3] = Atom::Ptr(20, false, false);
        assert!(io_is_saturated(&app));
        assert!(!io_is_ready(&app));
        assert_eq!(io_next_pointer(&app, 0), Some((3, 20)));
        app[3] = Atom::Int(0);
        assert!(io_is_ready(&app));
    }

    #[test]
    fn io_operands_and_over_application() {
        let mut app = [Atom::Nop; APP_LENGTH];
        app[0] = Atom::IO(true);
        app[1] = Atom::Ptr(7, false, false);
        app[2] = Atom::Ptr(8, false, false);
        app[3] = Atom::Ptr(9, false, false);
        app[4] = Atom::Ptr(10, false, false); // non-strict extra argument
        assert_eq!(io_next_pointer(&app, 0), Some((1, 7)));
        assert_eq!(io_next_pointer(&app, 1), Some((2, 8)));
        assert_eq!(io_next_pointer(&app, 2), Some((3, 9)));
        assert_eq!(io_next_pointer(&app, 3), None);
        app[1] = Atom::Int(-1);
        app[2] = Atom::Int(0);
        app[3] = Atom::Int(1);
        assert!(io_is_ready(&app));
    }
}
