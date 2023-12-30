use anyhow::Result;
use diesel::prelude::*;
use smithe_database::{db_models::pidgtm_compile_times::PidgtmCompileTimes, schema::pidgtm_compile_times::dsl::*};

pub fn insert_pidgtm_compile_time(tis: i32) -> Result<()> {
    let new_pidgtm_compile_time = PidgtmCompileTimes::from(tis);
    let mut db_connection = smithe_database::connect()?;
    diesel::insert_into(pidgtm_compile_times)
        .values(&new_pidgtm_compile_time)
        .execute(&mut db_connection)?;
    Ok(())
}