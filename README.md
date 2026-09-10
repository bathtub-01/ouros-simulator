# Ouros Simulator

Cycle-accurate Rust simulator for **Ouros**, a pipelined processor for Haskell based on combinator graph reduction. Ouros uses dataflow-driven execution to pipeline graph reduction and supports automatic fine-grained multithreading and concurrent garbage collection.

This repository contains the simulator used to study cycle counts, reduction activity, allocation behaviour, thread-level activity, and garbage-collection behaviour in the Ouros architecture described in the paper *Ouros: A Dataflow-Driven Processor for Lazy Functional Programming Languages*.

## Requirements

- Rust and Cargo with Rust 2024 edition support.

## Quick start

```sh
git clone https://github.com/bathtub-01/ouros-simulator.git
cd ouros-simulator
cargo run --release -- --mode=all
```

Use `cargo run -- --help` to list the command-line options.

## Usage

Run the bundled benchmark suite:

```sh
cargo run --release -- --mode=all
```

The simulator prints the cycle count, number of reductions, allocations, and peak working-set size for each benchmark. The cycle-count summary is also written to `simu-out/all/cycles.csv`.

Run the garbage-collection evaluation:

```sh
cargo run --release -- --mode=gc
```

GC results are written under `simu-out/gc/`, including cycle counts, GC percentage, maximum pause length, heap/working-set ratios, peak working-set sizes, and GC-round counts.

Run one bundled program:

```sh
cargo run --release -- --prog-name=FIB
```

The currently selected benchmark set is:

`ADJOXO`, `BRAUN`, `CLAUSIFY`, `COUNTDOWN`, `FIB`, `MSS`, `QUEENS`, `QUEENS2`, `SUMEULER`, and `WHILEX`.

`--num-threads=N` controls the number of **host-side Rayon worker threads** used when benchmark runs are evaluated in parallel. It does not change the number of simulated Ouros hardware threads.

## Architecture configuration

The main simulator parameters are defined in:

```text
src/hardware/ouros/config.rs
```

This includes the simulated maximum thread count (`MAX_THREADS`), heap sizes, address-stack size, GC threshold, and App width. Parameters that affect the program representation, such as `APP_LENGTH` and `HOLES`, must remain consistent with the compiler.

## Compiling Haskell programs

The simulator consumes precompiled program images rather than Haskell source directly. The accompanying compiler is a fork of MicroHs:

https://github.com/bathtub-01/MicroHs

The fork can emit the Rust program-image format used by this simulator. The benchmark sources used for Ouros are under `microbenchmarks/` in that repository.

Build the compiler:

```sh
git clone https://github.com/bathtub-01/MicroHs.git
cd MicroHs
cabal install --overwrite-policy=always
```

To generate Rust images for the supplied benchmarks:

```sh
cd microbenchmarks
make all-rs
```

For a single `MyProgram.hs` in that directory, the corresponding command is:

```sh
mhs -i../lib MyProgram -omyprogram.rs
```

The generated `.rs` file defines the heap and combinator images consumed by the simulator. To add a new program:

1. Copy the generated `.rs` file into `src/hardware/ouros/benchmarks/`.
2. Register the module in `src/hardware/ouros/benchmarks/mod.rs`.
3. Add its exported program name to the `benchmarks!(...)` list in `src/main.rs` if it should be selectable from the command line or included in suite runs.
4. Rebuild and run it with `--prog-name=<NAME>`.

The supplied MicroHs benchmark sources are useful templates for the Haskell subset and entry-point style expected by the hardware-oriented backend.

## Related repositories

- Ouros simulator: https://github.com/bathtub-01/ouros-simulator
- Ouros hardware implementation: https://github.com/bathtub-01/ouros-chisel
- Ouros MicroHs compiler fork: https://github.com/bathtub-01/MicroHs
