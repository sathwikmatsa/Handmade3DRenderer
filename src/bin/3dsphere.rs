#![allow(non_snake_case)]

extern crate Handmade3DRenderer;
use Handmade3DRenderer::*;

fn main() {
    let canvas_size = 400;
    let mut canvas = Canvas::new(canvas_size, canvas_size);
    let eye = Vec3::point(0, 0, -5);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let pixel_size = wall_size / canvas_size as f32;
    let half = wall_size / 2.0;
    let mut shape = Sphere::new();
    shape.material.color = Color::new(1.0, 0.2, 1.0);
    let shape = Box::new(shape);
    let light = Light::new(Vec3::point(-10, 10, -10), Color::new(1.0, 1.0, 1.0));
    for row in 0..canvas_size {
        let world_y = half - pixel_size * row as f32;
        for col in 0..canvas_size {
            let world_x = -1.0 * half + pixel_size * col as f32;
            let poc = Vec3::point(world_x, world_y, wall_z);
            let ray = Ray::new(eye, (poc - eye).normalize());
            let xs : Intersections = ray.intersect(&shape);
            if let Some(x) = xs.hit() {
                let point = ray.position(x.t);
                let eye_v = -ray.direction;
                let normal_v = shape.normal_at(point);
                let color = shape.lighting_at(point, eye_v, normal_v, light);
                canvas.write_pixel(row, col, color);
            }
        }
    }
    canvas.save_as_ppm("3DSphere.ppm");
}
