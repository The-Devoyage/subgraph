use async_graphql::ErrorExtensions;
use json::JsonValue;
use log::debug;
use reqwest::Client;

use crate::data_sources::http::filter::HttpDataSourceFilter;

use super::Services;

impl Services {
    pub async fn update_many(
        client: Client,
        filter: HttpDataSourceFilter,
    ) -> Result<Vec<JsonValue>, async_graphql::Error> {
        debug!("Executing Update Many - HTTP");

        let response = Services::request(client, filter).await?;

        debug!("Response Received: {:?}", response);

        let json = json::parse(&response);

        debug!("JSON Parsed: {:?}", json);

        let mut results = Vec::new();

        match json {
            Ok(mut res) => res
                .members_mut()
                .for_each(|result| results.push(result.to_owned())),
            Err(_error) => Err(async_graphql::Error::new("HTTP Find One Failed")
                .extend_with(|err, e| e.set("details", err.message.as_str())))?,
        };

        Ok(results)
    }
}
