// ray_intersect.rs

use super::color::Color;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse: Color,
}
impl Material {
    pub fn black() -> Material {
        Material {
            diffuse : Color::new(0, 0, 0)
        }
    }
}