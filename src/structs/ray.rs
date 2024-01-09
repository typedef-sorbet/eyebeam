use image::Rgba;
use crate::structs::vec3::Vec3;
use crate::structs::scene::Scene;

use super::shape::Shape;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    const MAX_DEPTH: i32 = 16;

    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction: direction.unit()
        }
    }

    pub fn trace(&self, scene: &Scene, depth: i32) -> Rgba<u8> {
        if depth > Self::MAX_DEPTH {
            return scene.background
        }

        let objects_and_distances: Vec<(&Box<dyn Shape + Send + Sync>, f64)> = scene.shapes.iter().map(|s: &Box<dyn Shape + Send + Sync>| (s, s.closest_distance_along_ray(self))).collect();

        if objects_and_distances.is_empty() {
            println!("No objects in scene");
            scene.background
        } else {
            let &(nearest_shape, shortest_distance) = objects_and_distances.iter().reduce(|acc, cur| if acc.1 < cur.1 {acc} else {cur}).unwrap();

            if shortest_distance.is_infinite() {
                scene.background
            } else {
                // No illumination
                // nearest_shape.color()

                let point: Vec3 = self.origin + (self.direction * shortest_distance);
                nearest_shape.color_at(&point, self, scene, depth + 1)
            }
        }
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        let incident = self.direction;
        incident - *normal * (incident.dot(normal)) * 2
    }

    // bool intersect_triangle(
    //     in Ray R, in vec3 A, in vec3 B, in vec3 C, out float t, 
    //     out float u, out float v, out vec3 N
    // ) { 
    //    vec3 E1 = B-A;
    //    vec3 E2 = C-A;
    //          N = cross(E1,E2);
    //    float det = -dot(R.Dir, N);
    //    float invdet = 1.0/det;
    //    vec3 AO  = R.Origin - A;
    //    vec3 DAO = cross(AO, R.Dir);
    //    u =  dot(E2,DAO) * invdet;
    //    v = -dot(E1,DAO) * invdet;
    //    t =  dot(AO,N)  * invdet; 
    //    return (det >= 1e-6 && t >= 0.0 && u >= 0.0 && v >= 0.0 && (u+v) <= 1.0);
    // }

    pub fn intersect_triangle(&self, a: &Vec3, b: &Vec3, c: &Vec3) -> Option<Vec3> {
        let e1 = *b - *a;
        let e2 = *c - *a;
        let n = e1.cross(&e2);

        let det = -1.0 * self.direction.dot(&n);
        let invdet = 1.0 / det;

        let ao = self.origin - *a;
        let dao = ao.cross(&self.direction);

        let u = e2.dot(&dao) * invdet;
        let v = -1.0 * e1.dot(&dao) * invdet;
        let t = ao.dot(&n) * invdet;

        if det >= 1e-6 && t >= 0.0 && u >= 0.0 && v >= 0.0 && (u+v) <= 1.0 {
            Some(self.origin + self.direction * t)
        } else {
            None
        }
    }
}