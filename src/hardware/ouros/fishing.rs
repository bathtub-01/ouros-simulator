//! Failure-only random fishing. This is the protocol dispatcher side of the
//! NI, not the router or the Ouros graph reducer. There is no successful
//! claim/PUT path yet: a FISH arriving at a PE with usable sparks stays in the
//! NI receive queue until the future success handler is implemented.

use super::config::{FISH_MAX_AGE, FISH_RETRY_DELAY};
use super::network_interface::{NetworkFlit, NetworkPayload};

#[derive(Default, Debug, Clone, Copy)]
pub struct FishStats {
    pub launched: u64,
    pub forwarded: u64,
    pub failed_at_target: u64,
    pub failures_received: u64,
    pub unexpected_failures: u64,
}

pub struct Fishing {
    coord: (u8, u8),
    width: usize,
    height: usize,
    rng: u64,
    /// A single skid register is necessary to hold valid/bits stable during
    /// router backpressure. It is NOT a second TX FIFO.
    pending_tx: Option<NetworkFlit>,
    waiting_for_reply: bool,
    cooldown: u32,
    enabled: bool,
    pub stats: FishStats,
}

impl Fishing {
    pub fn new(coord: (u8, u8)) -> Self {
        assert!(FISH_MAX_AGE > 0, "FISH_MAX_AGE must be positive");
        Self {
            coord, width: 1, height: 1,
            rng: 0x9e3779b97f4a7c15_u64
                ^ ((coord.0 as u64 + 1) << 32)
                ^ (coord.1 as u64 + 1),
            pending_tx: None,
            waiting_for_reply: false,
            cooldown: 0,
            enabled: true,
            stats: FishStats::default(),
        }
    }

    /// Called by the mesh when the PE is attached to a particular network.
    pub fn configure_mesh(&mut self, width: usize, height: usize) {
        assert!(width > 0 && height > 0 && width <= 256 && height <= 256);
        assert!((self.coord.0 as usize) < width && (self.coord.1 as usize) < height);
        self.width = width;
        self.height = height;
    }

    /// Useful to stop background fish generation in a focused transport test.
    /// Incoming fish are still serviced when disabled.
    pub fn set_enabled(&mut self, enabled: bool) { self.enabled = enabled; }

    pub fn pending_tx(&self) -> Option<&NetworkFlit> { self.pending_tx.as_ref() }
    pub fn waiting_for_reply(&self) -> bool { self.waiting_for_reply }
    pub fn cooldown(&self) -> u32 { self.cooldown }

    // Local xorshift64* RNG; deterministic for tests, no host randomness/deps.
    fn next_random(&mut self) -> u64 {
        let mut x = self.rng;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.rng = x;
        x.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }

    /// Pick a random mesh node without allocating a temporary candidate list.
    /// Skip the self index and the optional origin index in the row-major map.
    fn random_target(&mut self, exclude_origin: Option<(u8, u8)>) -> Option<(u8, u8)> {
        let n = self.width * self.height;
        let own = self.coord.1 as usize * self.width + self.coord.0 as usize;
        let origin = exclude_origin.and_then(|(x, y)| {
            if x as usize >= self.width || y as usize >= self.height {
                None
            } else { Some(y as usize * self.width + x as usize) }
        }).unwrap_or(own);
        let mut excluded = [own, origin];
        excluded.sort_unstable();
        let count_excluded = if own == origin { 1 } else { 2 };
        if n <= count_excluded { return None; }
        let mut index = (self.next_random() as usize) % (n - count_excluded);
        for &blocked in &excluded[..count_excluded] {
            if index >= blocked { index += 1; }
        }
        Some(((index % self.width) as u8, (index / self.width) as u8))
    }

    fn make_packet(&self, dest: (u8, u8), load: NetworkPayload) -> NetworkFlit {
        NetworkFlit { source: self.coord, dest, load }
    }

    /// True only when the dispatcher can accept the incoming control flit
    /// at this edge. If local sparks are eligible, hold FISH for the future
    /// success implementation (rather than manufacturing a false failure).
    pub fn accepts(&self, packet: &NetworkFlit, has_spark: bool, tx_fire: bool) -> bool {
        // Simultaneous router acceptance and RX consumption can reuse the
        // pending register on the same edge without dropping a response.
        if self.pending_tx.is_some() && !tx_fire { return false; }
        match &packet.load {
            NetworkPayload::Fish { .. } => !has_spark,
            NetworkPayload::FishFailed => true,
            NetworkPayload::App(_) => false,
        }
    }

