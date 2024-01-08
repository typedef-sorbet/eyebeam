use image::Rgba;

use super::animate::Animate;
use super::vec3::Vec3;
use super::ray::Ray;

use super::scene::Scene;

pub struct Camera {
    pub location: Vec3,
    pub look_at: Vec3,
    pub direction: Vec3,
    pub camera_right: Vec3,
    pub camera_up: Vec3,
    pub camera_moves: Vec<(Vec3, f64, Option<Vec3>)>,
    width: f64,
    height: f64
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
            camera_up,
            camera_moves: Vec::new(),
            width: width.into(),
            height: height.into()
        }
    }

    pub fn trace<T>(&self, scene: &Scene, x: T, y: T) -> Rgba<u8>
        where T: Into<f64> + Copy {

        let ray_x = self.camera_right * x.into();
        let ray_y = self.camera_up.invert() * y.into();
        let ray_dir = self.direction + ray_x + ray_y;
        let ray = Ray::new(self.location, ray_dir);

        ray.trace(scene, 0)
    }

    pub fn add_camera_move(&mut self, move_to: Vec3, duration: f64, look_at: Option<Vec3>) {
        self.camera_moves.push((move_to, duration, look_at));
    }
}

impl Animate for Camera {
    fn start(&mut self) {
        todo!()
    }

    fn update(&mut self, delta: f64) {
        if !self.camera_moves.is_empty() {
            let (move_to, duration, look_at) = self.camera_moves[0];

            // Move the camera to the specified point
            self.location = self.location + (Vec3::between(&self.location, &move_to) * (delta / duration));

            if let Some(lock_onto) = look_at {
                // Rotate the camera to look at the given point
                let direction = Vec3::between(&self.location, &lock_onto).unit();
                self.camera_right = Vec3::J.cross(&direction).unit() * (self.width / 2.0);
                self.camera_up = self.camera_right.cross(&direction).unit().invert() * (-self.height / 2.0);
            }

            // Snap-to and remove item from moves
            if Vec3::between(&self.location, &move_to).length() < 0.01 {
                self.location = move_to;
                // TODO should probably use a Queue type
                self.camera_moves.remove(0);
            }
        }
    }
}