use std::fmt::Debug;

use handlebars::{Handlebars, RenderError};
use log::{debug, error, trace};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServeAssets {
    pub path: String,
    pub route: String,
}

impl Default for ServeAssets {
    fn default() -> Self {
        ServeAssets {
            path: "/tmp".to_string(),
            route: "assets".to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServeSSR {
    pub path: String,
    pub route: String,
    pub enable_hydrate: Option<bool>,
}

impl Default for ServeSSR {
    fn default() -> Self {
        ServeSSR {
            path: "/tmp".to_string(),
            route: "ssr".to_string(),
            enable_hydrate: Some(false),
        }
    }
}

/// Configuration for the file upload and serving capabilities of the server.
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct ServeOptions {
    pub assets: Option<ServeAssets>,
    pub ssr: Option<ServeSSR>,
}

impl ServeOptions {
    /// Warp Middleware to determine if the file should be served.
    pub async fn handle_serve_asset(
        file: warp::fs::File,
        asset_options: Option<ServeAssets>,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        debug!("Handle Serve Asset");
        if asset_options.is_none() {
            error!("Assets Not Enabled");
            return Err(warp::reject::not_found());
        }
        Ok(file)
    }

    pub fn handle_hydrate_ssr(
        html_template: String,
        ssr_options: Option<ServeSSR>,
    ) -> Result<String, RenderError> {
        debug!("Handle Serve SSR: {:?}", ssr_options.clone());

        let handlebars = Handlebars::new();

        let html = handlebars.render_template(
            &html_template,
            &serde_json::json!({
                "name": "Oliver"
            }),
        );

        trace!("SSR HTML: {:?}", html);

        html
    }
}
