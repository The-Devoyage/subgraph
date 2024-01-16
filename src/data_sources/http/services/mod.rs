use async_graphql::ErrorExtensions;
use http::Method;
use log::{debug, error, trace};
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
        debug!("Executing Request - HTTP Data Source");

        let response = match filter.method {
            Method::GET => {
                trace!("Using GET Method");
                let res = client.get(filter.url).send().await?;
                match res.status().is_success() {
                    true => res.text().await?,
                    _ => {
                        error!("Response Status: {:?}", res.status());
                        Err(async_graphql::Error::new("HTTP Request Failed")
                            .extend_with(|_, e| e.set("status", res.status().as_u16())))?
                    }
                }
            }
            Method::POST => {
                trace!("Using POST Method");
                let res = client
                    .post(filter.url)
                    .json(&filter.request_body)
                    .send()
                    .await?;
                match res.status().is_success() {
                    true => res.text().await?,
                    _ => {
                        error!("Response Status: {:?}", res.text().await?);
                        Err(async_graphql::Error::new("HTTP Request Failed"))?
                    }
                }
            }
            Method::PUT => {
                trace!("Using PUT Method");
                let res = client
                    .put(filter.url)
                    .json(&filter.request_body)
                    .send()
                    .await?;
                match res.status().is_success() {
                    true => res.text().await?,
                    _ => {
                        error!("Response Status: {:?}", res.status());
                        Err(async_graphql::Error::new("HTTP Request Failed"))?
                    }
                }
            }
            Method::PATCH => {
                trace!("Using PATCH Method");
                let res = client
                    .patch(filter.url)
                    .json(&filter.request_body)
                    .send()
                    .await?;
                match res.status().is_success() {
                    true => res.text().await?,
                    _ => {
                        error!("Response Status: {:?}", res.status());
                        Err(async_graphql::Error::new("HTTP Request Failed"))?
                    }
                }
            }
            _ => {
                error!("Unsupported Method");
                Err(async_graphql::Error::new("Unsupported Method"))?
            }
        };
        trace!("Response: {:?}", response);
        Ok(response)
    }
}
