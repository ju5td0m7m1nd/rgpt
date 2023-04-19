use dotenv_codegen::dotenv;
use serde_json::{from_str, Map, Value};
use std::env;
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    dotenv!("OPENAI_API_KEY");

    let openai_api_key = env!("OPENAI_API_KEY");
    let mut file = File::create("src/api_key.rs").unwrap();
    write!(file, "pub const API_KEY: &str = \"{}\";", openai_api_key).unwrap();

    let mut settings_file = File::open("settings.json").unwrap();
    let mut settings_json = String::new();
    settings_file.read_to_string(&mut settings_json);
    let settings: Map<String, Value> = from_str(&settings_json).unwrap();

    let mut module_file =
        File::create("src/settings.rs").expect("Failed to create src/settings.rs");
    module_file
        .write_all(
            format!(
                "pub const SETTINGS: &str = {:?};",
                settings_json.to_string()
            )
            .as_bytes(),
        )
        .expect("Failed to write to src/settings.rs");
}
