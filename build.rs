use config_struct::StructOptions;

#[derive(Debug)]
enum Error {
    ConfigStruct(config_struct::Error),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Build script error")
    }
}

impl From<config_struct::Error> for Error {
    fn from(value: config_struct::Error) -> Self {
        Self::ConfigStruct(value)
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
