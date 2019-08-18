#[derive(Debug)]
enum Vec3Type {
    Point,
    Vector,
}

#[derive(Debug)]
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
