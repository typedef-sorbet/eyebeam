use raster::{Color, Image};
use super::{vec3::Vec3, shape::Shape, ray::Ray, scene::Scene, color::color_add};

pub struct Plane {
    pub point:Vec3,
    pub normal: Vec3,
    pub color: Color
}

impl Shape for Plane {
    fn intersections(&self, ray: &Ray) -> Vec<f64> {
        let angle = ray.direction.dot(&self.normal);

        if angle.abs() < f64::EPSILON {
            vec![]
        } else {
            return vec![(self.point - ray.origin).dot(&self.normal) / angle]
        }
    }

    fn color(&self) -> Color {
        self.color.clone()
    }

    fn normal_at(&self, _point: &Vec3) -> Vec3 {
        self.normal
    }
}

impl Plane {
    pub fn new(point: Vec3, direction: Vec3, color: Color) -> Self {
        Self {
            point,
            normal: direction.unit(),
            color
        }
    }
}