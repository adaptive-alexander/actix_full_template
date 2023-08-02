//! Crate prelude

pub use crate::error::GenericError;
pub use crate::logging::*;
pub use crate::db::*;
pub use crate::config::*;
pub use std::format as f;

pub type Result<T> = core::result::Result<T, GenericError>;

pub struct W<T>(pub T);
