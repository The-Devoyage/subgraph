use bson::{doc, to_document, Document};
use log::debug;
use mongodb::{options::UpdateOptions, Database};

use crate::data_sources::mongo::MongoDataSource;

use super::Services;

impl Services {
    pub async fn update_many(
        db: Database,
        mut input: Document,
        collection: String,
    ) -> Result<Vec<Document>, async_graphql::Error> {
        debug!("Executing Update Many");

        let coll = db.collection::<Document>(&collection);

        let mut filter = to_document(input.get("query").unwrap()).unwrap();

        filter = MongoDataSource::convert_object_id_string_to_object_id_from_doc(filter)?;

        debug!("Filter: {:?}", filter);

        input.remove("query");

        debug!("Input: {:?}", input);

        let options = UpdateOptions::builder().upsert(true).build();

        let update_doc = Services::create_nested_fields(&input);

        debug!("Update Doc: {:?}", update_doc);

        coll.update_many(filter, doc! {"$set": update_doc}, options)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        let documents = Services::find_many(db, input, collection).await;

        documents
    }
}
