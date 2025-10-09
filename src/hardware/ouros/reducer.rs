// Reducer for combinators:
//          +-----------------+===> spine
// addr <-->| >               |
// input ==>|     Reducer     |
//          |                 |
//          +-----------------+===> app

use super::program::*;
use crate::hardware::common::{Register, SinglePortMem};
use crate::hardware::ouros::combinator::{parse_pat, Hole, ParseRes, ALL_PATTERNS, DECODE_TABLE};
use crate::hardware::ouros::config::*;
use crate::hardware::ouros::program::{ActiveApp, App, Atom, FrozenApp};
use crate::hardware::utils::fire;
use crate::hw_module::{HwInput, HwModule};

fn dash_atom(atom: &Atom) -> Atom {
    match atom {
        Atom::PTR(p, _, false) => Atom::PTR(*p, false, false),
        _ => atom.clone(),
    }
}

#[derive(Default, Clone, PartialEq, Debug)]
enum Stm {
    #[default]
    IDLE,
    SPINE,
    APP,
}

#[derive(Default)]
pub struct ReducerInput {
    pub in_valid: bool,
    pub in_app: ActiveApp,
    pub spine_ready: bool,
    pub app_ready: bool,
    pub free_addr: usize, // receive free address from GC
}

impl HwInput for ReducerInput {}

#[derive(Default)]
pub struct ReducerStat {
    pub busy_cycles: u32,
    pub busy_per_cycle: Vec<bool>,
    pub holder_contents: Vec<Option<ActiveApp>>,
    pub blocked_cycles: u32,
}

pub struct Reducer {
    pub input: ReducerInput,
    comb_table: SinglePortMem<App>,
    reg_in: Register<ActiveApp>,
    reg_addr: Register<usize>,
    reg_spine: Register<App>,
    reg_stm: Register<Stm>,
    reg_idx: Register<usize>,
    reg_ctr: Register<usize>,
    stat: ReducerStat,
    stat_detail_lv: u8,
}

impl Reducer {
    pub fn new(size: usize) -> Self {
        Self {
            input: ReducerInput {
                in_valid: false,
                in_app: Default::default(),
                spine_ready: false,
                app_ready: false,
                free_addr: Default::default(),
            },
            comb_table: SinglePortMem::new(size),
            stat: Default::default(),
            stat_detail_lv: Default::default(),
            reg_in: Default::default(),
            reg_addr: Default::default(),
            reg_spine: Default::default(),
            reg_stm: Default::default(),
            reg_idx: Default::default(),
            reg_ctr: Default::default(),
        }
    }

    pub fn program(mut self, prog: &Vec<Vec<Atom>>) -> Self {
        fn convert(atms: &Vec<Atom>) -> App {
            assert!(atms.len() <= APP_LENGTH);
            let mut app: App = std::array::from_fn(|_| Atom::NOP);
            for (i, atm) in atms.iter().enumerate() {
                app[i] = atm.clone();
            }
            app
        }
        let img = prog.iter().map(convert).collect();
        self.comb_table.image(&img);
        self
    }

    pub fn detail(mut self, lv: u8) -> Self {
        self.stat_detail_lv = lv;
        self
    }

    pub fn spine_valid(&self) -> bool {
        *self.reg_stm.value() == Stm::SPINE
    }

    pub fn spine_bits(&self) -> ActiveApp {
        ActiveApp {
            stack_idx: self.reg_in.value().stack_idx,
            load: {
                let old_spn = &self.reg_in.value().load;
                let before = arity_of(&old_spn[0]) as usize + 1;
                let mut res = self.inst(&self.comb_table.dout());
                let after = app_length(&res);
                assert!(
                    old_spn[before..].iter().filter(|a| !is_nop(a)).count() + after <= APP_LENGTH
                );
                for i in 0..(APP_LENGTH - before) {
                    if after + i < APP_LENGTH {
                        res[after + i] = old_spn[before + i].clone();
                    }
                }
                res
            },
        }
    }

    pub fn app_valid(&self) -> bool {
        *self.reg_stm.value() == Stm::APP
    }

