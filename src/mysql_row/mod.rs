use log::{debug, error, trace};
use sqlx::{mysql::MySqlRow, Column, Row, TypeInfo};

pub trait FromMySqlRow {
    fn to_document(
        &self,
        fields: Option<Vec<&str>>,
    ) -> Result<bson::Document, async_graphql::Error>;
}

impl FromMySqlRow for MySqlRow {
    fn to_document(
        &self,
        fields: Option<Vec<&str>>,
    ) -> Result<bson::Document, async_graphql::Error> {
        debug!("Converting MySqlRow to Document");

        let mut document = bson::Document::new();

        for column in self.columns() {
            let column_name = column.name();

            if let Some(fields) = &fields {
                if !fields.contains(&column_name) {
                    continue;
                }
            }

            let column_type = column.type_info().name();

            match column_type {
                "VARCHAR" => {
                    let value: Option<&str> = self.try_get(column_name)?;
                    document.insert(column_name, value);
                }
                "INT" => {
                    let value: Option<i64> = self.try_get(column_name)?;
                    document.insert(column_name, value);
                }
                "BOOL" => {
                    let value: Option<bool> = self.try_get(column_name)?;
                    document.insert(column_name, value);
                }
                "UUID" => {
                    let value: Option<&str> = self.try_get(column_name)?;
                    document.insert(column_name, value);
                }
                "DATETIME" => {
                    let value: Option<&str> = self.try_get(column_name)?;
                    document.insert(column_name, value);
                }
                "TEXT" => {
                    let value: Option<&str> = self.try_get(column_name)?;
                    document.insert(column_name, value);
                }
                "TIMESTAMP" => {
                    let value: Option<chrono::DateTime<chrono::Utc>> = self.try_get(column_name)?;
                    document.insert(column_name, value);
                }
                "BIGINT" => {
                    let value: Option<i64> = self.try_get(column_name)?;
                    document.insert(column_name, value);
                }
                "CHAR" => {
                    let value: Option<&str> = self.try_get(column_name)?;
                    document.insert(column_name, value);
                }
                _ => {
                    error!("Column type not supported: {}", column_type);
                    return Err(async_graphql::Error::new(format!(
                        "Column type not supported: {}",
                        column_type
                    )));
                }
            }
        }

        trace!("Document: {:?}", document);

        Ok(document)
    }
}
