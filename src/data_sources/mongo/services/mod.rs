use bson::{doc, Document};
use log::{debug, trace};

use crate::filter_operator::FilterOperator;

mod create_one;
mod find_many;
mod find_one;
mod update_many;
mod update_one;

#[derive(Debug)]
pub struct Services;

impl Services {
    pub fn create_nested_fields(doc: &Document) -> Document {
        debug!("Creating Nested Filter");
        let mut set_doc = Document::new();
        for (key, value) in doc.iter() {
            if let Some(sub_doc) = value.as_document() {
                debug!("Found Sub Document: {:?}", sub_doc);
                let sub_set_doc = Services::create_nested_fields(sub_doc);
                for (sub_key, sub_value) in sub_set_doc.iter() {
                    let nested_key = format!("{}.{}", key, sub_key);
                    set_doc.insert(nested_key, sub_value.clone());
                }
            } else {
                set_doc.insert(key.clone(), value.clone());
            }
        }
        debug!("Created Nested Filter: {:?}", set_doc);
        set_doc
    }

    /// Takes in a graphql input `query` and parses it into a nested find filter.
    pub fn create_nested_find_filter(query_doc: &Document) -> Document {
        debug!("Creating Nested Find Filter");
        trace!("Query Doc: {:?}", query_doc);
        let mut find_doc = Document::new();
        for (key, value) in query_doc.clone().iter_mut() {
            // If the value is a doc, create a key that represents the nested field
            if let Some(sub_doc) = value.as_document() {
                trace!("Found Sub Document: {:?}", sub_doc);
                let sub_set_doc = Services::create_nested_find_filter(sub_doc);
                for (sub_key, sub_value) in sub_set_doc.iter() {
                    let is_filter_operator =
                        FilterOperator::list_mongo_operators().contains(sub_key);
                    if is_filter_operator {
                        // let filter = doc! {
                        //     sub_key.clone(): Regex {
                        //         pattern: sub_value.as_str().unwrap().to_string(),
                        //         options: "i".to_string()
                        //     }
                        // };
                        // trace!("Inserted Filter: {:?}", filter);
                        // find_doc.insert(key.clone(), filter);
                        let filter_operator = FilterOperator::from_str(sub_key).unwrap();
                        let filter = FilterOperator::convert_value_to_mongo(
                            &filter_operator,
                            key,
                            sub_value,
                        );
                        trace!("Inserted Filter: {:?}", filter);
                        find_doc.insert(key.clone(), filter);
                    } else {
                        let nested_key = format!("{}.{}", key, sub_key);
                        find_doc.insert(nested_key, sub_value.clone());
                    }
                }
            } else {
                if let Some(array) = value.as_array() {
                    trace!("Found Array: {:?}", array);
                    // If not a filter array, then call recursively on each document in the array
                    if key == "$and" || key == "$or" {
                        let mut docs = vec![];
                        for b in array {
                            let find_filter =
                                Services::create_nested_find_filter(b.as_document().unwrap());
                            docs.push(find_filter);
                        }
                        find_doc.insert(key.clone(), docs);
                        continue;
                    }

                    // Handle Object Types
                    trace!("Checking if array is docs");
                    let is_docs = array.iter().all(|bson| bson.as_document().is_some());
                    if is_docs {
                        let mut docs = vec![];
                        for b in array {
                            docs.push(doc! { key.clone(): {"$elemMatch": b}})
                        }
                        if docs.len() > 0 {
                            find_doc.insert("$and", docs);
                        } else {
                            find_doc.insert(key.clone(), doc! { "$in": array });
                        }
                    } else {
                        // Handle array of primitives.
                        trace!("Array is not docs");
                        find_doc.insert(key.clone(), doc! { "$in": array });
                    }
                    continue;
                }
                // Handle primitive default
                trace!("Found Primitive: {:?}", value);
                find_doc.insert(key.clone(), value.clone());
            }
        }
        debug!("Created Nested Find Filter");
        trace!("Nested Find Filter: {:?}", find_doc);
        find_doc
    }
}
