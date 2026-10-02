//! Property-based checks for a *finite* packet workload.
//!
//! The generated sink stalls have finite duration; after that, every sink
//! remains ready. This is essential for a meaningful eventual-delivery test.
//! A cycle bound makes failures terminate and gives proptest a counterexample
//! to shrink. It is a test of bounded liveness, not a mathematical proof.

use super::Network;
use crate::hw_module::HwModule;
use super::super::router::Packet;
use proptest::prelude::*;
use proptest::test_runner::{TestCaseError, TestCaseResult};
use std::collections::VecDeque;

fn coordinates(node: usize, width: usize) -> (u8, u8) {
    ((node % width) as u8, (node / width) as u8)
}

fn check_finite_traffic(
    width: usize,
    height: usize,
    traffic: &[(u8, u8)],
    stall_masks: &[u16],
    hotspot: bool,
) -> TestCaseResult {
    let nodes = width * height;
    let total = traffic.len();
    let mut noc = Network::<u64>::new(width, height);

    // Each source offers its packets in a fixed order. A packet only leaves
    // this queue on a valid/ready injection handshake (not merely on valid).
    let mut sources: Vec<VecDeque<usize>> = (0..nodes).map(|_| VecDeque::new()).collect();
    let mut plans = Vec::with_capacity(total);
    for (id, &(raw_src, raw_dst)) in traffic.iter().enumerate() {
        let src = (raw_src as usize) % nodes;
        let dst = if hotspot {
            (nodes - 1) / 2
        } else {
            (raw_dst as usize) % nodes
        };
        sources[src].push_back(id);
        plans.push((src, dst));
    }

    // Ground truth is in INJECTION order for every (source, destination)
    // flow, not just the order in which test traffic was generated.
    let mut flow_expected: Vec<VecDeque<usize>> =
        (0..nodes * nodes).map(|_| VecDeque::new()).collect();
    let mut injected = vec![false; total];
    let mut received = vec![false; total];
    let mut injected_count = 0usize;
    let mut received_count = 0usize;

    // Arbitrarily stalled sinks for the generated prefix, *always ready*
    // thereafter. With finitely many packets, this creates drain conditions
    // under which a deadlock-free fair mesh should eventually become empty.
    let limit = 1_000 + 40 * total + 20 * nodes + stall_masks.len();
    for cycle in 0..limit {
        for node in 0..nodes {
            let (x, y) = coordinates(node, width);
            let port = &mut noc.input.local[y as usize][x as usize];
            port.out_ready = stall_masks
                .get(cycle)
                .map(|mask| (mask & (1u16 << node)) != 0)
                .unwrap_or(true);

            if let Some(&id) = sources[node].front() {
                let (src, dst) = plans[id];
                port.in_valid = true;
                port.in_bits = Packet {
                    source: coordinates(src, width),
                    dest: coordinates(dst, width),
                    load: (id + 1) as u64, // zero is reserved as an invalid ID
                };
            } else {
                port.in_valid = false;
                port.in_bits = Packet::default();
            }
        }

        // All observations must be taken BEFORE the edge. Router output
        // valid/bits are derived from its current FIFO heads; the previous
        // bug involved stale combinational arbiter inputs after an edge.
        let mut accepts = Vec::new();
        let mut deliveries = Vec::new();
        for node in 0..nodes {
            let (x, y) = coordinates(node, width);
            let output = noc.local_out(x as usize, y as usize);
            if output.in_ready && !sources[node].is_empty() {
                accepts.push(node);
            }
            if output.out_valid && noc.input.local[y as usize][x as usize].out_ready {
                deliveries.push((node, output.out_bits));
            }
        }

        // The entire network advances exactly once. Never call child
        // update_local() directly, including to observe pre-edge signals.
        noc.tick().map_err(|err| {
            TestCaseError::fail(format!("cycle {cycle}: NoC tick failed: {err}"))
        })?;

        // Register all injections before checking deliveries: this also
        // supports a hypothetical zero-cycle local path in the future.
        for src in accepts {
            let id = sources[src].pop_front().expect("accepted source had a packet");
            prop_assert!(!injected[id], "packet {} injected twice", id + 1);
            injected[id] = true;
            injected_count += 1;
            let dst = plans[id].1;
            flow_expected[src * nodes + dst].push_back(id);
        }

        for (sink, packet) in deliveries {
            prop_assert!(
                (1..=(total as u64)).contains(&packet.load),
                "unknown packet ID {} received at node {} cycle {}",
                packet.load, sink, cycle
            );
            let id = (packet.load - 1) as usize;
            let (src, dst) = plans[id];
            prop_assert!(injected[id], "packet {} arrived without injection", id + 1);
            prop_assert!(!received[id], "duplicate delivery of packet {}", id + 1);
            prop_assert_eq!(sink, dst, "packet {} reached wrong local port", id + 1);
            prop_assert_eq!(packet.source, coordinates(src, width),
                "packet {} source corrupted", id + 1);
            prop_assert_eq!(packet.dest, coordinates(dst, width),
                "packet {} destination corrupted", id + 1);

            // This is the per-(src,dst) ordering property, independent of
            // interleaving with other sources or destinations.
            let expected_id = flow_expected[src * nodes + dst].pop_front();
            prop_assert_eq!(expected_id, Some(id),
                "flow ({} -> {}) reordered at cycle {}; got packet {}",
                src, dst, cycle, id + 1);
            received[id] = true;
            received_count += 1;
        }
        if received_count == total {
            break;
        }
    }

    prop_assert_eq!(injected_count, total,
        "not all offered packets could be injected (source deadlock?)");
    prop_assert_eq!(received_count, total,
        "accepted packets failed to drain (loss, unfairness or deadlock)");
    prop_assert!(sources.iter().all(VecDeque::is_empty));
    prop_assert!(flow_expected.iter().all(VecDeque::is_empty));
    prop_assert!(injected.iter().all(|flag| *flag));
    prop_assert!(received.iter().all(|flag| *flag));

    // Continue advancing an otherwise idle mesh to detect *late* replay
    // after the last expected packet; all outputs remain ready throughout.
    for _ in 0..(32 + 8 * nodes) {
        for row in &mut noc.input.local {
            for port in row {
                port.in_valid = false;
                port.out_ready = true;
            }
        }
        for node in 0..nodes {
            let (x, y) = coordinates(node, width);
            let output = noc.local_out(x as usize, y as usize);
            prop_assert!(!output.out_valid,
                "unexpected/duplicate flit after all packets were delivered: node {} load {}",
                node, output.out_bits.load);
        }
        noc.tick().map_err(|e| TestCaseError::fail(format!("drain tick: {e}")))?;
    }
    Ok(())
}

