extern crate kiss3d;
extern crate nalgebra as na;
extern crate ncollide2d;
extern crate nphysics2d;
extern crate num_traits as num;
extern crate rand;
extern crate time;

pub use crate::engine::GraphicsManager;
pub use crate::testbed::Testbed;

mod engine;
pub mod objects;
mod testbed;
