use std::collections::HashMap;

use image::{GenericImage, RgbaImage};

use super::texture::Texture;

pub struct BlockAtlas {
    pub buffer: RgbaImage,
    pub tiles: HashMap<String, u16>,
    pub tile_size: u32,
    pub atlas_size: u32,
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

            if last_width != 0
                && last_height != 0
                && (image.width() != last_width || image.height() != last_height)
            {
                panic!("All textures must be the same size");
            }

            last_width = image.width();
            last_height = image.height();

            texture_data.push(image);
        }

        let cols = (textures.len() as f32).sqrt().ceil() as u32;
        let rows = cols;

        let mut atlas = RgbaImage::new(cols * last_width, rows * last_height);
        let mut tiles = HashMap::new();

        // Write the atlas
        for (i, image) in texture_data.iter().enumerate() {
            let x = (i as u32 % cols) * last_width;
            let y = (i as u32 / rows) * last_height;

            let filename = textures[i]
                .split('/')
                .last()
                .unwrap()
                .split('.')
                .next()
                .unwrap();

            tiles.insert(filename.to_owned(), i as u16);

            atlas
                .copy_from(image, x, y)
                .expect("Failed to copy texture to atlas");
        }

        atlas.save("atlas.png").expect("Failed to save atlas");
        Ok(Self {
            tile_size: last_width,
            atlas_size: cols * last_width,
            buffer: atlas,
            tiles,
        })
    }

    pub fn create_texture_handle(&self, device: &wgpu::Device, queue: &wgpu::Queue) -> Texture {
        Texture::new(device, queue, self.buffer.clone())
    }

    pub fn get_texture_id(&self, texture: &str) -> u16 {
        match self.tiles.get(texture) {
            Some(id) => *id,
            None => panic!("Texture with name: {:?} not found. Make sure your texture is in assets/textures and is a png file", texture),
        }
    }
}
