// Top level module of the Ouros core.

use std::cmp::max;

use crate::hardware::common::fifo::FIFOStat;
use crate::hardware::common::memory::DualPortMemStat;
use crate::hardware::common::{Fifo, PArbiter, RArbiter, Ring};
use crate::hw_module::{HwInput, HwModule};

use super::addr_box::AddrBox;
use super::alu::{Alu, AluStat};
use super::config::*;
use super::deref_heap_new::{DrfHeap, DrfHeapStat};
use super::garbage_collector::{GbgCollector, GbgCollectorStat};
use super::program::*;
use super::reducer::{Reducer, ReducerStat};

enum DESTs {
    ToDHeap,
    ToALU,
    ToReducer,
    ToIO,
}

// fn is_lit_seq(app: &App) -> bool {
//     match app[0] {
//         Atom::Seq(true) => is_lit_atom(&app[2]),
//         _ => false,
//     }
// }

fn get_dests(app: &App) -> DESTs {
    if io_is_ready(app) {
        DESTs::ToIO
    } else if is_prm(&app[0]) && is_int(&app[1]) && is_int(&app[2]) {
        DESTs::ToALU
    } else if !is_whnf(app) && (is_comb(&app[0]) || is_con(&app[0])) {
        DESTs::ToReducer
    } else {
        // NOTE [Seq, 1, x] also goes to DHeap...
        DESTs::ToDHeap
    }
}

#[derive(Default)]
pub struct OurosCoreInput {
    pub start: bool,
    /// Boundary to the future IO unit. A request is consumed only on
    /// io_request_valid() && io_request_ready.
    pub io_request_ready: bool,
    /// The IO unit must return the completed Pair with the original stack_idx.
    pub io_response_valid: bool,
    pub io_response_bits: ActiveApp,
}

impl HwInput for OurosCoreInput {}

// #[derive(Default)]
pub struct OurosCoreStat<'a> {
    pub dheap_stat: &'a DrfHeapStat,
    pub reducer_stat: &'a ReducerStat,
    pub alu_stat: &'a AluStat,
    pub fifos_stat: [&'a FIFOStat; 16],
    pub mem_stat: &'a DualPortMemStat,
    pub gc_stat: &'a GbgCollectorStat,
    pub peak_workset_size: usize,
}

pub struct OurosCore {
    pub heap_size: usize,
    cycle_ctr: u32,
    peak_workset_size: usize,
    free_from: usize,
    pub input: OurosCoreInput,
    pub dheap: DrfHeap,
    gc: GbgCollector,
    abox: AddrBox,
    reducer: Reducer,
    alu: Alu,

    buffers_dealloc: Fifo<usize, 2, false>,
    buffers_free_addr: Fifo<usize, 2, false>,
    buffers_feedback: Fifo<usize, 2, false>,

    buffers_dheap_a_0: Fifo<ActiveApp, BUFFER_SIZE, false>,
    buffers_dheap_a_1: Fifo<ActiveApp, BUFFER_SIZE, false>,
    buffers_dheap_a_2: Fifo<ActiveApp, BUFFER_SIZE, false>,
    buffers_dheap_a_3: Fifo<ActiveApp, BUFFER_SIZE, false>,
    buffers_dheap_a_4: Fifo<ActiveApp, BUFFER_SIZE, false>, // completed IO
    arbiter_dheap_a: PArbiter<ActiveApp, 5>,

    buffers_io_0: Fifo<ActiveApp, BUFFER_SIZE, false>, // DHeap normal output
    buffers_io_1: Fifo<ActiveApp, BUFFER_SIZE, false>, // DHeap ready request
    arbiter_io: PArbiter<ActiveApp, 2>,

    buffers_dheap_b_0: Fifo<FrozenApp, BUFFER_SIZE, false>,
    buffers_dheap_b_1: Fifo<FrozenApp, BUFFER_SIZE, false>,
    arbiter_dheap_b: PArbiter<FrozenApp, 2>,

    rings_dheap_b_0: Ring<usize, BUFFER_SIZE>,
    rings_dheap_b_1: Ring<usize, BUFFER_SIZE>,

