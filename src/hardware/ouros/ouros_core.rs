// Top level module of the Ouros core.

use crate::hardware::common::fifo::FIFOStat;
use crate::hardware::common::memory::DualPortMemStat;
use crate::hardware::common::{Arbiter, Ring, FIFO};
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
}

fn is_lit_seq(app: &App) -> bool {
    match app[0] {
        Atom::SEQ(true) => is_lit_atom(&app[2]),
        _ => false,
    }
}

fn get_dests(app: &App) -> DESTs {
    if is_prm(&app[0]) && is_int(&app[1]) && is_int(&app[2]) {
        DESTs::ToALU
    } else if !is_whnf(&app) && (is_comb(&app[0]) || is_lit_seq(&app) || is_con(&app[0])) {
        DESTs::ToReducer
    } else {
        DESTs::ToDHeap
    }
}

#[derive(Default)]
pub struct OurosCoreInput {
    pub start: bool,
}

impl HwInput for OurosCoreInput {}

// #[derive(Default)]
pub struct OurosCoreStat<'a> {
    pub dheap_stat: &'a DrfHeapStat,
    pub reducer_stat: &'a ReducerStat,
    pub alu_stat: &'a AluStat,
    pub fifos_stat: [&'a FIFOStat; 13],
    pub mem_stat: &'a DualPortMemStat,
    pub gc_stat: &'a GbgCollectorStat,
}

pub struct OurosCore {
    pub input: OurosCoreInput,
    pub dheap: DrfHeap,
    gc: GbgCollector,
    abox: AddrBox,
    reducer: Reducer,
    alu: Alu,

    buffers_dealloc: FIFO<usize, 2, false>,
    buffers_free_addr: FIFO<usize, 2, false>,

    buffers_dheap_a_0: FIFO<ActiveApp, BUFFER_SIZE, false>,
    buffers_dheap_a_1: FIFO<ActiveApp, BUFFER_SIZE, false>,
    buffers_dheap_a_2: FIFO<ActiveApp, BUFFER_SIZE, false>,
    buffers_dheap_a_3: FIFO<ActiveApp, BUFFER_SIZE, false>,
    arbiter_dheap_a: Arbiter<ActiveApp, 4>,

    buffers_dheap_b_0: FIFO<FrozenApp, BUFFER_SIZE, true>,
    buffers_dheap_b_1: FIFO<FrozenApp, BUFFER_SIZE, true>,
    arbiter_dheap_b: Arbiter<FrozenApp, 2>,

    rings_dheap_b_0: Ring<usize, BUFFER_SIZE>,
    rings_dheap_b_1: Ring<usize, BUFFER_SIZE>,

    buffers_reducer_0: FIFO<ActiveApp, BUFFER_SIZE, false>,
    buffers_reducer_1: FIFO<ActiveApp, BUFFER_SIZE, false>,
    buffers_reducer_2: FIFO<ActiveApp, BUFFER_SIZE, false>,
    buffers_reducer_3: FIFO<ActiveApp, BUFFER_SIZE, false>,
    arbiter_reducer: Arbiter<ActiveApp, 4>,

    buffers_alu_0: FIFO<ActiveApp, BUFFER_SIZE, false>,
    buffers_alu_1: FIFO<ActiveApp, BUFFER_SIZE, false>,
    buffers_alu_2: FIFO<ActiveApp, BUFFER_SIZE, false>,
    arbiter_alu: Arbiter<ActiveApp, 3>,
}