    pub fn app_bits(&self) -> FrozenApp {
        FrozenApp {
            heap_addr: *self.reg_addr.value() + *self.reg_ctr.value() - {
                if self.app_valid() {
                    // only to satify Rust..
                    1
                } else {
                    0
                }
            },
            load: self.inst(&self.comb_table.dout()),
        }
    }

    pub fn in_ready(&self) -> bool {
        match *self.reg_stm.value() {
            Stm::IDLE => true,
            Stm::SPINE => !self.more_app(self.comb_table.dout()),
            Stm::APP => {
                fire(self.input.app_ready, self.app_valid())
                    && !self.more_app(self.reg_spine.value())
            }
        }
    }

    /// The number of heap cells that will be consumed in this cycle
    pub fn addr_consumed(&self) -> usize {
        if *self.reg_stm.value() == Stm::SPINE {
            self.comb_table
                .dout()
                .iter()
                .filter(|a| match a {
                    Atom::PTR(_, _, new) => *new,
                    _ => false,
                })
                .count()
        } else {
            0
        }
    }

    pub fn get_stat(&self) -> &ReducerStat {
        &self.stat
    }

    pub fn in_fire(&self) -> bool {
        fire(self.input.in_valid, self.in_ready())
    }

    fn more_app(&self, app: &App) -> bool {
        let idx = *self.reg_idx.value() + 1;
        app.iter().skip(idx as usize).any(is_new)
    }

    fn find_app(&self, app: &App) -> usize {
        let idx = *self.reg_idx.value() + 1;
        // x x x o * o o o
        // 0 1 2 3 4 5 6 7
        idx + app.iter().skip(idx).position(is_new).unwrap()
    }

    /// instantiate an app from template
    fn inst(&self, app: &App) -> App {
        let mut res: App = app.clone();
        for a in &mut res {
            match a {
                Atom::PTR(p, _, new) => {
                    if *new {
                        // assert_eq!(*self.reg_stm.value(), Stm::SPINE);
                        *a = Atom::PTR(self.input.free_addr + *p, true, false);
                    }
                }
                Atom::ARG(arg) => *a = self.reg_in.value().load[*arg + 1].clone(),
                _ => {}
            }
        }
        res
    }

