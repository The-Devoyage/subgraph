use std::str::FromStr;

use bson::Bson;
use log::{debug, error, trace};

use crate::configuration::subgraph::data_sources::sql::DialectEnum;

use super::{FromBson, SqlValue};

impl FromBson for Bson {
    fn to_sql_value(
        &self,
        dialect: Option<&DialectEnum>,
    ) -> Result<SqlValue, async_graphql::Error> {
        debug!("Converting Bson To SqlValue");
        let sql_value = match self {
            Bson::String(s) => {
                let mut value = SqlValue::String(s.clone());

                let is_uuid = uuid::Uuid::parse_str(s);
                if is_uuid.is_ok() {
                    value = match dialect {
                        Some(DialectEnum::POSTGRES) => SqlValue::UUID(is_uuid.unwrap()),
                        _ => SqlValue::String(s.clone()),
                    };
                }

                let is_date = chrono::DateTime::<chrono::Utc>::from_str(s);

                if is_date.is_ok() {
                    value = SqlValue::DateTime(is_date.unwrap());
                }

                value
            }
            Bson::Int32(i) => SqlValue::Int(*i),
            Bson::Int64(i) => SqlValue::Int(*i as i32),
            Bson::Boolean(b) => SqlValue::Bool(*b),
            Bson::ObjectId(o) => SqlValue::ObjectID(o.to_string()),
            Bson::DateTime(d) => SqlValue::DateTime(chrono::DateTime::<chrono::Utc>::from(*d)),
            Bson::Array(a) => {
                let mut sql_value = Vec::new();
                for v in a {
                    sql_value.push(v.to_sql_value(dialect)?);
                }
                // Match the type to get the correct SqlValue
                match sql_value[0] {
                    SqlValue::String(_) => SqlValue::StringList(
                        sql_value
                            .iter()
                            .map(|v| match v {
                                SqlValue::String(s) => s.clone(),
                                _ => panic!("Bson::to_sql_value: not supported"),
                            })
                            .collect(),
                    ),
                    SqlValue::Int(_) => SqlValue::IntList(
                        sql_value
                            .iter()
                            .map(|v| match v {
                                SqlValue::Int(i) => *i,
                                _ => panic!("Bson::to_sql_value: not supported"),
                            })
                            .collect(),
                    ),
                    SqlValue::Bool(_) => SqlValue::BoolList(
                        sql_value
                            .iter()
                            .map(|v| match v {
                                SqlValue::Bool(b) => *b,
                                _ => panic!("Bson::to_sql_value: not supported"),
                            })
                            .collect(),
                    ),
                    SqlValue::UUID(_) => SqlValue::UUIDList(
                        sql_value
                            .iter()
                            .map(|v| match v {
                                SqlValue::UUID(u) => u.clone(),
                                _ => panic!("Bson::to_sql_value: not supported"),
                            })
                            .collect(),
                    ),
                    SqlValue::DateTime(_) => SqlValue::DateTimeList(
                        sql_value
                            .iter()
                            .map(|v| match v {
                                SqlValue::DateTime(d) => d.clone(),
                                _ => panic!("Bson::to_sql_value: not supported"),
                            })
                            .collect(),
                    ),
                    SqlValue::ObjectID(_) => SqlValue::ObjectIDList(
                        sql_value
                            .iter()
                            .map(|v| match v {
                                SqlValue::ObjectID(o) => o.clone(),
                                _ => panic!("Bson::to_sql_value: not supported"),
                            })
                            .collect(),
                    ),
                    _ => {
                        error!("Bson::to_sql_value: not supported");
                        return Err(async_graphql::Error::new(
                            "Bson::to_sql_value: not supported",
                        ));
                    }
                }
            }
            _ => {
                error!("Bson::to_sql_value: not supported");
                return Err(async_graphql::Error::new(
                    "Bson::to_sql_value: not supported",
                ));
            }
        };

        trace!("Bson::to_sql_value: {:?}", sql_value);

        Ok(sql_value)
    }
}
