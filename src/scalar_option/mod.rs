use serde::{Deserialize, Serialize};

pub mod document_field_to_async_graphql_value;
pub mod get_from_document;
pub mod json_to_async_graphql_value;
pub mod mysql_rr_to_input_doc;
pub mod pg_rr_to_input_doc;
pub mod rr_to_async_graphql_value;
pub mod rr_to_serde_json_value;
pub mod sqlite_rr_to_input_doc;
pub mod to_bson_element_type;
pub mod to_evalexpr;
pub mod to_input_type_ref;
pub mod to_mongo;
pub mod to_nn_type_ref;
pub mod to_nullable_type_ref;

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
