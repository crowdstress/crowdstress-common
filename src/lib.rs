mod drawing_object;
mod exit;
mod geometry;
mod primitives;
mod room;
mod vector;
mod wall;

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
        wall::Wall,
    };
}