    buffers_reducer_0: Fifo<ActiveApp, BUFFER_SIZE, false>,
    buffers_reducer_1: Fifo<ActiveApp, BUFFER_SIZE, false>,
    buffers_reducer_2: Fifo<ActiveApp, BUFFER_SIZE, false>,
    buffers_reducer_3: Fifo<ActiveApp, BUFFER_SIZE, false>,
    arbiter_reducer: PArbiter<ActiveApp, 4>,

    buffers_alu_0: Fifo<ActiveApp, BUFFER_SIZE, false>,
    buffers_alu_1: Fifo<ActiveApp, BUFFER_SIZE, false>,
    buffers_alu_2: Fifo<ActiveApp, BUFFER_SIZE, false>,
    arbiter_alu: PArbiter<ActiveApp, 3>,
}

impl OurosCore {
    pub fn new(prog: &Program, detail_lv: u8, heap_size: usize, gc_at: f32) -> Self {
        let buffer_usage: bool = detail_lv >= DLV_BUFFER_USAGE;
        Self {
            heap_size,
            cycle_ctr: 0,
            peak_workset_size: 0,
            free_from: prog.heap_img.len(),
            input: Default::default(),
            dheap: DrfHeap::new(heap_size)
                .program(&prog.heap_img)
                .detail(detail_lv),
            gc: GbgCollector::new(heap_size, prog.heap_img.len(), gc_at)
                .init_freelist()
                .detail(detail_lv),
            abox: AddrBox::new(),
            reducer: Reducer::new(PROG_SIZE)
                .program(&prog.comb_img)
                .detail(detail_lv),
            alu: Alu::new().detail(detail_lv),

            buffers_dealloc: Fifo::new(),
            buffers_free_addr: Fifo::new(),
            buffers_feedback: Fifo::new(),

            buffers_dheap_a_0: Fifo::new().record_stat(buffer_usage),
            buffers_dheap_a_1: Fifo::new().record_stat(buffer_usage),
            buffers_dheap_a_2: Fifo::new().record_stat(buffer_usage),
            buffers_dheap_a_3: Fifo::new().record_stat(buffer_usage),
            buffers_dheap_a_4: Fifo::new().record_stat(buffer_usage),
            arbiter_dheap_a: PArbiter::new(),

            buffers_io_0: Fifo::new().record_stat(buffer_usage),
            buffers_io_1: Fifo::new().record_stat(buffer_usage),
            arbiter_io: PArbiter::new(),

            buffers_dheap_b_0: Fifo::new().record_stat(buffer_usage),
            buffers_dheap_b_1: Fifo::new().record_stat(buffer_usage),
            arbiter_dheap_b: PArbiter::new(),

            rings_dheap_b_0: Ring::new(),
            rings_dheap_b_1: Ring::new(),

            buffers_reducer_0: Fifo::new().record_stat(buffer_usage),
            buffers_reducer_1: Fifo::new().record_stat(buffer_usage),
            buffers_reducer_2: Fifo::new().record_stat(buffer_usage),
            buffers_reducer_3: Fifo::new().record_stat(buffer_usage),
            arbiter_reducer: PArbiter::new(),

            buffers_alu_0: Fifo::new().record_stat(buffer_usage),
            buffers_alu_1: Fifo::new().record_stat(buffer_usage),
            buffers_alu_2: Fifo::new().record_stat(buffer_usage),
            arbiter_alu: PArbiter::new(),
        }
    }

    /// Ready/valid interface to the not-yet-implemented IO unit.
    /// Unlike a Reducer-to-IO bypass, every request has passed DHeap.
    pub fn io_request_valid(&self) -> bool {
        self.buffers_io_0.out_valid() || self.buffers_io_1.out_valid()
    }

    pub fn io_request_bits(&self) -> Option<&ActiveApp> {
        // Observe the current FIFO state, not the arbiter's combinational
        // inputs captured during the previous tick. Same priority as arbiter_io.
        self.buffers_io_0.dout().or_else(|| self.buffers_io_1.dout())
    }

