use std::collections::HashMap;
use super::object::Object;
use super::sphere::Sphere;
use super::light::Light;
use super::vec3::Vec3;
use super::color::Color;
use super::matrix::Matrix;
use super::ray::Ray;
use super::intersection::*;

pub struct World {
    pub objects: HashMap<usize, Box<Object>>,
    pub lights: Vec<Light>,
}

impl World {
    pub fn new() -> Self {
        World {
            objects: HashMap::new(),
            lights: Vec::new(),
        }
    }
    pub fn default() -> Self {
        let mut world = World::new();
        world.lights.push(Light::new(Vec3::point(-10, -10, -10), Color::new(1.0, 1.0, 1.0)));
        let mut s1 = Sphere::new();
        let id1 = s1.get_id();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.7;
        let mut s2 = Sphere::new();
        let id2 = s2.get_id();
        s2.set_transform(Matrix::scaling(0.5, 0.5, 0.5));
        world.objects.insert(id1, Box::new(s1));
        world.objects.insert(id2, Box::new(s2));
        world
    }
    pub fn intersect_with(&self, ray: &Ray) -> Intersections {
        let mut intersections = Intersections::new();
        for (_, boxed_obj) in self.objects.iter() {
            intersections.crossings.extend((*boxed_obj).intersection(ray).crossings);
        }
        intersections.crossings.sort();
        intersections
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use super::super::float_cmp;
    #[test]
    fn create_world() {
        let world = World::new();
        assert_eq!(world.objects.len(), 0);
        assert_eq!(world.lights.len(), 0);
    }
    #[test]
    fn default_world() {
        let world = World::default();
        assert_eq!(world.objects.len(), 2);
        assert_eq!(world.lights[0], Light::new(Vec3::point(-10, -10, -10), Color::new(1.0, 1.0, 1.0)));
    }
    #[test]
    fn intersect_with_ray() {
        let world = World::default();
        let ray = Ray::new(Vec3::point(0, 0, -5), Vec3::vector(0, 0, 1));
        let xs = world.intersect_with(&ray);
        assert_eq!(xs.len(), 4);
        assert!(float_cmp::equal(xs[0].t, 4.0));
        assert!(float_cmp::equal(xs[1].t, 4.5));
        assert!(float_cmp::equal(xs[2].t, 5.5));
        assert!(float_cmp::equal(xs[3].t, 6.0));
    }
}
