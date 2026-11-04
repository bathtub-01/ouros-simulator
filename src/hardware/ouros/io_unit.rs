//! Cycle-stepped AXI4-Lite master, controller, and abstract terminal UART.
//!
//! IO is a graph reduction, not a separate instruction stream. This component
//! accepts one already-strict ActiveApp from DHeap, performs an AXI transaction,
//! and holds the resulting Pair until the DHeap return buffer accepts it.

use std::collections::{HashMap, VecDeque};

use crate::hardware::ouros::program::{io_is_ready, ActiveApp, App, Atom};
use crate::hw_module::{HwInput, HwModule};

/// Word-aligned 32-bit AXI4-Lite UART registers (byte addresses).
/// RX_DATA blocks when empty; STATUS can be polled without blocking.
pub const UART_RX_DATA: u32 = 0x00;
pub const UART_TX_DATA: u32 = 0x04;
pub const UART_STATUS: u32 = 0x08;
pub const UART_CONTROL: u32 = 0x0c;
pub const UART_RX_READY: u32 = 1;
pub const UART_TX_READY: u32 = 2;

#[derive(Clone, Copy, Debug)]
pub struct AxiTiming {
    /// Number of complete cycles between a request handshake and response.
    pub response_latency: u32,
    /// Channel readiness may be independently delayed for testing.
    pub aw_delay: u32,
    pub w_delay: u32,
    pub ar_delay: u32,
}

impl Default for AxiTiming {
    fn default() -> Self {
        Self { response_latency: 2, aw_delay: 0, w_delay: 0, ar_delay: 0 }
    }
}

/// Abstract IO controller: independent AW, W and AR acceptances, and B/R
/// responses. A register bank backs unmapped word addresses. No AXI errors.
pub struct AxiLiteController {
    pub timing: AxiTiming,
    cycle: u64,
    aw: Option<u32>,
    w: Option<u32>,
    b_countdown: Option<u32>,
    r_countdown: Option<(u32, u32)>,
    r_data: Option<u32>,
    regs: HashMap<u32, u32>,
    rx: VecDeque<u8>,
    tx: VecDeque<u8>,
    pub reads: u64,
    pub writes: u64,
}

impl AxiLiteController {
    pub fn new(timing: AxiTiming) -> Self {
        Self {
            timing, cycle: 0, aw: None, w: None, b_countdown: None,
            r_countdown: None, r_data: None, regs: HashMap::new(),
            rx: VecDeque::new(), tx: VecDeque::new(), reads: 0, writes: 0,
        }
    }

    pub fn push_rx(&mut self, byte: u8) { self.rx.push_back(byte); }
    pub fn pop_tx(&mut self) -> Option<u8> { self.tx.pop_front() }
    pub fn register(&self, address: u32) -> u32 {
        self.regs.get(&(address & !3)).copied().unwrap_or(0)
    }
    pub fn set_register(&mut self, address: u32, value: u32) {
        self.regs.insert(address & !3, value);
    }
    pub fn rx_len(&self) -> usize { self.rx.len() }

    pub fn aw_ready(&self) -> bool {
        self.aw.is_none() && self.b_countdown.is_none()
            && self.cycle >= self.timing.aw_delay as u64
    }
    pub fn w_ready(&self) -> bool {
        self.w.is_none() && self.b_countdown.is_none()
            && self.cycle >= self.timing.w_delay as u64
    }
    pub fn ar_ready(&self) -> bool {
        self.r_countdown.is_none() && self.r_data.is_none()
            && self.cycle >= self.timing.ar_delay as u64
    }
    pub fn b_valid(&self) -> bool { self.b_countdown == Some(0) }
    pub fn r_valid(&self) -> bool { self.r_data.is_some() }
    pub fn r_bits(&self) -> Option<u32> { self.r_data }

    fn can_read(&self, addr: u32) -> bool {
        (addr & !3) != UART_RX_DATA || !self.rx.is_empty()
    }
    fn read(&mut self, addr: u32) -> u32 {
        match addr & !3 {
            UART_RX_DATA => self.rx.pop_front().unwrap() as u32,
            UART_STATUS => (if self.rx.is_empty() { 0 } else { UART_RX_READY }) | UART_TX_READY,
            _ => self.register(addr),
        }
    }
    fn write(&mut self, addr: u32, value: u32) {
        match addr & !3 {
            UART_TX_DATA => self.tx.push_back(value as u8),
            UART_RX_DATA | UART_STATUS => { /* read-only */ }
            _ => self.set_register(addr, value),
        }
    }

