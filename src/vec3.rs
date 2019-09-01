use std::ops::{Add, Sub, Neg, Mul, Div};
use super::float_cmp;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Vec3Type {
    Point,
    Vector,
}

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub t: Vec3Type,
}

pub trait Vec3Coordinate {
    fn value(self) -> f32;
}

impl Vec3Coordinate for f32 {
    fn value(self) -> f32 {
        self
    }
}
impl Vec3Coordinate for i32 {
    fn value(self) -> f32 {
        self as f32
    }
}

impl Vec3 {
    pub fn point<T>(x: T, y: T, z: T) -> Self
    where T: Vec3Coordinate {
        Self {
            x: x.value(),
            y: y.value(),
            z: z.value(),
            t: Vec3Type::Point,
        }
    }
    pub fn vector<T>(x: T, y: T, z: T) -> Self
    where T: Vec3Coordinate {
        Self {
            x: x.value(),
            y: y.value(),
            z: z.value(),
            t: Vec3Type::Vector,
        }
    }
    pub fn new(v: Vec<f32>) -> Self {
        assert_eq!(v.len(), 4, "vector length doesn't equal 4, can't create Vec3");
        let vec_type : Vec3Type = if float_cmp::equal(v[3], 0.0) { Vec3Type::Vector } else { Vec3Type::Point };
        Self {
            x: v[0],
            y: v[1],
            z: v[2],
            t: vec_type,
        }
    }
    pub fn as_vec(&self) -> Vec<f32> {
        let w : f32 = if self.t == Vec3Type::Vector { 0.0 } else { 1.0 };
        vec![self.x, self.y, self.z, w]
    }
    pub fn magnitude(&self) -> f32 {
        assert_eq!(self.t, Vec3Type::Vector, "Cannot call magnitude method on Point type");
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn magnitude_square(&self) -> f32 {
        assert_eq!(self.t, Vec3Type::Vector, "Cannot call magnitude_square method on Point type");
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn normalize(&self) -> Self {
        assert_eq!(self.t, Vec3Type::Vector, "Cannot call normalize method on Point type");
        let m = self.magnitude();
        Self {
            x: self.x / m,
            y: self.y / m,
            z: self.z / m,
            t: self.t,
        }
    }
    pub fn reflect(&self, normal: Self) -> Self {
        assert_eq!(self.t, Vec3Type::Vector, "Reflect method is undefined for Point type");
        assert_eq!(normal.t, Vec3Type::Vector, "normal has to be Vector type");
        *self - normal * 2 * self.dot(normal)
    }
    pub fn dot(&self, other: Self) -> f32 {
        assert_eq!(self.t, Vec3Type::Vector, "Cannot call dot product on two point types");
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(&self, other: Self) -> Self {
        assert_eq!(self.t, Vec3Type::Vector, "Cannot call cross product on two point types");
        Vec3::vector(self.y*other.z - self.z*other.y, self.z*other.x - self.x*other.z, self.x*other.y - self.y*other.x)
    }
    pub fn is_point(&self) -> bool {
        self.t == Vec3Type::Point
    }
    pub fn is_vector(&self) -> bool {
        self.t == Vec3Type::Vector
    }
}

// operator overloading

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {

        let output_t;
        if self.t == Vec3Type::Point && other.t == Vec3Type::Point {
            panic!("Cannot add a point to a point.");
        } else if self.t == Vec3Type::Point {
            // point + vector -> point
            output_t = Vec3Type::Point;
        } else {
            // vector + vector -> vector
            output_t = other.t;
        }

        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            t: output_t,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {

        let output_t;
        if self.t == Vec3Type::Point && other.t == Vec3Type::Point {
            // p2 - p1 -> vector(p2-p1)
            output_t = Vec3Type::Vector;
        } else if self.t == Vec3Type::Point {
            // point - vector -> point
            output_t = Vec3Type::Point;
        } else if self.t == Vec3Type::Vector && other.t == Vec3Type::Vector{
            // vector - vector -> vector
            output_t = Vec3Type::Vector;
        } else {
            panic!("Cannot subtract a point from a vector.");
        }

        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            t: output_t,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            t: self.t,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs.value(),
            y: self.y * rhs.value(),
            z: self.z * rhs.value(),
            t: self.t,
        }
    }
}

impl Mul<i32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        Self {
            x: self.x * rhs.value(),
            y: self.y * rhs.value(),
            z: self.z * rhs.value(),
            t: self.t,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        if rhs.value() == 0.0 {
            panic!("Cannot divide by zero-valued `scalar`!");
        }
        Self {
            x: self.x / rhs.value(),
            y: self.y / rhs.value(),
            z: self.z / rhs.value(),
            t: self.t,
        }
    }
}

impl Div<i32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: i32) -> Self {
        if rhs.value() == 0.0 {
            panic!("Cannot divide by zero-valued `scalar`!");
        }
        Self {
            x: self.x / rhs.value(),
            y: self.y / rhs.value(),
            z: self.z / rhs.value(),
            t: self.t,
        }
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        float_cmp::equal(self.x, other.x) &&
        float_cmp::equal(self.y, other.y) &&
        float_cmp::equal(self.z, other.z) &&
        self.t == other.t
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_vec3() {
        let p = Vec3::point(3, -2, 5);
        let v = Vec3::vector(-2, 3, 1);
        let l = p + v;
        assert_eq!(l, Vec3::point(1, 1, 6));
    }
    #[test]
    fn subtracting_vec3() {
        let p1 = Vec3::point(3, 2, 1);
        let p2 = Vec3::point(5, 6, 7);
        let v = Vec3::vector(-2, -4, -6);
        assert_eq!(v, p1 - p2);

        let v1 = Vec3::vector(5, 6, 7);
        let p3 = Vec3::point(-2, -4, -6);
        assert_eq!(p3, p1 - v1);

        let v2 = Vec3::vector(3, 2, 1);
        assert_eq!(v, v2 - v1);
    }
    #[test]
    fn negating_vec3() {
        let v1 = Vec3::vector(1, 2, 3);
        let v2 = -v1;
        assert_eq!(v2, Vec3::vector(-1, -2, -3));
    }
    #[test]
    fn multiplying_with_scalar() {
        let v1 = Vec3::vector(1, 2, 3);
        let v2 = v1 * 3.0;
        assert_eq!(v2, Vec3::vector(3, 6, 9));
    }

