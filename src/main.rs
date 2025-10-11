mod hardware;
mod hw_module;

use std::fs::{self, File};
use std::io::prelude::*;
use std::io::{self, Write};
use std::path::Path;

use hardware::ouros::ouros_core::OurosCore;
use hardware::ouros::program::{app_length, ActiveApp, App, Program};
// use hardware::ouros::deref_heap_new::
use hardware::ouros::benchmarks::*;
use hw_module::HwModule;

fn simulate(prog: &Program, detail_lv: u8) -> (OurosCore, u32) {
    let mut ouros = OurosCore::new(prog, detail_lv);
    let mut cycle: u32 = 0;

    ouros.tick();

    // kick start the machine
    ouros.input.start = true;
    ouros.tick();
    ouros.input.start = false;

    loop {
        assert!(cycle < 1_000_000);
        if ouros.done() || cycle == 1_000_00 {
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

const DIR: &str = "simu-out/";

/// inspect a program with full stat details
fn inspect_prog(prog: &Program) -> std::io::Result<()> {
    let log_path = Path::new(DIR).join("log.txt");
    let threads_path = Path::new(DIR).join("threads.csv");
    let red_rate_path = Path::new(DIR).join("red-rate.csv");
    let alu_rate_path = Path::new(DIR).join("alu-rate.csv");
    let buffer_util_path = Path::new(DIR).join("buffer-util.csv");
    let stm_dist_path = Path::new(DIR).join("stm-dist.csv");

    fs::create_dir_all(DIR)?;

    let mut log = File::create(log_path)?;
    let mut threads = File::create(threads_path)?;
    let mut red_rate = File::create(red_rate_path)?;
    let mut alu_rate = File::create(alu_rate_path)?;
    let mut buffer_util = File::create(buffer_util_path)?;
    let mut stm_dist = File::create(stm_dist_path)?;

    let (ouros, runtime_cycles) = simulate(prog, u8::max_value());
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
        "       Reducer busy cycles: {} ({:.2}%), blocked cycles: {}",
        stats.1.busy_cycles,
        (stats.1.busy_cycles as f32) / (runtime_cycles as f32) * 100.0,
        stats.1.blocked_cycles
    )?;
    writeln!(
        log,
        "         ALU busy cycles: {} ({:.2}%)",
        stats.2.busy_cycles,
        (stats.2.busy_cycles as f32) / (runtime_cycles as f32) * 100.0
    )?;
    writeln!(
        log,
        "Heap memory accesses: {} (a_read {}, a_write {}, b_read {}, b_write {})",
        stats.4.a_reads + stats.4.a_writes + stats.4.b_reads + stats.4.b_writes,
        stats.4.a_reads,
        stats.4.a_writes,
        stats.4.b_reads,
        stats.4.b_writes,
    )?;
    writeln!(
        log,
        "heap cell consumed: {}, heap update: {}, avoided: {}",
        ouros.dheap.addr_bumper.value(),
        stats.0.heap_update,
        stats.0.update_avoided
    );
    writeln!(log, "============= REGISTER CONTENTS ==================")?;

    for (i, s) in stats
        .0
        .holder_contents
        .iter()
        .zip(&stats.1.holder_contents)
        .zip(&stats.2.holder_contents)
        .zip(&stats.0.heap_stm)
        .zip(&stats.0.serving_id)
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
    let chunk_size = stats.0.work_threads.len() / points_on_graph;
    // write thread stats
    writeln!(threads, "time,occupied,active")?;
    let threads_data = chunk_threads(&stats.0.work_threads, chunk_size);
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
    let red_rate_data: Vec<f32> = chunk_rate(&stats.1.busy_per_cycle, chunk_size);
    write_busy_rate(&mut red_rate, &red_rate_data, chunk_size)?;

    let alu_rate_data: Vec<f32> = chunk_rate(&stats.2.busy_per_cycle, chunk_size);
    write_busy_rate(&mut alu_rate, &alu_rate_data, chunk_size)?;

    // write buffer utilisation
    writeln!(buffer_util, "time,alu_0,alu_1,alu_2,dheap_a_0,dheap_a_1,dheap_a_2,dheap_a_3,dheap_b,,reducer_0,reducer_1,reducer_2,reducer_3")?;
    let buffer_util_data = stats.3.map(|s| chunk_util(&s.length_per_cycle, chunk_size));
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
        .zip(stats.0.stm_cycles)
    {
        writeln!(stm_dist, "{},{}", s, c)?;
    }

    Ok(())
}

macro_rules! benchmarks {
    ($($name:ident),* $(,)?) => {
        {
            let benchmarks = [$(& $name),*];
            let names = [$(stringify!($name)),*];
            (benchmarks, names)
        }
    };
}

/// run the benchmark suite with less stat details
fn run_benchmarks() -> std::io::Result<()> {
    let (benchmarks, names) = benchmarks!(
        BOOL_AND // ADJOXO, BRAUN, CLAUSIFY, COUNTDOWN, FIB, MSS, ORDLIST, PERMSORT, QUEENS, QUEENS2,
                 // SKIABSEVAL, SUMEULER, SUMPUZ, TAUT, TRIBELIE, TREEPARI, TREESUM, WHILEX,
    );
    let mut counter = 0;
    let results = benchmarks.map(|p| {
        let res = simulate(&p, 100);
        counter += 1;
        print!("\r{}/{} finished.", counter, benchmarks.len());
        io::stdout().flush().unwrap();
        res
    });
    println!();
    results
        .iter()
        .zip(names)
        .for_each(|((core, cycles), n)| println!("{:<12} {:>8} cycles", n, cycles));
    Ok(())
}

fn main() -> std::io::Result<()> {
    inspect_prog(&FIB)
    // run_benchmarks()
}
