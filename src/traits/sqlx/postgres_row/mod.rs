use bson::Document;
use log::{debug, error, trace};
use sqlx::{postgres::PgRow, Column, Row, TypeInfo};

pub trait FromPostgresRow {
    fn to_document(&self, field_names: Option<Vec<&str>>)
        -> Result<Document, async_graphql::Error>;
}

impl FromPostgresRow for PgRow {
    fn to_document(
        &self,
        field_names: Option<Vec<&str>>,
    ) -> Result<Document, async_graphql::Error> {
        debug!("Converting Postgres Row to Document");
        let mut document = Document::new();

        for column in self.columns() {
            let column_name = column.name();

            if let Some(field_names) = &field_names {
                if !field_names.contains(&column_name) {
                    continue;
                }
            }

            let column_type = column.type_info().name();

            match column_type {
                "VARCHAR" => {
                    let value: Option<&str> = self.try_get(column_name)?;
                    document.insert(column_name, value);
                }
                "CHAR" => {
                    let value: Option<&str> = self.try_get(column_name)?;
                    document.insert(column_name, value);
                }
                "TEXT" => {
                    let value: Option<&str> = self.try_get(column_name)?;
                    document.insert(column_name, value);
                }
                "DATETIME" => {
                    let value: Option<&str> = self.try_get(column_name)?;
                    document.insert(column_name, value);
                }
                "TIMESTAMP" => {
                    let value: Option<&str> = self.try_get(column_name)?;
                    document.insert(column_name, value);
                }
                "BIGINT" => {
                    let value: Option<i64> = self.try_get(column_name)?;
                    document.insert(column_name, value);
                }
                "UUID" => {
                    let value: Option<uuid::Uuid> = self.try_get(column_name)?;
                    if let Some(value) = value {
                        document.insert(column_name, value.to_string());
                    }
                }
                "BOOL" => {
                    let value: Option<bool> = self.try_get(column_name)?;
                    document.insert(column_name, value);
                }

                _ => {
                    error!("Unsupported Column Type: {}", column_type);
                    return Err(async_graphql::Error::new(format!(
                        "Unsupported Column Type: {}",
                        column_type
                    )));
                }
            }
        }

        trace!("{:?}", document);

        Ok(document)
    }
}
