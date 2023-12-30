use crate::schema::pidgtm_compile_times;
use diesel::prelude::*;

#[derive(Debug, Insertable, Queryable, QueryableByName)]
#[diesel(table_name = pidgtm_compile_times)]
pub struct PidgtmCompileTimes {
    pub time_in_seconds: i32,
    pub calculation_timestamp: chrono::NaiveDateTime, // Using chrono crate for timestamp handling
}

impl From<i32> for PidgtmCompileTimes {
    fn from(time_in_seconds: i32) -> Self {
        Self { 
            time_in_seconds,
            calculation_timestamp: chrono::Utc::now().naive_utc(), // Current timestamp
        }
    }
}
