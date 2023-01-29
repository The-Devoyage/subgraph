// use async_graphql::futures_util::StreamExt;
// use bson::{doc, Document};
// use log::{debug, error, info, log_enabled, trace, warn, Level};
// use mongodb::{
//     options::{IndexOptions, ListIndexesOptions},
//     IndexModel,
// };

// pub struct Indexes;

// impl Indexes {
//     pub async fn list_indexes(db: &mongodb::Database) {
//         info!("Checking Indexes.");
//         let list_indexes_options = ListIndexesOptions::builder().build();
//         let mut cursor = db
//             .collection::<Document>("users")
//             .list_indexes(list_indexes_options)
//             .await
//             .expect("Failed to find indexes.");
//         while let Some(result) = cursor.next().await {
//             match result {
//                 Ok(index) => {
//                     if log_enabled!(Level::Debug) {
//                         debug!("Found index. {:?}", index);
//                     }
//                     info! {"Found index: {:?}", index.options.unwrap().name};
//                 }
//                 Err(error) => {
//                     error!("Error when finding indexes.");
//                     if log_enabled!(Level::Debug) {
//                         debug!("Error when finding index {:?}", error);
//                     }
//                 }
//             }
//         }
//     }

//     pub async fn create_index(db: &mongodb::Database) {
//         let index_options = IndexOptions::builder().unique(true).build();
//         let index_model = IndexModel::builder()
//             .keys(doc! {"email": 1})
//             .options(index_options)
//             .build();

//         db.collection::<Document>("users")
//             .create_index(index_model, None)
//             .await
//             .expect("Failed to create indexes.");

//         trace!("Created index.")
//     }

//     pub async fn remove_index(db: &mongodb::Database) {
//         let index_name = "email_1";

//         let result = db
//             .collection::<Document>("users")
//             .drop_index("email_1", None)
//             .await;

//         match result {
//             Ok(()) => info!("Removed index: {}.", index_name),
//             Err(error) => {
//                 if log_enabled!(Level::Debug) {
//                     debug!("Index Remove Error: {:?}", error)
//                 }
//                 error!("Failed to remove index: {}.", index_name)
//             }
//         }
//     }
// }
