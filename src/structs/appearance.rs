use raster::Color;

use super::{finish::Finish, vec3::Vec3, color::color_scale, scene::Scene, ray::Ray};

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

    pub fn reflect(&self, point: &Vec3, reflex: &Vec3, scene: &Scene, depth: i32) ->  Color {
        if self.finish.reflect <= 0.0 { 
            Color::black()
        } else {
            let reflected_ray = Ray::new(*point, *reflex);
            let reflected_color = reflected_ray.trace(scene, depth);
            color_scale(&reflected_color, self.finish.reflect)
        }
    }
}