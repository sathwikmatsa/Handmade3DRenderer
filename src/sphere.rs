use super::vec3::Vec3;
use super::light::Light;
use super::color::Color;
use super::material::Material;
use super::matrix::Matrix;
use super::ray::*;
use super::object::*;
use super::intersection::*;

#[derive(Debug, Clone)]
pub struct Sphere {
    id: usize,
    pub transform: Matrix,
    pub material: Material,
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
    fn normal_at(&self, world_point: Vec3) -> Vec3 {
        let object_point = self.transform.inverse_matrix() * world_point;
        let object_normal = object_point - Vec3::point(0, 0, 0);
        let mut world_normal = (self.transform.inverse_matrix().transpose() * object_normal.as_vec())
                                .get_tuple();
        world_normal[3] = 0.0;
        Vec3::new(world_normal).normalize()
    }
    fn lighting_at(&self, point: Vec3, eye_v: Vec3, light: Light) -> Color {
        let normal_v = self.normal_at(point);
        let eye_v = eye_v.normalize();
        self.material.lighting(light, point, eye_v, normal_v)
    }
}

impl Sphere {
    pub fn new() -> Self {
        let id = Self::get_uid();
        Self{
            id,
            transform : Matrix::identity_matrix(4),
            material: Material::default(),
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
    pub fn set_material(&mut self, material: Material) {
        self.material = material;
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
    #[test]
    fn normal_on_x() {
        let s = Sphere::new();
        let n = s.normal_at(Vec3::point(1, 0, 0));
        assert_eq!(n, Vec3::vector(1, 0, 0));
    }
    #[test]
    fn normal_on_y() {
        let s = Sphere::new();
        let n = s.normal_at(Vec3::point(0, 1, 0));
        assert_eq!(n, Vec3::vector(0, 1, 0));
    }
    #[test]
    fn normal_on_z() {
        let s = Sphere::new();
        let n = s.normal_at(Vec3::point(0, 0, 1));
        assert_eq!(n, Vec3::vector(0, 0, 1));
    }
    #[test]
    fn normal_on_non_axial() {
        let s = Sphere::new();
        let c = f32::sqrt(3.0) / 3.0;
        let n = s.normal_at(Vec3::point(c, c, c));
        assert_eq!(n, Vec3::vector(c, c, c));
    }
    #[test]
    fn normal_is_normalized() {
        let s = Sphere::new();
        let c = f32::sqrt(3.0) / 3.0;
        let n = s.normal_at(Vec3::point(c, c, c));
        assert_eq!(n, n.normalize());
    }
    #[test]
    fn normal_on_translated_sphere() {
        let mut s = Sphere::new();
        s.set_transform(Matrix::translation(0.0, 1.0, 0.0));
        let n = s.normal_at(Vec3::point(0.0, 1.70711, -0.70711));
        assert_eq!(n, Vec3::vector(0.0, 0.70711, -0.70711));
    }
    #[test]
    fn normal_on_transformed_sphere() {
        let mut s = Sphere::new();
        let m = Matrix::scaling(1.0, 0.5, 1.0) * &Matrix::rotation_z(std::f32::consts::PI / 5.0);
        s.set_transform(m);
        let n = s.normal_at(Vec3::point(0.0, f32::sqrt(2.0) / 2.0, -1.0*f32::sqrt(2.0) / 2.0));
        assert_eq!(n, Vec3::vector(0.0, 0.97014, -0.24254));
    }
    #[test]
    fn sphere_material() {
        let mut s = Sphere::new();
        assert_eq!(s.material, Material::default());
        let mut material = Material::default();
        material.shininess = 150.0;
        let mat_c = material.clone();
        s.set_material(material);
        assert_eq!(s.material, mat_c);
    }
}
