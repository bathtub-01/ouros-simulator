//! Buffered receive and direct transmit connection between an Ouros PE and
//! the local port of the existing 2-D mesh router.
//!
//! The router owns the TX-side queue (its local input FIFO). The NI only
//! buffers RX traffic. The PE-side dispatcher currently handles only the
//! failure-only FISH protocol and will later inject ActiveApps/FrozenApps
//! into DHeap arbiters. This transport module deliberately does not interpret
//! messages, allocate proxies, or access heap memory.

use crate::hardware::common::fifo::Fifo;
use crate::hardware::noc::router::{Packet, RouterPortIn, RouterPortOut};
use crate::hw_module::{HwInput, HwModule};

use super::config::NI_RX_DEPTH;
use super::program::App;

/// The router owns the source/destination header; a payload is a sum type.
/// `origin` is the original fisherman: Packet.source changes on every hop.
#[derive(Clone, Debug, PartialEq)]
pub enum NetworkPayload {
    App(App),
    Fish { origin: (u8, u8), age: u16 },
    FishFailed,
}

impl Default for NetworkPayload {
    fn default() -> Self { Self::App(Default::default()) }
}

pub type NetworkFlit = Packet<NetworkPayload>;

/// Signals supplied by the future protocol dispatcher on the PE side.
/// The dispatcher must hold `tx_valid` and `tx_bits` while `tx_ready` is low.
#[derive(Clone, Default)]
pub struct CoreNetworkInput {
    pub tx_valid: bool,
    pub tx_bits: NetworkFlit,
    pub rx_ready: bool,
}

/// Ready-valid stream observed by the future dispatcher on the PE side.
#[derive(Clone, Default)]
pub struct CoreNetworkOutput {
    pub tx_ready: bool,
    pub rx_valid: bool,
    pub rx_bits: NetworkFlit,
}

#[derive(Default)]
pub struct NetworkInterfaceInput {
    /// Router's local output signals, including its local input readiness.
    pub router: RouterPortOut<NetworkPayload>,
    /// Signals from the (not-yet-implemented) protocol dispatcher.
    pub core: CoreNetworkInput,
}

impl HwInput for NetworkInterfaceInput {}

pub struct NetworkInterface {
    pub input: NetworkInterfaceInput,
    coord: (u8, u8),
    rx_fifo: Fifo<NetworkFlit, NI_RX_DEPTH, false>,
}

impl NetworkInterface {
    pub fn new(coord: (u8, u8)) -> Self {
        assert!(NI_RX_DEPTH > 0, "NI_RX_DEPTH must be positive");
        Self {
            input: Default::default(),
            coord,
            rx_fifo: Fifo::new(),
        }
    }

    pub fn coordinates(&self) -> (u8, u8) {
        self.coord
    }

    /// Drive the router local input. There is deliberately no TX FIFO: the
    /// router's local-input buffer provides storage and backpressure.
    /// Source coordinates are always set here (not trusted from the sender).
    pub fn router_port(&self) -> RouterPortIn<NetworkPayload> {
        let mut bits = self.input.core.tx_bits.clone();
        bits.source = self.coord;
        RouterPortIn {
            in_valid: self.input.core.tx_valid,
            in_bits: bits,
            out_ready: self.rx_fifo.in_ready(),
        }
    }

    /// Combinational core-facing outputs. The RX FIFO is non-fall-through:
    /// a newly received flit appears on this port after the receiving edge.
    pub fn core_port(&self) -> CoreNetworkOutput {
        CoreNetworkOutput {
            tx_ready: self.input.router.in_ready,
            rx_valid: self.rx_fifo.out_valid(),
            rx_bits: self.rx_fifo.dout().cloned().unwrap_or_default(),
        }
    }

    pub fn rx_len(&self) -> usize {
        self.rx_fifo.queue.len()
    }
}

impl HwModule for NetworkInterface {
    fn update_local(&mut self) -> Result<(), String> {
        self.rx_fifo.input.in_valid = self.input.router.out_valid;
        self.rx_fifo.input.din = self.input.router.out_bits.clone();
        self.rx_fifo.input.out_ready = self.input.core.rx_ready;
        Ok(())
    }

    fn tick_children(&mut self) -> Result<(), String> {
        self.rx_fifo.tick()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hardware::ouros::program::Atom;
    use crate::hardware::ouros::config::APP_LENGTH;

    fn app_flit(dest: (u8, u8), v: i32) -> NetworkFlit {
        NetworkFlit {
            source: (99, 99), dest,
            load: NetworkPayload::App([Atom::Int(v); APP_LENGTH]),
        }
    }

    #[test]
    fn tx_is_direct_and_uses_router_ready() {
        let mut ni = NetworkInterface::new((1, 2));
        ni.input.core.tx_valid = true;
        ni.input.core.tx_bits = NetworkFlit {
            source: (99, 99), dest: (3, 4),
            load: NetworkPayload::Fish { origin: (0, 0), age: 2 },
        };
        ni.input.router.in_ready = false;
        assert!(!ni.core_port().tx_ready);
        let port = ni.router_port();
        assert!(port.in_valid);
        assert_eq!(port.in_bits.source, (1, 2));
        assert_eq!(port.in_bits.dest, (3, 4));
        assert_eq!(port.in_bits.load, NetworkPayload::Fish { origin: (0, 0), age: 2 });
        ni.input.router.in_ready = true;
        assert!(ni.core_port().tx_ready);
        assert_eq!(ni.rx_len(), 0);
    }

    #[test]
    fn rx_fifo_preserves_order_and_backpressures() {
        let mut ni = NetworkInterface::new((0, 0));
        ni.input.core.rx_ready = false;
        for i in 0..NI_RX_DEPTH {
            assert!(ni.router_port().out_ready);
            ni.input.router.out_valid = true;
            ni.input.router.out_bits = app_flit((0, 0), i as i32);
            ni.tick().unwrap();
        }
        assert!(!ni.router_port().out_ready);
        ni.input.router.out_bits = app_flit((0, 0), 999);
        ni.tick().unwrap();
        assert_eq!(ni.rx_len(), NI_RX_DEPTH);

        ni.input.router.out_valid = false;
        ni.input.core.rx_ready = true;
        for i in 0..NI_RX_DEPTH {
            let output = ni.core_port();
            assert!(output.rx_valid);
            assert_eq!(output.rx_bits.load, NetworkPayload::App([Atom::Int(i as i32); APP_LENGTH]));
            ni.tick().unwrap();
        }
        assert!(!ni.core_port().rx_valid);
        assert!(ni.router_port().out_ready);
    }

    #[test]
    fn tx_and_rx_can_operate_on_same_edge() {
        let mut ni = NetworkInterface::new((1, 1));
        ni.input.router.in_ready = true;
        ni.input.router.out_valid = true;
        ni.input.router.out_bits = NetworkFlit {
            source: (0, 0), dest: (1, 1), load: NetworkPayload::FishFailed,
        };
        ni.input.core.tx_valid = true;
        ni.input.core.tx_bits = app_flit((0, 0), 42);
        let tx = ni.router_port();
        assert!(tx.in_valid && tx.out_ready);
        assert_eq!(tx.in_bits.load, NetworkPayload::App([Atom::Int(42); APP_LENGTH]));
        ni.tick().unwrap();
        let rx = ni.core_port();
        assert!(rx.rx_valid);
        assert_eq!(rx.rx_bits.load, NetworkPayload::FishFailed);
    }
}
