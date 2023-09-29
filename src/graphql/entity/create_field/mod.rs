use async_graphql::dynamic::{Field, FieldFuture, TypeRef};
use log::debug;

use crate::{
    configuration::subgraph::entities::service_entity_field::ServiceEntityFieldConfig,
    data_sources::DataSource,
};

use super::ServiceEntity;

mod resolve_fields;
mod resolve_nested;
mod resolve_root;

impl ServiceEntity {
    pub fn create_field(
        entity_field: ServiceEntityFieldConfig,
        type_ref: TypeRef,
        data_source: DataSource,
        is_root_object: bool,
        entity_required: bool,
    ) -> Field {
        debug!("Creating Field, {:?}", entity_field.name);

        let field = Field::new(entity_field.name.clone(), type_ref, move |ctx| {
            let cloned_entity_field = entity_field.clone();
            let data_source = data_source.clone();

            FieldFuture::new(async move {
                match is_root_object {
                    false => ServiceEntity::resolve_nested(&ctx, &cloned_entity_field),
                    true => ServiceEntity::resolve_root(
                        &ctx,
                        &data_source,
                        &cloned_entity_field,
                        entity_required,
                    ),
                }
            })
        });

        debug!("---Created Field: {:?}", field);

        field
    }
}
