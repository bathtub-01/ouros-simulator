use crate::hardware::common::{DualPortMem, Register};
use crate::hw_module::{HwInput, HwModule};

#[derive(Default)]
enum CollectorState {
    #[default]
    IDLE,
    MARK,
    SWEEP,
}

#[derive(Default, Clone, Debug)]
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
    pub deallocate_valid: bool,
    pub deallocate_bits: usize,
    pub addr_out_ready: bool,
}

impl HwInput for GbgCollectorInput {}

#[derive(Default)]
pub struct GbgCollectorStat {
    pub immediate_reuse: u32, // gain from one-bit ref count
}

pub struct GbgCollector {
    pub input: GbgCollectorInput,
    pub gc_mem: DualPortMem<GCCell>,
    reg_free_head: Register<usize>,
    reg_work_head: Register<usize>,
    reg_addr_drawed: Register<bool>, // whether freelist was drawed in previous cycle
    stat: GbgCollectorStat,
}

impl GbgCollector {
    pub fn new(heap_size: usize, free_from: usize) -> Self {
        Self {
            input: Default::default(),
            gc_mem: DualPortMem::new(heap_size),
            reg_free_head: Register::init(free_from),
            reg_work_head: Default::default(),
            reg_addr_drawed: Default::default(),
            stat: Default::default(),
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

    /// draw an address from the freelist
    pub fn addr_out_bits(&self) -> usize {
        if self.deallocate_fire() {
            self.input.deallocate_bits
        } else {
            if *self.reg_addr_drawed.value() {
                self.gc_mem.dout_a().ptr
            } else {
                *self.reg_free_head.value()
            }
        }
    }

    pub fn deallocate_ready(&self) -> bool {
        true
    }

    pub fn deallocate_fire(&self) -> bool {
        self.input.deallocate_valid && self.deallocate_ready()
    }

    pub fn addr_out_valid(&self) -> bool {
        true
    }

    pub fn addr_out_fire(&self) -> bool {
        self.addr_out_valid() && self.input.addr_out_ready
    }

    pub fn get_stat(&self) -> &GbgCollectorStat {
        &self.stat
    }
}

impl HwModule for GbgCollector {
    fn update_local(&mut self) {
        self.gc_mem.input.default_input();

        let real_freelist_head: usize = if *self.reg_addr_drawed.value() {
            self.gc_mem.dout_a().ptr
        } else {
            *self.reg_free_head.value()
        };

        self.reg_addr_drawed.connect(&false);

        if *self.reg_addr_drawed.value() {
            self.reg_free_head.connect(&self.gc_mem.dout_a().ptr);
        }

        match (self.deallocate_fire(), self.addr_out_fire()) {
            (true, false) => {
                self.push_to_freelist(self.input.deallocate_bits, real_freelist_head);
            }
            (false, true) => {
                self.reg_addr_drawed.connect(&true);
                self.gc_mem.read_a(real_freelist_head);
            }
            _ => {}
        }
    }

    fn update_stat(&mut self) {
        if self.deallocate_fire() {
            self.stat.immediate_reuse += 1;
        }
    }

    fn tick_children(&mut self) {
        self.gc_mem.tick();
        self.reg_free_head.tick();
        self.reg_work_head.tick();
        self.reg_addr_drawed.tick();
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
