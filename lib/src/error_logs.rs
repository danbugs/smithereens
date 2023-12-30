use diesel::prelude::*;
use smithe_database::{schema::error_logs::dsl::*, db_models::error_logs::NewErrorLog};
use anyhow::Result;

pub fn insert_error_log(err: String) -> Result<()> {
    let new_error_log = NewErrorLog::new(err);
    let mut db_connection = smithe_database::connect()?;
    diesel::insert_into(error_logs).values(&new_error_log).execute(&mut db_connection)?;
    Ok(())
}