    /*
        fn gen_result(
            &self,
        ) -> (
            (bool, ActiveApp),
            (bool, FrozenApp),
            (bool, FrozenApp),
            (bool, FrozenApp),
        ) {
            let mut res_spine: (bool, ActiveApp) = Default::default();
            let mut res_app1: (bool, FrozenApp) = Default::default();
            let mut res_app2: (bool, FrozenApp) = Default::default();
            let mut res_app3: (bool, FrozenApp) = Default::default();

            match self.input.in_app.load[0] {
                Atom::COM(arity, code, is) => {
                    let res = &self.decode_table[code as usize];
                    let in_app = &self.input.in_app;
                    let trans = |h: &Hole| match h {
                        Hole::Arg(a) => in_app.load[is[*a as usize] as usize + 1].clone(),
                        // newly created PTRs are unique by default
                        Hole::Ptr(p) => Atom::PTR(*p as usize + self.input.free_addr, true),
                    };
                    let gen_res = |v: &Vec<Hole>, a: &mut [Atom]| {
                        for i in 0..a.len() {
                            if i < v.len() {
                                a[i] = trans(&v[i]);
                            } else {
                                a[i] = Atom::NOP;
                            }
                        }
                    };
                    // perform reduction
                    gen_res(&res.spine, &mut res_spine.1.load);
                    gen_res(&res.app1, &mut res_app1.1.load);
                    gen_res(&res.app2, &mut res_app2.1.load);
                    gen_res(&res.app3, &mut res_app3.1.load);
                    // handle 1-bit ref counting
                    let mut temp_res: Vec<Atom> = Vec::new();
                    temp_res.extend_from_slice(&res_spine.1.load);
                    temp_res.extend_from_slice(&res_app1.1.load);
                    temp_res.extend_from_slice(&res_app2.1.load);
                    temp_res.extend_from_slice(&res_app3.1.load);
                    let set_flag = |arr: &mut [Atom]| {
                        for atom in arr.iter_mut() {
                            match atom {
                                Atom::PTR(p, _) => {
                                    let same_ptrs = temp_res
                                        .iter()
                                        .filter(|atm| match atm {
                                            Atom::PTR(pp, _) => p == pp,
                                            _ => false,
                                        })
                                        .count();
                                    if same_ptrs > 1 {
                                        *atom = Atom::PTR(*p, false);
                                    }
                                }
                                _ => {}
                            }
                        }
                    };
                    set_flag(&mut res_spine.1.load);
                    set_flag(&mut res_app1.1.load);
                    set_flag(&mut res_app2.1.load);
                    set_flag(&mut res_app3.1.load);
                    // append the spine for over-applied cases:
                    // e.g., S a b c x y = a c (b c) x y
                    let before = arity as usize + 1;
                    let after = res.spine.len();
                    for i in 0..(APP_LENGTH - before) {
                        if after + i < APP_LENGTH {
                            res_spine.1.load[after + i] = in_app.load[before + i].clone();
                        } else if before + i < APP_LENGTH && in_app.load[before + i] != Atom::NOP {
                            // over-sized result will be a runtime error..
                            panic!("reducer: over-sized over-applied app!");
                        } else {
                            break;
                        }
                    }
                    // pass the stack idx for the spine; heap addr for nested apps
                    res_spine.1.stack_idx = in_app.stack_idx;
                    res_app1.1.heap_addr = self.input.free_addr;
                    res_app2.1.heap_addr = self.input.free_addr + 1;
                    res_app3.1.heap_addr = self.input.free_addr + 2;
                }
                Atom::Y => {
                    let in_app = &self.input.in_app;
                    let mut spine_app: App = self.input.in_app.load.clone();
                    let mut app1_app: [Atom; HOLES - 1] = Default::default();

                    spine_app[0] = dash_atom(&in_app.load[1]);
                    spine_app[1] = Atom::PTR(self.input.free_addr, false);
                    app1_app[0] = dash_atom(&in_app.load[1]);
                    app1_app[1] = Atom::PTR(self.input.free_addr, false);

                    res_spine.1.load = spine_app;
                    res_app1.1.load = app1_app;

                    res_spine.1.stack_idx = in_app.stack_idx;
                    res_app1.1.heap_addr = self.input.free_addr;
                }
                Atom::SEQ(true) => {
                    let in_app = &self.input.in_app;
                    let mut spine_app: App = Default::default();
                    for (i, atom) in in_app.load.iter().skip(2).enumerate() {
                        spine_app[i] = atom.clone();
                    }
                    res_spine.1.load = spine_app;
                    res_spine.1.stack_idx = in_app.stack_idx;
                }
                _ => { /* vacancy for non-busy case  */ }
            }

            if REDUCER_PIPE {
                res_spine.0 = self.in_fire();
                res_app1.0 = !is_nop(&res_app1.1.load[0]);
                res_app2.0 = !is_nop(&res_app2.1.load[0]);
                res_app3.0 = !is_nop(&res_app3.1.load[0]);
            } else {
                res_spine.0 = self.input.in_valid
                    && self.input.app1_ready
                    && self.input.app2_ready
                    && self.input.app3_ready;
                res_app1.0 = self.in_fire() && !is_nop(&res_app1.1.load[0]);
                res_app2.0 = self.in_fire() && !is_nop(&res_app2.1.load[0]);
                res_app3.0 = self.in_fire() && !is_nop(&res_app3.1.load[0]);
            }

            (res_spine, res_app1, res_app2, res_app3)
    }
        */

    fn step_next(&mut self) {
        if self.in_fire() {
            self.reg_stm.connect(&Stm::SPINE);
            self.reg_in.connect(&self.input.in_app);
            self.reg_idx.connect(&0);
            self.reg_ctr.connect(&0);
            self.reg_addr.connect(&self.input.free_addr);
            match self.input.in_app.load[0] {
                Atom::COM(_, addr) => self.comb_table.read(addr),
                _ => todo!(),
            };
        } else {
            self.reg_stm.connect(&Stm::IDLE);
        }
    }
}

