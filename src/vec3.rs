use std::ops::{Add, Sub, Neg, Mul, Div};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Vec3Type {
    Point,
    Vector,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
    t: Vec3Type,
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
}
