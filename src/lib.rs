// This normally wouldn't be needed since Rust 2018, but,
// due to the fact that conforming to it requires some maintainer effort,
// Diesel still hasn't done it. Plus, it seems that they won't conform to this
// new idiom until v2 (see this: https://gitter.im/diesel-rs/diesel/archives/2020/11/15).
#[macro_use]
extern crate diesel;

pub mod db;
pub mod db_models;
pub mod pidgtm_command_handlers;
pub mod queries;
pub mod schema;
pub mod smithe_command_handlers;
pub mod startgg;
