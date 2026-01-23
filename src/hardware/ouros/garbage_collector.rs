use crate::hardware::common::{DualPortMem, Register};
use crate::hw_module::{HwInput, HwModule};

use super::config::{DLV_GC, GC_THRESHOLD, HEAP_SIZE};
use super::program::{get_ptr, is_ptr, App};

#[derive(Default, Clone, Debug, PartialEq)]
enum CollectorState {
    #[default]
    IDLE,
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

#[derive(Default)]
pub struct GbgCollectorInput {
    pub feedback_valid: bool,
    pub feedback_bits: usize,
    pub deallocate_valid: bool,
    pub deallocate_bits: usize,
    pub addr_out_ready: bool,
    pub heap_read_bits: App,
    pub heap_read_valid: bool,
}

impl HwInput for GbgCollectorInput {}

#[derive(Default)]
pub struct GbgCollectorStat {
    pub allocations: u32,
    pub feedbacks: u32,
    pub feedbacks_shadowed: u32,
    pub immediate_reuse: u32,           // gain from one-bit ref count
    pub m_request_per_cycle: Vec<bool>, // mutator request
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
    reg_work_on: Register<usize>, // to lock the worklist object we're working on
    reg_app_idx: Register<usize>,
    stat: GbgCollectorStat,
    stat_detail_lv: u8,
}

impl GbgCollector {
    pub fn new(heap_size: usize, free_from: usize) -> Self {
        Self {
            input: Default::default(),
            reg_collector: Default::default(),
            gc_mem: DualPortMem::new(heap_size),
            reg_free_head: Register::init(free_from),
            reg_work_head: Default::default(),
            reg_free_drawed: Default::default(),
            reg_work_drawed: Default::default(),
            reg_free_len: Register::init(HEAP_SIZE - free_from),
            reg_work_len: Default::default(),
            reg_sweeper: Default::default(),
            reg_bk_reader: Default::default(),
            reg_heap_reader: Default::default(),
            reg_pre_gc: Default::default(),
            reg_move: Default::default(),
            reg_work_on: Default::default(),
            reg_app_idx: Default::default(),
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

    fn push_to_freelist(&mut self, addr: usize, old_head: usize) {
        self.reg_free_head.connect(&addr);
        self.gc_mem.write_a(
            addr,
            GCCell {
                state: CellState::FreeList,
                ptr: old_head,
            },
        );
    }

    fn push_to_worklist(&mut self, addr: usize, old_head: usize) {
        self.reg_work_head.connect(&addr);
        self.gc_mem.write_b(
            addr,
            GCCell {
                state: CellState::FreeList,
                ptr: old_head,
            },
        );
    }

    /// draw an address from the freelist
    pub fn addr_out_bits(&self) -> usize {
        if self.deallocate_fire() {
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
    }

    pub fn deallocate_fire(&self) -> bool {
        self.input.deallocate_valid && self.deallocate_ready()
    }

    pub fn addr_out_valid(&self) -> bool {
        // FIXME track when FreeList is empty
        true
    }

    pub fn addr_out_fire(&self) -> bool {
        self.addr_out_valid() && self.input.addr_out_ready
    }

    pub fn get_stat(&self) -> &GbgCollectorStat {
        &self.stat
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
    fn more_ptr(&self, app: &App) -> bool {
        let idx = *self.reg_app_idx.value() + 1;
        app.iter().skip(idx as usize).any(|a| is_ptr(a))
    }

    /// stolen from `reducer.rs`
    fn find_ptr(&self, app: &App) -> usize {
        let idx = *self.reg_app_idx.value() + 1;
        idx + app.iter().skip(idx).position(|a| is_ptr(a)).unwrap()
    }

    fn step_idle(&mut self) {
        if !self.mutator_request() && *self.reg_free_len.value() <= GC_THRESHOLD {
            self.reg_collector.connect(&CollectorState::MARK);
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
            self.reg_move.connect(&0);
            self.reg_pre_gc.connect(&false);
        }
    }

    fn step_mark(&mut self) {
        // ========== defaults (update regs from previous demands) ==========
        self.reg_pre_gc.connect(&false);
        if *self.reg_pre_gc.value() {
            self.reg_bk_reader.connect(self.gc_mem.dout_a());
        }

        // ========== move-0 logic (pop a node from worklist) ==========
        if !self.mutator_request() && *self.reg_move.value() == 0 {
            if *self.reg_work_len.value() == 0 {
                self.reg_collector.connect(&CollectorState::SWEEP);
            } else {
                let w_head = *self.reg_work_head.value();
                self.gc_mem.write_a(
                    w_head,
                    GCCell {
                        state: CellState::Marked,
                        ptr: 0,
                    },
                );
                self.reg_work_drawed.connect(&true);
                self.work_len_minus_one();
                self.reg_work_on.connect(&w_head);
                self.reg_move.connect(&1);
                self.reg_app_idx.connect(&0);
            }
        }

        // ========== move-1 logic (wait until main heap read success) ==========
        if self.input.heap_read_valid && *self.reg_move.value() == 1 {
            let in_app = &self.input.heap_read_bits;
            // splition of move-0 and move-1 is needed because mutator can modify worklist
            if in_app.iter().any(|atm| is_ptr(atm)) {
                self.reg_heap_reader.connect(&in_app);
                self.reg_move.connect(&2);
                // pre read for move-2
                let found = in_app.iter().position(|atm| is_ptr(atm)).unwrap();
                self.gc_mem.read_a(get_ptr(&in_app[found]));
                self.reg_app_idx.connect(&found);
                self.reg_pre_gc.connect(&true);
            } else {
                self.reg_move.connect(&0);
            }
        }

        // ========== move-2 logic (push one, read one) ==========
        if !self.mutator_request() && *self.reg_move.value() == 2 {
            let read_out = if *self.reg_pre_gc.value() {
                self.gc_mem.dout_a()
            } else {
                self.reg_bk_reader.value()
            };
            let current_app = self.reg_heap_reader.value();
            if read_out.state == CellState::Unmarked {
                // push it to worklist
                self.push_to_worklist(
                    get_ptr(&current_app[*self.reg_app_idx.value()]),
                    *self.reg_work_head.value(),
                );
            }

            let current_app = self.reg_heap_reader.value();
            if self.more_ptr(current_app) {
                // read the next PTR
                let found = self.find_ptr(current_app);
                self.gc_mem.read_a(get_ptr(&current_app[found]));
                self.reg_app_idx.connect(&found);
                self.reg_pre_gc.connect(&true);
            } else {
                // go back to move-0, handle the next worklist node
                self.reg_move.connect(&0);
            }
        }
    }

    fn step_sweep(&mut self) {}
}

impl HwModule for GbgCollector {
    fn update_local(&mut self) {
        self.gc_mem.input.default_input();

        let real_freelist_head: usize = if *self.reg_free_drawed.value() {
            self.gc_mem.dout_a().ptr
        } else {
            *self.reg_free_head.value()
        };

        self.reg_free_drawed.connect(&false);
        self.reg_free_head.connect(&real_freelist_head);

        match (self.deallocate_fire(), self.addr_out_fire()) {
            (true, false) => {
                if *self.reg_collector.value() != CollectorState::MARK {
                    self.push_to_freelist(self.input.deallocate_bits, real_freelist_head);
                    self.reg_free_len.connect(&(self.reg_free_len.value() + 1));
                }
            }
            (false, true) => {
                self.reg_free_drawed.connect(&true);
                self.gc_mem.read_a(real_freelist_head); // NOTE it's still a free addr
                self.reg_free_len.connect(&(self.reg_free_len.value() - 1));
            }
            _ => {}
        }

        if self.feedback_fire() {
            let cell_state = match self.reg_collector.value() {
                CollectorState::IDLE => CellState::Unmarked,
                CollectorState::MARK => CellState::Marked,
                CollectorState::SWEEP => todo!(), // unswept ..
            };
            self.gc_mem.write_b(
                self.input.feedback_bits,
                GCCell {
                    state: cell_state,
                    ptr: 0,
                },
            );
        }

        match self.reg_collector.value() {
            CollectorState::IDLE => self.step_idle(),
            CollectorState::MARK => self.step_mark(),
            CollectorState::SWEEP => self.step_sweep(),
        }
    }

    fn update_stat(&mut self) {
        if self.stat_detail_lv >= DLV_GC {
            if self.deallocate_fire() {
                self.stat.immediate_reuse += 1;
            }

            if self.feedback_fire() {
                self.stat.feedbacks += 1;
                if self.deallocate_fire() || self.addr_out_fire() {
                    self.stat.feedbacks_shadowed += 1;
                }
            }

            if self.addr_out_fire() {
                self.stat.allocations += 1;
            }

            self.stat.m_request_per_cycle.push(self.mutator_request())
        }
    }

    fn tick_children(&mut self) {
        self.reg_collector.tick();
        self.gc_mem.tick();
        self.reg_free_head.tick();
        self.reg_work_head.tick();
        self.reg_free_drawed.tick();
        self.reg_free_len.tick();
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
