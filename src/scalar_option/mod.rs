use bson::spec::ElementType;
use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ScalarOption {
    String,
    Int,
    Boolean,
    ObjectID,
    Object,
    UUID,
    DateTime,
}

impl ScalarOption {
    pub fn to_bson_type(self) -> ElementType {
        debug!("Converting Scalar To BSON Element Type: {:?}", self);
        match self {
            ScalarOption::String => ElementType::String,
            ScalarOption::Int => ElementType::Int32,
            ScalarOption::Boolean => ElementType::Boolean,
            ScalarOption::ObjectID => ElementType::ObjectId,
            ScalarOption::Object => ElementType::EmbeddedDocument,
            ScalarOption::UUID => ElementType::String,
            ScalarOption::DateTime => ElementType::DateTime,
        }
    }
}
