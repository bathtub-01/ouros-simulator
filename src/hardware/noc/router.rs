use crate::hardware::common::arbiter::RArbiter;
use crate::hardware::common::fifo::Fifo;
use crate::hw_module::{HwInput, HwModule};

#[derive(PartialEq)]
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

fn port_to_buffer<T: Clone + Default, const N: usize, const P: bool>(
    p: &RouterPortIn<T>,
    b: &mut Fifo<Packet<T>, N, P>,
) {
    b.input.in_valid = p.in_valid;
    b.input.din = p.in_bits.clone();
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

/// T is the type of the *payload* of the noc packets.
/// `x` and `y` are the router's address in the 2-D mesh.
pub struct Router<T: Clone + Default> {
    pub input: RouterInput<T>,
    x: u8,
    y: u8,
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

impl<T: Clone + Default> Router<T> {
    pub fn new(x: u8, y: u8) -> Self {
        Self {
            input: Default::default(),
            x,
            y,
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

    pub fn coordinates(&self) -> (u8, u8) {
        (self.x, self.y)
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
    fn route_to(&self, p: Option<&Packet<T>>) -> Option<Goto> {
        use Goto::*;
        if let Some(packet) = p {
            let (x, y) = packet.dest;
            if x < self.x {
                Some(West)
            } else if x > self.x {
                Some(East)
            } else if y < self.y {
                Some(South)
            } else if y > self.y {
                Some(North)
            } else {
                Some(Local)
            }
        } else {
            None
        }
    }

    fn port_to_buffer_in(&mut self) {
        port_to_buffer(&self.input.north, &mut self.in_north_buffer);
        port_to_buffer(&self.input.south, &mut self.in_south_buffer);
        port_to_buffer(&self.input.west, &mut self.in_west_buffer);
        port_to_buffer(&self.input.east, &mut self.in_east_buffer);
        port_to_buffer(&self.input.local, &mut self.in_local_buffer);
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

    fn arbiter_out_ready(&mut self) {
        self.out_north_arbiter.input.out_ready = self.input.north.out_ready;
        self.out_south_arbiter.input.out_ready = self.input.south.out_ready;
        self.out_west_arbiter.input.out_ready = self.input.west.out_ready;
        self.out_east_arbiter.input.out_ready = self.input.east.out_ready;
        self.out_local_arbiter.input.out_ready = self.input.local.out_ready;
    }

    fn arbiter_in_valid(&mut self, go_to_vec: &[Option<Goto>; 5]) {
        self.out_north_arbiter.input.in_valid = go_to_vec
            .iter()
            .map(|goto| *goto == Some(Goto::North))
            .collect();
        self.out_south_arbiter.input.in_valid = go_to_vec
            .iter()
            .map(|goto| *goto == Some(Goto::South))
            .collect();
        self.out_west_arbiter.input.in_valid = go_to_vec
            .iter()
            .map(|goto| *goto == Some(Goto::West))
            .collect();
        self.out_east_arbiter.input.in_valid = go_to_vec
            .iter()
            .map(|goto| *goto == Some(Goto::East))
            .collect();
        self.out_local_arbiter.input.in_valid = go_to_vec
            .iter()
            .map(|goto| *goto == Some(Goto::Local))
            .collect();
    }

    fn buffer_out_ready_xbar(&mut self) {
        let north_arbiter_ready_vec = self.out_north_arbiter.in_ready_vec();
        let south_arbiter_ready_vec = self.out_south_arbiter.in_ready_vec();
        let west_arbiter_ready_vec = self.out_west_arbiter.in_ready_vec();
        let east_arbiter_ready_vec = self.out_east_arbiter.in_ready_vec();
        let local_arbiter_ready_vec = self.out_local_arbiter.in_ready_vec();

        let ors = |i: usize| {
            north_arbiter_ready_vec[i]
                || south_arbiter_ready_vec[i]
                || west_arbiter_ready_vec[i]
                || east_arbiter_ready_vec[i]
                || local_arbiter_ready_vec[i]
        };

        self.in_north_buffer.input.out_ready = ors(0);
        self.in_south_buffer.input.out_ready = ors(1);
        self.in_west_buffer.input.out_ready = ors(2);
        self.in_east_buffer.input.out_ready = ors(3);
        self.in_local_buffer.input.out_ready = ors(4);
    }
}

impl<T: Clone + Default> HwModule for Router<T> {
    fn update_local(&mut self) -> Result<(), String> {
        // NOTE input assignment order of arbiters matters

        let go_to_vec = [
            self.route_to(self.in_north_buffer.dout()),
            self.route_to(self.in_south_buffer.dout()),
            self.route_to(self.in_west_buffer.dout()),
            self.route_to(self.in_east_buffer.dout()),
            self.route_to(self.in_local_buffer.dout()),
        ];

        // buffer in valid + bits
        self.port_to_buffer_in();

        // arbiter out ready
        self.arbiter_out_ready();

        // arbiter in bits
        self.connect_arbiter_in_bits();

        // arbiter in valid
        self.arbiter_in_valid(&go_to_vec);

        // buffer out ready
        self.buffer_out_ready_xbar();

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
