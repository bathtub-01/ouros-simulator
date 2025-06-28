// Top level module of the Ouros core.

use crate::hardware::common::fifo::FIFOStat;
use crate::hardware::common::memory::DualPortMemStat;
use crate::hardware::common::{Arbiter, FIFO};
use crate::hardware::ouros::program::app_length;
use crate::hw_module::{HwInput, HwModule};

use super::alu::{Alu, AluStat};
use super::deref_heap::{DrfHeap, DrfHeapStat};
use super::program::{is_whnf, ActiveApp, App, Atom, FrozenApp, Program};
use super::reducer::{Reducer, ReducerStat};

#[derive(Default)]
pub struct OurosCoreInput {
    pub start: bool,
}

impl HwInput for OurosCoreInput {}

// TODO: add more detailed FIFO depth configuration.
pub struct OurosCore {
    pub input: OurosCoreInput,
    dheap: DrfHeap,
    reducer: Reducer,
    alu: Alu,

    buffers_dheap_a_0: FIFO<ActiveApp, 8>,
    buffers_dheap_a_1: FIFO<ActiveApp, 8>,
    buffers_dheap_a_2: FIFO<ActiveApp, 8>,
    buffers_dheap_a_3: FIFO<ActiveApp, 8>,
    arbiter_dheap_a: Arbiter<ActiveApp, 4>,

    buffers_dheap_b_0: FIFO<FrozenApp, 8>,
    buffers_dheap_b_1: FIFO<FrozenApp, 8>,
    buffers_dheap_b_2: FIFO<FrozenApp, 8>,
    arbiter_dheap_b: Arbiter<FrozenApp, 3>,

    buffers_reducer_0: FIFO<ActiveApp, 8>,
    buffers_reducer_1: FIFO<ActiveApp, 8>,
    buffers_reducer_2: FIFO<ActiveApp, 8>,
    buffers_reducer_3: FIFO<ActiveApp, 8>,
    arbiter_reducer: Arbiter<ActiveApp, 4>,

    buffers_alu_0: FIFO<ActiveApp, 8>,
    buffers_alu_1: FIFO<ActiveApp, 8>,
    buffers_alu_2: FIFO<ActiveApp, 8>,
    arbiter_alu: Arbiter<ActiveApp, 3>,
}

impl OurosCore {
    pub fn new(prog: &Program) -> Self {
        Self {
            input: Default::default(),
            dheap: DrfHeap::new(1024 * 64).program(prog),
            reducer: Reducer::new(),
            alu: Alu::new(),

            buffers_dheap_a_0: FIFO::new(),
            buffers_dheap_a_1: FIFO::new(),
            buffers_dheap_a_2: FIFO::new(),
            buffers_dheap_a_3: FIFO::new(),
            arbiter_dheap_a: Arbiter::new(),

            buffers_dheap_b_0: FIFO::new(),
            buffers_dheap_b_1: FIFO::new(),
            buffers_dheap_b_2: FIFO::new(),
            arbiter_dheap_b: Arbiter::new(),

            buffers_reducer_0: FIFO::new(),
            buffers_reducer_1: FIFO::new(),
            buffers_reducer_2: FIFO::new(),
            buffers_reducer_3: FIFO::new(),
            arbiter_reducer: Arbiter::new(),

            buffers_alu_0: FIFO::new(),
            buffers_alu_1: FIFO::new(),
            buffers_alu_2: FIFO::new(),
            arbiter_alu: Arbiter::new(),
        }
    }

    pub fn done(&self) -> bool {
        self.dheap.done()
    }

