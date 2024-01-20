use image::{GenericImage, RgbaImage};

use super::texture::Texture;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: usize,
    pub h: usize,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: usize, h: usize) -> Self {
        Self { x, y, w, h }
    }
}

#[derive(Debug)]
pub struct AtlasRect {
    pub id: u16,
    pub name: String,
    pub rect: Rect,
}

pub struct BlockAtlas {
    pub buffer: RgbaImage,
    pub tiles: Vec<AtlasRect>,
    pub size: vek::Extent2<u32>,
}

impl BlockAtlas {

    
    pub fn create(textures: &[String]) -> std::io::Result<Self> {
        let mut texture_data = Vec::new();
        let (mut last_width, mut last_height) = (0, 0);
        for path in textures {
            let image = match image::open(path) {
                Ok(image) => image,
                Err(e) => panic!("Failed to load texture: {}. Path: {}", e, path),
            };

            if last_width != 0 && last_height != 0 && (image.width() != last_width || image.height() != last_height) {
                panic!("All textures must be the same size");
            }

            last_width = image.width();
            last_height = image.height();

            texture_data.push(image);
        }

        let atlas_width = (textures.len() as f32).sqrt().ceil() as u32;
        let atlas_height = atlas_width;

        let mut atlas = RgbaImage::new(atlas_width * last_width, atlas_height * last_height);
        let mut tiles = vec![];

        for (i, image) in texture_data.iter().enumerate() {
            let x = (i as u32 % atlas_width) * last_width;
            let y = (i as u32 / atlas_width) * last_height;
            tiles.push(AtlasRect {
                id: i as u16,
                name: textures[i].clone(),
                rect: Rect::new(x as f32, y as f32, last_width as usize, last_height as usize),
            });
            atlas.copy_from(image, x, y).expect("Failed to copy texture to atlas");
        }

        atlas.save("atlas.png").expect("Failed to save atlas");
        Ok(Self {
            size: vek::Extent2::new(atlas_width * last_width, atlas_height * last_height),
            buffer: atlas,
            tiles,

        })
    }

    pub fn create_texture_handle(&self, device: &wgpu::Device, queue: &wgpu::Queue) -> Texture {
       Texture::new(device, queue, self.buffer.clone())
    } 
}
