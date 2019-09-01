use super::color::Color;
use super::vec3::Vec3;

pub struct Light {
    pub position: Vec3,
    pub intensity: Color,
}

impl Light {
    pub fn new(position: Vec3, intensity: Color) -> Self {
        assert!(position.is_point(), "position has to be Point type");
        Self { position, intensity }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn create_light_source() {
        let position = Vec3::point(0, 0, 0);
        let intensity = Color::new(1.0, 1.0, 1.0);
        let light = Light::new(position, intensity);
        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
