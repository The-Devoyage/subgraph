use bson::spec::ElementType;
use log::{debug, error, trace};
use serde::{Deserialize, Serialize};

use crate::utils::clean_string::clean_string;

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
    /// Convert self to BSON Element Type.
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

    /// Convert JSON Value to EvalExpr Value based on the scalar type.
    pub fn to_evalexpr_type(
        self,
        value: &serde_json::Value,
    ) -> Result<evalexpr::Value, evalexpr::EvalexprError> {
        debug!("Converting Scalar To EvalExpr Type: {:?}", self);

        let value = match self {
            ScalarOption::String | ScalarOption::UUID => {
                evalexpr::Value::String(clean_string(&value.to_string(), None))
            }
            ScalarOption::Int => {
                let int_scalar = value
                    .as_i64()
                    .ok_or(evalexpr::EvalexprError::CustomMessage(
                        "Scalar is not supported in context.".to_string(),
                    ))?;
                evalexpr::Value::Int(int_scalar)
            }
            ScalarOption::Boolean => {
                let bool_scalar = value
                    .as_bool()
                    .ok_or(evalexpr::EvalexprError::CustomMessage(
                        "Scalar is not supported in context.".to_string(),
                    ))?;
                evalexpr::Value::Boolean(bool_scalar)
            }
            ScalarOption::DateTime => {
                let date_time = value
                    .as_str()
                    .ok_or(evalexpr::EvalexprError::CustomMessage(
                        "Scalar is not supported in context.".to_string(),
                    ))?;
                evalexpr::Value::String(date_time.to_string())
            }
            ScalarOption::ObjectID => {
                let object_id = value.get("$oid");
                if object_id.is_none() {
                    error!("ObjectID, `$oid`, not found.");
                    return Err(evalexpr::EvalexprError::CustomMessage(
                        "ObjectID not found.".to_string(),
                    ));
                }
                let object_id =
                    object_id
                        .unwrap()
                        .as_str()
                        .ok_or(evalexpr::EvalexprError::CustomMessage(
                            "ObjectID not found.".to_string(),
                        ))?;
                evalexpr::Value::String(object_id.to_string())
            }
            _ => {
                error!("Scalar can not be converted to EvalExpr type: {:?}", self);
                return Err(evalexpr::EvalexprError::CustomMessage(
                    format!("Scalar can not be converted to EvalExpr type: {:?}", self).to_string(),
                ));
            }
        };

        trace!("Converted Scalar To EvalExpr Type: {:?}", value);
        Ok(value)
    }
}
