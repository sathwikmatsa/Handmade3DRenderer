use super::color::*;
use super::matrix::Matrix;
use super::vec3::*;
use std::fmt;

#[derive(Clone)]
pub struct Pattern {
    colors: Vec<Color>,
    function: fn(Vec3, &Vec<Color>) -> Color,
    pub transform: Matrix,
}

impl fmt::Debug for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "no of colors: {}", self.colors.len())
    }
}

impl Pattern {
    pub fn new(colors: Vec<Color>, function: fn(Vec3, &Vec<Color>) -> Color) -> Self {
        Self { colors, function, transform: Matrix::identity_matrix(4) }
    }
    pub fn color_at(&self, point_in_space: Vec3) -> Color {
        (self.function)(point_in_space, &self.colors)
    }
    pub fn pattern_at(&self, world_point: Vec3, obj_transform: &Matrix) -> Color {
        let obj_point = obj_transform.inverse_matrix() * world_point;
        let pattern_point = self.transform.inverse_matrix() * obj_point;

        self.color_at(pattern_point)
    }
    // predefined patterns
    pub fn stripe(colors: Vec<Color>) -> Self {
        fn stripe_fn(point: Vec3, colors: &Vec<Color>) -> Color {
            let n = colors.len();
            colors[point.x.floor() as usize % n]
        }
        Self {
            colors: colors,
            function: stripe_fn,
            transform: Matrix::identity_matrix(4),
        }
    }
    pub fn gradient(colors: Vec<Color>) -> Self {
        assert_eq!(colors.len(), 2, "Gradient pattern takes exactly two colors.");
        fn gradient_fn(point: Vec3, gradient: &Vec<Color>) -> Color {
            let distance = gradient[1] - gradient[0];
            let fraction = point.x - point.x.floor();
            gradient[0] + distance * fraction
        }
        Self {
            colors: colors,
            function: gradient_fn,
            transform: Matrix::identity_matrix(4),
        }
    }
    pub fn ring(colors: Vec<Color>) -> Self {
        fn ring_fn(point: Vec3, colors : &Vec<Color>) -> Color {
            let n = colors.len();
            colors[((point.x * point.x) + (point.z * point.z)).sqrt().floor() as usize % n]
        }
        Self {
            colors: colors,
            function: ring_fn,
            transform: Matrix::identity_matrix(4),
        }
    }
    pub fn checkers(colors: Vec<Color>) -> Self {
        fn checkers_fn(point: Vec3, colors : &Vec<Color>) -> Color {
            let n = colors.len();
            colors[(point.x.floor() + point.y.floor() + point.z.floor()) as usize % n]
        }
        Self {
            colors: colors,
            function: checkers_fn,
            transform: Matrix::identity_matrix(4),
        }
        // TODO: UV mapping for spherical texture mapping
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
    #[test]
    fn stripes_with_pattern_transformation() {
        let colors = vec![WHITE, BLACK];
        let mut pattern = Pattern::stripe(colors);
        pattern.transform = Matrix::scaling(2.0, 2.0, 2.0);
        let color = pattern.pattern_at(Vec3::point(1.5, 0.0, 0.0), &Matrix::identity_matrix(4));
        assert_eq!(color, WHITE);
    }
    #[test]
    fn stripes_with_obj_transformation() {
        let colors = vec![WHITE, BLACK];
        let pattern = Pattern::stripe(colors);
        let color = pattern.pattern_at(Vec3::point(1.5, 0.0, 0.0), &Matrix::scaling(2.0, 2.0, 2.0));
        assert_eq!(color, WHITE);
    }
    #[test]
    fn stripes_with_obj_pattern_transformation() {
        let colors = vec![WHITE, BLACK];
        let mut pattern = Pattern::stripe(colors);
        pattern.transform = Matrix::translation(0.5, 0.0, 0.0);
        let color = pattern.pattern_at(Vec3::point(2.5, 0.0, 0.0), &Matrix::scaling(2.0, 2.0, 2.0));
        assert_eq!(color, WHITE);
    }
    #[test]
    fn gradient_pattern() {
        let pattern = Pattern::gradient(vec![WHITE, BLACK]);
        assert_eq!(pattern.color_at(Vec3::point(0, 0, 0)), WHITE);
        assert_eq!(pattern.color_at(Vec3::point(0.25, 0.0, 0.0)), Color::new(0.75, 0.75, 0.75));
        assert_eq!(pattern.color_at(Vec3::point(0.5, 0.0, 0.0)), Color::new(0.5, 0.5, 0.5));
        assert_eq!(pattern.color_at(Vec3::point(0.75, 0.0, 0.0)), Color::new(0.25, 0.25, 0.25));
    }
    #[test]
    fn ring_pattern() {
        let pattern = Pattern::ring(vec![WHITE, BLACK]);
        assert_eq!(pattern.color_at(Vec3::point(0, 0, 0)), WHITE);
        assert_eq!(pattern.color_at(Vec3::point(1, 0, 0)), BLACK);
        assert_eq!(pattern.color_at(Vec3::point(0, 0, 1)), BLACK);
        assert_eq!(pattern.color_at(Vec3::point(0.708, 0.0, 0.708)), BLACK);
    }
    #[test]
    fn checkers_pattern() {
        let pattern = Pattern::checkers(vec![WHITE, BLACK]);
        assert_eq!(pattern.color_at(Vec3::point(0, 0, 0)), WHITE);
        // repeat in x
        assert_eq!(pattern.color_at(Vec3::point(0.99, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(Vec3::point(1.01, 0.0, 0.0)), BLACK);
        // repeat in y
        assert_eq!(pattern.color_at(Vec3::point(0.0, 0.99, 0.0)), WHITE);
        assert_eq!(pattern.color_at(Vec3::point(0.0, 1.01, 0.0)), BLACK);
        // repeat in z
        assert_eq!(pattern.color_at(Vec3::point(0.0, 0.0, 0.99)), WHITE);
        assert_eq!(pattern.color_at(Vec3::point(0.0, 0.0, 1.01)), BLACK);
    }
}
