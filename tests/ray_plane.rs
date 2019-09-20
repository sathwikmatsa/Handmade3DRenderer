#![allow(non_snake_case)]

extern crate Handmade3DRenderer;
use Handmade3DRenderer::*;

#[test]
fn ray_parallel_to_plane() {
    let p = Box::new(Plane::new());
    let r = Ray::new(Vec3::point(0, 10, 0), Vec3::vector(0, 0, 1));
    let xs = r.intersect(&p);
    assert_eq!(xs.len(), 0);
}

#[test]
fn intersect_with_coplanar_ray() {
    let p = Box::new(Plane::new());
    let r = Ray::new(Vec3::point(0, 0, 0), Vec3::vector(0, 0, 1));
    let xs = r.intersect(&p);
    assert_eq!(xs.len(), 0);
}

#[test]
fn intersect_plane_from_above() {
    let p = Plane::new();
    let id = p.get_id();
    let p = Box::new(p);
    let r = Ray::new(Vec3::point(0, 1, 0), Vec3::vector(0, -1, 0));
    let xs = r.intersect(&p);
    assert_eq!(xs.len(), 1);
    assert!(float_cmp::equal(xs[0].t, 1.0));
    assert_eq!(xs[0].obj_id, id);
}

#[test]
fn intersect_plane_from_below() {
    let p = Plane::new();
    let id = p.get_id();
    let p = Box::new(p);
    let r = Ray::new(Vec3::point(0, -1, 0), Vec3::vector(0, 1, 0));
    let xs = r.intersect(&p);
    assert_eq!(xs.len(), 1);
    assert!(float_cmp::equal(xs[0].t, 1.0));
    assert_eq!(xs[0].obj_id, id);
}
