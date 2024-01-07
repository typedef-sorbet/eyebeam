use super::{vec3::Vec3, shape::Shape, ray::Ray, appearance::Appearance};

pub struct Plane {
    pub point:Vec3,
    pub normal: Vec3,
    pub appearance: Appearance
}

impl Shape for Plane {
    fn intersections(&self, ray: &Ray) -> Vec<f64> {
        let angle = ray.direction.dot(&self.normal);

        if angle.abs() < f64::EPSILON {
            vec![]
        } else {
            vec![(self.point - ray.origin).dot(&self.normal) / angle]
        }
    }

    fn appearance(&self) -> Appearance {
        self.appearance.clone()
    }

    fn normal_at(&self, _point: &Vec3) -> Vec3 {
        self.normal
    }
}

impl Plane {
    pub fn new(point: Vec3, direction: Vec3, appearance: Appearance) -> Self {
        Self {
            point,
            normal: direction.unit(),
            appearance
        }
    }
}