impl OurosCore {
    pub fn new(prog: &Program, detail_lv: u8) -> Self {
        let buffer_usage: bool = detail_lv >= DLV_BUFFER_USAGE;
        Self {
            input: Default::default(),
            dheap: DrfHeap::new(HEAP_SIZE)
                .program(&prog.heap_img)
                .detail(detail_lv),
            gc: GbgCollector::new(HEAP_SIZE, prog.heap_img.len())
                .init_freelist()
                .detail(detail_lv),
            abox: AddrBox::new(),
            reducer: Reducer::new(PROG_SIZE)
                .program(&prog.comb_img)
                .detail(detail_lv),
            alu: Alu::new().detail(detail_lv),

            buffers_dealloc: FIFO::new(),
            buffers_free_addr: FIFO::new(),

            buffers_dheap_a_0: FIFO::new().record_stat(buffer_usage),
            buffers_dheap_a_1: FIFO::new().record_stat(buffer_usage),
            buffers_dheap_a_2: FIFO::new().record_stat(buffer_usage),
            buffers_dheap_a_3: FIFO::new().record_stat(buffer_usage),
            arbiter_dheap_a: Arbiter::new(),

            buffers_dheap_b_0: FIFO::new().record_stat(buffer_usage),
            buffers_dheap_b_1: FIFO::new().record_stat(buffer_usage),
            arbiter_dheap_b: Arbiter::new(),

            rings_dheap_b_0: Ring::new(),
            rings_dheap_b_1: Ring::new(),

            buffers_reducer_0: FIFO::new().record_stat(buffer_usage),
            buffers_reducer_1: FIFO::new().record_stat(buffer_usage),
            buffers_reducer_2: FIFO::new().record_stat(buffer_usage),
            buffers_reducer_3: FIFO::new().record_stat(buffer_usage),
            arbiter_reducer: Arbiter::new(),

            buffers_alu_0: FIFO::new().record_stat(buffer_usage),
            buffers_alu_1: FIFO::new().record_stat(buffer_usage),
            buffers_alu_2: FIFO::new().record_stat(buffer_usage),
            arbiter_alu: Arbiter::new(),
        }
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
                self.buffers_dheap_b_0.get_stat(),
                self.buffers_dheap_b_1.get_stat(),
                self.buffers_reducer_0.get_stat(),
                self.buffers_reducer_1.get_stat(),
                self.buffers_reducer_2.get_stat(),
                self.buffers_reducer_3.get_stat(),
            ],
            mem_stat: self.dheap.get_mem_stat(),
            gc_stat: self.gc.get_stat(),
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
    buffers: [&mut FIFO<T, N, P>; A],
    arbiter: &mut Arbiter<T, A>,
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
    fn update_local(&mut self) {
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
        self.buffers_reducer_0.input.default_input();
        self.buffers_reducer_1.input.default_input();
        self.buffers_reducer_2.input.default_input();
        self.buffers_reducer_3.input.default_input();
        self.buffers_alu_0.input.default_input();
        self.buffers_alu_1.input.default_input();
        self.buffers_alu_2.input.default_input();

        // connect start signal
        self.dheap.input.start = self.input.start;

        // this 'kind of' fixes the ring problem
        for _ in 0..3 {
            // gc control signals
            self.buffers_dealloc.input.din = self.dheap.dealloc_bits();
            self.buffers_dealloc.input.in_valid = self.dheap.dealloc_valid();
            self.buffers_free_addr.input.din = self.gc.addr_out_bits();
            self.buffers_free_addr.input.in_valid = self.gc.addr_out_valid();
            self.gc.input.deallocate_bits = *self.buffers_dealloc.dout().unwrap_or(&0);
            self.gc.input.deallocate_valid = self.buffers_dealloc.out_valid();
            self.gc.input.addr_out_ready = self.buffers_free_addr.in_ready();

            self.abox.input.free_addr_bits = *self.buffers_free_addr.dout().unwrap_or(&0);
            self.abox.input.free_addr_valid = self.buffers_free_addr.out_valid();
            self.abox.input.addr_consume[CONSUMERS - 1] = self.dheap.need_split();
            self.abox.input.addr_consume[0..CONSUMERS - 1]
                .copy_from_slice(&self.reducer.consume_demands());
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

            // connect arbiters as components' input (arbiter first)
            self.arbiter_dheap_a.input.out_ready = self.dheap.port_a_ready();
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
                ],
                &mut self.arbiter_dheap_a,
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

            if self.dheap.out_main_valid() {
                match get_dests(&self.dheap.out_main_bits().load) {
                    DESTs::ToDHeap => {
                        self.buffers_dheap_a_0.input.in_valid = true;
                        self.buffers_dheap_a_0.input.din = self.dheap.out_main_bits().clone();
                        self.dheap.input.out_main_ready = self.buffers_dheap_a_0.in_ready();
                    }
                    DESTs::ToALU => {
                        self.buffers_alu_1.input.in_valid = true;
                        self.buffers_alu_1.input.din = self.dheap.out_main_bits().clone();
                        self.dheap.input.out_main_ready = self.buffers_alu_1.in_ready();
                    }
                    DESTs::ToReducer => {
                        self.buffers_reducer_1.input.in_valid = true;
                        self.buffers_reducer_1.input.din = self.dheap.out_main_bits().clone();
                        self.dheap.input.out_main_ready = self.buffers_reducer_1.in_ready();
                    }
                }
            }

            if self.dheap.out_sub_valid() {
                match get_dests(&self.dheap.out_sub_bits().load) {
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
            self.buffers_dheap_b_1.input.din = self.dheap.out_big_drg_bits();

            let out_spine = &self.reducer.spine_bits();
            if self.reducer.spine_valid() {
                match get_dests(&out_spine.load) {
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
                if !is_whnf(&self.alu.output_bits().load) {
                    self.buffers_reducer_2.input.in_valid = true;
                    self.buffers_reducer_2.input.din = self.alu.output_bits();
                    self.alu.input.output_ready = self.buffers_reducer_2.in_ready();
                } else {
                    self.buffers_dheap_a_2.input.in_valid = true;
                    self.buffers_dheap_a_2.input.din = self.alu.output_bits();
                    self.alu.input.output_ready = self.buffers_dheap_a_2.in_ready();
                }
            }

            // when dheap reads an app, search whether that app is still outstanding
            let search = self.dheap.search();
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
    }

    fn tick_children(&mut self) {
        self.dheap.tick();
        self.reducer.tick();
        self.alu.tick();
        self.gc.tick();
        self.abox.tick();

        self.buffers_dealloc.tick();
        self.buffers_free_addr.tick();

        self.buffers_dheap_a_0.tick();
        self.buffers_dheap_a_1.tick();
        self.buffers_dheap_a_2.tick();
        self.buffers_dheap_a_3.tick();
        self.arbiter_dheap_a.tick();

        self.buffers_dheap_b_0.tick();
        self.buffers_dheap_b_1.tick();
        self.arbiter_dheap_b.tick();

        self.rings_dheap_b_0.tick();
        self.rings_dheap_b_1.tick();

        self.buffers_reducer_0.tick();
        self.buffers_reducer_1.tick();
        self.buffers_reducer_2.tick();
        self.buffers_reducer_3.tick();
        self.arbiter_reducer.tick();

        self.buffers_alu_0.tick();
        self.buffers_alu_1.tick();
        self.buffers_alu_2.tick();
        self.arbiter_alu.tick();
    }
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
