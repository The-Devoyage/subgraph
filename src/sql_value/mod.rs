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
