use raster::Color;

use super::{shape::Shape, vec3::Vec3, ray::Ray, scene::Scene, color::color_add};

pub struct Sphere { 
    pub center: Vec3,
    pub radius: f64,
    pub color: Color
}

impl Sphere {
    pub fn new<T>(center: Vec3, radius: T, color: Color) -> Self
        where T: Into<f64> + Copy
    {
        Self {
            center,
            radius: radius.into(),
            color
        }
    }
}

impl Shape for Sphere {
    fn intersections(&self, ray: &Ray) -> Vec<f64> {
        // don't ask me what this does lol
        let os = Vec3::between(&self.center, &ray.origin);
        let b = 2.0 * os.dot(&ray.direction);
        let c = os.squid() - self.radius * self.radius;
        let discriminant = b * b - 4.0 * c;

        if discriminant < 0.0 {
            vec![]
        } else if discriminant.abs() < f64::EPSILON {
            vec![-b / 2.0]
        } else {
            let root = discriminant.sqrt();
            vec![(-b - root) / 2.0, (-b + root) / 2.0]
        }
    }

    fn color(&self) -> Color {
        self.color.clone()
    }

    fn normal_at(&self, point: &Vec3) -> Vec3 {
        (Vec3::between(point, &self.center)).unit().invert()
    }

    fn color_at(&self, point: &Vec3, scene: &Scene) -> Color {
        let normal = self.normal_at(point);
        let mut color = Color::black();

        // point / vector calculations seem to be correct -- the issue might be in illuminate?

        for light in &scene.lights {
            let v = Vec3::between(point, &light.position);
            let brightness = normal.dot(&v.unit());

            // println!("Normal vector at point {:?}: {:?} -- dot product is {}", normal, v, brightness);

            if brightness <= 0.0 { 
                continue;
            }

            let illumination = light.illuminate(self.color.clone(), *point, brightness);

            color = color_add(&color, &illumination);
        }

        return color;
    }
}