    pub fn io_response_ready(&self) -> bool {
        self.buffers_dheap_a_4.in_ready()
    }

    pub fn done(&self) -> bool {
        self.dheap.done()
    }

    pub fn get_stat(&self) -> OurosCoreStat {
        OurosCoreStat {
            dheap_stat: self.dheap.get_stat(),
            reducer_stat: self.reducer.get_stat(),
            alu_stat: self.alu.get_stat(),
            fifos_stat: [
                self.buffers_alu_0.get_stat(),
                self.buffers_alu_1.get_stat(),
                self.buffers_alu_2.get_stat(),
                self.buffers_dheap_a_0.get_stat(),
                self.buffers_dheap_a_1.get_stat(),
                self.buffers_dheap_a_2.get_stat(),
                self.buffers_dheap_a_3.get_stat(),
                self.buffers_dheap_a_4.get_stat(),
                self.buffers_io_0.get_stat(),
                self.buffers_io_1.get_stat(),
                self.buffers_dheap_b_0.get_stat(),
                self.buffers_dheap_b_1.get_stat(),
                self.buffers_reducer_0.get_stat(),
                self.buffers_reducer_1.get_stat(),
                self.buffers_reducer_2.get_stat(),
                self.buffers_reducer_3.get_stat(),
            ],
            mem_stat: self.dheap.get_mem_stat(),
            gc_stat: self.gc.get_stat(),
            peak_workset_size: self.peak_workset_size,
        }
    }
}

fn assign_some<T: Clone>(sink: &mut T, source: Option<&T>) {
    match source {
        None => {}
        Some(v) => {
            *sink = v.clone();
        }
    }
}

/// Connect input buffers of the arbiter
fn buffers_arbiter<T: Clone + Default, const N: usize, const A: usize, const P: bool>(
    buffers: [&mut Fifo<T, N, P>; A],
    arbiter: &mut PArbiter<T, A>,
) {
    // first handle arbiter's inputs
    arbiter.input.link(|input| {
        for (i, b) in buffers.iter().enumerate() {
            assign_some(&mut input.in_bits[i], b.dout());
            input.in_valid[i] = b.out_valid();
        }
    });
    let select = arbiter.select();
    for (i, b) in buffers.into_iter().enumerate() {
        b.input.out_ready = arbiter.in_ready(i, select);
    }
}

