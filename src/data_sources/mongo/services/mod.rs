use bson::{doc, Document};
use log::debug;

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
    pub fn create_nested_find_filter(doc: &Document) -> Document {
        debug!("Creating Nested Find Filter From Doc: {:?}", doc);
        let mut find_doc = Document::new();
        for (key, value) in doc.clone().iter_mut() {
            if let Some(sub_doc) = value.as_document() {
                let sub_set_doc = Services::create_nested_find_filter(sub_doc);
                for (sub_key, sub_value) in sub_set_doc.iter() {
                    let nested_key = format!("{}.{}", key, sub_key);
                    find_doc.insert(nested_key, sub_value.clone());
                }
            } else {
                if let Some(array) = value.as_array() {
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
                        find_doc.insert(key.clone(), doc! { "$in": array });
                    }
                    continue;
                }
                find_doc.insert(key.clone(), value.clone());
            }
        }
        debug!("Created Nested Find Filter: {:?}", find_doc);
        find_doc
    }
}
