use std::sync::atomic::{AtomicUsize, Ordering};
use super::ray::Ray;
use std::ops::{Index};

static mut ID : AtomicUsize = AtomicUsize::new(0);

pub trait Object {
    fn intersection<'a>(&'a self, ray: &Ray) -> Intersections<'a, Self> where Self : Sized;
    fn get_uid() -> usize {
        unsafe {
            ID.fetch_add(1, Ordering::SeqCst)
        }
    }
}

pub struct Intersection<'a, T: Object>{
    pub t: f32,
    pub obj: &'a T,
}

impl<'a, T: Object> Intersection<'a, T> {
    pub fn new(t: f32, obj: &'a T) -> Intersection<'a, T> {
        Self {t, obj}
    }
}

pub struct Intersections<'a, T: Object> {
    pub crossings: Vec<Intersection<'a, T>>,
}

impl<'a, T: Object> Intersections<'a, T> {
    pub fn new() -> Self {
        Self {
            crossings: Vec::new(),
        }
    }
    pub fn push(&mut self, hit: Intersection<'a, T>) {
        self.crossings.push(hit);
    }
    pub fn len(&self) -> usize {
        self.crossings.len()
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
    use super::super::vec3::Vec3;

    #[test]
    fn create_intersection_object() {
        let sphere = Sphere::new(Vec3::point(0.0, 0.0, 0.0), 1.0);
        let intersection = Intersection::new(1.0, &sphere);
        assert_eq!(intersection.t, 1.0);
        assert_eq!(intersection.obj, &sphere);
    }
}