impl HwModule for OurosCore {
    fn update_local(&mut self) -> Result<(), String> {
        let _span = tracy_client::span!("ouros: update_local");
        /*
        NOTE: there is an assignment ring in the circuit, the order of
        assigning inputs matters and might be buggy here.
        Keep this in mind if anything strange occurs in the future..
         */
        // set default inputs (for conditionally linked ports)
        self.buffers_dheap_a_0.input.default_input();
        self.buffers_dheap_a_1.input.default_input();
        self.buffers_dheap_a_2.input.default_input();
        self.buffers_dheap_a_3.input.default_input();
        self.buffers_dheap_a_4.input.default_input();
        self.buffers_io_0.input.default_input();
        self.buffers_io_1.input.default_input();
        self.buffers_reducer_0.input.default_input();
        self.buffers_reducer_1.input.default_input();
        self.buffers_reducer_2.input.default_input();
        self.buffers_reducer_3.input.default_input();
        self.buffers_alu_0.input.default_input();
        self.buffers_alu_1.input.default_input();
        self.buffers_alu_2.input.default_input();

        // connect start signal
        self.dheap.input.start = self.input.start;
        self.buffers_dheap_a_4.input.in_valid = self.input.io_response_valid;
        self.buffers_dheap_a_4.input.din = self.input.io_response_bits.clone();

        // this 'kind of' fixes the ring problem
        for _ in 0..2 {
            // gc control signals
            self.buffers_dealloc.input.din = self.dheap.dealloc_bits();
            self.buffers_dealloc.input.in_valid = self.dheap.dealloc_valid();
            self.buffers_free_addr.input.din = self.gc.addr_out_bits();
            self.buffers_free_addr.input.in_valid = self.gc.addr_out_valid();
            self.buffers_feedback.input.din = self.abox.feedback_bits();
            self.buffers_feedback.input.in_valid = self.abox.feedback_valid();

            self.gc.input.deallocate_bits = *self.buffers_dealloc.dout().unwrap_or(&0);
            self.gc.input.deallocate_valid = self.buffers_dealloc.out_valid();
            self.gc.input.addr_out_ready = self.buffers_free_addr.in_ready();

            self.gc.input.feedback_bits = *self.buffers_feedback.dout().unwrap_or(&0);
            self.gc.input.feedback_valid = self.buffers_feedback.out_valid();

            self.abox.input.free_addr_bits = *self.buffers_free_addr.dout().unwrap_or(&0);
            self.abox.input.free_addr_valid = self.buffers_free_addr.out_valid();
            self.abox.input.addr_consume[CONSUMERS - 1] = self.dheap.free_addr_req();
            self.abox.input.addr_consume[0..CONSUMERS - 1]
                .copy_from_slice(&self.reducer.consume_demands());
            self.abox.input.dheap_feedback_valid = self.dheap.need_split();
            self.abox.input.dheap_feedback_bits = self.dheap.free_addr_feedback();
            let free_addr_bits = self.abox.consume_addr_bits();
            let free_addr_valid = self.abox.consume_addr_valid();
            self.dheap.input.free_addr = free_addr_bits[CONSUMERS - 1];
            self.dheap.input.free_addr_valid = free_addr_valid[CONSUMERS - 1];
            self.reducer
                .input
                .free_addrs
                .copy_from_slice(&free_addr_bits[0..CONSUMERS - 1]);
            self.reducer
                .input
                .free_addrs_valid
                .copy_from_slice(&free_addr_valid[0..CONSUMERS - 1]);

            self.buffers_dealloc.input.out_ready = self.gc.deallocate_ready();
            self.buffers_free_addr.input.out_ready = self.abox.addr_request();
            self.buffers_feedback.input.out_ready = self.gc.feedback_ready();

            self.gc.input.heap_read_valid = self.dheap.heap_read_valid();
            self.gc.input.heap_read_bits = *self.dheap.heap_read_bits();
            self.dheap.input.read_heap_req_valid = self.gc.read_heap_req_valid();
            self.dheap.input.read_heap_req_addr = self.gc.read_heap_req_addr();

            self.gc.input.monitor_valid = self.reducer.spine_valid();
            self.gc.input.monitor_bits = self.reducer.spine_bits();
            self.gc.input.monitor_big_drf_valid = self.dheap.out_big_drf_valid();
            self.gc.input.monitor_big_drf_stk = self.dheap.out_main_bits()?.stack_idx as usize;
            self.gc.input.monitor_big_drf_bits = self.dheap.out_main_bits()?.load[0];
            self.gc.input.monitor_unset_valid = self.dheap.port_a_fire();
            self.gc.input.monitor_unset = self.dheap.input.port_a_bits.stack_idx as usize;

            // connect arbiters as components' input (arbiter first)
            self.arbiter_dheap_a.input.out_ready = self.dheap.port_a_ready();
            self.arbiter_io.input.out_ready = self.input.io_request_ready;
            self.arbiter_dheap_b.input.out_ready = self.dheap.port_b_ready();
            self.arbiter_reducer.input.out_ready = self.reducer.in_ready();
            self.arbiter_alu.input.out_ready = self.alu.input_ready();

            // connect buffers to arbiters
            buffers_arbiter(
                [
                    &mut self.buffers_dheap_a_0,
                    &mut self.buffers_dheap_a_1,
                    &mut self.buffers_dheap_a_2,
                    &mut self.buffers_dheap_a_3,
                    &mut self.buffers_dheap_a_4,
                ],
                &mut self.arbiter_dheap_a,
            );
            buffers_arbiter(
                [&mut self.buffers_io_0, &mut self.buffers_io_1],
                &mut self.arbiter_io,
            );
            buffers_arbiter(
                [&mut self.buffers_dheap_b_0, &mut self.buffers_dheap_b_1],
                &mut self.arbiter_dheap_b,
            );
            buffers_arbiter(
                [
                    &mut self.buffers_reducer_0,
                    &mut self.buffers_reducer_1,
                    &mut self.buffers_reducer_2,
                    &mut self.buffers_reducer_3,
                ],
                &mut self.arbiter_reducer,
            );
            buffers_arbiter(
                [
                    &mut self.buffers_alu_0,
                    &mut self.buffers_alu_1,
                    &mut self.buffers_alu_2,
                ],
                &mut self.arbiter_alu,
            );

            // connect arbiters as components' input
            self.dheap.input.link(|input| {
                input.port_a_valid = self.arbiter_dheap_a.out_valid();
                match self.arbiter_dheap_a.out_bits(self.arbiter_dheap_a.select()) {
                    None => {}
                    Some(v) => {
                        input.port_a_bits = v.clone();
                    }
                }
                input.port_b_valid = self.arbiter_dheap_b.out_valid();
                match self.arbiter_dheap_b.out_bits(self.arbiter_dheap_b.select()) {
                    None => {}
                    Some(v) => {
                        input.port_b_bits = v.clone();
                    }
                }
            });

            self.reducer.input.link(|input| {
                input.in_valid = self.arbiter_reducer.out_valid();
                match self.arbiter_reducer.out_bits(self.arbiter_reducer.select()) {
                    None => {}
                    Some(v) => {
                        input.in_app = v.clone();
                    }
                }
            });
            self.alu.input.link(|input| {
                input.input_valid = self.arbiter_alu.out_valid();
                match self.arbiter_alu.out_bits(self.arbiter_alu.select()) {
                    None => {}
                    Some(v) => {
                        input.input_bits = v.clone();
                    }
                }
            });

            // Saturated requests leave DHeap through one of two output paths.
            // Buffering decouples strict argument processing from the IO unit.
            self.buffers_io_1.input.in_valid = self.dheap.out_io_valid();
            self.buffers_io_1.input.din = self.dheap.out_io_bits().clone();
            self.dheap.input.out_io_ready = self.buffers_io_1.in_ready();

            if self.dheap.out_main_valid() {
                let dest = if self.dheap.out_main_from_startup() {
                    DESTs::ToDHeap
                } else {
                    get_dests(&self.dheap.out_main_bits()?.load)
                };
                match dest {
                    DESTs::ToIO => {
                        self.buffers_io_0.input.in_valid = true;
                        self.buffers_io_0.input.din = self.dheap.out_main_bits()?.clone();
                        self.dheap.input.out_main_ready = self.buffers_io_0.in_ready();
                    }
                    DESTs::ToDHeap => {
                        self.buffers_dheap_a_0.input.in_valid = true;
                        self.buffers_dheap_a_0.input.din = self.dheap.out_main_bits()?.clone();
                        self.dheap.input.out_main_ready = self.buffers_dheap_a_0.in_ready();
                    }
                    DESTs::ToALU => {
                        self.buffers_alu_1.input.in_valid = true;
                        self.buffers_alu_1.input.din = self.dheap.out_main_bits()?.clone();
                        self.dheap.input.out_main_ready = self.buffers_alu_1.in_ready();
                    }
                    DESTs::ToReducer => {
                        self.buffers_reducer_1.input.in_valid = true;
                        self.buffers_reducer_1.input.din = self.dheap.out_main_bits()?.clone();
                        self.dheap.input.out_main_ready = self.buffers_reducer_1.in_ready();
                    }
                }
            }

            if self.dheap.out_sub_valid() {
                match get_dests(&self.dheap.out_sub_bits().load) {
                    DESTs::ToIO => {
                        // out_sub is produced by a fresh frozen App becoming
                        // demanded; route this active app through DHeap first.
                        self.buffers_dheap_a_3.input.in_valid = true;
                        self.buffers_dheap_a_3.input.din = self.dheap.out_sub_bits().clone();
                        self.dheap.input.out_sub_ready = self.buffers_dheap_a_3.in_ready();
                    }
                    DESTs::ToDHeap => {
                        self.buffers_dheap_a_3.input.in_valid = true;
                        self.buffers_dheap_a_3.input.din = self.dheap.out_sub_bits().clone();
                        self.dheap.input.out_sub_ready = self.buffers_dheap_a_3.in_ready();
                    }
                    DESTs::ToALU => {
                        self.buffers_alu_2.input.in_valid = true;
                        self.buffers_alu_2.input.din = self.dheap.out_sub_bits().clone();
                        self.dheap.input.out_sub_ready = self.buffers_alu_2.in_ready();
                    }
                    DESTs::ToReducer => {
                        self.buffers_reducer_3.input.in_valid = true;
                        self.buffers_reducer_3.input.din = self.dheap.out_sub_bits().clone();
                        self.dheap.input.out_sub_ready = self.buffers_reducer_3.in_ready();
                    }
                }
            }

            self.reducer.input.app_ready = self.buffers_dheap_b_0.in_ready();
            self.buffers_dheap_b_0.input.in_valid = self.reducer.app_valid();
            self.buffers_dheap_b_0.input.din = self.reducer.app_bits().clone();

            self.buffers_dheap_b_1.input.in_valid = self.dheap.out_big_drf_valid();
            self.buffers_dheap_b_1.input.din = self.dheap.out_big_drg_bits()?;

            let out_spine = &self.reducer.spine_bits();
            if self.reducer.spine_valid() {
                match get_dests(&out_spine.load) {
                    DESTs::ToIO => {
                        // No Reducer-to-IO shortcut: DHeap must check strictness.
                        self.buffers_dheap_a_1.input.in_valid = true;
                        self.buffers_dheap_a_1.input.din = out_spine.clone();
                        self.reducer.input.spine_ready = self.buffers_dheap_a_1.in_ready();
                    }
                    DESTs::ToDHeap => {
                        self.buffers_dheap_a_1.input.in_valid = true;
                        self.buffers_dheap_a_1.input.din = out_spine.clone();
                        self.reducer.input.spine_ready = self.buffers_dheap_a_1.in_ready();
                    }
                    DESTs::ToALU => {
                        self.buffers_alu_0.input.in_valid = true;
                        self.buffers_alu_0.input.din = out_spine.clone();
                        self.reducer.input.spine_ready = self.buffers_alu_0.in_ready();
                    }
                    DESTs::ToReducer => {
                        self.buffers_reducer_0.input.in_valid = true;
                        self.buffers_reducer_0.input.din = out_spine.clone();
                        self.reducer.input.spine_ready = self.buffers_reducer_0.in_ready();
                    }
                }
            }

            if self.alu.output_valid() {
                if !is_whnf(&self.alu.output_bits()?.load) {
                    self.buffers_reducer_2.input.in_valid = true;
                    self.buffers_reducer_2.input.din = self.alu.output_bits()?;
                    self.alu.input.output_ready = self.buffers_reducer_2.in_ready();
                } else {
                    self.buffers_dheap_a_2.input.in_valid = true;
                    self.buffers_dheap_a_2.input.din = self.alu.output_bits()?;
                    self.alu.input.output_ready = self.buffers_dheap_a_2.in_ready();
                }
            }

            // when dheap reads an app, search whether that app is still outstanding
            let search = self.dheap.search()?;
            self.reducer.input.search = search;
            self.rings_dheap_b_0.input.search = search;
            self.rings_dheap_b_0.input.in_fire = self.buffers_dheap_b_0.in_fire();
            self.rings_dheap_b_0.input.din = self.buffers_dheap_b_0.input.din.heap_addr;
            self.rings_dheap_b_0.input.out_fire = self.buffers_dheap_b_0.out_fire();
            self.rings_dheap_b_1.input.search = search;
            self.rings_dheap_b_1.input.in_fire = self.buffers_dheap_b_1.in_fire();
            self.rings_dheap_b_1.input.din = self.buffers_dheap_b_1.input.din.heap_addr;
            self.rings_dheap_b_1.input.out_fire = self.buffers_dheap_b_1.out_fire();
            self.dheap.input.found = self.reducer.found()
                || self.rings_dheap_b_0.found()
                || self.rings_dheap_b_1.found();
        }

        self.cycle_ctr += 1;
        if self.cycle_ctr == 500 {
            let work_set = traverse(
                &self.dheap.heap_mem.ram,
                self.heap_size,
                self.free_from,
                self.gc.reg_monitors.value(),
            );
            self.peak_workset_size = max(work_set.len(), self.peak_workset_size);
            self.cycle_ctr = 0;
            // if work_set.len() > 100 && work_set.len() < 200 {
            //     println!("======== live set ========");
            //     work_set.sort();
            //     for addr in work_set {
            //         println!("{}: {:?}", addr, self.dheap.heap_mem.ram[addr]);
            //     }
            // }
        }
        Ok(())
    }

