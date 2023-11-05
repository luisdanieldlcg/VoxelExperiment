use std::{collections::HashMap, path::Path};

use core::block::BlockId;
use log::info;
use render::atlas::AtlasRect;
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct BlockMap(pub HashMap<BlockId, Block>);

#[derive(Debug, Clone)]
pub struct Block {
    pub id: u32,
    pub name: String,
    pub textures: VoxelTextures,
}

#[derive(Debug, Clone)]
pub struct VoxelTextures {
    pub top: u32,
    pub side: u32,
    pub bottom: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BlockTexturesParser {
    top: String,
    side: String,
    bottom: String,
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

pub fn load_blocks<P: AsRef<Path>>(path: P, block_atlas: &[AtlasRect]) -> BlockMap {
    let Ok(dir) = std::fs::read_dir(path.as_ref()) else {
        panic!(
            "The directory `{}` does not exists.",
            path.as_ref().display()
        );
    };

    let mut map = BlockMap::default();

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
            textures: VoxelTextures {
                top: top.id,
                side: side.id,
                bottom: bottom.id,
            },
        };
        info!("Loaded block: {:?}", block);
        map.0.insert(block_id, block);
    }
    info!("Loaded {} blocks", map.0.len());
    map
}
