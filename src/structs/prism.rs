use std::cmp::min;

use raster::Color;

use super::{vec3::Vec3, shape::Shape, util::{fmin, fmax}};

pub struct Prism { 
    pub corner_ll: Vec3,
    pub corner_ur: Vec3,
    pub color: Color
}

impl Shape for Prism {
    fn intersections(&self, _ray: &super::ray::Ray) -> Vec<f64> {
        panic!("Attempting to call intersections on a struct that hasn't implemented it")
    }

    fn color(&self) -> Color {
        self.color.clone()
    }

    fn normal_at(&self, _point: &Vec3) -> Vec3 {
        Vec3::O
    }

    fn material(&self) -> raster::Image {
        raster::Image::blank(0, 0)
    }
}

impl Prism {
    pub fn new(corner_a: Vec3, corner_b: Vec3, color: Color) -> Self {
        Self {
            corner_ll: Vec3::new(fmin(corner_a.x, corner_b.x), fmin(corner_a.y, corner_b.y), fmin(corner_a.z, corner_b.z)),
            corner_ur: Vec3::new(fmax(corner_a.x, corner_b.x), fmax(corner_a.y, corner_b.y), fmax(corner_a.z, corner_b.z)),
            color
        }
    }
}