    fn tick_children(&mut self) -> std::result::Result<(), std::string::String> {
        let _span = tracy_client::span!("ouros_core: tick_children");
        self.dheap.tick()?;
        self.reducer.tick()?;
        self.alu.tick()?;
        self.gc.tick()?;
        self.abox.tick()?;

        self.buffers_dealloc.tick()?;
        self.buffers_free_addr.tick()?;
        self.buffers_feedback.tick()?;

        self.buffers_dheap_a_0.tick()?;
        self.buffers_dheap_a_1.tick()?;
        self.buffers_dheap_a_2.tick()?;
        self.buffers_dheap_a_3.tick()?;
        self.buffers_dheap_a_4.tick()?;
        self.arbiter_dheap_a.tick()?;

        self.buffers_io_0.tick()?;
        self.buffers_io_1.tick()?;
        self.arbiter_io.tick()?;

        self.buffers_dheap_b_0.tick()?;
        self.buffers_dheap_b_1.tick()?;
        self.arbiter_dheap_b.tick()?;

        self.rings_dheap_b_0.tick()?;
        self.rings_dheap_b_1.tick()?;

        self.buffers_reducer_0.tick()?;
        self.buffers_reducer_1.tick()?;
        self.buffers_reducer_2.tick()?;
        self.buffers_reducer_3.tick()?;
        self.arbiter_reducer.tick()?;

        self.buffers_alu_0.tick()?;
        self.buffers_alu_1.tick()?;
        self.buffers_alu_2.tick()?;
        self.arbiter_alu.tick()
    }
}

