use crate::configuration::subgraph::data_sources::sql::DialectEnum;

mod to_sql_value;

#[derive(Debug, Clone, PartialEq)]
pub enum SqlValue {
    String(String),
    Int(i32),
    Bool(bool),
    StringList(Vec<String>),
    IntList(Vec<i32>),
    BoolList(Vec<bool>),
    UUID(uuid::Uuid),
    UUIDList(Vec<uuid::Uuid>),
    DateTime(chrono::DateTime<chrono::Utc>),
    DateTimeList(Vec<chrono::DateTime<chrono::Utc>>),
    ObjectID(String),
    ObjectIDList(Vec<String>),
}

pub trait FromBson {
    fn to_sql_value(&self, dialect: Option<&DialectEnum>)
        -> Result<SqlValue, async_graphql::Error>;
}
