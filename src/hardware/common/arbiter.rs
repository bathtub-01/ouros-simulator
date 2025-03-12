use crate::hardware::common::Register;
use crate::hardware::utils::fire;
use crate::hw_module::{HwModule, HwStates};
use crate::{input, local};

#[derive(Default)]
struct ArbiterInput<T: Clone + Default> {
    in_valid: Vec<bool>,
    in_bits: Vec<T>,
    out_ready: bool,
}

#[derive(Default)]
struct ArbiterLocal {
    priority: Register<usize>,
}

/// N:1 round-robin arbiter.
#[derive(Default)]
pub struct Arbiter<T: Clone + Default, const N: usize> {
    states: HwStates<ArbiterInput<T>, ArbiterLocal>,
}

impl<T: Clone + Default, const N: usize> Arbiter<T, N> {
    fn new() -> Self {
        Self {
            states: HwStates {
                input: ArbiterInput {
                    in_valid: vec![false; N],
                    in_bits: vec![T::default(); N],
                    out_ready: true,
                },
                local: Default::default(),
            },
        }
    }

    fn in_ready(&self, n: usize, select: Option<usize>) -> bool {
        if input!(self).out_ready {
            match select {
                None => false,
                Some(p) => p == n,
            }
        } else {
            false
        }
    }

    fn out_bits(&self, select: Option<usize>) -> Option<&T> {
        match select {
            None => None,
            Some(p) => Some(&input!(self).in_bits[p]),
        }
    }

    fn out_valid(&self) -> bool {
        input!(self).in_valid.iter().any(|&x| x)
    }

    fn select(&self) -> Option<usize> {
        if input!(self).out_ready {
            for i in 0..N {
                let port = (i + local!(self).priority.value()) % N;
                if input!(self).in_valid[port] {
                    return Some(port);
                }
            }
        }
        None
    }
}

impl<T: Clone + Default, const N: usize> HwModule for Arbiter<T, N> {
    fn update_local(&mut self) {
        match self.select() {
            None => {}
            Some(p) => local!(self).priority.connect(&((p + 1) % N)),
        }
    }

    fn tick_children(&mut self) {
        local!(self).priority.tick();
    }
}

#[test]
fn arbiter_spec() {
    let mut arbiter: Arbiter<u32, 4> = Arbiter::new();

    arbiter.tick();

    for i in 0..50 {
        arbiter.states.link_input(|input| {
            input.in_bits.iter_mut().zip(0..4).for_each(|(b, idx)| {
                *b = idx + 10;
            });
            input.in_valid.iter_mut().for_each(|v| {
                *v = true;
            });
            input.out_ready = true;
        });
        // println!("priority: {}", arbiter.states.local.priority.value());

        let select = arbiter.select();
        assert_eq!(select, Some((i % 4) as usize));
        assert_eq!(arbiter.out_bits(select), Some(&(i % 4 + 10)));

        for p in 0..4 {
            assert_eq!(arbiter.in_ready(p, select), Some(p) == select);
        }

        arbiter.tick();
    }
}
