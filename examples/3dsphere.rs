#![allow(non_snake_case)]

use Handmade3DRenderer::*;

fn main() {
    let mut world = World::new();
    world.lights.push(Light::new(
        Vec3::point(-10, 10, -10),
        Color::new(1.0, 1.0, 1.0),
    ));
    let mut shape = Sphere::new();
    shape.material.color = Color::new(1.0, 0.2, 1.0);
    world.objects.insert(shape.get_id(), Box::new(shape));
    let canvas_size = 200;
    let mut camera = Camera::new(canvas_size, canvas_size, std::f32::consts::PI / 6.0);
    camera.transform = Matrix::view_transformation(
        Vec3::point(0, 0, -5),
        Vec3::point(0, 0, 0),
        Vec3::vector(0, 1, 0),
    );
    let canvas = camera.render(&world);
    canvas.save_as_ppm("3d_sphere.ppm");
}
