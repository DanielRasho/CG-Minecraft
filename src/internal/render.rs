use rayon::prelude::*;

use nalgebra_glm::Vec3;
use std::f32::consts::PI;

use super::camera::Camera;
use super::framebuffer::Framebuffer;
use super::entitiy::color::Color;
use super::entitiy::intersect::Intersect;
use super::entitiy::object::Object;
use super::entitiy::light::{Light,  AmbientLight};

const REFLECTION_DEPTH: u32 = 3;
const ORIGIN_BIAS: f32 = 1e-4;

pub fn cast_ray(
    ray_origin: &Vec3,
    ray_direction: &Vec3,
    objects: &[Box<dyn Object + Sync>],
    lights: &[Box<dyn Light + Sync>],
    day_angle: f32,
    ambient_light: &AmbientLight,
    depth: u32,
) -> Color {
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
        return calculate_background_color(day_angle);
    }

    // Start with ambient light contribution (scaled by ambient intensity)
    let mut final_color = intersect.color * ambient_light.intensity;

    // Iterate over each light and accumulate contributions
    for light in lights {
        // Calculate diffuse and specular light for each light source
        let light_dir = (light.get_position() - intersect.point).normalize();
        let view_dir = (ray_origin - intersect.point).normalize();
        let reflect_dir = reflect(&-light_dir, &intersect.normal); // Reflect direction calculated per light

        let shadow_intensity = cast_shadow(&intersect, light, objects);
        let light_intensity = light.get_intensity() * (1.0 - shadow_intensity);

        let diffuse_intensity = intersect.normal.dot(&light_dir).clamp(0.0, 1.0);
        let diffuse = intersect.color * intersect.material.albedo[0] * diffuse_intensity * light_intensity;

        let specular_intensity = view_dir.dot(&reflect_dir).max(0.0).powf(intersect.material.specular);
        let specular = light.get_color() * intersect.material.albedo[1] * specular_intensity * light_intensity;

        // Add diffuse and specular components to the final color
        final_color = final_color + diffuse + specular;
    }

    // Calculate reflection (move reflect calculation outside of the loop)
    let mut reflect_color = Color::new(0, 0, 0);
    let reflectivity = intersect.material.reflectivity;
    if reflectivity > 0.0 {
        let reflect_dir = reflect(&-ray_direction, &intersect.normal).normalize(); // Now using ray direction for reflection
        let reflect_origin = intersect.point;
        reflect_color = cast_ray(&reflect_origin, &reflect_dir, objects, lights, day_angle, ambient_light, depth + 1);
    }

    // Calculate refraction
    let mut refract_color = Color::new(0, 0, 0);
    let transparency = intersect.material.transparency;
    if transparency > 0.0 {
        let refract_dir = refract(ray_direction, &intersect.normal, intersect.material.refractive_index);
        let refract_origin = offset_origin(&intersect, &refract_dir);
        refract_color = cast_ray(&refract_origin, &refract_dir, objects, lights, day_angle, ambient_light, depth + 1);
    }

    // Combine the results of lighting, reflection, and refraction
    (final_color) * (1.0 - reflectivity)
        + (reflect_color * reflectivity)
        + (refract_color * transparency)
}

