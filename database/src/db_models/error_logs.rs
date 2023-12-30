use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::schema::error_logs;

#[derive(Debug, Insertable)]
#[diesel(table_name = error_logs)]
pub struct NewErrorLog {
    pub error_timestamp: NaiveDateTime,
    pub error_message: String,
}

impl NewErrorLog {
    pub fn new(error_message: String) -> Self {
        Self {
            error_timestamp: chrono::Utc::now().naive_utc(),
            error_message,
        }
    }
}
