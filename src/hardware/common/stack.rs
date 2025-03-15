use crate::hardware::utils::fire;
use crate::hw_module::{HwInput, HwModule};

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

impl<T: Clone + Default> HwInput for StackInput<T> {}

/// A simple stack that enables asynchronise read and synchronise write
/// of its top element.
pub struct Stack<T: Clone + Default, const N: usize> {
    input: StackInput<T>,
    mem: Vec<T>,
}
