#![allow(non_snake_case)]

extern crate Handmade3DRenderer;
use Handmade3DRenderer::*;

struct Projectile {
    position: Vec3,
    velocity: Vec3,
}

struct Environment {
    gravity: Vec3,
    wind: Vec3,
}

fn tick(e: &Environment, p: Projectile) -> Projectile{
    Projectile {
        position: p.position + p.velocity,
        velocity: p.velocity + e.gravity + e.wind,
    }
}

fn main() {
    let velocity_scaling_factor = 11.25;
    let mut p = Projectile{position: Vec3::point(0, 1, 0), velocity: Vec3::vector(1.0, 1.8, 0.0).normalize() * velocity_scaling_factor};
    let e = Environment{gravity: Vec3::vector(0.0, -0.1, 0.0), wind: Vec3::vector(-0.01, 0.0, 0.0)};

    let mut canvas = Canvas::new(900, 550);
    let color = Color::new(1.0, 1.0, 1.0);

    while p.position.y > 0.0 {
        canvas.write_pixel(p.position.x as u32, canvas.height - p.position.y as u32, color);
        p = tick(&e, p);
    }
    canvas.canvas_to_ppm("projectile.ppm".to_string());
}
