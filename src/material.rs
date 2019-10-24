use super::color::*;
use super::light::Light;
use super::matrix::Matrix;
use super::pattern::Pattern;
use super::vec3::Vec3;

#[derive(Debug, Clone)]
pub struct Material {
    pub color: Color,
    pub pattern: Option<Pattern>,
    // Phong Reflection Model
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
    pub reflective: f32,
}

impl Material {
    pub fn default() -> Self {
        Self {
            color: Color::new(1.0, 1.0, 1.0),
            pattern: None,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
        }
    }
    pub fn new(
        color: Color,
        ambient: f32,
        diffuse: f32,
        specular: f32,
        shininess: f32,
        reflective: f32,
        pattern: Option<Pattern>,
    ) -> Self {
        assert!(
            (0.0 <= ambient) && (ambient <= 1.0),
            "ambient value is out of bounds"
        );
        assert!(
            (0.0 <= diffuse) && (diffuse <= 1.0),
            "diffuse value is out of bounds"
        );
        assert!(
            (0.0 <= specular) && (specular <= 1.0),
            "specular value is out of bounds"
        );
        assert!(
            (10.0 <= shininess) && (shininess <= 200.0),
            "shininess value is out of bounds"
        );
        Self {
            color,
            pattern,
            ambient,
            diffuse,
            specular,
            shininess,
            reflective,
        }
    }
    pub fn lighting(
        &self,
        obj_transform: &Matrix,
        light: Light,
        point: Vec3,
        eye_v: Vec3,
        normal_v: Vec3,
        in_shadow: bool,
    ) -> Color {
        let material_color;
        if let Some(pattern) = &self.pattern {
            material_color = pattern.pattern_at(point, obj_transform);
        } else {
            material_color = self.color;
        }

        let effective_color = material_color * light.intensity;
        let ambient = effective_color * self.ambient;

        let light_v = (light.position - point).normalize();
        let diffuse;
        let specular;

        // light_dot_normal represents the cosine of the angle between the light vector and the normal vector.
        // A negative number means the light is on the other side of the surface.
        let light_dot_normal = light_v.dot(normal_v);

        if light_dot_normal < 0.0 || in_shadow {
            diffuse = Color::new(0.0, 0.0, 0.0);
            specular = Color::new(0.0, 0.0, 0.0);
        } else {
            diffuse = effective_color * self.diffuse * light_dot_normal;
            // reflect_dot_eye represents the cosine of the angle between the
            // reflection vector and the eye vector.
            // A negative number means the light reflects away from the eye.

            let reflect_v = -light_v.reflect(normal_v);
            let reflect_dot_eye = reflect_v.dot(eye_v);

            if reflect_dot_eye <= 0.0 {
                specular = Color::new(0.0, 0.0, 0.0);
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }

        ambient + diffuse + specular
    }
}

#[cfg(test)]
pub mod tests {
    use super::super::float_cmp;
    use super::*;
    #[test]
    fn create_material() {
        let m = Material::new(Color::new(1.0, 1.0, 1.0), 0.1, 0.9, 0.9, 200.0, 0.0, None);
        let d = Material::default();
        assert_eq!(m.color, d.color);
        assert!(float_cmp::equal(m.ambient, d.ambient));
        assert!(float_cmp::equal(m.specular, d.specular));
        assert!(float_cmp::equal(m.diffuse, d.diffuse));
        assert!(float_cmp::equal(m.shininess, d.shininess));
        assert!(float_cmp::equal(m.reflective, d.reflective));
    }
    #[test]
    fn lighting_light_eye_surface() {
        let m = Material::default();
        let position = Vec3::point(0, 0, 0);
        let eyev = Vec3::vector(0, 0, -1);
        let normalv = Vec3::vector(0, 0, -1);
        let light = Light::new(Vec3::point(0, 0, -10), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(
            &Matrix::identity_matrix(4),
            light,
            position,
            eyev,
            normalv,
            false,
        );
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }
    #[test]
    fn lighting_light_eye_offset_surface() {
        let m = Material::default();
        let position = Vec3::point(0, 0, 0);
        let eyev = Vec3::vector(0.0, f32::sqrt(2.0) / 2.0, -1.0 * f32::sqrt(2.0) / 2.0);
        let normalv = Vec3::vector(0, 0, -1);
        let light = Light::new(Vec3::point(0, 0, -10), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(
            &Matrix::identity_matrix(4),
            light,
            position,
            eyev,
            normalv,
            false,
        );
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }
    #[test]
    fn lighting_light_offset_eye_surface() {
        let m = Material::default();
        let position = Vec3::point(0, 0, 0);
        let eyev = Vec3::vector(0, 0, -1);
        let normalv = Vec3::vector(0, 0, -1);
        let light = Light::new(Vec3::point(0, 10, -10), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(
            &Matrix::identity_matrix(4),
            light,
            position,
            eyev,
            normalv,
            false,
        );
        assert_eq!(result, Color::new(0.7363961, 0.7363961, 0.7363961));
    }
    #[test]
    fn lighting_eye_in_path_of_reflectionv() {
        let m = Material::default();
        let position = Vec3::point(0, 0, 0);
        let eyev = Vec3::vector(
            0.0,
            -1.0 * f32::sqrt(2.0) / 2.0,
            -1.0 * f32::sqrt(2.0) / 2.0,
        );
        let normalv = Vec3::vector(0, 0, -1);
        let light = Light::new(Vec3::point(0, 10, -10), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(
            &Matrix::identity_matrix(4),
            light,
            position,
            eyev,
            normalv,
            false,
        );
        assert_eq!(result, Color::new(1.6363853, 1.6363853, 1.6363853));
    }
    #[test]
    fn lighting_light_behind_surface() {
        let m = Material::default();
        let position = Vec3::point(0, 0, 0);
        let eyev = Vec3::vector(0, 0, -1);
        let normalv = Vec3::vector(0, 0, -1);
        let light = Light::new(Vec3::point(0, 0, 10), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(
            &Matrix::identity_matrix(4),
            light,
            position,
            eyev,
            normalv,
            false,
        );
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
    #[test]
    fn lighting_with_surface_in_shadow() {
        let m = Material::default();
        let position = Vec3::point(0, 0, 0);
        let eyev = Vec3::vector(0, 0, -1);
        let normalv = Vec3::vector(0, 0, -1);
        let light = Light::new(Vec3::point(0, 0, -10), Color::new(1.0, 1.0, 1.0));
        let in_shadow = true;
        let result = m.lighting(
            &Matrix::identity_matrix(4),
            light,
            position,
            eyev,
            normalv,
            in_shadow,
        );
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
    #[test]
    fn lighting_with_pattern_applied() {
        let m = Material::new(
            WHITE,
            1.0,
            0.0,
            0.0,
            10.0,
            0.0,
            Some(Pattern::stripe(vec![WHITE, BLACK])),
        );
        let eyev = Vec3::vector(0, 0, -1);
        let normalv = Vec3::vector(0, 0, -1);
        let light = Light::new(Vec3::point(0, 0, -10), Color::new(1.0, 1.0, 1.0));
        let c1 = m.lighting(
            &Matrix::identity_matrix(4),
            light,
            Vec3::point(0.9, 0.0, 0.0),
            eyev,
            normalv,
            false,
        );
        let c2 = m.lighting(
            &Matrix::identity_matrix(4),
            light,
            Vec3::point(1.1, 0.0, 0.0),
            eyev,
            normalv,
            false,
        );
        assert_eq!(c1, Color::new(1.0, 1.0, 1.0));
        assert_eq!(c2, Color::new(0.0, 0.0, 0.0));
    }
}
