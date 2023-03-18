use async_graphql::{Error, ErrorExtensions};
use json::JsonValue;
use log::info;
use reqwest::Client;

use crate::data_sources::http::filter::HttpDataSourceFilter;

use super::Services;

impl Services {
    pub async fn create_one(
        client: Client,
        filter: HttpDataSourceFilter,
    ) -> Result<JsonValue, Error> {
        info!("Executing Create One - HTTP Data Source");

        let result = client
            .post(filter.url)
            .json(&filter.request_body)
            .send()
            .await?
            .text()
            .await?;

        let json = json::parse(&result);

        match json {
            Ok(res) => Ok(res),
            Err(_error) => Err(Error::new("HTTP Find One Failed")
                .extend_with(|err, e| e.set("details", err.message.as_str()))),
        }
    }
}
