use nalgebra_glm::{Vec3};
use super::material::{Intersect, Material};
use std::f32::INFINITY;

pub trait Object {
   fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect;
}

pub struct Cube {
    pub min: Vec3, // minimum corner of the cube
    pub max: Vec3, // maximum corner of the cube
    pub material: Material,
}

impl Cube {
    pub fn new(min: Vec3, max: Vec3, material: Material) -> Cube {
        Cube { min, max, material }
    }
}

impl Object for Cube {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        // Initialize the tmin and tmax values for each axis
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
            return Intersect::empty(); // No intersection
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
            return Intersect::empty(); // No intersection
        }

        if tzmin > tmin {
            tmin = tzmin;
        }

        if tzmax < tmax {
            tmax = tzmax;
        }

        // If the cube is behind the ray, there is no intersection
        if tmin < 0.0 {
            return Intersect::empty();
        }

        // Calculate the intersection point and normal
        let distance = tmin;
        let intersection_point = ray_origin + ray_direction * distance;
        
        // Determine the normal by comparing the intersection point to the cube's faces
        let mut normal = Vec3::zeros();

        if intersection_point.x.abs() - self.min.x.abs() < 1e-4 {
            normal = Vec3::new(-1.0, 0.0, 0.0);
        } else if intersection_point.x.abs() - self.max.x.abs() < 1e-4 {
            normal = Vec3::new(1.0, 0.0, 0.0);
        } else if intersection_point.y.abs() - self.min.y.abs() < 1e-4 {
            normal = Vec3::new(0.0, -1.0, 0.0);
        } else if intersection_point.y.abs() - self.max.y.abs() < 1e-4 {
            normal = Vec3::new(0.0, 1.0, 0.0);
        } else if intersection_point.z.abs() - self.min.z.abs() < 1e-4 {
            normal = Vec3::new(0.0, 0.0, -1.0);
        } else if intersection_point.z.abs() - self.max.z.abs() < 1e-4 {
            normal = Vec3::new(0.0, 0.0, 1.0);
        }

        // Create and return the intersection object
        Intersect::new(intersection_point, normal, distance, self.material)
    }
}