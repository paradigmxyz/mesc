//! MESC is a way for all of the tools on a system to share an rpc config

#![allow(dead_code)]
#![warn(missing_docs, unreachable_pub, unused_crate_dependencies)]
#![deny(unused_must_use, rust_2018_idioms)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]

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
