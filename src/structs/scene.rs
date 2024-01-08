use image::Rgba;
use super::{camera::Camera, shape::Shape, light::Light, animate::Animate};

pub struct Scene<'a> {
    pub camera: Camera,
    pub background: Rgba<u8>,
    pub shapes: Vec<Box<dyn Shape + Send + Sync + 'a>>,
    pub lights: Vec<Light>
}

impl Scene<'_> {
    pub fn new(camera: Camera, background: Rgba<u8>) -> Self {
        Scene { camera, background, shapes: Vec::new(), lights: Vec::new() }
    }

    pub fn trace<T>(&self, x: T, y: T) -> Rgba<u8> 
        where T: Into<f64> + Copy 
    {
        self.camera.trace(self, x, y)
    }
}

impl Animate for Scene<'_> {
    fn start(&mut self) {
        todo!()
    }

    fn update(&mut self, delta: f64) {
        self.camera.update(delta);
    }
}