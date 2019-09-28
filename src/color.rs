use std::ops::{Add, Sub, Mul};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

// predefined constants
pub const BLACK : Color = Color {red: 0.0, green: 0.0, blue: 0.0};
pub const WHITE : Color = Color {red: 1.0, green: 1.0, blue: 1.0};

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            red: r,
            green: g,
            blue: b,
        }
    }
    pub fn equals(&self, other: Self) -> bool {
        ((self.red - other.red).abs() < 0.00001) &&
        ((self.green - other.green).abs() < 0.00001) &&
        ((self.blue - other.blue).abs() < 0.00001)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        }
    }
}

impl Mul<i32> for Color {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        Self {
            red: self.red * (rhs as f32),
            green: self.green * (rhs as f32),
            blue: self.blue * (rhs as f32),
        }
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl Mul<Self> for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        // hadamard product
        Self {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn create_color_pixel() {
        let _c = Color::new(-0.5, 0.4, 1.7);
    }
    #[test]
    fn adding_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let c3 = Color::new(1.6, 0.7, 1.0);
        assert!(c3.equals(c1 + c2));
    }
    #[test]
    fn subtracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let c3 = Color::new(0.2, 0.5, 0.5);
        assert!(c3.equals(c1 - c2));
    }
    #[test]
    fn multiplying_color_with_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);
        let d = Color::new(0.4, 0.6, 0.8);
        assert!(d.equals(c*2));
    }
    #[test]
    fn multiplying_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        let c3 = Color::new(0.9, 0.2, 0.04);
        assert!(c3.equals(c1 * c2));
    }
}
