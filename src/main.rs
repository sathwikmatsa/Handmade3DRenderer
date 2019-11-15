#![allow(non_snake_case)]

use Handmade3DRenderer::*;

fn main() {
    let mut world = World::new();
    world.lights.push(Light::new(
        Vec3::point(-10, 10, -10),
        Color::new(1.0, 1.0, 1.0),
    ));

    let mut sphere = Sphere::new();
    sphere.transform = Matrix::translation(-0.5, 1.0, 0.5);
    sphere.material.color = Color::new(1.0, 0.0, 0.0);
    sphere.material.diffuse = 0.7;
    sphere.material.specular = 0.3;

    let mut plane = Plane::new();
    plane.material.reflective = 0.7;
    plane.material.pattern = Some(Pattern::checkers(vec![
        Color::new(0.41, 0.41, 0.41),
        Color::new(0.82, 0.82, 0.82),
    ]));

    world.objects.insert(plane.get_id(), Box::new(plane));
    world.objects.insert(sphere.get_id(), Box::new(sphere));

    let mut camera = Camera::new(70 * 10, 50 * 10, std::f32::consts::PI / 3.0);
    camera.transform = Matrix::view_transformation(
        Vec3::point(0.0, 1.5, -5.0),
        Vec3::point(0, 1, 0),
        Vec3::vector(0, 1, 0),
    );
    let canvas = camera.render(&world);
    canvas.save_as_ppm("pattern_scene.ppm");
}
