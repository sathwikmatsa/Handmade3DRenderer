use std::sync::atomic;
use std::cmp::Ordering;
use super::ray::Ray;
use super::vec3::Vec3;
use super::float_cmp;
use std::ops::{Index};

static mut ID : atomic::AtomicUsize = atomic::AtomicUsize::new(0);

pub trait Object {
    fn get_uid() -> usize { unsafe {ID.fetch_add(1, atomic::Ordering::SeqCst)} }
    fn intersection<'a>(&'a self, ray: &Ray) -> Intersections<'a, Self> where Self : Sized;
    fn normal_at(&self, point: Vec3) -> Vec3;
}

#[derive(Debug, Clone)]
pub struct Intersection<'a, T: Object>{
    pub t: f32,
    pub obj: &'a T,
}

impl<'a, T: Object> Intersection<'a, T> {
    pub fn new(t: f32, obj: &'a T) -> Intersection<'a, T> {
        Self {t, obj}
    }
}

impl <'a, T:Object> Eq for Intersection<'a, T> {}

impl<'a, T: Object> Ord for Intersection<'a, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        if float_cmp::equal(self.t, other.t) {return Ordering::Equal;}
        else if float_cmp::greater(self.t, other.t) {return Ordering::Greater;}
        else {return Ordering::Less;}
    }
}

impl<'a, T: Object> PartialOrd for Intersection<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a, T: Object> PartialEq for Intersection<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        float_cmp::equal(self.t, other.t)
    }
}

#[derive(Debug)]
pub struct Intersections<'a, T: Object> {
    pub crossings: Vec<Intersection<'a, T>>,
}

impl<'a, T: Object> Intersections<'a, T> {
    pub fn new() -> Self {
        Self {
            crossings: Vec::new(),
        }
    }
    pub fn push(&mut self, crossing: Intersection<'a, T>) {
        let pos = self.crossings.binary_search(&crossing).unwrap_or_else(|e| e);
        self.crossings.insert(pos, crossing);
    }
    pub fn len(&self) -> usize {
        self.crossings.len()
    }
    pub fn hit(&self) -> &Intersection<'a, T> {
        // intersection with lowest nonnegative t value
        let ray_origin = Intersection::new(0.0, self.crossings[0].obj);
        &self.crossings[self.crossings.binary_search(&ray_origin).unwrap_or_else(|e| e)]
    }
}

impl<'a, T: Object> Index<usize> for Intersections<'a, T> {
    type Output = Intersection<'a, T>;

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
        let intersection = Intersection::new(1.0, &sphere);
        assert_eq!(intersection.t, 1.0);
        assert_eq!(intersection.obj, &sphere);
    }
}
