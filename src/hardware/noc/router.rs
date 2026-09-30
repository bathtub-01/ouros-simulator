use crate::hardware::common::arbiter::RArbiter;
use crate::hardware::common::fifo::Fifo;
use crate::hw_module::{HwInput, HwModule};

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

///  T is the type of the *payload* of the noc packets
pub struct Router<T: Clone + Default, const X: u8, const Y: u8> {
    pub input: RouterInput<T>,
    in_north_buffer: Fifo<Packet<T>, 2, false>,
    in_south_buffer: Fifo<Packet<T>, 2, false>,
    in_west_buffer: Fifo<Packet<T>, 2, false>,
    in_east_buufer: Fifo<Packet<T>, 2, false>,
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
            in_east_buufer: Fifo::new(),
            in_local_buffer: Fifo::new(),
            out_north_arbiter: RArbiter::new(),
            out_south_arbiter: RArbiter::new(),
            out_west_arbiter: RArbiter::new(),
            out_east_arbiter: RArbiter::new(),
            out_local_arbiter: RArbiter::new(),
        }
    }

    pub fn north_out(&self) -> RouterPortOut<T> {
        todo!()
    }

    pub fn south_out(&self) -> RouterPortOut<T> {
        todo!()
    }

    pub fn west_out(&self) -> RouterPortOut<T> {
        todo!()
    }

    pub fn east_out(&self) -> RouterPortOut<T> {
        todo!()
    }

    pub fn local_out(&self) -> RouterPortOut<T> {
        todo!()
    }
}

impl<T: Clone + Default, const X: u8, const Y: u8> HwModule for Router<T, X, Y> {
    fn update_local(&mut self) -> Result<(), String> {
        todo!()
    }

    fn tick_children(&mut self) -> Result<(), String> {
        todo!()
    }
}
