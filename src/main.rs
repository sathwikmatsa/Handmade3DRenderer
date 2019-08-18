extern crate Handmade3DRenderer;
use Handmade3DRenderer::*;

fn main() {
    let p = Vec3::point(1, 2, 3);
    let v = Vec3::vector(4, 5, 6);
    println!("Point {:#?}", p);
    println!("Vector {:#?}", v);
}
