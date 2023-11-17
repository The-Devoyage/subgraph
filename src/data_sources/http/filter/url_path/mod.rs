use bson::{to_document, Document};
use log::{debug, info};
use reqwest::Url;

use crate::configuration::subgraph::entities::ServiceEntityConfig;
use crate::data_sources::http::HttpDataSource;
use crate::graphql::schema::ResolverType;

impl HttpDataSource {
    pub async fn create_parameratized_path(
        mut url: Url,
        entity: &ServiceEntityConfig,
        resolver_type: ResolverType,
    ) -> Result<Url, async_graphql::Error> {
        info!("Creating Parameratized Path");
        debug!("For Resolver Type: {:?}", resolver_type);

        let entity_path = entity.data_source.as_ref().unwrap().path.as_ref().unwrap();

        debug!("Entity Path Defined: {:?}", entity_path);

        url.set_path(entity_path);

        debug!("Set Path: {:?}", url);

        let entity_resolvers = ServiceEntityConfig::get_resolvers(entity.clone());

        if entity_resolvers.is_none() {
            return Ok(url);
        }

        url = match resolver_type {
            ResolverType::FindOne => {
                let find_one_resolver = entity_resolvers.as_ref().unwrap().find_one.as_ref();

                if find_one_resolver.is_none() {
                    return Ok(url);
                }

                debug!("Current URL: {:?}", url);

                let resolver_path = find_one_resolver.unwrap().path.as_ref();

                if resolver_path.is_none() {
                    return Ok(url);
                }

                debug!(
                    "Resolver Path Defined: {:?}",
                    find_one_resolver.unwrap().path.as_ref()
                );

                let path = format!("{}{}", url.path(), resolver_path.unwrap());
                url.set_path(&path);
                url
            }
            ResolverType::FindMany => {
                let find_many_resolver = entity_resolvers.as_ref().unwrap().find_many.as_ref();

                if find_many_resolver.is_none() {
                    return Ok(url);
                }

                debug!("Current URL: {:?}", url);

                let resolver_path = find_many_resolver.unwrap().path.as_ref();

                if resolver_path.is_none() {
                    return Ok(url);
                }

                debug!(
                    "Resolver Path Defined: {:?}",
                    find_many_resolver.unwrap().path.as_ref()
                );

                let path = format!("{}{}", url.path(), resolver_path.unwrap());
                url.set_path(&path);
                url
            }
            ResolverType::CreateOne => {
                let create_one_resolver = entity_resolvers.as_ref().unwrap().create_one.as_ref();

                if create_one_resolver.is_none() {
                    return Ok(url);
                }

                debug!("Current URL: {:?}", url);

                let resolver_path = create_one_resolver.unwrap().path.as_ref();

                if resolver_path.is_none() {
                    return Ok(url);
                }

                debug!(
                    "Resolver Path Defined: {:?}",
                    create_one_resolver.unwrap().path.as_ref()
                );

                let path = format!("{}{}", url.path(), resolver_path.unwrap());
                url.set_path(&path);
                url
            }
            ResolverType::UpdateOne => {
                let update_one_resolver = entity_resolvers.as_ref().unwrap().update_one.as_ref();

                if update_one_resolver.is_none() {
                    return Ok(url);
                }

                debug!("Current URL: {:?}", url);

                let resolver_path = update_one_resolver.unwrap().path.as_ref();

                if resolver_path.is_none() {
                    return Ok(url);
                }

                debug!(
                    "Resolver Path Defined: {:?}",
                    update_one_resolver.unwrap().path.as_ref()
                );

                let path = format!("{}{}", url.path(), resolver_path.unwrap());
                url.set_path(&path);
                url
            }
            ResolverType::UpdateMany => {
                let update_many_resolver = entity_resolvers.as_ref().unwrap().update_many.as_ref();

                if update_many_resolver.is_none() {
                    return Ok(url);
                }

                debug!("Current URL: {:?}", url);

                let resolver_path = update_many_resolver.unwrap().path.as_ref();

                if resolver_path.is_none() {
                    return Ok(url);
                }

                debug!(
                    "Resolver Path Defined: {:?}",
                    update_many_resolver.unwrap().path.as_ref()
                );

                let path = format!("{}{}", url.path(), resolver_path.unwrap());
                url.set_path(&path);
                url
            }
            _ => panic!("Invalid resolver type"),
        };
        Ok(url)
    }

    pub async fn create_path_filters(
        url: Url,
        mut input: Document,
        resolver_type: ResolverType,
    ) -> Result<Url, async_graphql::Error> {
        debug!("Creating Path Filters");

        let mut path_segments = url.path_segments().ok_or_else(|| "URL Has no path.")?;
        let mut url = Url::parse(url.as_str())?;

        input = match resolver_type {
            ResolverType::FindOne
            | ResolverType::FindMany
            | ResolverType::UpdateOne
            | ResolverType::UpdateMany => to_document(input.get("query").unwrap())?,
            ResolverType::CreateOne => return Ok(url),
            _ => Err(async_graphql::Error::new("Invalid resolver type"))?,
        };

        while let Some(path_segment) = path_segments.next() {
            debug!("Path Segment: {:?}", path_segment);

            if let Some(identifier) = path_segment.chars().nth(0) {
                if identifier.to_string() == ":" {
                    let mut chars = path_segment.chars();
                    chars.next();
                    let param = input.get(chars.as_str());
                    debug!("Param Found: {:?}", param);
                    if param.is_some() {
                        url = url.join(&param.unwrap().to_string()).unwrap_or(url.clone());
                    }
                } else {
                    url = url.join(path_segment).unwrap_or(url.clone());
                }
            }
        }

        debug!("Url: {:?}", url);

        Ok(url)
    }
}
