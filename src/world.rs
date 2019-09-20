use std::collections::HashMap;
use super::object::Object;
use super::sphere::Sphere;
use super::light::Light;
use super::vec3::Vec3;
use super::color::Color;
use super::matrix::Matrix;
use super::ray::Ray;
use super::intersection::*;
use super::float_cmp::EPSILON;

pub struct World {
    pub objects: HashMap<usize, Box<Object>>,
    pub lights: Vec<Light>,
}

unsafe impl Sync for World {}

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
    pub fn compute_state(&self, ray: &Ray, intersection: &Intersection) -> IntersectionState {
        let point = ray.position(intersection.t);
        let mut normalv = self.objects.get(&intersection.obj_id)
                        .unwrap().normal_at(point);
        let eyev = -ray.direction;
        // checking for ray originating from inside the object
        let inside;
        if normalv.dot(eyev) < 0.0 {
            inside = true;
            normalv = -normalv;
        } else {
            inside = false;
        }

        let point = ray.position(intersection.t);
        let over_point = point + normalv * EPSILON;

        IntersectionState {
            t: intersection.t,
            obj_id: intersection.obj_id,
            point: point,
            over_point: over_point,
            eyev: eyev,
            normalv: normalv,
            inside: inside,
        }
    }
    pub fn shade_hit(&self, state: IntersectionState) -> Color {
        let mut color = Color::new(0.0, 0.0, 0.0);
        for (light_index, light) in self.lights.iter().enumerate() {
            let in_shadow = self.is_shadowed(state.over_point, light_index);
            color = color + self.objects.get(&state.obj_id).unwrap()
                        .lighting_at(state.point, state.eyev, state.normalv, *light, in_shadow)
        }
        color
    }
    pub fn color_at(&self, ray: &Ray) -> Color {
        let xs = self.intersect_with(&ray);
        if let Some(x) = xs.hit() {
           let state = self.compute_state(&ray, &x);
           self.shade_hit(state)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    }
    pub fn is_shadowed(&self, point: Vec3, light_index: usize) -> bool {
        let point_to_light = self.lights[light_index].position - point;
        let distance = point_to_light.magnitude();
        let direction = point_to_light.normalize();

        let ray = Ray::new(point, direction);
        let xs = self.intersect_with(&ray);

        if let Some(hit) = xs.hit() {
            if hit.t < distance {
                return true;
            }
        }
        return false;
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
    #[test]
    fn precompute_intersection_state() {
        let ray = Ray::new(Vec3::point(0, 0, -5), Vec3::vector(0, 0, 1));
        let sphere = Sphere::new();
        let id = sphere.get_id();
        let mut world = World::new();
        world.objects.insert(id, Box::new(sphere));
        let intersection = Intersection::new(4.0, id);
        let state = world.compute_state(&ray, &intersection);
        assert_eq!(state.t, intersection.t);
        assert_eq!(state.obj_id, intersection.obj_id);
        assert_eq!(state.point, Vec3::point(0, 0, -1));
        assert_eq!(state.eyev, Vec3::vector(0, 0, -1));
        assert_eq!(state.normalv, Vec3::vector(0, 0, -1));
        assert_eq!(state.inside, false);
    }
    #[test]
    fn compute_state_of_hit_inside_object() {
        let ray = Ray::new(Vec3::point(0, 0, 0), Vec3::vector(0, 0, 1));
        let sphere = Sphere::new();
        let id = sphere.get_id();
        let mut world = World::new();
        world.objects.insert(id, Box::new(sphere));
        let intersection = Intersection::new(1.0, id);
        let state = world.compute_state(&ray, &intersection);
        assert_eq!(state.t, intersection.t);
        assert_eq!(state.obj_id, intersection.obj_id);
        assert_eq!(state.point, Vec3::point(0, 0, 1));
        assert_eq!(state.eyev, Vec3::vector(0, 0, -1));
        assert_eq!(state.normalv, Vec3::vector(0, 0, -1));
        assert_eq!(state.inside, true);
    }
    #[test]
    fn shading_intersection() {
        let ray = Ray::new(Vec3::point(0, 0, -5), Vec3::vector(0, 0, 1));
        let world = World::default();
        let id = world.objects.keys().min().unwrap();
        let intersection = Intersection::new(4.0, *id);
        let state = world.compute_state(&ray, &intersection);
        let color = world.shade_hit(state);
        assert!(color.equals(Color::new(0.38066, 0.47582, 0.28549)));
    }
    #[test]
    fn shading_intersection_inside() {
        let ray = Ray::new(Vec3::point(0, 0, 0), Vec3::vector(0, 0, 1));
        let mut world = World::default();
        world.lights[0] = Light::new(Vec3::point(0.0, 0.25, 0.0), Color::new(1.0, 1.0, 1.0));
        let id = world.objects.keys().max().unwrap();
        let intersection = Intersection::new(0.5, *id);
        let state = world.compute_state(&ray, &intersection);
        let color = world.shade_hit(state);
        assert!(color.equals(Color::new(0.90498, 0.90498, 0.90498)));
    }
    #[test]
    fn ray_misses() {
        let world = World::default();
        let ray = Ray::new(Vec3::point(0, 0, -5), Vec3::vector(0, 1, 0));
        assert!(world.color_at(&ray).equals(Color::new(0.0, 0.0, 0.0)));
    }
    #[test]
    fn ray_hits() {
        let world = World::default();
        let ray = Ray::new(Vec3::point(0, 0, -5), Vec3::vector(0, 0, 1));
        assert!(world.color_at(&ray).equals(Color::new(0.38066, 0.47583, 0.2855)));
    }
    #[test]
    fn intersection_behind_ray() {
        let mut world = World::new();
        world.lights.push(Light::new(Vec3::point(-10, -10, -10), Color::new(1.0, 1.0, 1.0)));
        let mut s1 = Sphere::new();
        let id1 = s1.get_id();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.7;
        s1.material.ambient = 1.0;
        let mut s2 = Sphere::new();
        let id2 = s2.get_id();
        s2.set_transform(Matrix::scaling(0.5, 0.5, 0.5));
        s2.material.ambient = 1.0;
        world.objects.insert(id1, Box::new(s1));
        world.objects.insert(id2, Box::new(s2));
        let ray = Ray::new(Vec3::point(0.0, 0.0, 0.75), Vec3::vector(0, 0, -1));
        assert!(world.color_at(&ray).equals(Color::new(1.0, 1.0, 1.0)));
    }
    #[test]
    fn not_collinear_no_shadow() {
        let world = World::default();
        let p = Vec3::point(0, 10, 0);
        assert_eq!(world.is_shadowed(p, 0), false);
    }
    #[test]
    fn obj_between_point_and_light() {
        let world = World::default();
        let p = Vec3::point(10, 10, 10);
        assert_eq!(world.is_shadowed(p, 0), true);
    }
    #[test]
    fn obj_behind_light() {
        let world = World::default();
        let p = Vec3::point(-20, 20, -20);
        assert_eq!(world.is_shadowed(p, 0), false);
    }
    #[test]
    fn obj_behind_point() {
        let world = World::default();
        let p = Vec3::point(-2, 2, -2);
        assert_eq!(world.is_shadowed(p, 0), false);
    }
    #[test]
    fn shade_hit_intersection() {
        let mut world = World::new();
        world.lights.push(Light::new(Vec3::point(0, 0, -10), Color::new(1.0, 1.0, 1.0)));
        let s1 = Sphere::new();
        let mut s2 = Sphere::new();
        s2.transform = Matrix::translation(0.0, 0.0, 10.0);
        let s2_id = s2.get_id();
        world.objects.insert(s1.get_id(), Box::new(s1));
        world.objects.insert(s2.get_id(), Box::new(s2));
        let ray = Ray::new(Vec3::point(0, 0, 5), Vec3::vector(0, 0, 1));
        let xs = Intersection::new(4.0, s2_id);
        let comps = world.compute_state(&ray, &xs);
        let color = world.shade_hit(comps);
        assert_eq!(color, Color::new(0.1, 0.1, 0.1));
    }
    #[test]
    fn hit_should_offset_point() {
        let mut world = World::new();
        let ray = Ray::new(Vec3::point(0, 0, -5), Vec3::vector(0, 0, 1));
        let mut shape = Sphere::new();
        shape.transform = Matrix::translation(0.0, 0.0, 1.0);
        let shape_id = shape.get_id();
        world.objects.insert(shape_id, Box::new(shape));
        let xs = Intersection::new(5.0, shape_id);
        let comps = world.compute_state(&ray, &xs);
        assert!(comps.over_point.z < -EPSILON/2.0);
        assert!(comps.point.z > comps.over_point.z);
    }
}
