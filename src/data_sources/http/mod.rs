use async_graphql::dynamic::{FieldValue, ValueAccessor};
use http::{header::HeaderName, HeaderMap, HeaderValue};
use log::{debug, info};
use reqwest::{Client, Url};

use crate::{
    configuration::subgraph::{
        data_sources::http::{DefaultHeader, HttpDataSourceConfig},
        entities::ServiceEntity,
    },
    graphql::schema::ResolverType,
};

use super::DataSource;
pub mod filter;
pub mod services;

#[derive(Debug, Clone)]
pub struct HttpDataSource {
    pub client: Client,
    pub config: HttpDataSourceConfig,
}

impl HttpDataSource {
    fn get_headers(default_headers: Option<&Vec<DefaultHeader>>) -> HeaderMap {
        let mut headers = HeaderMap::new();

        if default_headers.is_some() {
            let cloned_default_headers = default_headers.unwrap().clone();

            for default_header in cloned_default_headers {
                let header_name = HeaderName::from_bytes(default_header.name.as_bytes()).unwrap();
                let header_value =
                    HeaderValue::from_bytes(default_header.value.as_bytes()).unwrap();
                headers.insert(header_name, header_value);
            }
        }

        headers
    }

    pub async fn init(http_data_source_config: &HttpDataSourceConfig) -> DataSource {
        let header_config = http_data_source_config.default_headers.as_ref();
        let headers = HttpDataSource::get_headers(header_config.clone());
        let client = Client::builder().default_headers(headers).build();

        match client {
            Ok(client) => DataSource::HTTP(HttpDataSource {
                client,
                config: http_data_source_config.clone(),
            }),
            Err(error) => {
                log::error!("Failed to build HTTP Client.");
                debug!("{:?}", error);
                panic!()
            }
        }
    }

    pub async fn execute_operation<'a>(
        data_source: &DataSource,
        input: &ValueAccessor<'_>,
        entity: ServiceEntity,
        resolver_type: ResolverType,
    ) -> Result<FieldValue<'a>, async_graphql::Error> {
        info!("Executing HTTP Data Source Operation");

        let data_source = match data_source {
            DataSource::HTTP(ds) => ds,
            _ => unreachable!(),
        };

        debug!("HTTP Data Source: {:?}", data_source);

        let filter =
            HttpDataSource::create_filter(data_source, input, &entity, resolver_type).await?;

        match resolver_type {
            ResolverType::FindOne => {
                let result =
                    services::Services::find_one(data_source.client.clone(), filter).await?;

                Ok(FieldValue::owned_any(result))
            }
            ResolverType::FindMany => {
                let results =
                    services::Services::find_many(data_source.client.clone(), filter).await?;

                Ok(FieldValue::list(
                    results.into_iter().map(|doc| FieldValue::owned_any(doc)),
                ))
            }
            ResolverType::CreateOne => {
                let result =
                    services::Services::create_one(data_source.client.clone(), filter).await?;

                Ok(FieldValue::owned_any(result))
            }
        }
    }
}
