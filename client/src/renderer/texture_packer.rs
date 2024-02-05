use std::{fs, path::Path};

use crate::png_utils::{self, PngImage};

pub struct Atlas {
    pub tile_size: u32,
    pub image: PngImage,
}

impl Atlas {
    /// Creates a texture atlas from the textures in the given directory.
    ///
    /// # Limitations
    /// - All textures must be of the same size.
    /// - Subdirectories are being ignored.
    pub fn pack_textures<P: AsRef<Path>>(resource: P) -> Self {
        let paths = fs::read_dir(&resource)
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

        if paths.is_empty() {
            panic!("No textures found in {}", resource.as_ref().display());
        }
        // the number of tiles per row/column
        let atlas_tile_count = (paths.len() as f32).sqrt().ceil() as usize;
        log::info!("Atlas tiles per row/col: {}", atlas_tile_count);

        // I need to know what the size of each individual tile is.
        // I can get this from the first texture, assuming they are all the same size.
        let first_image = png_utils::read(&paths[0]).unwrap();
        let atlas_width = first_image.width as usize * atlas_tile_count;
        let atlas_height = first_image.height as usize * atlas_tile_count;
        let mut pixels = vec![0; atlas_width * atlas_height * 4];

        log::info!("Atlas size: {}x{}", atlas_width, atlas_height);

        for (i, path) in paths.iter().enumerate() {
            if path.is_dir() {
                continue; // skip just for now
            }
            let Ok(image) = png_utils::read(path) else {
                log::warn!("Failed to read texture at {}", path.display());
                continue;
            };

            if image.width != first_image.width || image.height != first_image.height {
                log::warn!(
                    "Found texture with invalid size: {}x{} (expected {}x{})",
                    image.width,
                    image.height,
                    first_image.width,
                    first_image.height
                );
                continue;
            }
            log::info!(
                "Packing texture into atlas... id={} path={}",
                i,
                path.display()
            );

            let pixel_x = i % atlas_tile_count * image.width as usize;
            let pixel_y = i / atlas_tile_count * image.height as usize;

            for y in 0..image.height as usize {
                for x in 0..image.width as usize {
                    let pixel =
                        png_utils::get_pixel(&image.pixels, image.width, x as u32, y as u32);
                    png_utils::set_pixel(
                        &mut pixels,
                        atlas_width as u32,
                        pixel_x as u32 + x as u32,
                        pixel_y as u32 + y as u32,
                        pixel,
                    );
                }
            }
        }
        // TODO: temporal
        png_utils::write(
            "assets/atlas.png",
            &pixels,
            atlas_width as u32,
            atlas_height as u32,
        )
        .unwrap();

        Self {
            tile_size: first_image.width,
            image: PngImage {
                width: atlas_width as u32,
                height: atlas_height as u32,
                pixels,
            },
        }
    }
}
