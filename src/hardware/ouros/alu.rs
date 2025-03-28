// ALU, simply takes an application and produce the result:
//           +------------+
//  input ==>|     ALU    |===> output
//           +------------+

use crate::{
    hardware::ouros::program::{AluOp, App},
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
pub struct Alu {
    pub input: AluInput,
    holder: (bool, ActiveApp),
}

impl Alu {
    pub fn new() -> Self {
        Default::default()
    }

    fn input_fire(&self) -> bool {
        self.input.input_valid && self.input_ready()
    }

    fn output_fire(&self) -> bool {
        self.output_valid() && self.input.output_ready
    }

    pub fn input_ready(&self) -> bool {
        !self.holder.0 || self.output_fire()
    }

    pub fn output_valid(&self) -> bool {
        self.holder.0
    }

    pub fn output_bits(&self) -> &ActiveApp {
        &self.holder.1
    }
}

impl HwModule for Alu {
    fn update_local(&mut self) {
        use super::program::AluOp::*;
        use super::program::Atom::*;

        if self.output_fire() {
            self.holder.0 = false;
        }

        fn take_int(atom: &Atom) -> i32 {
            match *atom {
                INT(i) => i,
                _ => {
                    panic!("alu: wrong operand type!");
                }
            }
        }

        if self.input_fire() {
            let oprand1: i32 = take_int(&self.input.input_bits.load[1]);
            let oprand2: i32 = take_int(&self.input.input_bits.load[2]);
            let res: Atom;

            fn comb_bool(b: bool, inv: &bool) -> Atom {
                if b ^ inv {
                    COM(2, 0, [0, 0, 0, 0, 0, 0])
                } else {
                    COM(2, 0, [1, 0, 0, 0, 0, 0])
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
            self.holder.0 = true;
            self.holder.1.stack_idx = self.input.input_bits.stack_idx;
            self.holder.1.load = {
                let mut arr: App = Default::default();
                arr[0] = res;
                arr
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
        input.output_ready = true;
        input.input_valid = true;
        input.input_bits.stack_idx = 2;
        input.input_bits.load = [
            PRM(AluOp::LE, false),
            INT(7),
            INT(7),
            NOP,
            NOP,
            NOP,
            NOP,
            NOP,
        ];
    });
    alu.tick();

    println!("{:?}", alu.output_bits());
}
