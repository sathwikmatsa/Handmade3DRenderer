use super::vec3::Vec3;
use super::float_cmp;
use super::ray::*;

static mut ID : usize = 0;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub id: usize,
    pub origin: Vec3,
    pub radius: f32,
}

impl Object for Sphere {
    fn intersection(&self, ray: &Ray) -> Vec<f32> {
        // TODO: this function assumes sphere is centered at origin
        let sphere_to_ray = ray.origin - Vec3::point(0, 0, 0);
        let a = ray.direction.magnitude_square();
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.magnitude_square() - 1.0;
        let discriminant = b * b - 4.0 * a * c;
        let mut intersections : Vec<f32>  = Vec::new();

        if discriminant >= 0.0 {
            intersections.push((-b - discriminant.sqrt()) / 2.0 * a);
            intersections.push((-b + discriminant.sqrt()) / 2.0 * a);
        }
        intersections
    }
}

impl Sphere {
    pub fn new(origin: Vec3, radius: f32) -> Self {
        assert!(origin.is_point(), "`origin` is not a point, failed to create Sphere");
        unsafe {
            ID += 1;
            let id = ID;
            Self{
                id,
                origin,
                radius,
            }
        }
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool{
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
        assert_eq!(s1.id, 1);
        assert_eq!(s2.id, 2);
        assert_eq!(s1, s2);
    }
}
