use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use common::block::BlockId;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockDescriptor {
    pub name: String,
    pub textures: Textures,
}

impl BlockDescriptor {
    pub fn textures(&self) -> (&String, &String, &String) {
        // if all is defined, then use it for all sides
        match &self.textures.all {
            Some(all) => (all, all, all),
            None => (
                self.textures.top.as_ref().unwrap(),
                self.textures.side.as_ref().unwrap(),
                self.textures.bottom.as_ref().unwrap(),
            ),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Textures {
    all: Option<String>,
    top: Option<String>,
    bottom: Option<String>,
    side: Option<String>,
}

pub struct BlockMap {
    blocks: HashMap<BlockId, BlockDescriptor>,
    textures: Vec<String>,
}

impl BlockMap {
    pub fn load_blocks<P: AsRef<Path>>(blocks: P, textures: P) -> Self {
        let Ok(dir) = std::fs::read_dir(&blocks) else {
            panic!(
                "The directory `{}` does not exists.",
                blocks.as_ref().display()
            );
        };
        let mut registry = HashMap::new();
        let mut texture_list = HashSet::new();
        for entry in dir.flatten() {
            info!("Loading block: {:?}", entry.path());
            let file = match std::fs::read_to_string(entry.path()) {
                Ok(file) => file,
                Err(e) => {
                    log::error!("Failed to read file: {}", e);
                    continue;
                },
            };

            let config = toml::from_str::<BlockDescriptor>(&file).expect("Failed to parse file");
            let path = textures.as_ref().to_str().unwrap();
            match &config.textures.all {
                Some(all) => {
                    texture_list.insert(format!("{}/{}.png", path, all));
                },
                None => {
                    if let Some(top) = &config.textures.top {
                        texture_list.insert(format!("{}/{}.png", path, top));
                    }

                    if let Some(bottom) = &config.textures.bottom {
                        texture_list.insert(format!("{}/{}.png", path, bottom));
                    }

                    if let Some(side) = &config.textures.side {
                        texture_list.insert(format!("{}/{}.png", path, side));
                    }
                },
            }
            registry.insert(BlockId::from(config.name.as_str()), config);
        }

        Self {
            blocks: registry,
            textures: texture_list.into_iter().collect(),
        }
    }

    pub fn get(&self, id: BlockId) -> Option<&BlockDescriptor> {
        self.blocks.get(&id)
    }

    pub fn textures(&self) -> &[String] {
        &self.textures
    }
}
