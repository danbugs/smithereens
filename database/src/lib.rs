// This normally wouldn't be needed since Rust 2018, but,
// due to the fact that conforming to it requires some maintainer effort,
// Diesel still hasn't done it. Plus, it seems that they won't conform to this
// new idiom until v2 (see this: https://gitter.im/diesel-rs/diesel/archives/2020/11/15).
#[macro_use]
extern crate diesel;

pub mod schema;
pub mod db_models;

use std::env;

use anyhow::Result;
use diesel::{Connection, PgConnection};

pub const PIDGTM_DATABASE_URL_ENVVAR_NAME: &str = "PIDGTM_DATABASE_URL";

pub fn connect() -> Result<PgConnection> {
    Ok(PgConnection::establish(&env::var(
        PIDGTM_DATABASE_URL_ENVVAR_NAME,
    )?)?)
}
