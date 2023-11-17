use async_graphql::ErrorExtensions;
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
                let res = client.get(filter.url).send().await?;
                match res.status().is_success() {
                    true => res.text().await?,
                    _ => {
                        debug!("Response Status: {:?}", res.status());
                        Err(async_graphql::Error::new("HTTP Request Failed")
                            .extend_with(|_, e| e.set("status", res.status().as_u16())))?
                    }
                }
            }
            Method::POST => {
                debug!("Using POST Method");
                let res = client
                    .post(filter.url)
                    .json(&filter.request_body)
                    .send()
                    .await?;
                match res.status().is_success() {
                    true => res.text().await?,
                    _ => {
                        debug!("Response Status: {:?}", res.status());
                        Err(async_graphql::Error::new("HTTP Request Failed"))?
                    }
                }
            }
            Method::PUT => {
                debug!("Using PUT Method");
                let res = client
                    .put(filter.url)
                    .json(&filter.request_body)
                    .send()
                    .await?;
                match res.status().is_success() {
                    true => res.text().await?,
                    _ => {
                        debug!("Response Status: {:?}", res.status());
                        Err(async_graphql::Error::new("HTTP Request Failed"))?
                    }
                }
            }
            Method::PATCH => {
                debug!("Using PATCH Method");
                let res = client
                    .patch(filter.url)
                    .json(&filter.request_body)
                    .send()
                    .await?;
                match res.status().is_success() {
                    true => res.text().await?,
                    _ => {
                        debug!("Response Status: {:?}", res.status());
                        Err(async_graphql::Error::new("HTTP Request Failed"))?
                    }
                }
            }
            _ => {
                debug!("Unsupported Method");
                Err(async_graphql::Error::new("Unsupported Method"))?
            }
        };
        Ok(response)
    }
}
