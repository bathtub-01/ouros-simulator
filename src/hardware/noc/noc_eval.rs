//! Cycle-accurate NoC traffic experiment.
//!
//! Run through the small Cargo binary wrapper in `src/bin/noc_eval.rs`:
//!   cargo run --release --bin noc_eval -- --width 4 --height 4 --packets 50000 \
//!       --rate-start 0.02 --rate-end 0.50 --rate-step 0.02 --pattern both \
//!       --hotspot 2,2 --hotspot-prob 0.5
//!
//! A rate is the Bernoulli probability of *offering* one new packet at
//! each node on each clock cycle, NOT the probability of successfully
//! injecting into the router. An unbounded source-side queue retains
//! offered packets when the local router is not ready.
//!
//! Latency is measured at local valid/ready handshakes, in cycles:
//!   network_delay = received_cycle - injected_cycle
//!   source_wait   = injected_cycle - offered_cycle
//! All accepted packets are drained before the run ends.

use crate::hardware::noc::network::{Network, MAX_MESH_DIM};
use crate::hardware::noc::router::Packet;
use crate::hw_module::HwModule;
use std::collections::VecDeque;
use std::convert::TryFrom;
use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Pattern {
    Random,
    Hotspot,
}

impl Pattern {
    fn name(self) -> &'static str {
        match self {
            Self::Random => "random",
            Self::Hotspot => "hotspot",
        }
    }
}

struct Config {
    width: usize,
    height: usize,
    packets: usize,
    rates: Vec<f64>,
    patterns: Vec<Pattern>,
    hotspot: (usize, usize),
    hotspot_prob: f64,
    seed: u64,
    max_cycles: Option<u64>,
    output_dir: PathBuf,
}

fn usage() -> &'static str {
    "Usage: cargo run --release --bin noc_eval -- [OPTIONS]\n\
     \n\
     --width N             Mesh width (default 4)\n\
     --height N            Mesh height (default 4)\n\
     --packets N           Total offered/delivered packets PER RUN (default 20000)\n\
     --pattern MODE        random, hotspot, or both (default both)\n\
     --hotspot X,Y         Hotspot coordinates (default mesh centre)\n\
     --hotspot-prob P      Probability of choosing hotspot (default 0.50)\n\
     --rate-start R        Sweep start, packets/node/cycle (default 0.02)\n\
     --rate-end R          Sweep end, inclusive (default 0.50)\n\
     --rate-step R         Sweep increment (default 0.02)\n\
     --rates R1,R2,...     Explicit rate list; overrides sweep arguments\n\
     --seed N              Deterministic PRNG seed (default 1)\n\
     --max-cycles N        Abort a run after this many cycles (default automatic)\n\
     --out-dir PATH        CSV destination (default simu-out/noc)\n\
     -h, --help            Show this message"
}

fn parse_pair(text: &str) -> Result<(usize, usize)> {
    let (x, y) = text.split_once(',').ok_or("expected coordinates X,Y")?;
    Ok((x.parse()?, y.parse()?))
}

