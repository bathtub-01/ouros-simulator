// Dereference Heap, also handles thread management:
//           +-----------------+
// addr  <-->|    Dereference  |===> to_reducer
// port_a ==>|                 |
// port_b ==>|       Heap      |===> to_self
//           +-----------------+

use crate::hardware::common::stack::StackOp;
use crate::hardware::common::{DualPortMem, Register, Stack};
use crate::hardware::ouros::config::{APP_LENGTH, HOLES};
use crate::hardware::ouros::program::{ActiveApp, App, Atom, FrozenApp};
use crate::hardware::utils::fire;
use crate::hw_module::{HwInput, HwModule};

#[derive(Default)]
struct DrfHeapInput {
    port_a_valid: bool,
    port_a_bits: ActiveApp,
    port_b_valid: bool,
    port_b_bits: FrozenApp,
    to_reducer_ready: bool,
    to_self_ready: bool,
    // address request from reducer (for GC)
    addr_consumed: usize,
}

impl HwInput for DrfHeapInput {}

type StackCell = usize;

#[derive(Default)]
enum Dest {
    #[default]
    ToReducer,
    ToSelf,
}

#[derive(Default, Clone, PartialEq)]
enum Stm {
    #[default]
    IDLE,
    WHNF,
    IA,
}

#[derive(Default, Clone)]
struct HeapCell {
    working: bool,
    stack_idx: u8,
    app: App,
}

pub struct DrfHeap {
    input: DrfHeapInput,
    stm: Register<Stm>,
    holder_in: ActiveApp,
    thread_stack: [Stack<StackCell, 128>; 8],
    heap_mem: DualPortMem<HeapCell>,
    holder_out: (Dest, bool, ActiveApp), // output-reg: (destination, valid, app)
}

impl DrfHeap {
    pub fn new(heap_size: usize) -> Self {
        Self {
            input: Default::default(),
            stm: Default::default(),
            holder_in: Default::default(),
            thread_stack: std::array::from_fn(|_| Stack::new()),
            heap_mem: DualPortMem::new(heap_size),
            holder_out: Default::default(),
        }
    }

    fn port_a_fire(&self) -> bool {
        fire(self.input.port_a_valid, self.port_a_ready())
    }

    fn output_fire(&self) -> bool {
        fire(self.to_reducer_valid(), self.input.to_reducer_ready)
            || fire(self.to_self_valid(), self.input.to_self_ready)
    }

    pub fn port_a_ready(&self) -> bool {
        // port_a is ready to handle a new application in this cycle
        !self.holder_out.1 || self.output_fire()
    }

    pub fn port_b_ready(&self) -> bool {
        // FIXME
        true
    }

    pub fn to_reducer_valid(&self) -> bool {
        match self.holder_out.0 {
            Dest::ToReducer => self.holder_out.1,
            Dest::ToSelf => false,
        }
    }

    pub fn to_reducer_bits(&self) -> &ActiveApp {
        &self.holder_out.2
    }

    pub fn to_self_valid(&self) -> bool {
        match self.holder_out.0 {
            Dest::ToReducer => false,
            Dest::ToSelf => self.holder_out.1,
        }
    }

    pub fn to_self_bits(&self) -> &ActiveApp {
        &self.holder_out.2
    }
}

fn arity_of(atom: &Atom) -> u8 {
    use Atom::*;
    match atom {
        COM(a, _, _) => *a,
        PRM(_) => 2,
        Y => 1,
        _ => 0,
    }
}

/// The length of an application, stripping off NOPs.
fn app_length(app: &App) -> usize {
    let found = app.iter().enumerate().find(|&(_, atom)| *atom == Atom::NOP);
    match found {
        Some((idx, _)) => idx,
        None => APP_LENGTH,
    }
}

#[test]
fn app_length_spec() {
    use Atom::*;
    let a: App = [PTR(0), INT(1), INT(2), INT(3), NOP, NOP, NOP, NOP];
    let b: App = [Y, Y, Y, Y, Y, Y, Y, Y];
    assert_eq!(app_length(&a), 4);
    assert_eq!(app_length(&b), APP_LENGTH);
}

