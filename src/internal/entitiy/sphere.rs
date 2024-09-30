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
        // Calculate the vector from the ray's origin to the sphere's center
        let l = self.center - ray_origin;

        // Calculate the projection of l onto the ray direction
        let tca = dot(ray_direction, &l);

        // If tca is negative, the sphere is behind the ray, no intersection
        if tca < 0.0 {
            return Intersect::empty();
        }

        // Calculate the distance squared from the sphere's center to the ray
        let d2 = dot(&l, &l) - tca * tca;

        // If the distance is greater than the sphere's radius, no intersection
        if d2 > self.radius * self.radius {
            return Intersect::empty();
        }

        // Calculate thc, the distance from the closest approach to the intersection points
        let thc = (self.radius * self.radius - d2).sqrt();

        // Calculate the distances to the intersection points along the ray
        let t0 = tca - thc;
        let t1 = tca + thc;

        // If both t0 and t1 are negative, the sphere is behind the ray
        if t0 < 0.0 && t1 < 0.0 {
            return Intersect::empty();
        }

        // Determine the distance to the intersection point
        let distance = if t0 < 0.0 { t1 } else { t0 };

        // Calculate the intersection point and normal
        let intersection_point = ray_origin + ray_direction * distance;
        let normal = (intersection_point - self.center).normalize();

        // Create and return the intersection object
        Intersect::new(intersection_point, normal, distance, self.material)
    }
}