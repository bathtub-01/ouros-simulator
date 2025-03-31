// Dereference Heap, also handles thread management:
//           +-----------------+
// addr  <-->|    Dereference  |===> to_reducer
// port_a ==>|                 |
// port_b ==>|       Heap      |===> to_self
//           +-----------------+

use super::config::APP_LENGTH;
use super::program::{app_length, is_whnf, ActiveApp, App, Atom, FrozenApp, Program};
use crate::hardware::common::stack::StackOp;
use crate::hardware::common::{DualPortMem, Register, Stack};
use crate::hardware::utils::fire;
use crate::hw_module::{HwInput, HwModule};

#[derive(Default)]
pub struct DrfHeapInput {
    pub start: bool,
    pub port_a_valid: bool,
    pub port_a_bits: ActiveApp,
    pub port_b_valid: bool,
    pub port_b_bits: FrozenApp,
    pub to_reducer_ready: bool,
    pub to_self_ready: bool,
    pub addr_consumed: usize, // address request from reducer (for GC)
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
    IAw,
    OPa,
    OPaw,
    OPb,
    OPbw,
}

#[derive(Default, Clone)]
struct HeapCell {
    working: bool,
    stack_idx: u8,
    app: App,
}

#[derive(Default)]
pub struct DrfHeapStat {
    pub holder_contents: Vec<Option<App>>,
}

// TODO: enable stack depth configuration
pub struct DrfHeap {
    pub input: DrfHeapInput,
    stm: Register<Stm>,
    holder_in: ActiveApp,
    addr_holder: usize,
    thread_stack: [Stack<StackCell, 128>; 8],
    heap_mem: DualPortMem<HeapCell>,
    holder_out: (Dest, bool, ActiveApp), // output-reg: (destination, valid, app)
    working: Register<bool>,             // track whether the machine is working
    addr_bumper: Register<usize>,
    stat: DrfHeapStat,
}

impl DrfHeap {
    pub fn new(heap_size: usize) -> Self {
        Self {
            input: Default::default(),
            stm: Default::default(),
            holder_in: Default::default(),
            addr_holder: Default::default(),
            thread_stack: std::array::from_fn(|_| Stack::new()),
            heap_mem: DualPortMem::new(heap_size),
            holder_out: Default::default(),
            working: Default::default(),
            addr_bumper: Default::default(),
            stat: Default::default(),
        }
    }

