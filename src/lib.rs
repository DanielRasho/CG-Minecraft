mod internal;

use internal::camera::Camera;
use minifb::{Window, WindowOptions, Key};
use nalgebra_glm::Vec3;
use std::sync::Arc;
use std::time::Duration;
use std::f32::consts::PI;
use internal::framebuffer::Framebuffer;
use internal::render::render;
use internal::entitiy::color::Color;
use internal::entitiy::material::{Material, Diffuse};
use internal::entitiy::cube::Cube;
// use internal::entitiy::sphere::Sphere;
use internal::entitiy::object::Object;
use internal::entitiy::light::{Light, AmbientLight};

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
    let white_fur = Arc::new( Material {
        diffuse: Diffuse::Color(Color::new(100, 100, 100)),
        specular: 10.0,
        albedo: [0.6, 0.3],
        reflectivity: 0.2,
        transparency: 0.0,
        refractive_index: 1.0,
    });
    
    let black_fur = Arc::new (Material {
        diffuse: Diffuse::Color(Color::new(80, 0, 0)),
        specular: 10.0,
        albedo: [0.9, 0.1],
        reflectivity: 0.2,
        transparency: 0.5,
        refractive_index: 1.0,
    });

    let objects: [Box<dyn Object + Sync>; 2] = [
        Box::new(Cube{ max: Vec3::new(0.5,0.5,0.5), min: Vec3::new(-0.5, -0.5, -0.5), material: Arc::clone(&white_fur)}), // EARS
        Box::new(Cube{ max: Vec3::new(1.0,2.0,2.5), min: Vec3::new(0.0, 1.0, 1.5), material: Arc::clone(&white_fur)}), // EARS
    ];
    
    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 15.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0)
    );
    
    let lights : [Light; 2] = [
        Light::new(
        Vec3::new(0.0, 7.0, 7.0),
        Color::new(255, 255, 255),
        1.0),
        Light::new(
        Vec3::new(0.0, 7.0, 7.0),
        Color::new(10, 255, 10),
        1.0),
    ];

    let ambient_light = AmbientLight::new(Color::new(230, 164, 50), 0.3);
    
    const ROTATION_SPEED : f32 = PI / 10.0;
    const ZOOM_SPEED : f32 = 0.2;

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

        // camera zoom
        if window.is_key_down(Key::J) {
            camera.zoom(ZOOM_SPEED);
        }
        if window.is_key_down(Key::K) {
            camera.zoom(-ZOOM_SPEED);
        }

        if camera.check_if_changed() {
            render(&mut framebuffer, &objects, &camera, &lights, &ambient_light);
        }

        window
         .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
         .unwrap();

        std::thread::sleep(frame_delay)
    }
  
}
