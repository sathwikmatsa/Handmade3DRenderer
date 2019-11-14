pub const EPSILON: f32 = 0.0001;
pub const SQRT2: f32 = std::f32::consts::SQRT_2;
pub const SQRT3: f32 = 1.732_050_8;
pub const INVSQRT2: f32 = std::f32::consts::FRAC_1_SQRT_2;

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
