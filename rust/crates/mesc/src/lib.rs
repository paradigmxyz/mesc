//! MESC is a way for all of the tools on a system to share an rpc config

#![warn(missing_docs, unreachable_pub, unused_crate_dependencies)]
#![deny(unused_must_use, rust_2018_idioms)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]

/// directory matching chain_id's to network names
pub mod directory;
mod types;
mod validate;
pub use types::*;
mod interface;
/// load module
pub mod load;
/// metadata
pub mod metadata;
pub mod network_names;
pub use interface::*;
/// overrides module
pub mod overrides;
/// queries module
pub mod query;
/// write module
pub mod write;
