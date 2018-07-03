extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

extern crate regex;
extern crate image;
extern crate texture_packer;

mod file_data;
mod file_texture;
mod serialize;

use texture_packer::{TexturePacker, TexturePackerConfig};
use texture_packer::exporter::ImageExporter;

use std::fs::File;

fn main() {
    // Initial setup
    let input_folder = "assets/".to_string();
    let output_folder = "resources/".to_string();
    let output_file_name = "skyline-packer-output".to_string();

    // Perform texture packing
    let config = TexturePackerConfig {
        max_width: 400,
        max_height: 400,
        allow_rotation: false,
        texture_outlines: false,
        border_padding: 2,
        ..Default::default()
    };
    let mut packer = TexturePacker::new_skyline(config);
    for file_textures in file_texture::find_all(input_folder) {
        packer.pack_own(file_textures.file.name, file_textures.texture);
    }

    // Save Json
    let atlas = serialize::to_atlas(packer.get_frames());
    let json_path = format!("{}{}.json", output_folder, output_file_name);
    let json_file = File::create(json_path).unwrap();
    serde_json::to_writer_pretty(json_file, &atlas).unwrap();

    // Save Image
    let exporter = ImageExporter::export(&packer).unwrap();
    let image_path = format!("{}{}.png", output_folder, output_file_name);
    let mut image_file = File::create(image_path).unwrap();
    exporter.write_to(&mut image_file, image::PNG).unwrap();
}
