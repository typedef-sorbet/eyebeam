use raster::Color;
use crate::structs::vec3::Vec3;
use crate::structs::scene::Scene;

use super::shape::Shape;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction: direction.unit()
        }
    }

    pub fn trace(&self, scene: &Scene) -> Color { 
        let objects_and_distances: Vec<(&Box<dyn Shape>, f64)> = scene.shapes.iter().map(|s: &Box<dyn Shape>| (s, s.closest_distance_along_ray(self))).collect();

        if objects_and_distances.is_empty() {
            println!("No objects in scene");
            scene.background.clone()
        } else {
            let &(nearest_shape, shortest_distance) = objects_and_distances.iter().reduce(|acc, cur| if acc.1 < cur.1 {acc} else {cur}).unwrap();

            if shortest_distance.is_infinite() {
                scene.background.clone()
            } else {
                // No illumination
                // nearest_shape.color()

                let point: Vec3 = self.origin + (self.direction * shortest_distance);
                nearest_shape.color_at(&point, self, scene)
            }
        }
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        // let inverse = self.direction.invert();                                  // dumb local variable bullshit
        // inverse + (((*normal * normal.dot(&inverse)) + self.direction) * 2)     // what the fuck?
        let incident = self.direction;
        incident - *normal * (incident.dot(normal)) * 2
    }
}