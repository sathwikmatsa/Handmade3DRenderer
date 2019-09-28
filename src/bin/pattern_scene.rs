#![allow(non_snake_case)]

extern crate Handmade3DRenderer;
use Handmade3DRenderer::*;

fn main() {
    let mut world = World::new();
    world.lights.push(Light::new(Vec3::point(-10, 10, -10), Color::new(1.0, 1.0, 1.0)));

    let mut middle_sphere = Sphere::new();
    middle_sphere.transform = Matrix::translation(-0.5, 1.0, 0.5);
    middle_sphere.material.color = Color::new(0.1, 1.0, 0.5);
    middle_sphere.material.diffuse = 0.7;
    middle_sphere.material.specular = 0.3;
    let mut ring_pattern = Pattern::ring(vec![WHITE, BLUE, RED, WHITE, RED]);
    ring_pattern.transform = Matrix::scaling(0.2, 0.2, 0.2).rotate_x(std::f32::consts::PI/2.0);
    middle_sphere.material.pattern = Some(ring_pattern);

    let mut left_sphere = Sphere::new();
    left_sphere.transform = Matrix::translation(-1.5, 0.33, -0.75)*
                                &Matrix::scaling(0.33, 0.33, 0.33);
    left_sphere.material.color = Color::new(1.0, 0.8, 0.1);
    left_sphere.material.diffuse = 0.7;
    left_sphere.material.specular = 0.3;
    let mut stripe_pattern = Pattern::stripe(vec![YELLOW, GREEN]);
    stripe_pattern.transform = Matrix::scaling(0.2, 0.2, 0.2);
    left_sphere.material.pattern = Some(stripe_pattern);

    let mut right_sphere = Sphere::new();
    right_sphere.transform = Matrix::translation(1.5, 0.5, -0.5)*
                                &Matrix::scaling(0.5, 0.5, 0.5);
    right_sphere.material.color = Color::new(0.5, 1.0, 0.1);
    right_sphere.material.diffuse = 0.7;
    right_sphere.material.specular = 0.3;
    right_sphere.material.pattern = Some(Pattern::checkers(vec![WHITE, BLACK]));

    let mut plane = Plane::new();
    plane.material.pattern = Some(Pattern::gradient(vec![ORANGE, BLUE]));

    world.objects.insert(plane.get_id(), Box::new(plane));
    world.objects.insert(left_sphere.get_id(), Box::new(left_sphere));
    world.objects.insert(middle_sphere.get_id(), Box::new(middle_sphere));
    world.objects.insert(right_sphere.get_id(), Box::new(right_sphere));

    let mut camera = Camera::new(100*5, 50*5, std::f32::consts::PI/3.0);
    camera.transform = Matrix::view_transformation(
                        Vec3::point(0.0, 1.5, -5.0),
                        Vec3::point(0, 1, 0),
                        Vec3::vector(0, 1, 0),
    );
    let canvas = camera.render(&world);
    canvas.save_as_ppm("pattern_scene.ppm");
}
