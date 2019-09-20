use super::matrix::Matrix;
use super::ray::Ray;
use super::vec3::Vec3;
use super::canvas::Canvas;
use super::world::World;

extern crate rayon;
use rayon::prelude::*;

#[derive(Clone, Debug)]
pub struct Camera {
    pub hsize: u32,
    pub vsize: u32,
    pub field_of_view: f32,
    pub transform: Matrix,
    pub pixel_size: f32,
    pub half_width: f32,
    pub half_height: f32,
}

impl Camera {
    pub fn new(hsize: u32, vsize: u32, field_of_view: f32) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f32 / vsize as f32;

        let half_width;
        let half_height;
        if aspect >= 1.0 {
           half_width = half_view;
           half_height = half_view / aspect as f32;
        } else {
           half_width = half_view * aspect as f32;
           half_height = half_view;
        }

        let pixel_size = half_width * 2.0 / hsize as f32;
        Self { hsize, vsize, field_of_view, half_width, half_height,
                pixel_size, transform: Matrix::identity_matrix(4) }
    }
    pub fn ray_for_pixel(&self, px: u32, py: u32) -> Ray {
        let xoffset = (px as f32 + 0.5) * self.pixel_size;
        let yoffset = (py as f32 + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let pixel = self.transform.inverse_matrix() * Vec3::point(world_x, world_y, -1.0);
        let origin = self.transform.inverse_matrix() * Vec3::point(0, 0, 0);
        let direction = (pixel - origin).normalize();
        Ray::new(origin, direction)
    }
    pub fn render(&self, world: &World) -> Canvas {
        let mut canvas = Canvas::new(self.hsize, self.vsize);

        canvas.grid.par_iter_mut()
                   .enumerate()
                   .for_each(|(index, color)| {
                       let x = index as u32 / self.hsize;
                       let y = index as u32 % self.hsize;
                       let ray = self.ray_for_pixel(y, x);
                       *color = world.color_at(&ray);
                   });

        canvas
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use super::super::color::*;
    #[test]
    fn create_camera() {
        let camera = Camera::new(160, 120, std::f32::consts::PI / 2.0);
        assert_eq!(camera.hsize, 160);
        assert_eq!(camera.vsize, 120);
        assert_eq!(camera.field_of_view, std::f32::consts::PI / 2.0);
        assert_eq!(camera.transform, Matrix::identity_matrix(4));
    }
    #[test]
    fn pixel_size() {
        let c1 = Camera::new(200, 125, std::f32::consts::PI / 2.0);
        assert_eq!(c1.pixel_size, 0.01);
        let c2 = Camera::new(125, 200, std::f32::consts::PI / 2.0);
        assert_eq!(c2.pixel_size, 0.01);
    }
    #[test]
    fn compute_ray_for_pixel() {
        let mut camera = Camera::new(201, 101, std::f32::consts::PI / 2.0);
        let r1 = camera.ray_for_pixel(100, 50);
        let r2 = camera.ray_for_pixel(0, 0);
        camera.transform = Matrix::rotation_y(std::f32::consts::PI / 4.0) *
                            &Matrix::translation(0.0, -2.0, 5.0);
        let r3 = camera.ray_for_pixel(100, 50);
        assert_eq!(r1.origin, Vec3::point(0, 0, 0));
        assert_eq!(r1.direction, Vec3::vector(0, 0, -1));
        assert_eq!(r2.origin, Vec3::point(0, 0, 0));
        assert_eq!(r2.direction, Vec3::vector(0.66519, 0.33259, -0.66851));
        assert_eq!(r3.origin, Vec3::point(0, 2, -5));
        assert_eq!(r3.direction, Vec3::vector(0.70710665, 0.0, -0.7071069));
    }
    #[test]
    fn render_image() {
        let world = World::default();
        let mut camera = Camera::new(11, 11, std::f32::consts::PI / 2.0);
        camera.transform = Matrix::view_transformation(
                Vec3::point(0, 0, -5),
                Vec3::point(0, 0, 0),
                Vec3::vector(0, 1, 0)
        );
        let image = camera.render(&world);
        assert_eq!(image.pixel_at(5, 5), Color::new(0.38066125, 0.4758265, 0.28549594));
    }
}
