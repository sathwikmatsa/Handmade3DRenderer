#![allow(non_snake_case)]

extern crate Handmade3DRenderer;
use Handmade3DRenderer::*;
use std::thread;
use std::cell::UnsafeCell;
use std::sync::{Arc};
use std::env;

struct LockFree<T> {
    data: UnsafeCell<T>,
}

unsafe impl<T> Sync for LockFree<T> {}

fn main() {
    let args: Vec<String> = env::args().collect();
    let N_THREADS = if args.len() > 1 {*(&args[1].parse::<u32>().unwrap_or(4))} else {4};
    const CANVAS_SIZE : u32 = 400;
    let shared_canvas = Arc::new(LockFree{ data: UnsafeCell::new(Canvas::new(CANVAS_SIZE, CANVAS_SIZE))});
    let eye = Arc::new(Vec3::point(0, 0, -5));
    const WALL_Z : f32 = 10.0;
    const WALL_SIZE : f32 = 7.0;
    const PIXEL_SIZE : f32 = WALL_SIZE / CANVAS_SIZE as f32;
    const HALF : f32 = WALL_SIZE / 2.0;
    let mut shape = Sphere::new();
    shape.material.color = Color::new(1.0, 0.2, 1.0);
    let sphere = Arc::new(Box::new(shape));
    let light = Arc::new(Light::new(Vec3::point(-10, 10, -10), Color::new(1.0, 1.0, 1.0)));
    let mut handles = Vec::new();
    for i in 0..N_THREADS {
        let canvas = shared_canvas.clone();
        let sphere = sphere.clone();
        let eye = *eye.clone();
        let light = *light.clone();
        let n = N_THREADS;
        handles.push(thread::spawn(move || {
            for row in i*CANVAS_SIZE/n..(i+1)*CANVAS_SIZE/n {
                let world_y = HALF - PIXEL_SIZE * row as f32;
                for col in 0..CANVAS_SIZE {
                    let world_x = -1.0 * HALF + PIXEL_SIZE * col as f32;
                    let poc = Vec3::point(world_x, world_y, WALL_Z);
                    let ray = Ray::new(eye, (poc - eye).normalize());
                    let xs : Intersections = ray.intersect(&(*sphere));
                    if let Some(x) = xs.hit() {
                        let point = ray.position(x.t);
                        let eye_v = -ray.direction;
                        let normal_v = sphere.normal_at(point);
                        let color = sphere.lighting_at(point, eye_v, normal_v, light, false);
                        unsafe {(*canvas.data.get()).set_pixel(row, col, color);}
                    }
                }
            }}));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    unsafe{(*shared_canvas.data.get()).save_as_ppm("3DSphereParallel.ppm");}
}
