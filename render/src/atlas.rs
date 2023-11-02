use std::{collections::HashMap, path::Path};

use common::block::BlockId;
use image::{GenericImage, RgbaImage};
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct AtlasTileTexture {
    id: u32,
    name: String,
    pixel_x: f32,
    pixel_y: f32,
    tile_width: u32,
    tile_height: u32,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub id: u32,
    pub name: String,
    pub textures: BlockTextures,
}

#[derive(Debug, Clone)]
pub struct BlockTextures {
    pub top: u32,
    pub side: u32,
    pub bottom: u32,
    // TODO: support using cardinal directions
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BlockParser {
    settings: SettingsParser,
    textures: BlockTexturesParser,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct SettingsParser {
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BlockTexturesParser {
    top: String,
    side: String,
    bottom: String,
}

pub type BlockMap = HashMap<BlockId, Block>;

pub fn load_blocks<P: AsRef<Path>>(path: P, block_atlas: Vec<AtlasTileTexture>) -> BlockMap {
    let Ok(dir) = std::fs::read_dir(path.as_ref()) else {
        panic!(
            "The directory `{}` does not exists.",
            path.as_ref().display()
        );
    };

    let mut map = BlockMap::new();

    for entry in dir.flatten() {
        info!("Loading block: {:?}", entry.path());
        let file = std::fs::read_to_string(entry.path()).expect("Failed to read file");
        let config = toml::from_str::<BlockParser>(&file).expect("Failed to parse file");
        let side = block_atlas
            .iter()
            .find(|t| t.name == config.textures.side)
            .expect("Failed to find side texture");

        let top = block_atlas
            .iter()
            .find(|t| t.name == config.textures.top)
            .expect("Failed to find top texture");

        let bottom = block_atlas
            .iter()
            .find(|t| t.name == config.textures.bottom)
            .expect("Failed to find bottom texture");
        let block_id = BlockId::from(config.settings.name.to_lowercase().as_str());
        let block = Block {
            id: block_id as u32,
            name: config.settings.name,
            textures: BlockTextures {
                top: top.id,
                side: side.id,
                bottom: bottom.id,
            },
        };
        info!("Loaded block: {:?}", block);
        map.insert(block_id, block);
    }
    info!("Loaded {} blocks", map.len());
    map
}

pub fn create_atlas<P: AsRef<Path>>(
    textures_path: P,
    tile_width: u32,
    tile_height: u32,
) -> (RgbaImage, Vec<AtlasTileTexture>) {
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

        let x = (id % (texture_width / tile_width)) * tile_width;
        let y = (id / (texture_width / tile_width)) * tile_height;

        tiles.push(AtlasTileTexture {
            id,
            name: file_name.to_string_lossy().into_owned(),
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
    (buffer, tiles)
}
