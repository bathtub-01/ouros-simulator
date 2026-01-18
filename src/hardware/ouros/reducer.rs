// Reducer for combinators:
//          +-----------------+===> spine
// addr <-->| >               |
// input ==>|     Reducer     |
//          |                 |
//          +-----------------+===> app

use super::alu::compute;
use super::program::*;
use crate::hardware::common::{Register, SinglePortMem};
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
    SPECIAL,
}

#[derive(Default)]
pub struct ReducerInput {
    pub in_valid: bool,
    pub in_app: ActiveApp,
    pub spine_ready: bool,
    pub app_ready: bool,
    pub free_addrs: [usize; CONSUMERS_REDUCER],
    pub free_addrs_valid: [bool; CONSUMERS_REDUCER],
    pub search: usize, // for `exist` searching
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
    pub reg_spine: Register<App>,
    addr_regs: [Register<usize>; CONSUMERS_REDUCER],
    reg_arity: Register<u8>,
    reg_stm: Register<Stm>,
    reg_idx: Register<usize>,
    reg_app_mask: Register<bool>, // to handle blocked SPECIAL
    stat: ReducerStat,
    stat_detail_lv: u8,
}

impl Reducer {
    pub fn new(size: usize) -> Self {
        Self {
            input: Default::default(),
            comb_table: SinglePortMem::new(size),
            stat: Default::default(),
            stat_detail_lv: Default::default(),
            reg_in: Default::default(),
            addr_regs: Default::default(),
            reg_spine: Default::default(),
            reg_stm: Default::default(),
            reg_idx: Default::default(),
            reg_arity: Default::default(),
            reg_app_mask: Default::default(),
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
            || (*self.reg_stm.value() == Stm::SPECIAL && *self.reg_app_mask.value())
    }

    pub fn spine_bits(&self) -> ActiveApp {
        ActiveApp {
            stack_idx: self.reg_in.value().stack_idx,
            load: {
                if *self.reg_stm.value() == Stm::SPECIAL {
                    // Y case
                    let mut res: App = self.reg_in.value().load.clone();
                    res[0] = dash_atom(&self.reg_in.value().load[1]);
                    res[1] = Atom::PTR(self.input.free_addrs[0], false, false);
                    res
                } else {
                    let old_spn = &self.reg_in.value().load;
                    let before = *self.reg_arity.value() as usize + 1;
                    let mut res = self.inst(&self.comb_table.dout());
                    let after = app_length(&res);
                    assert!(
                        old_spn[before..].iter().filter(|a| !is_nop(a)).count() + after
                            <= APP_LENGTH
                    );
                    for i in 0..(APP_LENGTH - before) {
                        if after + i < APP_LENGTH {
                            res[after + i] = old_spn[before + i].clone();
                        }
                    }
                    res
                }
            },
        }
    }

    pub fn app_valid(&self) -> bool {
        *self.reg_stm.value() == Stm::APP || *self.reg_stm.value() == Stm::SPECIAL
    }

    pub fn app_bits(&self) -> FrozenApp {
        FrozenApp {
            heap_addr: {
                match self.reg_stm.value() {
                    Stm::SPINE | Stm::IDLE => 0,
                    Stm::APP => {
                        if let Atom::PTR(p, _, _) = self.reg_spine.value()[*self.reg_idx.value()] {
                            *self.addr_regs[p].value()
                        } else if let Atom::SPE(_, _, _, _, p) =
                            self.reg_spine.value()[*self.reg_idx.value()]
                        {
                            *self.addr_regs[p].value()
                        } else {
                            unreachable!()
                        }
                    }
                    Stm::SPECIAL => self.input.free_addrs[0],
                }
            },
            load: {
                if *self.reg_stm.value() == Stm::SPECIAL {
                    let mut res: App = Default::default();
                    res[0] = dash_atom(&self.reg_in.value().load[1]);
                    res[1] = Atom::PTR(self.input.free_addrs[0], false, false);
                    res
                } else {
                    self.inst(&self.comb_table.dout())
                }
            },
        }
    }

    pub fn in_ready(&self) -> bool {
        let state_correct = match *self.reg_stm.value() {
            Stm::IDLE => true,
            Stm::SPINE => !self.more_app(self.comb_table.dout()),
            Stm::APP => {
                fire(self.input.app_ready, self.app_valid())
                    && !self.more_app(self.reg_spine.value())
            }
            Stm::SPECIAL => fire(self.input.app_ready, self.app_valid()), // Y case
        };
        // demand all input free addrs are valid
        state_correct && self.input.free_addrs_valid.iter().all(|&vld| vld)
    }

    /// The number of heap cells that will be consumed in this cycle
    pub fn addr_consumed(&self) -> usize {
        if *self.reg_stm.value() == Stm::SPINE {
            self.comb_table
                .dout()
                .iter()
                .filter(|a| self.is_nested(a))
                .count()
        } else if *self.reg_stm.value() == Stm::SPECIAL && self.input.app_ready {
            1
        } else {
            0
        }
    }

    /// A bit vector indicating whether free addr is demanded at each slot
    pub fn consume_demands(&self) -> [bool; CONSUMERS_REDUCER] {
        std::array::from_fn(|i| i < self.addr_consumed())
    }

    /// whether an app is not emitted yet
    pub fn found(&self) -> bool {
        match *self.reg_stm.value() {
            Stm::APP => {
                for atm in self.reg_spine.value().iter().skip(*self.reg_idx.value()) {
                    match atm {
                        Atom::PTR(p, _, true) => {
                            if self.input.search == *self.addr_regs[*p].value() {
                                return true;
                            }
                        }
                        Atom::SPE(_, _, _, _, p) if self.is_nested(atm) => {
                            if self.input.search == *self.addr_regs[*p].value() {
                                return true;
                            }
                        }
                        _ => {}
                    }
                }
                false
            }
            Stm::SPECIAL => self.input.search == self.input.free_addrs[0],
            _ => false,
        }
    }

    pub fn get_stat(&self) -> &ReducerStat {
        &self.stat
    }

    pub fn in_fire(&self) -> bool {
        fire(self.input.in_valid, self.in_ready())
    }

    fn get_instant(&self, c: &SpeCell) -> i32 {
        let in_app = &self.reg_in.value().load;
        match c {
            SpeCell::ARG(arg) => take_int(&in_app[arg + 1]),
            SpeCell::LIT(i) => *i,
        }
    }

    fn is_instant(&self, c: &SpeCell) -> bool {
        let in_app = &self.reg_in.value().load;
        match c {
            SpeCell::ARG(arg) => is_int(&in_app[arg + 1]),
            SpeCell::LIT(_) => true,
        }
    }

    fn is_nested(&self, a: &Atom) -> bool {
        match a {
            Atom::PTR(_, _, new) => *new,
            Atom::SPE(_, _, l, r, _) => !(self.is_instant(&l) && self.is_instant(&r)),
            _ => false,
        }
    }

    fn more_app(&self, app: &App) -> bool {
        let idx = *self.reg_idx.value() + 1;
        app.iter().skip(idx as usize).any(|a| self.is_nested(a))
    }

    fn find_app(&self, app: &App) -> usize {
        let idx = *self.reg_idx.value() + 1;
        // x x x o * o o o
        // 0 1 2 3 4 5 6 7
        idx + app
            .iter()
            .skip(idx)
            .position(|a| self.is_nested(a))
            .unwrap()
    }

    /// instantiate an app from template
    fn inst(&self, app: &App) -> App {
        let mut res: App = app.clone();
        let mut hole = 0;
        for a in &mut res {
            match a {
                Atom::PTR(p, _, new) => {
                    if *new {
                        // assert_eq!(*self.reg_stm.value(), Stm::SPINE);
                        *a = Atom::PTR(self.input.free_addrs[*p - hole], true, false);
                    }
                }
                Atom::SPE(op, rev, l, r, p) => {
                    if self.is_instant(l) && self.is_instant(r) {
                        // speculation success
                        let op1 = self.get_instant(l);
                        let op2 = self.get_instant(r);
                        hole += 1;
                        *a = compute(op, *rev, op1, op2);
                    } else {
                        // speculation fails
                        *a = Atom::PTR(self.input.free_addrs[*p - hole], true, false);
                    }
                }
                Atom::ARG(arg, unq) => {
                    *a = {
                        let mut r = self.reg_in.value().load[*arg + 1].clone();
                        if let Atom::PTR(p, unq_, false) = r {
                            r = Atom::PTR(p, *unq && unq_, false);
                        }
                        r
                    }
                }
                _ => {}
            }
        }
        res
    }

    fn step_next(&mut self) {
        if self.in_fire() {
            self.reg_in.connect(&self.input.in_app);
            self.reg_idx.connect(&0);

            match self.input.in_app.load[0] {
                Atom::COM(arity, addr) => {
                    self.reg_stm.connect(&Stm::SPINE);
                    self.reg_arity.connect(&arity);
                    self.comb_table.read(addr)
                }
                Atom::CON(_, fields, idx) => {
                    self.reg_stm.connect(&Stm::SPINE);
                    if let Atom::TAB(base, free_vars) = self.input.in_app.load[fields + 1] {
                        self.reg_in.input.load[0] = Atom::COM(0, base + idx);
                        self.reg_arity.connect(&(fields as u8 + free_vars + 1));
                        self.comb_table.read(base + idx);
                    } else {
                        panic!()
                    }
                }
                Atom::Y => {
                    self.reg_app_mask.connect(&true);
                    self.reg_stm.connect(&Stm::SPECIAL);
                }
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
                // !NOTE! We are assuming this state won't be blocked.
                let template = self.comb_table.dout();
                if self.more_app(template) {
                    let founded = self.find_app(template);
                    self.reg_spine.connect(template);
                    self.reg_stm.connect(&Stm::APP);
                    self.reg_idx.connect(&founded);
                    self.comb_table.read(
                        get_comb_addr(&self.reg_in.value().load[0])
                            + get_ptr(&template[founded])
                            + 1,
                    );
                    self.addr_regs
                        .iter_mut()
                        .zip(self.input.free_addrs)
                        .for_each(|(reg, addr)| {
                            reg.connect(&addr);
                        });
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
            Stm::SPECIAL => {
                self.reg_app_mask.connect(&false);
                if fire(self.input.app_ready, self.app_valid()) {
                    self.step_next();
                }
            }
        }
        if self.spine_valid() && !self.input.spine_ready {
            println!("spine leaked!: {:?}", self.spine_bits());
            panic!();
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
        self.addr_regs.iter_mut().for_each(|reg| {
            reg.tick();
        });
        self.reg_spine.tick();
        self.reg_arity.tick();
        self.reg_stm.tick();
        self.reg_idx.tick();
        self.reg_app_mask.tick();
    }
}
