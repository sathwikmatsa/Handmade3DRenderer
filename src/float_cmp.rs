const EPSILON : f32 = 0.0001;

pub fn equal(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}
