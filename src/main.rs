mod hardware;
mod hw_module;

use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;

use hardware::ouros::ouros_core::OurosCore;
use hardware::ouros::program::{app_length, App};
use hw_module::HwModule;

fn simulate() -> (OurosCore, u32) {
    use hardware::ouros::benchmarks::*;
    let mut ouros = OurosCore::new(&FIB);
    let mut cycle: u32 = 0;

    ouros.tick();

    // kick start the machine
    ouros.input.start = true;
    ouros.tick();
    ouros.input.start = false;

    loop {
        assert!(cycle < 1_000_000);
        if ouros.done() || cycle == 4000 {
            break;
        }
        ouros.tick();
        cycle += 1;
    }

    (ouros, cycle)
}

fn compress(oapp: &Option<App>) -> String {
    match oapp {
        None => "empty".to_string(),
        Some(app) => {
            let app_str = app
                .iter()
                .take(app_length(app))
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(", ");
            format!("[{}]", app_str)
        }
    }
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

fn write_busy_rate(file: &mut File, data: &Vec<f32>, chunk_size: usize) -> std::io::Result<()> {
    writeln!(file, "time,rate")?;
    for (i, t) in data.iter().enumerate() {
        writeln!(file, "{},{:.2}", i * chunk_size + chunk_size / 2, t * 100.0)?;
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let dir = "simu-out/";
    let log_path = Path::new(dir).join("log.txt");
    let threads_path = Path::new(dir).join("threads.csv");
    let red_rate_path = Path::new(dir).join("red-rate.csv");
    let alu_rate_path = Path::new(dir).join("alu-rate.csv");
    let buffer_util_path = Path::new(dir).join("buffer-util.csv");

    fs::create_dir_all(dir)?;

    let mut log = File::create(log_path)?;
    let mut threads = File::create(threads_path)?;
    let mut red_rate = File::create(red_rate_path)?;
    let mut alu_rate = File::create(alu_rate_path)?;
    let mut buffer_util = File::create(buffer_util_path)?;

    let (ouros, runtime_cycles) = simulate();
    let stats = ouros.get_stat();

    println!(
        "==== Simulation done! Cycles consumed: {} (wsated {}) ====",
        runtime_cycles, stats.0.wasted_cycles
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
        stats.1.busy_cycles,
        (stats.1.busy_cycles as f32) / (runtime_cycles as f32) * 100.0
    )?;
    writeln!(
        log,
        "         ALU busy cycles: {} ({:.2}%)",
        stats.2.busy_cycles,
        (stats.2.busy_cycles as f32) / (runtime_cycles as f32) * 100.0
    )?;
    writeln!(log, "============= REGISTER CONTENTS ==================")?;

    for (i, s) in stats
        .0
        .holder_contents
        .iter()
        .zip(&stats.1.holder_contents)
        .zip(&stats.2.holder_contents)
        .map(|((a, b), c)| (a, b, c))
        .enumerate()
    {
        writeln!(
            log,
            "{} dheap: {} reducer: {} alu: {}",
            i,
            compress(s.0),
            compress(s.1),
            compress(s.2)
        )?;
    }

    // write thread stats
    writeln!(threads, "time,threads")?;
    for (i, t) in stats.0.work_threads.iter().enumerate() {
        writeln!(threads, "{},{}", i, t)?;
    }

    // write busy rate
    let chunk_size = 50;

    let red_rate_data: Vec<f32> = chunk_rate(&stats.1.busy_per_cycle, chunk_size);
    write_busy_rate(&mut red_rate, &red_rate_data, chunk_size)?;

    let alu_rate_data: Vec<f32> = chunk_rate(&stats.2.busy_per_cycle, chunk_size);
    write_busy_rate(&mut alu_rate, &alu_rate_data, chunk_size)?;

    // write buffer utilisation
    writeln!(buffer_util, "time,alu_0,alu_1,dheap_a_0,dheap_a_1,dheap_a_2,dheap_b_0,dheap_b_1,dheap_b_2,reducer_0,reducer_1,reducer_2")?;
    let buffer_util_data = stats.3.map(|s| chunk_util(&s.length_per_cycle, chunk_size));
    for i in 0..buffer_util_data[0].len() {
        writeln!(
            buffer_util,
            "{},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2}",
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
        )?;
    }

    Ok(())
}
