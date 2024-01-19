use std::path::Path;

use image::{GenericImage, RgbaImage};
use log::{debug, error, warn};

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
    pub id: u32,
    pub name: String,
    pub rect: Rect,
}

pub struct BlockAtlas {
    pub texture: crate::render::Texture,
    pub tiles: Vec<AtlasRect>,
    pub size: vek::Extent2<u32>,
}

impl BlockAtlas {
    pub fn create<P: AsRef<Path>>(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        textures_path: P,
        tile_width: u32,
        tile_height: u32,
    ) -> std::io::Result<Self> {
        let texture_width = 512;
        let texture_height = 512;

        log::debug!(
            "Creating block atlas with dimensions {}x{}",
            texture_width,
            texture_height
        );

        let mut atlas_buffer = RgbaImage::new(texture_width, texture_height);
        let mut tiles = vec![];
        let mut id = 0;

        for entry in std::fs::read_dir(textures_path.as_ref())? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_file() || path.extension().unwrap_or_default() != "png" {
                warn!("Skipping non-png file: {}", path.display());
                continue;
            }

            let file_name = match path.file_stem() {
                Some(s) => s,
                None => {
                    warn!("Skipping file with no name: {}", path.display());
                    continue;
                },
            };

            let image = match image::open(&path) {
                Ok(image) => image,
                Err(err) => {
                    warn!("Failed to load texture {}: {}", path.display(), err);
                    continue;
                },
            };

            if image.width() != tile_width || image.height() != tile_height {
                warn!(
                    "Skipping non-{}x{} texture: {}",
                    tile_width,
                    tile_height,
                    path.display()
                );
                continue;
            }

            // coordinates of the tile in the texture, e.g (0, 0), (16, 0), (32, 0), ...
            let x = (id % (texture_width / tile_width)) * tile_width;
            let y = (id / (texture_width / tile_width)) * tile_height;
            // pixel coordinates are in the range [0, 1]
            let rect = Rect::new(
                x as f32 / texture_width as f32,
                y as f32 / texture_height as f32,
                tile_width as usize,
                tile_height as usize,
            );

            tiles.push(AtlasRect {
                id,
                name: file_name.to_string_lossy().into_owned(),
                rect,
            });

            if let Err(e) = atlas_buffer.copy_from(&image, x, y) {
                error!("Failed to write texture: {}", e);
                continue;
            }

            id += 1;
        }
        // buffer.save("atlas.png").unwrap();

        debug!("{} textures loaded.", id);

        let atlas_texture = crate::render::Texture::new(
            device,
            queue,
            image::DynamicImage::ImageRgba8(atlas_buffer),
        );

        Ok(Self {
            texture: atlas_texture,
            tiles,
            size: vek::Extent2::new(texture_width, texture_height),
        })
    }
}
