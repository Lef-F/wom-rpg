use std::{fs::File, io::Read};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Template {
    pub system_mode: String,
    pub introduction: String,
}

pub fn read_yaml_file(file_path: &str) -> Result<Template, Box<dyn std::error::Error>> {
    // Read the file content into a string
    let mut contents = String::new();
    let mut file = File::open(file_path)?;
    file.read_to_string(&mut contents)?;

    // Deserialize the YAML data into your Rust struct
    let data: Template = serde_yaml::from_str(&contents)?;

    Ok(data)
}
