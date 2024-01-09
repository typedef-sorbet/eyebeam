mod structs;

use std::{path::PathBuf, thread, sync::{Arc, Mutex}};

use image::Rgba;
use structs::{scene::Scene, camera::Camera, vec3::Vec3, sphere::Sphere, light::Light, plane::Plane, prism::Prism, appearance::Appearance, finish::Finish, color::color_from_hex, animate::Animate};

use ndarray::Array3;
use video_rs::{Encoder, EncoderSettings, Locator, Time, PixelFormat, Options};

use crate::structs::mesh::ColoredMesh;

fn main() {
    video_rs::init().unwrap();

    let mut camera: Camera = Camera::new(
        Vec3::new(-5, -5, -12),
        Vec3::O,
        4.0, 9.0 / 4.0
    );

    let frame_delta: f64 = 1.0 / 60.0;
    let duration: f64 = 3.0;

    camera.add_camera_move(Vec3::new(5, -5, -12), 3.0, Some(Vec3::O));
    
    let background = Rgba([0, 0, 0, 255]);

    let mut scene: Scene = Scene::new(camera, background);

    let cyan_appearance     = Appearance::new(color_from_hex("#00FFFF").unwrap(), Finish::new(0.0, 0.7, 1.0, 0.7));

    // spheres
    scene.shapes.push(Box::new(ColoredMesh::new("res/teapot.obj", Vec3::new(-5, 0, 0), cyan_appearance.clone())));   // cyan

    // plane
    scene.shapes.push(Box::new(Plane::new(Vec3::O + Vec3::J, Vec3::J.invert(), cyan_appearance)));

    scene.lights.push(Light::new(Vec3::new(5, -5, -5) * 10, color_from_hex("#FFFFFF").unwrap()));
    // scene.lights.push(Light::new(Vec3::new(0, 0, -10), color_from_hex("#FF0000").unwrap()));

    let destination: Locator = PathBuf::from("out/video.mp4").into();
    let settings = EncoderSettings::for_h264_yuv420p(1600, 900, false);

    let mut encoder = Encoder::new(&destination, settings).expect("Unable to create encoder");

    let video_frame_time = Time::from_nth_of_a_second(60);
    let mut position = Time::zero();

    let mut pixel_data = Arc::new(Mutex::new(Array3::<u8>::zeros((900, 1600, 3))));

    for frame_num in 0..(duration / frame_delta).ceil() as i32 {
        println!("Drawing frame {}...", frame_num);

        thread::scope(|scope| {
            scope.spawn(|| {
                for x in 0..800 {
                    for y in 0..450 {
                        let color = scene.trace((x as f64 / 1600.0) - 0.5, (y as f64 / 900.0) - 0.5);
    
                        {
                            let mut data = pixel_data.lock().unwrap();
                            data[[y, x, 0]] = color.0[0];
                            data[[y, x, 1]] = color.0[1];
                            data[[y, x, 2]] = color.0[2];
                        }
                    }
                }
            });
    
            scope.spawn(|| {
                for x in 0..800 {
                    for y in 450..900 {
                        let color = scene.trace((x as f64 / 1600.0) - 0.5, (y as f64 / 900.0) - 0.5);
    
                        {
                            let mut data = pixel_data.lock().unwrap();
                            data[[y, x, 0]] = color.0[0];
                            data[[y, x, 1]] = color.0[1];
                            data[[y, x, 2]] = color.0[2];
                        }
                    }
                }
            });
    
            scope.spawn(|| {
                for x in 800..1600 {
                    for y in 0..450 {
                        let color = scene.trace((x as f64 / 1600.0) - 0.5, (y as f64 / 900.0) - 0.5);
    
                        {
                            let mut data = pixel_data.lock().unwrap();
                            data[[y, x, 0]] = color.0[0];
                            data[[y, x, 1]] = color.0[1];
                            data[[y, x, 2]] = color.0[2];
                        }
                    }
                }
            });

            scope.spawn(|| {
                for x in 800..1600 {
                    for y in 450..900 {
                        let color = scene.trace((x as f64 / 1600.0) - 0.5, (y as f64 / 900.0) - 0.5);
        
                        {
                            let mut data = pixel_data.lock().unwrap();
                            data[[y, x, 0]] = color.0[0];
                            data[[y, x, 1]] = color.0[1];
                            data[[y, x, 2]] = color.0[2];
                        }
                    }
                }
            });
        });
        
        println!("Encoding frame {}...", frame_num);
        // let frame = Array3::from_shape_fn((900, 1600, 3), |(y, x, c)| pixels[900 * x + y][c]);
        encoder.encode(&pixel_data.lock().unwrap(), &position).expect(&format!("Unable to encode {}th frame", frame_num));

        // update scene and video encoder
        scene.update(frame_delta);
        position = position.aligned_with(&video_frame_time).add();

        // pixels.clear();
        pixel_data = Arc::new(Mutex::new(Array3::<u8>::zeros((900, 1600, 3))));
    }

    encoder.finish().expect("Unable to finish encoding.");
    println!("Done.")
}