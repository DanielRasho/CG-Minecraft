use nalgebra_glm::{Vec3, dot};
use super::material::Material;
use super::intersect::Intersect;
use super::object::Object;

pub struct Sphere{
    pub center: Vec3,
    pub radius: f32,
    pub material: Material
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Sphere{
        Sphere{center, radius, material}
    }
}

impl Object for Sphere{
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        let oc = *ray_origin - self.center;

        let a = ray_direction.dot(ray_direction);
        let b = 2.0 * oc.dot(ray_direction);
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            let t = (-b - discriminant.sqrt()) / (2.0 * a);
            if t > 0.0 {
                let point = ray_origin + ray_direction * t;
                let normal = (point - self.center).normalize();
                let distance = t;

                return Intersect::new(point, normal, distance, self.material);
            }
        }

        Intersect::empty()
    }
}