fn parse_args() -> Result<Option<Config>> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("{}", usage());
        return Ok(None);
    }

    let mut width = 4usize;
    let mut height = 4usize;
    let mut packets = 20_000usize;
    let mut pattern = String::from("both");
    let mut hotspot = None;
    let mut hotspot_prob = 0.5f64;
    let mut start = 0.02f64;
    let mut end = 0.50f64;
    let mut step = 0.02f64;
    let mut explicit_rates: Option<String> = None;
    let mut seed = 1u64;
    let mut max_cycles = None;
    let mut output_dir = PathBuf::from("simu-out/noc");

    let mut it = args.iter();
    while let Some(flag) = it.next() {
        let value = it.next().ok_or_else(|| format!("missing value for {flag}"))?;
        match flag.as_str() {
            "--width" => width = value.parse()?,
            "--height" => height = value.parse()?,
            "--packets" => packets = value.parse()?,
            "--pattern" => pattern = value.clone(),
            "--hotspot" => hotspot = Some(parse_pair(value)?),
            "--hotspot-prob" => hotspot_prob = value.parse()?,
            "--rate-start" => start = value.parse()?,
            "--rate-end" => end = value.parse()?,
            "--rate-step" => step = value.parse()?,
            "--rates" => explicit_rates = Some(value.clone()),
            "--seed" => seed = value.parse()?,
            "--max-cycles" => max_cycles = Some(value.parse()?),
            "--out-dir" => output_dir = PathBuf::from(value),
            _ => return Err(format!("unknown option: {flag}\n{}", usage()).into()),
        }
    }

    if !(1..=MAX_MESH_DIM).contains(&width) || !(1..=MAX_MESH_DIM).contains(&height) {
        return Err(format!("width and height must each be 1..={MAX_MESH_DIM}").into());
    }
    if packets == 0 {
        return Err("--packets must be positive".into());
    }
    if !hotspot_prob.is_finite() || !(0.0..=1.0).contains(&hotspot_prob) {
        return Err("--hotspot-prob must lie in [0,1]".into());
    }
    let hotspot = hotspot.unwrap_or((width / 2, height / 2));
    if hotspot.0 >= width || hotspot.1 >= height {
        return Err("hotspot coordinates are outside the mesh".into());
    }
    if matches!(max_cycles, Some(0)) {
        return Err("--max-cycles must be positive".into());
    }
    let patterns = match pattern.as_str() {
        "random" => vec![Pattern::Random],
        "hotspot" => vec![Pattern::Hotspot],
        "both" => vec![Pattern::Random, Pattern::Hotspot],
        _ => return Err("--pattern must be random, hotspot, or both".into()),
    };

    let rates = if let Some(csv) = explicit_rates {
        csv.split(',')
            .map(|s| s.trim().parse::<f64>().map_err(|e| -> Box<dyn Error> { Box::new(e) }))
            .collect::<Result<Vec<_>>>()?
    } else {
        if !start.is_finite() || !end.is_finite() || !step.is_finite()
            || step <= 0.0 || start > end
        {
            return Err("invalid rate sweep bounds/step".into());
        }
        // Derive each point from its index, avoiding cumulative FP drift.
        let count = ((end - start) / step + 1e-9).floor() as usize + 1;
        if count > 10_000 {
            return Err("rate sweep contains over 10000 points".into());
        }
        (0..count).map(|i| start + (i as f64) * step).collect()
    };
    if rates.is_empty() || rates.iter().any(|r| !r.is_finite() || *r <= 0.0 || *r > 1.0) {
        return Err("all rates must be in (0,1]".into());
    }
    // Each point writes a distinct filename, so reject accidental collisions.
    let mut sorted_labels: Vec<String> = rates.iter().map(|r| format!("{r:.6}")).collect();
    sorted_labels.sort();
    if sorted_labels.windows(2).any(|w| w[0] == w[1]) {
        return Err("duplicate injection rates at six decimal places".into());
    }

    Ok(Some(Config {
        width, height, packets, rates, patterns, hotspot, hotspot_prob,
        seed, max_cycles, output_dir,
    }))
}

/// Simple deterministic, allocation-free PRNG; no extra Cargo dependency.
struct SplitMix64(u64);

impl SplitMix64 {
    fn new(seed: u64) -> Self { Self(seed) }

    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }

    fn fraction(&mut self) -> f64 {
        ((self.next_u64() >> 11) as f64) * (1.0 / ((1u64 << 53) as f64))
    }

    fn usize_below(&mut self, limit: usize) -> usize {
        assert!(limit != 0);
        let limit = limit as u64;
        let cutoff = limit.wrapping_neg() % limit;
        loop {
            let candidate = self.next_u64();
            if candidate >= cutoff { return (candidate % limit) as usize; }
        }
    }
}

