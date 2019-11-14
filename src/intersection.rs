use super::float_cmp;
use super::float_cmp::EPSILON;
use super::ray::Ray;
use super::vec3::Vec3;
use super::world::World;
use std::cmp::Ordering;
use std::ops::Index;

#[derive(Debug)]
pub struct State {
    pub t: f32,
    pub obj_id: usize,
    pub point: Vec3,
    pub over_point: Vec3,
    pub eyev: Vec3,
    pub normalv: Vec3,
    pub reflectv: Vec3,
    pub inside: bool,
}

#[derive(Debug, Copy, Clone)]
pub struct Intersection {
    pub t: f32,
    pub obj_id: usize,
}

impl Intersection {
    pub fn new(t: f32, obj_id: usize) -> Self {
        Self { t, obj_id }
    }
    pub fn compute_state(&self, ray: &Ray, world: &World) -> State {
        let point = ray.position(self.t);
        let mut normalv = world.objects.get(&self.obj_id).unwrap().normal_at(point);
        let eyev = -ray.direction;
        let reflectv = ray.direction.reflect(normalv);
        // checking for ray originating from inside the object
        let inside;
        if normalv.dot(eyev) < 0.0 {
            inside = true;
            normalv = -normalv;
        } else {
            inside = false;
        }

        let point = ray.position(self.t);
        // required to prevent intersection to be treated as shadow
        let over_point = point + normalv * 15.0 * EPSILON;

        State {
            t: self.t,
            obj_id: self.obj_id,
            point,
            over_point,
            eyev,
            normalv,
            reflectv,
            inside,
        }
    }
}

impl Eq for Intersection {}

impl Ord for Intersection {
    fn cmp(&self, other: &Self) -> Ordering {
        if float_cmp::equal(self.t, other.t) {
            Ordering::Equal
        } else if float_cmp::greater(self.t, other.t) {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        float_cmp::equal(self.t, other.t)
    }
}

#[derive(Debug, Default)]
pub struct Intersections {
    pub crossings: Vec<Intersection>,
}

impl Intersections {
    pub fn new() -> Self {
        Self {
            crossings: Vec::new(),
        }
    }
    pub fn push(&mut self, crossing: Intersection) {
        let pos = self
            .crossings
            .binary_search(&crossing)
            .unwrap_or_else(|e| e);
        self.crossings.insert(pos, crossing);
    }
    pub fn len(&self) -> usize {
        self.crossings.len()
    }
    pub fn is_empty(&self) -> bool {
        self.crossings.len() == 0
    }
    pub fn hit(&self) -> Option<Intersection> {
        // intersection with lowest nonnegative t value
        if self.is_empty() {
            return None;
        }
        let ray_origin = Intersection::new(0.0, self.crossings[0].obj_id);
        let r = self.crossings.binary_search(&ray_origin);
        match r {
            Ok(i) => Some(self.crossings[i]),
            Err(i) => {
                if i < self.len() {
                    Some(self.crossings[i])
                } else {
                    None
                }
            }
        }
    }
}

impl Index<usize> for Intersections {
    type Output = Intersection;

    fn index(&self, index: usize) -> &Self::Output {
        &self.crossings[index]
    }
}

#[cfg(test)]
pub mod tests {
    use super::super::matrix::Matrix;
    use super::super::plane::Plane;
    use super::super::sphere::Sphere;
    use super::float_cmp::*;
    use super::*;

    #[test]
    fn create_intersection_object() {
        let sphere = Sphere::new();
        let intersection = Intersection::new(1.0, sphere.get_id());
        assert_eq!(intersection.t, 1.0);
        assert_eq!(intersection.obj_id, sphere.get_id());
    }
    #[test]
    fn precompute_intersection_state() {
        let ray = Ray::new(Vec3::point(0, 0, -5), Vec3::vector(0, 0, 1));
        let sphere = Sphere::new();
        let id = sphere.get_id();
        let mut world = World::new();
        world.objects.insert(id, Box::new(sphere));
        let intersection = Intersection::new(4.0, id);
        let state = intersection.compute_state(&ray, &world);
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
        let state = intersection.compute_state(&ray, &world);
        assert_eq!(state.t, intersection.t);
        assert_eq!(state.obj_id, intersection.obj_id);
        assert_eq!(state.point, Vec3::point(0, 0, 1));
        assert_eq!(state.eyev, Vec3::vector(0, 0, -1));
        assert_eq!(state.normalv, Vec3::vector(0, 0, -1));
        assert_eq!(state.inside, true);
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
        let comps = xs.compute_state(&ray, &world);
        assert!(comps.over_point.z < -EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }
    #[test]
    fn precompute_reflectv() {
        let mut world = World::new();
        let shape = Plane::new();
        let shape_id = shape.get_id();
        world.objects.insert(shape_id, Box::new(shape));
        let ray = Ray::new(
            Vec3::point(0, 1, -1),
            Vec3::vector(0.0, -INVSQRT2, INVSQRT2),
        );
        let xs = Intersection::new(SQRT2, shape_id);
        let comps = xs.compute_state(&ray, &world);
        assert_eq!(comps.reflectv, Vec3::vector(0.0, INVSQRT2, INVSQRT2));
    }
}
