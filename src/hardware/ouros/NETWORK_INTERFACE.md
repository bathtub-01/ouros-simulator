# Ouros NI and failure-only random fishing

## Packet and NI boundary

The NoC transports `Packet<NetworkPayload>`; the outer packet contains `source` and `dest` PE coordinates, and the payload is a sum type:

```rust
pub enum NetworkPayload {
    App(App),
    Fish { origin: (u8, u8), age: u16 },
    FishFailed,
}
```

`Packet.source` is overwritten by the local NI each time a packet is sent, **including on forwarding**. `Fish.origin` persists across all hops to identify the original fisherman. `age` counts failed probes, **not network-router hops**. No FETCH, PUT, proxy, or migration payload encoding is defined yet.

The NI is a transport: one bounded RX FIFO (`NI_RX_DEPTH`), **no TX FIFO**. Its core-side TX drives the router's two-entry local input FIFO directly, using ready/valid backpressure. The fishing dispatcher uses a *single pending-flit register* to hold output stable until accepted by the router; that register is not a general transmit queue. The NI itself does not decode or rewrite Apps. Non-fishing `App` flits remain visible through the existing external PE protocol stub and are never automatically routed to DHeap A/B.

## Failure-only fishing

Each PE has a `Fishing` controller in `processing_element.rs`:

1. If **thread 0's DHeap address stack is empty**, the PE has no outstanding fish, and it is not cooling down, generate `Fish { origin: self, age: 0 }` to a pseudorandom mesh PE other than itself. This also works for an unstarted idle PE; a 1-node mesh simply does not fish.
2. A PE receiving `Fish` checks whether its spark pool has **any** candidate passing `stillSpark && !working && !WHNF`. This is an advisory probe only, not an atomic claim.
3. If there is **no** candidate, increment `age` and forward the same fish to another pseudorandom PE excluding the current PE and its **original** requester. The forwarded flit's `source` becomes the forwarding PE; `origin` remains unchanged.
4. If the incremented age reaches `FISH_MAX_AGE`, or there is no eligible next hop (e.g., a 2-node mesh), return `FishFailed` directly to the origin. That reply must successfully enter the router input FIFO before the controller can process another incoming FISH.
5. The fisherman holds only **one** outstanding request. On `FishFailed`, clear the pending-request state and wait **`FISH_RETRY_DELAY` complete cycles** before attempting again; send only if thread 0 is still empty.

The RNG is a reproducible, per-PE xorshift64* generator. It samples a set of allowed mesh coordinates without rejection or temporary allocation; this is a simple random probe scheme, *not* guided fishing.

**Success explicitly deferred:** If a PE receives `Fish` and has a usable spark candidate, it does **not** send `FishFailed` and does **not** pop the spark. It leaves the request at the head of its NI RX FIFO for a future claim/migration handler. Such a request can block subsequent RX traffic: this is a deliberate limitation of the failure-only milestone, not a usable success protocol.

## Configuration and follow-on work

- `NI_RX_DEPTH = 4`: receive buffering per NI.
- `FISH_MAX_AGE = 4`: number of unsuccessful candidate probes, including the first recipient.
- `FISH_RETRY_DELAY = 16`: full cycles between receiving failure and starting another request.

No distributed GC, reference counting, graph transfer, proxy allocation, or FETCH/PUT handling is implemented. A success handler must atomically reserve a spark against local activation and reclaim, preserve its reachable graph across migration, and establish distributed reference counts before releasing protection. It must also deal with stillSpark address reuse (ABA); the candidate probe alone is not a proof of eligibility to migrate.

The mesh calls each PE's dispatcher once per simulated cycle, using the same old-cycle RX/TX handshakes as the NI and router. Unit tests cover forwarding exclusions, age, two-node/single-node edges, busy thread 0, failed reply/cooldown, TX backpressure, and a multi-hop mesh failure trace.
