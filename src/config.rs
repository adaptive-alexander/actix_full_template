use crate::prelude::*;
use serde::Deserialize;

fn read_config() -> Result<Config> {
    let toml_s =
        std::fs::read_to_string("config.toml").expect("Missing config file config.toml in root.");
    match toml::from_str::<Config>(&toml_s) {
        Ok(c) => Ok(c),
        Err(e) => Err(GenericError::TomlError(e)),
    }
}

#[derive(Deserialize)]
struct Config {}
