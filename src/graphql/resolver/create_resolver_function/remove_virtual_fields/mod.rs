use bson::Document;
use log::{debug, trace};

use crate::{
    configuration::subgraph::entities::service_entity_field::ServiceEntityFieldConfig,
    filter_operator::FilterOperator, graphql::resolver::ServiceResolver,
};

impl ServiceResolver {
    pub fn remove_virtual_fields(
        input_document: &Document,
        fields: &Vec<ServiceEntityFieldConfig>,
    ) -> Document {
        debug!("Removing Virtual Fields");
        let mut output_document = input_document.clone();

        let query = input_document.get_document("query");
        let values = input_document.get_document("values");

        if values.is_ok() {
            let values = values.unwrap();
            let values = ServiceResolver::handle_remove_from_doc(values, fields);
            output_document.remove("values");
            output_document.insert("values", values);
        }

        if query.is_ok() {
            let query = query.unwrap();
            let query = ServiceResolver::handle_remove_from_doc(query, fields);

            output_document.remove("query");
            output_document.insert("query", query);
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
        let and_filters = document.get_array(FilterOperator::And.as_str());
        let or_filters = document.get_array(FilterOperator::Or.as_str());

        // Remove virtual fields from root document filter.
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

        // Remove virtual fields from AND filters.
        if and_filters.is_ok() {
            let and_filters = and_filters.unwrap();
            let mut new_and_filters = Vec::new();
            for filter in and_filters {
                let filter = filter.as_document().unwrap();
                let filter = ServiceResolver::handle_remove_from_doc(filter, fields);
                new_and_filters.push(filter);
            }
            output_document.remove(FilterOperator::And.as_str());
            output_document.insert(FilterOperator::And.as_str(), new_and_filters);
        }

        // Remove virtual fields from OR filters.
        if or_filters.is_ok() {
            let or_filters = or_filters.unwrap();
            let mut new_or_filters = Vec::new();
            for filter in or_filters {
                let filter = filter.as_document().unwrap();
                let filter = ServiceResolver::handle_remove_from_doc(filter, fields);
                new_or_filters.push(filter);
            }
            output_document.remove(FilterOperator::Or.as_str());
            output_document.insert(FilterOperator::Or.as_str(), new_or_filters);
        }

        trace!("Output Document: {:?}", output_document);
        output_document
    }
}
