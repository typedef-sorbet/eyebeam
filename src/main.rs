#![feature(portable_simd)]

mod structs;

use raster::{Image, Color, save};
use structs::{scene::Scene, camera::Camera, vec3::Vec3, sphere::Sphere, light::Light, plane::Plane, prism::Prism};

fn main() {
    let mut image = Image::blank(1600, 900);

    let camera: Camera = Camera::new(
        &Vec3::new(-5, -12, -12),
        &Vec3::K,
        4.0, 9.0 / 4.0
    );
    
    let background = Color::black();

    let mut scene: Scene = Scene::new(camera, background);

    // spheres
    scene.shapes.push(Box::new(Sphere::new(Vec3::new(-5, 0, 0), 1, Color::hex("#00FFFF").unwrap())));   // cyan
    scene.shapes.push(Box::new(Sphere::new(Vec3::new( 5, 0, 0), 1, Color::hex("#FF0000").unwrap())));   // red
    scene.shapes.push(Box::new(Sphere::new(Vec3::new( 0,-5, 0), 1, Color::hex("#FF00FF").unwrap())));   // magenta
    scene.shapes.push(Box::new(Sphere::new(Vec3::new( 0, 0,-5), 1, Color::hex("#FFFF00").unwrap())));   // yellow
    scene.shapes.push(Box::new(Sphere::new(Vec3::new( 0, 0, 5), 1, Color::hex("#0000FF").unwrap())));   // blue
    scene.shapes.push(Box::new(Sphere::new(Vec3::new( 0, 0, 0), 1, Color::hex("#FFFFFF").unwrap())));   // white (duh)

    // plane
    scene.shapes.push(Box::new(Plane::new(Vec3::O + Vec3::J, Vec3::J.invert(), Color::white())));

    // TODO Prism not currently working
    // scene.shapes.push(Box::new(Prism::new(Vec3::new(3, 0, -3), Vec3::new(4, -1, -4), Color::white())));

    scene.lights.push(Light::new(Vec3::new(5, -5, -5) * 10, Color::hex("#FFFFFF").unwrap()));
    // scene.lights.push(Light::new(Vec3::new(0, 0, -10), Color::hex("#FF0000").unwrap()));

    // Draw a 50x50 grid of pixels
    for x in 0..1600 {
        for y in 0..900 {
            image.set_pixel(x, y, scene.trace((x as f64 / 1600.0) - 0.5, (y as f64 / 900.0) - 0.5)).unwrap();
        }
    }

    save(&image, "out/img.png").unwrap();
}