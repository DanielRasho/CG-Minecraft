use nalgebra_glm::Vec3;
use super::material::Material;
use std::f32::INFINITY;
use super::color::Color;
use once_cell::sync::Lazy;

static BLACK_MATERIAL: Lazy<Material> = Lazy::new(|| Material::black());

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Intersect<'a> {
    pub point: Vec3,            // hitting location
    pub normal: Vec3,           // normal of the surface
    pub distance: f32,          // distance of the ray
    pub is_intersecting: bool,  // true if hit an object
    pub material: &'a Material, // material of the surface hit
    pub color: Color            // the actual color hit on the surface
}

impl<'a> Intersect<'a> {
    pub fn new(point: Vec3, normal: Vec3, distance: f32, material: &'a Material, color: Color) -> Self {
        Intersect {
            point,
            normal,
            distance,
            is_intersecting: true,
            material,
            color,
        }
    }

    pub fn empty() -> Self {
        Intersect {
            point: Vec3::zeros(),
            normal: Vec3::zeros(),
            distance: INFINITY,
            is_intersecting: false,
            material: &BLACK_MATERIAL,
            color: Color::new(0, 0, 0),
        }
    }
}
