mod config;
mod error;
mod logging;
pub mod prelude;
mod tests;

pub fn sync_test(_inp: &str) {}

#[tracing::instrument(name = "Async test subscriber")]
pub async fn async_test(_inp: usize) {}
