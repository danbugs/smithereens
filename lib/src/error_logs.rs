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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_error_log() -> Result<()> {
        // get count of all error_logs w/ "test error" message
        let mut db_connection = smithe_database::connect().unwrap();
        let err_logs = error_logs.filter(error_message.eq("test error"));
        let count = err_logs.count().get_result::<i64>(&mut db_connection).unwrap();

        // insert error
        let err = "test error".to_string();
        insert_error_log(err.clone())?;

        // get count again and check that it increased by 1
        let new_count = err_logs.count().get_result::<i64>(&mut db_connection).unwrap();
        assert_eq!(new_count, count + 1);
        
        // delete all error_logs w/ "test error" message
        diesel::delete(err_logs).execute(&mut db_connection)?;

        Ok(())
    }
}