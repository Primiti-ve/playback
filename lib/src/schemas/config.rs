use std::error::Error;
use playback_toml;

#[derive(Debug)]
pub struct ConfigSchema {}

pub fn decode_config(content: &str) -> Result<ConfigSchema, Box<dyn Error>> {
    println!("parsed: {:?}", playback_toml::parse(content));

    Ok(ConfigSchema {})
}