/// Select a uniformly random *different* node (except on a 1x1 mesh).
fn random_other_node(rng: &mut SplitMix64, source_index: usize, nodes: usize) -> usize {
    if nodes == 1 { return 0; }
    let r = rng.usize_below(nodes - 1);
    if r >= source_index { r + 1 } else { r }
}

fn choose_destination(
    rng: &mut SplitMix64, src: usize, nodes: usize,
    pattern: Pattern, hotspot_index: usize, hotspot_prob: f64,
) -> usize {
    if pattern == Pattern::Hotspot && src != hotspot_index && rng.fraction() < hotspot_prob {
        hotspot_index
    } else {
        random_other_node(rng, src, nodes)
    }
}

struct PacketMeta {
    source: usize,
    dest: usize,
    offered: u64,
    injected: Option<u64>,
    received: bool,
}

struct RunStats {
    cycles: u64,
    generation_cycles: u64,
    injection_rate_during_generation: f64,
    throughput_per_node: f64,
    mean: f64,
    p50: u64,
    p95: u64,
    p99: u64,
    max: u64,
    mean_source_wait: f64,
    mean_end_to_end: f64,
}

fn percentile(sorted: &[u64], percentile: f64) -> u64 {
    let i = ((sorted.len() as f64 * percentile).ceil() as usize).saturating_sub(1);
    sorted[i.min(sorted.len() - 1)]
}

