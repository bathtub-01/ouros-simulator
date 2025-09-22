// Reducer for structured combinators:
//          +-----------------+===> spine
// addr <-->| >               |
// input ==>|     Reducer     |===> app1
//          |                 |===> app2
//          +-----------------+===> app3
// The reducer is pipelined, and can handle one input in each
// clcok cycle. Inputs and Outputs has ready-valid signals,
// the reducer will be stalled if outputs fail to emit.

use crate::hardware::ouros::combinator::{parse_pat, Hole, ParseRes, ALL_PATTERNS, DECODE_TABLE};
use crate::hardware::ouros::config::*;
use crate::hardware::ouros::program::{ActiveApp, App, Atom, FrozenApp};
use crate::hardware::utils::fire;
use crate::hw_module::{HwInput, HwModule};

use super::ouros_core::is_nop;

fn dash_atom(atom: &Atom) -> Atom {
    match atom {
        Atom::PTR(p, _) => Atom::PTR(*p, false),
        _ => atom.clone(),
    }
}

#[derive(Default)]
pub struct ReducerInput {
    pub in_valid: bool,
    pub in_app: ActiveApp,
    pub spine_ready: bool,
    pub app1_ready: bool,
    pub app2_ready: bool,
    pub app3_ready: bool,
    pub free_addr: usize, // receive free address from GC
}

impl HwInput for ReducerInput {}

#[derive(Default)]
pub struct ReducerStat {
    pub busy_cycles: u32,
    pub busy_per_cycle: Vec<bool>,
    pub holder_contents: Vec<Option<ActiveApp>>,
}

pub struct Reducer {
    pub input: ReducerInput,
    decode_table: &'static [ParseRes; 64],
    spine_holder: (bool, ActiveApp),
    app1_holder: (bool, FrozenApp),
    app2_holder: (bool, FrozenApp),
    app3_holder: (bool, FrozenApp),
    stat: ReducerStat,
    stat_detail_lv: u8,
}

impl Reducer {
    pub fn new() -> Self {
        Self {
            input: ReducerInput {
                in_valid: false,
                in_app: Default::default(),
                spine_ready: false,
                app1_ready: false,
                app2_ready: false,
                app3_ready: false,
                free_addr: Default::default(),
            },
            decode_table: &DECODE_TABLE,
            spine_holder: Default::default(),
            app1_holder: Default::default(),
            app2_holder: Default::default(),
            app3_holder: Default::default(),
            stat: Default::default(),
            stat_detail_lv: Default::default(),
        }
    }

    pub fn detail(mut self, lv: u8) -> Self {
        self.stat_detail_lv = lv;
        self
    }

    /// (spine, app1, app2, app3)
    pub fn out_bundle(
        &self,
    ) -> (
        (bool, ActiveApp),
        (bool, FrozenApp),
        (bool, FrozenApp),
        (bool, FrozenApp),
    ) {
        if REDUCER_PIPE {
            (
                self.spine_holder.clone(),
                self.app1_holder.clone(),
                self.app2_holder.clone(),
                self.app3_holder.clone(),
            )
        } else {
            self.gen_result()
        }
    }
    /*
        pub fn spine(&self) -> &(bool, ActiveApp) {
            if REDUCER_PIPE {
                &self.spine_holder
            } else {
                unimplemented!()
            }
        }

        pub fn app1(&self) -> &(bool, FrozenApp) {
            if REDUCER_PIPE {
                &self.app1_holder
            } else {
                unimplemented!()
            }
        }

        pub fn app2(&self) -> &(bool, FrozenApp) {
            if REDUCER_PIPE {
                &self.app2_holder
            } else {
                unimplemented!()
            }
        }

        pub fn app3(&self) -> &(bool, FrozenApp) {
            if REDUCER_PIPE {
                &self.app3_holder
            } else {
                unimplemented!()
            }
        }
    */
    pub fn in_ready(&self) -> bool {
        if REDUCER_PIPE {
            // all holder registers should be either (1) free or (2) firing in this cycle
            let spine = self.spine_holder.0;
            let app1 = self.app1_holder.0;
            let app2 = self.app2_holder.0;
            let app3 = self.app3_holder.0;

            (!spine || fire(spine, self.input.spine_ready))
                && (!app1 || fire(app1, self.input.app1_ready))
                && (!app2 || fire(app2, self.input.app2_ready))
                && (!app3 || fire(app3, self.input.app3_ready))
        } else {
            self.input.spine_ready
                && self.input.app1_ready
                && self.input.app2_ready
                && self.input.app3_ready
        }
    }

