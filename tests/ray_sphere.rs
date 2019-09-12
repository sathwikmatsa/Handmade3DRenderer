#![allow(non_snake_case)]

extern crate Handmade3DRenderer;
use Handmade3DRenderer::*;

#[test]
fn ray_sphere_intersection_two() {
    let ray = Ray::new(Vec3::point(0, 0, -5), Vec3::vector(0, 0, 1));
    let sphere = Box::new(Sphere::new());
    let xs : Intersections = ray.intersect(&sphere);
    assert_eq!(xs.len(), 2);
    assert!(float_cmp::equal(xs[0].t, 4.0));
    assert!(float_cmp::equal(xs[1].t, 6.0));
}

#[test]
fn ray_sphere_tangent_intersection() {
    let ray = Ray::new(Vec3::point(0, 1, -5), Vec3::vector(0, 0, 1));
    let sphere = Box::new(Sphere::new());
    let xs : Intersections = ray.intersect(&sphere);
    assert_eq!(xs.len(), 2);
    assert!(float_cmp::equal(xs[0].t, 5.0));
    assert!(float_cmp::equal(xs[1].t, 5.0));
}

#[test]
fn ray_sphere_no_intersection() {
    let ray = Ray::new(Vec3::point(0, 2, -5), Vec3::vector(0, 0, 1));
    let sphere = Box::new(Sphere::new());
    let xs : Intersections = ray.intersect(&sphere);
    assert_eq!(xs.len(), 0);
}
#[test]
fn ray_originates_inside_sphere() {
    let ray = Ray::new(Vec3::point(0, 0, 0), Vec3::vector(0, 0, 1));
    let sphere = Box::new(Sphere::new());
    let xs : Intersections = ray.intersect(&sphere);
    assert_eq!(xs.len(), 2);
    assert!(float_cmp::equal(xs[0].t, -1.0));
    assert!(float_cmp::equal(xs[1].t, 1.0));
}
#[test]
fn sphere_behind_ray() {
    let ray = Ray::new(Vec3::point(0, 0, 5), Vec3::vector(0, 0, 1));
    let sphere = Box::new(Sphere::new());
    let xs : Intersections = ray.intersect(&sphere);
    assert_eq!(xs.len(), 2);
    assert!(float_cmp::equal(xs[0].t, -6.0));
    assert!(float_cmp::equal(xs[1].t, -4.0));
}
#[test]
fn intersect_scaled_sphere() {
    let ray = Ray::new(Vec3::point(0, 0, -5), Vec3::vector(0, 0, 1));
    let mut sphere = Sphere::new();
    sphere.set_transform(Matrix::scaling(2.0, 2.0, 2.0));
    let sphere = Box::new(sphere);
    let xs : Intersections = ray.intersect(&sphere);
    assert_eq!(xs.len(), 2);
    assert!(float_cmp::equal_debug(xs[0].t, 3.0));
    assert!(float_cmp::equal_debug(xs[1].t, 7.0));
}
#[test]
fn intersect_translated_sphere() {
    let ray = Ray::new(Vec3::point(0, 0, -5), Vec3::vector(0, 0, 1));
    let mut sphere = Sphere::new();
    sphere.set_transform(Matrix::translation(5.0, 0.0, 0.0));
    let sphere = Box::new(sphere);
    let xs : Intersections = ray.intersect(&sphere);
    assert_eq!(xs.len(), 0);
}
