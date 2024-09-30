mod internal;

use internal::camera::Camera;
use minifb::{Window, WindowOptions, Key};
use nalgebra_glm::Vec3;
use std::time::Duration;
use std::f32::consts::PI;
use internal::framebuffer::Framebuffer;
use internal::render::render;
use internal::entitiy::color::Color;
use internal::entitiy::material::Material;
use internal::entitiy::sphere::Sphere;
use internal::entitiy::object::Object;

pub fn start(){
    
    // Window Size configuration
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width =  window_width;
    let framebuffer_height = window_height;
    
    // Frame Rate
    let frame_delay = Duration::from_millis(16);
  
    // Window Objects initialization
    let mut framebuffer = Framebuffer::new(window_width, window_height, Color::new(0, 0, 0));
    let mut window = Window::new(
      "Minecraft Diorama",
      window_width,
      window_height,
      WindowOptions::default()
    ).unwrap();
    
    // Create an array of Box<dyn Object>
    let whiteFur = Material{diffuse: Color::new(240, 240, 240)};
    let blackFur = Material{diffuse: Color::new(10, 10, 10)};

    let objects: [Box<dyn Object>; 8] = [
        Box::new(Sphere{ center: Vec3::new(0.0, 0.0, -10.0), radius: 4.0, material : whiteFur}),// FACE
        Box::new(Sphere{ center: Vec3::new(0.9, 0.4, -5.0), radius: 0.8, material: blackFur}),  // EYE PATH
        Box::new(Sphere{ center: Vec3::new(-0.9, 0.4, -5.0), radius: 0.8, material: blackFur}),
        Box::new(Sphere{ center: Vec3::new(0.0, -0.4, -5.0), radius: 0.3, material: blackFur}), // NOSE
        Box::new(Sphere{ center: Vec3::new(0.4, 0.1, -3.0), radius: 0.1, material: whiteFur}), // EYE
        Box::new(Sphere{ center: Vec3::new(-0.4, 0.1, -3.0), radius: 0.1, material: whiteFur}), // EYE
        Box::new(Sphere{ center: Vec3::new(4.0, 3.5, -14.0), radius: 3.0, material: blackFur}), // EARS
        Box::new(Sphere{ center: Vec3::new(-4.0, 3.5, -14.0), radius: 3.0, material: blackFur}), // EARS
    ];
    
    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 15.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0)
    );
    
    const ROTATION_SPEED : f32 = PI / 10.0;

    // RENDER LOOP
    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }
        
        // camera orbit controls
        if window.is_key_down(Key::Right) {
            camera.orbit(ROTATION_SPEED, 0.0);
        }
        if window.is_key_down(Key::Left) {
            camera.orbit(-ROTATION_SPEED, 0.0);
        }
        if window.is_key_down(Key::Down) {
            camera.orbit(0.0, -ROTATION_SPEED);
        }
        if window.is_key_down(Key::Up) {
            camera.orbit(0.0, ROTATION_SPEED);
        }

        render(&mut framebuffer, &objects, &camera);

        window
         .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
         .unwrap();

        std::thread::sleep(frame_delay)
    }
  
}