use raster::Color;

use super::{vec3::Vec3, light::Light, color::color_scale};

#[derive(Clone)]
pub struct Finish {
    pub ambient: f64,
    pub diffuse: f64,
    pub shiny: f64
}

impl Finish {
    pub const DEFAULT: Finish = Finish { ambient: 0.0, diffuse: 0.7, shiny: 0.0};

    pub fn new(ambient: f64, diffuse: f64, shiny: f64) -> Self {
        Self {
            ambient,
            diffuse,
            shiny
        }
    }

    pub fn add_highlight(&self, reflex: &Vec3, light: &Light, light_vector: &Vec3) -> Color {
        if self.shiny <= 0.0 {
            Color::black()
        } else {
            let mut intensity = reflex.dot(&light_vector.unit());
            if intensity <= 0.0 {
                Color::black()
            } else {
                let exponent = 32.0 * self.shiny * self.shiny;
                intensity = intensity.powf(exponent);
                color_scale(&light.color, self.shiny * intensity)
            }
        }
    }
}