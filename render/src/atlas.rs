use std::path::Path;

use image::{RgbaImage, GenericImage};
use log::{debug, warn, error, info};

pub fn create_atlas<P: AsRef<Path>>(textures_path: P, tile_width: u32, tile_height: u32) -> RgbaImage {
    let Ok(dir) = std::fs::read_dir(textures_path.as_ref()) else {
        panic!("The directory `{}` does not exists.", textures_path.as_ref().display());
    };

    let dir_iter = dir.into_iter().filter_map(|entry| {
        match entry {
            Ok(entry) => Some(entry),
            Err(err) => {
                debug!("Failed to read a directory entry: {}", err);
                None
            }
        }
    });
    
    let texture_width = 512;
    let texture_height = 512;
    let mut buffer = RgbaImage::new(texture_width, texture_height);
    let mut id = 0;

    for entry in dir_iter {
        let path = entry.path();

        if !path.is_file() || path.extension().unwrap_or_default() != "png" {
            warn!("Skipping non-png file: {}", path.display());
            continue;
        }
        
        let image = match image::open(&path) {
            Ok(image) => image,
            Err(err) => {
                warn!("Failed to load texture {}: {}", path.display(), err);
                continue;
            }
        };

        if image.width() != tile_width || image.height() != tile_height {
            warn!("Skipping non-{}x{} texture: {}", tile_width, tile_height, path.display());
            continue;
        }

        let x = (id % (texture_width / tile_width)) * tile_width;
        let y = (id / (texture_width / tile_width)) * tile_height;

        if let Err(e) = buffer.copy_from(&image, x, y) {
            error!("Failed to write texture: {}", e);
            continue;
        }

        id += 1;
    }

    buffer.save("atlas.png").unwrap();

    info!("{} textures loaded.", id);
    info!("{}x{} texture atlas created.", texture_width, texture_height);
    buffer
}