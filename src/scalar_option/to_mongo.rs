use log::{debug, error, trace};
use std::str::FromStr;

use super::ScalarOption;
use bson::{oid::ObjectId, Bson};

#[derive(Debug, Clone)]
pub enum MongoValue {
    String(String),
    Int(i64),
    Boolean(bool),
    ObjectID(ObjectId),
    Object(serde_json::Value),
    UUID(uuid::Uuid),
    DateTime(chrono::DateTime<chrono::Utc>),
}

impl ScalarOption {
    /// Convert a bson value to a mongo value.
    /// Returns none if the value does not need to be converted.
    pub fn bson_to_mongo_value(
        &self,
        value: &Bson,
    ) -> Result<Option<MongoValue>, async_graphql::Error> {
        debug!("Converting {:?} to mongo value", self);

        let value = match self {
            ScalarOption::ObjectID => {
                // if the is a string, convert it to an object id.
                if let bson::Bson::String(object_id_string) = value {
                    let object_id = ObjectId::from_str(&object_id_string).map_err(|e| {
                        error!("Failed to convert string to object id. Error: {:?}", e);
                        async_graphql::Error::new(format!(
                            "Failed to convert string to object id. Error: {:?}",
                            e
                        ))
                    })?;
                    Some(MongoValue::ObjectID(object_id))
                } else if let bson::Bson::ObjectId(object_id) = value {
                    Some(MongoValue::ObjectID(object_id.clone()))
                } else {
                    error!("Failed to convert {:?} to object id", value);
                    return Err(async_graphql::Error::new(format!(
                        "Failed to convert {:?} to object id",
                        value
                    )));
                }
            }
            ScalarOption::DateTime => {
                // if the is a string, convert it to a date time so that it is saved the
                // right way inside the mongo db.
                if let bson::Bson::String(date_time_string) = value {
                    trace!("Converting string to date time: {}", date_time_string);
                    let date_time = chrono::DateTime::<chrono::Utc>::from_str(&date_time_string)
                        .map_err(|e| {
                            error!("Failed to convert string to date time. Error: {:?}", e);
                            async_graphql::Error::new(format!(
                                "Failed to convert string to date time. Error: {:?}",
                                e
                            ))
                        })?;
                    Some(MongoValue::DateTime(date_time))
                } else if let bson::Bson::DateTime(date_time) = value {
                    trace!("Converting date time to date time: {:?}", date_time);
                    let datetime = chrono::DateTime::<chrono::Utc>::from(*date_time);
                    Some(MongoValue::DateTime(datetime))
                } else {
                    error!("Failed to convert {:?} to date time", value);
                    return Err(async_graphql::Error::new(format!(
                        "Failed to convert {:?} to date time",
                        value
                    )));
                }
            }
            _ => None,
        };

        trace!("Mongo Value: {:?}", value);

        Ok(value)
    }
}