fn evaluate(config: &Config, pattern: Pattern, rate: f64, path: &Path) -> Result<RunStats> {
    let nodes = config.width * config.height;
    let hotspot_index = config.hotspot.1 * config.width + config.hotspot.0;
    let mut rng = SplitMix64::new(config.seed);
    let mut network = Network::<u64>::new(config.width, config.height);
    let mut queues: Vec<VecDeque<usize>> = (0..nodes).map(|_| VecDeque::new()).collect();
    let mut meta = Vec::<PacketMeta>::with_capacity(config.packets);
    let mut latencies = Vec::<u64>::with_capacity(config.packets);
    let mut delivered = 0usize;
    let mut injected = 0usize;
    let mut injected_during_generation = 0usize;
    let mut generation_cycles = 0u64;
    let mut total_wait = 0u128;
    let mut total_end_to_end = 0u128;
    let mut total_delay = 0u128;

    let file = File::create(path)?;
    let mut csv = BufWriter::new(file);
    writeln!(csv, "packet_id,src_x,src_y,dst_x,dst_y,offered_cycle,injected_cycle,received_cycle,source_wait_cycles,network_delay_cycles,end_to_end_delay_cycles")?;

    // A finite run always drains all generated packets. Provide a generous
    // timeout that also accommodates very small offered rates.
    let expected_offering_cycles = (config.packets as f64 / (nodes as f64 * rate)).ceil() as u64;
    let default_limit = expected_offering_cycles.saturating_mul(12)
        .saturating_add((config.packets as u64).saturating_mul(50))
        .saturating_add(10_000);
    let limit = config.max_cycles.unwrap_or(default_limit);

    for cycle in 0..limit {
        if delivered == config.packets {
            break;
        }

        let generating = meta.len() < config.packets;
        if generating {
            generation_cycles = cycle + 1;
            for source in 0..nodes {
                if meta.len() == config.packets { break; }
                if rng.fraction() < rate {
                    let dest = choose_destination(
                        &mut rng, source, nodes, pattern,
                        hotspot_index, config.hotspot_prob,
                    );
                    let id_index = meta.len();
                    meta.push(PacketMeta {
                        source, dest, offered: cycle, injected: None, received: false,
                    });
                    queues[source].push_back(id_index);
                }
            }
        }

        // Set the input signal bundle for the whole cycle. A queued packet
        // holds its valid/bits until the router acknowledges it by in_ready.
        for source in 0..nodes {
            let x = source % config.width;
            let y = source / config.width;
            let port = &mut network.input.local[y][x];
            port.out_ready = true; // Ideal, always-ready local receiving cores.
            if let Some(&id_index) = queues[source].front() {
                port.in_valid = true;
                port.in_bits = Packet {
                    source: (x as u8, y as u8),
                    dest: ((meta[id_index].dest % config.width) as u8,
                           (meta[id_index].dest / config.width) as u8),
                    load: (id_index as u64) + 1, // 0 is reserved as invalid.
                };
            } else {
                port.in_valid = false;
                port.in_bits = Packet::default();
            }
        }

        // Sample valid/ready from pre-edge registered state. These are the
        // transfers completed ON the ensuing network.tick(). Do not call
        // network.update_local() here; the HwModule lifecycle owns it.
        let outputs: Vec<_> = (0..nodes)
            .map(|node| network.local_out(node % config.width, node / config.width))
            .collect();

        // Capture handshake events before advancing the whole network. Every
        // endpoint is unconditionally ready, so any out_valid is accepted.
        let mut injections = Vec::new();
        let mut receptions = Vec::new();
        for source in 0..nodes {
            if outputs[source].in_ready {
                if let Some(&id_index) = queues[source].front() {
                    injections.push((source, id_index));
                }
            }
            if outputs[source].out_valid {
                let packet = &outputs[source].out_bits;
                let id_index = usize::try_from(packet.load.checked_sub(1)
                    .ok_or("received zero packet identifier")?)?;
                if id_index >= meta.len() {
                    return Err(format!("received unknown packet ID {} at cycle {cycle}", packet.load).into());
                }
                if meta[id_index].dest != source ||
                    packet.dest != ((source % config.width) as u8, (source / config.width) as u8) ||
                    packet.source != ((meta[id_index].source % config.width) as u8,
                                      (meta[id_index].source / config.width) as u8) {
                    return Err(format!("packet {} delivered to wrong node at cycle {cycle}", packet.load).into());
                }
                receptions.push(id_index);
            }
        }

        network.tick().map_err(|e| format!("NoC cycle {cycle}: {e}"))?;

        for (source, id_index) in injections {
            let popped = queues[source].pop_front();
            if popped != Some(id_index) || meta[id_index].injected.is_some() {
                return Err(format!("inconsistent injection bookkeeping for packet {}", id_index + 1).into());
            }
            meta[id_index].injected = Some(cycle);
            injected += 1;
            if generating { injected_during_generation += 1; }
        }

        for id_index in receptions {
            let m = &mut meta[id_index];
            if m.received { return Err(format!("packet {} received twice", id_index + 1).into()); }
            let injection_cycle = m.injected.ok_or_else(||
                format!("packet {} received before injection", id_index + 1))?;
            let network_delay = cycle.checked_sub(injection_cycle).ok_or("negative packet latency")?;
            let source_wait = injection_cycle - m.offered;
            let end_to_end = cycle - m.offered;
            m.received = true;
            delivered += 1;
            total_delay += u128::from(network_delay);
            total_wait += u128::from(source_wait);
            total_end_to_end += u128::from(end_to_end);
            latencies.push(network_delay);
            writeln!(csv, "{},{},{},{},{},{},{},{},{},{},{}",
                id_index + 1,
                m.source % config.width, m.source / config.width,
                m.dest % config.width, m.dest / config.width,
                m.offered, injection_cycle, cycle,
                source_wait, network_delay, end_to_end,
            )?;
        }

        if delivered == config.packets {
            csv.flush()?;
            latencies.sort_unstable();
            let cycles = cycle + 1;
            return Ok(RunStats {
                cycles,
                generation_cycles,
                injection_rate_during_generation: injected_during_generation as f64 /
                    (nodes as f64 * generation_cycles as f64),
                throughput_per_node: delivered as f64 / (nodes as f64 * cycles as f64),
                mean: total_delay as f64 / delivered as f64,
                p50: percentile(&latencies, 0.50),
                p95: percentile(&latencies, 0.95),
                p99: percentile(&latencies, 0.99),
                max: latencies[latencies.len() - 1],
                mean_source_wait: total_wait as f64 / delivered as f64,
                mean_end_to_end: total_end_to_end as f64 / delivered as f64,
            });
        }
    }
    Err(format!(
        "simulation timed out after {limit} cycles for {} rate {rate:.6} (generated {}, injected {}, delivered {} of {}); partial CSV: {}",
        pattern.name(), meta.len(), injected, delivered, config.packets, path.display(),
    ).into())
}

