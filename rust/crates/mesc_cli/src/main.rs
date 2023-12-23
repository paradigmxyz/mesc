#![allow(dead_code)]

mod cli;
mod metadata;
mod network;
mod printing;
mod rpc;
mod types;

pub(crate) use cli::*;
pub(crate) use printing::*;
use types::*;

#[tokio::main]
async fn main() -> Result<(), MescCliError> {
    cli::run_cli().await
}