proptest! {
    // Many inexpensive small meshes tend to provide more useful shrinkable
    // counterexamples than a few very large benchmark runs.
    #![proptest_config(ProptestConfig {
        cases: 64,
        max_shrink_iters: 1024,
        .. ProptestConfig::default()
    })]
    #[test]
    fn finite_delivery_and_fifo_order(
        width in 1usize..=4,
        height in 1usize..=4,
        traffic in proptest::collection::vec((any::<u8>(), any::<u8>()), 1..65),
        sink_stalls in proptest::collection::vec(any::<u16>(), 0..40),
        hotspot in any::<bool>(),
    ) {
        check_finite_traffic(width, height, &traffic, &sink_stalls, hotspot)?;
    }
}

#[test]
fn sustained_hotspot_and_same_flow_contention() {
    // Repeated 0 -> (3,3) and 3 -> (3,3) flows under burst contention.
    let traffic: Vec<(u8, u8)> = (0..48)
        .map(|i| if i % 3 == 0 { (0, 15) } else { (3, 15) })
        .collect();
    // For the first few cycles, the destination is intermittently stalled.
    let stalls = vec![0u16, 0, 1 << 15, 0, 1 << 15, 0, 0, 1 << 15];
    check_finite_traffic(4, 4, &traffic, &stalls, false).unwrap();
}
