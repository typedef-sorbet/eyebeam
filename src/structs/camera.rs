use raster::Color;

use super::vec3::Vec3;
use super::ray::Ray;

use super::scene::Scene;

pub struct Camera {
    pub location: Vec3,
    pub look_at: Vec3,
    pub direction: Vec3,
    pub camera_right: Vec3,
    pub camera_up: Vec3
}

impl Camera {
    pub fn new<T>(location: Vec3, look_at: Vec3, width: T, height: T) -> Self 
        where T: Into<f64> + Copy {
        let direction = Vec3::between(&location, &look_at).unit();
        let camera_right = Vec3::J.cross(&direction).unit() * (width.into() / 2.0);
        let camera_up = camera_right.cross(&direction).unit().invert() * (-height.into() / 2.0);
        
        Camera {
            location,
            look_at,
            direction,
            camera_right,
            camera_up
        }
    }

    pub fn trace<T>(&self, scene: &Scene, x: T, y: T) -> Color
        where T: Into<f64> + Copy {

        let ray_x = self.camera_right * x.into();
        let ray_y = self.camera_up.invert() * y.into();
        let ray_dir = self.direction + ray_x + ray_y;
        let ray = Ray::new(self.location, ray_dir);

        ray.trace(scene)
    }
}