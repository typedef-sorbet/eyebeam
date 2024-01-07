use raster::Color;

use super::{finish::Finish, vec3::Vec3, color::color_scale};

#[derive(Clone)]
pub struct Appearance {
    pub material: Color,
    pub finish: Finish
}

impl Appearance {
    pub fn new(material: Color, finish: Finish) -> Self {
        Self {
            material,
            finish
        }
    }

    pub fn color_at(&self, _point: &Vec3) -> Color {
        self.material.clone()
    }

    pub fn ambient_color_at(&self, point: &Vec3) -> Color {
        color_scale(&self.color_at(point), self.finish.ambient)
    }

    pub fn diffuse_color_at(&self, point: &Vec3) -> Color {
        color_scale(&self.color_at(point), self.finish.diffuse)
    }
}