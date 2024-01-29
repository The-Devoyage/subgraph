use log::{debug, error, trace};

use crate::utils::clean_string::clean_string;

pub trait FromSerdeJson {
    fn to_evalalexpr_value(&self) -> Result<evalexpr::Value, evalexpr::EvalexprError>;
}

impl FromSerdeJson for serde_json::Value {
    fn to_evalalexpr_value(&self) -> Result<evalexpr::Value, evalexpr::EvalexprError> {
        debug!("Converting serde_json::Value to evalexpr::Value");

        let value =
            match self {
                serde_json::Value::String(s) => {
                    evalexpr::Value::String(clean_string(&s.to_string(), None))
                }
                serde_json::Value::Null => evalexpr::Value::Empty,
                serde_json::Value::Number(n) => {
                    let num = n.as_i64();
                    if num.is_none() {
                        error!("Invalid number: {:?}", n);
                        return Err(evalexpr::EvalexprError::CustomMessage(
                            "Invalid number.".to_string(),
                        ));
                    }
                    evalexpr::Value::Int(num.unwrap())
                }
                serde_json::Value::Bool(b) => evalexpr::Value::Boolean(*b),
                serde_json::Value::Array(a) => evalexpr::Value::Tuple(
                    a.iter()
                        .map(|v| v.to_evalalexpr_value())
                        .collect::<Result<Vec<evalexpr::Value>, evalexpr::EvalexprError>>()?,
                ),
                serde_json::Value::Object(o) => {
                    let object_id = o.get("$oid");
                    if object_id.is_none() {
                        error!("Invalid object: {:?}", o);
                        return Err(evalexpr::EvalexprError::CustomMessage(
                            "Invalid object.".to_string(),
                        ));
                    }
                    let object_id = object_id.unwrap().as_str().ok_or(
                        evalexpr::EvalexprError::CustomMessage("Invalid object id.".to_string()),
                    )?;
                    evalexpr::Value::String(object_id.to_string())
                }
            };

        trace!(
            "Converted serde_json::Value to evalexpr::Value: {:?}",
            value
        );

        Ok(value)
    }
}