pub fn render(
    framebuffer: &mut Framebuffer,
    objects: &[Box<dyn Object + Sync>],
    camera: &Camera,
    lights: &[Box<dyn Light + Sync>],
    day_angle: f32,
    ambient_light: &AmbientLight,
) {
    const FIELD_OF_VIEW: f32 = PI / 3.0;
    let perspective_scale: f32 = (FIELD_OF_VIEW / 2.0).tan();

    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;

    // Process each row of the framebuffer in parallel
    framebuffer
        .buffer
        .par_chunks_mut(framebuffer.width as usize)
        .enumerate()
        .for_each(|(y, row)| {
            // Map the pixel coordinate to screen space [-1, 1]
            let screen_y = -(2.0 * y as f32) / height + 1.0;
            let screen_y = screen_y * perspective_scale;

            row.iter_mut().enumerate().for_each(|(x, pixel)| {
                // Map the pixel coordinate to screen space [-1, 1]
                let screen_x = (2.0 * x as f32) / width - 1.0;
                let screen_x = screen_x * aspect_ratio * perspective_scale;

                // Calculate the direction of the ray for this pixel
                let ray_direction = Vec3::new(screen_x, screen_y, -1.0).normalize();
                let rotated_direction = camera.change_basis(&ray_direction);

                // Cast the ray and get the pixel color
                let pixel_color =
                    cast_ray(&camera.eye, &rotated_direction, objects, lights, day_angle, ambient_light, REFLECTION_DEPTH);
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

fn cast_shadow(intersect: &Intersect, light: &Box<dyn Light + Sync>, objects: &[Box<dyn Object + Sync>]) -> f32 {
    let light_dir = (light.get_position() - intersect.point).normalize();
    let shadow_ray_origin = intersect.point + intersect.normal * 1e-4; // Avoid self-shadowing bias
    let light_distance = (light.get_position() - shadow_ray_origin).magnitude(); // Distance from light to intersection

    let mut shadow_intensity = 0.0; // Start with no shadow

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

fn calculate_background_color(day_angle: f32) -> Color {
    let angle = day_angle;

    const DAY_START: f32 = 0.0;                           // Amanecer
    const DAY_MID: f32 = std::f32::consts::PI / 3.0;     // Medio día (reducido)
    const DAY_END: f32 = std::f32::consts::PI * 2.0 / 3.0; // Atardecer (reducido)
    const NIGHT_START: f32 = DAY_END + 2.0;               // Inicio de la Noche   

    let r: u8;
    let g: u8;
    let b: u8;

    if angle >= DAY_START && angle < DAY_MID {
        // Gradiente de azul oscuro a amarillo-naranja y luego a celeste (Amanecer)
        let ratio = angle / (DAY_MID - DAY_START);
        
        // De azul oscuro (0) a naranja (255) a celeste (135)
        if ratio < 0.5 {
            let sub_ratio = ratio * 2.0; // Escalamos a [0, 1]
            r = (0.0 * (1.0 - sub_ratio) + 220.0 * sub_ratio) as u8; // Azul oscuro a naranja (220)
            g = (0.0 * (1.0 - sub_ratio) + 162.0 * sub_ratio) as u8; // Azul oscuro a naranja (162)
            b = (50.0 * (1.0 - sub_ratio) + 30.0 * sub_ratio) as u8; // Azul oscuro a naranja (30)
        } else {
            let sub_ratio = (ratio - 0.5) * 2.0; // Escalamos a [0, 1]
            r = (220.0 * (1.0 - sub_ratio) + 135.0 * sub_ratio) as u8; // Naranja (220) a celeste (135)
            g = (162.0 * (1.0 - sub_ratio) + 206.0 * sub_ratio) as u8; // Naranja (162) a celeste (206)
            b = (30.0 * (1.0 - sub_ratio) + 250.0 * sub_ratio) as u8; // Naranja (30) a celeste (250)
        }

    } else if angle >= DAY_MID && angle < DAY_END {
        // Día: Celeste
        r = 135;
        g = 206;
        b = 250;

    } else if angle >= DAY_END && angle < NIGHT_START {
        // Gradiente de celeste a amarillo-naranja y luego a azul oscuro (Atardecer)
        let ratio = (angle - DAY_END) / (NIGHT_START - DAY_END);
        
        // De celeste (135) a amarillo-naranja (255)
        if ratio < 0.5 {
            let sub_ratio = ratio * 2.0; // Escalamos a [0, 1]
            r = (135.0 * (1.0 - sub_ratio) + 220.0 * sub_ratio) as u8; // Celeste a naranja (220)
            g = (206.0 * (1.0 - sub_ratio) + 162.0 * sub_ratio) as u8; // Celeste a naranja (162)
            b = (250.0 * (1.0 - sub_ratio) + 30.0 * sub_ratio) as u8; // Celeste a naranja (30)
        } else {
            let sub_ratio = (ratio - 0.5) * 2.0; // Escalamos a [0, 1]
            r = (220.0 * (1.0 - sub_ratio) + 0.0 * sub_ratio) as u8; // Naranja (220) a azul oscuro (0)
            g = (162.0 * (1.0 - sub_ratio) + 0.0 * sub_ratio) as u8; // Naranja (162) a azul oscuro (0)
            b = (30.0 * (1.0 - sub_ratio) + 50.0 * sub_ratio) as u8; // Naranja (30) a azul oscuro (50)
        }

    } else {
        // Noche: Azul oscuro
        r = 0; 
        g = 0; 
        b = 50; // Azul oscuro
    }

    Color::new(r , g , b )
}