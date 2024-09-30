
use std::f32::INFINITY;

use nalgebra_glm::Vec3;

use super::camera::Camera;
use super::framebuffer::{self, Framebuffer};
use super::entitiy::{color::Color, material::{Intersect, Material}};
use super::entitiy::sphere::{Object};

pub fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Box<dyn Object>]) -> Color {
    let mut intersect = Intersect{ 
        distance: INFINITY, 
        is_intersecting: false, 
        material: Material{
            diffuse : Color::new(66, 135, 245)
        }
    };

    for object in objects {
        let i = object.ray_intersect(ray_origin, ray_direction);
        if i.is_intersecting && i.distance < intersect.distance {
            intersect = i
        }        
    }
    
    return intersect.material.diffuse;
}

pub fn render(framebuffer: &mut Framebuffer, objects: &[Box<dyn Object>], camera: &Camera) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            // Map the pixel coordinate to screen space [-1, 1]
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            // Adjust for aspect ratio
            let screen_x = screen_x * aspect_ratio;

            // Calculate the direction of the ray for this pixel
            let ray_direction = Vec3::new(screen_x, screen_y, -1.0).normalize();

            let rotated_direction = camera.change_basis(&ray_direction);

            // println!("{},{},{}", ray_direction.x, ray_direction.y, ray_direction.z);
            // println!("{},{},{}", rotated_direction.x, rotated_direction.y, rotated_direction.z);
            // println!("========================");

            let pixel_color = cast_ray(&camera.eye, &rotated_direction, objects);

            // Draw the pixel on screen with the returned color
            framebuffer.set_current_color(pixel_color);
            framebuffer.draw_point(x, y);
        }
    }
}