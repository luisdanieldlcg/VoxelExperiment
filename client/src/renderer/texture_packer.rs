use std::{fs, path::Path};

use tracing::info;

use crate::png_utils;

/// Creates a texture atlas from the textures in the given directory.
/// 
/// # Limitations
/// - All textures must be of the same size.
pub fn pack_textures<P: AsRef<Path>>(resource: P) {

    let paths = fs::read_dir(resource)
        .unwrap()
        .map(|x| x.map(|x| x.path()))
        // filter out anything that does not contain a png
        .filter(|x| {
            x.as_ref()
                .unwrap()
                .extension()
                .map(|x| x == "png")
                .unwrap_or(false)
        })        
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    
    // the number of tiles per row/column
    let atlas_tile_count = (paths.len() as f32).sqrt().ceil() as usize;

    // I need to know what the size of each individual tile is.
    // I can get this from the first texture, assuming they are all the same size.
    let first_image = png_utils::read(&paths[0]).unwrap();
    let atlas_width = first_image.width as usize * atlas_tile_count;
    let atlas_height = first_image.height as usize * atlas_tile_count;
    let mut pixels = vec![0; atlas_width * atlas_height  * 4];

    tracing::info!("Atlas size: {}x{}", atlas_width, atlas_height);

    for (i, path) in paths.iter().enumerate() {
        if path.is_dir() {
            continue; // skip just for now
        }

        let Ok(image) = png_utils::read(path) else {
            tracing::warn!("Failed to read texture at {}", path.display());
            continue;
        };

        info!(id=i, path=%path.display(), "Packing texture");
    
        let pixel_x = i % atlas_tile_count * image.width as usize;
        let pixel_y = i / atlas_tile_count * image.height as usize;

        for row in 0..atlas_width {
            for col in 0..atlas_height {
                // let atlas_index = (row + pixel_x) * 4 + (col + pixel_y) * atlas_width * 4;
                // let image_index = row * 4 + col * image.width as usize * 4;

                // pixels[atlas_index] = image.pixels[image_index];
                // pixels[atlas_index + 1] = image.pixels[image_index + 1];
                // pixels[atlas_index + 2] = image.pixels[image_index + 2];
                // pixels[atlas_index + 3] = image.pixels[image_index + 3];
            }
        }
    }

    png_utils::write("assets/atlas.png", &pixels, atlas_width as u32, atlas_height as u32).unwrap();
}
