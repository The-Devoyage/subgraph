use async_graphql::{Error, ErrorExtensions};
use log::info;
use reqwest::{Client, Response};

use crate::data_sources::http::filter::HttpDataSourceFilter;

use super::Services;

impl Services {
    pub async fn create_one(
        client: Client,
        filter: HttpDataSourceFilter,
    ) -> Result<Response, Error> {
        info!("Executing Create One - HTTP Data Source");

        let result = client.post(filter.url).send().await;

        match result {
            Ok(res) => Ok(res),
            Err(_error) => Err(Error::new("HTTP Find One Failed")
                .extend_with(|err, e| e.set("details", err.message.as_str()))),
        }
    }
}
