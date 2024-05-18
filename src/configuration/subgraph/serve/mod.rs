use std::fmt::Debug;

use async_graphql::{Request, Variables};
use handlebars::{Handlebars, RenderError};
use log::{debug, error, trace, warn};
use serde::{Deserialize, Serialize};
use warp::{http::Response, reject::Rejection};

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

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct SsrRequestBody {
    query: String,
    operation_name: String,
    variables: serde_json::Value,
}

/// Configuration for the file upload and serving capabilities of the server.
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct ServeOptions {
    pub assets: Option<ServeAssets>,
    pub ssr: Option<ServeSSR>,
}

pub type FilePath = String;
pub type Extension = Option<String>;

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
        context: Option<serde_json::Value>,
    ) -> Result<String, RenderError> {
        debug!("Handle Serve SSR: {:?}", ssr_options.clone());
        trace!("SSR Context: {:?}", context);

        let handlebars = Handlebars::new();

        let html = handlebars.render_template(&html_template, &context);

        trace!("SSR HTML: {:?}", html);

        html
    }

    // Recusively search up the path for the index.html file
    pub fn search_up_path(file_path: String) -> Result<String, Rejection> {
        warn!("Search Up Path: {:?}", file_path);
        let path = file_path;
        loop {
            trace!("Search Up Path: {:?}", path);
            let path = path.split("/").collect::<Vec<&str>>();
            let path = path[0..path.len() - 2].join("/");
            let file_path = format!("{}/index.html", path);
            trace!("Implied HTML Extension: {:?}", file_path);
            let file = std::fs::read_to_string(file_path).ok();
            if file.is_some() {
                break Ok(file.unwrap());
            }
            // if all the way up to the root directory, return a 404
            if path == "" {
                return Err(warp::reject::not_found());
            }
        }
    }

    pub fn format_file_path(file_path: String) -> Result<(FilePath, Extension), Rejection> {
        debug!("Format File Path: {:?}", file_path);
        let ext;
        let is_directory = std::path::Path::extension(std::path::Path::new(&file_path)).is_none();
        trace!("Is Directory: {:?}", is_directory);

        let formatted = if is_directory {
            let file_path = format!("{}/index.html", file_path);
            trace!("Implied HTML Extension: {:?}", file_path);
            let file = std::fs::read_to_string(file_path.clone()).ok();
            ext = Some("html".to_string());
            let file = match file {
                Some(file) => file,
                None => {
                    let file = ServeOptions::search_up_path(file_path);
                    match file {
                        Ok(file) => file,
                        Err(err) => {
                            error!("SSR File Error: {:?}", err);
                            return Err(warp::reject::not_found());
                        }
                    }
                }
            };
            file
        } else {
            trace!("Explicit Extension: {:?}", file_path);
            ext = file_path.split(".").last().map(|s| s.to_string());
            let file = std::fs::read_to_string(file_path.clone());
            match file {
                Ok(file) => file,
                Err(err) => {
                    error!("SSR File Error: {:?}", err);
                    return Err(warp::reject::not_found());
                }
            }
        };

        Ok((formatted, ext))
    }

    pub async fn process_ssr(
        path: warp::filters::path::FullPath,
        body: serde_json::Value,
        search: serde_json::Value,
        ssr_options: Option<ServeSSR>,
        _headers: warp::http::HeaderMap,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        trace!("SSR Path: {:?}", path);
        trace!("SSR Body: {:?}", body);
        trace!("SSR Search: {:?}", search);

        // Serve a file located at the same path as the request path
        let file = format!(
            "{}{}",
            ssr_options.clone().unwrap_or_default().path,
            path.as_str()
        );

        trace!("SSR File: {:?}", file);

        // If the file does not have an extension, assume it is a directory and serve the index.html file
        let (file_path, ext) = ServeOptions::format_file_path(file)?;

        // If enable_ssr, hydrate the SSR template
        let enable_hydrate = ssr_options
            .clone()
            .unwrap_or_default()
            .enable_hydrate
            .unwrap_or_default();

        if enable_hydrate && ext == Some("html".to_string()) {
            // Hydrate the SSR template
            let query = if let Some(query) = body.get("query") {
                query.as_str()
            } else {
                let value = search.get("query");
                match value {
                    Some(v) => v.as_str(),
                    None => None,
                }
            };
            let operation_name = if let Some(operation_name) = body.get("operation_name") {
                operation_name.as_str()
            } else {
                let value = search.get("operation_name");
                match value {
                    Some(v) => v.as_str(),
                    None => None,
                }
            };
            let variables = if let Some(variables) = body.get("variables") {
                Some(variables)
            } else if let Some(search) = search.get("variables") {
                Some(search)
            } else {
                None
            };
            let props = if let Some(props) = body.get("props") {
                Some(props)
            } else if let Some(search) = search.get("props") {
                Some(search)
            } else {
                None
            };

            // Prepare the graphql request to inject into the SSR template
            if query.is_some() || operation_name.is_some() || variables.is_some() {
                if query.is_none() {
                    error!("SSR Hydrate Error: Query is required");
                    return Err(warp::reject::not_found());
                }
                if operation_name.is_none() {
                    error!("SSR Hydrate Error: Operation Name is required");
                    return Err(warp::reject::not_found());
                }
                if variables.is_none() {
                    error!("SSR Hydrate Error: Variables is required");
                    return Err(warp::reject::not_found());
                }
            }

            let mut context = None;
            if query.is_some() && operation_name.is_some() && variables.is_some() {
                trace!("SSR Hydrate Query: {:?}", query);
                trace!("SSR Hydrate Operation Name: {:?}", operation_name);
                trace!("SSR Hydrate Variables: {:?}", variables);
                let variables: serde_json::Value =
                    serde_json::from_str(variables.unwrap().as_str().unwrap()).unwrap();
                trace!("SSR Hydrate Variables: {:?}", variables);
                let variables = Variables::from_json(variables);
                trace!("SSR Hydrate Variables: {:?}", variables);
                let graphql_request = Request::new(query.unwrap())
                    .operation_name(operation_name.unwrap())
                    .variables(variables);
                let client = reqwest::Client::new();
                let response = client
                    //TODO: Make the graphql endpoint configurable
                    .post("http://localhost:3001/graphql")
                    .json(&graphql_request)
                    .send()
                    .await
                    .unwrap();
                // Check if is graphql error
                let response = response.json::<serde_json::Value>().await.map_err(|err| {
                    error!("SSR Hydrate Error: {:?}", err);
                    warp::reject::not_found() //TODO: Return a proper error
                })?;

                trace!("SSR Hydrate Response: {:?}", response);

                if response.get("errors").is_some() {
                    error!("SSR Hydrate Error: {:?}", response);
                    return Err(warp::reject::not_found()); //TODO: Return a proper error
                }

                context = Some(response.get("data").unwrap().clone());
            }

            if props.is_some() {
                trace!("SSR Hydrate Props: {:?}", props);
                // TODO: Check if props is an object
                let props: serde_json::Value =
                    serde_json::from_str(props.unwrap().as_str().unwrap()).unwrap();
                trace!("SSR Hydrate Props: {:?}", props);
                if context.is_none() {
                    context = Some(props);
                } else {
                    let mut context = context.as_ref().unwrap().clone();
                    let props = props.clone();
                    let context = context.as_object_mut().unwrap();
                    context.insert("props".to_string(), props.clone());
                }
            }

            let html = ServeOptions::handle_hydrate_ssr(file_path, ssr_options.clone(), context);
            match html {
                Ok(html) => Ok(Response::builder().body(html)),
                Err(err) => {
                    error!("SSR Hydrate Error: {:?}", err);
                    return Err(warp::reject::not_found());
                }
            }
        } else if enable_hydrate && ext != Some("html".to_string()) {
            warn!("SSR Hydrate Error: SSR Hydrate is only available for HTML files");
            return Ok(Response::builder().body(file_path));
        } else {
            // Return the file as is
            Ok(Response::builder().body(file_path))
        }
    }
}
