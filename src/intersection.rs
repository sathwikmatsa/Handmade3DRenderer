use std::cmp::Ordering;
use super::float_cmp;
use super::vec3::Vec3;
use std::ops::{Index};

#[derive(Debug)]
pub struct IntersectionState {
    pub t: f32,
    pub obj_id: usize,
    pub point: Vec3,
    pub over_point: Vec3,
    pub eyev: Vec3,
    pub normalv: Vec3,
    pub inside: bool,
}

#[derive(Debug, Copy, Clone)]
pub struct Intersection{
    pub t: f32,
    pub obj_id: usize,
}

impl Intersection {
    pub fn new(t: f32, obj_id: usize) -> Intersection {
        Self {t, obj_id}
    }
}

impl Eq for Intersection {}

impl Ord for Intersection {
    fn cmp(&self, other: &Self) -> Ordering {
        if float_cmp::equal(self.t, other.t) {return Ordering::Equal;}
        else if float_cmp::greater(self.t, other.t) {return Ordering::Greater;}
        else {return Ordering::Less;}
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

#[derive(Debug)]
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
        let pos = self.crossings.binary_search(&crossing).unwrap_or_else(|e| e);
        self.crossings.insert(pos, crossing);
    }
    pub fn len(&self) -> usize {
        self.crossings.len()
    }
    pub fn hit(&self) -> Option<Intersection> {
        // intersection with lowest nonnegative t value
        if self.len() == 0 {return None;}
        let ray_origin = Intersection::new(0.0, self.crossings[0].obj_id);
        let r = self.crossings.binary_search(&ray_origin);
        match r {
            Ok(i) => Some(self.crossings[i]),
            Err(i) => if i < self.len() {Some(self.crossings[i])} else {None},
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
    use super::*;
    use super::super::sphere::Sphere;

    #[test]
    fn create_intersection_object() {
        let sphere = Sphere::new();
        let intersection = Intersection::new(1.0, sphere.get_id());
        assert_eq!(intersection.t, 1.0);
        assert_eq!(intersection.obj_id, sphere.get_id());
    }
}

