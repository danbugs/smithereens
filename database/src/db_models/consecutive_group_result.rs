use diesel::sql_types::*;

#[derive(Debug, QueryableByName)]
pub struct ConsecutiveGroupResult {
    #[sql_type = "BigInt"]
    pub grp: i64,
    #[sql_type = "BigInt"]
    pub consecutive_count: i64,
}
