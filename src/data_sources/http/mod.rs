use async_graphql::dynamic::FieldValue;
use bson::Document;
use http::{header::HeaderName, HeaderMap, HeaderValue};
use log::{debug, error, info, trace};
use reqwest::Client;

use crate::{
    configuration::subgraph::{
        data_sources::http::{DefaultHeader, HttpDataSourceConfig},
        entities::ServiceEntityConfig,
        SubGraphConfig,
    },
    graphql::{
        entity::create_return_types::{ResolverResponse, ResolverResponseMeta},
        schema::ResolverType,
    },
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
        input: Document,
        entity: ServiceEntityConfig,
        resolver_type: ResolverType,
        subgraph_config: &SubGraphConfig,
    ) -> Result<Option<FieldValue<'a>>, async_graphql::Error> {
        debug!("Executing HTTP Data Source Operation");

        let data_source = match data_source {
            DataSource::HTTP(ds) => ds,
            _ => unreachable!(),
        };

        trace!("HTTP Data Source: {:?}", data_source);

        let filter =
            HttpDataSource::create_filter(data_source, input, &entity, resolver_type).await?;

        trace!("Filter Created: {:?}", filter);

        match resolver_type {
            ResolverType::FindOne => {
                let result =
                    services::Services::find_one(data_source.client.clone(), filter).await?;
                let res = ResolverResponse {
                    data: vec![FieldValue::owned_any(result)],
                    meta: ResolverResponseMeta {
                        request_id: uuid::Uuid::new_v4().to_string(),
                        service_name: subgraph_config.service.name.clone(),
                        service_version: subgraph_config.service.version.clone(),
                        executed_at: chrono::Utc::now()
                            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                        count: 1,
                        total: 1,
                        page: 1,
                        user_uuid: None,
                    },
                };
                Ok(Some(FieldValue::owned_any(res)))
            }
            ResolverType::FindMany => {
                let results =
                    services::Services::find_many(data_source.client.clone(), filter).await?;
                let count = results.len();
                let res = ResolverResponse {
                    data: results
                        .into_iter()
                        .map(|doc| FieldValue::owned_any(doc))
                        .collect(),
                    meta: ResolverResponseMeta {
                        request_id: uuid::Uuid::new_v4().to_string(),
                        service_name: subgraph_config.service.name.clone(),
                        service_version: subgraph_config.service.version.clone(),
                        executed_at: chrono::Utc::now()
                            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                        count: count as i64,
                        total: count as i64,
                        page: 1,
                        user_uuid: None,
                    },
                };
                Ok(Some(FieldValue::owned_any(res)))
            }
            ResolverType::CreateOne => {
                let result =
                    services::Services::create_one(data_source.client.clone(), filter).await?;
                let res = ResolverResponse {
                    data: vec![FieldValue::owned_any(result)],
                    meta: ResolverResponseMeta {
                        request_id: uuid::Uuid::new_v4().to_string(),
                        service_name: subgraph_config.service.name.clone(),
                        service_version: subgraph_config.service.version.clone(),
                        executed_at: chrono::Utc::now()
                            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                        count: 1,
                        total: 1,
                        page: 1,
                        user_uuid: None,
                    },
                };
                Ok(Some(FieldValue::owned_any(res)))
            }
            ResolverType::UpdateOne => {
                let result =
                    services::Services::update_one(data_source.client.clone(), filter).await?;
                let res = ResolverResponse {
                    data: vec![FieldValue::owned_any(result)],
                    meta: ResolverResponseMeta {
                        request_id: uuid::Uuid::new_v4().to_string(),
                        service_name: subgraph_config.service.name.clone(),
                        service_version: subgraph_config.service.version.clone(),
                        executed_at: chrono::Utc::now()
                            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                        count: 1,
                        total: 1,
                        page: 1,
                        user_uuid: None,
                    },
                };
                Ok(Some(FieldValue::owned_any(res)))
            }
            ResolverType::UpdateMany => {
                let results =
                    services::Services::update_many(data_source.client.clone(), filter).await?;
                let count = results.len();
                let res = ResolverResponse {
                    data: results
                        .into_iter()
                        .map(|doc| FieldValue::owned_any(doc))
                        .collect(),
                    meta: ResolverResponseMeta {
                        request_id: uuid::Uuid::new_v4().to_string(),
                        service_name: subgraph_config.service.name.clone(),
                        service_version: subgraph_config.service.version.clone(),
                        executed_at: chrono::Utc::now()
                            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                        count: count as i64,
                        total: count as i64,
                        page: 1,
                        user_uuid: None,
                    },
                };
                Ok(Some(FieldValue::owned_any(res)))
            }
            _ => panic!("Invalid resolver type"),
        }
    }
}
