extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

extern crate regex;
extern crate image;
extern crate texture_packer;

mod file_name;
mod serialize;

use std::fs;
use std::fs::File;

use texture_packer::texture::Texture;
use texture_packer::{TexturePacker, TexturePackerConfig};
use texture_packer::importer::ImageImporter;
use texture_packer::exporter::ImageExporter;


fn main() {
    //
    // Perform texture packing
    //
    let config = TexturePackerConfig {
        max_width: 400,
        max_height: 400,
        allow_rotation: false,
        texture_outlines: false,
        border_padding: 2,
        ..Default::default()
    };

    let mut packer = TexturePacker::new_skyline(config);

    if let Ok(entries) = fs::read_dir("assets/") {

        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name().into_string().unwrap();
                let file = file_name::extract(&file_name);
                let texture = ImageImporter::import_from_file(&entry.path())
                    .unwrap();

                packer.pack_own(file.name, texture);
            }
        }
    }

    //
    // Print the information
    // TODO: save this in a .json file
    //
    println!("Dimensions : {}x{}", packer.width(), packer.height());
    let atlas = serialize::to_atlas(packer.get_frames());
    let json = serialize::to_json(atlas);
    println!("{}", json.unwrap());

    //
    // Save the result
    //
    let exporter = ImageExporter::export(&packer).unwrap();
    let mut file = File::create("resources/skyline-packer-output.png").unwrap();
    exporter.write_to(&mut file, image::PNG).unwrap();
}
