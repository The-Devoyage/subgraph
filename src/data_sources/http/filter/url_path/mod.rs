use async_graphql::dynamic::ValueAccessor;
use bson::Document;
use log::{debug, info};
use reqwest::Url;

use crate::configuration::subgraph::entities::ServiceEntity;
use crate::configuration::subgraph::entities::ServiceEntityResolver::{
    CreateOne, FindMany, FindOne,
};
use crate::data_sources::http::HttpDataSource;
use crate::graphql::schema::ResolverType;

impl HttpDataSource {
    pub async fn create_parameratized_path(
        mut url: Url,
        entity: &ServiceEntity,
        resolver_type: ResolverType,
    ) -> Result<Url, async_graphql::Error> {
        info!("Creating Parameratized Path");
        debug!("For Resolver Type: {:?}", resolver_type);

        let entity_path = entity.data_source.as_ref().unwrap().path.as_ref().unwrap();

        debug!("Entity Path Defined: {:?}", entity_path);

        url.set_path(entity_path);

        debug!("Set Path: {:?}", url);

        debug!(
            "Entity Resolver Config: {:?}",
            entity
                .data_source
                .as_ref()
                .unwrap()
                .resolvers
                .as_ref()
                .unwrap()
        );
        let entity_resolvers = entity
            .data_source
            .as_ref()
            .unwrap()
            .resolvers
            .as_ref()
            .unwrap();

        url = match resolver_type {
            ResolverType::FindOne => {
                let entity_resolver = entity_resolvers.iter().find(|resolver| match resolver {
                    FindOne(_find_one_resolver) => true,
                    _ => false,
                });

                if entity_resolver.is_some() {
                    let find_one_resolver = match entity_resolver.unwrap() {
                        FindOne(find_one_resolver) => find_one_resolver,
                        _ => unreachable!(),
                    };

                    debug!(
                        "Resolver Path Defined: {:?}",
                        find_one_resolver.path.as_ref().unwrap()
                    );
                    debug!("Current URL: {:?}", url);
                    let resolver_path = find_one_resolver.path.as_ref().unwrap();
                    let path = format!("{}{}", url.path(), resolver_path);

                    url.set_path(&path);
                    return Ok(url);
                }
                url
            }
            ResolverType::FindMany => {
                let entity_resolver = entity_resolvers.iter().find(|resolver| match resolver {
                    FindMany(_find_one_resolver) => true,
                    _ => false,
                });

                if entity_resolver.is_some() {
                    let find_many_resolver = match entity_resolver.unwrap() {
                        FindMany(find_many_resolver) => find_many_resolver,
                        _ => panic!("Unable To Locate Find Many Resolver Data Source Config"),
                    };

                    debug!("Current URL: {:?}", url);
                    let resolver_path = find_many_resolver.path.as_ref();

                    if resolver_path.is_some() {
                        let path = format!("{}{}", url.path(), resolver_path.unwrap());
                        url.set_path(&path);
                    }
                    return Ok(url);
                }
                url
            }
            ResolverType::CreateOne => {
                let entity_resolver = entity_resolvers.iter().find(|resolver| match resolver {
                    CreateOne(_create_one_resolver) => true,
                    _ => false,
                });

                if entity_resolver.is_some() {
                    let create_one_resolver = match entity_resolver.unwrap() {
                        CreateOne(create_one_resolver) => create_one_resolver,
                        _ => panic!("Unable To Locate Create One Resolver Data Source Config"),
                    };

                    debug!("Current URL: {:?}", url);
                    let resolver_path = create_one_resolver.path.as_ref();

                    if resolver_path.is_some() {
                        let path = format!("{}{}", url.path(), resolver_path.unwrap());
                        url.set_path(&path);
                    }
                    return Ok(url);
                }
                url
            }
        };
        debug!("Created Parameratized Path, {:?}", url);
        Ok(url)
    }

    pub async fn create_path_filters(
        url: Url,
        input: &ValueAccessor<'_>,
    ) -> Result<Url, async_graphql::Error> {
        debug!("Creating Path Filters");

        let mut path_segments = url.path_segments().ok_or_else(|| "URL Has no path.")?;
        let document = input.deserialize::<Document>()?;
        debug!("Deserialized Input {:?}", document);
        let mut url = Url::parse(url.as_str())?;

        while let Some(path_segment) = path_segments.next() {
            debug!("Path Segment: {:?}", path_segment);

            if let Some(identifier) = path_segment.chars().nth(0) {
                if identifier.to_string() == ":" {
                    let mut chars = path_segment.chars();
                    chars.next();
                    let param = document.get(chars.as_str());
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
