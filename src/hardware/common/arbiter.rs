use crate::error;
use crate::hardware::common::Register;
use crate::hw_module::{HwInput, HwModule};

#[derive(Default)]
pub struct ArbiterInput<T: Clone + Default> {
    pub in_valid: Vec<bool>,
    pub in_bits: Vec<T>,
    pub out_ready: bool,
}

impl<T: Clone + Default> HwInput for ArbiterInput<T> {}

/// N:1 round-robin arbiter. As this is mainly a combinatory circuit,
/// always give the inputs before use its outputs.
pub struct RArbiter<T: Clone + Default, const N: usize> {
    pub input: ArbiterInput<T>,
    priority: Register<usize>,
}

impl<T: Clone + Default, const N: usize> RArbiter<T, N> {
    pub fn new() -> Self {
        Self {
            input: ArbiterInput {
                in_valid: vec![false; N],
                in_bits: vec![T::default(); N],
                out_ready: true,
            },
            priority: Default::default(),
        }
    }

    pub fn in_ready(&self, n: usize, select: Option<usize>) -> bool {
        if self.input.out_ready {
            match select {
                None => false,
                Some(p) => p == n,
            }
        } else {
            false
        }
    }

    pub fn out_bits(&self, select: Option<usize>) -> Option<&T> {
        match select {
            None => None,
            Some(p) => Some(&self.input.in_bits[p]),
        }
    }

    pub fn out_valid(&self) -> bool {
        self.input.in_valid.iter().any(|&x| x)
    }

    pub fn select(&self) -> Result<usize, error::sim::local::RArbiter> {
        for i in 0..N {
            let port = (i + self.priority.value()) % N;
            if self.input.in_valid[port] {
                return Ok(port);
            }
        }
        Err(error::sim::local::RArbiter::Selection)
    }
}

impl<T: Clone + Default, const N: usize> HwModule for RArbiter<T, N> {
    fn update_local(&mut self) -> Result<(), error::sim::TickHw> {
        self.select()
            .map_err(|r_arbiter_error| {
                error::sim::TickHw::Rarbiter(r_arbiter_error, error::sim::SimState)
            })
            .map(|p| self.priority.connect(&((p + 1) % N)))
    }

    fn tick_children(&mut self) -> std::result::Result<(), error::sim::TickHw> {
        self.priority.tick()
    }
}

/// N:1 priority arbiter, always grant access to ports at lower index.
/// As this is mainly a combinatory circuit, always give the inputs before use its outputs.
pub struct PArbiter<T: Clone + Default, const N: usize> {
    pub input: ArbiterInput<T>,
}

impl<T: Clone + Default, const N: usize> PArbiter<T, N> {
    pub fn new() -> Self {
        Self {
            input: ArbiterInput {
                in_valid: vec![false; N],
                in_bits: vec![T::default(); N],
                out_ready: true,
            },
        }
    }

    pub fn in_ready(&self, n: usize, select: Option<usize>) -> bool {
        if self.input.out_ready {
            match select {
                None => false,
                Some(p) => p == n,
            }
        } else {
            false
        }
    }

    pub fn out_bits(&self, select: Option<usize>) -> Option<&T> {
        match select {
            None => None,
            Some(p) => Some(&self.input.in_bits[p]),
        }
    }

    pub fn out_valid(&self) -> bool {
        self.input.in_valid.iter().any(|&x| x)
    }

    pub fn select(&self) -> Option<usize> {
        self.input.in_valid.iter().position(|&vld| vld)
    }
}

impl<T: Clone + Default, const N: usize> HwModule for PArbiter<T, N> {
    // no local states to update
    fn update_local(&mut self) -> Result<(), error::sim::TickHw> {
        Ok(())
    }
    fn tick_children(&mut self) -> std::result::Result<(), error::sim::TickHw> {
        Ok(())
    }
}

#[test]
fn arbiter_spec() {
    let mut arbiter: RArbiter<u32, 4> = RArbiter::new();

    arbiter.tick().expect("expected to tick arbitrer");

    for i in 0..50 {
        arbiter.input.link(|input| {
            input.in_bits.iter_mut().zip(0..4).for_each(|(b, idx)| {
                *b = idx + 10;
            });
            input.in_valid.iter_mut().for_each(|v| {
                *v = true;
            });
            input.out_ready = true;
        });
        // println!("priority: {}", arbiter.states.local.priority.value());

        let select = arbiter.select().expect("should have selected arbiter");
        assert_eq!(select, (i % 4) as usize);
        assert_eq!(arbiter.out_bits(Some(select)), Some(&(i % 4 + 10)));

        for p in 0..4 {
            assert_eq!(arbiter.in_ready(p, Some(select)), p == select);
        }

        arbiter.tick().expect("expected to tick arbitrer");
    }
}
