#![allow(non_snake_case)]

extern crate Handmade3DRenderer;
use Handmade3DRenderer::Matrix;
use Handmade3DRenderer::Vec3;

#[test]
fn translation_of_point() {
    let transform = Matrix::translation(5.0, -3.0, 2.0);
    let point = Vec3::point(-3, 4, 5);
    assert_eq!(transform * point, Vec3::point(2, 1, 7));
}
#[test]
fn inverse_translation() {
    let transform = Matrix::translation(5.0, -3.0, 2.0);
    let inv_transform = transform.inverse_matrix();
    let point = Vec3::point(-3, 4, 5);
    assert_eq!(inv_transform * point, Vec3::point(-8, 7, 3));
}
#[test]
fn translating_vectors() {
    let transform = Matrix::translation(5.0, -3.0, 2.0);
    let vector = Vec3::vector(-3, 4, 5);
    assert_eq!(transform * vector, vector);
}
#[test]
fn scaling() {
    let transform = Matrix::scaling(2.0, 3.0, 4.0);
    let inv_transform = transform.inverse_matrix();
    let point = Vec3::point(-4, 6, 8);
    let vector = Vec3::vector(-4, 6, 8);
    assert_eq!(&transform * point, Vec3::point(-8, 18, 32));
    assert_eq!(&transform * vector, Vec3::vector(-8, 18, 32));
    assert_eq!(inv_transform * point, Vec3::point(-2, 2, 2));
}
#[test]
fn rotation_around_x() {
    let full_quarter = Matrix::rotation_x(std::f32::consts::PI / 2.0);
    let point = Vec3::point(0, 1, 0);
    assert_eq!(full_quarter * point, Vec3::point(0, 0, 1));
}
#[test]
fn rotation_around_y() {
    let full_quarter = Matrix::rotation_y(std::f32::consts::PI / 2.0);
    let point = Vec3::point(0, 0, 1);
    assert_eq!(full_quarter * point, Vec3::point(1, 0, 0));
}
#[test]
fn rotation_around_z() {
    let full_quarter = Matrix::rotation_z(std::f32::consts::PI / 2.0);
    let point = Vec3::point(0, 1, 0);
    assert_eq!(full_quarter * point, Vec3::point(-1, 0, 0));
}
#[test]
fn shearing() {
    let point = Vec3::point(2, 3, 4);
    let xyt = Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let xzt = Matrix::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
    let yxt = Matrix::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
    let yzt = Matrix::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    let zxt = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
    let zyt = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    assert_eq!(xyt * point, Vec3::point(5, 3, 4));
    assert_eq!(xzt * point, Vec3::point(6, 3, 4));
    assert_eq!(yxt * point, Vec3::point(2, 5, 4));
    assert_eq!(yzt * point, Vec3::point(2, 7, 4));
    assert_eq!(zxt * point, Vec3::point(2, 3, 6));
    assert_eq!(zyt * point, Vec3::point(2, 3, 7));
}
#[test]
fn chaining_transformations() {
    let p = Vec3::point(1, 0, 1);
    let A = Matrix::rotation_x(std::f32::consts::PI / 2.0);
    let B = Matrix::scaling(5.0, 5.0, 5.0);
    let C = Matrix::translation(10.0, 5.0, 7.0);
    let p2 = &A * p;
    let p3 = &B * p2;
    let p4 = &C * p3;
    assert_eq!(p4, Vec3::point(15, 0, 7));
    // chained transformation
    assert_eq!(p4, &C * &B * &A * p);
    // fluent API
    assert_eq!(p4, Matrix::identity_matrix(4)
                           .rotate_x(std::f32::consts::PI / 2.0)
                           .scale(5.0, 5.0, 5.0)
                           .translate(10.0, 5.0, 7.0) * p)
}
