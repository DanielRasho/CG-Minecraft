use nalgebra_glm::Vec3;
use super::intersect::Intersect;

pub trait Object : Sync {
   fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect;
}
