use diesel::sql_types::*;

#[derive(Debug, QueryableByName)]
pub struct ConsecutiveGroupResult {
    #[diesel(sql_type = BigInt)]
    pub grp: i64,
    #[diesel(sql_type = BigInt)]
    pub consecutive_count: i64,
}