    #[test]
    fn dividing_by_scalar() {
        let v1 = Vec3::vector(2, 4, 8);
        let v2 = v1 / 2.0;
        assert_eq!(v2, Vec3::vector(1, 2, 4));
    }
    #[test]
    fn magnitude_of_vector() {
        let v1 = Vec3::vector(1, 0, 0);
        assert_eq!(1.0, v1.magnitude());
        let v2 = Vec3::vector(1, 2, 3);
        let v3 = Vec3::vector(-1, -2, -3);
        assert_eq!(v2.magnitude(), v3.magnitude());
    }
    #[test]
    fn normalize_vector() {
        let v1 = Vec3::vector(4, 0, 0);
        let v2 = v1.normalize();
        assert_eq!(v2, Vec3::vector(1, 0, 0));

        let v = Vec3::vector(1, 2, 3);
        let uv = v.normalize();

        assert!(float_cmp::equal(1.0, uv.magnitude()));
    }
    #[test]
    fn dot_product() {
        let v1 = Vec3::vector(1, 2, 3);
        let v2 = Vec3::vector(2, 3, 4);
        assert!(float_cmp::equal(20.0, v1.dot(v2)));
    }
    #[test]
    fn cross_product() {
        let v1 = Vec3::vector(1, 2, 3);
        let v2 = Vec3::vector(2, 3, 4);
        let v3 = Vec3::vector(-1, 2, -1);
        let v4 = Vec3::vector(1, -2, 1);
        assert_eq!(v3, v1.cross(v2));
        assert_eq!(v4, v2.cross(v1));
    }
    #[test]
    fn compare_f32() {
        let i = 0.00001;
        let j = 0.00001;
        assert_eq!(true, float_cmp::equal(i, j));
    }
    #[test]
    fn reflecting_vector() {
        let v = Vec3::vector(1, -1, 0);
        let n = Vec3::vector(0, 1, 0);
        let r = v.reflect(n);
        assert_eq!(r, Vec3::vector(1, 1, 0));

        let v = Vec3::vector(0, -1, 0);
        let n = Vec3::vector(f32::sqrt(2.0)/2.0, f32::sqrt(2.0)/2.0, 0.0);
        let r = v.reflect(n);
        assert_eq!(r, Vec3::vector(1, 0, 0));
    }
}
