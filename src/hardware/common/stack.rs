use crate::hw_module::{HwInput, HwModule};

#[derive(Default)]
pub enum StackOp {
    #[default]
    NOP, // non-op
    PUSH, // push one
    POP,  // pop one
    MOD,  // modify top
}

#[derive(Default)]
pub struct StackInput<T: Clone + Default> {
    pub op: StackOp,
    pub din: T,
}

impl<T: Clone + Default> HwInput for StackInput<T> {
    fn default_input(&mut self) {
        self.link(|input| {
            input.op = StackOp::NOP;
            // Don't care default input.din
        });
    }
}

/// A simple stack that enables asynchronise read and synchronise write
/// of its top element. Also allows reading top-1 element.
pub struct Stack<T: Clone + Default, const N: usize> {
    pub input: StackInput<T>,
    mem: Vec<T>,
}

impl<T: Clone + Default, const N: usize> Stack<T, N> {
    pub fn new() -> Self {
        Self {
            input: Default::default(),
            mem: Vec::with_capacity(N),
        }
    }

    /// top element of the stack
    pub fn top(&self) -> Option<&T> {
        match self.mem.last() {
            None => None,
            Some(v) => Some(v),
        }
    }

    /// second element of the stack
    pub fn second(&self) -> Option<&T> {
        match self.mem.get(self.mem.len() - 2) {
            None => None,
            Some(v) => Some(v),
        }
    }

    /// current depth of the stack
    pub fn elements(&self) -> usize {
        self.mem.len()
    }
}

impl<T: Clone + Default, const N: usize> HwModule for Stack<T, N> {
    fn update_local(&mut self) {
        match self.input.op {
            StackOp::NOP => {}
            StackOp::PUSH => {
                self.mem.push(self.input.din.clone());
            }
            StackOp::POP => {
                self.mem.pop();
            }
            StackOp::MOD => {
                self.mem.pop();
                self.mem.push(self.input.din.clone());
            }
        }
    }

    fn tick_children(&mut self) {}
}

#[test]
fn stack_spec() {
    let mut stack: Stack<u32, 64> = Stack::new();

    stack.tick();

    for i in 0..64 {
        stack.input.link(|input| {
            input.op = StackOp::PUSH;
            input.din = i;
        });
        stack.tick();
    }

    for i in 0..64 {
        assert_eq!(stack.top(), Some(&(63 - i)));

        stack.input.link(|input| {
            input.op = StackOp::MOD;
            input.din = i;
        });
        stack.tick();
        assert_eq!(stack.top(), Some(&i));

        stack.input.link(|input| {
            input.op = StackOp::POP;
        });
        stack.tick();
    }
}