///////////////////// helper function for heap graph traversal /////////////////////
/// traverse the heap and return all reachable addrs from the roots
fn traverse(
    ram: &Vec<App>,
    size: usize,
    free_from: usize,
    monitors: &[(bool, App); MAX_THREADS],
) -> Vec<usize> {
    let mut visited: Vec<bool> = vec![false; size];
    let mut traverse_stk: Vec<usize> = Vec::new();
    let mut res: Vec<usize> = Vec::new();

    // push all roots
    for i in 0..free_from {
        traverse_stk.push(i);
        visited[i] = true;
    }

    for m in monitors {
        if m.0 {
            for atm in &m.1 {
                if is_ptr(atm) && !visited[get_ptr(atm)] {
                    traverse_stk.push(get_ptr(atm));
                    visited[get_ptr(atm)] = true;
                }
            }
        }
    }

    while !traverse_stk.is_empty() {
        let work_on = traverse_stk.pop().unwrap();
        let app = &ram[work_on];
        res.push(work_on);
        for atm in app {
            if is_ptr(atm) && !visited[get_ptr(atm)] {
                // if get_ptr(atm) == 175 {
                //     println!("{} reachable from {}", get_ptr(atm), work_on);
                // }
                traverse_stk.push(get_ptr(atm));
                visited[get_ptr(atm)] = true;
            }
        }
    }

    res
}

