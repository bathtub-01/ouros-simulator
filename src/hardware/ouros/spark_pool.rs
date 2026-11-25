//! Bounded FIFO of disposable `par` hints, with one `still_spark` bit per
//! heap cell. Queued sparks are *not* roots for garbage collection.
//!
//! GC reports reclaimed addresses (both sweep and immediate reuse); a
//! reclaimed address invalidates all queued hints referencing that cell.
//! Invalid hints stay in the FIFO until removed. `still_spark` is a validity
//! hint, NOT a claim/migration barrier: stealing will need atomic ownership,
//! in-flight GC rooting, and distributed references.

use crate::hw_module::{HwInput, HwModule};
use crate::hardware::common::DualPortMem;
use super::program::Atom;

#[derive(Default)]
pub struct SparkPoolInput {
    pub offer_valid: bool,
    pub offer_bits: Atom,
    /// Remove the FIFO head; a future stealer will check eligibility first.
    pub pop_ready: bool,
    /// GC reclaimed one cell in this cycle, either via sweep or immediate
    /// one-bit-reference-count deallocation.
    pub unset_valid: bool,
    pub unset_bits: usize,
}

impl HwInput for SparkPoolInput {}

pub struct SparkPool<const N: usize> {
    pub input: SparkPoolInput,
    mem: [Atom; N],
    still_spark: DualPortMem<bool>,
    head: usize,
    tail: usize,
    count: usize,
    pub offered: u64,
    pub dropped: u64,
}

