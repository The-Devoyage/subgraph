use std::str::FromStr;

use async_graphql::dynamic::ResolverContext;
use bson::{oid::ObjectId, Document};
use log::{debug, error};

use crate::{
    configuration::subgraph::entities::{ScalarOptions, ServiceEntity},
    graphql::schema::ServiceSchemaBuilder,
};

impl ServiceSchemaBuilder {
    pub fn get_internal_input(
        as_type_entity_parent: Option<ServiceEntity>,
        ctx: &ResolverContext,
    ) -> Result<Document, async_graphql::Error> {
        debug!("Getting Internal Input");
        let field_input = ctx.args.try_get(&format!("{}", ctx.field().name()))?;
        let field_name = ctx.field().name().to_string();
        let parent_value = ctx.parent_value.downcast_ref::<Document>().unwrap().clone();
        let field = ServiceEntity::get_field(as_type_entity_parent.unwrap(), field_name.clone())?;
        let join_on = field.join_on.unwrap();
        let field_input = field_input.deserialize::<Document>().unwrap();
        let mut field_input = field_input.clone();
        let scalar = field.scalar;
        let list = field.list.unwrap_or(false);
        match list {
            true => {
                let join_on_value = parent_value.get_array(field_name.clone());
                let join_on_value = match join_on_value {
                    Ok(join_on_value) => join_on_value,
                    Err(_) => {
                        return Err(async_graphql::Error::new(format!(
                            "Field {} not found.",
                            field_name
                        )))
                    }
                };
                match scalar {
                    ScalarOptions::String => {
                        let join_on_value = join_on_value
                            .iter()
                            .map(|value| value.to_string())
                            .collect::<Vec<String>>();
                        field_input.insert(join_on.clone(), join_on_value);
                    }
                    ScalarOptions::Int => {
                        let join_on_value = join_on_value
                            .iter()
                            .map(|value| value.as_i32().unwrap())
                            .collect::<Vec<i32>>();
                        field_input.insert(join_on.clone(), join_on_value);
                    }
                    ScalarOptions::Boolean => {
                        let join_on_value = join_on_value
                            .iter()
                            .map(|value| value.as_bool().unwrap())
                            .collect::<Vec<bool>>();
                        field_input.insert(join_on.clone(), join_on_value);
                    }
                    ScalarOptions::ObjectID => {
                        let join_on_value = join_on_value
                            .iter()
                            .map(|value| ObjectId::from_str(value.as_str().unwrap()).unwrap())
                            .collect::<Vec<ObjectId>>();
                        field_input.insert(join_on.clone(), join_on_value);
                    }
                    _ => panic!("Invalid Scalar Type"),
                }
            }
            false => match scalar {
                ScalarOptions::Int => {
                    let join_on_value = parent_value.get_i32(field_name.clone());
                    let join_on_value = match join_on_value {
                        Ok(join_on_value) => join_on_value,
                        Err(_) => {
                            error!("Field {} not found. Invalid Int", field_name);
                            return Err(async_graphql::Error::new(format!(
                                "Field {} not found. Invalid Int",
                                field_name
                            )));
                        }
                    };
                    field_input.insert(join_on.clone(), join_on_value);
                }
                ScalarOptions::String => {
                    let join_on_value = parent_value.get_str(field_name.clone());
                    let join_on_value = match join_on_value {
                        Ok(join_on_value) => join_on_value,
                        Err(_) => {
                            error!("Field {} not found. Invalid String", field_name);
                            return Err(async_graphql::Error::new(format!(
                                "Field {} not found. Invalid String",
                                field_name
                            )));
                        }
                    };
                    field_input.insert(join_on.clone(), join_on_value);
                }
                ScalarOptions::Boolean => {
                    let join_on_value = parent_value.get_bool(field_name.clone());
                    let join_on_value = match join_on_value {
                        Ok(join_on_value) => join_on_value,
                        Err(_) => {
                            error!("Field {} not found. Invalid Boolean", field_name);
                            return Err(async_graphql::Error::new(format!(
                                "Field {} not found. Invalid Boolean",
                                field_name
                            )));
                        }
                    };
                    field_input.insert(join_on.clone(), join_on_value);
                }
                ScalarOptions::ObjectID => {
                    let join_on_value = parent_value.get_object_id(field_name.clone());
                    debug!("Join On Value: {:?}", join_on_value);
                    let join_on_value = match join_on_value {
                        Ok(join_on_value) => join_on_value,
                        Err(_) => {
                            let strign_object_id = parent_value.get_str(field_name.clone())?;
                            let join_on_value = ObjectId::from_str(strign_object_id)?;
                            join_on_value
                        }
                    };
                    field_input.insert(join_on.clone(), join_on_value);
                }
                _ => panic!("Unsupported Scalar Type"),
            },
        }
        debug!("Internal Input: {:?}", field_input);
        Ok(field_input)
    }
}
