mod hardware;
mod hw_module;

use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use std::cmp::max;
use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::Write;
use std::path::Path;
use std::sync::LazyLock;

use hardware::ouros::config::{BIG_HEAP, DLV_GC, GC_AT, HEAP_SIZE};
use hardware::ouros::ouros_core::OurosCore;
use hardware::ouros::program::{app_length, ActiveApp, App, Program};

use hardware::ouros::benchmarks::{self, *};
use hw_module::HwModule;

fn simulate(prog: &Program, detail_lv: u8, heap_size: usize, gc_at: f32) -> (OurosCore, u32) {
    let mut ouros = OurosCore::new(prog, detail_lv, heap_size, gc_at);
    let mut cycle: u32 = 0;

    ouros.tick();

    // kick start the machine
    ouros.input.start = true;
    ouros.tick();
    ouros.input.start = false;

    loop {
        assert!(cycle < 1_000_000);
        if ouros.done() || cycle == 1_000_000 {
            break;
        }
        ouros.tick();
        cycle += 1;
    }

    (ouros, cycle)
}

fn compress(oapp: &Option<ActiveApp>) -> String {
    match oapp {
        None => "empty".to_string(),
        Some(app) => {
            let app_str = app
                .load
                .iter()
                .take(app_length(&app.load))
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(", ");
            format!("{}-[{}]", app.stack_idx, app_str)
        }
    }
}

fn chunk_threads(threads: &Vec<(u8, u8)>, chunk_size: usize) -> Vec<(f32, f32)> {
    threads
        .chunks(chunk_size)
        .map(|chunk| {
            let occupied = chunk.iter().fold((0, 0), |(a, b), (c, d)| {
                (a as u32 + *c as u32, b as u32 + *d as u32)
            });
            (
                occupied.0 as f32 / chunk.len() as f32,
                occupied.1 as f32 / chunk.len() as f32,
            )
        })
        .collect()
}

fn chunk_rate(bpc: &Vec<bool>, chunk_size: usize) -> Vec<f32> {
    bpc.chunks(chunk_size)
        .map(|chunk| {
            let busy_count = chunk.iter().filter(|&&b| b).count();
            busy_count as f32 / chunk.len() as f32
        })
        .collect()
}

fn chunk_util(util: &Vec<u8>, chunk_size: usize) -> Vec<f32> {
    util.chunks(chunk_size)
        .map(|chunk| {
            let sum: u32 = chunk.iter().rfold(0, |a, b| a as u32 + *b as u32);
            sum as f32 / chunk.len() as f32
        })
        .collect()
}

fn write_threads() -> std::io::Result<()> {
    Ok(())
}

fn write_busy_rate(file: &mut File, data: &Vec<f32>, chunk_size: usize) -> std::io::Result<()> {
    writeln!(file, "time,rate")?;
    for (i, t) in data.iter().enumerate() {
        writeln!(file, "{},{:.2}", i * chunk_size + chunk_size / 2, t * 100.0)?;
    }
    Ok(())
}

fn percent_of(v: u32, total: u32) -> f32 {
    (v as f32 / total as f32) * 100.0
}

const DIR_SIMU_OUT: &str = "simu-out/";

