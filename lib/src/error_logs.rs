use anyhow::Result;
use diesel::prelude::*;
use smithe_database::{db_models::error_logs::NewErrorLog, schema::error_logs::dsl::*};

pub fn insert_error_log(err: String) -> Result<()> {
    let new_error_log = NewErrorLog::new(err);
    let mut db_connection = smithe_database::connect()?;
    diesel::insert_into(error_logs)
        .values(&new_error_log)
        .execute(&mut db_connection)?;
    Ok(())
}
