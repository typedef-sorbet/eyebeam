use raster::{Color, Image};

use super::{ray::Ray, vec3::Vec3, scene::Scene};

pub trait Shape {
    fn intersections(&self, _ray: &Ray) -> Vec<f64> {
        panic!("Attempting to call intersections on a struct that hasn't implemented it")
    }

    fn closest_distance_along_ray(&self, ray: &Ray) -> f64 {
        *self.intersections(ray).iter()
             .filter(|&&d| d > 0.000001)                        // throw away anything less than this threshold
             .reduce(|acc, cur| if cur < acc {cur} else {acc})  // get the min
             .unwrap_or(&f64::INFINITY)                         // if there were no intersections, return inf so the background is rendered
    }

    fn color(&self) -> Color {
        Color::black()
    }

    fn normal_at(&self, _point: &Vec3) -> Vec3 {
        Vec3::O
    }

    fn color_at(&self, _point: &Vec3, _scene: &Scene) -> Color {
        Color::white()
    }

    fn material(&self) -> Image {
        Image::blank(0, 0)
    }
}