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
use internal::entitiy::object::Object;
use internal::entitiy::light::{AmbientLight, DayLight, Light, PointLight};
use internal::entitiy::texture::{BOOK_SHELF, CHEST, COBBLESTONE, CRAFTING_TABLE, DARK_OAK_PLANKS, FURNACE, GLASS, GLOWSTONE, GRASS, JUKEBOX, OAK_LOG, OAK_PLANKS};
use internal::entitiy::grid::{self, Grid};

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
    let grass = Arc::new( Material {
        diffuse: Diffuse::Texture(GRASS.clone()),
        specular: 10.0,
        albedo: [0.6, 0.3],
        reflectivity: 0.0,
        transparency: 0.0,
        refractive_index: 1.0,
    });
    
    let cobbleston = Arc::new (Material {
        diffuse: Diffuse::Texture(COBBLESTONE.clone()),
        specular: 10.0,
        albedo: [0.9, 0.1],
        reflectivity: 0.2,
        transparency: 0.0,
        refractive_index: 1.0,
    });

    let oak_log = Arc::new (Material {
        diffuse: Diffuse::Texture(OAK_LOG.clone()),
        specular: 10.0,
        albedo: [0.9, 0.1],
        reflectivity: 0.2,
        transparency: 0.0,
        refractive_index: 1.0,
    });

    let oak_planks = Arc::new (Material {
        diffuse: Diffuse::Texture(OAK_PLANKS.clone()),
        specular: 10.0,
        albedo: [0.9, 0.1],
        reflectivity: 0.2,
        transparency: 0.0,
        refractive_index: 1.0,
    });

    let dark_oak_planks = Arc::new (Material {
        diffuse: Diffuse::Texture(DARK_OAK_PLANKS.clone()),
        specular: 10.0,
        albedo: [0.9, 0.1],
        reflectivity: 0.2,
        transparency: 0.0,
        refractive_index: 1.0,
    });

    let furnace = Arc::new (Material {
        diffuse: Diffuse::Texture(FURNACE.clone()),
        specular: 10.0,
        albedo: [0.9, 0.1],
        reflectivity: 0.2,
        transparency: 0.0,
        refractive_index: 1.0,
    });

    let book_shelf = Arc::new (Material {
        diffuse: Diffuse::Texture(BOOK_SHELF.clone()),
        specular: 10.0,
        albedo: [0.9, 0.1],
        reflectivity: 0.2,
        transparency: 0.0,
        refractive_index: 1.0,
    });
    let jukebox = Arc::new (Material {
        diffuse: Diffuse::Texture(JUKEBOX.clone()),
        specular: 10.0,
        albedo: [0.9, 0.1],
        reflectivity: 0.2,
        transparency: 0.0,
        refractive_index: 1.0,
    });

    let chest = Arc::new (Material {
        diffuse: Diffuse::Texture(CHEST.clone()),
        specular: 10.0,
        albedo: [0.9, 0.1],
        reflectivity: 0.2,
        transparency: 0.0,
        refractive_index: 1.0,
    });

    let crafting_table = Arc::new (Material {
        diffuse: Diffuse::Texture(CRAFTING_TABLE.clone()),
        specular: 10.0,
        albedo: [0.9, 0.1],
        reflectivity: 0.2,
        transparency: 0.0,
        refractive_index: 1.0,
    });

    let glass = Arc::new (Material {
        diffuse: Diffuse::Texture(GLASS.clone()),
        specular: 10.0,
        albedo: [0.9, 0.1],
        reflectivity: 0.2,
        transparency: 0.0,
        refractive_index: 1.0,
    });

    let glowstone = Arc::new (Material {
        diffuse: Diffuse::Texture(GLOWSTONE.clone()),
        specular: 10.0,
        albedo: [0.9, 0.1],
        reflectivity: 0.2,
        transparency: 0.0,
        refractive_index: 1.0,
    });

    let mut diorama = Grid::new(1.0, 11, 9, 10);
    diorama.add_layer(0, &vec![
        (0, 0, Arc::clone(&grass)),
        (1, 0, Arc::clone(&grass)),
        (2, 0, Arc::clone(&grass)),
        (3, 0, Arc::clone(&grass)),
        (4, 0, Arc::clone(&grass)),
        (5, 0, Arc::clone(&grass)),
        (6, 0, Arc::clone(&grass)),
        (7, 0, Arc::clone(&grass)),
        (8, 0, Arc::clone(&grass)),

        (0, 1, Arc::clone(&grass)),
        (0, 2, Arc::clone(&grass)),
        (0, 3, Arc::clone(&grass)),
        (0, 4, Arc::clone(&grass)),
        (0, 5, Arc::clone(&grass)),
        (0, 6, Arc::clone(&grass)),
        (0, 7, Arc::clone(&grass)),

        (8, 1, Arc::clone(&grass)),
        (8, 2, Arc::clone(&grass)),
        (8, 3, Arc::clone(&grass)),
        (8, 4, Arc::clone(&grass)),
        (8, 5, Arc::clone(&grass)),
        (8, 6, Arc::clone(&grass)),
        (8, 7, Arc::clone(&grass)),
        (8, 8, Arc::clone(&grass)),
        (8, 9, Arc::clone(&grass)),

        (2, 9, Arc::clone(&grass)),
        (3, 9, Arc::clone(&grass)),
        (4, 9, Arc::clone(&grass)),
        (5, 9, Arc::clone(&grass)),
        (6, 9, Arc::clone(&grass)),
        (7, 9, Arc::clone(&grass)),
       
        (1, 8, Arc::clone(&grass)),
        (1, 7, Arc::clone(&grass)),
        (2, 8, Arc::clone(&grass)),
    ]);

    diorama.add_layer(1, &vec![
        (1, 1, Arc::clone(&oak_log)),
        (7, 1, Arc::clone(&oak_log)),
        (7, 8, Arc::clone(&oak_log)),
        
        
        (1, 2, Arc::clone(&cobbleston)),
        (1, 3, Arc::clone(&cobbleston)),
        (1, 4, Arc::clone(&cobbleston)),
        (1, 5, Arc::clone(&cobbleston)),
        (1, 6, Arc::clone(&cobbleston)),

        (2, 1, Arc::clone(&cobbleston)),
        (3, 1, Arc::clone(&cobbleston)),
        (4, 1, Arc::clone(&cobbleston)),
        (5, 1, Arc::clone(&cobbleston)),
        (6, 1, Arc::clone(&cobbleston)),

        (7, 2, Arc::clone(&cobbleston)),
        (7, 3, Arc::clone(&cobbleston)),
        (7, 4, Arc::clone(&cobbleston)),
        (7, 5, Arc::clone(&cobbleston)),
        (7, 6, Arc::clone(&cobbleston)),
        (7, 7, Arc::clone(&cobbleston)),

        (3, 8, Arc::clone(&cobbleston)),
        (4, 8, Arc::clone(&cobbleston)),
        (5, 8, Arc::clone(&cobbleston)),
        (6, 8, Arc::clone(&cobbleston)),

        (2, 7, Arc::clone(&cobbleston)),
        (3, 7, Arc::clone(&cobbleston)),
        (4, 7, Arc::clone(&cobbleston)),
        (5, 7, Arc::clone(&cobbleston)),

        (2, 6, Arc::clone(&cobbleston)),
        (3, 6, Arc::clone(&cobbleston)),
        (4, 6, Arc::clone(&cobbleston)),
        (5, 6, Arc::clone(&cobbleston)),

        (2, 5, Arc::clone(&cobbleston)),
        (3, 5, Arc::clone(&cobbleston)),
        (4, 5, Arc::clone(&cobbleston)),
        (5, 5, Arc::clone(&cobbleston)),

        (3, 4, Arc::clone(&cobbleston)),
        (4, 4, Arc::clone(&cobbleston)),
        (5, 4, Arc::clone(&cobbleston)),
        (6, 4, Arc::clone(&cobbleston)),

        (3, 3, Arc::clone(&cobbleston)),
        (4, 3, Arc::clone(&cobbleston)),
        (5, 3, Arc::clone(&cobbleston)),
        (6, 3, Arc::clone(&cobbleston)),

        (2, 2, Arc::clone(&cobbleston)),
        (3, 2, Arc::clone(&cobbleston)),
        (4, 2, Arc::clone(&cobbleston)),
        (5, 2, Arc::clone(&cobbleston)),
        (6, 2, Arc::clone(&cobbleston)),
    ]);

    diorama.add_layer(2, &vec![
        (1, 1, Arc::clone(&oak_log)),
        (7, 1, Arc::clone(&oak_log)),
        (7, 8, Arc::clone(&oak_log)),

        (1, 2, Arc::clone(&oak_planks)),
        (1, 3, Arc::clone(&oak_planks)),
        (1, 4, Arc::clone(&oak_planks)),
        (1, 5, Arc::clone(&oak_planks)),

        (2, 1, Arc::clone(&oak_planks)),
        (3, 1, Arc::clone(&oak_planks)),
        (5, 1, Arc::clone(&oak_planks)),
        (6, 1, Arc::clone(&oak_planks)),

        (7, 2, Arc::clone(&oak_planks)),
        (7, 3, Arc::clone(&oak_planks)),
        (7, 4, Arc::clone(&oak_planks)),
        (7, 5, Arc::clone(&oak_planks)),
        (7, 6, Arc::clone(&oak_planks)),
        (7, 7, Arc::clone(&oak_planks)),
        
        (2, 3, Arc::clone(&crafting_table)),
        (2, 4, Arc::clone(&chest)),

        (6, 5, Arc::clone(&furnace)),
        (6, 6, Arc::clone(&furnace)),

        (6, 7, Arc::clone(&dark_oak_planks)),
    ]);

    diorama.add_layer(3, &vec![
        (1, 1, Arc::clone(&oak_log)),
        (7, 1, Arc::clone(&oak_log)),
        (7, 8, Arc::clone(&oak_log)),

        (1, 2, Arc::clone(&oak_planks)),
        (1, 3, Arc::clone(&oak_planks)),

        (2, 1, Arc::clone(&glass)),
        (3, 1, Arc::clone(&oak_planks)),
        (5, 1, Arc::clone(&oak_planks)),
        (6, 1, Arc::clone(&glass)),

        (7, 2, Arc::clone(&oak_planks)),
        (7, 3, Arc::clone(&oak_planks)),
        (7, 4, Arc::clone(&oak_planks)),
        (7, 5, Arc::clone(&oak_planks)),
        (7, 6, Arc::clone(&oak_planks)),
        (7, 7, Arc::clone(&oak_planks)),

        (6, 5, Arc::clone(&furnace)),

        (6, 6, Arc::clone(&dark_oak_planks)),
    ]);
    
    diorama.add_layer(4, &vec![
        (1, 1, Arc::clone(&oak_log)),
        (7, 1, Arc::clone(&oak_log)),
        (7, 8, Arc::clone(&oak_log)),

        (1, 2, Arc::clone(&oak_planks)),

        (2, 1, Arc::clone(&oak_planks)),
        (3, 1, Arc::clone(&oak_planks)),
        (4, 1, Arc::clone(&oak_planks)),
        (5, 1, Arc::clone(&oak_planks)),
        (6, 1, Arc::clone(&oak_planks)),

        (7, 2, Arc::clone(&oak_planks)),
        (7, 3, Arc::clone(&oak_planks)),
        (7, 4, Arc::clone(&oak_planks)),
        (7, 5, Arc::clone(&oak_planks)),
        (7, 6, Arc::clone(&glass)),
        (7, 7, Arc::clone(&oak_planks)),

        (6, 5, Arc::clone(&dark_oak_planks)),
    ]);

    diorama.add_layer(5, &vec![
        (1, 1, Arc::clone(&oak_log)),
        (7, 1, Arc::clone(&oak_log)),
        (7, 8, Arc::clone(&oak_log)),

        (1, 2, Arc::clone(&oak_planks)),

        (2, 1, Arc::clone(&oak_planks)),
        (3, 1, Arc::clone(&oak_planks)),
        (4, 1, Arc::clone(&oak_planks)),
        (5, 1, Arc::clone(&oak_planks)),
        (6, 1, Arc::clone(&oak_planks)),

        (7, 2, Arc::clone(&oak_planks)),
        (7, 3, Arc::clone(&oak_planks)),
        (7, 4, Arc::clone(&oak_planks)),
        (7, 5, Arc::clone(&oak_planks)),
        (7, 6, Arc::clone(&oak_planks)),
        (7, 7, Arc::clone(&oak_planks)),

        (6, 4, Arc::clone(&dark_oak_planks)),
        
        (3, 5, Arc::clone(&dark_oak_planks)),
        (4, 5, Arc::clone(&dark_oak_planks)),
        (5, 5, Arc::clone(&dark_oak_planks)),

        (2, 4, Arc::clone(&dark_oak_planks)),
        (3, 4, Arc::clone(&dark_oak_planks)),
        (4, 4, Arc::clone(&dark_oak_planks)),
        (5, 4, Arc::clone(&dark_oak_planks)),

        (2, 3, Arc::clone(&dark_oak_planks)),
        (3, 3, Arc::clone(&dark_oak_planks)),
        (4, 3, Arc::clone(&dark_oak_planks)),
        (5, 3, Arc::clone(&dark_oak_planks)),

        (2, 2, Arc::clone(&glowstone)),
        (3, 2, Arc::clone(&dark_oak_planks)),
        (4, 2, Arc::clone(&dark_oak_planks)),
        (5, 2, Arc::clone(&dark_oak_planks)),
        (6, 2, Arc::clone(&glowstone)),
    ]);

    diorama.add_layer(6, &vec![
        (1, 1, Arc::clone(&oak_log)),
        (7, 1, Arc::clone(&oak_log)),
        (7, 8, Arc::clone(&oak_log)),

        (1, 2, Arc::clone(&oak_planks)),

        (2, 1, Arc::clone(&glass)),
        (3, 1, Arc::clone(&glass)),
        (4, 1, Arc::clone(&glass)),
        (5, 1, Arc::clone(&glass)),
        (6, 1, Arc::clone(&glass)),

        (7, 2, Arc::clone(&oak_planks)),
        (7, 3, Arc::clone(&oak_planks)),
        (7, 4, Arc::clone(&oak_planks)),
        (7, 5, Arc::clone(&oak_planks)),
        (7, 6, Arc::clone(&oak_planks)),
        (7, 7, Arc::clone(&oak_planks)),

        (6, 3, Arc::clone(&jukebox)),

        (2, 2, Arc::clone(&book_shelf)),
        (2, 3, Arc::clone(&book_shelf)),
    ]);
    
    diorama.add_layer(7, &vec![
        (2, 1, Arc::clone(&oak_planks)),
        (3, 1, Arc::clone(&glass)),
        (4, 1, Arc::clone(&glass)),
        (5, 1, Arc::clone(&glass)),
        (6, 1, Arc::clone(&oak_planks)),

        (1, 0, Arc::clone(&dark_oak_planks)),
        (1, 1, Arc::clone(&dark_oak_planks)),
        (1, 2, Arc::clone(&dark_oak_planks)),
        (1, 3, Arc::clone(&dark_oak_planks)),
        (1, 4, Arc::clone(&dark_oak_planks)),

        (7, 0, Arc::clone(&dark_oak_planks)),
        (7, 1, Arc::clone(&dark_oak_planks)),
        (7, 2, Arc::clone(&dark_oak_planks)),
        (7, 3, Arc::clone(&dark_oak_planks)),
        (7, 4, Arc::clone(&dark_oak_planks)),
        (7, 5, Arc::clone(&dark_oak_planks)),
        (7, 6, Arc::clone(&dark_oak_planks)),
        (7, 7, Arc::clone(&dark_oak_planks)),
        (7, 8, Arc::clone(&dark_oak_planks)),
    ]);

    diorama.add_layer(8, &vec![
        (3, 1, Arc::clone(&oak_planks)),
        (4, 1, Arc::clone(&glass)),
        (5, 1, Arc::clone(&oak_planks)),

        (2, 0, Arc::clone(&dark_oak_planks)),
        (2, 1, Arc::clone(&dark_oak_planks)),
        (2, 2, Arc::clone(&dark_oak_planks)),
        (2, 3, Arc::clone(&dark_oak_planks)),
        (2, 4, Arc::clone(&dark_oak_planks)),
        (2, 5, Arc::clone(&dark_oak_planks)),

        (6, 0, Arc::clone(&dark_oak_planks)),
        (6, 1, Arc::clone(&dark_oak_planks)),
        (6, 2, Arc::clone(&dark_oak_planks)),
        (6, 3, Arc::clone(&dark_oak_planks)),
        (6, 4, Arc::clone(&dark_oak_planks)),
        (6, 5, Arc::clone(&dark_oak_planks)),
        (6, 6, Arc::clone(&dark_oak_planks)),
        (6, 7, Arc::clone(&dark_oak_planks)),
    ]);
    diorama.add_layer(9, &vec![
        (4, 0, Arc::clone(&dark_oak_planks)),

        (3, 0, Arc::clone(&dark_oak_planks)),
        (3, 1, Arc::clone(&dark_oak_planks)),
        (3, 2, Arc::clone(&dark_oak_planks)),
        (3, 3, Arc::clone(&dark_oak_planks)),
        (3, 4, Arc::clone(&dark_oak_planks)),
        (3, 5, Arc::clone(&dark_oak_planks)),

        (5, 0, Arc::clone(&dark_oak_planks)),
        (5, 1, Arc::clone(&dark_oak_planks)),
        (5, 2, Arc::clone(&dark_oak_planks)),
        (5, 3, Arc::clone(&dark_oak_planks)),
        (5, 4, Arc::clone(&dark_oak_planks)),
        (5, 5, Arc::clone(&dark_oak_planks)),
        (5, 6, Arc::clone(&dark_oak_planks)),
    ]);
    diorama.add_layer(10, &vec![

        (4, 0, Arc::clone(&dark_oak_planks)),
        (4, 1, Arc::clone(&dark_oak_planks)),
        (4, 2, Arc::clone(&dark_oak_planks)),
        (4, 3, Arc::clone(&dark_oak_planks)),
        (4, 4, Arc::clone(&dark_oak_planks)),
        (4, 5, Arc::clone(&dark_oak_planks)),
    ]);
    
    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 20.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0)
    );
    
    let lights: [Box<dyn Light + Sync>; 2] = [
        Box::new(
            PointLight::new(
            Vec3::new(-0.5, -2.0, 1.5),
            Color::new(242, 130, 39),
            0.5)
        ),
        Box::new(
            PointLight::new(
            Vec3::new(-0.5, 1.0, 0.0),
            Color::new(242, 130, 39),
            0.5)
        ),
    ];

    let ambient_light = AmbientLight::new(Color::new(230, 164, 50), 0.3);

    let mut sun = DayLight::new(
        Vec3::new(10.0, 0.0, 0.0), 
        Vec3::new(0.0,0.0,0.0),
        10.0, 0.0, 
        Color::new(255, 255, 255), 
        1.0
    );
    
    
    let mut day_angle = PI / 3.0;
    
    const ROTATION_SPEED : f32 = PI / 10.0;
    const ZOOM_SPEED : f32 = 0.2;
    const DAY_SPEED : f32 = PI / 40.0;

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

        // Day Change
        if window.is_key_down(Key::M) {
            sun.translate_day_light(DAY_SPEED);
            day_angle = (day_angle + DAY_SPEED) % (2.0 * PI);
            render(&mut framebuffer, &diorama.objects, &camera, &lights, &sun, &ambient_light);
        }
        if window.is_key_down(Key::N) {
            sun.translate_day_light(-DAY_SPEED);
            day_angle = (day_angle - DAY_SPEED) % (2.0 * PI);
            render(&mut framebuffer, &diorama.objects, &camera, &lights, &sun, &ambient_light);
        }

        if camera.check_if_changed() {
            render(&mut framebuffer, &diorama.objects, &camera, &lights, &sun, &ambient_light);
        }

        window
         .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
         .unwrap();

        std::thread::sleep(frame_delay)
    }
  
}

