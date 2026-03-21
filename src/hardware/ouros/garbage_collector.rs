use std::cmp::max;

use crate::hardware::common::{DualPortMem, Register};
use crate::hw_module::{HwInput, HwModule};

use super::config::{APP_LENGTH, CACHE_SIZE, DLV_GC, MAX_THREADS};
use super::program::{get_ptr, is_ptr, ActiveApp, App, Atom};
use std::collections::VecDeque;

struct FixedFifo<T> {
    capacity: usize,
    data: VecDeque<T>,
}

impl<T: PartialEq> FixedFifo<T> {
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            data: VecDeque::with_capacity(capacity),
        }
    }

    fn push(&mut self, item: T) {
        if self.data.len() == self.capacity {
            self.data.pop_front(); // evict the oldest
        }
        self.data.push_back(item);
    }

    fn contains(&self, v: &T) -> bool {
        self.data.contains(v)
    }

    fn flush(&mut self) {
        self.data = VecDeque::with_capacity(self.capacity);
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
enum CollectorState {
    #[default]
    IDLE,
    ROOT,
    MARK,
    SWEEP,
}

#[derive(Default, Clone, Debug, PartialEq)]
enum CellState {
    #[default]
    Unmarked,
    Marked,
    FreeList,
    WorkList,
}

/// Basic cell of the GC bookkeeping memory
#[derive(Default, Clone, Debug)]
struct GCCell {
    state: CellState,
    ptr: usize,
}

fn no_ptr_cell(st: CellState) -> GCCell {
    GCCell { state: st, ptr: 0 }
}

#[derive(Default)]
pub struct GbgCollectorInput {
    pub feedback_valid: bool,
    pub feedback_bits: usize,
    pub deallocate_valid: bool,
    pub deallocate_bits: usize,
    pub addr_out_ready: bool,
    pub heap_read_bits: App,
    pub heap_read_valid: bool,
    pub monitor_valid: bool,
    pub monitor_bits: ActiveApp,
    pub monitor_big_drf_valid: bool,
    pub monitor_big_drf_bits: Atom,
    pub monitor_big_drf_stk: usize,
    pub monitor_unset_valid: bool,
    pub monitor_unset: usize,
}

impl HwInput for GbgCollectorInput {}

#[derive(Default)]
pub struct GbgCollectorStat {
    pub allocations: u32,
    pub feedbacks: u32,
    pub feedbacks_shadowed: u32,
    pub immediate_reuse: u32,           // gain from one-bit ref count
    pub m_request_per_cycle: Vec<bool>, // mutator request
    pub gc_rounds: u32,
    pub mark_cycles: u32,
    pub mark_cycles_move: [u32; 5],
    pub cache_hit: u32,
    pub cache_miss: u32,
    pub free_len: Vec<usize>,
    pub work_len: Vec<usize>,
}

pub struct GbgCollector {
    pub input: GbgCollectorInput,
    reg_collector: Register<CollectorState>,
    gc_mem: DualPortMem<GCCell>,
    reg_free_head: Register<usize>,
    reg_work_head: Register<usize>,
    reg_free_drawed: Register<bool>, // whether freelist was drawed in previous cycle
    reg_work_drawed: Register<bool>,
    reg_free_len: Register<usize>,
    reg_work_len: Register<usize>,
    reg_sweeper: Register<usize>,
    reg_bk_reader: Register<GCCell>,
    reg_heap_reader: Register<App>,
    reg_pre_gc: Register<bool>, // whet previous cycle is used by GC
    reg_move: Register<u8>,
    reg_work_on: Register<usize>, // to lock the worklist object for marking
    reg_app_idx: Register<usize>,
    pub reg_monitors: Register<[(bool, App); MAX_THREADS]>,
    reg_monitor_idx: Register<usize>,
    const_sweep_from: usize,
    const_heap_size: usize,
    const_gc_at: f32,
    const_gc_threshold: usize,
    gc_cache: FixedFifo<usize>,
    stat: GbgCollectorStat,
    stat_detail_lv: u8,
}

impl GbgCollector {
    pub fn new(heap_size: usize, free_from: usize, gc_at: f32) -> Self {
        Self {
            input: Default::default(),
            reg_collector: Default::default(),
            gc_mem: DualPortMem::new(heap_size),
            reg_free_head: Register::init(free_from),
            reg_work_head: Default::default(),
            reg_free_drawed: Default::default(),
            reg_work_drawed: Default::default(),
            reg_free_len: Register::init(heap_size - free_from),
            reg_work_len: Default::default(),
            reg_sweeper: Default::default(),
            reg_bk_reader: Default::default(),
            reg_heap_reader: Default::default(),
            reg_pre_gc: Default::default(),
            reg_move: Default::default(),
            reg_work_on: Default::default(),
            reg_app_idx: Default::default(),
            reg_monitors: Default::default(),
            reg_monitor_idx: Default::default(),
            const_sweep_from: free_from,
            const_heap_size: heap_size,
            const_gc_at: gc_at,
            const_gc_threshold: (heap_size as f32 * gc_at) as usize,
            gc_cache: FixedFifo::new(CACHE_SIZE),
            stat: Default::default(),
            stat_detail_lv: Default::default(),
        }
    }

    /// setup the freelist before execution
    pub fn init_freelist(mut self) -> Self {
        let from = *self.reg_free_head.value();
        let heap_size = self.gc_mem.ram.len();
        let mut img: Vec<GCCell> = vec![Default::default(); heap_size];
        for i in from..heap_size {
            img[i] = GCCell {
                state: CellState::FreeList,
                ptr: i + 1,
            };
        }
        self.gc_mem.image(&img);
        self
    }

    pub fn detail(mut self, lv: u8) -> Self {
        self.stat_detail_lv = lv;
        self
    }

    fn push_to_freelist(&mut self, addr: usize, old_head: usize, use_port_a: bool) {
        self.reg_free_head.connect(&addr);
        let cell = GCCell {
            state: CellState::FreeList,
            ptr: old_head,
        };
        if use_port_a {
            self.gc_mem.write_a(addr, cell);
        } else {
            self.gc_mem.write_b(addr, cell);
        }
    }

    fn push_to_worklist(&mut self, addr: usize, old_head: usize) {
        self.reg_work_head.connect(&addr);
        self.gc_mem.write_b(
            addr,
            GCCell {
                state: CellState::WorkList,
                ptr: old_head,
            },
        );
    }

    /// draw an address from the freelist
    pub fn addr_out_bits(&self) -> usize {
        if self.deallocate_fire()
            && *self.reg_collector.value() != CollectorState::MARK
            && self.input.deallocate_bits != *self.reg_sweeper.value()
        {
            self.input.deallocate_bits
        } else {
            if *self.reg_free_drawed.value() {
                self.gc_mem.dout_a().ptr
            } else {
                *self.reg_free_head.value()
            }
        }
    }

    pub fn feedback_ready(&self) -> bool {
        true
    }

    pub fn feedback_fire(&self) -> bool {
        self.input.feedback_valid && self.feedback_ready()
    }

    pub fn deallocate_ready(&self) -> bool {
        true
        // false
    }

    pub fn deallocate_fire(&self) -> bool {
        self.input.deallocate_valid && self.deallocate_ready()
    }

    pub fn addr_out_valid(&self) -> bool {
        *self.reg_free_len.value() > 0
    }

    pub fn addr_out_fire(&self) -> bool {
        self.addr_out_valid() && self.input.addr_out_ready
    }

    pub fn get_stat(&self) -> &GbgCollectorStat {
        &self.stat
    }

    pub fn read_heap_req_valid(&self) -> bool {
        *self.reg_collector.value() == CollectorState::MARK
    }

    pub fn read_heap_req_addr(&self) -> usize {
        if *self.reg_move.value() == 0 {
            *self.reg_work_head.value()
        } else if *self.reg_move.value() == 1
            && self.input.heap_read_valid
            && !self.mutator_request()
        {
            let in_app = &self.input.heap_read_bits;
            if in_app.iter().any(|atm| is_ptr(atm)) {
                0
            } else {
                if *self.reg_work_drawed.value() {
                    self.gc_mem.dout_a().ptr
                } else {
                    *self.reg_work_head.value()
                }
            }
        } else if *self.reg_move.value() == 2 && !self.mutator_request() {
            let read_out = if *self.reg_pre_gc.value() {
                self.gc_mem.dout_a()
            } else {
                self.reg_bk_reader.value()
            };
            let mark_this = read_out.state == CellState::Unmarked;
            let current_app = self.reg_heap_reader.value();
            if self.more_ptr(current_app) {
                0
            } else {
                if mark_this {
                    get_ptr(&current_app[*self.reg_app_idx.value()])
                } else {
                    *self.reg_work_head.value()
                }
            }
        } else {
            *self.reg_work_on.value()
        }
    }

    /// whether there is a mutator request
    fn mutator_request(&self) -> bool {
        match self.reg_collector.value() {
            // MARK state ignores deallocate requests
            CollectorState::MARK => self.addr_out_fire() || self.feedback_fire(),
            _ => self.addr_out_fire() || self.deallocate_fire() || self.feedback_fire(),
        }
    }

    fn free_len_minus_one(&mut self) {
        self.reg_free_len.connect(&(*self.reg_free_len.value() - 1));
    }

    fn free_len_plus_one(&mut self) {
        self.reg_free_len.connect(&(*self.reg_free_len.value() + 1));
    }

    fn work_len_minus_one(&mut self) {
        self.reg_work_len.connect(&(*self.reg_work_len.value() - 1));
    }

    fn work_len_plus_one(&mut self) {
        self.reg_work_len.connect(&(*self.reg_work_len.value() + 1));
    }

    /// stolen from `reducer.rs`
    fn more_ptr<const N: usize>(&self, app: &[Atom; N]) -> bool {
        let idx = *self.reg_app_idx.value() + 1;
        app.into_iter()
            .skip(idx as usize)
            .any(|a| is_ptr(a) && !self.gc_cache.contains(&get_ptr(a)))
    }

    /// stolen from `reducer.rs`
    fn find_ptr<const N: usize>(&self, app: &[Atom; N]) -> usize {
        let idx = *self.reg_app_idx.value() + 1;
        idx + app
            .iter()
            .skip(idx)
            .position(|a| is_ptr(a) && !self.gc_cache.contains(&get_ptr(a)))
            .unwrap()
    }

    fn mark_read(&mut self, addr: usize) {
        if self.stat_detail_lv >= DLV_GC {
            if self.gc_cache.contains(&addr) {
                self.stat.cache_hit += 1;
            } else {
                self.stat.cache_miss += 1;
            }
        }
        self.gc_mem.read_a(addr);
        // self.gc_cache.push(addr);
    }

    fn consume_worklist(&mut self) {
        if *self.reg_work_len.value() == 0 {
            // pre read for SWEEP
            self.reg_sweeper.connect(&0);
            self.gc_mem.read_a(0);
            self.reg_pre_gc.connect(&true);
            self.reg_collector.connect(&CollectorState::SWEEP);
        } else {
            // this is the ONLY operation to reduce worklist length
            let w_head: usize = if *self.reg_work_drawed.value() {
                self.gc_mem.dout_a().ptr
            } else {
                *self.reg_work_head.value()
            };
            // println!("marking: {}", w_head);
            self.gc_mem.write_a(w_head, no_ptr_cell(CellState::Marked));
            // self.gc_cache.push(w_head);
            self.reg_work_drawed.connect(&true); // next head will be read out by write_a
            self.work_len_minus_one();
            self.reg_work_on.connect(&w_head);
            self.reg_move.connect(&1);
            self.reg_app_idx.connect(&0);
        }
    }

    fn step_idle(&mut self) {
        if !self.mutator_request() && *self.reg_free_len.value() <= self.const_gc_threshold {
            self.reg_collector.connect(&CollectorState::ROOT);
            // put `main` into worklist
            self.reg_work_head.connect(&0);
            self.reg_work_len.connect(&1);
            self.gc_mem.write_b(
                0,
                GCCell {
                    state: CellState::WorkList,
                    ptr: 0,
                },
            );
            self.reg_sweeper.connect(&0);
            self.reg_monitor_idx.connect(&0);
        }
    }

    fn step_root(&mut self) {
        if !self.mutator_request() {
            if *self.reg_sweeper.value() < self.const_sweep_from - 1 {
                // push next cell into worlist
                let next = self.reg_sweeper.value() + 1;
                self.reg_sweeper.connect(&next);
                self.push_to_worklist(next, *self.reg_work_head.value());
                self.work_len_plus_one();
            } else {
                // go to MARK
                self.reg_collector.connect(&CollectorState::MARK);
                self.reg_move.connect(&0);
                self.reg_pre_gc.connect(&false);
                self.gc_cache.flush();
                // println!("================== MARK START =====================");

                // for i in 0..100 {
                //     println!("brefore MARK, addr-{} is in {:?}", i, self.gc_mem.ram[i]);
                // }
            }
        }
    }

    fn step_mark(&mut self) {
        if self.stat_detail_lv >= DLV_GC && !self.mutator_request() {
            self.stat.mark_cycles += 1;
            self.stat.mark_cycles_move[*self.reg_move.value() as usize] += 1;
        }
        // ========== defaults (update regs from previous demands) ==========
        self.reg_pre_gc.connect(&false);
        if *self.reg_pre_gc.value() {
            self.reg_bk_reader.connect(self.gc_mem.dout_a());
        }

        // ========== move-0 logic (pop a node from worklist) ==========
        if *self.reg_move.value() == 0 && !self.mutator_request() {
            self.consume_worklist();
        }

        // ========== move-1 logic (wait until main heap read success) ==========
        // TODO heap read does not have to sync with no-mutator-req
        if *self.reg_move.value() == 1 && self.input.heap_read_valid && !self.mutator_request() {
            // println!(
            //     // useful MARK log
            //     "work on: {} | app: {:?} | worklist len: {}",
            //     self.reg_work_on.value(),
            //     self.input.heap_read_bits,
            //     self.reg_work_len.value()
            // );

            // let real_worklist_head: usize = if *self.reg_work_drawed.value() {
            //     self.gc_mem.dout_a().ptr
            // } else {
            //     *self.reg_work_head.value()
            // };
            // let mut fl: Vec<usize> = Vec::new();

            // for i in 0..*self.reg_work_len.value() {
            //     if i == 0 {
            //         fl.push(real_worklist_head);
            //     } else {
            //         let fst = fl.last().unwrap();
            //         assert!(self.gc_mem.ram[*fst].state == CellState::WorkList);
            //         fl.push(self.gc_mem.ram[*fst].ptr);
            //     }
            // }
            // println!("worklist at this point: {:?}", fl);
            ///////////////////////////////////////////////
            let in_app = &self.input.heap_read_bits;
            // splition of move-0 and move-1 is needed because mutator requests can modify worklist
            if in_app.iter().any(|atm| is_ptr(atm)) {
                self.reg_heap_reader.connect(&in_app);
                self.reg_move.connect(&2);
                // pre read for move-2
                let found = in_app.iter().position(|atm| is_ptr(atm)).unwrap();
                // println!("go to move-2, read on: {}", get_ptr(&in_app[found]));
                self.mark_read(get_ptr(&in_app[found]));
                // self.gc_mem.read_a(get_ptr(&in_app[found]));
                self.reg_app_idx.connect(&found);
                self.reg_pre_gc.connect(&true);
            } else {
                self.consume_worklist();
            }
        }

        // ========== move-2 logic (push one, read one) ==========
        if *self.reg_move.value() == 2 && !self.mutator_request() {
            let read_out = if *self.reg_pre_gc.value() {
                self.gc_mem.dout_a()
            } else {
                self.reg_bk_reader.value()
            };
            let current_app = self.reg_heap_reader.value();
            let mark_this = read_out.state == CellState::Unmarked;
            if mark_this {
                // push it to worklist
                self.push_to_worklist(
                    get_ptr(&current_app[*self.reg_app_idx.value()]),
                    *self.reg_work_head.value(),
                );
                self.work_len_plus_one();
            }

            let current_app = self.reg_heap_reader.value();
            if self.more_ptr(current_app) {
                // read the next PTR
                let found = self.find_ptr(current_app);
                self.mark_read(get_ptr(&current_app[found]));
                self.reg_app_idx.connect(&found);
                self.reg_pre_gc.connect(&true);
            } else if self
                .reg_monitors
                .value()
                .iter()
                .skip(*self.reg_monitor_idx.value())
                .any(|r| r.0)
            {
                // if there is a monitor yet to be handled, put it in reader and use move-2 logic
                let pick_monitor = self
                    .reg_monitors
                    .value()
                    .iter()
                    .skip(*self.reg_monitor_idx.value())
                    .position(|r| r.0)
                    .unwrap()
                    + *self.reg_monitor_idx.value();
                // self.reg_monitors.input[pick_monitor].0 = false;
                self.reg_monitor_idx.connect(&(pick_monitor + 1));
                let monitor_app = &self.reg_monitors.input[pick_monitor].1;
                if is_ptr(&monitor_app[0]) {
                    self.reg_pre_gc.connect(&true);
                    self.gc_mem.read_a(get_ptr(&monitor_app[0]));
                } else {
                    self.reg_pre_gc.connect(&false);
                    self.reg_bk_reader.connect(&no_ptr_cell(CellState::Marked));
                }
                self.reg_heap_reader.connect(monitor_app);
                self.reg_app_idx.connect(&0);
                self.reg_move.connect(&2);
            } else {
                // go back to move-0, handle the next worklist node
                if mark_this {
                    let w_head = get_ptr(&current_app[*self.reg_app_idx.value()]);
                    self.gc_mem.write_a(w_head, no_ptr_cell(CellState::Marked));
                    self.gc_mem.input.port_b.is_write = false;
                    // self.gc_cache.push(w_head);
                    self.reg_work_head
                        .connect(&self.reg_work_head.value().clone());
                    self.reg_work_len
                        .connect(&self.reg_work_len.value().clone());
                    self.reg_work_on.connect(&w_head);
                    self.reg_move.connect(&1);
                    self.reg_app_idx.connect(&0);
                } else if *self.reg_work_len.value() == 0 {
                    // pre read for SWEEP
                    self.reg_sweeper.connect(&0);
                    self.gc_mem.read_a(0);
                    self.reg_pre_gc.connect(&true);
                    self.reg_collector.connect(&CollectorState::SWEEP);
                } else {
                    let w_head = *self.reg_work_head.value();
                    self.gc_mem.write_a(w_head, no_ptr_cell(CellState::Marked));
                    // self.gc_cache.push(w_head);
                    self.reg_work_drawed.connect(&true); // next head will be read out by write_a
                    self.work_len_minus_one();
                    self.reg_work_on.connect(&w_head);
                    self.reg_move.connect(&1);
                    self.reg_app_idx.connect(&0);
                }
            }
        }
    }

    fn step_sweep(&mut self) {
        // ========== defaults (update regs from previous demands) ==========
        self.reg_pre_gc.connect(&false);
        if *self.reg_pre_gc.value() {
            self.reg_bk_reader.connect(self.gc_mem.dout_a());
        }

        // ========== sweep logic (push one, read one) ==========
        if !self.mutator_request() {
            let read_out = if *self.reg_pre_gc.value() {
                self.gc_mem.dout_a()
            } else {
                self.reg_bk_reader.value()
            };

            // if *self.reg_sweeper.value() == 156 {
            //     println!("sweep on 156: {:?}", self.gc_mem.ram[156]);
            // }

            // NOTE all cells below const_mark_from will be marked due to ROOT
            if read_out.state == CellState::Marked {
                // recover as Unmarked
                // self.gc_mem
                // .write_b(*self.reg_sweeper.value(), no_ptr_cell(CellState::Unmarked));
                self.gc_mem
                    .write_b(*self.reg_sweeper.value(), no_ptr_cell(CellState::Unmarked));
                // if *self.reg_sweeper.value() == 156 {
                //     println!("recover 156 as Unmarked");
                // }
            } else if read_out.state == CellState::Unmarked {
                // push it to freelist
                let old_head: usize = if *self.reg_free_drawed.value() {
                    self.gc_mem.dout_a().ptr
                } else {
                    *self.reg_free_head.value()
                };
                self.push_to_freelist(*self.reg_sweeper.value(), old_head, false);
                self.free_len_plus_one();
                // if *self.reg_sweeper.value() == 433 {
                //     println!("pushing 433 to freelist!, old head: {}", old_head);
                // }
                if old_head == 0 {
                    println!("old head is 0 when pushing to freelist!");
                }
            } else if read_out.state == CellState::WorkList {
                panic!("Broken WorkList! addr: {}", *self.reg_sweeper.value());
            }

            if *self.reg_sweeper.value() < self.const_heap_size - 1 {
                // sweep next cell
                let next = self.reg_sweeper.value() + 1;
                self.reg_sweeper.connect(&next);
                self.gc_mem.read_a(next);
                self.reg_pre_gc.connect(&true);
            } else {
                // go back to IDLE
                self.reg_collector.connect(&CollectorState::IDLE);
            }
        }
    }
}

impl HwModule for GbgCollector {
    fn update_local(&mut self) {
        if *self.reg_collector.value() != CollectorState::MARK {
            // if self.input.monitor_unset_valid {
            //     self.reg_monitors.input[self.input.monitor_unset] = (false, Default::default());
            // }
            if self.input.monitor_big_drf_valid {
                self.reg_monitors.input[self.input.monitor_big_drf_stk] = {
                    let mut app: [Atom; APP_LENGTH] = self.reg_monitors.value()
                        [self.input.monitor_big_drf_stk]
                        .1
                        .clone();
                    app[0] = self.input.monitor_big_drf_bits.clone();
                    (true, app)
                }
            }
            if self.input.monitor_valid {
                self.reg_monitors.input[self.input.monitor_bits.stack_idx as usize] =
                    (true, self.input.monitor_bits.load.clone());
            }
        }

        let real_freelist_head: usize = if *self.reg_free_drawed.value() {
            self.gc_mem.dout_a().ptr
        } else {
            *self.reg_free_head.value()
        };
        let real_worklist_head: usize = if *self.reg_work_drawed.value() {
            self.gc_mem.dout_a().ptr
        } else {
            *self.reg_work_head.value()
        };
        // if real_freelist_head == 16 {
        //     println!(
        //         "real_freelist_head == 16-{:?}, read from: {}-{:?}, state: {:?}, len: {}, sweep: {}",
        //         self.gc_mem.ram[16],
        //         self.gc_mem.input.port_a.addr,
        //         self.gc_mem.ram[self.gc_mem.input.port_a.addr],
        //         self.reg_collector.value(),
        //         self.reg_free_len.value(),
        //         self.reg_sweeper.value(),
        //     );
        // }
        self.gc_mem.input.default_input();

        self.reg_free_drawed.connect(&false);
        self.reg_free_head.connect(&real_freelist_head);
        self.reg_work_drawed.connect(&false);
        self.reg_work_head.connect(&real_worklist_head);

        let can_dealloc = *self.reg_collector.value() != CollectorState::MARK
            && self.input.deallocate_bits != *self.reg_sweeper.value();

        if self.deallocate_fire() && can_dealloc {
            if self.addr_out_fire() {
                self.gc_mem
                    .write_a(self.input.deallocate_bits, no_ptr_cell(CellState::FreeList));
            } else {
                self.push_to_freelist(self.input.deallocate_bits, real_freelist_head, true);
                self.free_len_plus_one();
            }
        } else {
            if self.addr_out_fire() {
                self.reg_free_drawed.connect(&true);
                self.gc_mem.read_a(real_freelist_head); // NOTE it's still a free addr
                self.free_len_minus_one();
            } else {
                /* nothing to do */
            }
        }

        if self.feedback_fire() {
            // if self.input.feedback_bits == 175 {
            //     println!(
            //         "feedback {} arrived GC, state: {:?}",
            //         self.input.feedback_bits,
            //         self.reg_collector.value()
            //     );
            // }
            let cell_state = match self.reg_collector.value() {
                CollectorState::IDLE | CollectorState::ROOT => CellState::Unmarked,
                CollectorState::MARK => CellState::WorkList,
                CollectorState::SWEEP => {
                    if self.input.feedback_bits <= *self.reg_sweeper.value() {
                        CellState::Unmarked
                    } else {
                        CellState::Marked
                    }
                }
            };
            if cell_state == CellState::WorkList {
                // if self.input.feedback_bits == 156 {
                //     println!(
                //         "feedback: add {} to worklist, 156: {:?}",
                //         self.input.feedback_bits, self.gc_mem.ram[156]
                //     );
                //     let mut fl: Vec<usize> = Vec::new();
                //     fl.push(real_freelist_head);
                //     for _ in 0..*self.reg_free_len.value() {
                //         let fst = fl.last().unwrap();
                //         assert!(self.gc_mem.ram[*fst].state == CellState::FreeList);
                //         fl.push(self.gc_mem.ram[*fst].ptr);
                //     }
                //     println!("freelist at this point: {:?}", fl);
                // }
                self.reg_work_head.connect(&self.input.feedback_bits);
                self.work_len_plus_one();
            }
            self.gc_mem.write_b(
                self.input.feedback_bits,
                GCCell {
                    state: cell_state,
                    ptr: real_worklist_head,
                },
            );
        }

        // background GC work
        match self.reg_collector.value() {
            CollectorState::IDLE => self.step_idle(),
            CollectorState::ROOT => self.step_root(),
            CollectorState::MARK => self.step_mark(),
            CollectorState::SWEEP => self.step_sweep(),
        }

        if self.stat_detail_lv >= DLV_GC {
            if *self.reg_collector.value() == CollectorState::IDLE
                && self.reg_collector.input == CollectorState::ROOT
            {
                self.stat.gc_rounds += 1;
            }
        }

        // print!("collector: {:?}", self.reg_collector.value());
        // let look_at = 175;
        // if self.addr_out_fire() && self.addr_out_bits() == look_at {
        //     println!(
        //         "emit {} as free addr!, {:?}, state: {:?}, dout: {}, reg_head: {}, dealloc: {}, sweep {}",
        //         look_at,
        //         self.gc_mem.ram[look_at],
        //         self.reg_collector.value(),
        //         self.gc_mem.dout_a().ptr,
        //         self.reg_free_head.value(),
        //         self.input.deallocate_bits,
        //         self.reg_sweeper.value()
        //     );
        // }
        if self.addr_out_fire() && self.addr_out_bits() == 0 {
            println!(
                "emit 0 as free addr!, 0: {:?}, state: {:?}, dout: {}, reg_head: {}, dealloc: {}, sweep {}",
                self.gc_mem.ram[0],
                self.reg_collector.value(),
                self.gc_mem.dout_a().ptr,
                self.reg_free_head.value(),
                self.input.deallocate_bits,
                self.reg_sweeper.value()
            );
        }
    }

    fn update_stat(&mut self) {
        // println!(
        //     "collector state: {:?}, worklist len: {}, worklist head: {}, freelist len: {}, freelist head: {}, monitor idx: {}",
        //     self.reg_collector.value(),
        //     self.reg_work_len.value(),
        //     self.reg_work_head.input,
        //     self.reg_free_len.value(),
        //     self.reg_free_head.input,
        //     self.reg_monitor_idx.value()
        // );

        if self.addr_out_fire() {
            self.stat.allocations += 1;
        }

        if self.stat_detail_lv >= DLV_GC {
            if self.deallocate_fire() && *self.reg_collector.value() != CollectorState::MARK {
                self.stat.immediate_reuse += 1;
            }

            if self.feedback_fire() {
                self.stat.feedbacks += 1;
                if self.deallocate_fire() || self.addr_out_fire() {
                    self.stat.feedbacks_shadowed += 1;
                }
            }

            self.stat.m_request_per_cycle.push(self.mutator_request());

            self.stat.free_len.push(*self.reg_free_len.value());
            self.stat.work_len.push(*self.reg_work_len.value());
        }
    }

    fn tick_children(&mut self) {
        self.reg_collector.tick();
        self.gc_mem.tick();
        self.reg_free_head.tick();
        self.reg_work_head.tick();
        self.reg_free_drawed.tick();
        self.reg_work_drawed.tick();
        self.reg_free_len.tick();
        self.reg_work_len.tick();
        self.reg_sweeper.tick();
        self.reg_bk_reader.tick();
        self.reg_heap_reader.tick();
        self.reg_pre_gc.tick();
        self.reg_move.tick();
        self.reg_work_on.tick();
        self.reg_app_idx.tick();
        self.reg_monitors.tick();
        self.reg_monitor_idx.tick();
    }
}

#[test]
fn gc_spec_init() {
    let gc = GbgCollector::new(100, 8).init_freelist();
    println!("{:#?}", gc.gc_mem.ram);
}

#[test]
fn gc_spec_draw() {
    let size = 1024;
    let from = 42;
    let mut gc = GbgCollector::new(size, from).init_freelist();
    let mut it = from;
    use rand::Rng;
    let mut rng = rand::rng();

    gc.tick();
    while it < size {
        let draw: bool = rng.random();
        if draw {
            gc.input.addr_out_ready = true;
            assert_eq!(gc.addr_out_bits(), it);
            it = it + 1;
        } else {
            gc.input.addr_out_ready = false;
        }
        gc.tick();
    }
}
