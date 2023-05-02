use http::Method;
use log::{debug, info};
use reqwest::Client;

use super::filter::HttpDataSourceFilter;

pub mod create_one;
pub mod find_many;
pub mod find_one;
pub mod update_many;
pub mod update_one;

pub struct Services;

impl Services {
    pub async fn request(
        client: Client,
        filter: HttpDataSourceFilter,
    ) -> Result<String, async_graphql::Error> {
        info!("Executing Request - HTTP Data Source");

        let response = match filter.method {
            Method::GET => {
                debug!("Using GET Method");
                client.get(filter.url).send().await?.text().await?
            }
            Method::POST => {
                debug!("Using POST Method");
                client
                    .post(filter.url)
                    .json(&filter.request_body)
                    .send()
                    .await?
                    .text()
                    .await?
            }
            Method::PUT => {
                debug!("Using PUT Method");
                client
                    .put(filter.url)
                    .json(&filter.request_body)
                    .send()
                    .await?
                    .text()
                    .await?
            }
            Method::PATCH => {
                debug!("Using PATCH Method");
                client
                    .patch(filter.url)
                    .json(&filter.request_body)
                    .send()
                    .await?
                    .text()
                    .await?
            }
            _ => {
                debug!("Using Default Method: GET");
                client.get(filter.url).send().await?.text().await?
            }
        };
        Ok(response)
    }
}
