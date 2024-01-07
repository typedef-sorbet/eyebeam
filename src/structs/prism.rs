use std::cmp::min;

use raster::Color;

use super::{vec3::Vec3, shape::Shape, util::{fmin, fmax}, ray::Ray};

pub struct Prism { 
    pub corner_ll: Vec3,
    pub corner_ur: Vec3,
    pub color: Color
}

impl Shape for Prism {
    fn intersections(&self, ray: &Ray) -> Vec<f64> {
        let mut res: Vec<f64> = Vec::new();

        for axis in [Vec3::I, Vec3::J, Vec3::K] {
            res.append(self.intersect_on_axis(&axis, ray).as_mut());
        }

        return res;
    }

    fn color(&self) -> Color {
        self.color.clone()
    }

    fn normal_at(&self, point: &Vec3) -> Vec3 {
        for axis in [Vec3::I, Vec3::J, Vec3::K] {
            if (self.corner_ll.component(&axis) - point.component(&axis)).abs() < 0.00001 {
                return axis.invert();
            } else if (self.corner_ur.component(&axis) - point.component(&axis)).abs() < 0.00001 { 
                return axis;
            }
        }

        panic!("Given point is not on the surface of this prism: {:?}", point);
    }

    fn material(&self) -> raster::Image {
        raster::Image::blank(0, 0)
    }
}

impl Prism {
    pub fn new(corner_a: Vec3, corner_b: Vec3, color: Color) -> Self {
        Self {
            corner_ll: Vec3::new(fmin(corner_a.x, corner_b.x), fmin(corner_a.y, corner_b.y), fmin(corner_a.z, corner_b.z)),
            corner_ur: Vec3::new(fmax(corner_a.x, corner_b.x), fmax(corner_a.y, corner_b.y), fmax(corner_a.z, corner_b.z)),
            color
        }
    }

    pub fn intersect_on_axis(&self, axis: &Vec3, ray: &Ray) -> Vec<f64> {
        let other_axes: Vec<Vec3> = vec![Vec3::I, Vec3::J, Vec3::K].into_iter().filter(|v| v != axis).collect();

        let mut intersections = vec![];

        if ray.direction.component(axis).abs() > f64::EPSILON {
            for vertex in [self.corner_ll, self.corner_ur] {
                let intersect = (vertex.component(axis) - ray.origin.component(axis)) / ray.direction.component(axis);
                let point = ray.origin + (ray.direction * intersect);
                if self.contains(&point, &other_axes[0]) && self.contains(&point, &other_axes[1]) {
                    intersections.push(intersect);
                }
            }
        }

        return intersections;
    }

    pub fn contains(&self, point: &Vec3, axis: &Vec3) -> bool {
        self.corner_ll.component(axis) < point.component(axis) && point.component(axis) < self.corner_ur.component(axis)
    }
}