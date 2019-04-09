pub extern crate guillotiere;

#[cfg(feature = "serialization")]
#[macro_use]
pub extern crate serde;

mod graph;
mod allocator;
pub mod svg;

pub use graph::*;
pub use allocator::*;
pub use svg::dump_svg;

type FloatRectangle = euclid::Box2D<f32>;
type FloatPoint = euclid::Point2D<f32>;
type FloatSize = euclid::Size2D<f32>;


