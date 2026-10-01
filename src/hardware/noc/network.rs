//! Two-dimensional mesh of NoC routers, exposing only the local ports.
//!
//! The coordinate system is (x, y): x grows eastward and y grows northward.
//! Local ports are indexed `[y][x]` in `Network::input.local`, or accessed
//! individually with `Network::local_out(x, y)`.
//!
//! All links use the router's existing valid/ready protocol. The mesh has no
//! wraparound: missing links at the perimeter have invalid input and are never
//! ready to accept output. Routers advance on the same simulated clock edge.

use crate::hw_module::HwModule;

use super::router::{Router, RouterInput, RouterPortIn, RouterPortOut};

/// Maximum number of nodes along either axis. A packet's destination
/// coordinates are `u8`, so valid router addresses range from 0 to 255.
pub const MAX_MESH_DIM: usize = u8::MAX as usize + 1;

/// External signals driven into every router's *local* port.
/// `local[y][x]` is the port at coordinates `(x, y)`.
pub struct NetworkInput<T: Clone + Default> {
    pub local: Vec<Vec<RouterPortIn<T>>>,
}

struct RouterOutputs<T: Clone + Default> {
    north: RouterPortOut<T>,
    south: RouterPortOut<T>,
    west: RouterPortOut<T>,
    east: RouterPortOut<T>,
}

impl<T: Clone + Default> RouterOutputs<T> {
    fn from_router(router: &Router<T>) -> Self {
        Self {
            north: router.north_out(),
            south: router.south_out(),
            west: router.west_out(),
            east: router.east_out(),
        }
    }
}

/// Convert a neighbor's output bundle to the input bundle on this side of
/// the link. The `ready` signal travels in the opposite direction to `valid`.
fn connect<T: Clone + Default>(other: &RouterPortOut<T>) -> RouterPortIn<T> {
    RouterPortIn {
        in_valid: other.out_valid,
        in_bits: other.out_bits.clone(),
        out_ready: other.in_ready,
    }
}

/// A synchronous `width` × `height` mesh of packet routers.
///
/// Each router is stored by value and has runtime coordinates, so the network
/// does not require a factory, trait objects, or any const-generic expansion.
pub struct Network<T: Clone + Default> {
    pub input: NetworkInput<T>,
    width: usize,
    height: usize,
    routers: Vec<Router<T>>,
}

impl<T: Clone + Default> Network<T> {
    /// Construct a mesh with routers `(0,0)` through `(width-1,height-1)`.
    /// Panics for zero dimensions or dimensions larger than `MAX_MESH_DIM`.
    pub fn new(width: usize, height: usize) -> Self {
        assert!(
            (1..=MAX_MESH_DIM).contains(&width) && (1..=MAX_MESH_DIM).contains(&height),
            "NoC dimensions must be between 1 and {MAX_MESH_DIM}"
        );

        let mut routers = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                // Safe because width and height are at most 256.
                routers.push(Router::new(x as u8, y as u8));
            }
        }

        let local = (0..height)
            .map(|_| (0..width).map(|_| RouterPortIn::default()).collect())
            .collect();

        Self {
            input: NetworkInput { local },
            width,
            height,
            routers,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn index(&self, x: usize, y: usize) -> usize {
        assert!(
            x < self.width && y < self.height,
            "NoC coordinate out of range"
        );
        y * self.width + x
    }

    /// Read `out_valid`, `out_bits`, and injection `in_ready` for the local
    /// port at `(x, y)`. The returned bundle is a snapshot (not a live ref).
    pub fn local_out(&self, x: usize, y: usize) -> RouterPortOut<T> {
        self.routers[self.index(x, y)].local_out()
    }
}

