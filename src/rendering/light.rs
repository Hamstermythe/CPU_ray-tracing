use crate::Point;
//use crate::Rgba;

use sdl2::pixels::Color;


#[derive(Debug, PartialEq, Clone)]
pub struct Light {
    pub position: Point,
    pub radius: f64, // distance de penetration
    pub color: Color,
}
impl Light {
    pub fn new(position: Point, radius: f64, color: Color) -> Light {
        Light { position, radius, color }
    }
}
