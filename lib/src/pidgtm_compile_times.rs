use anyhow::Result;
use diesel_async::RunQueryDsl;
use smithe_database::{
    db_models::pidgtm_compile_times::PidgtmCompileTimes, schema::pidgtm_compile_times::dsl::*,
};

pub async fn insert_pidgtm_compile_time(tis: i32) -> Result<()> {
    let new_pidgtm_compile_time = PidgtmCompileTimes::from(tis);
    let mut db_connection = smithe_database::connect().await?;
    diesel::insert_into(pidgtm_compile_times)
        .values(&new_pidgtm_compile_time)
        .execute(&mut db_connection).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;
    use diesel::prelude::*;

    #[tokio::test]
    #[cfg(feature = "skip_db_tests")]
    async fn test_insert_pidgtm_compile_time() -> Result<()> {
        // get count of all pidgtm_compile_times w/ -999 ti
        let mut db_connection = smithe_database::connect().await?;
        let pct = pidgtm_compile_times.filter(time_in_seconds.eq(-999));
        let count = pct.count().get_result::<i64>(&mut db_connection).await?;

        // insert pidgtm_compile_time
        let tis = -999;
        insert_pidgtm_compile_time(tis).await?;

        // get count again and check that it increased by 1
        let new_count = pct.count().get_result::<i64>(&mut db_connection).await?;
        assert_eq!(new_count, count + 1);

        // delete all pidgtm_compile_times w/ -999 ti
        diesel::delete(pct).execute(&mut db_connection).await?;

        Ok(())
    }
}
