use std::path::Path;

use image::{GenericImage, RgbaImage};
use log::{debug, error, info, warn};

#[derive(Debug)]
pub struct AtlasTileTexture {
    pub id: u32,
    pub name: String,
    pub pixel_x: f32,
    pub pixel_y: f32,
    pub tile_width: u32,
    pub tile_height: u32,
}

pub struct BlockAtlas {
    pub tiles: Vec<AtlasTileTexture>,
}

pub fn create_atlas<P: AsRef<Path>>(
    textures_path: P,
    tile_width: u32,
    tile_height: u32,
) -> (BlockAtlas, RgbaImage) {
    let Ok(dir) = std::fs::read_dir(textures_path.as_ref()) else {
        panic!(
            "The directory `{}` does not exists.",
            textures_path.as_ref().display()
        );
    };

    let dir_iter = dir.into_iter().filter_map(|entry| match entry {
        Ok(entry) => Some(entry),
        Err(err) => {
            debug!("Failed to read a directory entry: {}", err);
            None
        },
    });

    let texture_width = 512;
    let texture_height = 512;
    let mut buffer = RgbaImage::new(texture_width, texture_height);
    let mut tiles = vec![];
    let mut id = 0;

    for entry in dir_iter {
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

        tiles.push(AtlasTileTexture {
            id,
            name: file_name.to_string_lossy().into_owned(),
            // pixel coordinates are in the range [0, 1]
            pixel_x: x as f32 / texture_width as f32,
            pixel_y: y as f32 / texture_height as f32,
            tile_width,
            tile_height,
        });

        if let Err(e) = buffer.copy_from(&image, x, y) {
            error!("Failed to write texture: {}", e);
            continue;
        }

        id += 1;
    }
    // buffer.save("atlas.png").unwrap();

    info!("{} textures loaded.", id);
    info!(
        "{}x{} texture atlas created.",
        texture_width, texture_height
    );
    (BlockAtlas { tiles }, buffer)
}
