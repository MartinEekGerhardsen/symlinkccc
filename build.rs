use std::{clone, path::Path};

use config_struct::StructOptions;

#[derive(Debug)]
enum Error {
    ConfigStruct(config_struct::Error),
}

impl From<config_struct::Error> for Error {
    fn from(value: config_struct::Error) -> Self {
        Error::ConfigStruct(value)
    }
}

fn main() -> Result<(), Error> {
    config_struct::create_struct(
        "config/default.toml",
        "src/config.rs",
        &StructOptions::default(),
    )?;

    Ok(())
}
