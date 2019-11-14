#![allow(non_snake_case)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]

pub mod vec3;
pub use vec3::*;
pub mod color;
pub use color::*;
pub mod canvas;
pub use canvas::*;
pub mod matrix;
pub use matrix::*;
pub mod world;
pub use world::*;
pub mod ray;
pub use ray::*;
pub mod light;
pub use light::*;
pub mod camera;
pub use camera::*;
pub mod material;
pub use material::*;
pub mod object;
pub use object::*;
pub mod intersection;
pub use intersection::*;
pub mod pattern;
pub use pattern::*;
pub mod sphere;
pub use sphere::*;
pub mod plane;
pub use plane::*;
pub mod float_cmp;
pub use float_cmp::*;
