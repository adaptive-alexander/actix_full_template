//! Crate prelude

pub use crate::error::GenericError;
pub use std::format as f;

pub type Result<T> = core::result::Result<T, GenericError>;

pub struct W<T>(pub T);
