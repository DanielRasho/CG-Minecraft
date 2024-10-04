use once_cell::sync::Lazy;

use super::color::Color;
use std::sync::Arc;
use super::texture::Texture;

// use once_cell::sync::Lazy;
// use std::sync::Arc;

// I WANT TO DEFINE TEXTURE LIKE SO, so that are
static BOOK_SHELF: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/bookshelf.png")));
static CHEST: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/chest.png")));
static JUKEBOX: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/jukebox.png")));
static FURNACE: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/furnace.png")));
static CRAFTING_TABLE: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/crafting_table.png")));

static COBBLESTONE: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/cobblestone.png")));

static DARK_OAK_PLANKS: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/dark_oak_planks.png")));
static OAK_PLANKS: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/oak_planks.png")));
static OAK_LOG: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/oak_log.png")));

static GLASS: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/glass.png")));

static GLOWSTONE: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/glowstone.png")));

static GRASS: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/grass.png")));

#[derive(Debug, Clone)]
pub enum Diffuse {
    Color(Color),
    Texture(Arc<Texture>)
}

#[derive(Debug, Clone)]
pub struct Material {
    pub diffuse: Diffuse,
    pub specular: f32,
    pub albedo: [f32; 2],
    pub reflectivity: f32,
    pub transparency : f32,
    pub refractive_index : f32
}
impl Material {
    pub fn black() -> Material {
        Material {
            diffuse : Diffuse::Color(Color::new(0, 0, 0)),
            specular  : 0.0,
            albedo: [0.0, 0.0],
            reflectivity: 0.0,
            transparency : 0.0,
            refractive_index: 0.0
        }
    }
}
