use image::Rgba;

use super::{ray::Ray, vec3::Vec3, scene::Scene, color::color_add, appearance::Appearance};

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

    fn color(&self) -> Rgba<u8> {
        // Default to black
        Rgba([0, 0, 0, 255])
    }

    fn normal_at(&self, _point: &Vec3) -> Vec3 {
        Vec3::O
    }

    fn appearance(&self) -> Appearance {
        panic!("No default appearance for base Shape")
    }

    fn color_at(&self, point: &Vec3, ray: &Ray, scene: &Scene, depth: i32) -> Rgba<u8> {
        let normal = self.normal_at(point);
        let mut color = self.appearance().ambient_color_at(point);
        let reflex = ray.reflect(&normal);
        let reflection =  self.appearance().reflect(point, &reflex, scene, depth);

        color = color_add(&color, &reflection);

        // point / vector calculations seem to be correct -- the issue might be in illuminate?

        for light in &scene.lights {
            let v = Vec3::between(point, &light.position);

            // If this shape is in another shape's shadow, stop the calculation here
            if scene.shapes.iter().any(|shape| shape.casts_shadow(point, v)) { continue; }

            let brightness = normal.dot(&v.unit());

            if brightness <= 0.0 { continue; }

            let illumination = light.illuminate(self.appearance().diffuse_color_at(point), *point, brightness);

            color = color_add(&color, &illumination);

            let highlight = self.appearance().finish.add_highlight(&reflex, light, &v);

            color = color_add(&color, &highlight);
        }

        color
    }

    fn casts_shadow(&self, point: &Vec3, light_vector: Vec3) -> bool {
        let distance_to_light = Vec3::between(point, &light_vector).length();
        let ray = Ray::new(*point, light_vector);

        self.closest_distance_along_ray(&ray) <= distance_to_light
    }
}