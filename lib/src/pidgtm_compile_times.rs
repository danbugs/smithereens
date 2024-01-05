use anyhow::Result;
use diesel::prelude::*;
use smithe_database::{
    db_models::pidgtm_compile_times::PidgtmCompileTimes, schema::pidgtm_compile_times::dsl::*,
};

pub fn insert_pidgtm_compile_time(tis: i32) -> Result<()> {
    let new_pidgtm_compile_time = PidgtmCompileTimes::from(tis);
    let mut db_connection = smithe_database::connect()?;
    diesel::insert_into(pidgtm_compile_times)
        .values(&new_pidgtm_compile_time)
        .execute(&mut db_connection)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_pidgtm_compile_time() -> Result<()> {
        // get count of all pidgtm_compile_times w/ 1 ti
        let mut db_connection = smithe_database::connect().unwrap();
        let pct = pidgtm_compile_times.filter(time_in_seconds.eq(1));
        let count = pct.count().get_result::<i64>(&mut db_connection).unwrap();

        // insert pidgtm_compile_time
        let tis = 1;
        insert_pidgtm_compile_time(tis)?;

        // get count again and check that it increased by 1
        let new_count = pct.count().get_result::<i64>(&mut db_connection).unwrap();
        assert_eq!(new_count, count + 1);

        // delete all pidgtm_compile_times w/ 1 ti
        diesel::delete(pct).execute(&mut db_connection)?;

        Ok(())
    }
}
