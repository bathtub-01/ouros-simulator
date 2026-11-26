//! One PE: graph-reduction core, NI, and a minimal control-message dispatcher.
//! Fishing handles only failure paths. App traffic remains externally exposed
//! for the future FETCH/PUT dispatcher, never injected into DHeap implicitly.

use crate::hardware::noc::router::{RouterPortIn, RouterPortOut};
use crate::hw_module::{HwInput, HwModule};

use super::fishing::Fishing;
use super::network_interface::{
    CoreNetworkInput, CoreNetworkOutput, NetworkFlit, NetworkInterface, NetworkPayload,
};
use super::ouros_core::OurosCore;
use super::program::Program;

#[derive(Default)]
pub struct ProcessingElementInput {
    pub start: bool,
    /// Non-fishing protocol stub, retained for future FETCH/PUT and App tests.
    pub network: CoreNetworkInput,
}

impl HwInput for ProcessingElementInput {}

pub struct ProcessingElement {
    pub input: ProcessingElementInput,
    pub core: OurosCore,
    pub ni: NetworkInterface,
    pub fishing: Fishing,
}

impl ProcessingElement {
    pub fn new(
        coord: (u8, u8),
        program: &Program,
        detail_lv: u8,
        heap_size: usize,
        gc_at: f32,
    ) -> Self {
        Self {
            input: Default::default(),
            core: OurosCore::new(program, detail_lv, heap_size, gc_at),
            ni: NetworkInterface::new(coord),
            fishing: Fishing::new(coord),
        }
    }

    pub fn coordinates(&self) -> (u8, u8) { self.ni.coordinates() }

    pub fn configure_mesh(&mut self, width: usize, height: usize) {
        self.fishing.configure_mesh(width, height);
    }

    pub fn link_router(&mut self, port: RouterPortOut<NetworkPayload>) {
        self.ni.input.router = port;
    }

    /// Internal fish/control traffic takes TX priority. It is held stable by
    /// the fishing skid register while the router's input FIFO is full.
    fn selected_tx(&self) -> (bool, NetworkFlit) {
        match self.fishing.pending_tx() {
            Some(packet) => (true, packet.clone()),
            None => (self.input.network.tx_valid, self.input.network.tx_bits.clone()),
        }
    }

    /// Current combinational PE -> router signals, before the clock edge.
    pub fn router_port(&self) -> RouterPortIn<NetworkPayload> {
        let mut port = self.ni.router_port();
        let (valid, mut bits) = self.selected_tx();
        bits.source = self.coordinates();
        port.in_valid = valid;
        port.in_bits = bits;
        port
    }

    /// Non-fishing protocol-facing stream. Fish/Failed are consumed by the
    /// dispatcher, not exposed as executable Apps on this stream.
    pub fn core_network_port(&self) -> CoreNetworkOutput {
        let mut port = self.ni.core_port();
        if self.fishing.pending_tx().is_some() { port.tx_ready = false; }
        if port.rx_valid && !matches!(&port.rx_bits.load, NetworkPayload::App(_)) {
            port.rx_valid = false;
        }
        port
    }
}

impl HwModule for ProcessingElement {
    fn update_local(&mut self) -> Result<(), String> {
        self.core.input.start = self.input.start;
        let ni_output = self.ni.core_port(); // old-cycle RX FIFO and ready
        let incoming = if ni_output.rx_valid { Some(ni_output.rx_bits.clone()) } else { None };
        let control = incoming.as_ref().is_some_and(|pkt| {
            matches!(&pkt.load, NetworkPayload::Fish { .. } | NetworkPayload::FishFailed)
        });
        let fish_tx_fire = self.fishing.pending_tx().is_some() && ni_output.tx_ready;
        let rx_ready = if control {
            let has_spark = matches!(&incoming.as_ref().unwrap().load, NetworkPayload::Fish { .. })
                && self.core.has_stealable_spark_candidate();
            self.fishing.accepts(incoming.as_ref().unwrap(), has_spark, fish_tx_fire)
        } else {
            self.input.network.rx_ready
        };

        let (tx_valid, tx_bits) = self.selected_tx();
        self.ni.input.core = CoreNetworkInput { tx_valid, tx_bits, rx_ready };
        // The NI RX FIFO and the fishing FSM must observe exactly the same
        // handshake. A backed-up reply consumes neither an incoming FISH nor
        // a slot in the router's local TX FIFO until ready.
        let fish_rx = if control && rx_ready { incoming } else { None };
        let thread_zero_empty = self.core.thread_zero_empty();
        self.fishing.advance(fish_tx_fire, fish_rx, thread_zero_empty);
        Ok(())
    }

    fn tick_children(&mut self) -> Result<(), String> {
        self.core.tick()?;
        self.ni.tick()?;
        Ok(())
    }
}
