//! A bounded ring of `par` evaluation hints. No spark is evaluated locally by
//! inserting it; a future work-stealer may claim one with `pop_ready`.
//!
//! Root scanning uses a separate, indexed, combinational read port. GC holds
//! `gc_lock` throughout its root scan, so neither insertion nor removal can
//! invalidate the scan. Normal offers are *lossy* only when the FIFO is full.

use crate::hw_module::{HwInput, HwModule};
use super::program::Atom;

#[derive(Default)]
pub struct SparkPoolInput {
    pub offer_valid: bool,
    pub offer_bits: Atom,
    pub pop_ready: bool,
    pub gc_lock: bool,
}

impl HwInput for SparkPoolInput {}

pub struct SparkPool<const N: usize> {
    pub input: SparkPoolInput,
    mem: [Atom; N],
    head: usize,
    tail: usize,
    count: usize,
    pub offered: u64,
    pub dropped: u64,
}

impl<const N: usize> SparkPool<N> {
    pub fn new() -> Self {
        assert!(N > 0, "spark pool must have at least one entry");
        Self {
            input: Default::default(),
            mem: [Atom::Nop; N],
            head: 0,
            tail: 0,
            count: 0,
            offered: 0,
            dropped: 0,
        }
    }

    pub fn len(&self) -> usize { self.count }
    pub fn is_empty(&self) -> bool { self.count == 0 }
    pub fn is_full(&self) -> bool { self.count == N }

    /// A logical index, oldest first; a GC lookup does not dequeue anything.
    pub fn search(&self, index: usize) -> Option<Atom> {
        if index >= self.count { None } else { Some(self.mem[(self.head + index) % N]) }
    }

    pub fn front(&self) -> Option<Atom> { self.search(0) }

    /// No backpressure due to capacity: saturated pools silently drop hints.
    /// GC locking is handled at DHeap port A, not by dropping an offer here.
    pub fn offer_ready(&self) -> bool { !self.input.gc_lock }

    /// Whether an offer will actually be retained (rather than discarded).
    pub fn offer_will_store(&self) -> bool {
        self.offer_ready() && (self.count < N || (self.input.pop_ready && self.count > 0))
    }
}

impl<const N: usize> HwModule for SparkPool<N> {
    fn update_local(&mut self) -> Result<(), String> {
        if self.input.gc_lock {
            // The producer must be held upstream; never lose a spark due to GC.
            debug_assert!(!self.input.offer_valid);
            return Ok(());
        }
        let pop = self.input.pop_ready && self.count > 0;
        if pop {
            self.mem[self.head] = Atom::Nop;
            self.head = (self.head + 1) % N;
            self.count -= 1;
        }
        if self.input.offer_valid {
            self.offered += 1;
            if self.count < N {
                self.mem[self.tail] = self.input.offer_bits;
                self.tail = (self.tail + 1) % N;
                self.count += 1;
            } else {
                self.dropped += 1;
            }
        }
        Ok(())
    }

    fn tick_children(&mut self) -> Result<(), String> { Ok(()) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fifo_wrap_scan_and_drop() {
        let mut pool = SparkPool::<3>::new();
        for n in 1..=3 {
            pool.input.offer_valid = true;
            pool.input.offer_bits = Atom::Ptr(n, false, false);
            pool.tick().unwrap();
        }
        assert!(pool.is_full());
        assert_eq!(pool.search(2), Some(Atom::Ptr(3, false, false)));
        pool.input.offer_bits = Atom::Ptr(99, false, false);
        pool.tick().unwrap();
        assert_eq!(pool.dropped, 1);
        pool.input.offer_valid = false;
        pool.input.pop_ready = true;
        pool.tick().unwrap();
        assert_eq!(pool.front(), Some(Atom::Ptr(2, false, false)));
        pool.input.offer_valid = true;
        pool.input.offer_bits = Atom::Ptr(4, false, false);
        pool.tick().unwrap();
        assert_eq!(pool.search(1), Some(Atom::Ptr(4, false, false)));
        assert_eq!(pool.search(3), None);
        pool.input.gc_lock = true;
        pool.input.offer_valid = false;
        pool.input.pop_ready = true;
        pool.tick().unwrap();
        assert_eq!(pool.len(), 2);
    }

    #[test]
    fn full_pool_allows_pop_and_offer_in_one_cycle() {
        let mut pool = SparkPool::<1>::new();
        pool.input.offer_valid = true;
        pool.input.offer_bits = Atom::Int(1);
        pool.tick().unwrap();
        pool.input.pop_ready = true;
        pool.input.offer_bits = Atom::Int(2);
        pool.tick().unwrap();
        assert_eq!(pool.front(), Some(Atom::Int(2)));
    }
}