/// Determine whether an Application is in Weak-Head-Normal-Form.
fn is_whnf(app: &App) -> bool {
    // +, a, b --- false
    // +, a    --- true
    arity_of(&app[0]) >= app_length(app) as u8
}

/// Determine the destination of an output application
fn which_dest(app: &App) -> Dest {
    if is_whnf(app) {
        Dest::ToSelf
    } else {
        match app[0] {
            Atom::PTR(_) => Dest::ToSelf,
            _ => Dest::ToReducer,
        }
    }
}

/// Dereference app's head pointer
fn deref(app: &App, target: &App) -> App {
    match app[0] {
        Atom::PTR(_) => {
            let app_len = app_length(app);
            let target_len = app_length(target);
            if app_len + target_len - 1 > APP_LENGTH {
                panic!("dheap: deref: deref result too long!");
            }
            let mut res: App = std::array::from_fn(|_| Atom::NOP);
            for i in 0..target_len {
                res[i] = target[i].clone();
            }
            for i in 1..app_len {
                res[target_len + i - 1] = app[i].clone();
            }
            res
        }
        _ => panic!("dheap: deref: app head is not a PTR!"),
    }
}

#[test]
fn deref_spec() {
    use Atom::*;
    let app: App = [PTR(0), INT(1), INT(2), INT(3), NOP, NOP, NOP, NOP];
    let target1: App = [PTR(11), PTR(22), PTR(33), PTR(44), PTR(55), NOP, NOP, NOP];
    let res1: App = [
        PTR(11),
        PTR(22),
        PTR(33),
        PTR(44),
        PTR(55),
        INT(1),
        INT(2),
        INT(3),
    ];
    let target2: App = [PTR(11), PTR(22), PTR(33), NOP, NOP, NOP, NOP, NOP];
    let res2: App = [PTR(11), PTR(22), PTR(33), INT(1), INT(2), INT(3), NOP, NOP];
    assert_eq!(deref(&app, &target1), res1);
    assert_eq!(deref(&app, &target2), res2);
}

