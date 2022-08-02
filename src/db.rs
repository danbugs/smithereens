use std::env;

use anyhow::Result;
use diesel::{PgConnection, Connection};

pub const PIDGTM_DATABASE_URL_ENVVAR_NAME: &str = "PIDGTM_DATABASE_URL";

pub fn connect() -> Result<PgConnection> {
    Ok(PgConnection::establish(&env::var(PIDGTM_DATABASE_URL_ENVVAR_NAME)?)?)
}