    pub fn get_stat(
        &self,
    ) -> (
        &DrfHeapStat,
        &ReducerStat,
        &AluStat,
        [&FIFOStat; 14],
        &DualPortMemStat,
    ) {
        (
            self.dheap.get_stat(),
            self.reducer.get_stat(),
            self.alu.get_stat(),
            [
                self.buffers_alu_0.get_stat(),
                self.buffers_alu_1.get_stat(),
                self.buffers_alu_2.get_stat(),
                self.buffers_dheap_a_0.get_stat(),
                self.buffers_dheap_a_1.get_stat(),
                self.buffers_dheap_a_2.get_stat(),
                self.buffers_dheap_a_3.get_stat(),
                self.buffers_dheap_b_0.get_stat(),
                self.buffers_dheap_b_1.get_stat(),
                self.buffers_dheap_b_2.get_stat(),
                self.buffers_reducer_0.get_stat(),
                self.buffers_reducer_1.get_stat(),
                self.buffers_reducer_2.get_stat(),
                self.buffers_reducer_3.get_stat(),
            ],
            self.dheap.get_mem_stat(),
        )
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
fn buffers_arbiter<T: Clone + Default, const N: usize, const A: usize>(
    buffers: [&mut FIFO<T, N>; A],
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

pub fn is_ptr(atom: &Atom) -> bool {
    match atom {
        Atom::PTR(_) => true,
        _ => false,
    }
}

pub fn is_comb(atom: &Atom) -> bool {
    match atom {
        Atom::COM(_, _, _) | Atom::Y => true,
        _ => false,
    }
}

pub fn is_int(atom: &Atom) -> bool {
    match atom {
        Atom::INT(_) => true,
        _ => false,
    }
}

pub fn is_prm(atom: &Atom) -> bool {
    match atom {
        Atom::PRM(_, _) => true,
        _ => false,
    }
}

impl HwModule for OurosCore {
    fn update_local(&mut self) {
        /*
        NOTICE: there is a combinatory logic ring in the whole cicuit,
        which is critical to allow pipelining when buffers are full.
        Due to the combinatory logic ring, the order of assigning inputs
        matters and might be buggy here.
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
                [
                    &mut self.buffers_dheap_b_0,
                    &mut self.buffers_dheap_b_1,
                    &mut self.buffers_dheap_b_2,
                ],
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
                if is_prm(&self.dheap.out_main_bits().load[0])
                    && is_int(&self.dheap.out_main_bits().load[1])
                    && is_int(&self.dheap.out_main_bits().load[2])
                {
                    // connect to alu
                    self.buffers_alu_1.input.in_valid = true;
                    self.buffers_alu_1.input.din = self.dheap.out_main_bits().clone();
                    self.dheap.input.out_main_ready = self.buffers_alu_1.in_ready();
                } else if !is_whnf(&self.dheap.out_main_bits().load)
                    && is_comb(&self.dheap.out_main_bits().load[0])
                {
                    // connect to reducer
                    self.buffers_reducer_1.input.in_valid = true;
                    self.buffers_reducer_1.input.din = self.dheap.out_main_bits().clone();
                    self.dheap.input.out_main_ready = self.buffers_reducer_1.in_ready();
                } else {
                    // connect to dheap
                    self.buffers_dheap_a_0.input.in_valid = true;
                    self.buffers_dheap_a_0.input.din = self.dheap.out_main_bits().clone();
                    self.dheap.input.out_main_ready = self.buffers_dheap_a_0.in_ready();
                }
            }

            if self.dheap.out_sub_valid() {
                if is_prm(&self.dheap.out_sub_bits().load[0])
                    && is_int(&self.dheap.out_sub_bits().load[1])
                    && is_int(&self.dheap.out_sub_bits().load[2])
                {
                    // connect to alu
                    self.buffers_alu_2.input.in_valid = true;
                    self.buffers_alu_2.input.din = self.dheap.out_sub_bits().clone();
                    self.dheap.input.out_sub_ready = self.buffers_alu_2.in_ready();
                } else if !is_whnf(&self.dheap.out_sub_bits().load)
                    && is_comb(&self.dheap.out_sub_bits().load[0])
                {
                    // connect to reducer
                    self.buffers_reducer_3.input.in_valid = true;
                    self.buffers_reducer_3.input.din = self.dheap.out_sub_bits().clone();
                    self.dheap.input.out_sub_ready = self.buffers_reducer_3.in_ready();
                } else {
                    // connect to dheap
                    self.buffers_dheap_a_3.input.in_valid = true;
                    self.buffers_dheap_a_3.input.din = self.dheap.out_sub_bits().clone();
                    self.dheap.input.out_sub_ready = self.buffers_dheap_a_3.in_ready();
                }
            }

            self.buffers_dheap_b_0.input.in_valid = self.reducer.app1().0;
            self.buffers_dheap_b_0.input.din = self.reducer.app1().1.clone();
            self.reducer.input.app1_ready = self.buffers_dheap_b_0.in_ready();

            self.buffers_dheap_b_1.input.in_valid = self.reducer.app2().0;
            self.buffers_dheap_b_1.input.din = self.reducer.app2().1.clone();
            self.reducer.input.app2_ready = self.buffers_dheap_b_1.in_ready();

            self.buffers_dheap_b_2.input.in_valid = self.reducer.app3().0;
            self.buffers_dheap_b_2.input.din = self.reducer.app3().1.clone();
            self.reducer.input.app3_ready = self.buffers_dheap_b_2.in_ready();

            if self.reducer.spine().0 {
                if is_prm(&self.reducer.spine().1.load[0])
                    && is_int(&self.reducer.spine().1.load[1])
                    && is_int(&self.reducer.spine().1.load[2])
                {
                    // connect to alu
                    self.buffers_alu_0.input.in_valid = true;
                    self.buffers_alu_0.input.din = self.reducer.spine().1.clone();
                    self.reducer.input.spine_ready = self.buffers_alu_0.in_ready();
                } else if !is_whnf(&self.reducer.spine().1.load)
                    && is_comb(&self.reducer.spine().1.load[0])
                {
                    // connect to reducer
                    self.buffers_reducer_0.input.in_valid = true;
                    self.buffers_reducer_0.input.din = self.reducer.spine().1.clone();
                    self.reducer.input.spine_ready = self.buffers_reducer_0.in_ready();
                } else {
                    // connect to dheap
                    self.buffers_dheap_a_1.input.in_valid = true;
                    self.buffers_dheap_a_1.input.din = self.reducer.spine().1.clone();
                    self.reducer.input.spine_ready = self.buffers_dheap_a_1.in_ready();
                }
            }

            if self.alu.output_valid() {
                if !is_whnf(&self.alu.output_bits().load) {
                    self.buffers_reducer_2.input.in_valid = true;
                    self.buffers_reducer_2.input.din = self.alu.output_bits().clone();
                    self.alu.input.output_ready = self.buffers_reducer_2.in_ready();
                } else {
                    self.buffers_dheap_a_2.input.in_valid = true;
                    self.buffers_dheap_a_2.input.din = self.alu.output_bits().clone();
                    self.alu.input.output_ready = self.buffers_dheap_a_2.in_ready();
                }
            }
        }

        // gc control signals
        self.reducer.input.free_addr = self.dheap.free_addr();
        self.dheap.input.addr_consumed = self.reducer.addr_consumed();
    }

    fn tick_children(&mut self) {
        self.dheap.tick();
        self.reducer.tick();
        self.alu.tick();

        self.buffers_dheap_a_0.tick();
        self.buffers_dheap_a_1.tick();
        self.buffers_dheap_a_2.tick();
        self.buffers_dheap_a_3.tick();
        self.arbiter_dheap_a.tick();

        self.buffers_dheap_b_0.tick();
        self.buffers_dheap_b_1.tick();
        self.buffers_dheap_b_2.tick();
        self.arbiter_dheap_b.tick();

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

/// Quickly test whether the machine terminates and produces
/// correct results.
#[test]
fn ouros_core_spec() {
    use super::benchmarks::*;
    use Atom::*;
    // [(program, result)]
    let progs = [
        // (&FIB, INT(89)),
        (&BOOL_AND, COM(2, 0, [1, 0, 0, 0, 0, 0])),
        (&BOOL_NEST, COM(2, 0, [1, 0, 0, 0, 0, 0])),
        (&ALU_OP, INT(162)),
        (&MAP_Y, INT(1275)),
    ];

    for (p, r) in progs {
        let mut ouros = OurosCore::new(p);
        let mut cycle: i32 = 0;

        ouros.tick();

        // kick start the machine
        ouros.input.start = true;
        ouros.tick();
        ouros.input.start = false;

        loop {
            assert!(cycle < 10_000);
            if ouros.done() {
                break;
            }
            ouros.tick();
            cycle += 1;
        }

        let res = &ouros.dheap.input.port_a_bits.load[0];
        assert_eq!(*res, r);
    }
}
