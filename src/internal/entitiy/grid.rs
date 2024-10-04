use nalgebra_glm::Vec3;
use std::sync::Arc;
use super::material::Material;
use super::cube::Cube;
use super::object::Object; // Assuming you have an Object trait defined

pub struct Grid {
    pub cube_width: f32,
    pub height: usize,
    pub width: usize,
    pub depth: usize,
    pub objects: Vec<Box<dyn Object + Sync>>, // Change to store objects instead of cubes
}

impl Grid {
    pub fn new(cube_width: f32, grid_height: usize, grid_width: usize, grid_depth: usize) -> Grid {
        Grid {
            cube_width,
            height: grid_height,
            width: grid_width,
            depth: grid_depth,
            objects: Vec::new(), // Initialize objects vector
        }
    }

    pub fn add_cube(&mut self, x: usize, y: usize, z: usize, material: Arc<Material>) {
        // Check if coordinates are within the grid bounds
        let grid_x_half = self.width as f32 / 2.0;
        let grid_y_half = self.height as f32 / 2.0;
        let grid_z_half = self.depth as f32 / 2.0;

        if x < self.width && y < self.height && z < self.depth {
            let min = Vec3::new(
                (x as f32) - grid_x_half * self.cube_width,
                (y as f32) - grid_y_half * self.cube_width,
                (z as f32) - grid_z_half * self.cube_width,
            );
            let max = Vec3::new(
                min.x + self.cube_width,
                min.y + self.cube_width,
                min.z + self.cube_width,
            );

            let cube = Cube::new(min, max, material);
            self.objects.push(Box::new(cube)); // Store the cube as a Box<dyn Object + Sync>
        } else {
            eprintln!("Attempted to add a cube out of grid bounds.");
        }
    }

    pub fn add_layer(&mut self, layer: usize, list: &[(usize, usize, Arc<Material>)]) {
        // Check if the layer is within bounds
        if layer < self.height {
            for &(x, z, ref material) in list {
                self.add_cube(x, layer, z, Arc::clone(material));
            }
        } else {
            eprintln!("Attempted to add cubes to a layer out of grid bounds.");
        }
    }
}