pub fn run() -> Result<()> {
    let Some(config) = parse_args()? else { return Ok(()); };
    fs::create_dir_all(&config.output_dir)?;
    let summary_path = config.output_dir.join("summary.csv");
    let mut summary = BufWriter::new(File::create(&summary_path)?);
    writeln!(summary,
        "pattern,width,height,hotspot_x,hotspot_y,hotspot_prob,offered_rate,packets,seed,cycles,generation_cycles,accepted_rate_generation,throughput_per_node,mean_network_delay,p50_network_delay,p95_network_delay,p99_network_delay,max_network_delay,mean_source_wait,mean_end_to_end,packet_csv"
    )?;

    for pattern in &config.patterns {
        for &rate in &config.rates {
            let label = format!("{rate:.6}").replace('.', "p");
            let file_name = format!("{}_{}x{}_rate_{}_seed_{}.csv",
                pattern.name(), config.width, config.height, label, config.seed);
            let path = config.output_dir.join(&file_name);
            eprintln!("NoC: {} rate={rate:.4} mesh={}x{} packets={} ...",
                pattern.name(), config.width, config.height, config.packets);
            let result = evaluate(&config, *pattern, rate, &path)?;
            writeln!(summary, "{},{},{},{},{},{:.6},{:.6},{},{},{},{},{:.8},{:.8},{:.4},{},{},{},{},{:.4},{:.4},{}",
                pattern.name(), config.width, config.height,
                config.hotspot.0, config.hotspot.1, config.hotspot_prob,
                rate, config.packets, config.seed, result.cycles,
                result.generation_cycles, result.injection_rate_during_generation,
                result.throughput_per_node, result.mean,
                result.p50, result.p95, result.p99, result.max,
                result.mean_source_wait, result.mean_end_to_end, file_name,
            )?;
            summary.flush()?;
            eprintln!("  mean={:.2} p95={} throughput/node={:.4} cycles={} -> {}",
                result.mean, result.p95, result.throughput_per_node,
                result.cycles, path.display());
        }
    }
    eprintln!("NoC summary: {}", summary_path.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distinct_uniform_destinations_except_one_node_mesh() {
        let mut rng = SplitMix64::new(12);
        for n in [2, 3, 16, 30] {
            for source in 0..n {
                for _ in 0..100 {
                    let dest = random_other_node(&mut rng, source, n);
                    assert!(dest < n);
                    assert_ne!(source, dest);
                }
            }
        }
        assert_eq!(random_other_node(&mut rng, 0, 1), 0);
    }

    #[test]
    fn smoke_network_delivers_every_packet() {
        let path = std::env::temp_dir().join(format!(
            "ouros_noc_eval_smoke_{}.csv", std::process::id()
        ));
        let config = Config {
            width: 2, height: 2, packets: 32, rates: vec![0.5],
            patterns: vec![Pattern::Random], hotspot: (1, 1),
            hotspot_prob: 0.5, seed: 33, max_cycles: Some(10_000),
            output_dir: std::env::temp_dir(),
        };
        let stats = evaluate(&config, Pattern::Random, 0.5, &path).unwrap();
        assert!(stats.mean >= 1.0);
        let csv = std::fs::read_to_string(&path).unwrap();
        assert_eq!(csv.lines().count(), 33); // header + 32 deliveries
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn hotspot_probability_one_targets_hotspot_from_other_nodes() {
        let mut rng = SplitMix64::new(77);
        for source in 0..16 {
            if source != 5 {
                assert_eq!(choose_destination(&mut rng, source, 16, Pattern::Hotspot, 5, 1.0), 5);
            }
        }
    }
}
