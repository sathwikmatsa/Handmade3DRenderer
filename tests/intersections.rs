#![allow(non_snake_case)]

extern crate Handmade3DRenderer;
use Handmade3DRenderer::*;

#[test]
fn intersection_encapsulation() {
    let sphere = Sphere::new();
    let intersection = Intersection::new(3.5, &sphere);
    assert!(float_cmp::equal(intersection.t, 3.5));
    assert_eq!(intersection.obj.get_id(), sphere.get_id());
}
#[test]
fn aggregating_intersections() {
    let sphere = Sphere::new();
    let i1 = Intersection::new(1.0, &sphere);
    let i2 = Intersection::new(2.0, &sphere);
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
    let sphere = Sphere::new();
    let xs = ray.intersect(&sphere);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].obj.get_id(), xs[1].obj.get_id());
    assert_eq!(xs[0].obj.get_id(), sphere.get_id());
}
#[test]
fn hit_intersection() {
    let sphere = Sphere::new();

    // when all intersections have +ve t
    let i1 = Intersection::new(1.0, &sphere);
    let i1_c = i1.clone();
    let i2 = Intersection::new(2.0, &sphere);
    let mut intersections = Intersections::new();
    intersections.push(i1);
    intersections.push(i2);
    assert_eq!(intersections.hit(), &i1_c);

    // when some intersections have -ve t
    let i1 = Intersection::new(-1.0, &sphere);
    let i2 = Intersection::new(1.0, &sphere);
    let i2_c = i2.clone();
    let mut intersections = Intersections::new();
    intersections.push(i1);
    intersections.push(i2);
    assert_eq!(intersections.hit(), &i2_c);

}
