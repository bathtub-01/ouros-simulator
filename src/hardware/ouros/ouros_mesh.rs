//! Mesh composition: each router local port connects to a PE's NI. The PE
//! dispatcher implements failure-only random fishing; App traffic remains
//! externally exposed. No successful stealing, FETCH/PUT, proxy allocation,
//! or DHeap network injection is implemented.

use crate::hardware::noc::network::Network;
use crate::hw_module::HwModule;

use super::network_interface::NetworkPayload;
use super::processing_element::ProcessingElement;

pub struct OurosMesh {
    pub network: Network<NetworkPayload>,
    /// Row-major order: `pes[y * width + x]` is located at `(x, y)`.
    pub pes: Vec<ProcessingElement>,
    width: usize,
    height: usize,
}

impl OurosMesh {
    /// `pes` must contain exactly one PE per router, in row-major order.
    pub fn new(width: usize, height: usize, mut pes: Vec<ProcessingElement>) -> Self {
        let network = Network::new(width, height);
        assert_eq!(pes.len(), width * height, "one PE per router is required");
        for (index, pe) in pes.iter().enumerate() {
            let coord = ((index % width) as u8, (index / width) as u8);
            assert_eq!(pe.coordinates(), coord, "PE coordinate mismatch");
        }
        for pe in &mut pes { pe.configure_mesh(width, height); }
        let mut mesh = Self {
            network,
            pes,
            width,
            height,
        };
        mesh.refresh_ni_router_outputs();
        mesh
    }

    fn index(&self, x: usize, y: usize) -> usize {
        assert!(x < self.width && y < self.height, "PE coordinate out of range");
        y * self.width + x
    }

    pub fn pe(&self, x: usize, y: usize) -> &ProcessingElement {
        &self.pes[self.index(x, y)]
    }

    pub fn pe_mut(&mut self, x: usize, y: usize) -> &mut ProcessingElement {
        let idx = self.index(x, y);
        &mut self.pes[idx]
    }

    fn refresh_ni_router_outputs(&mut self) {
        // Router outputs reflect *current* FIFO state. Make them visible to
        // the NI's core-side ready signal without waiting for another tick.
        for y in 0..self.height {
            for x in 0..self.width {
                let router = self.network.local_out(x, y);
                self.pes[y * self.width + x].link_router(router);
            }
        }
    }
}

impl HwModule for OurosMesh {
    fn update_local(&mut self) -> Result<(), String> {
        self.refresh_ni_router_outputs();
        // Capture the PE-to-router inputs for this same cycle. The NI's TX
        // stream is combinational; there is no extra TX FIFO or clock edge.
        for y in 0..self.height {
            for x in 0..self.width {
                self.network.input.local[y][x] =
                    self.pes[y * self.width + x].router_port();
            }
        }
        Ok(())
    }

