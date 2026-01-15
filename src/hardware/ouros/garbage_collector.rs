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
    pub addr_out_ready: bool,
}

impl HwInput for GbgCollectorInput {}

pub struct GbgCollector {
    pub input: GbgCollectorInput,
    pub gc_mem: DualPortMem<GCCell>,
    reg_free_head: Register<usize>,
    reg_work_head: Register<usize>,
    reg_addr_drawed: Register<bool>, // whether freelist was drawed in previous cycle
}

impl GbgCollector {
    pub fn new(heap_size: usize, free_from: usize) -> Self {
        Self {
            input: Default::default(),
            gc_mem: DualPortMem::new(heap_size),
            reg_free_head: Register::init(free_from),
            reg_work_head: Default::default(),
            reg_addr_drawed: Default::default(),
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

    /// draw an address from the freelist
    pub fn addr_out_bits(&self) -> usize {
        if *self.reg_addr_drawed.value() {
            self.gc_mem.dout_a().ptr
        } else {
            *self.reg_free_head.value()
        }
    }

    pub fn addr_out_valid(&self) -> bool {
        true
    }

    pub fn addr_out_fire(&self) -> bool {
        self.addr_out_valid() && self.input.addr_out_ready
    }
}

impl HwModule for GbgCollector {
    fn update_local(&mut self) {
        self.reg_addr_drawed.connect(&self.addr_out_fire());
        if self.addr_out_fire() {
            let next: usize;
            if *self.reg_addr_drawed.value() {
                next = self.gc_mem.dout_a().ptr;
            } else {
                next = *self.reg_free_head.value();
            }
            self.gc_mem.read_a(next);
        }
        if *self.reg_addr_drawed.value() {
            self.reg_free_head.connect(&self.gc_mem.dout_a().ptr);
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
