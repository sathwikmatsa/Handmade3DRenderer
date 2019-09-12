#![allow(non_snake_case)]

extern crate Handmade3DRenderer;
use Handmade3DRenderer::*;

#[test]
fn intersection_encapsulation() {
    let id = 3;
    let intersection = Intersection::new(3.5, id);
    assert!(float_cmp::equal(intersection.t, 3.5));
    assert_eq!(intersection.obj_id, id);
}
#[test]
fn aggregating_intersections() {
    let i1 = Intersection::new(1.0, 1);
    let i2 = Intersection::new(2.0, 1);
    let mut intersections = Intersections::new();
    intersections.push(i1);
    intersections.push(i2);
    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].t, 1.0);
    assert_eq!(intersections[1].t, 2.0);
}
#[test]
fn ray_intersections() {
    let ray = Ray::new(Vec3::point(0, 0, -5), Vec3::vector(0, 0, 1));
    let sphere = Box::new(Sphere::new());
    let xs = ray.intersect(&sphere);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].obj_id, xs[1].obj_id);
    assert_eq!(xs[0].obj_id, (*sphere).get_id());
}
#[test]
fn hit_intersection() {
    let id = 1;

    // when all intersections have +ve t
    let i1 = Intersection::new(1.0, id);
    let i1_c = i1.clone();
    let i2 = Intersection::new(2.0, id);
    let mut intersections = Intersections::new();
    intersections.push(i1);
    intersections.push(i2);
    assert_eq!(intersections.hit(), Some(i1_c));

    // when some intersections have -ve t
    let i1 = Intersection::new(-1.0, id);
    let i2 = Intersection::new(1.0, id);
    let i2_c = i2.clone();
    let mut intersections = Intersections::new();
    intersections.push(i1);
    intersections.push(i2);
    assert_eq!(intersections.hit(), Some(i2_c));

    // when all intersections have -ve t
    let i1 = Intersection::new(-1.0, id);
    let i2 = Intersection::new(-2.0, id);
    let mut intersections = Intersections::new();
    intersections.push(i1);
    intersections.push(i2);
    assert_eq!(intersections.hit(), None);
}
