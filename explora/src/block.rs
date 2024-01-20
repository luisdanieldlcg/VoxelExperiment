use std::{collections::HashMap, path::Path};

use common::block::BlockId;
use log::info;
use serde::{Deserialize, Serialize};

pub struct BlockMap {
    blocks: HashMap<BlockId, BlockDescriptor>,
    textures_path: String,
}

impl BlockMap {

    pub fn load_blocks<P: AsRef<Path>>(blocks: P, textures: P) -> Self {
        let Ok(dir) = std::fs::read_dir(&blocks) else {
            panic!(
                "The directory `{}` does not exists.",
                blocks.as_ref().display()
            );
        };
        let mut registry: HashMap<BlockId, BlockDescriptor> = HashMap::new();
        for entry in dir.flatten() {
            info!("Loading block: {:?}", entry.path());
            let file = match std::fs::read_to_string(entry.path()) {
                Ok(file) => file,
                Err(e) => {
                    log::error!("Failed to read file: {}", e);
                    continue;
                }
            };
            let config = toml::from_str::<BlockDescriptor>(&file).expect("Failed to parse file");
            registry.insert(BlockId::from(config.name.as_str()), config);
        }

        let registry = dbg!(registry);
        Self {
            blocks: registry,
            textures_path: textures.as_ref().to_str().unwrap().to_string(),
        }
    }

    pub fn get(&self, id: BlockId) -> Option<&BlockDescriptor> {
        self.blocks.get(&id)
    }

    pub fn texture_list(&self) -> Vec<String> {

        // also eliminate duplicates
        let mut textures = vec![];
        for (_, block) in self.blocks.iter() {
            if let Some(all) = &block.textures.all {
                textures.push(format!("{}/{}.png", self.textures_path, all));
            }
            if let Some(top) = &block.textures.top {
                textures.push(format!("{}/{}.png", self.textures_path, top));
            }
            if let Some(bottom) = &block.textures.bottom {
                textures.push(format!("{}/{}.png",self.textures_path, bottom));
            }
            if let Some(side) = &block.textures.side {
                textures.push(format!("{}/{}.png",self.textures_path, side));
            }
        }

        textures.sort();
        textures.dedup();
        textures
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockDescriptor {
    name: String,
    textures: Textures,
}

#[derive(Debug, Serialize, Deserialize)]
struct Textures {
    all: Option<String>,
    top: Option<String>,
    bottom: Option<String>,
    side: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub id: u32,
    pub name: String,
    pub textures: VoxelTextures,
}

#[derive(Debug, Clone)]
pub struct VoxelTextures {
    pub top: u16,
    pub side: u16,
    pub bottom: u16,
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

// pub fn load_blocks<P: AsRef<Path>>(path: P, block_atlas: &[AtlasRect]) -> BlockMap {
//     let Ok(dir) = std::fs::read_dir(path.as_ref()) else {
//         panic!(
//             "The directory `{}` does not exists.",
//             path.as_ref().display()
//         );
//     };

//     let mut map = BlockMap::default();

//     for entry in dir.flatten() {
//         info!("Loading block: {:?}", entry.path());
//         let file = std::fs::read_to_string(entry.path()).expect("Failed to read file");
//         let config = toml::from_str::<BlockParser>(&file).expect("Failed to parse file");
//         let side = block_atlas
//             .iter()
//             .find(|t| t.name == config.textures.side)
//             .expect("Failed to find side texture");

//         let top = block_atlas
//             .iter()
//             .find(|t| t.name == config.textures.top)
//             .expect("Failed to find top texture");

//         let bottom = block_atlas
//             .iter()
//             .find(|t| t.name == config.textures.bottom)
//             .expect("Failed to find bottom texture");

//         let block_id = BlockId::from(config.settings.name.to_lowercase().as_str());
//         let block = Block {
//             id: block_id as u32,
//             name: config.settings.name,
//             textures: VoxelTextures {
//                 top: top.id,
//                 side: side.id,
//                 bottom: bottom.id,
//             },
//         };
//         info!("Loaded block: {:?}", block);
//         map.0.insert(block_id, block);
//     }
//     info!("Loaded {} blocks", map.0.len());
//     map
// }
