use crate::hardware::common::arbiter::RArbiter;
use crate::hardware::common::fifo::Fifo;
use crate::hw_module::{HwInput, HwModule};

enum Goto {
    North,
    South,
    West,
    East,
    Local,
}

fn extract_bits<T: Clone + Default>(p: Option<&Packet<T>>) -> Packet<T> {
    p.unwrap_or(&Default::default()).clone()
}

#[derive(Default, Clone)]
pub struct Packet<T: Clone + Default> {
    pub source: (u8, u8),
    pub dest: (u8, u8),
    pub load: T,
}

// -- in bits   ->
// -- in valid  ->
// <- in ready  --
// <- out bits  --
// <- out valid --
// -- out ready ->
#[derive(Default)]
pub struct RouterPortIn<T: Clone + Default> {
    pub in_valid: bool,
    pub in_bits: Packet<T>,
    pub out_ready: bool,
}

#[derive(Default)]
pub struct RouterPortOut<T: Clone + Default> {
    pub out_valid: bool,
    pub out_bits: Packet<T>,
    pub in_ready: bool,
}

#[derive(Default)]
pub struct RouterInput<T: Clone + Default> {
    pub north: RouterPortIn<T>,
    pub south: RouterPortIn<T>,
    pub west: RouterPortIn<T>,
    pub east: RouterPortIn<T>,
    pub local: RouterPortIn<T>,
}

///  T is the type of the *payload* of the noc packets.
/// X and Y are the router's address in the 2-D mesh.
pub struct Router<T: Clone + Default, const X: u8, const Y: u8> {
    pub input: RouterInput<T>,
    in_north_buffer: Fifo<Packet<T>, 2, false>,
    in_south_buffer: Fifo<Packet<T>, 2, false>,
    in_west_buffer: Fifo<Packet<T>, 2, false>,
    in_east_buffer: Fifo<Packet<T>, 2, false>,
    in_local_buffer: Fifo<Packet<T>, 2, false>,
    out_north_arbiter: RArbiter<Packet<T>, 5>,
    out_south_arbiter: RArbiter<Packet<T>, 5>,
    out_west_arbiter: RArbiter<Packet<T>, 5>,
    out_east_arbiter: RArbiter<Packet<T>, 5>,
    out_local_arbiter: RArbiter<Packet<T>, 5>,
}

impl<T: Clone + Default, const X: u8, const Y: u8> Router<T, X, Y> {
    pub fn new() -> Self {
        Self {
            input: Default::default(),
            in_north_buffer: Fifo::new(),
            in_south_buffer: Fifo::new(),
            in_west_buffer: Fifo::new(),
            in_east_buffer: Fifo::new(),
            in_local_buffer: Fifo::new(),
            out_north_arbiter: RArbiter::new(),
            out_south_arbiter: RArbiter::new(),
            out_west_arbiter: RArbiter::new(),
            out_east_arbiter: RArbiter::new(),
            out_local_arbiter: RArbiter::new(),
        }
    }

    pub fn north_out(&self) -> RouterPortOut<T> {
        RouterPortOut {
            out_valid: self.out_north_arbiter.out_valid(),
            out_bits: extract_bits(
                self.out_north_arbiter
                    .out_bits(self.out_north_arbiter.select()),
            ),
            in_ready: self.in_north_buffer.in_ready(),
        }
    }

    pub fn south_out(&self) -> RouterPortOut<T> {
        RouterPortOut {
            out_valid: self.out_south_arbiter.out_valid(),
            out_bits: extract_bits(
                self.out_south_arbiter
                    .out_bits(self.out_south_arbiter.select()),
            ),
            in_ready: self.in_south_buffer.in_ready(),
        }
    }

    pub fn west_out(&self) -> RouterPortOut<T> {
        RouterPortOut {
            out_valid: self.out_west_arbiter.out_valid(),
            out_bits: extract_bits(
                self.out_west_arbiter
                    .out_bits(self.out_west_arbiter.select()),
            ),
            in_ready: self.in_west_buffer.in_ready(),
        }
    }

    pub fn east_out(&self) -> RouterPortOut<T> {
        RouterPortOut {
            out_valid: self.out_east_arbiter.out_valid(),
            out_bits: extract_bits(
                self.out_east_arbiter
                    .out_bits(self.out_east_arbiter.select()),
            ),
            in_ready: self.in_east_buffer.in_ready(),
        }
    }

    pub fn local_out(&self) -> RouterPortOut<T> {
        RouterPortOut {
            out_valid: self.out_local_arbiter.out_valid(),
            out_bits: extract_bits(
                self.out_local_arbiter
                    .out_bits(self.out_local_arbiter.select()),
            ),
            in_ready: self.in_local_buffer.in_ready(),
        }
    }

    // X-first static routing
    fn route_to(&self, (x, y): (u8, u8)) -> Goto {
        use Goto::*;
        if x < X {
            West
        } else if x > X {
            East
        } else if y < Y {
            South
        } else if y > Y {
            North
        } else {
            Local
        }
    }

    fn connect_arbiter_in_bits(&mut self) {
        let full_bundle = vec![
            extract_bits(self.in_north_buffer.dout()),
            extract_bits(self.in_south_buffer.dout()),
            extract_bits(self.in_west_buffer.dout()),
            extract_bits(self.in_east_buffer.dout()),
            extract_bits(self.in_local_buffer.dout()),
        ];
        self.out_north_arbiter.input.in_bits = full_bundle.clone();
        self.out_south_arbiter.input.in_bits = full_bundle.clone();
        self.out_west_arbiter.input.in_bits = full_bundle.clone();
        self.out_east_arbiter.input.in_bits = full_bundle.clone();
        self.out_local_arbiter.input.in_bits = full_bundle.clone();
    }
}

impl<T: Clone + Default, const X: u8, const Y: u8> HwModule for Router<T, X, Y> {
    fn update_local(&mut self) -> Result<(), String> {
        self.in_north_buffer.input.default_input();
        self.in_south_buffer.input.default_input();
        self.in_west_buffer.input.default_input();
        self.in_east_buffer.input.default_input();
        self.in_local_buffer.input.default_input();

        self.connect_arbiter_in_bits();

        if self.in_north_buffer.out_valid() {
            if let Some(packet) = self.in_north_buffer.dout() {
                match self.route_to(packet.dest) {
                    Goto::North => todo!(),
                    Goto::South => todo!(),
                    Goto::West => todo!(),
                    Goto::East => todo!(),
                    Goto::Local => todo!(),
                }
            }
        }

        Ok(())
    }

    fn tick_children(&mut self) -> Result<(), String> {
        self.in_north_buffer.tick()?;
        self.in_south_buffer.tick()?;
        self.in_west_buffer.tick()?;
        self.in_east_buffer.tick()?;
        self.in_local_buffer.tick()?;
        self.out_north_arbiter.tick()?;
        self.out_south_arbiter.tick()?;
        self.out_west_arbiter.tick()?;
        self.out_east_arbiter.tick()?;
        self.out_local_arbiter.tick()?;
        Ok(())
    }
}
