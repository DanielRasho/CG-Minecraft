use std::sync::Arc;
use nalgebra_glm::Vec3;
use super::material::{self, Diffuse, Material};
use super::intersect::Intersect;
use super::object::Object;
use super::color::Color;
use super::texture::Texture;

const NUM_FACE_COLUMNS: usize = 6; // Number of columns in the texture atlas
const NUM_FACE_ROWS: usize = 1;     // Number of rows in the texture atlas
const FACE_SIZE: f32 = 16.0;         // Size of each face in pixels
                                     //
pub struct Cube {
    pub min: Vec3,             // minimum corner of the cube
    pub max: Vec3,             // maximum corner of the cube
    pub material: Arc<Material>, // reference to the material
}

impl Cube{
    pub fn new(min: Vec3, max: Vec3, material: Arc<Material>) -> Cube {
        Cube { min, max, material }
    }
}

impl<'a> Object for Cube {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        let mut tmin = (self.min.x - ray_origin.x) / ray_direction.x;
        let mut tmax = (self.max.x - ray_origin.x) / ray_direction.x;

        if tmin > tmax {
            std::mem::swap(&mut tmin, &mut tmax);
        }

        let mut tymin = (self.min.y - ray_origin.y) / ray_direction.y;
        let mut tymax = (self.max.y - ray_origin.y) / ray_direction.y;

        if tymin > tymax {
            std::mem::swap(&mut tymin, &mut tymax);
        }

        if (tmin > tymax) || (tymin > tmax) {
            return Intersect::empty();
        }

        if tymin > tmin {
            tmin = tymin;
        }

        if tymax < tmax {
            tmax = tymax;
        }

        let mut tzmin = (self.min.z - ray_origin.z) / ray_direction.z;
        let mut tzmax = (self.max.z - ray_origin.z) / ray_direction.z;

        if tzmin > tzmax {
            std::mem::swap(&mut tzmin, &mut tzmax);
        }

        if (tmin > tzmax) || (tzmin > tmax) {
            return Intersect::empty();
        }

        if tzmin > tmin {
            tmin = tzmin;
        }

        if tzmax < tmax {
            tmax = tzmax;
        }

        // Si tmin es positivo, es una intersección en la dirección del rayo
        if tmin > 0.0 {
            let point = ray_origin + ray_direction * tmin;
            let normal = self.calculate_normal(&point); // Calcula la normal en el punto de intersección
            let distance = tmin;

            // Calculate UV coordinates
            let (u, v) = self.calculate_uv(&point, & normal);
            
            let surface_color = match &self.material.diffuse {
                Diffuse::Color(color) => color.clone(),
                Diffuse::Texture(texture) => {
                    // Sample color from texture atlas based on UV coordinates
                    let face = self.get_face_index(&normal).unwrap();
                    self.sample_texture(texture, u, v, face)
                }
            };

            // ERROR HERE
            return Intersect::new(point, normal, distance, &self.material, surface_color); // Now using a reference to the material
        }

        Intersect::empty()
    }
}

impl Cube {

    // Determine the face index based on the normal vector
    fn get_face_index(&self, normal: &Vec3) -> Option<usize> {
        if (normal.x).abs() > (normal.y).abs() && (normal.x).abs() > (normal.z).abs() {
            return if normal.x < 0.0 { Some(5) } else { Some(4) }; // Left or Right face
        } else if (normal.y).abs() > (normal.x).abs() && (normal.y).abs() > (normal.z).abs() {
            return if normal.y < 0.0 { Some(1) } else { Some(0) }; // Bottom or Top face
        } else {
            return if normal.z < 0.0 { Some(2) } else { Some(3) }; // Back or Front face
        }
    }
    
    fn calculate_normal(&self, point: &Vec3) -> Vec3 {
        // Comparamos el punto de intersección con las caras del cubo para determinar la normal
        let epsilon = 1e-4; // Un pequeño valor para la precisión

        if (point.x - self.min.x).abs() < epsilon {
            return Vec3::new(-1.0, 0.0, 0.0); // Cara izquierda
        } else if (point.x - self.max.x).abs() < epsilon {
            return Vec3::new(1.0, 0.0, 0.0); // Cara derecha
        } else if (point.y - self.min.y).abs() < epsilon {
            return Vec3::new(0.0, -1.0, 0.0); // Cara inferior
        } else if (point.y - self.max.y).abs() < epsilon {
            return Vec3::new(0.0, 1.0, 0.0); // Cara superior
        } else if (point.z - self.min.z).abs() < epsilon {
            return Vec3::new(0.0, 0.0, -1.0); // Cara trasera
        } else if (point.z - self.max.z).abs() < epsilon {
            return Vec3::new(0.0, 0.0, 1.0); // Cara frontal
        }

        Vec3::new(0.0, 0.0, 0.0) // Normal por defecto (si no se encuentra coincidencia)
    }

    fn calculate_uv(&self, point: &Vec3, normal: &Vec3) -> (f32, f32) {
        if (normal.x).abs() > (normal.y).abs() && (normal.x).abs() > (normal.z).abs() {
            // Left or right face (rotate by 180 degrees)
            let u = (point.z - self.min.z) / (self.max.z - self.min.z); // Z to U mapping, reversed
            let v = 1.0 - (point.y - self.min.y) / (self.max.y - self.min.y); // Y to V mapping
            return (u.clamp(0.0, 1.0), v.clamp(0.0, 1.0)); // Ensure u and v are within [0, 1]
        } else if (normal.y).abs() > (normal.x).abs() && (normal.y).abs() > (normal.z).abs() {
            // Bottom or top face
            let u = (point.x - self.min.x) / (self.max.x - self.min.x);
            let v = (point.z - self.min.z) / (self.max.z - self.min.z);
            // Ensure u and v are within [0, 1] range
            return (u.clamp(0.0, 1.0), v.clamp(0.0, 1.0));
        } else {
            // Front or back face
            let u = (point.x - self.min.x) / (self.max.x - self.min.x);
            let v = 1.0 - (point.y - self.min.y) / (self.max.y - self.min.y);
            // Ensure u and v are within [0, 1] range
            return (u.clamp(0.0, 1.0), v.clamp(0.0, 1.0));
        }
    }

    fn sample_texture(&self, texture: &Arc<Texture>, u: f32, v: f32, face_index: usize) -> Color {
        let tex_width = FACE_SIZE * NUM_FACE_COLUMNS as f32; // Total width for all columns
        let tex_height = FACE_SIZE * NUM_FACE_ROWS as f32;   // Total height for all rows
    
        // Calculate the row and column based on the face index
        let col = face_index % NUM_FACE_COLUMNS; // Number of columns
        let row = face_index / NUM_FACE_COLUMNS;  // Number of rows
    
        // Sample the color based on UV coordinates
        let x = (u * FACE_SIZE + (col as f32 * FACE_SIZE)).clamp(0.0, tex_width - 1.0) as usize;
        let y = (v * FACE_SIZE + (row as f32 * FACE_SIZE)).clamp(0.0, tex_height - 1.0) as usize;
    
        // Use the method to get color from the texture atlas
        texture.get_color(x, y) // Implement this method in your Texture struct
    }
}