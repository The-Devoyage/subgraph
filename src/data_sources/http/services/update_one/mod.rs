use async_graphql::{Error, ErrorExtensions};
use json::JsonValue;
use log::{debug, info};
use reqwest::Client;

use crate::data_sources::http::filter::HttpDataSourceFilter;

use super::Services;

impl Services {
    pub async fn update_one(
        client: Client,
        filter: HttpDataSourceFilter,
    ) -> Result<JsonValue, async_graphql::Error> {
        info!("Executing Update One - HTTP Data Source");

        let response = Services::request(client, filter).await?;
        debug!("Response: {:?}", response);

        let json = json::parse(&response);
        debug!("JSON: {:?}", json);

        let res = match json {
            Ok(json) => Ok(json),
            Err(error) => {
                debug!("{:?}", error);
                Err(Error::new("HTTP Update One Failed")
                    .extend_with(|err, e| e.set("details", err.message.as_str())))
            }
        };

        res
    }
}
