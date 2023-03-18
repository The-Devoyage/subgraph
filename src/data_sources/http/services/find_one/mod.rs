use async_graphql::{Error, ErrorExtensions};
use json::JsonValue;
use log::{debug, info};
use reqwest::Client;

use crate::data_sources::http::filter::HttpDataSourceFilter;

use super::Services;

impl Services {
    pub async fn find_one(
        client: Client,
        filter: HttpDataSourceFilter,
    ) -> Result<JsonValue, async_graphql::Error> {
        info!("Executing Find One - HTTP Data Source");

        let response = &client.get(filter.url).send().await?.text().await?;
        debug!("Response Received: {:?}", response);

        let json = json::parse(response);
        debug!("Response in JSON: {:?}", json);

        let res = match json {
            Ok(res) => Ok(res),
            Err(error) => {
                debug!("{:?}", error);
                Err(Error::new("HTTP Find One Failed")
                    .extend_with(|err, e| e.set("details", err.message.as_str())))
            }
        };

        res
    }
}
