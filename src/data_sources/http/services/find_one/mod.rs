use async_graphql::{Error, ErrorExtensions};
use json::JsonValue;
use log::{debug, error, trace};
use reqwest::Client;

use crate::data_sources::http::filter::HttpDataSourceFilter;

use super::Services;

impl Services {
    pub async fn find_one(
        client: Client,
        filter: HttpDataSourceFilter,
    ) -> Result<JsonValue, async_graphql::Error> {
        debug!("Executing Find One - HTTP Data Source");

        let response = Services::request(client, filter).await.map_err(|error| {
            error!("{:?}", error);
            Error::new("HTTP Find One Failed")
                .extend_with(|err, e| e.set("details", err.message.as_str()))
        })?;
        trace!("Response Received: {:?}", response);

        let json = json::parse(&response);
        trace!("Response in JSON: {:?}", json);

        let res = match json {
            Ok(res) => Ok(res),
            Err(error) => {
                error!("{:?}", error);
                Err(Error::new("HTTP Find One Failed")
                    .extend_with(|err, e| e.set("details", err.message.as_str())))
            }
        };

        res
    }
}
