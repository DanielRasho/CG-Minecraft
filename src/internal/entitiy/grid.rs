use nalgebra_glm::Vec3;
use super::color::Color;
use super::material::Material;
use super::cube::Cube;

// Assuming you already have the Cube struct and the Object trait from the previous code

pub struct Grid {
    pub cubes: Vec<Cube>, // A list of cubes in the grid
}

impl Grid {
    pub fn new(width: f32, corner_points: Vec<Vec3>, material: Material) -> Grid {
        // Create an empty list to store the cubes
        let mut cubes = Vec::new();

        // Iterate over the provided corner points
        for point in corner_points {
            // Calculate the opposite corner of the cube
            let max_corner = point + Vec3::new(width, width, width);

            // Create a new cube and add it to the list
            let cube = Cube::new(point, max_corner, material.clone());
            cubes.push(cube);
        }

        // Return the grid containing all the cubes
        Grid { cubes }
    }
}

// Usage example
fn main() {
    // Define the material for the cubes
    let material = Material {
        // Add material properties here
        diffuse: Color::new(3, 3, 3),
        specular: 0.0,
        albedo: [0.0, 0.0]
    };

    // Define the width of the cubes
    let cube_width = 1.0;

    // Define a list of corner points
    let corner_points = vec![
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        // Add more points as needed
    ];

    // Create the grid with the specified width and corner points
    let grid = Grid::new(cube_width, corner_points, material);

    // Now `grid.cubes` contains all the cubes with the specified corners and width
}