/// inspect a program with full stat details
fn inspect_prog(prog: &Program) -> std::io::Result<()> {
    let log_path = Path::new(DIR_SIMU_OUT).join("log.txt");
    let threads_path = Path::new(DIR_SIMU_OUT).join("threads.csv");
    let red_rate_path = Path::new(DIR_SIMU_OUT).join("red-rate.csv");
    let alu_rate_path = Path::new(DIR_SIMU_OUT).join("alu-rate.csv");
    let gc_mreq_rate_path = Path::new(DIR_SIMU_OUT).join("gc-mutator-requst-rate.csv");
    let buffer_util_path = Path::new(DIR_SIMU_OUT).join("buffer-util.csv");
    let stm_dist_path = Path::new(DIR_SIMU_OUT).join("stm-dist.csv");

    fs::create_dir_all(DIR_SIMU_OUT)?;

    let mut log = File::create(log_path)?;
    let mut threads = File::create(threads_path)?;
    let mut red_rate = File::create(red_rate_path)?;
    let mut alu_rate = File::create(alu_rate_path)?;
    let mut gc_mreq_rate = File::create(gc_mreq_rate_path)?;
    let mut buffer_util = File::create(buffer_util_path)?;
    let mut stm_dist = File::create(stm_dist_path)?;

    let (ouros, runtime_cycles) = simulate(prog, u8::max_value(), HEAP_SIZE, GC_AT);
    let stats = ouros.get_stat();

    println!(
        "==== Simulation done! Cycles consumed: {} ====",
        runtime_cycles
    );

    // write log
    writeln!(log, "==================== SUMMARY =====================")?;
    writeln!(
        log,
        "     Simulation done! Cycles consumed: {}",
        runtime_cycles
    )?;
    writeln!(
        log,
        "       Reducer busy cycles: {} ({:.2}%)",
        stats.reducer_stat.busy_cycles,
        percent_of(stats.reducer_stat.busy_cycles, runtime_cycles)
    )?;
    writeln!(
        log,
        "         ALU busy cycles: {} ({:.2}%)",
        stats.alu_stat.busy_cycles,
        percent_of(stats.alu_stat.busy_cycles, runtime_cycles)
    )?;
    writeln!(
        log,
        "Heap memory accesses: {} (a_read {}, a_write {}, b_read {}, b_write {})",
        stats.mem_stat.a_reads
            + stats.mem_stat.a_writes
            + stats.mem_stat.b_reads
            + stats.mem_stat.b_writes,
        stats.mem_stat.a_reads,
        stats.mem_stat.a_writes,
        stats.mem_stat.b_reads,
        stats.mem_stat.b_writes,
    )?;
    writeln!(
        log,
        "heap allocations: {} | heap update: {} | avoided: {}",
        stats.gc_stat.allocations, stats.dheap_stat.heap_update, stats.dheap_stat.update_avoided
    )?;
    writeln!(log, "==================== GC STATS ====================")?;
    writeln!(
        log,
        "GC rounds: {} | 1-bit ref count recycle: {} | GC feedbacks: {} ({} shadowed)",
        stats.gc_stat.gc_rounds,
        stats.gc_stat.immediate_reuse,
        stats.gc_stat.feedbacks,
        stats.gc_stat.feedbacks_shadowed
    )?;
    writeln!(
        log,
        "GC stalls (Reducer): {} ({:.2}%, longest {}) | GC stalls (DHeap): {} ({:.2}%, longest {}) ",
        stats.reducer_stat.gc_stall_cycles,
        percent_of(stats.reducer_stat.gc_stall_cycles, runtime_cycles), stats.reducer_stat.gc_longest_stall,
        stats.dheap_stat.gc_stall_cycles,
        percent_of(stats.dheap_stat.gc_stall_cycles, runtime_cycles), stats.dheap_stat.gc_longest_stall
    )?;
    writeln!(
        log,
        "peak workset size: {} (heap size {:.2}x) | cycles on marking: {} ({:?})",
        stats.gc_stat.peak_workset_size,
        (HEAP_SIZE as f32) / (stats.gc_stat.peak_workset_size as f32),
        stats.gc_stat.mark_cycles,
        stats.gc_stat.mark_cycles_move
    )?;
    let gc_mark_reads = stats.gc_stat.cache_hit + stats.gc_stat.cache_miss;
    writeln!(
        log,
        "new apps with ptr: {} | new apps without ptr: {} | gc cache hit: {} ({:.2}%) miss: {} ({:.2}%)",
        stats.reducer_stat.nested_with_ptr,
        stats.reducer_stat.nested_no_ptr,
        stats.gc_stat.cache_hit, percent_of(stats.gc_stat.cache_hit, gc_mark_reads),
        stats.gc_stat.cache_miss, percent_of(stats.gc_stat.cache_miss, gc_mark_reads),
    )?;
    let jump_move_sum = stats.gc_stat.jump_move_01
        + stats.gc_stat.jump_move_10
        + stats.gc_stat.jump_move_11
        + stats.gc_stat.jump_move_12
        + stats.gc_stat.jump_move_22
        + stats.gc_stat.jump_move_20;
    writeln!(
        log,
        "mark moves | 01: {} ({:.2}%) | 10: {} ({:.2}%) | 11: {} ({:.2}%) | 12: {} ({:.2}%) | 22: {} ({:.2}%) | 20: {} ({:.2}%)",
        stats.gc_stat.jump_move_01, percent_of(stats.gc_stat.jump_move_01, jump_move_sum),
        stats.gc_stat.jump_move_10, percent_of(stats.gc_stat.jump_move_10, jump_move_sum),
        stats.gc_stat.jump_move_11, percent_of(stats.gc_stat.jump_move_11, jump_move_sum),
        stats.gc_stat.jump_move_12, percent_of(stats.gc_stat.jump_move_12, jump_move_sum),
        stats.gc_stat.jump_move_22, percent_of(stats.gc_stat.jump_move_22, jump_move_sum),
        stats.gc_stat.jump_move_20, percent_of(stats.gc_stat.jump_move_20, jump_move_sum),
    )?;
    writeln!(log, "============= REGISTER CONTENTS ==================")?;

    for (i, s) in stats
        .dheap_stat
        .holder_contents
        .iter()
        .zip(&stats.reducer_stat.holder_contents)
        .zip(&stats.alu_stat.holder_contents)
        .zip(&stats.dheap_stat.heap_stm)
        .zip(&stats.dheap_stat.serving_id)
        .map(|((((a, b), c), d), e)| (a, b, c, d, e))
        .enumerate()
    {
        writeln!(
            log,
            "{} dheap[{}-{}]: {} reducer: {} alu: {}",
            i,
            s.3,
            s.4,
            compress(s.0),
            compress(s.1),
            compress(s.2)
        )?;
    }

    let points_on_graph = 150;
    let chunk_size = stats.dheap_stat.work_threads.len() / points_on_graph;
    // write thread stats
    writeln!(threads, "time,occupied,active")?;
    let threads_data = chunk_threads(&stats.dheap_stat.work_threads, chunk_size);
    for (i, t) in threads_data.iter().enumerate() {
        writeln!(
            threads,
            "{},{},{}",
            i * chunk_size + chunk_size / 2,
            t.0,
            t.1
        )?;
    }

    // write busy rate
    let red_rate_data: Vec<f32> = chunk_rate(&stats.reducer_stat.busy_per_cycle, chunk_size);
    write_busy_rate(&mut red_rate, &red_rate_data, chunk_size)?;

    let alu_rate_data: Vec<f32> = chunk_rate(&stats.alu_stat.busy_per_cycle, chunk_size);
    write_busy_rate(&mut alu_rate, &alu_rate_data, chunk_size)?;

    let gc_mreq_rate_data: Vec<f32> = chunk_rate(&stats.gc_stat.m_request_per_cycle, chunk_size);
    write_busy_rate(&mut gc_mreq_rate, &gc_mreq_rate_data, chunk_size)?;

    // write buffer utilisation
    writeln!(buffer_util, "time,alu_0,alu_1,alu_2,dheap_a_0,dheap_a_1,dheap_a_2,dheap_a_3,dheap_b,,reducer_0,reducer_1,reducer_2,reducer_3")?;
    let buffer_util_data = stats
        .fifos_stat
        .map(|s| chunk_util(&s.length_per_cycle, chunk_size));
    for i in 0..buffer_util_data[0].len() {
        writeln!(
            buffer_util,
            "{},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2}",
            i * chunk_size + chunk_size / 2,
            buffer_util_data[0][i],
            buffer_util_data[1][i],
            buffer_util_data[2][i],
            buffer_util_data[3][i],
            buffer_util_data[4][i],
            buffer_util_data[5][i],
            buffer_util_data[6][i],
            buffer_util_data[7][i],
            buffer_util_data[8][i],
            buffer_util_data[9][i],
            buffer_util_data[10][i],
            buffer_util_data[11][i],
        )?;
    }

    // write stm distributioin
    writeln!(stm_dist, "state,cycles")?;
    for (s, c) in ["IDLE", "WHNF", "IA", "RESUME"]
        .iter()
        .zip(stats.dheap_stat.stm_cycles)
    {
        writeln!(stm_dist, "{},{}", s, c)?;
    }

    Ok(())
}

