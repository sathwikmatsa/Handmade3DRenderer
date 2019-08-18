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
    let velocity_scaling_factor = 2;
    let mut p = Projectile{position: Vec3::point(0, 1, 0), velocity: Vec3::vector(1, 1, 0).normalize() * velocity_scaling_factor};
    let e = Environment{gravity: Vec3::vector(0.0, -0.1, 0.0), wind: Vec3::vector(-0.01, 0.0, 0.0)};

    while p.position.y > 0.0 {
        p = tick(&e, p);
        //println!("projectile pos: {:?}", p.position);
    }

    println!("Virtual projectile reached ~{} units", p.position.x);

}