impl HwModule for DrfHeap {
    fn update_local(&mut self) {
        // always give default inputs at the beginning of a cycle
        self.heap_mem.input.default_input();
        for stk in &mut self.thread_stack {
            stk.input.default_input();
        }

        // handle port_a

        // handle new input based on its shape, and jump to next state
        // fn handle_new_input(h: &mut DrfHeap) {
        //     h.stm.connect(&Stm::IDLE);
        // }
        match *self.stm.value() {
            Stm::IDLE => {
                if self.port_a_fire() {
                    // handle new input
                    let stk = &mut self.thread_stack[self.input.port_a_bits.stack_idx as usize];
                    // put input into holder
                    self.holder_in = self.input.port_a_bits.clone();
                    if is_whnf(&self.input.port_a_bits.load) {
                        // read return app
                        self.heap_mem.input.link(|input| {
                            input.port_a.is_write = false;
                            match stk.second() {
                                None => panic!("dheap: thread stack error!"),
                                Some(addr) => {
                                    input.port_a.addr = *addr;
                                }
                            }
                        });
                        // jump to next state
                        self.stm.connect(&Stm::WHNF);
                    } else {
                        // read the target
                        self.heap_mem.input.link(|input| {
                            input.port_a.is_write = false;
                            match self.input.port_a_bits.load[0] {
                                Atom::PTR(p) => {
                                    input.port_a.addr = p;
                                }
                                _ => panic!("dheap: unknown IA (redex should not enter dheap)!"),
                            }
                        });
                        // jump to next state
                        self.stm.connect(&Stm::IA);
                    }
                }
            }
            Stm::WHNF => {
                let stk = &mut self.thread_stack[self.holder_in.stack_idx as usize];
                // TODO: handle multiple sharer case..

                // no more sharer
                // FIXME: not deref yet..
                let target = &self.holder_in.load;
                // write incoming WHNF
                self.heap_mem.input.link(|input| {
                    input.port_a.is_write = true;
                    match stk.top() {
                        None => panic!("dheap: thread stack error!"),
                        Some(addr) => {
                            input.port_a.addr = *addr;
                        }
                    }
                    input.port_a.din = HeapCell {
                        working: false, // already in WHNF
                        stack_idx: self.holder_in.stack_idx,
                        app: target.clone(),
                    };
                });
                // pop the stack
                stk.input.link(|input| {
                    input.op = StackOp::POP;
                });
                // put output register
                let demander = &self.heap_mem.dout_a().app;
                let deref_res = deref(demander, target);
                self.holder_out = (
                    which_dest(&deref_res),
                    true,
                    ActiveApp {
                        stack_idx: self.heap_mem.dout_a().stack_idx,
                        load: deref_res,
                    },
                );
                // jump to next state
                self.stm.connect(&Stm::IDLE);
            }
            Stm::IA => {
                let target = &self.heap_mem.dout_a().app;
                let demander = &self.holder_in.load;
                if is_whnf(target) {
                    // put output register
                    let deref_res = deref(demander, target);
                    self.holder_out = (
                        which_dest(&deref_res),
                        true,
                        ActiveApp {
                            stack_idx: self.holder_in.stack_idx,
                            load: deref_res,
                        },
                    );
                    // jump to next state
                    self.stm.connect(&Stm::IDLE);
                } else {
                    let stk = &mut self.thread_stack[self.holder_in.stack_idx as usize];
                    if self.heap_mem.dout_a().working {
                        // TODO: handle multiple sharer case (target in computation)..
                        panic!("not impl yet!");
                    } else {
                        // target is fresh

                        // FIXME: also need to write the control info of the target
                        // write incoming demander (suspend it)
                        self.heap_mem.input.link(|input| {
                            input.port_a.is_write = true;
                            match stk.top() {
                                None => panic!("dheap: thread stack error!"),
                                Some(addr) => {
                                    input.port_a.addr = *addr;
                                }
                            }
                            input.port_a.din = HeapCell {
                                working: true,
                                stack_idx: self.holder_in.stack_idx,
                                app: demander.clone(),
                            };
                        });
                        // push target to the stack (start new thread)
                        stk.input.link(|input| {
                            input.op = StackOp::PUSH;
                            match demander[0] {
                                Atom::PTR(p) => {
                                    input.din = p;
                                }
                                _ => panic!("dheap: demander head should be a PTR!"),
                            }
                        });
                        // put output register
                        let target = &self.heap_mem.dout_a().app;
                        self.holder_out = (
                            which_dest(target),
                            true,
                            ActiveApp {
                                stack_idx: self.holder_in.stack_idx,
                                load: target.clone(),
                            },
                        );
                        // jump to next state
                        self.stm.connect(&Stm::IDLE);
                    }
                }
            }
        }

        if self.output_fire() {
            self.holder_out.1 = false;
        }

        // handle port_b
        // TODO: combinatory check when an app in demand in entering port_b
        // always write frozen applications
        if fire(self.input.port_b_valid, self.port_b_ready()) {
            self.heap_mem.input.link(|input| {
                input.port_b.is_write = true;
                input.port_b.addr = self.input.port_b_bits.heap_addr;
                input.port_b.din = HeapCell {
                    working: false,
                    stack_idx: 0,
                    app: {
                        let mut extended: App = std::array::from_fn(|_| Atom::NOP);
                        for (i, a) in self.input.port_b_bits.load.iter().enumerate() {
                            extended[i] = a.clone();
                        }
                        extended
                    },
                };
            });
        }
    }

    fn tick_children(&mut self) {
        self.stm.tick();
        for stk in &mut self.thread_stack {
            stk.tick();
        }
        self.heap_mem.tick();
    }
}
