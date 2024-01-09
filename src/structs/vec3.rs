use std::ops::{Mul, Div, Add, Sub};

// Yeah, I know this has been done a million times before, but this is an exercise.
// TODO add cached length for speed increase?
#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

// Gluing together traits for simple operator math
impl<T> Mul<T> for Vec3
    where T: Into<f64> + Copy
{
    type Output = Vec3;

    fn mul(self, rhs: T) -> Self::Output {
        Vec3 { 
            x: self.x * rhs.into(),
            y: self.y * rhs.into(),
            z: self.z * rhs.into()
        }
    }
}

impl<T> Div<T> for Vec3 
    where T: Into<f64> + Copy
{
    type Output = Vec3;

    fn div(self, rhs: T) -> Self::Output {
        self * (1f64 / rhs.into())
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (rhs.invert())
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < f64::EPSILON && 
        (self.y - other.y).abs() < f64::EPSILON && 
        (self.z - other.z).abs() < f64::EPSILON
    }
}

// helper for converting 3-wide float slices to vectors
impl<T> From<&[T; 3]> for Vec3
    where T: Into<f64> + Copy
{
    fn from(value: &[T; 3]) -> Self {
        Self {
            x: value[0].into(),
            y: value[1].into(),
            z: value[2].into()
        }
    }
}

// helper for converting arbitrary float slices to vectors
impl<T> From<&[T]> for Vec3
    where T: Into<f64> + Copy
{
    fn from(value: &[T]) -> Self {
        assert!(value.len() >= 3);
        Self {
            x: value[0].into(),
            y: value[1].into(),
            z: value[2].into()
        }
    }
}

impl Vec3 {
    // canonical basis unit vectors for R3
    pub const I: Vec3 = Vec3 {x: 1f64, y: 0f64, z: 0f64};
    pub const J: Vec3 = Vec3 {x: 0f64, y: 1f64, z: 0f64};
    pub const K: Vec3 = Vec3 {x: 0f64, y: 0f64, z: 1f64};
    pub const O: Vec3 = Vec3 {x: 0f64, y: 0f64, z: 0f64};

    pub fn new<T>(x: T, y: T, z: T) -> Self 
        where T: Into<f64> + Copy
    {
        Vec3 { x: x.into(), y: y.into(), z: z.into() }
    }

    pub fn between(from: &Vec3, to: &Vec3) -> Vec3 {
        *to - *from
    }

    pub fn length(&self) -> f64 {
        (self.squid()).sqrt()
    }

    pub fn unit(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn invert(&self) -> Vec3 {
        Vec3 {
            x: self.x * -1f64,
            y: self.y * -1f64,
            z: self.z * -1f64
        }
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    pub fn cross(&self, rhs: &Self) -> Vec3 {
        // https://en.wikipedia.org/wiki/Cross_product#Coordinate_notation
        Vec3 { 
            x: self.y * rhs.z - self.z * rhs.y, 
            y: self.z * rhs.x - self.x * rhs.z, 
            z: self.x * rhs.y - self.y * rhs.x 
        }
    }

    pub fn squid(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn component(&self, axis: &Self) -> f64 {
        // Can't use a match here, since Vec3 cannot derive Eq
        if *axis == Vec3::I {
            self.x
        } else if *axis == Vec3::J {
            self.y
        } else if *axis == Vec3::K {
            self.z
        } else {
            panic!("component only accepts one of &Vec3::{{I, J, K}}");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::vec3::Vec3;
    
    #[test]
    fn vectors() {
        // addition
        let a = Vec3 {x: 1f64, y: 2f64, z: 3f64};
        assert_eq!(a + a, Vec3 {x: 2f64, y: 4f64, z: 6f64});

        // subtraction
        assert_eq!(a - a, Vec3::O);

        // multiplication
        assert_eq!(a * 3, Vec3 {x: 3f64, y: 6f64, z: 9f64});

        // division
        assert_eq!(a / 3, Vec3 {x: 1f64/3f64, y: 2f64/3f64, z: 1f64});

        // length
        assert_eq!(Vec3::I.length(), 1f64);
        assert_eq!(Vec3::O.length(), 0f64);
        assert_eq!(a.length(), 14f64.sqrt());

        // unit
        // assert_eq!(a.unit(), Vec3 {x: 0.267261241912424f64, y: 0.534522483824849f64, z: 0.801783725737273f64});

        // dot product
        assert_eq!(Vec3::I.dot(&Vec3::I), 1f64);
        assert_eq!(Vec3::I.dot(&Vec3::J), 0f64);
        assert!(Vec3::new(1, 1, 1).dot(&Vec3::new(2, 0, 0)) - 2.0 < 0.0001);

        // cross product
        assert_eq!(Vec3::I.cross(&Vec3::J), Vec3::K);
        assert_eq!(Vec3::J.cross(&Vec3::K), Vec3::I);
        assert_eq!(Vec3::K.cross(&Vec3::I), Vec3::J);
        assert_eq!(Vec3::I.cross(&Vec3::I), Vec3::O);
    }
}