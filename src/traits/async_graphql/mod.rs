use async_graphql::dynamic::indexmap::IndexMap;
use log::{debug, trace};

pub trait FromJson {
    fn to_async_graphql_value(&self) -> async_graphql::Value;
}

impl FromJson for json::JsonValue {
    fn to_async_graphql_value(&self) -> async_graphql::Value {
        debug!("Converting json::JsonValue to async_graphql::Value");

        let value = match self {
            json::JsonValue::String(s) => async_graphql::Value::String(s.to_string()),
            json::JsonValue::Null => async_graphql::Value::Null,
            json::JsonValue::Short(s) => async_graphql::Value::String(s.to_string()),
            json::JsonValue::Number(n) => {
                let num: f64 = n.clone().into();
                async_graphql::Value::from(num)
            }
            json::JsonValue::Boolean(b) => async_graphql::Value::Boolean(*b),
            json::JsonValue::Array(a) => async_graphql::Value::List(
                a.iter()
                    .map(|v| v.to_async_graphql_value())
                    .collect::<Vec<async_graphql::Value>>(),
            ),
            json::JsonValue::Object(o) => {
                let mut index_map = IndexMap::new();
                for (k, v) in o.iter() {
                    let name = async_graphql::Name::new(k);
                    index_map.insert(name, v.to_async_graphql_value());
                }
                async_graphql::Value::Object(index_map)
            }
        };

        trace!(
            "Converted json::JsonValue to async_graphql::Value: {:?}",
            value
        );

        value
    }
}
