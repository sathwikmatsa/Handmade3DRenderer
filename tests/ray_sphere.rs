#![allow(non_snake_case)]

extern crate Handmade3DRenderer;
use Handmade3DRenderer::*;

#[test]
fn ray_sphere_intersection_two() {
    let ray = Ray::new(Vec3::point(0, 0, -5), Vec3::vector(0, 0, 1));
    let sphere = Sphere::new(Vec3::point(0, 0, 0), 1.0);
    let xs : Intersections<Sphere> = ray.intersect(&sphere);
    assert_eq!(xs.len(), 2);
    assert!(float_cmp::equal(xs[0].t, 4.0));
    assert!(float_cmp::equal(xs[1].t, 6.0));
}

#[test]
fn ray_sphere_tangent_intersection() {
    let ray = Ray::new(Vec3::point(0, 1, -5), Vec3::vector(0, 0, 1));
    let sphere = Sphere::new(Vec3::point(0, 0, 0), 1.0);
    let xs : Intersections<Sphere> = ray.intersect(&sphere);
    assert_eq!(xs.len(), 2);
    assert!(float_cmp::equal(xs[0].t, 5.0));
    assert!(float_cmp::equal(xs[1].t, 5.0));
}

#[test]
fn ray_sphere_no_intersection() {
    let ray = Ray::new(Vec3::point(0, 2, -5), Vec3::vector(0, 0, 1));
    let sphere = Sphere::new(Vec3::point(0, 0, 0), 1.0);
    let xs : Intersections<Sphere> = ray.intersect(&sphere);
    assert_eq!(xs.len(), 0);
}
#[test]
fn ray_originates_inside_sphere() {
    let ray = Ray::new(Vec3::point(0, 0, 0), Vec3::vector(0, 0, 1));
    let sphere = Sphere::new(Vec3::point(0, 0, 0), 1.0);
    let xs : Intersections<Sphere> = ray.intersect(&sphere);
    assert_eq!(xs.len(), 2);
    assert!(float_cmp::equal(xs[0].t, -1.0));
    assert!(float_cmp::equal(xs[1].t, 1.0));
}
#[test]
fn sphere_behind_ray() {
    let ray = Ray::new(Vec3::point(0, 0, 5), Vec3::vector(0, 0, 1));
    let sphere = Sphere::new(Vec3::point(0, 0, 0), 1.0);
    let xs : Intersections<Sphere> = ray.intersect(&sphere);
    assert_eq!(xs.len(), 2);
    assert!(float_cmp::equal(xs[0].t, -6.0));
    assert!(float_cmp::equal(xs[1].t, -4.0));
}
