use std::sync::atomic;
use super::ray::Ray;
use super::light::Light;
use super::color::Color;
use super::vec3::Vec3;
use super::intersection::Intersections;

static mut ID : atomic::AtomicUsize = atomic::AtomicUsize::new(0);

pub trait Object {
    fn get_uid() -> usize { unsafe {ID.fetch_add(1, atomic::Ordering::SeqCst)} }
    fn intersection(&self, ray: &Ray) -> Intersections;
    fn normal_at(&self, point: Vec3) -> Vec3;
    fn lighting_at(&self, point: Vec3, eye_v: Vec3, light: Light) -> Color;
}

