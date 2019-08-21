const EPSILON : f32 = 0.00001;

pub fn equal(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}
