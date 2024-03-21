use async_graphql::Error;
use json::JsonValue;
use log::{debug, error, trace};
use reqwest::Client;

use crate::data_sources::http::filter::HttpDataSourceFilter;

use super::Services;

impl Services {
    pub async fn create_one(
        client: Client,
        filter: HttpDataSourceFilter,
    ) -> Result<JsonValue, Error> {
        debug!("Executing Create One - HTTP Data Source");

        let result = Services::request(client, filter).await?;
        trace!("Create One Result: {}", result);

        if result.is_empty() {
            return json::parse("{}").map_err(|e| {
                error!("Error parsing JSON: {}", e);
                Error::new("HTTP Create One Failed")
            });
        }

        let json = json::parse(&result);

        match json {
            Ok(res) => Ok(res),
            Err(error) => {
                error!("Error parsing JSON: {}", error);
                Err(Error::new("HTTP Create One Failed"))
            }
        }
    }
}
