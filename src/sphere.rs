use super::vec3::Vec3;
use super::float_cmp;
use super::ray::*;
use super::object::*;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    id: usize,
    pub origin: Vec3,
    pub radius: f32,
}

impl Object for Sphere {
    fn intersection<'a >(&'a self, ray: &Ray) -> Intersections<'a, Self> {
        // TODO: this function assumes sphere is centered at origin
        let sphere_to_ray = ray.origin - Vec3::point(0, 0, 0);
        let a = ray.direction.magnitude_square();
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.magnitude_square() - 1.0;
        let discriminant = b * b - 4.0 * a * c;
        let mut intersections  = Intersections::new();

        if discriminant >= 0.0 {
            intersections.push(Intersection {t: (-b - discriminant.sqrt()) / 2.0 * a, obj: self});
            intersections.push(Intersection {t: (-b + discriminant.sqrt()) / 2.0 * a, obj: self});
        }
        intersections
    }
}

impl Sphere {
    pub fn new(origin: Vec3, radius: f32) -> Self {
        assert!(origin.is_point(), "`origin` is not a point, failed to create Sphere");
        let id = Self::get_uid();
        Self{
            id,
            origin,
            radius,
        }
    }
    pub fn get_id(&self) -> usize {
        self.id
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        (self.origin == other.origin) && float_cmp::equal(self.radius, other.radius)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn create_spheres() {
        let s1 = Sphere::new(Vec3::point(0, 0, 0), 1.0);
        let s2 = Sphere::new(Vec3::point(0, 0, 0), 1.0);
        assert_ne!(s1.id, s2.id);
        assert!(s1 == s2);
    }
}
