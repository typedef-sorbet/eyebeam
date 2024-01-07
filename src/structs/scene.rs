use raster::Color;
use super::{camera::Camera, shape::Shape, light::Light};

pub struct Scene<'a> {
    pub camera: Camera,
    pub background: Color,
    pub shapes: Vec<Box<dyn Shape + 'a>>,
    pub lights: Vec<Light>
}

impl Scene<'_> {
    pub fn new(camera: Camera, background: Color) -> Self {
        Scene { camera, background, shapes: Vec::new(), lights: Vec::new() }
    }

    pub fn trace<T>(&self, x: T, y: T) -> Color 
        where T: Into<f64> + Copy 
    {
        self.camera.trace(self, x, y)
    }
}