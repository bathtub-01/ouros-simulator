# Ouros Simulator: RealWorld I/O Extension Roadmap

## Approach

Extend the graph-reduction machine with an `IO` Atom and a dedicated I/O unit. I/O remains an ordinary App reduction: the unit performs an AXI4-Lite transaction, then returns a Pair App to the dataflow ring. No imperative execution path or global core stall is introduced.

The surface interface is defined by `Ouros.IO`: `IO a` wraps `World -> (a, World)`, and `World` is a dummy `Int`. The compiler must place an application of `main` to the initial World token at heap address 0. The token is a normal `INT`, **not** a new Atom type. The graph dependency on World orders effects; completion of the I/O reduction makes the resulting World available.

| Primitive Atom | Arity | Saturated request | Result after transaction completion |
| --- | ---: | --- | --- |
| `IO false` (read) | 2 | `[IO false, INT addr, World]` | `[Pair comb, INT data, World]` |
| `IO true` (write) | 3 | `[IO true, INT addr, INT data, World]` | `[Pair comb, Dummy INT, World]` |

`World` in these examples denotes the dummy `INT` argument. Unsaturated `IO` Apps are in WHNF and cause no side effect. Saturated `IO` Apps are strict in their arguments: DHeap must resolve their pointers and produce unboxed arguments before dispatch. The I/O unit never accesses the heap.

**Routing:** All `IO` Apps, including those emitted by the Reducer, enter DHeap first. DHeap sends only ready, saturated `IO` Apps to the *I/O unit*. Completed Pair Apps return to **DHeap's input arbitration**, preserving ordinary dereference, thread, and reduction semantics. The I/O unit is an AXI4-Lite master connected to an external I/O controller, which provides access to AXI4-Lite peripherals. Model the controller and peripherals in the simulator; initially permit one outstanding transaction.

## TODO

### 1. Representation and program entry

- [x] Add `IO` to the Atom encoding, with a one-bit `isWrite` payload. Update parsing, printing, program-image decoding, and any exhaustive Atom matches.
- [x] Resolve Atom tag capacity and AXI4-Lite address/data widths; keep simulator and compiler encodings consistent. Document any initial restriction on 32-bit register values.
- [x] Accept program images with the compiler-generated root `[main, World]` at heap address 0; retain the existing initialization and startup flow.
- [x] Add helpers to identify `IO` arity, saturation, and operand positions without conflating `IO` with `PRM`.

### 2. DHeap strictness and routing

- [x] Extend DHeap's demand/dereference logic to force the address, write data (for writes), and World arguments of saturated `IO` Apps to unboxed `INT` values, using the existing suspension and reactivation mechanism.
- [x] Preserve partial `IO` applications as WHNF. Do not issue a transaction until all required arguments are present and unboxed.
- [x] Route every `IO` App through DHeap, including Reducer outputs; dispatch only ready, saturated Apps to the I/O unit.
- [x] Preserve thread identity, stack/dependency bookkeeping, and normal App handling across the I/O round trip; avoid issuing the same effect twice on suspension or replay.

### 3. I/O unit and AXI4-Lite model

- [x] Add a I/O unit (e.g. `src/hardware/ouros/io_unit.rs`) into Ouros, with ready-valid backpressure input and a single in-flight request.
- [x] Implement AXI4-Lite read (`AR`/`R`) and write (`AW`/`W`/`B`) channel handshakes, including independent write-address/data acceptance, backpressure, and response latency.
  ```
  Peripherals (e.g., UART) <-- abstracted --> I/O-controller <-- simulated AXI transaction --> I/O unit + Ouros
  ```
- [x] Model the I/O-controller as a register bank, and simulate a UART-styled terminal interacting with the register bank.
- [x] Complete each reduction **only after** the corresponding AXI response from the I/O controller: emit `[Pair comb, INT result, World]` for reads or `[Pair comb, Dummy INT, World]` for writes.
- [x] Re-inject completed Apps through DHeap input arbitration. Initially don't assume any AXI error.

### 4. Core integration and GC

- [x] Connect the I/O unit in `src/hardware/ouros/ouros_core.rs`; extend DHeap output routing and DHeap input arbitration without creating a direct Reducer-to-I/O bypass.
- [x] Add I/O-unit activity to simulator stepping. Other runnable threads must continue while an AXI transaction is pending.
- [x] Ensure Apps retained by the I/O unit or its queues remain visible to GC root tracking for the entire transaction, including under backpressure.
- [x] Preserve the existing semantics of `main` completion: the root result reaches WHNF and execution terminates.

## Initial milestone

Extend Ouros with a simulated UART, running a UART echo program written in Haskell.

**Out of scope for this milestone:** interrupts, multiple outstanding AXI transactions, realistic UART/timer device models.
