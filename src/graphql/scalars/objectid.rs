use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
use mongodb::bson::{doc, oid::ObjectId as OID};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ObjectId(OID);

#[Scalar]
impl ScalarType for ObjectId {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = &value {
            Ok(value.parse().map(ObjectId)?)
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_string())
    }
}