// /// Quickly test whether the machine terminates and produces
// /// correct results on some small programs.
// #[test]
// fn ouros_core_spec() {
//     use super::benchmarks::*;
//     use Atom::*;
//     // [(program, result)]
//     let progs = [
//         (&BOOL_AND, COM(2, 1)),
//         (&BOOL_NEST, COM(2, 1)),
//         (&ALU_OP, INT(162)),
//         // (&MAP_Y, INT(1275)),
//         // (&DEADLOCK, INT(29380)),
//         // (&USE_SEQ, INT(10)),
//     ];

//     for (p, r) in progs {
//         let mut ouros = OurosCore::new(p, 0);
//         let mut cycle: i32 = 0;

//         ouros.tick();

//         // kick start the machine
//         ouros.input.start = true;
//         ouros.tick();
//         ouros.input.start = false;

//         loop {
//             assert!(cycle < 10_000);
//             if ouros.done() {
//                 break;
//             }
//             ouros.tick();
//             cycle += 1;
//         }

//         let res = &ouros.dheap.input.port_a_bits.load[0];
//         assert_eq!(*res, r);
//         eprintln! {"passed!"};
//     }
// }

#[cfg(test)]
mod io_routing_tests {
    use super::*;

    #[test]
    fn ready_io_has_its_own_destination() {
        let mut app = [Atom::Nop; APP_LENGTH];
        app[0] = Atom::IO(false);
        app[1] = Atom::Int(13);
        assert!(matches!(get_dests(&app), DESTs::ToDHeap));
        app[2] = Atom::Ptr(100, false, false);
        assert!(matches!(get_dests(&app), DESTs::ToDHeap));
        app[2] = Atom::Int(0);
        assert!(matches!(get_dests(&app), DESTs::ToIO));
    }

