use raster::Color;
use super::{vec3::Vec3, color::{color_multiply, color_scale}};

pub struct Light {
    pub position: Vec3,
    pub color: Color
}

impl Light {
    pub fn new(position: Vec3, color: Color) -> Self {
        Self {position, color}
    }

    pub fn illuminate(&self, appearance: Color, _point: Vec3, brightness: f64) -> Color {
        color_scale(&color_multiply(&appearance, &self.color.clone()), brightness)
    }
}