    /// `tx_fire` refers only to an internally generated flit; `received`
    /// must have handshaken at the NI's RX port. Both are old-cycle signals.
    pub fn advance(
        &mut self,
        tx_fire: bool,
        received: Option<NetworkFlit>,
        thread_zero_empty: bool,
    ) {
        // Count down *full* cycles, excluding the failure-receive cycle.
        let cooling_this_cycle = self.cooldown > 0;
        if cooling_this_cycle { self.cooldown -= 1; }
        if tx_fire {
            debug_assert!(self.pending_tx.is_some());
            self.pending_tx = None;
        }
        let accepted_rx = received.is_some();
        if let Some(packet) = received {
            match packet.load {
                NetworkPayload::Fish { origin, age } => {
                    let new_age = age.saturating_add(1);
                    if new_age >= FISH_MAX_AGE {
                        self.pending_tx = Some(self.make_packet(origin, NetworkPayload::FishFailed));
                        self.stats.failed_at_target += 1;
                    } else if let Some(target) = self.random_target(Some(origin)) {
                        self.pending_tx = Some(self.make_packet(target, NetworkPayload::Fish {
                            origin,
                            age: new_age,
                        }));
                        self.stats.forwarded += 1;
                    } else {
                        // No other node besides the current hop and origin.
                        self.pending_tx = Some(self.make_packet(origin, NetworkPayload::FishFailed));
                        self.stats.failed_at_target += 1;
                    }
                }
                NetworkPayload::FishFailed => {
                    if self.waiting_for_reply {
                        self.waiting_for_reply = false;
                        self.cooldown = FISH_RETRY_DELAY;
                        self.stats.failures_received += 1;
                    } else {
                        self.stats.unexpected_failures += 1;
                    }
                }
                NetworkPayload::App(_) => unreachable!("App is not handled by fishing"),
            }
        }

        // One outstanding fishing attempt per PE. Never issue a new request
        // on the same edge as processing incoming control traffic or TX.
        if self.enabled && thread_zero_empty && !cooling_this_cycle
            && !accepted_rx && !tx_fire && self.cooldown == 0
            && !self.waiting_for_reply && self.pending_tx.is_none()
        {
            if let Some(target) = self.random_target(None) {
                self.pending_tx = Some(self.make_packet(target, NetworkPayload::Fish {
                    origin: self.coord,
                    age: 0,
                }));
                self.waiting_for_reply = true;
                self.stats.launched += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fish(origin: (u8, u8), dest: (u8, u8), age: u16) -> NetworkFlit {
        NetworkFlit { source: origin, dest, load: NetworkPayload::Fish { origin, age } }
    }

    #[test]
    fn no_self_or_origin_and_increasing_age() {
        let mut p = Fishing::new((1, 0));
        p.configure_mesh(4, 1);
        for age in 0..FISH_MAX_AGE.saturating_sub(1) {
            let incoming = fish((0, 0), (1, 0), age);
            assert!(p.accepts(&incoming, false, false));
            p.advance(false, Some(incoming), true);
            let out = p.pending_tx().unwrap();
            assert_ne!(out.dest, (1, 0));
            assert_ne!(out.dest, (0, 0));
            assert_eq!(out.load, NetworkPayload::Fish { origin: (0, 0), age: age + 1 });
            p.advance(true, None, false);
        }
        p.advance(false, Some(fish((0, 0), (1, 0), FISH_MAX_AGE - 1)), false);
        let out = p.pending_tx().unwrap();
        assert_eq!(out.dest, (0, 0));
        assert_eq!(out.load, NetworkPayload::FishFailed);
    }

    #[test]
    fn two_nodes_fail_early_and_one_node_does_not_fish() {
        let mut single = Fishing::new((0, 0));
        single.configure_mesh(1, 1);
        for _ in 0..20 { single.advance(false, None, true); }
        assert!(single.pending_tx().is_none());
        let mut receiver = Fishing::new((1, 0));
        receiver.configure_mesh(2, 1);
        receiver.advance(false, Some(fish((0, 0), (1, 0), 0)), false);
        assert_eq!(receiver.pending_tx().unwrap().dest, (0, 0));
        assert_eq!(receiver.pending_tx().unwrap().load, NetworkPayload::FishFailed);
    }

    #[test]
    fn valid_spark_holds_fish_without_fake_failure() {
        let receiver = Fishing::new((1, 0));
        assert!(!receiver.accepts(&fish((0, 0), (1, 0), 0), true, false));
        assert!(receiver.accepts(&fish((0, 0), (1, 0), 0), false, false));
    }

    #[test]
    fn blocked_tx_remains_stable_and_failure_triggers_cooldown() {
        let mut p = Fishing::new((0, 0));
        p.configure_mesh(2, 1);
        p.advance(false, None, true);
        let original = p.pending_tx().unwrap().clone();
        for _ in 0..8 { p.advance(false, None, true); }
        assert_eq!(p.pending_tx().unwrap().load, original.load);
        assert_eq!(p.stats.launched, 1);
        p.advance(true, None, true);
        assert!(p.waiting_for_reply());
        assert!(p.pending_tx().is_none());
        p.advance(false, Some(NetworkFlit {
            source: (1, 0), dest: (0, 0), load: NetworkPayload::FishFailed,
        }), true);
        assert_eq!(p.stats.failures_received, 1);
        assert!(!p.waiting_for_reply());
        assert_eq!(p.cooldown(), FISH_RETRY_DELAY);
        for _ in 0..FISH_RETRY_DELAY {
            p.advance(false, None, true);
            assert!(p.pending_tx().is_none());
        }
        p.advance(false, None, true);
        assert_eq!(p.stats.launched, 2);
    }

    #[test]
    fn simultaneous_tx_and_receive_reuses_one_register() {
        let mut p = Fishing::new((1, 0));
        p.configure_mesh(3, 1);
        p.advance(false, Some(fish((0, 0), (1, 0), 0)), false);
        assert!(p.pending_tx().is_some());
        let next = fish((2, 0), (1, 0), 0);
        assert!(!p.accepts(&next, false, false));
        assert!(p.accepts(&next, false, true));
        p.advance(true, Some(next), false);
        assert!(p.pending_tx().is_some());
        assert_eq!(p.stats.forwarded, 2);
    }

    #[test]
    fn busy_thread_zero_prevents_start_and_reply_wait_prevents_duplicates() {
        let mut p = Fishing::new((0, 0));
        p.configure_mesh(3, 1);
        for _ in 0..10 { p.advance(false, None, false); }
        assert_eq!(p.stats.launched, 0);
        p.advance(false, None, true);
        p.advance(true, None, true);
        for _ in 0..30 { p.advance(false, None, true); }
        assert_eq!(p.stats.launched, 1);
    }
}