    /// One clock edge of the controller. Ready/valid refer to the *old* state.
    pub fn tick_bus(&mut self, aw: Option<u32>, w: Option<u32>, ar: Option<u32>,
                    b_ready: bool, r_ready: bool) {
        let accept_aw = aw.is_some() && self.aw_ready();
        let accept_w = w.is_some() && self.w_ready();
        let accept_ar = ar.is_some() && self.ar_ready();
        let consume_b = b_ready && self.b_valid();
        let consume_r = r_ready && self.r_valid();

        if consume_b { self.b_countdown = None; }
        if consume_r { self.r_data = None; }
        if accept_aw { self.aw = aw.map(|a| a & !3); }
        if accept_w { self.w = w; }
        if accept_ar {
            self.r_countdown = ar.map(|a| (a & !3, self.timing.response_latency.saturating_add(1)));
        }

        // AW and W can arrive in either order. Commit only after both arrive.
        if self.aw.is_some() && self.w.is_some() && self.b_countdown.is_none() {
            let addr = self.aw.take().unwrap();
            let data = self.w.take().unwrap();
            self.write(addr, data);
            self.writes += 1;
            self.b_countdown = Some(self.timing.response_latency.saturating_add(1));
        }
        if let Some(n) = self.b_countdown.as_mut() {
            if *n > 0 { *n -= 1; }
        }
        if let Some((addr, n)) = self.r_countdown {
            if n > 0 {
                self.r_countdown = Some((addr, n - 1));
            } else if self.can_read(addr) {
                self.r_countdown = None;
                self.r_data = Some(self.read(addr));
                self.reads += 1;
            }
        }
        self.cycle += 1;
    }
}

#[derive(Default)]
pub struct IoUnitInput {
    pub request_valid: bool,
    pub request_bits: ActiveApp,
    pub response_ready: bool,
}
impl HwInput for IoUnitInput {}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase { Send, Wait, Complete }

struct Transaction {
    request: ActiveApp,
    address: u32,
    data: Option<u32>,
    world: Atom,
    aw_sent: bool,
    w_sent: bool,
    ar_sent: bool,
    phase: Phase,
    result: Option<ActiveApp>,
}

pub struct IoUnit {
    pub input: IoUnitInput,
    pub controller: AxiLiteController,
    transaction: Option<Transaction>,
    pub completed: u64,
}

impl IoUnit {
    pub fn new(timing: AxiTiming) -> Self {
        Self { input: Default::default(), controller: AxiLiteController::new(timing),
               transaction: None, completed: 0 }
    }
    pub fn request_ready(&self) -> bool { self.transaction.is_none() }
    pub fn response_valid(&self) -> bool {
        self.transaction.as_ref().is_some_and(|t| t.phase == Phase::Complete)
    }
    pub fn response_bits(&self) -> Option<&ActiveApp> {
        self.transaction.as_ref().and_then(|t| t.result.as_ref())
    }
    pub fn pending_active_app(&self) -> Option<&ActiveApp> {
        self.transaction.as_ref().map(|t| &t.request)
    }
    pub fn pending_app(&self) -> Option<&App> {
        self.transaction.as_ref().map(|t| &t.request.load)
    }

    fn begin(app: ActiveApp) -> Result<Transaction, String> {
        if !io_is_ready(&app.load) {
            return Err(format!("IO unit received non-ready App: {:?}", app.load));
        }
        let is_write = matches!(app.load[0], Atom::IO(true));
        let address = match app.load[1] { Atom::Int(n) => n as u32, _ => unreachable!() };
        let (data, world) = if is_write {
            (Some(match app.load[2] { Atom::Int(n) => n as u32, _ => unreachable!() }), app.load[3])
        } else { (None, app.load[2]) };
        Ok(Transaction { request: app, address, data, world, aw_sent: false,
                         w_sent: false, ar_sent: false, phase: Phase::Send, result: None })
    }

    fn finish(t: &mut Transaction, result: u32) {
        let is_write = t.data.is_some();
        // The Pair combinator has arity 3, and is placed at index 2 by the compiler.
        // When returning `((), World)`, `()` is just the I combinator; we abuse the record of K for it.
        let mut load: App = [Atom::Nop; 8];
        load[0] = Atom::Com(3, 2);
        load[1] = if is_write { Atom::Com(1,0) } else { Atom::Int(result as i32) };
        load[2] = t.world;
        // An over-applied IO primitive passes on its remaining arguments.
        let first_tail = if is_write { 4 } else { 3 };
        let mut next = 3;
        for &atom in t.request.load[first_tail..].iter() {
            if atom == Atom::Nop { break; }
            // The original App contains at most 8 Atoms; replacing a four-
            // or three-argument redex by a three-Atom pair always fits.
            load[next] = atom;
            next += 1;
        }
        t.result = Some(ActiveApp { stack_idx: t.request.stack_idx, load });
        t.phase = Phase::Complete;
    }
}