    /// When creating a `DrfHeap`, put a compiled program into the heap memory.
    pub fn program(mut self, prog: &Program) -> Self {
        fn convert(atms: &Vec<Atom>) -> HeapCell {
            assert!(atms.len() <= APP_LENGTH);
            let mut app: App = std::array::from_fn(|_| Atom::NOP);
            for (i, atm) in atms.iter().enumerate() {
                app[i] = atm.clone();
            }
            HeapCell {
                working: false,
                stack_idx: 0,
                app,
            }
        }
        // convert Vec<Vec<Atom>> to Vec<HeapCell>
        let img: Vec<HeapCell> = prog.iter().map(convert).collect();
        self.heap_mem.image(&img);
        self.addr_bumper.connect(&img.len());
        self.addr_bumper.tick();
        self
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

    /// machine's work has been finished
    pub fn done(&self) -> bool {
        !self.working.value()
    }

    pub fn free_addr(&self) -> usize {
        *self.addr_bumper.value()
    }

    pub fn get_stat(&self) -> &DrfHeapStat {
        &self.stat
    }
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

        // rise addr bumper
        self.addr_bumper
            .connect(&(self.addr_bumper.value() + self.input.addr_consumed));

        // start the machine
        if !self.working.value() {
            if self.input.start {
                self.working.connect(&true);
                // push to stack
                self.thread_stack[0].input.link(|input| {
                    input.op = StackOp::PUSH;
                    input.din = 0;
                });
                // put output register
                let main = &self.heap_mem.dout_a().app;
                self.holder_out = (
                    which_dest(main),
                    true,
                    ActiveApp {
                        stack_idx: 0,
                        load: main.clone(),
                    },
                );
            }
            return;
        }
        // stop the machine when finished
        if self.port_a_fire() {
            // if its main in WHNF
            if self.thread_stack[0].elements() == 1
                && self.input.port_a_bits.stack_idx == 0
                && is_whnf(&self.input.port_a_bits.load)
            {
                self.working.connect(&false);
                return;
            }
        }

        // clear holder when output fires
        if self.output_fire() {
            self.holder_out.1 = false;
        }

        // handle port_a

        // return the index of a free stack, None if all stacks are in use
        let pick_stack = || -> Option<usize> {
            for (i, stk) in self.thread_stack.iter().enumerate() {
                if stk.elements() == 0 {
                    return Some(i);
                }
            }
            None
        };

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
                        match self.input.port_a_bits.load[0] {
                            Atom::PTR(p) => {
                                // read the target FIXME: refactor to change such things as functions
                                self.heap_mem.input.link(|input| {
                                    input.port_a.is_write = false;
                                    input.port_a.addr = p;
                                });
                                self.addr_holder = p;
                                // jump to next state
                                self.stm.connect(&Stm::IA);
                            }
                            Atom::PRM(_, _) => {
                                // (op a b)
                                // check `a`
                                match self.input.port_a_bits.load[1] {
                                    Atom::PTR(p) => {
                                        // read the target
                                        self.heap_mem.input.link(|input| {
                                            input.port_a.is_write = false;
                                            input.port_a.addr = p;
                                        });
                                        // OPa need this
                                        self.addr_holder = p;
                                        // jump to next state
                                        self.stm.connect(&Stm::OPa);
                                    }
                                    Atom::INT(_) => match self.input.port_a_bits.load[2] {
                                        Atom::PTR(p) => {
                                            // read the target
                                            self.heap_mem.input.link(|input| {
                                                input.port_a.is_write = false;
                                                input.port_a.addr = p;
                                            });
                                            // OPb need this
                                            self.addr_holder = p;
                                            // jump to next state
                                            self.stm.connect(&Stm::OPb);
                                        }
                                        _ => panic!("dheap: unknown PRM argument `b` type!"),
                                    },
                                    _ => panic!("dheap: unknown PRM argument `a` type!"),
                                }
                            }
                            _ => {
                                panic!("dheap: unknown input shape (redex should not enter dheap)!")
                            }
                        }
                    }
                }
            }
            Stm::WHNF => {
                let stk = &mut self.thread_stack[self.holder_in.stack_idx as usize];
                // TODO: handle multiple sharer case..

                // no more sharer
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
                        // put target in holder_in, for next cycle's writing
                        let target = &self.heap_mem.dout_a().app;
                        self.holder_in = ActiveApp {
                            stack_idx: self.holder_in.stack_idx,
                            load: target.clone(),
                        };
                        // put output register
                        self.holder_out = (
                            which_dest(target),
                            true,
                            ActiveApp {
                                stack_idx: self.holder_in.stack_idx,
                                load: target.clone(),
                            },
                        );
                        // jump to next state
                        self.stm.connect(&Stm::IAw);
                    }
                }
            }
            Stm::IAw => {
                // FIXME: it's possible to save this state by seperating meta info
                // into a different memory.

                // write the updated target info.
                self.heap_mem.input.link(|input| {
                    input.port_a.is_write = true;
                    input.port_a.addr = self.addr_holder;
                    input.port_a.din = HeapCell {
                        working: true,
                        stack_idx: self.holder_in.stack_idx,
                        app: self.holder_in.load.clone(),
                    };
                });
                // jump to next state
                self.stm.connect(&Stm::IDLE);
            }
            Stm::OPa => {
                let target = &self.heap_mem.dout_a().app;
                if is_whnf(target) {
                    // in this case, `a` should be an Int
                    let a = {
                        match target[0] {
                            Atom::INT(i) => i,
                            _ => panic!("dheap: PRM argument `a` must be an Int!"),
                        }
                    };
                    // check `b`
                    match self.holder_in.load[2] {
                        Atom::PTR(p) => {
                            // read the target
                            self.heap_mem.input.link(|input| {
                                input.port_a.is_write = false;
                                input.port_a.addr = p;
                            });
                            self.holder_in.load[1] = Atom::INT(a);
                            self.addr_holder = p;
                            // jump to next state
                            self.stm.connect(&Stm::OPb);
                        }
                        Atom::INT(b) => {
                            // put output register
                            self.holder_out = (
                                Dest::ToReducer,
                                true,
                                ActiveApp {
                                    stack_idx: self.holder_in.stack_idx,
                                    load: {
                                        let mut res: App = Default::default();
                                        res[0] = self.holder_in.load[0].clone();
                                        res[1] = Atom::INT(a);
                                        res[2] = Atom::INT(b);
                                        res
                                    },
                                },
                            );
                            // jump to next state
                            self.stm.connect(&Stm::IDLE);
                        }
                        _ => panic!("dheap: unknown PRM argument `b` type!"),
                    }
                } else {
                    // `a` not in WHNF, emit `a`

                    let target = self.heap_mem.dout_a().app.clone();
                    // write the updated target info.
                    self.heap_mem.input.link(|input| {
                        input.port_a.is_write = true;
                        input.port_a.addr = self.addr_holder;
                        input.port_a.din = HeapCell {
                            working: true,
                            stack_idx: self.holder_in.stack_idx,
                            app: target.clone(),
                        };
                    });

                    // push `a` to the stack
                    self.thread_stack[self.holder_in.stack_idx as usize]
                        .input
                        .link(|input| {
                            input.op = StackOp::PUSH;
                            input.din = self.addr_holder;
                        });

                    // put register
                    self.holder_out = (
                        Dest::ToReducer,
                        true,
                        ActiveApp {
                            stack_idx: self.holder_in.stack_idx,
                            load: target,
                        },
                    );

                    // jump to next state
                    self.stm.connect(&Stm::OPaw);
                }
            }
            Stm::OPaw => {
                match self.holder_in.load[2] {
                    Atom::PTR(p) => {
                        // read the target
                        self.heap_mem.input.link(|input| {
                            input.port_a.is_write = false;
                            input.port_a.addr = p;
                        });
                        self.addr_holder = p;
                        // jump to next state
                        self.stm.connect(&Stm::OPb);
                    }
                    Atom::INT(_) => {
                        // store the demander
                        self.heap_mem.input.link(|input| {
                            input.port_a.is_write = true;
                            match self.thread_stack[self.holder_in.stack_idx as usize].second() {
                                Some(addr) => input.port_a.addr = *addr,
                                None => panic!("dheap: ill stack!"),
                            }
                            input.port_a.din = HeapCell {
                                working: true,
                                stack_idx: self.holder_in.stack_idx,
                                app: self.holder_in.load.clone(),
                            };
                        });

                        // jump to next state
                        self.stm.connect(&Stm::IDLE);
                    }
                    _ => panic!("dheap: unknown PRM argument `b` type!"),
                }
            }
            Stm::OPb => {
                let target = &self.heap_mem.dout_a().app;
                if is_whnf(target) {
                    // in this case, `b` should be an Int
                    let b = {
                        match target[0] {
                            Atom::INT(i) => i,
                            _ => panic!("dheap: PRM argument `b` must be an Int!"),
                        }
                    };
                    self.holder_in.load[2] = Atom::INT(b);

                    // put register
                    self.holder_out = (
                        Dest::ToReducer,
                        true,
                        ActiveApp {
                            stack_idx: self.holder_in.stack_idx,
                            load: self.holder_in.load.clone(),
                        },
                    );

                    // jump to next state
                    self.stm.connect(&Stm::IDLE);
                } else if self.output_fire() || !self.holder_out.1 {
                    // whether to spark a new thread for `b`
                    let target = self.heap_mem.dout_a().app.clone();
                    match self.holder_in.load[1] {
                        Atom::INT(_) => {
                            // create a thread for `b` in-place

                            // write the updated target info.
                            self.heap_mem.input.link(|input| {
                                input.port_a.is_write = true;
                                input.port_a.addr = self.addr_holder;
                                input.port_a.din = HeapCell {
                                    working: true,
                                    stack_idx: self.holder_in.stack_idx,
                                    app: target.clone(),
                                };
                            });

                            // push `b` to the stack
                            self.thread_stack[self.holder_in.stack_idx as usize]
                                .input
                                .link(|input| {
                                    input.op = StackOp::PUSH;
                                    input.din = self.addr_holder;
                                });

                            // put register
                            self.holder_out = (
                                Dest::ToReducer,
                                true,
                                ActiveApp {
                                    stack_idx: self.holder_in.stack_idx,
                                    load: target,
                                },
                            );

                            // jump to next state
                            self.stm.connect(&Stm::OPbw);
                        }
                        _ => {
                            // spark a new thread for `b`

                            match pick_stack() {
                                Some(stk_idx) => {
                                    // spark a new thread on an empty stack

                                    // write the updated target info.
                                    self.heap_mem.input.link(|input| {
                                        input.port_a.is_write = true;
                                        input.port_a.addr = self.addr_holder;
                                        input.port_a.din = HeapCell {
                                            working: true,
                                            stack_idx: stk_idx as u8,
                                            app: target.clone(),
                                        };
                                    });

                                    // push the stack
                                    self.thread_stack[stk_idx].input.link(|input| {
                                        input.op = StackOp::PUSH;
                                        input.din = self.addr_holder;
                                    });

                                    // put register
                                    self.holder_out = (
                                        Dest::ToReducer,
                                        true,
                                        ActiveApp {
                                            stack_idx: stk_idx as u8,
                                            load: target,
                                        },
                                    );

                                    // jump to next state
                                    self.stm.connect(&Stm::OPbw);
                                }
                                None => {
                                    // unable to spark a new thread, back to IDLE

                                    // store the demander
                                    self.heap_mem.input.link(|input| {
                                        input.port_a.is_write = true;
                                        match self.thread_stack[self.holder_in.stack_idx as usize]
                                            .second()
                                        {
                                            Some(addr) => input.port_a.addr = *addr,
                                            None => panic!("dheap: ill stack!"),
                                        }
                                        input.port_a.din = HeapCell {
                                            working: true,
                                            stack_idx: self.holder_in.stack_idx,
                                            app: self.holder_in.load.clone(),
                                        };
                                    });

                                    // jump to next state
                                    self.stm.connect(&Stm::IDLE);
                                }
                            }
                        }
                    }
                }
            }
            Stm::OPbw => {
                // store the demander
                self.heap_mem.input.link(|input| {
                    input.port_a.is_write = true;
                    match self.thread_stack[self.holder_in.stack_idx as usize].second() {
                        Some(addr) => input.port_a.addr = *addr,
                        None => panic!("dheap: ill stack!"),
                    }
                    input.port_a.din = HeapCell {
                        working: true,
                        stack_idx: self.holder_in.stack_idx,
                        app: self.holder_in.load.clone(),
                    };
                });

                // jump to next state
                self.stm.connect(&Stm::IDLE);
            }
        }

        // handle port_b
        // TODO: combinatory check when an app in demand in entering port_b
        // read-after-write is good, but what if we are reading something
        // that is not yet enter the heap?
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

    fn update_stat(&mut self) {
        if self.holder_out.1 {
            self.stat
                .holder_contents
                .push(Some(self.holder_out.2.load.clone()));
        } else {
            self.stat.holder_contents.push(None);
        }
    }

    fn tick_children(&mut self) {
        self.stm.tick();
        for stk in &mut self.thread_stack {
            stk.tick();
        }
        self.heap_mem.tick();
        self.working.tick();
        self.addr_bumper.tick();
    }
}
