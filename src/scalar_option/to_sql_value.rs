use std::str::FromStr;

use bson::Bson;
use log::{debug, error, trace};

use super::ScalarOption;
use crate::{configuration::subgraph::data_sources::sql::DialectEnum, data_sources::sql::SqlValue};

impl ScalarOption {
    pub fn to_sql_value(
        self,
        value: &Bson,
        dialect: Option<&DialectEnum>,
    ) -> Result<SqlValue, async_graphql::Error> {
        debug!(
            "Converting BSON To SQL Value Enum By Scalar Option: {:?}",
            self
        );
        let list = value.as_array().is_some();
        trace!("Is List: {:?}", list);

        let sql_value_enum = match self {
            ScalarOption::String => {
                if list {
                    let value = value.as_array();

                    // Check that all values in the array are strings
                    let is_strings_valid = value
                        .unwrap()
                        .iter()
                        .all(|x| x.as_str().is_some() || x.as_str().is_none());

                    if !is_strings_valid {
                        error!("String list contains non-string values.");
                        return Err(async_graphql::Error::new(
                            "String list contains non-string values.",
                        ));
                    }

                    let values = value
                        .unwrap()
                        .iter()
                        .map(|x| x.as_str().unwrap().to_string())
                        .collect();

                    SqlValue::StringList(values)
                } else {
                    let value = value.as_str();

                    if value.is_none() {
                        error!("Expected string value is not a string.");
                        return Err(async_graphql::Error::new(
                            "Expected string value is not a string.",
                        ));
                    }

                    SqlValue::String(value.unwrap().to_string())
                }
            }
            ScalarOption::Int => {
                if list {
                    let mut sql_value_enum = None;
                    let value = value.as_array();

                    // Check that all values in the array are i32
                    let is_ints_valid = value.unwrap().iter().all(|x| x.as_i32().is_some());

                    if is_ints_valid {
                        let values = value.unwrap().iter().map(|x| x.as_i32().unwrap()).collect();
                        sql_value_enum = Some(SqlValue::IntList(values));
                    } else {
                        // Check that all values in the array are i64
                        let is_ints_valid = value.unwrap().iter().all(|x| x.as_i64().is_some());

                        if is_ints_valid {
                            let values = value
                                .unwrap()
                                .iter()
                                .map(|x| x.as_i64().unwrap() as i32)
                                .collect();

                            sql_value_enum = Some(SqlValue::IntList(values));
                        }
                    }

                    if sql_value_enum.is_none() {
                        error!("Int list contains non-int values.");
                        return Err(async_graphql::Error::new(
                            "Int list contains non-int values.",
                        ));
                    }

                    sql_value_enum.unwrap()
                } else {
                    let value_i32 = value.as_i32();
                    if value_i32.is_some() {
                        SqlValue::Int(value_i32.unwrap())
                    } else {
                        let value = value.as_i64();
                        if value.is_none() {
                            error!("Expected int value is not an int.");
                            return Err(async_graphql::Error::new(
                                "Expected int value is not an int.",
                            ));
                        }
                        SqlValue::Int(value.unwrap() as i32)
                    }
                }
            }
            ScalarOption::Boolean => {
                if list {
                    let value = value.as_array();

                    // Check that all values in the array are bool
                    let is_bools_valid = value.unwrap().iter().all(|x| x.as_bool().is_some());

                    if !is_bools_valid {
                        error!("Boolean list contains non-boolean values.");
                        return Err(async_graphql::Error::new(
                            "Boolean list contains non-boolean values.",
                        ));
                    }

                    let values = value
                        .unwrap()
                        .iter()
                        .map(|x| x.as_bool().unwrap())
                        .collect();

                    SqlValue::BoolList(values)
                } else {
                    let value = value.as_bool();
                    if value.is_none() {
                        error!("Expected boolean value is not a boolean.");
                        return Err(async_graphql::Error::new(
                            "Expected boolean value is not a boolean.",
                        ));
                    }
                    SqlValue::Bool(value.unwrap())
                }
            }
            ScalarOption::UUID => {
                if list {
                    let value = value.as_array();

                    // Check that all values in the array are uuids
                    let is_uuids_valid = value.unwrap().iter().all(|x| {
                        let x = x.as_str().unwrap_or("");
                        uuid::Uuid::parse_str(x).is_ok()
                    });

                    if !is_uuids_valid {
                        error!("UUID list contains non-uuid values.");
                        return Err(async_graphql::Error::new(
                            "UUID list contains non-uuid values.",
                        ));
                    }

                    let values = value
                        .unwrap()
                        .iter()
                        .map(|x| {
                            let x = x.as_str().unwrap_or("");
                            let uuid = uuid::Uuid::parse_str(x);
                            uuid.unwrap()
                        })
                        .collect();

                    SqlValue::UUIDList(values)
                } else {
                    let mut sql_value_enum = None;
                    let string_value = value.as_str();
                    if string_value.is_none() {
                        error!("Expected UUID value is not a string.");
                        return Err(async_graphql::Error::new(
                            "Expected UUID value is not a string.",
                        ));
                    }

                    if dialect.is_some() {
                        let dialect = dialect.unwrap();
                        match dialect {
                            DialectEnum::SQLITE | DialectEnum::MYSQL => {
                                sql_value_enum =
                                    Some(SqlValue::String(string_value.unwrap().to_string()));
                            }
                            _ => {}
                        }
                    };

                    if sql_value_enum.is_none() {
                        let value = uuid::Uuid::parse_str(string_value.unwrap())
                            .map_err(|_| async_graphql::Error::new("Invalid UUID"))?;
                        sql_value_enum = Some(SqlValue::UUID(value));
                    }

                    sql_value_enum.unwrap()
                }
            }
            ScalarOption::DateTime => {
                if list {
                    let value = value.as_array();

                    // Check that all values are valid date times
                    let is_dates_valid = value.unwrap().iter().all(|x| {
                        let x = x.as_str().unwrap_or("");
                        let date_time = chrono::DateTime::<chrono::Utc>::from_str(x);
                        date_time.is_ok()
                    });

                    if !is_dates_valid {
                        error!("DateTime list contains non-DateTime values.");
                        return Err(async_graphql::Error::new(
                            "Invalid DateTime String in Vector",
                        ));
                    }

                    let values = value
                        .unwrap()
                        .iter()
                        .map(|x| {
                            let x = x.as_str().unwrap_or("");
                            let date_time = chrono::DateTime::from_str(x);
                            date_time.unwrap()
                        })
                        .collect();

                    SqlValue::DateTimeList(values)
                } else {
                    let date_time_string = match value.as_str() {
                        Some(dt) => dt,
                        None => {
                            error!("Invalid DateTime String");
                            return Err(async_graphql::Error::new("Invalid DateTime String"));
                        }
                    };

                    let date_time = match chrono::DateTime::from_str(date_time_string) {
                        Ok(dt) => dt,
                        Err(e) => {
                            error!("Failed to parse DateTime: {}", e);
                            return Err(async_graphql::Error::new("Failed to parse DateTime"));
                        }
                    };

                    SqlValue::DateTime(date_time)
                }
            }
            ScalarOption::ObjectID => {
                if list {
                    let value = value.as_array();

                    // If every value in the array is a valid ObjectID, then push it to the list
                    let is_object_ids_valid = value.unwrap().iter().all(|x| {
                        let x = x.as_str().unwrap_or("");
                        let object_id = bson::oid::ObjectId::from_str(x);
                        object_id.is_ok()
                    });

                    if !is_object_ids_valid {
                        error!("ObjectID list contains non-ObjectID values.");
                        return Err(async_graphql::Error::new(
                            "Invalid ObjectID String in Vector",
                        ));
                    }

                    let values = value
                        .unwrap()
                        .iter()
                        .map(|x| {
                            let x = x.as_str().unwrap_or("");
                            let object_id = bson::oid::ObjectId::from_str(x);
                            object_id.unwrap().to_string()
                        })
                        .collect();

                    SqlValue::ObjectIDList(values)
                } else {
                    let string_value = value.as_str().unwrap_or("");
                    let value = bson::oid::ObjectId::from_str(string_value);
                    if value.is_err() {
                        error!("Invalid ObjectID String");
                        return Err(async_graphql::Error::new("Invalid ObjectID String"));
                    }
                    SqlValue::ObjectID(value.unwrap().to_string())
                }
            }
            _ => {
                error!("This scalar option cannot be converted to a SQL Value.");
                return Err(async_graphql::Error::new(
                    "This scalar option cannot be converted to a SQL Value.",
                ));
            }
        };

        trace!("SQL Value: {:?}", sql_value_enum);
        Ok(sql_value_enum)
    }
}
