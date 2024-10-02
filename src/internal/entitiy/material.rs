// ray_intersect.rs

use super::color::Color;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse: Color,
    pub specular: f32,
    pub albedo: [f32; 2],
    pub reflectivity: f32,
    pub transparency : f32,
    pub refractive_index : f32
}
impl Material {
    pub fn black() -> Material {
        Material {
            diffuse : Color::new(0, 0, 0),
            specular  : 0.0,
            albedo: [0.0, 0.0],
            reflectivity: 0.0,
            transparency : 0.0,
            refractive_index: 0.0
        }
    }
}
