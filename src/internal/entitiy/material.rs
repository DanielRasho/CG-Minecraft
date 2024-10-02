// ray_intersect.rs

use super::color::Color;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse: Color,
    pub specular: f32,
    pub albedo: [f32; 3]
}
impl Material {
    pub fn black() -> Material {
        Material {
            diffuse : Color::new(0, 0, 0),
            specular  : 0.0,
            albedo: [0.0, 0.0, 0.0]
        }
    }
}
