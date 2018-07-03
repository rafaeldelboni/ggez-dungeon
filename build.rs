extern crate spritesheet_generator;

use spritesheet_generator::spritesheet_generator::generate;
use spritesheet_generator::spritesheet_generator_config::SpritesheetGeneratorConfig;

fn main() {
    let config = SpritesheetGeneratorConfig {
        max_width: 500,
        max_height: 500,
        border_padding: 4,
        input_folder: "assets/".to_string(),
        output_folder: "resources/".to_string(),
        output_file_name: "atlas".to_string(),
    };
    println!("Generating spritesheets: {:?}", config);

    generate(config);
    println!("Generating done.");
}
