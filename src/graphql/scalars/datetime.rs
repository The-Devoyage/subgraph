use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
use mongodb::bson::{doc, DateTime as DT};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DateTime(DT);

#[Scalar]
impl ScalarType for DateTime {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::String(s) => Ok(DateTime(DT::parse_rfc3339_str(s).unwrap())),
            _ => Err(InputValueError::expected_type(value)),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_string())
    }
}
