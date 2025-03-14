use crate::hardware::utils::fire;
use crate::hw_module::{HwModule, HwStates};
use crate::{input, local};

enum StackOp {
    NOP,  // non-op
    PUSH, // push one
    POP,  // pop one
    MOD,  // modify top
}

impl Default for StackOp {
    fn default() -> Self {
        StackOp::NOP
    }
}

#[derive(Default)]
struct StackInput<T: Clone + Default> {
    op: StackOp,
    din: T,
}

#[derive(Default)]
struct StackLocal<T: Clone + Default> {
    mem: Vec<T>,
}

/// A simple stack that enables asynchronise read and synchronise write
/// of its top element.
pub struct Stack<T: Clone + Default, const N: usize> {
    states: HwStates<StackInput<T>, StackLocal<T>>,
}