impl HwModule for IoUnit {
    fn update_local(&mut self) -> Result<(), String> {
        // Old-state handshakes, sampled before any state mutations.
        let request_fire = self.input.request_valid && self.request_ready();
        let response_fire = self.response_valid() && self.input.response_ready;
        if response_fire {
            self.transaction = None;
            self.completed += 1;
        }
        let mut aw = None;
        let mut w = None;
        let mut ar = None;
        let mut b_ready = false;
        let mut r_ready = false;
        if let Some(t) = self.transaction.as_mut() {
            match t.phase {
                Phase::Send => {
                    if t.data.is_some() {
                        if !t.aw_sent {
                            aw = Some(t.address);
                            if self.controller.aw_ready() { t.aw_sent = true; }
                        }
                        if !t.w_sent {
                            w = t.data;
                            if self.controller.w_ready() { t.w_sent = true; }
                        }
                        if t.aw_sent && t.w_sent { t.phase = Phase::Wait; }
                    } else {
                        if !t.ar_sent {
                            ar = Some(t.address);
                            if self.controller.ar_ready() { t.ar_sent = true; t.phase = Phase::Wait; }
                        }
                    }
                }
                Phase::Wait => {
                    if t.data.is_some() {
                        if self.controller.b_valid() {
                            b_ready = true;
                            Self::finish(t, 0);
                        }
                    } else if let Some(result) = self.controller.r_bits() {
                        r_ready = true;
                        Self::finish(t, result);
                    }
                }
                Phase::Complete => {}
            }
        }
        self.controller.tick_bus(aw, w, ar, b_ready, r_ready);
        if request_fire { self.transaction = Some(Self::begin(self.input.request_bits.clone())?); }
        Ok(())
    }
    fn tick_children(&mut self) -> Result<(), String> { Ok(()) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hardware::ouros::program::Atom::*;

    fn app(atoms: &[Atom]) -> ActiveApp {
        let mut load = [Nop; 8];
        load[..atoms.len()].copy_from_slice(atoms);
        ActiveApp { stack_idx: 2, load }
    }
    fn run(unit: &mut IoUnit, request: ActiveApp) -> ActiveApp {
        unit.input.request_bits = request;
        unit.input.request_valid = true;
        unit.tick().unwrap();
        unit.input.request_valid = false;
        for _ in 0..200 {
            if let Some(result) = unit.response_bits() { return result.clone(); }
            unit.tick().unwrap();
        }
        panic!("IO timed out");
    }
    #[test]
    fn read_write_await_axi_response_and_preserve_world() {
        let mut u = IoUnit::new(AxiTiming { response_latency: 4, aw_delay: 2, w_delay: 6, ar_delay: 0 });
        let write = run(&mut u, app(&[IO(true), Int(0x20), Int(-1), Int(99)]));
        assert_eq!(u.controller.register(0x20), u32::MAX);
        assert_eq!(write.stack_idx, 2);
        assert_eq!(write.load[..3], [Com(3,2), Com(1,0), Int(99)]);
        assert_eq!(u.completed, 0); // held until response-ready
        for _ in 0..8 { u.tick().unwrap(); assert!(u.response_valid()); }
        u.input.response_ready = true;
        u.tick().unwrap();
        u.input.response_ready = false;
        let read = run(&mut u, app(&[IO(false), Int(0x20), Int(101)]));
        assert_eq!(read.load[..3], [Com(3,2), Int(-1), Int(101)]);
        assert_eq!(u.controller.writes, 1);
        assert_eq!(u.controller.reads, 1);
    }
    #[test]
    fn over_applied_io_preserves_trailing_case_argument() {
        let mut u = IoUnit::new(AxiTiming::default());
        u.controller.set_register(0x20, 9);
        let response = run(&mut u, app(&[IO(false), Int(0x20), Int(15), Tab(42, 0)]));
        assert_eq!(response.load[..4], [Com(3, 2), Int(9), Int(15), Tab(42, 0)]);
        assert_eq!(response.stack_idx, 2);
    }

    #[test]
    fn aw_and_w_accept_in_either_order() {
        let mut ctrl = AxiLiteController::new(AxiTiming {
            aw_delay: 6, w_delay: 0, ar_delay: 0, response_latency: 3,
        });
        ctrl.tick_bus(None, Some(0xfeed), None, false, false);
        for _ in 0..5 { ctrl.tick_bus(None, None, None, false, false); }
        assert_eq!(ctrl.writes, 0);
        ctrl.tick_bus(Some(0x30), None, None, false, false);
        assert_eq!(ctrl.register(0x30), 0xfeed);
        assert_eq!(ctrl.writes, 1);
        for _ in 0..4 { ctrl.tick_bus(None, None, None, false, false); }
        assert!(ctrl.b_valid());
        ctrl.tick_bus(None, None, None, true, false);
        assert!(!ctrl.b_valid());
    }

    #[test]
    fn uart_rx_waits_for_input_and_tx_emits_bytes() {
        let mut u = IoUnit::new(AxiTiming::default());
        u.input.request_bits = app(&[IO(false), Int(0), Int(0)]);
        u.input.request_valid = true;
        u.tick().unwrap();
        u.input.request_valid = false;
        for _ in 0..10 { u.tick().unwrap(); }
        assert!(!u.response_valid());
        u.controller.push_rx(b'Z');
        for _ in 0..10 { if u.response_valid() { break; } u.tick().unwrap(); }
        assert_eq!(u.response_bits().unwrap().load[1], Int(b'Z' as i32));
        u.input.response_ready = true;
        u.tick().unwrap();
        u.input.response_ready = false;
        let _ = run(&mut u, app(&[IO(true), Int(4), Int(b'Q' as i32), Int(0)]));
        assert_eq!(u.controller.pop_tx(), Some(b'Q'));
    }
}