    /// The number of heap cells will be consumed in this cycle
    pub fn addr_consumed(&self) -> usize {
        if fire(self.input.in_valid, self.in_ready()) {
            match self.input.in_app.load[0] {
                Atom::COM(_, code, _) => {
                    let res = &self.decode_table[code as usize];
                    [
                        res.app1.is_empty(),
                        res.app2.is_empty(),
                        res.app3.is_empty(),
                    ]
                    .iter()
                    .filter(|&&x| !x)
                    .count()
                }
                Atom::Y => 1,
                Atom::SEQ(true) => 0,
                _ => panic!("reducer: app head is not a valid combinator!"),
            }
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
}

impl HwModule for Reducer {
    fn update_local(&mut self) {
        // FIXME: looks a bit wrong..
        if fire(self.spine_holder.0, self.input.spine_ready) {
            self.spine_holder.0 = false;
        }
        if fire(self.app1_holder.0, self.input.app1_ready) {
            self.app1_holder.0 = false;
        }
        if fire(self.app2_holder.0, self.input.app2_ready) {
            self.app2_holder.0 = false;
        }
        if fire(self.app3_holder.0, self.input.app3_ready) {
            self.app3_holder.0 = false;
        }

        if REDUCER_PIPE && fire(self.input.in_valid, self.in_ready()) {
            let res = self.gen_result();
            self.spine_holder = res.0;
            self.app1_holder = res.1;
            self.app2_holder = res.2;
            self.app3_holder = res.3;
        }
    }

    fn update_stat(&mut self) {
        if self.stat_detail_lv >= DLV_BUSY_RATE {
            if fire(self.input.in_valid, self.in_ready()) {
                self.stat.busy_cycles += 1;
                self.stat.busy_per_cycle.push(true);
            } else {
                self.stat.busy_per_cycle.push(false);
            }
        }

        if self.stat_detail_lv >= DLV_FULL_LOG {
            if self.spine_holder.0 {
                self.stat
                    .holder_contents
                    .push(Some(self.spine_holder.1.clone()));
            } else {
                self.stat.holder_contents.push(None);
            }
        }
    }

    fn tick_children(&mut self) {}
}

#[test]
// for now just some test-by-printing...
fn reducer_spec() {
    use Atom::*;
    let mut reducer = Reducer::new();
    let print_res = |r: &Reducer| {
        // println!("spine: {:?}", r.spine());
        // println!("app1: {:?}", r.app1());
        // println!("app2: {:?}", r.app2());
        // println!("app3: {:?}", r.app3());
        println!("==========================");
    };

    reducer.tick();

    reducer.input.link(|input| {
        input.spine_ready = true;
        input.app1_ready = true;
        input.app2_ready = true;
        input.app3_ready = true;
        input.free_addr = 42;

        input.in_valid = true;
        input.in_app.stack_idx = 101;
        input.in_app.load = [
            COM(6, 48, [2, 0, 1, 3, 4, 5]), // XX(XX(XX))
            PTR(0, true),
            PTR(1, true),
            PTR(2, true),
            INT(3),
            INT(4),
            INT(5),
            Y,
        ];
    });

    reducer.tick();
    print_res(&reducer);

    reducer.input.link(|input| {
        input.free_addr = 44;
        input.in_valid = true;
        input.in_app.stack_idx = 202;
        input.in_app.load = [
            COM(3, 6, [0, 2, 1, 2, 0, 0]), // XX(XX)
            PTR(0, false),
            PTR(1, true),
            PTR(2, true),
            INT(3),
            INT(4),
            INT(5),
            Y,
        ];
    });
    reducer.tick();
    print_res(&reducer);

    reducer.input.link(|input| {
        input.in_valid = false;
    });
    reducer.tick();
    print_res(&reducer);
}
