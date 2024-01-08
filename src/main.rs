#![feature(portable_simd)]

mod structs;

use std::fs::File;

use gif::ExtensionData;
use image::Rgba;
use structs::{scene::Scene, camera::Camera, vec3::Vec3, sphere::Sphere, light::Light, plane::Plane, prism::Prism, appearance::Appearance, finish::Finish, color::color_from_hex, animate::Animate};

fn main() {
    let mut camera: Camera = Camera::new(
        Vec3::new(-5, -5, -12),
        Vec3::K,
        4.0, 9.0 / 4.0
    );

    let frame_delta: f64 = 1.0 / 60.0; // 60 fps
    let duration: f64 = 1.0;

    camera.add_camera_move(Vec3::new(5, -5, -12), 1.0, Some(Vec3::K));
    
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
    let mut frame_bytes: Vec<u8> = Vec::new();

    let mut image = File::create("out/animation.gif").unwrap();
    let mut encoder = gif::Encoder::new(&mut image, 900, 1600, &[]).unwrap();

    encoder.write_extension(ExtensionData::new_control_ext(2, gif::DisposalMethod::Any, false, None)).unwrap();

    for frame_num in 0..(duration / frame_delta).ceil() as i32 {
        println!("Drawing frame {}...", frame_num);
        for x in 0..1600 {
            for y in 0..900 {
                // frame.put_pixel(x, y, scene.trace((x as f64 / 1600.0) - 0.5, (y as f64 / 900.0) - 0.5));
                // ???????
                let color = scene.trace((x as f64 / 1600.0) - 0.5, (y as f64 / 900.0) - 0.5);
                for byte in color.0 { frame_bytes.push(byte) }
            }
        }

        println!("Encoding frame {}...", frame_num);
        let frame = gif::Frame::from_rgba_speed(900, 1600, frame_bytes.as_mut_slice(), 10);

        encoder.write_frame(&frame).unwrap();

        frame_bytes.clear();

        scene.update(frame_delta);
    }

    println!("Done encoding.");
}