mod tests;
mod error;
mod config;
mod prelude;
pub mod logging;

pub fn sync_test(_inp: &str){}

#[tracing::instrument(
    name = "Async test subscriber",
)]
pub async fn async_test(_inp: usize){}
