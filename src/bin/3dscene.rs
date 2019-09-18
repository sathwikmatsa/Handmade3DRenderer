#![allow(non_snake_case)]

extern crate Handmade3DRenderer;
use Handmade3DRenderer::*;

fn main() {
    let mut world = World::new();
    world.lights.push(Light::new(Vec3::point(-10, 10, -10), Color::new(1.0, 1.0, 1.0)));

    let mut floor = Sphere::new();
    floor.transform = Matrix::scaling(10.0, 0.01, 10.0);
    floor.material.color = Color::new(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;

    let mut left_wall = Sphere::new();
    left_wall.transform = Matrix::translation(0.0, 0.0, 5.0)*
                            &Matrix::rotation_y(-std::f32::consts::PI/4.0)*
                            &Matrix::rotation_x(std::f32::consts::PI/2.0)*
                            &Matrix::scaling(10.0, 0.01, 10.0);
    left_wall.material = floor.material;

    let mut right_wall = Sphere::new();
    right_wall.transform = Matrix::translation(0.0, 0.0, 5.0)*
                            &Matrix::rotation_y(std::f32::consts::PI/4.0)*
                            &Matrix::rotation_x(std::f32::consts::PI/2.0)*
                            &Matrix::scaling(10.0, 0.01, 10.0);
    right_wall.material = floor.material;

    let mut middle_sphere = Sphere::new();
    middle_sphere.transform = Matrix::translation(-0.5, 1.0, 0.5);
    middle_sphere.material.color = Color::new(0.1, 1.0, 0.5);
    middle_sphere.material.diffuse = 0.7;
    middle_sphere.material.specular = 0.3;

    let mut left_sphere = Sphere::new();
    left_sphere.transform = Matrix::translation(-1.5, 0.33, -0.75)*
                                &Matrix::scaling(0.33, 0.33, 0.33);
    left_sphere.material.color = Color::new(1.0, 0.8, 0.1);
    left_sphere.material.diffuse = 0.7;
    left_sphere.material.specular = 0.3;

    let mut right_sphere = Sphere::new();
    right_sphere.transform = Matrix::translation(1.5, 0.5, -0.5)*
                                &Matrix::scaling(0.5, 0.5, 0.5);
    right_sphere.material.color = Color::new(0.5, 1.0, 0.1);
    right_sphere.material.diffuse = 0.7;
    right_sphere.material.specular = 0.3;

    world.objects.insert(floor.get_id(), Box::new(floor));
    world.objects.insert(left_wall.get_id(), Box::new(left_wall));
    world.objects.insert(right_wall.get_id(), Box::new(right_wall));
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
    canvas.save_as_ppm("scene_rayon_par.ppm");
}
