use async_graphql::Json;
use bson::Document;
use http::Method;
use log::{debug, info};
use reqwest::Url;

pub mod method;
pub mod request_body;
pub mod url_path;
pub mod url_search_query;

use crate::{
    configuration::subgraph::entities::ServiceEntityConfig, graphql::schema::ResolverType,
};

use super::HttpDataSource;

#[derive(Debug)]
pub struct HttpDataSourceFilter {
    pub url: Url,
    pub request_body: Option<Json<Document>>,
    pub method: Method,
}

impl HttpDataSource {
    pub async fn create_filter(
        data_source: &HttpDataSource,
        input: Document,
        entity: &ServiceEntityConfig,
        resolver_type: ResolverType,
    ) -> Result<HttpDataSourceFilter, async_graphql::Error> {
        info!("Creating Path Filters");
        let mut url = Url::parse(&data_source.config.url)?;

        debug!("Created Url: {:?}", url);

        url = HttpDataSource::create_parameratized_path(url, entity, resolver_type).await?;
        url = HttpDataSource::create_path_filters(url, input.clone(), resolver_type).await?;
        url = HttpDataSource::create_parameratized_search_query(url, entity, resolver_type).await?;
        url = HttpDataSource::create_query_string_filters(url, input.clone()).await?;
        let request_body = HttpDataSource::create_body_filters(input, resolver_type);

        let method = HttpDataSource::get_method(entity, resolver_type);

        Ok(HttpDataSourceFilter {
            url,
            request_body,
            method,
        })
    }
}
