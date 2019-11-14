use super::color::Color;
use super::float_cmp;
use super::intersection::*;
use super::light::Light;
use super::material::Material;
use super::matrix::Matrix;
use super::object::*;
use super::ray::*;
use super::vec3::Vec3;

#[derive(Debug)]
pub struct Plane {
    id: usize,
    pub transform: Matrix,
    pub material: Material,
}

impl Object for Plane {
    fn intersection(&self, actual_ray: &Ray) -> Intersections {
        let ray = actual_ray.transform(&self.transform.inverse_matrix());
        let mut intersections = Intersections::new();
        if !float_cmp::equal(ray.direction.y, 0.0) {
            let t = -ray.origin.y / ray.direction.y;
            intersections.push(Intersection { t, obj_id: self.id });
        }
        intersections
    }
    fn normal_at(&self, _world_point: Vec3) -> Vec3 {
        let object_normal = Vec3::vector(0, 1, 0);
        let mut world_normal =
            (self.transform.inverse_matrix().transpose() * object_normal.as_vec()).get_tuple();
        world_normal[3] = 0.0;
        Vec3::new(&world_normal).normalize()
    }
    fn lighting_at(
        &self,
        point: Vec3,
        eye_v: Vec3,
        normal_v: Vec3,
        light: Light,
        in_shadow: bool,
    ) -> Color {
        let eye_v = eye_v.normalize();
        self.material
            .lighting(&self.transform, light, point, eye_v, normal_v, in_shadow)
    }
    fn mut_material(&mut self) -> &mut Material {
        &mut self.material
    }
    fn material(&self) -> &Material {
        &self.material
    }
}

impl Plane {
    pub fn new() -> Self {
        let id = get_object_uid();
        Self {
            id,
            transform: Matrix::identity_matrix(4),
            material: Material::default(),
        }
    }
    pub fn get_id(&self) -> usize {
        self.id
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn plane_normal() {
        let plane = Plane::new();
        let n1 = plane.normal_at(Vec3::point(0, 0, 0));
        let n2 = plane.normal_at(Vec3::point(10, 0, -10));
        let n3 = plane.normal_at(Vec3::point(-5, 0, 150));
        assert_eq!(n1, Vec3::vector(0, 1, 0));
        assert_eq!(n2, Vec3::vector(0, 1, 0));
        assert_eq!(n3, Vec3::vector(0, 1, 0));
    }
}
