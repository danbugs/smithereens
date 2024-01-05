// This normally wouldn't be needed since Rust 2018, but,
// due to the fact that conforming to it requires some maintainer effort,
// Diesel still hasn't done it. Plus, it seems that they won't conform to this
// new idiom until v2 (see this: https://gitter.im/diesel-rs/diesel/archives/2020/11/15).
#[macro_use]
extern crate diesel;

pub mod db_models;
pub mod schema;

use std::env;

use anyhow::Result;
use diesel::{Connection, ConnectionResult, PgConnection};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub const PIDGTM_DATABASE_URL_ENVVAR_NAME: &str = "PIDGTM_DATABASE_URL";

pub fn init_logger() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

pub fn connect() -> Result<PgConnection> {
    // Try to connect forever until we succeed for a maximum of 100 (arbitraty) tries
    let tries = 100;
    for _ in 0..tries {
        match try_connect() {
            Ok(conn) => return Ok(conn),
            Err(e) => {
                tracing::error!("Failed to connect to database: {}", e);
                tracing::info!("Retrying in 5 seconds...");
                std::thread::sleep(std::time::Duration::from_secs(5));
            }
        }
    }

    Err(anyhow::anyhow!(
        "Failed to connect to database after 100 tries"
    ))
}

fn try_connect() -> ConnectionResult<PgConnection> {
    PgConnection::establish(
        &env::var(PIDGTM_DATABASE_URL_ENVVAR_NAME).unwrap_or_else(|_| {
            panic!(
                "{} environment variable not set",
                PIDGTM_DATABASE_URL_ENVVAR_NAME
            )
        }),
    )
}

#[cfg(test)]
mod tests {
    #![allow(unused)]    
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_connect() -> Result<()> {
        init_logger()?;
        assert!(connect().is_ok());

        Ok(())
    }
}
