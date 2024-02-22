use anyhow::Result;
use diesel_async::RunQueryDsl;
use smithe_database::{db_models::error_logs::NewErrorLog, schema::error_logs::dsl::*};

pub async fn insert_error_log(err: String) -> Result<()> {
    let new_error_log = NewErrorLog::new(err);
    let mut db_connection = smithe_database::connect().await?;
    diesel::insert_into(error_logs)
        .values(&new_error_log)
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
    async fn test_insert_error_log() -> Result<()> {
        // get count of all error_logs w/ "test error" message
        let mut db_connection = smithe_database::connect().await?;
        let err_logs = error_logs.filter(error_message.eq("test error"));
        let count = err_logs
            .count()
            .get_result::<i64>(&mut db_connection)
            .await?;

        // insert error
        let err = "test error".to_string();
        insert_error_log(err.clone()).await?;

        // get count again and check that it increased by 1
        let new_count = err_logs
            .count()
            .get_result::<i64>(&mut db_connection)
            .await?;
        assert_eq!(new_count, count + 1);

        // delete all error_logs w/ "test error" message
        diesel::delete(err_logs).execute(&mut db_connection).await?;

        Ok(())
    }
}
