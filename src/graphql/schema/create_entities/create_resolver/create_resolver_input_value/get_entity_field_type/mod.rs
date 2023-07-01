use crate::{
    configuration::subgraph::entities::{service_entity_field::ServiceEntityField, ScalarOptions},
    graphql::schema::{ResolverType, ServiceSchemaBuilder},
};
use async_graphql::dynamic::{InputObject, TypeRef};
use log::debug;

pub struct TypeRefWithInputs {
    pub type_ref: TypeRef,
    pub inputs: Vec<InputObject>,
}

impl ServiceSchemaBuilder {
    pub fn get_entity_string_field_type(
        resolver_type: &ResolverType,
        is_list: bool,
        is_required: bool,
    ) -> TypeRef {
        match resolver_type {
            ResolverType::FindOne
            | ResolverType::FindMany
            | ResolverType::UpdateOne
            | ResolverType::UpdateMany => {
                if is_list {
                    TypeRef::named_nn_list(TypeRef::STRING)
                } else {
                    TypeRef::named(TypeRef::STRING)
                }
            }
            ResolverType::CreateOne => match is_required {
                true => {
                    if is_list {
                        TypeRef::named_nn_list(TypeRef::STRING)
                    } else {
                        TypeRef::named_nn(TypeRef::STRING)
                    }
                }
                _ => {
                    if is_list {
                        TypeRef::named_nn_list(TypeRef::STRING)
                    } else {
                        TypeRef::named(TypeRef::STRING)
                    }
                }
            },
            _ => panic!("Invalid resolver type"),
        }
    }

    pub fn get_entity_int_field_type(
        resolver_type: &ResolverType,
        is_list: bool,
        is_required: bool,
    ) -> TypeRef {
        match resolver_type {
            ResolverType::FindOne
            | ResolverType::FindMany
            | ResolverType::UpdateOne
            | ResolverType::UpdateMany => {
                if is_list {
                    TypeRef::named_nn_list(TypeRef::INT)
                } else {
                    TypeRef::named(TypeRef::INT)
                }
            }
            ResolverType::CreateOne => match is_required {
                true => {
                    if is_list {
                        TypeRef::named_nn_list_nn(TypeRef::INT)
                    } else {
                        TypeRef::named_nn(TypeRef::INT)
                    }
                }
                _ => {
                    if is_required {
                        TypeRef::named_nn_list(TypeRef::INT)
                    } else {
                        TypeRef::named(TypeRef::INT)
                    }
                }
            },
            _ => panic!("Invalid resolver type"),
        }
    }

    pub fn get_entity_bool_field_type(
        resolver_type: &ResolverType,
        is_list: bool,
        is_required: bool,
    ) -> TypeRef {
        match resolver_type {
            ResolverType::FindOne
            | ResolverType::FindMany
            | ResolverType::UpdateOne
            | ResolverType::UpdateMany => {
                if is_list {
                    TypeRef::named_nn_list(TypeRef::BOOLEAN)
                } else {
                    TypeRef::named(TypeRef::BOOLEAN)
                }
            }
            ResolverType::CreateOne => match is_required {
                true => {
                    if is_list {
                        TypeRef::named_nn_list_nn(TypeRef::BOOLEAN)
                    } else {
                        TypeRef::named_nn(TypeRef::BOOLEAN)
                    }
                }
                _ => {
                    if is_list {
                        TypeRef::named_nn_list(TypeRef::BOOLEAN)
                    } else {
                        TypeRef::named(TypeRef::BOOLEAN)
                    }
                }
            },
            _ => panic!("Invalid resolver type"),
        }
    }

    pub fn get_entity_object_id_field_type(
        resolver_type: &ResolverType,
        is_list: bool,
        is_required: bool,
    ) -> TypeRef {
        match resolver_type {
            ResolverType::FindOne
            | ResolverType::FindMany
            | ResolverType::UpdateOne
            | ResolverType::UpdateMany => {
                if is_list {
                    TypeRef::named_nn_list("ObjectID")
                } else {
                    TypeRef::named("ObjectID")
                }
            }
            ResolverType::CreateOne => match is_required {
                true => {
                    if is_list {
                        TypeRef::named_nn_list_nn("ObjectID")
                    } else {
                        TypeRef::named_nn("ObjectID")
                    }
                }
                _ => {
                    if is_list {
                        TypeRef::named_nn_list("ObjectID")
                    } else {
                        TypeRef::named("ObjectID")
                    }
                }
            },
            _ => panic!("Invalid resolver type"),
        }
    }

