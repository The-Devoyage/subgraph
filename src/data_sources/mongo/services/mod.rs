use bson::Document;

// mod create_many;
mod create_one;
mod find_many;
mod find_one;
mod update_one;

#[derive(Debug)]
pub struct Services;

impl Services {
    pub fn create_nested_fields(doc: &Document) -> Document {
        let mut set_doc = Document::new();
        for (key, value) in doc.iter() {
            if let Some(sub_doc) = value.as_document() {
                let sub_set_doc = Services::create_nested_fields(sub_doc);
                for (sub_key, sub_value) in sub_set_doc.iter() {
                    let nested_key = format!("{}.{}", key, sub_key);
                    set_doc.insert(nested_key, sub_value.clone());
                }
            } else {
                set_doc.insert(key.clone(), value.clone());
            }
        }
        set_doc
    }
}
