use super::vec3::Vec3;
use super::object::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        assert!(origin.is_point(), "`origin` is not a point, failed to create Ray");
        assert!(direction.is_vector(), "`direction` is not a vector, failed to create Ray");
        Self {
            origin,
            direction,
        }
    }
    pub fn position(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
    pub fn intersect<'a, T: Object>(&self, obj: &'a T) -> Intersections<'a, T> {
        obj.intersection(self)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn create_ray() {
        let origin = Vec3::point(0, 0, 0);
        let direction = Vec3::vector(1, 2, 3);
        let ray = Ray::new(origin, direction);
        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);
    }
    #[test]
    fn position_of_ray() {
        let r = Ray::new(Vec3::point(2, 3, 4), Vec3::vector(1, 0, 0));
        assert_eq!(r.position(0.0), Vec3::point(2, 3, 4));
        assert_eq!(r.position(1.0), Vec3::point(3, 3, 4));
        assert_eq!(r.position(-1.0), Vec3::point(1, 3, 4));
        assert_eq!(r.position(2.5), Vec3::point(4.5, 3.0, 4.0));
    }
}
