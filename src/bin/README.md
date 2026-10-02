# Ouros NoC traffic evaluation

Copy the two Rust files into their corresponding locations in the **existing**
`ouros-simulator` repository (the `multi-core` branch):

- `src/hardware/noc/noc_eval.rs` — measurement implementation.
- `src/bin/noc_eval.rs` — standalone executable entry point, automatically
  discovered by Cargo; `Cargo.toml` does **not** need to be edited.

Requires the existing `src/hw_module.rs`, `src/hardware/common/{arbiter,fifo}.rs`,
`src/hardware/noc/{router,network}.rs` files. This bundle doesn't replace them.

Run from the repo root:

```bash
cargo run --release --bin noc_eval -- \
  --width 4 --height 4 --packets 50000 \
  --pattern both --hotspot 2,2 --hotspot-prob 0.5 \
  --rate-start 0.02 --rate-end 0.5 --rate-step 0.02 \
  --seed 42
```

For an explicit sweep use `--rates 0.01,0.05,0.1,0.2,0.4` instead of the
three sweep options. `--pattern` supports `random`, `hotspot` and `both`.
`--help` shows all parameters. Run tests with
`cargo test --bin noc_eval`.

## Semantics

- Every node independently **offers** a new packet in a cycle with probability
  `rate`. The offered load is in packets / node / cycle and can exceed the
  network's service rate.
- Source-side host queues retain offered packets until a local
  `valid && ready` handshake. Nothing is dropped on backpressure.
- Random destinations are uniform over other nodes (except a 1x1 mesh).
- In hotspot mode, nodes other than the hotspot explicitly choose it with
  probability `hotspot_prob`; otherwise they choose a uniformly random
  destination. The hotspot node picks a random other destination. The actual
  proportion of traffic arriving at the hotspot includes random background
  traffic, so it can exceed `hotspot_prob`.
- All local sinks are always ready. Each rate and pattern starts from a fresh
  empty network, generates exactly `--packets` packets, then drains them.
- Cycles are sampled at the endpoints' ready/valid handshake. `network_delay`
  is reception cycle minus accepted injection cycle. `source_wait` is accepted
  injection cycle minus packet generation cycle. `end_to_end` is the sum. The
  former measures router/network transit; the latter also captures injection
  queueing under overload.

## Output

Under `simu-out/noc` by default:

- `summary.csv`: one row per pattern and rate, including average/p50/p95/p99/max
  network transit delay, average source wait and end-to-end delay, delivered
  throughput and accepted rate during generation.
- One CSV per pattern/rate with **one row for every delivered packet**,
  including src/dst coordinates, generation/injection/reception cycles and
  all three latency measures. These files are written in completion order.

`summary.csv` is overwritten on each invocation; use `--out-dir` to keep
separate experiment sets. No warm-up or steady-state window is applied, so the
statistics include cold start and finite-run drain. For the classic curve,
plot `offered_rate` against `mean_network_delay` (or `mean_end_to_end` if you
want saturation-induced waiting at injection queues to be included).



# Property-based NoC tests (add-on)

This bundle adds `src/hardware/noc/noc_property_tests.rs` and a single
`#[cfg(test)]` module declaration at the end of `network.rs`. There are **no
production-behavior changes**.

Add this development dependency to the repository root `Cargo.toml`:

```toml
[dev-dependencies]
proptest = "1"
```

If `[dev-dependencies]` already exists, add just `proptest = "1"` to it.

Run:

```sh
cargo test --bin noc_eval finite_delivery_and_fifo_order -- --nocapture
cargo test --bin noc_eval sustained_hotspot_and_same_flow_contention
```

The standalone `noc_eval` launcher imports `network.rs`, so these tests are
available with `cargo test --bin noc_eval` as well as any other target that
includes `network.rs`.

The generated cases cover 1x1 to 4x4 meshes; up to 64 packets; random or
hotspot destinations; and a finite prefix of randomized sink backpressure.
Every injected packet carries a unique positive ID. The test checks:

1. A packet can only be received if it was accepted for injection.
2. Every offered packet gets injected, and every accepted packet is received
   **exactly once** before a generous bounded timeout.
3. A packet arrives at its proper local port with unchanged source/dest.
4. For a particular `(src,dest)` pair, delivered IDs match injection order.
5. No extra packet appears during an additional idle-drain interval.

As with all randomized testing, passing is evidence, not a formal liveness
proof. Eventual delivery depends on assumptions such as finite injection,
receivers eventually asserting ready, fair arbitration, and no router faults.
The generated receivers all become continuously ready after the stall prefix.
