use once_cell::sync::Lazy;
use std::sync::Arc;
use image::{ImageReader, Pixel, DynamicImage, GenericImageView};
use std::fmt;
use super::color::Color;

pub static BOOK_SHELF: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/bookshelf.png")));
pub static CHEST: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/chest.png")));
pub static JUKEBOX: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/jukebox.png")));
pub static FURNACE: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/furnace.png")));
pub static CRAFTING_TABLE: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/crafting_table.png")));

pub static COBBLESTONE: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/cobblestone.png")));

pub static DARK_OAK_PLANKS: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/dark_oak_planks.png")));
pub static OAK_PLANKS: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/oak_planks.png")));
pub static OAK_LOG: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/oak_log.png")));

pub static GLASS: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/glass.png")));

pub static GLOWSTONE: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/glowstone.png")));

pub static GRASS: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/grass.png")));

#[derive(Clone)]
pub struct Texture {
  image: DynamicImage,
  pub width: usize,
  pub height: usize,
  color_array: Vec<Color>,
}

impl Texture {
  pub fn new(file_path: &str) -> Texture {
    let img = match ImageReader::open(file_path) {
        Ok(reader) => match reader.decode() {
            Ok(image) => image,
            Err(e) => {
                println!("Error decoding image: {}", e);
                return Texture::black(); // Usar una textura negra como respaldo
            }
        },
        Err(e) => {
            println!("Error opening image file: {}", e);
            return Texture::black(); // Usar una textura negra como respaldo
        }
    };
    let width = img.width() as usize;
    let height = img.height() as usize;
    let mut texture = Texture {
      image: img,
      width,
      height,
      color_array: vec![Color::new(0,0,0); width * height],
    };
    texture.load_color_array();
    texture
  }

  fn load_color_array(&mut self) {
    for x in 0..self.width {
        for y in 0..self.height {
            let pixel = self.image.get_pixel(x as u32, y as u32).to_rgb();
            let color = ((pixel[0] as u32) << 16) | ((pixel[1] as u32) << 8) | (pixel[2] as u32);
            self.color_array[y * self.width + x] = Color::from_hex(color);
        }
    }
}

  pub fn get_color(&self, x: usize, y: usize) -> Color {
    if x >= self.width || y >= self.height {
      Color::from_hex(0xFF00FF)
    } else {
      self.color_array[y * self.width + x]
    }
  }

  pub fn black() -> Texture {
    let width = 1; // Ancho de 1 píxel
    let height = 1; // Alto de 1 píxel
    let mut texture = Texture {
        image: DynamicImage::new_rgb8(width as u32, height as u32),
        width,
        height,
        color_array: vec![Color::new(0, 0, 0); width * height], // Colores negros
    };
    texture.load_color_array(); // Cargar el color negro
    texture
}
}

impl fmt::Debug for Texture {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Texture")
      .field("width", &self.width)
      .field("height", &self.height)
      .finish()
  }
}
