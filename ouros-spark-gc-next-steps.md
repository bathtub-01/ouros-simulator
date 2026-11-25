# Ouros multi-core: Spark–GC integration notes

**Status:** Design notes for the future work-stealing implementation. No change to the current single-core spark-pool logic is required.

## Existing behaviour

- `par a b` offers a pointer to `a` to a bounded FIFO and continues with `b`.
- **Queued sparks are hints, not GC roots.** A full pool may drop an offer; GC may reclaim an otherwise unreachable spark target or its descendants.
- The spark pool owns per-heap-address `stillSpark` bits. GC clears a bit when it reclaims that address, **both on sweep and on immediate one-bit-RC reclamation**. FIFO entries are not removed by GC and may be stale.
- `spark_front_stealable()` currently checks `stillSpark && !working && !WHNF`. This is a **candidate filter only**: no claim, migration, or remote fetch has been implemented.

## Required claim protocol

1. **Validate and reserve atomically.** Serialize `FISH` claiming against local DHeap activation, GC reclamation, and address reuse. Retire invalid FIFO entries without generating work.
2. **Promote the claim to a temporary GC root.** Preserving only the root App's original contents is insufficient: any child App could already have been reclaimed. The claimed root **and its reachable graph** must be protected until the migration's references are safely published.
3. **Respect GC phases.** Claims in `Idle` may register the temporary root normally. Claims during `Root` must be included in the pending root set; claims during `Mark` need a **new-root/worklist barrier** that completes before sweep. **Defer or reject claims during `Sweep`**: descendants may already have been reclaimed, even while `stillSpark` for the root remains set. There is no need for a blanket “claim only when GC is idle” rule.
4. **Transfer one App, then establish remote references.** As proposed, `PUT(MOVE)` moves only the root App, with its child pointers represented as destination-local fetch proxies. Account for each destination proxy to a child by incrementing that child's incoming remote-reference count on the source PE; a source-side forwarding proxy to the moved root likewise creates an incoming reference on the destination PE. Protect in-flight transfers and establish these references **before releasing the temporary root**. Newly established remote roots during an active collection need a new-root barrier at the referenced PE.
5. **Allow ordinary reclamation afterward.** Once ownership transfer and reference accounting are committed, the old source App can become a forwarding proxy. GC may reclaim that proxy if unreachable and if no outstanding demand, in-flight message, or remote reference requires it. Useless speculative work may safely execute remotely.

## Address-reuse and FIFO concerns

- **ABA/re-spark:** GC can clear `stillSpark[p]`, later reuse `p`, and a new `par` can set it again. An older FIFO entry for `p` would then appear valid. If FIFO entries must identify their *original* App, add a per-allocation generation/epoch to both the entry and the address metadata; check it at the serialized claim point. Alternatively, explicitly define hints as address-based and allow an old entry to select a newly sparked App; the latter relaxes the original-content invariant.
- Multiple hints for the same address share one flag. Retiring one currently clears the flag for all, potentially discarding useful work; this is a **performance** issue, not a correctness issue, while sparks remain optional.
- Stale hints consume FIFO space until popped; a future stealer should discard them as it scans for a candidate.
- The flag/WHNF/working test must not be treated as proof that children are still live, especially mid-sweep.

## Tests to add with work stealing

- Claim versus local DHeap activation; claim versus immediate reclaim/reuse.
- Claim during `Root`/`Mark` (including a newly claimed root with unmarked descendants); defer during `Sweep`.
- Stale FIFO entry after sweep, and ABA after address reuse and re-sparking.
- Ownership transfer with shared child pointers, correct remote-reference increments, in-flight `PUT`, and reclamation of an unused source forwarding proxy.
- Claim cancellation or failed message enqueueing must release any reservation/temporary root without losing required references.

**Invariant:** Unclaimed hints can be discarded at any time. A successfully *claimed* spark must identify a valid, protected computation until its transfer and distributed reachability bookkeeping commit.
