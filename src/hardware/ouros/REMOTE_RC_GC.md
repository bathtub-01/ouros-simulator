# Remote-reference roots in the local collector

Each Ouros GC owns a separate `SinglePortMem<u16>` with one entry per local
heap cell, initially zero. This is the **incoming** remote-reference count:
remote references from other PEs to a local heap address. Its exact update
protocol is **not implemented** in this change. In a hardware version the
counter width should be sized for the maximum supported outstanding references,
not assumed to be 16 bits simply because the simulator uses `u16`.

## Rooting

A collection's Root phase has two consecutive stages:

1. Insert the existing permanent program-image roots into the worklist.
2. Scan the remote-reference memory sequentially, one address per cycle.
   When the count is nonzero, append that address to the worklist unless it
   is already one of the permanent program-image roots.

The scan uses a synchronous read port and respects the collector's existing
mutator-request priority: a stalled Root scan retains its current read result
and does not enqueue the same root twice. Mark starts only after the scan
finishes. Queued sparks remain **non-roots**.

## Required for later distributed GC work

- Nonzero counts must refer to allocated cells, not freelist entries.
- Reference creation/destruction needs serialized increment/decrement logic;
  underflow/overflow must be prevented.
- A 0-to-nonzero transition while a collection is in Mark must provide a
  new-root barrier. During Sweep, a previously reclaimed cell cannot be
  resurrected just by incrementing its count.
- In-flight transfers and their references must be accounted for before the
  source releases temporary claim roots.
- Incoming reference counts can retain garbage spanning multiple PEs in a
  cycle; distributed cycle collection remains out of scope.

Full-heap RC scanning costs O(heap size) cycles per collection; it is
intentionally simple for this initial implementation.
