#![allow(non_snake_case)]

extern crate Handmade3DRenderer;
use Handmade3DRenderer::*;

fn main() {
    let width = 500;
    let height = 500;
    let mut canvas = Canvas::new(width, height);
    let color = Color::new(1.0, 1.0, 1.0);
    let origin = Vec3::point(0, 0, 0);
    let radius = 0.4 * width as f32;
    let mut clock_position = Matrix::translation(radius, 0.0, 0.0) * origin;
    let separation_angle : f32 = 2.0 * std::f32::consts::PI / 12.0;
    for _ in 0..12 {
        // translate origin to center of canvas
        let canvas_point = Matrix::translation(width as f32 / 2.0, height as f32 / 2.0 , 0.0) * clock_position;
        canvas.write_pixel(canvas_point.x as u32, canvas_point.y as u32, color);
        clock_position = Matrix::rotation_z(separation_angle) * clock_position;
    }
    canvas.save_as_ppm("analog_clock.ppm");
}
