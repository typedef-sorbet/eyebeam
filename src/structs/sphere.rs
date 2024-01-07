use raster::Color;

use super::{shape::Shape, vec3::Vec3, ray::Ray};

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
}