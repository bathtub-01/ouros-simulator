# Ouros Network Interface (transport skeleton)

The logical network flit is `Packet<NetworkPayload>`:

- `Packet.source` and `Packet.dest` are the existing NoC router coordinates.
- `NetworkPayload.kind` identifies a message family. `App`, `Fish`, `NoWork`, `Fetch`, `Put`, and `Release` are **labels only** for now, with no corresponding protocol behavior.
- `NetworkPayload.app` is one full Ouros `App`. Some future control packets may not use this field.

The NI does not duplicate the router's local-input FIFO on the outgoing path. `CoreNetworkInput.tx_valid/tx_bits` feed that input directly; `CoreNetworkOutput.tx_ready` reflects the router's local input readiness. The sending side must hold its flit stable until `valid && ready`.

Incoming packets are buffered in the NI's non-pipelined `NI_RX_DEPTH`-entry FIFO, then exposed via a separate `rx_valid/rx_bits/rx_ready` handshake. A full RX FIFO backpressures the router. The NI overwrites the source coordinates on transmission with its own PE coordinates.

`ProcessingElement` owns an `OurosCore` and an NI. `OurosMesh` connects every NI to one router's local port and advances all routers and PEs on the same simulated clock. `ProcessingElementInput.network` is a *temporary protocol-side stub*; **it is not connected to DHeap A or B**. A future protocol dispatcher will interpret incoming messages, request local allocations when needed, construct `ActiveApp` or `FrozenApp`, and join the appropriate DHeap input arbiters.

This milestone does **not** implement message semantics, packet encoding/physical wire width, proxy creation, work stealing, FETCH, distributed GC, or integration of the core's generated Apps with the NI. No received flit is automatically treated as a live App or installed in the heap.