    #[test]
    fn io_request_survives_backpressure_and_response_reenters_dheap() {
        use crate::hw_module::HwModule;

        // A standalone saturated IO App at the root exercises the initial
        // DHeap input path as well as the ready/valid boundary. This uses an
        // INT as a synthetic completion solely to test transport; the real
        // IO unit must return the Pair specified in the roadmap.
        let prog = Program {
            heap_img: vec![vec![Atom::IO(false), Atom::Int(13), Atom::Int(0)]],
            comb_img: vec![],
        };
        let mut core = OurosCore::new(&prog, 0, 128, 0.0);
        core.tick().unwrap();
        core.input.start = true;
        core.tick().unwrap();
        core.input.start = false;
        for _ in 0..100 {
            if core.io_request_valid() { break; }
            core.tick().unwrap();
        }
        assert!(core.io_request_valid());
        let request = core.io_request_bits().unwrap();
        assert_eq!(request.stack_idx, 0);
        assert_eq!(request.load[0], Atom::IO(false));
        assert_eq!(request.load[1], Atom::Int(13));
        assert_eq!(request.load[2], Atom::Int(0));
        for _ in 0..8 {
            core.tick().unwrap(); // not ready; request must be stable
            assert!(core.io_request_valid());
        }
        core.input.io_request_ready = true;
        core.tick().unwrap();
        core.input.io_request_ready = false;
        assert!(!core.io_request_valid()); // exactly one request was consumed

        core.input.io_response_bits = ActiveApp {
            stack_idx: 0,
            load: [Atom::Int(42), Atom::Nop, Atom::Nop, Atom::Nop,
                   Atom::Nop, Atom::Nop, Atom::Nop, Atom::Nop],
        };
        core.input.io_response_valid = true;
        assert!(core.io_response_ready());
        core.tick().unwrap();
        core.input.io_response_valid = false;
        for _ in 0..100 {
            if core.done() { break; }
            core.tick().unwrap();
        }
        assert!(core.done());
        assert!(!core.io_request_valid());
    }

    #[test]
    fn io_response_port_starts_empty() {
        let prog = Program {
            heap_img: vec![vec![Atom::Int(10)]],
            comb_img: vec![],
        };
        let core = OurosCore::new(&prog, 0, 128, 0.2);
        assert!(!core.io_request_valid());
        assert!(core.io_request_bits().is_none());
        assert!(core.io_response_ready());
    }
}
