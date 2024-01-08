#![feature(portable_simd)]

mod structs;

use std::io::Cursor;

use image::{codecs::gif::GifEncoder, Frame, RgbaImage, ImageBuffer, ImageFormat, Rgba};
use raster::{Image, Color, save};
use structs::{scene::Scene, camera::Camera, vec3::Vec3, sphere::Sphere, light::Light, plane::Plane, prism::Prism, appearance::Appearance, finish::Finish, color::color_from_hex};

fn main() {
    let mut image = Image::blank(1600, 900);

    let camera: Camera = Camera::new(
        Vec3::new(-5, -5, -12),
        Vec3::K,
        4.0, 9.0 / 4.0
    );
    
    let background = Rgba([0, 0, 0, 255]);

    let mut scene: Scene = Scene::new(camera, background);

    let cyan_appearance     = Appearance::new(color_from_hex("#00FFFF").unwrap(), Finish::new(0.0, 0.7, 1.0, 0.7));
    let red_appearance      = Appearance::new(color_from_hex("#FF0000").unwrap(), Finish::new(0.0, 0.7, 1.0, 0.7));
    let magenta_appearance  = Appearance::new(color_from_hex("#FF00FF").unwrap(), Finish::new(0.0, 0.7, 1.0, 0.7));
    let yellow_appearance   = Appearance::new(color_from_hex("#FFFF00").unwrap(), Finish::new(0.0, 0.7, 1.0, 0.0));
    let blue_appearance     = Appearance::new(color_from_hex("#0000FF").unwrap(), Finish::DEFAULT);
    let green_appearance    = Appearance::new(color_from_hex("#00FF00").unwrap(), Finish::new(0.0, 0.7, 1.0, 0.7));
    let white_appearance    = Appearance::new(color_from_hex("#FFFFFF").unwrap(), Finish::DEFAULT);

    // spheres
    scene.shapes.push(Box::new(Sphere::new(Vec3::new(-5, 0, 0), 1, cyan_appearance.clone())));   // cyan
    scene.shapes.push(Box::new(Sphere::new(Vec3::new( 5, 0, 0), 1, red_appearance)));   // red
    scene.shapes.push(Box::new(Sphere::new(Vec3::new( 0,-5, 0), 1, magenta_appearance)));   // magenta
    scene.shapes.push(Box::new(Sphere::new(Vec3::new( 0, 0,-5), 1, yellow_appearance)));   // yellow
    scene.shapes.push(Box::new(Sphere::new(Vec3::new( 0, 0, 5), 1, blue_appearance)));   // blue
    scene.shapes.push(Box::new(Sphere::new(Vec3::new( 0, 0, 0), 1, white_appearance.clone())));   // white (duh)

    // plane
    scene.shapes.push(Box::new(Plane::new(Vec3::O + Vec3::J, Vec3::J.invert(), cyan_appearance)));

    // prism
    scene.shapes.push(Box::new(Prism::new(Vec3::new(3, 0, -3) + Vec3::J, Vec3::new(5, -2, -5) + Vec3::J, green_appearance)));

    scene.lights.push(Light::new(Vec3::new(5, -5, -5) * 10, color_from_hex("#FFFFFF").unwrap()));
    // scene.lights.push(Light::new(Vec3::new(0, 0, -10), color_from_hex("#FF0000").unwrap()));

    // Create a byte buffer, a cursor into that buffer, and an encoder with a handle to that cursor
    let mut bytes: Vec<u8> = Vec::new();
    let mut cursor = Cursor::new(&mut bytes);
    let gif_encoder = GifEncoder::new_with_speed(cursor, 10);

    let mut img: RgbaImage = ImageBuffer::new(1600, 900);

    // Draw a 50x50 grid of pixels
    for x in 0..1600 {
        for y in 0..900 {
            // image.set_pixel(x, y, scene.trace((x as f64 / 1600.0) - 0.5, (y as f64 / 900.0) - 0.5)).unwrap();
            img.put_pixel(x, y, scene.trace((x as f64 / 1600.0) - 0.5, (y as f64 / 900.0) - 0.5))
        }
    }

    img.save("out/img.png").unwrap();
}