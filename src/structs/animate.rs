pub trait Animate {
    fn start(&mut self);
    fn update(&mut self, delta: f64);    
}