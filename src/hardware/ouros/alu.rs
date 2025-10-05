// ALU, simply takes an application and produce the result.
// It allows over-applied applications, e.g., ((=) 1 2 a b) = (FALSE a b)
//           +------------+
//  input ==>|     ALU    |===> output
//           +------------+

use super::program::AluOp::*;
use super::program::Atom::*;
use crate::{
    hardware::{
        ouros::{
            config::*,
            program::{AluOp, App},
        },
        utils::fire,
    },
    hw_module::{HwInput, HwModule},
};

use super::program::{ActiveApp, Atom};

#[derive(Default)]
pub struct AluInput {
    pub input_valid: bool,
    pub input_bits: ActiveApp,
    pub output_ready: bool,
}

impl HwInput for AluInput {}

#[derive(Default)]
pub struct AluStat {
    pub busy_cycles: u32,
    pub busy_per_cycle: Vec<bool>,
    pub holder_contents: Vec<Option<ActiveApp>>,
}

#[derive(Default)]
pub struct Alu {
    pub input: AluInput,
    holder: (bool, ActiveApp),
    stat: AluStat,
    stat_detail_lv: u8,
}

impl Alu {
    pub fn new() -> Self {
        Alu {
            input: AluInput {
                input_valid: false,
                input_bits: Default::default(),
                output_ready: true,
            },
            holder: Default::default(),
            stat: Default::default(),
            stat_detail_lv: Default::default(),
        }
    }

    pub fn detail(mut self, lv: u8) -> Self {
        self.stat_detail_lv = lv;
        self
    }

    fn input_fire(&self) -> bool {
        self.input.input_valid && self.input_ready()
    }

    fn output_fire(&self) -> bool {
        self.output_valid() && self.input.output_ready
    }

    pub fn input_ready(&self) -> bool {
        if ALU_PIPE {
            !self.holder.0 || self.output_fire()
        } else {
            self.output_fire()
        }
    }

    pub fn output_valid(&self) -> bool {
        if ALU_PIPE {
            self.holder.0
        } else {
            self.input.input_valid
        }
    }

    pub fn output_bits(&self) -> ActiveApp {
        if ALU_PIPE {
            self.holder.1.clone()
        } else {
            self.gen_result()
        }
    }

    pub fn get_stat(&self) -> &AluStat {
        &self.stat
    }

    fn gen_result(&self) -> ActiveApp {
        fn take_int(atom: &Atom) -> i32 {
            match *atom {
                INT(i) => i,
                _ => {
                    panic!("alu: wrong operand type! {:?}", atom);
                }
            }
        }

        let oprand1: i32 = take_int(&self.input.input_bits.load[1]);
        let oprand2: i32 = take_int(&self.input.input_bits.load[2]);
        let res: Atom;

        fn comb_bool(b: bool, inv: &bool) -> Atom {
            if b ^ inv {
                // COM(2, 0, [1, 0, 0, 0, 0, 0]) // MicroHs - True
                NOP
            } else {
                // COM(2, 0, [0, 0, 0, 0, 0, 0]) // MicroHs - False
                NOP
            }
        }

        match &self.input.input_bits.load[0] {
            PRM(op, inv) => match op {
                EQ => {
                    res = comb_bool(oprand1 == oprand2, inv);
                }
                LE => {
                    res = comb_bool(oprand1 <= oprand2, inv);
                }
                LT => {
                    res = comb_bool(oprand1 < oprand2, inv);
                }
                ADD => {
                    res = INT(oprand1 + oprand2);
                }
                SUB => {
                    res = INT(oprand1 - oprand2);
                }
                MUL => {
                    res = INT(oprand1 * oprand2);
                }
            },
            _ => {
                panic!("alu: app head is not an primitive op!");
            }
        }

        ActiveApp {
            stack_idx: self.input.input_bits.stack_idx,
            load: {
                let mut arr: App = Default::default();
                arr[0] = res;
                for i in 3..APP_LENGTH {
                    if self.input.input_bits.load[i] != NOP {
                        arr[i - 2] = self.input.input_bits.load[i].clone();
                    } else {
                        break;
                    }
                }
                arr
            },
        }
    }
}

impl HwModule for Alu {
    fn update_local(&mut self) {
        if fire(self.holder.0, self.input.output_ready) {
            self.holder.0 = false;
        }

        if self.input_fire() {
            self.holder.0 = true;
            self.holder.1 = self.gen_result();
        }
    }

    fn update_stat(&mut self) {
        if self.stat_detail_lv >= DLV_BUSY_RATE {
            if fire(self.input.input_valid, self.input_ready()) {
                self.stat.busy_cycles += 1;
                self.stat.busy_per_cycle.push(true);
            } else {
                self.stat.busy_per_cycle.push(false);
            }
        }

        if self.stat_detail_lv >= DLV_FULL_LOG {
            if self.holder.0 {
                self.stat.holder_contents.push(Some(self.holder.1.clone()));
            } else {
                self.stat.holder_contents.push(None);
            }
        }
    }

    fn tick_children(&mut self) {}
}

#[test]
fn alu_spec() {
    use Atom::*;
    let mut alu = Alu::new();

    alu.tick();
    alu.input.link(|input| {
        input.output_ready = false;
        input.input_valid = true;
        input.input_bits.stack_idx = 2;
        input.input_bits.load = [
            PRM(AluOp::LE, false),
            INT(7),
            INT(7),
            PTR(11, false, false),
            PTR(22, false, false),
            NOP,
            NOP,
            NOP,
        ];
    });
    alu.tick();
    println!("{:?}, valid: {}", alu.output_bits(), alu.output_valid());

    alu.input.link(|input| {
        input.output_ready = true;
        input.input_valid = false;
    });
    alu.tick();
    println!("{:?}, valid: {}", alu.output_bits(), alu.output_valid());
}
