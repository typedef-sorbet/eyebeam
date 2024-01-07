use raster::{Color, Image};
use super::{vec3::Vec3, shape::Shape, ray::Ray, scene::Scene, color::color_add};

pub struct Plane {
    pub normal: Vec3,
    pub dist_from_normal: f64,
    pub color: Color
}

impl Shape for Plane {
    fn intersections(&self, ray: &Ray) -> Vec<f64> {
        let angle = ray.direction.dot(&self.normal);

        if angle.abs() < f64::EPSILON {
            vec![]
        } else {
            let b = self.normal.dot(&(ray.origin - (self.normal * self.dist_from_normal)));
            vec![-b / angle]
        }
    }

    fn color(&self) -> Color {
        self.color.clone()
    }

    fn normal_at(&self, _point: &Vec3) -> Vec3 {
        self.normal
    }

    fn material(&self) -> Image {
        raster::Image::blank(0, 0)
    }
}

impl Plane {
    pub fn new(direction: Vec3, distance: f64, color: Color) -> Self {
        Self {
            normal: direction.unit(),
            dist_from_normal: distance,
            color
        }
    }
}