use async_graphql::dynamic::ResolverContext;
use bson::Document;
use log::debug;

use crate::{
    data_sources::sql::services::ResponseRow,
    graphql::resolver::ServiceResolver,
    traits::sqlx::{
        mysql_row::FromMySqlRow, postgres_row::FromPostgresRow, sqlite_row::FromSqliteRow,
    },
};

impl ServiceResolver {
    /// Gets the parent value from the context, which is either a document or a response row.
    /// Converts the response row into a document, which is what the resolver expects.
    pub fn get_parent_value(
        ctx: &ResolverContext,
        field_name: &str,
    ) -> Result<Option<Document>, async_graphql::Error> {
        debug!("Getting Parent Value");

        // Try to downcast as a document, which is what is returned from mongo db.
        // If it fails, then try to downcast as a response row, which is what is returned from sql.
        let parent_value = match ctx.parent_value.try_downcast_ref::<Option<Document>>() {
            Ok(parent_value) => {
                if let Some(parent_value) = parent_value {
                    Some(parent_value.clone())
                } else {
                    debug!("No Parent Value - Mongo");
                    None
                }
            }
            Err(_) => {
                if let Some(rr) = ctx.parent_value.downcast_ref::<Option<ResponseRow>>() {
                    match rr {
                        // If the parent value is a ResponseRow, we can use it to get the value of the
                        // field.
                        // Map the value into a Document, which is what the resolver expects.
                        Some(response_row) => match response_row {
                            ResponseRow::SqLite(rr) => {
                                Some(rr.to_document(Some(vec![field_name]))?)
                            }
                            ResponseRow::MySql(mysql_row) => {
                                Some(mysql_row.to_document(Some(vec![field_name]))?)
                            }
                            ResponseRow::Postgres(rr) => {
                                Some(rr.to_document(Some(vec![field_name]))?)
                            }
                        },
                        None => {
                            debug!("No Parent Value - SQL");
                            None
                        }
                    }
                } else {
                    None
                }
            }
        };

        debug!("Parent Value: {:?}", parent_value);
        Ok(parent_value)
    }
}