    fn tick_children(&mut self) -> Result<(), String> {
        // Both subsystems see pre-edge snapshots: no PE observes a router
        // that has already advanced, and no router sees next-cycle NI data.
        for (i, pe) in self.pes.iter_mut().enumerate() {
            pe.tick().map_err(|err| format!("PE {i}: {err}"))?;
        }
        self.network.tick()?;
        self.refresh_ni_router_outputs();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hardware::ouros::network_interface::{NetworkFlit, NetworkPayload};
    use crate::hardware::ouros::program::{Atom, Program};
    use crate::hardware::ouros::config::APP_LENGTH;

    fn test_pe(x: u8, y: u8) -> ProcessingElement {
        let program = Program {
            heap_img: vec![vec![Atom::Int(0)]],
            comb_img: vec![],
        };
        ProcessingElement::new((x, y), &program, 0, 64, 0.0)
    }

    #[test]
    fn sends_one_app_across_mesh_without_protocol_dispatch() {
        let mut mesh = OurosMesh::new(
            2, 2,
            vec![test_pe(0, 0), test_pe(1, 0), test_pe(0, 1), test_pe(1, 1)],
        );
        let message = NetworkFlit {
            source: (99, 99),
            dest: (1, 1),
            load: NetworkPayload::App([Atom::Int(123); APP_LENGTH]),
        };
        // Disable autonomous fishing to isolate the App transport test.
        for pe in &mut mesh.pes { pe.fishing.set_enabled(false); }
        assert!(mesh.pe(0, 0).core_network_port().tx_ready);
        mesh.pe_mut(0, 0).input.network.tx_valid = true;
        mesh.pe_mut(0, 0).input.network.tx_bits = message;
        mesh.tick().unwrap();
        mesh.pe_mut(0, 0).input.network.tx_valid = false;

        for _ in 0..64 {
            let received = mesh.pe(1, 1).core_network_port();
            if received.rx_valid {
                assert_eq!(received.rx_bits.source, (0, 0));
                assert_eq!(received.rx_bits.dest, (1, 1));
                assert_eq!(received.rx_bits.load, NetworkPayload::App([Atom::Int(123); APP_LENGTH]));
                mesh.pe_mut(1, 1).input.network.rx_ready = true;
                mesh.tick().unwrap();
                assert!(!mesh.pe(1, 1).core_network_port().rx_valid);
                return;
            }
            mesh.tick().unwrap();
        }
        panic!("App flit did not reach remote NI");
    }
}

#[cfg(test)]
mod fishing_mesh_tests {
    use super::*;
    use crate::hardware::ouros::config::FISH_RETRY_DELAY;
    use crate::hardware::ouros::program::{Atom, Program};

    fn pe(x: u8) -> ProcessingElement {
        let program = Program {
            heap_img: vec![vec![Atom::Int(0)]],
            comb_img: vec![],
        };
        ProcessingElement::new((x, 0), &program, 0, 64, 0.0)
    }

    #[test]
    fn random_fish_fails_across_mesh_then_cools_down() {
        let mut mesh = OurosMesh::new(3, 1, vec![pe(0), pe(1), pe(2)]);
        // Only node 0 generates fish, but every PE responds and can forward.
        mesh.pe_mut(1, 0).fishing.set_enabled(false);
        mesh.pe_mut(2, 0).fishing.set_enabled(false);

        for _ in 0..512 {
            mesh.tick().unwrap();
            if mesh.pe(0, 0).fishing.stats.failures_received > 0 { break; }
        }
        assert_eq!(mesh.pe(0, 0).fishing.stats.launched, 1);
        assert_eq!(mesh.pe(0, 0).fishing.stats.failures_received, 1);
        assert!(!mesh.pe(0, 0).fishing.waiting_for_reply());
        let forwards = mesh.pe(1, 0).fishing.stats.forwarded
            + mesh.pe(2, 0).fishing.stats.forwarded;
        let failed = mesh.pe(1, 0).fishing.stats.failed_at_target
            + mesh.pe(2, 0).fishing.stats.failed_at_target;
        assert!(forwards > 0);
        assert_eq!(failed, 1);

        for _ in 0..FISH_RETRY_DELAY {
            mesh.tick().unwrap();
            assert_eq!(mesh.pe(0, 0).fishing.stats.launched, 1);
        }
        mesh.tick().unwrap();
        assert_eq!(mesh.pe(0, 0).fishing.stats.launched, 2);
    }

    #[test]
    fn two_pe_mesh_responds_without_forwarding() {
        let mut mesh = OurosMesh::new(2, 1, vec![pe(0), pe(1)]);
        mesh.pe_mut(1, 0).fishing.set_enabled(false);
        for _ in 0..256 {
            mesh.tick().unwrap();
            if mesh.pe(0, 0).fishing.stats.failures_received != 0 { break; }
        }
        assert_eq!(mesh.pe(0, 0).fishing.stats.failures_received, 1);
        assert_eq!(mesh.pe(1, 0).fishing.stats.forwarded, 0);
        assert_eq!(mesh.pe(1, 0).fishing.stats.failed_at_target, 1);
    }
}