impl<T: Clone + Default> HwModule for Network<T> {
    fn update_local(&mut self) -> Result<(), String> {
        // This phase only wires inputs for the child routers. In particular,
        // do not call their `update_local` methods here: each child's `tick()`
        // will perform its own local update before ticking its children.
        // Snapshot every link output before assigning any router inputs.
        let outputs: Vec<_> = self
            .routers
            .iter()
            .map(RouterOutputs::from_router)
            .collect();

        for y in 0..self.height {
            for x in 0..self.width {
                let local = &self.input.local[y][x];
                if local.in_valid
                    && (usize::from(local.in_bits.dest.0) >= self.width
                        || usize::from(local.in_bits.dest.1) >= self.height)
                {
                    return Err(format!(
                        "NoC destination {:?} at local port ({x}, {y}) is outside {}x{} mesh",
                        local.in_bits.dest, self.width, self.height
                    ));
                }

                let idx = y * self.width + x;
                self.routers[idx].input = RouterInput {
                    north: if y + 1 < self.height {
                        connect(&outputs[(y + 1) * self.width + x].south)
                    } else {
                        RouterPortIn::default()
                    },
                    south: if y > 0 {
                        connect(&outputs[(y - 1) * self.width + x].north)
                    } else {
                        RouterPortIn::default()
                    },
                    west: if x > 0 {
                        connect(&outputs[y * self.width + x - 1].east)
                    } else {
                        RouterPortIn::default()
                    },
                    east: if x + 1 < self.width {
                        connect(&outputs[y * self.width + x + 1].west)
                    } else {
                        RouterPortIn::default()
                    },
                    local: RouterPortIn {
                        in_valid: local.in_valid,
                        in_bits: local.in_bits.clone(),
                        out_ready: local.out_ready,
                    },
                };
            }
        }

        Ok(())
    }

    fn tick_children(&mut self) -> Result<(), String> {
        // All inter-router signals were captured by update_local before this
        // loop, so advancing routers sequentially does not change the inputs
        // sampled by any other router during this clock cycle.
        for (idx, router) in self.routers.iter_mut().enumerate() {
            router.tick().map_err(|err| {
                format!(
                    "NoC router ({}, {}) failed: {err}",
                    idx % self.width,
                    idx / self.width
                )
            })?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cycle(network: &mut Network<u32>) {
        network.tick().unwrap();
    }

    fn send_and_receive(width: usize, height: usize, src: (u8, u8), dst: (u8, u8)) {
        let mut mesh = Network::<u32>::new(width, height);
        mesh.input.local[dst.1 as usize][dst.0 as usize].out_ready = true;
        let injector = &mut mesh.input.local[src.1 as usize][src.0 as usize];
        injector.in_valid = true;
        injector.in_bits.source = src;
        injector.in_bits.dest = dst;
        injector.in_bits.load = 0x8765_4321;

        mesh.update_local().unwrap();
        assert!(mesh.local_out(src.0 as usize, src.1 as usize).in_ready);
        cycle(&mut mesh);
        mesh.input.local[src.1 as usize][src.0 as usize].in_valid = false;

        for _ in 0..64 {
            // An output is visible after the previous tick. A ready receiver
            // consumes that output at the *next* tick.
            let output = mesh.local_out(dst.0 as usize, dst.1 as usize);
            if output.out_valid {
                assert_eq!(output.out_bits.source, src);
                assert_eq!(output.out_bits.dest, dst);
                assert_eq!(output.out_bits.load, 0x8765_4321);
                return;
            }
            cycle(&mut mesh);
        }
        panic!("NoC packet did not reach {dst:?} from {src:?}");
    }

    #[test]
    fn routes_both_directions_and_same_node() {
        send_and_receive(4, 3, (0, 0), (3, 2));
        send_and_receive(4, 3, (3, 2), (0, 0));
        send_and_receive(4, 3, (0, 2), (3, 0));
        send_and_receive(1, 1, (0, 0), (0, 0));
    }

    #[test]
    fn rejects_invalid_dimensions_and_destinations() {
        assert!(std::panic::catch_unwind(|| Network::<u32>::new(0, 2)).is_err());
        assert!(std::panic::catch_unwind(|| Network::<u32>::new(2, 257)).is_err());
        let mut mesh = Network::<u32>::new(2, 2);
        mesh.input.local[0][0].in_valid = true;
        mesh.input.local[0][0].in_bits.dest = (2, 0);
        assert!(mesh.update_local().is_err());
    }

    #[test]
    fn holds_output_while_sink_is_not_ready() {
        let mut mesh = Network::<u32>::new(1, 1);
        mesh.input.local[0][0].in_valid = true;
        mesh.input.local[0][0].in_bits.dest = (0, 0);
        mesh.input.local[0][0].in_bits.load = 123;
        cycle(&mut mesh);
        mesh.input.local[0][0].in_valid = false;
        cycle(&mut mesh);

        for _ in 0..4 {
            assert!(mesh.local_out(0, 0).out_valid);
            assert_eq!(mesh.local_out(0, 0).out_bits.load, 123);
            cycle(&mut mesh);
        }
        mesh.input.local[0][0].out_ready = true;
        cycle(&mut mesh);
        cycle(&mut mesh);
        assert!(!mesh.local_out(0, 0).out_valid);
    }
}
