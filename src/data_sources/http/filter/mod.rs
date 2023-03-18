use async_graphql::{dynamic::ValueAccessor, Json};
use bson::Document;
use log::{debug, info};
use reqwest::Url;

pub mod request_body;
pub mod url_path;
pub mod url_search_query;

use crate::{configuration::subgraph::entities::ServiceEntity, graphql::schema::ResolverType};

use super::HttpDataSource;

#[derive(Debug)]
pub struct HttpDataSourceFilter {
    pub url: Url,
    pub request_body: Option<Json<Document>>,
}

impl HttpDataSource {
    pub async fn create_filter(
        data_source: &HttpDataSource,
        input: &ValueAccessor<'_>,
        entity: &ServiceEntity,
        resolver_type: ResolverType,
    ) -> Result<HttpDataSourceFilter, async_graphql::Error> {
        info!("Creating Path Filters");
        let mut url = Url::parse(&data_source.config.url)?;

        debug!("Created Url: {:?}", url);

        url = HttpDataSource::create_parameratized_path(url, entity, resolver_type).await?;
        url = HttpDataSource::create_path_filters(url, input).await?;
        url = HttpDataSource::create_parameratized_search_query(url, entity, resolver_type).await?;
        url = HttpDataSource::create_query_string_filters(url, input).await?;
        let request_body = HttpDataSource::create_body_filters(input, resolver_type);

        Ok(HttpDataSourceFilter { url, request_body })
    }
}
