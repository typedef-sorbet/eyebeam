mod structs;

use std::{path::PathBuf, thread, sync::{Arc, Mutex}};

use argparse::{ArgumentParser, StoreTrue, Store};
use image::Rgba;
use raster::{Color, Image};
use structs::{scene::Scene, camera::Camera, vec3::Vec3, light::Light, plane::Plane, appearance::Appearance, finish::Finish, color::color_from_hex, animate::Animate};

use ndarray::Array3;
use video_rs::{Encoder, EncoderSettings, Locator, Time};

use crate::structs::mesh::ColoredMesh;

fn main() {
    let mut video: bool = false;
    let mut duration: f64 = 3.0;
    let mut frame_rate: usize = 60;
    let mut filename = String::new();

    {
        let mut ap = ArgumentParser::new();

        ap.set_description("Eyebeam renderer.");

        ap.refer(&mut video)
          .add_option(&["-v", "--video"], StoreTrue, "Renders a video instead of an image.");

        ap.refer(&mut duration)
          .add_option(&["-d", "--duration"], Store, "Duration of the video.");

        ap.refer(&mut frame_rate)
          .add_option(&["-f", "--frame-rate"], Store, "Frame rate of the video.");

        ap.refer(&mut filename)
          .add_option(&["-o", "--output"], Store, "Filename to store the rendered image/video under.");

        ap.parse_args_or_exit();
    }

    if filename.is_empty() {
        filename = (if video {"video.mp4"} else {"img.png"}).into();
    }

    let camera: Camera = Camera::new(
        Vec3::new(-5, -5, -12),
        Vec3::O,
        4.0, 9.0 / 4.0
    );
    
    let background = Rgba([0, 0, 0, 255]);

    let mut scene: Scene = Scene::new(camera, background);

    let cyan_appearance = Appearance::new(color_from_hex("#00FFFF").unwrap(), Finish::new(0.0, 0.7, 1.0, 1.0));
    let red_appearance = Appearance::new(color_from_hex("#FF0000").unwrap(), Finish::new(0.0, 0.7, 0.7, 0.7));

    // shapes
    scene.shapes.push(Box::new(ColoredMesh::new("res/teapot.obj", Vec3::new(-5, -5, 0), red_appearance)));

    // plane
    scene.shapes.push(Box::new(Plane::new(Vec3::O + Vec3::J, Vec3::J.invert(), cyan_appearance)));

    // light
    scene.lights.push(Light::new(Vec3::new(5, -5, -5) * 10, color_from_hex("#FFFFFF").unwrap()));

    
    if video {
        // video encoder setup
        video_rs::init().unwrap();
        
        let mut pixel_data = Arc::new(Mutex::new(Array3::<u8>::zeros((900, 1600, 3))));
        let frame_delta: f64 = 1.0 / frame_rate as f64;

        scene.camera.add_camera_move(Vec3::new(5, -5, -12), 3.0, Some(Vec3::O));

        let destination: Locator = PathBuf::from("out/video.mp4").into();
        let settings = EncoderSettings::for_h264_yuv420p(1600, 900, false);
    
        let mut video_encoder = Encoder::new(&destination, settings).expect("Unable to create encoder");
    
        let video_frame_time = Time::from_nth_of_a_second(frame_rate);
        let mut video_position = Time::zero();

        for frame_num in 0..(duration / frame_delta).ceil() as i32 {
            println!("Drawing frame {}...", frame_num);
    
            thread::scope(|scope| {
                // shadow these so that they don't need to get moved into the scoped threads
                let scene = &scene;
                let pixel_data = &pixel_data;
    
                for j in 0..16 {
                    scope.spawn(move || {
                        for x in j*100..(j+1)*100 {
                            for y in 0..900 {
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
                }
            });
    
            println!("Encoding frame {}...", frame_num);
            // let frame = Array3::from_shape_fn((900, 1600, 3), |(y, x, c)| pixels[900 * x + y][c]);
            video_encoder.encode(&pixel_data.lock().unwrap(), &video_position).expect(&format!("Unable to encode {}th frame", frame_num));
        
            // update scene and video encoder
            scene.update(frame_delta);
            video_position = video_position.aligned_with(&video_frame_time).add();
        
            // pixels.clear();
            pixel_data = Arc::new(Mutex::new(Array3::<u8>::zeros((900, 1600, 3))));
        }
    
        video_encoder.finish().expect("Unable to finish encoding.");
    } else {
        let image: Arc<Mutex<Image>> = Arc::new(Mutex::new(Image::blank(1600, 900)));

        thread::scope(|scope| {
            // shadow these so that they don't need to get moved into the scoped threads
            let scene = &scene;
            let image = &image;

            for j in 0..16 {
                scope.spawn(move || {
                    for x in j*100..(j+1)*100 {
                        for y in 0..900 {
                            let color = scene.trace((x as f64 / 1600.0) - 0.5, (y as f64 / 900.0) - 0.5);
                            
                            {
                                let mut img = image.lock().unwrap();
                                img.set_pixel(x, y, Color { r: color.0[0], g: color.0[1], b: color.0[2], a: 0xFF }).expect("Unable to set pixel");
                            }
                        }
                    }
                });
            }
        });

        raster::save(&image.lock().unwrap(), &format!("out/{}", filename)).expect("Unable to save");
    }

    println!("Done.")
}