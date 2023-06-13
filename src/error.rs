use thiserror::Error;

#[derive(Error, Debug)]
pub enum GenericError {
    #[error("StaticError")]
    StaticError(&'static str),
    #[error("TomlError")]
    TomlError(#[from] toml::de::Error),
}
