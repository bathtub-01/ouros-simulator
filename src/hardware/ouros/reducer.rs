// Reducer for structured combinators:
//          +-----------------+===> spine
// addr <-->| >               |
// input ==>|     Reducer     |===> app1
//          |                 |===> app2
//          +-----------------+===> app3
// The reducer is pipelined, and can handle one input in each
// clcok cycle. Inputs and Outputs has ready-valid signals,
// the reducer will be stalled if outputs fail to emit.

use crate::hardware::ouros::combinator::{all_patterns, parse_pat, Hole, ParseRes};
use crate::hardware::ouros::program::{App, Atom, APP_LENGTH, HOLES};
use crate::hardware::utils::fire;
use crate::hw_module::{HwModule, HwStates};
use crate::{input, local};

#[derive(Default)]
struct ReducerInput {
    in_valid: bool,
    in_app: App,
    spine_ready: bool,
    app1_ready: bool,
    app2_ready: bool,
    app3_ready: bool,
    // receive free address from GC
    free_addr: usize,
}

struct ReducerLocal {
    decode_table: [ParseRes; 64],
    spine_holder: (bool, [Atom; APP_LENGTH]), // to support over-applied apps, spine should be a full app length
    app1_holder: (bool, [Atom; HOLES - 1]),
    app2_holder: (bool, [Atom; HOLES - 2]),
    app3_holder: (bool, [Atom; HOLES - 3]),
}

impl Default for ReducerLocal {
    fn default() -> Self {
        let parsed: Vec<ParseRes> = all_patterns().iter().map(|p| parse_pat(p)).collect();
        let decode_table: [ParseRes; 64] = parsed
            .try_into()
            .expect("pattern decode table size should match");

        Self {
            decode_table,
            spine_holder: Default::default(),
            app1_holder: Default::default(),
            app2_holder: Default::default(),
            app3_holder: Default::default(),
        }
    }
}

pub struct Reducer {
    states: HwStates<ReducerInput, ReducerLocal>,
}

impl Reducer {
    fn new() -> Self {
        Self {
            states: Default::default(),
        }
    }

    fn spine(&self) -> &(bool, [Atom; APP_LENGTH]) {
        &local!(self).spine_holder
    }

    fn app1(&self) -> &(bool, [Atom; HOLES - 1]) {
        &local!(self).app1_holder
    }

    fn app2(&self) -> &(bool, [Atom; HOLES - 2]) {
        &local!(self).app2_holder
    }

    fn app3(&self) -> &(bool, [Atom; HOLES - 3]) {
        &local!(self).app3_holder
    }

    fn in_ready(&self) -> bool {
        // all holder registers should be either (1) free or (2) firing in this cycle
        let spine = local!(self).spine_holder.0;
        let app1 = local!(self).app1_holder.0;
        let app2 = local!(self).app2_holder.0;
        let app3 = local!(self).app3_holder.0;

        (!spine || fire(spine, input!(self).spine_ready))
            && (!app1 || fire(app1, input!(self).app1_ready))
            && (!app2 || fire(app2, input!(self).app2_ready))
            && (!app3 || fire(app3, input!(self).app3_ready))
    }

    /// The number of heap cells will be consumed in this cycle
    fn addr_consumed(&self) -> usize {
        if fire(input!(self).in_valid, self.in_ready()) {
            match input!(self).in_app[0] {
                Atom::COM(_, code, _) => {
                    let res = &local!(self).decode_table[code as usize];
                    [
                        res.app1.is_empty(),
                        res.app2.is_empty(),
                        res.app3.is_empty(),
                    ]
                    .iter()
                    .filter(|&&x| !x)
                    .count()
                }
                _ => panic!("reducer: app head is not combinator!"),
            }
        } else {
            0
        }
    }
}

impl HwModule for Reducer {
    fn update_local(&mut self) {
        if fire(self.spine().0, input!(self).spine_ready) {
            local!(self).spine_holder.0 = false;
        }
        if fire(self.app1().0, input!(self).app1_ready) {
            local!(self).app1_holder.0 = false;
        }
        if fire(self.app2().0, input!(self).app2_ready) {
            local!(self).app2_holder.0 = false;
        }
        if fire(self.app3().0, input!(self).app3_ready) {
            local!(self).app3_holder.0 = false;
        }

        if fire(input!(self).in_valid, self.in_ready()) {
            match input!(self).in_app[0] {
                Atom::COM(arity, code, is) => {
                    let res = &local!(self).decode_table[code as usize];
                    let spine = &mut local!(self).spine_holder;
                    let in_app = &input!(self).in_app;
                    let app1 = &mut local!(self).app1_holder;
                    let app2 = &mut local!(self).app2_holder;
                    let app3 = &mut local!(self).app3_holder;
                    let trans = |h: &Hole| match h {
                        Hole::Arg(a) => in_app[is[*a as usize] as usize + 1].clone(),
                        Hole::Ptr(p) => Atom::PTR(*p as usize + input!(self).free_addr),
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
                    gen_res(&res.spine, &mut spine.1);
                    gen_res(&res.app1, &mut app1.1);
                    gen_res(&res.app2, &mut app2.1);
                    gen_res(&res.app3, &mut app3.1);
                    // append the spine for over-applied cases:
                    // e.g., S a b c x y = a c (b c) x y
                    let before = arity as usize + 1;
                    let after = res.spine.len();
                    for i in 0..(APP_LENGTH - before) {
                        if after + i < APP_LENGTH {
                            spine.1[after + i] = in_app[before + i].clone();
                        } else if before + i < APP_LENGTH && in_app[before + i] != Atom::NOP {
                            // over-sized result will be a runtime error..
                            panic!("reducer: over-sized over-applied app!");
                        } else {
                            break;
                        }
                    }
                    // valid for output
                    spine.0 = true;
                    app1.0 = app1.1[0] != Atom::NOP;
                    app2.0 = app2.1[0] != Atom::NOP;
                    app3.0 = app3.1[0] != Atom::NOP;
                }
                _ => panic!("reducer: app head is not combinator!"),
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
        println!("spine: {:?}", r.spine());
        println!("app1: {:?}", r.app1());
        println!("app2: {:?}", r.app2());
        println!("app3: {:?}", r.app3());
        println!("==========================");
    };

    reducer.tick();

    reducer.states.link_input(|input| {
        input.spine_ready = true;
        input.app1_ready = true;
        input.app2_ready = true;
        input.app3_ready = true;
        input.free_addr = 42;

        input.in_valid = true;
        input.in_app = [
            COM(6, 48, [2, 0, 1, 3, 4, 5]), // XX(XX(XX))
            PTR(0),
            PTR(1),
            PTR(2),
            INT(3),
            INT(4),
            INT(5),
            Y,
        ];
    });

    reducer.tick();
    print_res(&reducer);

    reducer.states.link_input(|input| {
        input.free_addr = 44;
        input.in_valid = true;
        input.in_app = [
            COM(3, 6, [0, 2, 1, 2, 0, 0]), // XX(XX)
            PTR(0),
            PTR(1),
            PTR(2),
            INT(3),
            INT(4),
            INT(5),
            Y,
        ];
    });
    reducer.tick();
    print_res(&reducer);

    reducer.states.link_input(|input| {
        input.in_valid = false;
    });
    reducer.tick();
    print_res(&reducer);
}
