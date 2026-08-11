use crate::hw_module::{HwInput, HwModule};
use vstd::prelude::*;

verus! {
#[derive(Default, Debug, PartialEq)]
pub enum StackOp {
    #[default]
    Nop, // non-op
    Push, // push one
    Pop,  // pop one
    Mod,  // modify top
}

#[derive(Default, Debug)]
pub struct StackInput<T: Clone + Default> {
    pub op: StackOp,
    pub din: T,
}

impl<T: Clone + Default> HwInput for StackInput<T> {
    fn default_input(&mut self) {
        self.op = StackOp::Nop;
    }
}

pub assume_specification<T> [<StackInput<T> as Default>::default] () -> StackInput<T>
where
    T: std::default::Default + std::clone::Clone + std::default::Default,;

/// A simple stack that enables asynchronise read and synchronise write
/// of its top element. Also allows reading top-1 element.
pub struct Stack<T: Clone + Default, const N: usize> {
    pub input: StackInput<T>,
    pub mem: Vec<T>,
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
        self.mem.last()
    }

    /// second element of the stack
    pub fn second(&self) -> Option<&T> {
        if self.mem.len() <= 1 {
            None
        } else {
            self.mem.get(self.mem.len() - 2)
        }
    }

    /// current depth of the stack
    pub fn elements(&self) -> usize {
        self.mem.len()
    }

    pub fn push(&mut self, din: T) {
        self.input.op = StackOp::Push;
        self.input.din = din;
    }

    pub fn pop(&mut self) {
        self.input.op = StackOp::Pop;
    }

    pub fn modify(&mut self, din: T) {
        self.input.op = StackOp::Mod;
        self.input.din = din;
    }
}

impl<T: Clone + Default, const N: usize> HwModule for Stack<T, N> {
    fn update_local(&mut self) 
    ensures
    self.input.op is NOP ==> self.mem == old(self).mem,
    self.input.op is PUSH ==> self.mem@.len() > old(self).mem@.len(),
    self.input.op is POP && old(self).mem@.len() > 0 ==> self.mem@.len() < old(self).mem@.len(),
    self.input.op is MOD && old(self).mem@.len() > 0 ==> self.mem@.len() == old(self).mem@.len(),
    {
        match self.input.op {
            StackOp::Nop => {}
            StackOp::Push => {
                self.mem.push(self.input.din.clone());
            }
            StackOp::Pop => {
                self.mem.pop();
            }
            StackOp::Mod => {
                self.mem.pop();
                self.mem.push(self.input.din.clone());
            }
        }
    }

    fn tick_children(&mut self) {}
}

    fn push(vec: &mut Vec<bool>, b: bool)
    ensures old(vec).len() < vec.len()
    {vec.push(b);}
}

#[test]
fn stack_spec() {
    let mut stack: Stack<u32, 64> = Stack::new();

    stack.tick();

    for i in 0..64 {
        stack.input.link(|input| {
            input.op = StackOp::Push;
            input.din = i;
        });
        stack.tick();
    }

    for i in 0..64 {
        assert_eq!(stack.top(), Some(&(63 - i)));

        stack.input.link(|input| {
            input.op = StackOp::Mod;
            input.din = i;
        });
        stack.tick();
        assert_eq!(stack.top(), Some(&i));

        stack.input.link(|input| {
            input.op = StackOp::Pop;
        });
        stack.tick();
    }
}