    fn format_child_field_name(
        parent_field_name: &str,
        child_field_name: &str,
        resolver_type: &ResolverType,
    ) -> String {
        match resolver_type {
            ResolverType::FindOne
            | ResolverType::CreateOne
            | ResolverType::UpdateOne
            | ResolverType::UpdateMany => {
                format!("{}_{}_input", parent_field_name, child_field_name)
            }
            ResolverType::FindMany => format!("{}_{}s_input", parent_field_name, child_field_name),
            _ => panic!("Invalid resolver type"),
        }
    }

    pub fn get_entity_object_field_type(
        entity_field: &ServiceEntityField,
        resolver_type: &ResolverType,
        parent_input_prefix: &str,
    ) -> TypeRefWithInputs {
        let mut inputs = Vec::new();

        let input_name = ServiceSchemaBuilder::format_child_field_name(
            parent_input_prefix,
            &entity_field.name,
            resolver_type,
        );

        let object_inputs = ServiceSchemaBuilder::create_input(
            input_name.clone(),
            entity_field.fields.clone().unwrap_or(Vec::new()),
            resolver_type,
            None,
        );

        for input in object_inputs {
            inputs.push(input);
        }

        let type_ref = match resolver_type {
            ResolverType::FindOne
            | ResolverType::FindMany
            | ResolverType::UpdateOne
            | ResolverType::UpdateMany => {
                if entity_field.list == Some(true) {
                    TypeRef::named_nn_list(input_name)
                } else {
                    TypeRef::named(input_name)
                }
            }
            ResolverType::CreateOne => match entity_field.required {
                Some(true) => {
                    if entity_field.list == Some(true) {
                        TypeRef::named_nn_list_nn(input_name)
                    } else {
                        TypeRef::named_nn(input_name)
                    }
                }
                _ => {
                    if entity_field.list == Some(true) {
                        TypeRef::named_nn_list_nn(input_name)
                    } else {
                        TypeRef::named(input_name)
                    }
                }
            },
            _ => panic!("Invalid resolver type"),
        };

        TypeRefWithInputs { type_ref, inputs }
    }

    pub fn get_entity_field_type(
        entity_field: &ServiceEntityField,
        resolver_type: &ResolverType,
        parent_input_prefix: &str,
    ) -> TypeRefWithInputs {
        debug!("Creating Entity Field Type");
        let mut inputs = Vec::new();

        let is_list = entity_field.list.is_some() && entity_field.list.unwrap();
        let is_required = entity_field.required.is_some() && entity_field.required.unwrap();

        let type_ref = match &entity_field.scalar {
            ScalarOptions::String => ServiceSchemaBuilder::get_entity_string_field_type(
                resolver_type,
                is_list,
                is_required,
            ),
            ScalarOptions::Int => {
                ServiceSchemaBuilder::get_entity_int_field_type(resolver_type, is_list, is_required)
            }
            ScalarOptions::Boolean => ServiceSchemaBuilder::get_entity_bool_field_type(
                resolver_type,
                is_list,
                is_required,
            ),
            ScalarOptions::ObjectID => ServiceSchemaBuilder::get_entity_object_id_field_type(
                resolver_type,
                is_list,
                is_required,
            ),
            ScalarOptions::Object => {
                let type_ref_with_inputs = ServiceSchemaBuilder::get_entity_object_field_type(
                    entity_field,
                    resolver_type,
                    parent_input_prefix,
                );

                for input in type_ref_with_inputs.inputs {
                    inputs.push(input);
                }

                type_ref_with_inputs.type_ref
            }
        };

        TypeRefWithInputs { type_ref, inputs }
    }
}
