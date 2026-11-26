//! Initial processing-element shell. A protocol dispatcher will eventually
//! connect the NI's core-facing stream to DHeap A/B and other local services.
//! For now, `network` is an explicit, externally driven protocol-side stub;
//! no arbitrary network App is injected into a running thread or heap.

use crate::hardware::noc::router::{RouterPortIn, RouterPortOut};
use crate::hw_module::{HwInput, HwModule};

use super::network_interface::{
    CoreNetworkInput, CoreNetworkOutput, NetworkInterface, NetworkPayload,
};
use super::ouros_core::OurosCore;
use super::program::Program;

#[derive(Default)]
pub struct ProcessingElementInput {
    pub start: bool,
    /// Placeholder for a future protocol dispatcher.
    pub network: CoreNetworkInput,
}

impl HwInput for ProcessingElementInput {}

pub struct ProcessingElement {
    pub input: ProcessingElementInput,
    pub core: OurosCore,
    pub ni: NetworkInterface,
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
        }
    }

    pub fn coordinates(&self) -> (u8, u8) {
        self.ni.coordinates()
    }

    /// Connect the router's current local output (before the next edge).
    pub fn link_router(&mut self, port: RouterPortOut<NetworkPayload>) {
        self.ni.input.router = port;
    }

    /// NI-to-router signals, using the current protocol-side inputs.
    pub fn router_port(&self) -> RouterPortIn<NetworkPayload> {
        let mut port = self.ni.router_port();
        port.in_valid = self.input.network.tx_valid;
        port.in_bits = self.input.network.tx_bits.clone();
        port.in_bits.source = self.coordinates();
        port
    }

    pub fn core_network_port(&self) -> CoreNetworkOutput {
        self.ni.core_port()
    }
}

impl HwModule for ProcessingElement {
    fn update_local(&mut self) -> Result<(), String> {
        self.core.input.start = self.input.start;
        self.ni.input.core = self.input.network.clone();
        Ok(())
    }

    fn tick_children(&mut self) -> Result<(), String> {
        self.core.tick()?;
        self.ni.tick()?;
        Ok(())
    }
}
