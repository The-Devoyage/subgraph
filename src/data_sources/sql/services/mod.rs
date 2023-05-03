use sqlx::{mysql::MySqlRow, postgres::PgRow, sqlite::SqliteRow};

pub mod create_one;
pub mod find_many;
pub mod find_one;
pub mod update_many;
pub mod update_one;

#[derive(Debug)]
pub struct Services;

pub enum ResponseRow {
    MySql(MySqlRow),
    Postgres(PgRow),
    SqLite(SqliteRow),
}
