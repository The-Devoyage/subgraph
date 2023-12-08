use bson::Document;
use log::{debug, trace};

use crate::{
    configuration::subgraph::entities::service_entity_field::ServiceEntityFieldConfig,
    graphql::resolver::ServiceResolver,
};

impl ServiceResolver {
    pub fn remove_virtual_fields(
        input_document: &Document,
        fields: &Vec<ServiceEntityFieldConfig>,
    ) -> Document {
        debug!("Removing Virtual Fields");
        let mut output_document = input_document.clone();

        let values = output_document.get_document("values");
        // let query = output_document.get_document("query");

        if values.is_ok() {
            let values = values.unwrap();
            let values = ServiceResolver::handle_remove_from_doc(values, fields);
            output_document.remove("values");
            output_document.insert("values", values);
        }

        trace!("Output Document: {:?}", output_document);
        output_document
    }

    pub fn handle_remove_from_doc(
        document: &Document,
        fields: &Vec<ServiceEntityFieldConfig>,
    ) -> Document {
        debug!("Removing Fields from Document");
        let mut output_document = document.clone();

        for field in fields {
            if field.is_virtual.unwrap_or(false) {
                trace!("Removing Field: {:?}", field.name);
                output_document.remove(&field.name);
            }
            if field.fields.is_some() {
                let nested_document = output_document.get_document(&field.name);
                if nested_document.is_ok() {
                    trace!("Removing Fields from Nested Field: {:?}", field.name);
                    let nested_document = nested_document.unwrap();
                    let nested_document = ServiceResolver::handle_remove_from_doc(
                        nested_document,
                        &field.fields.clone().unwrap(),
                    );
                    output_document.remove(&field.name);
                    output_document.insert(field.name.clone(), nested_document);
                }
            }
        }

        trace!("Output Document: {:?}", output_document);
        output_document
    }
}
