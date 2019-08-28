use super::vec3::Vec3;
use super::matrix::Matrix;
use super::ray::*;
use super::object::*;

#[derive(Debug, Clone)]
pub struct Sphere {
    id: usize,
    pub transform: Matrix,
}

impl Object for Sphere {
    fn intersection<'a >(&'a self, actual_ray: &Ray) -> Intersections<'a, Self> {
        let ray = actual_ray.transform(&self.transform.inverse_matrix());
        let sphere_to_ray = ray.origin - Vec3::point(0, 0, 0);
        let a = ray.direction.magnitude_square();
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.magnitude_square() - 1.0;
        let discriminant = b * b - 4.0 * a * c;
        let mut intersections  = Intersections::new();

        if discriminant >= 0.0 {
            intersections.push(Intersection {t: (-b - discriminant.sqrt()) / (2.0 * a), obj: self});
            intersections.push(Intersection {t: (-b + discriminant.sqrt()) / (2.0 * a), obj: self});
        }
        intersections
    }
}

impl Sphere {
    pub fn new() -> Self {
        let id = Self::get_uid();
        Self{
            id,
            transform : Matrix::identity_matrix(4),
        }
    }
    pub fn get_id(&self) -> usize {
        self.id
    }
    pub fn set_transform(&mut self, transform: Matrix) {
        assert_eq!(transform.n_rows, 4, "Not a transform, invalid dimensions");
        assert_eq!(transform.n_cols, 4, "Not a transform, invalid dimensions");
        self.transform = transform;
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn create_spheres() {
        let s1 = Sphere::new();
        let s2 = Sphere::new();
        assert_ne!(s1.id, s2.id);
    }
    #[test]
    fn default_transform() {
        let s = Sphere::new();
        assert_eq!(s.transform, Matrix::identity_matrix(4));
    }
    #[test]
    fn change_transformation() {
        let mut s = Sphere::new();
        let new_transform = Matrix::translation(1.0, 2.0, 3.0);
        s.set_transform(new_transform);
        assert_eq!(s.transform, Matrix::translation(1.0, 2.0, 3.0));
    }
}
