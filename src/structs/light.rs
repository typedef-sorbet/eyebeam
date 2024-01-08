use image::Rgba;
use super::{vec3::Vec3, color::{color_multiply, color_scale}};

pub struct Light {
    pub position: Vec3,
    pub color: Rgba<u8>
}

impl Light {
    pub fn new(position: Vec3, color: Rgba<u8>) -> Self {
        Self {position, color}
    }

    pub fn illuminate(&self, appearance: Rgba<u8>, _point: Vec3, brightness: f64) -> Rgba<u8> {
        color_scale(&color_multiply(&appearance, &self.color.clone()), brightness)
    }
}