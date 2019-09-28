use super::color::*;
use super::vec3::*;
use std::fmt;

#[derive(Clone, PartialEq)]
pub struct Pattern {
    colors: Vec<Color>,
    function: fn(Vec3, usize) -> usize,
}

impl fmt::Debug for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "no of colors: {}", self.colors.len())
    }
}

impl Pattern {
    pub fn new(colors: Vec<Color>, function: fn(Vec3, usize) -> usize) -> Self {
        Self { colors, function }
    }
    pub fn color_at(&self, point_in_space: Vec3) -> Color {
        self.colors[(self.function)(point_in_space, self.colors.len())]
    }
    pub fn stripe(colors: Vec<Color>) -> Self {
        fn stripe_fn(point: Vec3, n: usize) -> usize {
            point.x.floor() as usize % n
        }
        Self {
            colors: colors,
            function: stripe_fn,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn create_stripe_pattern() {
        let colors = vec![WHITE, BLACK];
        let pattern = Pattern::stripe(colors);
        assert_eq!(pattern.colors[0], WHITE);
        assert_eq!(pattern.colors[1], BLACK);
    }
    #[test]
    fn get_color_at_point() {
        let colors = vec![WHITE, BLACK];
        let pattern = Pattern::stripe(colors);
        // constant in y
        assert_eq!(pattern.color_at(Vec3::point(0, 0, 0)), WHITE);
        assert_eq!(pattern.color_at(Vec3::point(0, 2, 0)), WHITE);
        assert_eq!(pattern.color_at(Vec3::point(0, 1, 0)), WHITE);
        // constant in z
        assert_eq!(pattern.color_at(Vec3::point(0, 0, 0)), WHITE);
        assert_eq!(pattern.color_at(Vec3::point(0, 0, 1)), WHITE);
        assert_eq!(pattern.color_at(Vec3::point(0, 0, 2)), WHITE);
        // alternates in x
        assert_eq!(pattern.color_at(Vec3::point(-1, 2, 0)), BLACK);
        assert_eq!(pattern.color_at(Vec3::point(0, 0, 0)), WHITE);
        assert_eq!(pattern.color_at(Vec3::point(1, 2, 0)), BLACK);
        assert_eq!(pattern.color_at(Vec3::point(2, 1, 0)), WHITE);
        assert_eq!(pattern.color_at(Vec3::point(2.3, 1.0, 0.0)), WHITE);
    }
}