macro_rules! benchmarks {
    ($($name:ident),* $(,)?) => {{
        let mut map = HashMap::new();
        $(
            map.insert(stringify!($name), & $name);
        )*
        map
    }};
}

/// run the benchmark suite with less stat details
fn run_benchmarks(progs: HashMap<&str, &LazyLock<Program>>) -> std::io::Result<()> {
    let mut vec: Vec<(&str, &LazyLock<Program>)> = progs.into_iter().collect();
    vec.sort_by_key(|(n, _)| *n);
    let (names, benchmarks): (Vec<&str>, Vec<&LazyLock<Program>>) = vec.into_iter().unzip();
    let results = benchmarks.iter().map(|p| {
        let res = simulate(&p, 0, HEAP_SIZE, GC_AT);
        res
    });

    results
        .zip(names)
        .for_each(|((_, cycles), n)| println!("{:<12} {:>8} cycles", n, cycles));

    Ok(())
}

const DIR_SIMU_OUT_GC: &str = "simu-out/gc/";

fn vec_to_string<T: std::fmt::Display>(vec: &Vec<T>) -> String {
    vec.iter()
        .map(|item| item.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

/// evaluate the GC behabiour of the benchmarks
fn eval_gc(progs: HashMap<&str, &LazyLock<Program>>) -> std::io::Result<()> {
    let gc_percent_path = Path::new(DIR_SIMU_OUT_GC).join("gc_percent.csv");
    let max_pause_path = Path::new(DIR_SIMU_OUT_GC).join("max_pause.csv");
    let heap_peak_path = Path::new(DIR_SIMU_OUT_GC).join("heap_peak.csv");
    let peak_workset_path = Path::new(DIR_SIMU_OUT_GC).join("peak_workset.csv");

    fs::create_dir_all(DIR_SIMU_OUT_GC)?;

    let mut gc_percent_file = File::create(gc_percent_path)?;
    let mut max_pause_file = File::create(max_pause_path)?;
    let mut heap_peak_file = File::create(heap_peak_path)?;
    let mut peak_workset_file = File::create(peak_workset_path)?;

    let mut vec: Vec<(&str, &LazyLock<Program>)> = progs.into_iter().collect();
    vec.sort_by_key(|(n, _)| *n);
    let (names, benchmarks): (Vec<&str>, Vec<&LazyLock<Program>>) = vec.into_iter().unzip();
    let results: Vec<_> = benchmarks
        .par_iter()
        .progress_count(benchmarks.len() as u64)
        .map(|p| {
            // run two tests to get gc free runtime and an approximate peak work set size
            let (_, gc_free_runtime) = simulate(&p, 0, BIG_HEAP, 0.0);
            // println!("GC FREE RUNTIME: {}", gc_free_runtime);
            let (c, _) = simulate(&p, DLV_GC, HEAP_SIZE, GC_AT);
            let peak_workset = c.get_stat().gc_stat.peak_workset_size;
            // run several more rounds with different heap size
            let points = [1.5, 2.0, 3.0, 5.0, 10.0];
            let res = points.map(|pt| {
                // println!(
                //     "PEAK: {}; HEAP SIZE: {}",
                //     peak_workset,
                //     (peak_workset as f32 * pt) as usize
                // );
                simulate(&p, DLV_GC, (peak_workset as f32 * pt) as usize, GC_AT)
            });
            let res_gc_percent: Vec<f32> = res
                .iter()
                .map(|(_, time)| percent_of(time - gc_free_runtime, *time))
                .collect();
            let res_max_pause: Vec<u32> = res
                .iter()
                .map(|(core, _)| {
                    max(
                        core.get_stat().reducer_stat.gc_longest_stall,
                        core.get_stat().dheap_stat.gc_longest_stall,
                    )
                })
                .collect();
            let res_points: Vec<f32> = res
                .iter()
                .map(|(core, _)| {
                    core.heap_size as f32 / (core.get_stat().gc_stat.peak_workset_size as f32)
                })
                .collect();
            (res_gc_percent, res_max_pause, res_points, peak_workset)
        })
        .collect();

    results
        .into_iter()
        .zip(names)
        .for_each(|((percent, max_pause, points, peak), n)| {
            println!(
                "{:<10} | Peak work set {} | GC% {:?} | Max pause {:?} | Heap size / Peak work set {:?}",
                n, peak, percent, max_pause, points
            );
            writeln!(gc_percent_file, "{},{}", n, vec_to_string(&percent)).unwrap();
            writeln!(max_pause_file, "{},{}", n, vec_to_string(&max_pause)).unwrap();
            writeln!(heap_peak_file, "{},{}", n, vec_to_string(&points)).unwrap();
            writeln!(peak_workset_file, "{},{}", n, peak).unwrap();
        });

    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let progs = benchmarks!(
        ADJOXO, BRAUN, CLAUSIFY, COUNTDOWN, FIB, MSS, ORDLIST, PERMSORT, QUEENS, QUEENS2,
        SKIABSEVAL, SUMEULER, SUMPUZ, TAUT, TREEPARI, /*TREESUM,*/ TRIBELIE, WHILEX,
    ); // ignoring TREESUM as it does not have much garbage..

    if args.len() == 1 {
        println!("usage: cargo run --release @ALL/@GC/<prog>");
        Ok(())
    } else {
        match args[1].as_str() {
            "@ALL" => run_benchmarks(progs),
            "@GC" => eval_gc(progs),
            prog => inspect_prog(progs.get(prog).unwrap()),
        }
    }
}
