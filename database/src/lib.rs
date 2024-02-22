// This normally wouldn't be needed since Rust 2018, but,
// due to the fact that conforming to it requires some maintainer effort,
// Diesel still hasn't done it. Plus, it seems that they won't conform to this
// new idiom until v2 (see this: https://gitter.im/diesel-rs/diesel/archives/2020/11/15).
#[macro_use]
extern crate diesel;

pub mod db_models;
pub mod schema;

use std::{env, io::Write};

use anyhow::Result;
use diesel_async::{AsyncConnection, AsyncPgConnection};
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

pub async fn connect() -> Result<AsyncPgConnection> {
    // Try to connect forever until we succeed for a maximum of 100 (arbitraty) tries
    let tries = 100;
    for _ in 0..tries {
        match try_connect().await {
            Ok(conn) => return Ok(conn),
            Err(e) => {
                println!("Failed to connect to database: {}", e);
                std::io::stdout().flush().unwrap();
                tracing::error!("Failed to connect to database: {}", e);
                tracing::info!("Retrying in 5 seconds...");
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }
    }

    Err(anyhow::anyhow!(
        "Failed to connect to database after 100 tries"
    ))
}

async fn try_connect() -> Result<AsyncPgConnection> {
    println!("Connecting to database...");
    std::io::stdout().flush().unwrap();
    AsyncPgConnection::establish(
        &env::var(PIDGTM_DATABASE_URL_ENVVAR_NAME).unwrap_or_else(|_| {
            panic!(
                "{} environment variable not set",
                PIDGTM_DATABASE_URL_ENVVAR_NAME
            )
        }),
    )
    .await
    .map_err(|e| anyhow::anyhow!(e))
}

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn test_connect() -> Result<()> {
        init_logger()?;
        assert!(connect().await.is_ok());

        Ok(())
    }
}
