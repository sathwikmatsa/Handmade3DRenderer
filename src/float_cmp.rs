pub const EPSILON: f32 = 0.0001;

pub fn equal(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}

pub fn equal_debug(a: f32, b: f32) -> bool {
    if (a - b).abs() < EPSILON {
        true
    } else {
        println!("left:{}\nright:{}", a, b);
        false
    }
}

pub fn greater(a: f32, b: f32) -> bool {
    (a - b) > EPSILON
}
