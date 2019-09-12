#![allow(non_snake_case)]

extern crate Handmade3DRenderer;
use Handmade3DRenderer::*;

fn main() {
    let canvas_size = 400;
    let mut canvas = Canvas::new(canvas_size, canvas_size);
    let color = Color::new(1.0, 0.0, 0.0);
    let ray_origin = Vec3::point(0, 0, -5);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let pixel_size = wall_size / canvas_size as f32;
    let half = wall_size / 2.0;
    let shape = Sphere::new();
    let shape = Box::new(shape);
    for row in 0..canvas_size {
        let world_y = half - pixel_size * row as f32;
        for col in 0..canvas_size {
            let world_x = -1.0 * half + pixel_size * col as f32;
            let point_on_wall = Vec3::point(world_x, world_y, wall_z);
            let ray = Ray::new(ray_origin, (point_on_wall - ray_origin).normalize());
            let xs : Intersections = ray.intersect(&shape);
            if xs.len() != 0 {
                canvas.write_pixel(row, col, color);
            }
        }
    }
    canvas.canvas_to_ppm("silhoutte_of_sphere.ppm".to_string());
}
