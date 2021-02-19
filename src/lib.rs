pub mod drawing_object;
pub mod exit;
pub mod geometry;
pub mod primitives;
pub mod room;
pub mod vector;

#[macro_use]
extern crate serde_derive;

pub mod prelude {
    pub use crate::{
        drawing_object::DrawingObject,
        exit::Exit,
        geometry,
        primitives::{Point, Polygon, Section},
        room::Room,
        vector::Vector,
    };
}