impl HwModule for Reducer {
    fn update_local(&mut self) {
        self.comb_table.input.default_input();
        match *self.reg_stm.value() {
            Stm::IDLE => {
                self.step_next();
            }
            Stm::SPINE => {
                // !NOTICE! We are assuming this state won't be blocked.
                let template = self.comb_table.dout();
                if self.more_app(template) {
                    let founded = self.find_app(template);
                    self.reg_spine.connect(template);
                    self.reg_stm.connect(&Stm::APP);
                    self.reg_idx.connect(&founded);
                    self.reg_ctr.connect(&1);
                    self.comb_table.read(
                        get_comb_addr(&self.reg_in.value().load[0])
                            + get_ptr(&template[founded])
                            + 1,
                    );
                } else {
                    self.step_next();
                }
            }
            Stm::APP => {
                let template = self.reg_spine.value();
                if fire(self.input.app_ready, self.app_valid()) {
                    if self.more_app(template) {
                        let founded = self.find_app(template);
                        self.reg_idx.connect(&founded);
                        self.reg_ctr.connect(&(*self.reg_ctr.value() + 1));
                        self.comb_table.read(
                            get_comb_addr(&self.reg_in.value().load[0])
                                + get_ptr(&template[founded])
                                + 1,
                        );
                    } else {
                        self.step_next();
                    }
                } else {
                    // fail to fire, keep reading
                    self.comb_table.read(
                        get_comb_addr(&self.reg_in.value().load[0])
                            + get_ptr(&template[*self.reg_idx.value()])
                            + 1,
                    );
                }
            }
        }
    }

    fn update_stat(&mut self) {
        if self.stat_detail_lv >= DLV_BUSY_RATE {
            if *self.reg_stm.value() != Stm::IDLE && self.input.app_ready && self.input.spine_ready
            {
                self.stat.busy_cycles += 1;
                self.stat.busy_per_cycle.push(true);
                if self.input.in_valid {
                    self.stat.blocked_cycles += 1;
                }
            } else {
                self.stat.busy_per_cycle.push(false);
            }
        }

        if self.stat_detail_lv >= DLV_FULL_LOG {
            if self.spine_valid() {
                self.stat.holder_contents.push(Some(self.spine_bits()));
            } else {
                self.stat.holder_contents.push(None);
            }
        }
    }

    fn tick_children(&mut self) {
        self.comb_table.tick();
        self.reg_in.tick();
        self.reg_addr.tick();
        self.reg_spine.tick();
        self.reg_stm.tick();
        self.reg_idx.tick();
        self.reg_ctr.tick();
    }
}

// #[test]
// // for now just some test-by-printing...
// fn reducer_spec() {
//     use Atom::*;
//     let mut reducer = Reducer::new();
//     let print_res = |r: &Reducer| {
//         // println!("spine: {:?}", r.spine());
//         // println!("app1: {:?}", r.app1());
//         // println!("app2: {:?}", r.app2());
//         // println!("app3: {:?}", r.app3());
//         println!("==========================");
//     };

//     reducer.tick();

//     reducer.input.link(|input| {
//         input.spine_ready = true;
//         input.app1_ready = true;
//         input.app2_ready = true;
//         input.app3_ready = true;
//         input.free_addr = 42;

//         input.in_valid = true;
//         input.in_app.stack_idx = 101;
//         input.in_app.load = [
//             COM(6, 48, [2, 0, 1, 3, 4, 5]), // XX(XX(XX))
//             PTR(0, true),
//             PTR(1, true),
//             PTR(2, true),
//             INT(3),
//             INT(4),
//             INT(5),
//             Y,
//         ];
//     });

//     reducer.tick();
//     print_res(&reducer);

//     reducer.input.link(|input| {
//         input.free_addr = 44;
//         input.in_valid = true;
//         input.in_app.stack_idx = 202;
//         input.in_app.load = [
//             COM(3, 6, [0, 2, 1, 2, 0, 0]), // XX(XX)
//             PTR(0, false),
//             PTR(1, true),
//             PTR(2, true),
//             INT(3),
//             INT(4),
//             INT(5),
//             Y,
//         ];
//     });
//     reducer.tick();
//     print_res(&reducer);

//     reducer.input.link(|input| {
//         input.in_valid = false;
//     });
//     reducer.tick();
//     print_res(&reducer);
// }
