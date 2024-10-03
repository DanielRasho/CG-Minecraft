use rayon::prelude::*;
use std::f32::INFINITY;

use nalgebra_glm::Vec3;
use std::f32::consts::PI;

use super::camera::Camera;
use super::framebuffer::Framebuffer;
use super::entitiy::{color::Color, material::Material};
use super::entitiy::intersect::Intersect;
use super::entitiy::object::Object;
use super::entitiy::light::{Light, AmbientLight};

const REFLECTION_DEPTH : u32 = 2;
const ORIGIN_BIAS : f32 = 1e-4;

pub fn cast_ray(
    ray_origin: &Vec3, 
    ray_direction: &Vec3, 
    objects: &[Box<dyn Object + Sync>], 
    light: &Light,
    ambient_light: &AmbientLight,
    depth: u32 ) -> Color {
        if depth > 3 {
        return Color::new(25, 25, 120);
    }

    let mut intersect = Intersect::empty();

    for object in objects {
        let i = object.ray_intersect(ray_origin, ray_direction);
        if i.is_intersecting && i.distance < intersect.distance {
            intersect = i;
        }        
    }

    if !intersect.is_intersecting {
        return Color::new(25, 25, 120);
    }

    // Calculate diffuse and specular light
    let light_dir = (light.position - intersect.point).normalize();
    let view_dir = (ray_origin - intersect.point).normalize();
    let reflect_dir = reflect(&-light_dir, &intersect.normal);

    let shadow_intensity = cast_shadow(&intersect, light, objects);
    let light_intensity = light.intensity * (1.0 - shadow_intensity);

    let diffuse_intensity = intersect.normal.dot(&light_dir).clamp(0.0, 1.0);
    let diffuse = intersect.material.diffuse * intersect.material.albedo[0] * diffuse_intensity * light_intensity;

    let specular_intensity = view_dir.dot(&reflect_dir).max(0.0).powf(intersect.material.specular);
    let specular = light.color * intersect.material.albedo[1] * specular_intensity * light_intensity;

    // Calculate reflection
    let mut reflect_color = Color::new(0, 0, 0);
    let reflectivity = intersect.material.reflectivity;
    if reflectivity > 0.0 {
        let ray_reflection = reflect_dir.normalize();
        let reflect_origin = intersect.point;
        reflect_color = cast_ray(&reflect_origin, &ray_reflection, objects, light, ambient_light, depth + 1);
    }

    // Calculate refraction
    let mut refract_color = Color::new(0, 0, 0);
    let transparency = intersect.material.transparency;
    if transparency > 0.0 {
        let refract_dir = refract(ray_direction, &intersect.normal, intersect.material.refractive_index);
        let refract_origin = offset_origin(&intersect, &refract_dir);
        refract_color = cast_ray(&refract_origin, &refract_dir, objects, light, ambient_light, depth + 1);
    }
    
    // Calculate ambient light contribution
    let ambient = ambient_light.color * intersect.material.albedo[0] * ambient_light.intensity;  // Ambient is modulated by albedo

    // Combine the lighting effects
    (ambient + diffuse + specular) * (1.0 - reflectivity) 
        + (reflect_color * reflectivity) 
        + (refract_color * transparency)
}

pub fn render(framebuffer: &mut Framebuffer, objects: &[Box<dyn Object + Sync>], camera: &Camera, light: &Light, ambient_light: &AmbientLight) {
    const FIELD_OF_VIEW: f32 = PI / 3.0;
    let PERSPECTIVE_SCALE: f32 = (FIELD_OF_VIEW / 2.0).tan();

    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;

    // Process each row of the framebuffer in parallel
    framebuffer.buffer.par_chunks_mut(framebuffer.width as usize).enumerate().for_each(|(y, row)| {
        // Map the pixel coordinate to screen space [-1, 1]
        let screen_y = -(2.0 * y as f32) / height + 1.0;
        let screen_y = screen_y * PERSPECTIVE_SCALE;

        row.iter_mut().enumerate().for_each(|(x, pixel)| {
            // Map the pixel coordinate to screen space [-1, 1]
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_x = screen_x * aspect_ratio * PERSPECTIVE_SCALE;

            // Calculate the direction of the ray for this pixel
            let ray_direction = Vec3::new(screen_x, screen_y, -1.0).normalize();
            let rotated_direction = camera.change_basis(&ray_direction);

            // Cast the ray and get the pixel color
            let pixel_color = cast_ray(&camera.eye, &rotated_direction, objects, light, ambient_light, REFLECTION_DEPTH);
            *pixel = pixel_color.to_hex(); // Convert color to u32 and assign to pixel
        });
    });
}

fn offset_origin(intersect: &Intersect, direction: &Vec3) -> Vec3 {
    let offset = intersect.normal * ORIGIN_BIAS;
    if direction.dot(&intersect.normal) < 0.0 {
        intersect.point - offset
    } else {
        intersect.point + offset
    }
}

fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * normal
}

fn refract(incident: &Vec3, normal: &Vec3, eta_t: f32) -> Vec3 {
    let cosi = -incident.dot(normal).clamp(-1.0, 1.0); // Clamp cosine of incident angle between -1 and 1
    let (n_cosi, eta, n_normal);

    if cosi < 0.0 {
        // Ray is entering the object
        n_cosi = -cosi;
        eta = 1.0 / eta_t; // Refractive index ratio for entering the material
        n_normal = -normal;
    } else {
        // Ray is leaving the object
        n_cosi = cosi;
        eta = eta_t;  // Assume the ray is exiting into air or vacuum (n=1)
        n_normal = *normal;
    }

    let k = 1.0 - eta * eta * (1.0 - n_cosi * n_cosi);

    if k < 0.0 {
        // Total internal reflection occurs
        reflect(incident, &n_normal)
    } else {
        eta * incident + (eta * n_cosi - k.sqrt()) * n_normal
    }
}

fn cast_shadow(
    intersect: &Intersect,
    light: &Light,
    objects: &[Box<dyn Object + Sync>],
) -> f32 {
    let light_dir = (light.position - intersect.point).normalize();
    let shadow_ray_origin = intersect.point + intersect.normal * 1e-4; // Avoid self-shadowing bias
    let light_distance = (light.position - shadow_ray_origin).magnitude(); // Distance from light to intersection

    let mut shadow_intensity = 0.0;  // Start with no shadow

    for object in objects {
        let shadow_intersect = object.ray_intersect(&shadow_ray_origin, &light_dir);
        if shadow_intersect.is_intersecting {
            // Compute the distance from the intersection to the shadow-casting object
            let occlusion_distance = shadow_intersect.distance;

            // Calculate shadow attenuation based on the distance between the object and the light source
            let attenuation = (occlusion_distance / light_distance).clamp(0.0, 1.0); // The farther the object, the smaller the shadow

            // Modify shadow intensity based on the object's distance
            shadow_intensity = 1.0 * attenuation; // Full shadow intensity when close, less when far
            break;
        }
    }

    shadow_intensity
}