impl<const N: usize> SparkPool<N> {
    pub fn new(heap_size: usize) -> Self {
        assert!(N > 0, "spark pool must have at least one entry");
        Self {
            input: Default::default(),
            mem: [Atom::Nop; N],
            still_spark: DualPortMem::new(heap_size),
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

    /// Read FIFO entries oldest first. This lookup is not used by the GC.
    pub fn search(&self, index: usize) -> Option<Atom> {
        if index >= self.count { None } else { Some(self.mem[(self.head + index) % N]) }
    }

    pub fn front(&self) -> Option<Atom> { self.search(0) }

    pub fn still_spark(&self, addr: usize) -> bool {
        self.still_spark.ram.get(addr).copied().unwrap_or(false)
    }

    /// Eligibility must *also* check that the target is not currently working
    /// and is not WHNF. It does not protect a target from a future GC cycle.
    pub fn front_candidate(&self) -> Option<usize> {
        match self.front() {
            Some(Atom::Ptr(addr, _, _)) if self.still_spark(addr) => Some(addr),
            _ => None,
        }
    }

    /// Full pools drop new hints without backpressuring DHeap.
    pub fn offer_will_store(&self) -> bool {
        matches!(self.input.offer_bits, Atom::Ptr(addr, _, _) if addr < self.still_spark.ram.len())
            && (self.count < N || self.input.pop_ready && self.count > 0)
    }
}

impl<const N: usize> HwModule for SparkPool<N> {
    fn update_local(&mut self) -> Result<(), String> {
        // Port A sets newly accepted hints, port B clears reclaimed cells.
        // Clearing a claimed spark belongs to the future atomic claim path;
        // merely popping an entry does not invalidate another matching hint.
        self.still_spark.input.default_input();
        let pop = self.input.pop_ready && self.count > 0;
        if pop {
            self.mem[self.head] = Atom::Nop;
            self.head = (self.head + 1) % N;
            self.count -= 1;
        }

        if self.input.offer_valid {
            self.offered += 1;
            // Only heap pointers represent migratable spark roots. Other
            // atoms (and out-of-range pointers) can be silently discarded.
            if let Atom::Ptr(addr, _, _) = self.input.offer_bits {
                if addr < self.still_spark.ram.len() && self.count < N {
                    self.mem[self.tail] = self.input.offer_bits;
                    self.tail = (self.tail + 1) % N;
                    self.count += 1;
                    // Same-cycle GC reclamation wins. Never issue two writes
                    // to the same flag address on the dual-port memory.
                    if !self.input.unset_valid || addr != self.input.unset_bits {
                        self.still_spark.write_a(addr, true);
                    }
                } else {
                    self.dropped += 1;
                }
            } else {
                self.dropped += 1;
            }
        }

        // Reclamation takes priority over a concurrent spark offer: an address
        // freed in this cycle must not remain marked as a valid spark.
        if self.input.unset_valid && self.input.unset_bits < self.still_spark.ram.len() {
            self.still_spark.write_b(self.input.unset_bits, false);
        }
        Ok(())
    }

    fn tick_children(&mut self) -> Result<(), String> { self.still_spark.tick() }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ptr(addr: usize) -> Atom { Atom::Ptr(addr, false, false) }

    #[test]
    fn fifo_wrap_scan_and_drop() {
        let mut pool = SparkPool::<3>::new(100);
        for n in 1..=3 {
            pool.input.offer_valid = true;
            pool.input.offer_bits = ptr(n);
            pool.tick().unwrap();
        }
        assert!(pool.is_full());
        assert_eq!(pool.search(2), Some(ptr(3)));
        assert!(pool.still_spark(1));
        pool.input.offer_bits = ptr(99);
        pool.tick().unwrap();
        assert_eq!(pool.dropped, 1);
        assert!(!pool.still_spark(99));

        pool.input.offer_valid = false;
        pool.input.pop_ready = true;
        pool.tick().unwrap();
        // Popping alone does not clear the bit; that is reserved for GC
        // reclamation or the future atomic migration/claim path.
        assert!(pool.still_spark(1));
        assert_eq!(pool.front(), Some(ptr(2)));

        pool.input.offer_valid = true;
        pool.input.offer_bits = ptr(4);
        pool.tick().unwrap();
        assert_eq!(pool.search(1), Some(ptr(4)));
        assert!(pool.still_spark(4));
        assert_eq!(pool.search(3), None);
    }

    #[test]
    fn full_pool_allows_pop_and_offer_in_one_cycle() {
        let mut pool = SparkPool::<1>::new(16);
        pool.input.offer_valid = true;
        pool.input.offer_bits = ptr(1);
        pool.tick().unwrap();
        pool.input.pop_ready = true;
        pool.input.offer_bits = ptr(2);
        pool.tick().unwrap();
        assert_eq!(pool.front(), Some(ptr(2)));
        assert!(pool.still_spark(1));
        assert!(pool.still_spark(2));
    }

    #[test]
    fn reclaimed_hint_stays_in_fifo_but_is_not_valid() {
        let mut pool = SparkPool::<2>::new(32);
        pool.input.offer_valid = true;
        pool.input.offer_bits = ptr(11);
        pool.tick().unwrap();
        assert_eq!(pool.front_candidate(), Some(11));
        pool.input.offer_valid = false;
        pool.input.unset_valid = true;
        pool.input.unset_bits = 11;
        pool.tick().unwrap();
        assert_eq!(pool.front(), Some(ptr(11)));
        assert_eq!(pool.front_candidate(), None);
    }

    #[test]
    fn gc_clear_wins_over_same_cycle_offer() {
        let mut pool = SparkPool::<2>::new(32);
        pool.input.offer_valid = true;
        pool.input.offer_bits = ptr(11);
        pool.input.unset_valid = true;
        pool.input.unset_bits = 11;
        pool.tick().unwrap();
        assert_eq!(pool.front(), Some(ptr(11)));
        assert_eq!(pool.front_candidate(), None);
    }

    #[test]
    fn non_pointer_offer_is_discarded() {
        let mut pool = SparkPool::<2>::new(32);
        pool.input.offer_valid = true;
        pool.input.offer_bits = Atom::Int(42);
        pool.tick().unwrap();
        assert!(pool.is_empty());
        assert_eq!(pool.dropped, 1);
    }
}
