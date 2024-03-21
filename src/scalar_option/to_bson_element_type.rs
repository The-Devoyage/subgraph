use bson::spec::ElementType;
use log::debug;

use super::ScalarOption;

impl ScalarOption {
    /// Convert self to BSON Element Type.
    pub fn to_bson_element_type(self) -> ElementType {
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
