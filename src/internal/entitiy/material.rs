use once_cell::sync::Lazy;

use super::color::Color;
use std::sync::Arc;
use super::texture::Texture;

// use once_cell::sync::Lazy;
// use std::sync::Arc;

// I WANT TO DEFINE TEXTURE LIKE SO, so that are

#[derive(Debug, Clone)]
pub enum Diffuse {
    Color(Color),
    Texture(Arc<Texture>)
}

#[derive(Debug, Clone)]
pub struct Material {
    pub diffuse: Diffuse,
    pub specular: f32,
    pub albedo: [f32; 2],
    pub reflectivity: f32,
    pub transparency : f32,
    pub refractive_index : f32
}
impl Material {
    pub fn black() -> Material {
        Material {
            diffuse : Diffuse::Color(Color::new(0, 0, 0)),
            specular  : 0.0,
            albedo: [0.0, 0.0],
            reflectivity: 0.0,
            transparency : 0.0,
            refractive_index: 0.0
        }
    }
}
