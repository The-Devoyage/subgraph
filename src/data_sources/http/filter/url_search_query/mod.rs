use async_graphql::dynamic::ValueAccessor;
use bson::Document;
use log::{debug, info};
use reqwest::Url;

use crate::configuration::subgraph::entities::ServiceEntityResolver::{
    CreateOne, FindMany, FindOne,
};
use crate::{
    configuration::subgraph::entities::ServiceEntity, data_sources::http::HttpDataSource,
    graphql::schema::ResolverType,
};

impl HttpDataSource {
    pub async fn create_parameratized_search_query(
        mut url: Url,
        entity: &ServiceEntity,
        resolver_type: ResolverType,
    ) -> Result<Url, async_graphql::Error> {
        info!("Creating Parameterized Search Query");

        let entity_search_query_pairs = entity.data_source.as_ref().unwrap().search_query.as_ref();
        debug!("Looking For Query Pairs: {:?}", entity_search_query_pairs);

        if entity_search_query_pairs.is_some() {
            let entity_search_query_pairs = entity_search_query_pairs.unwrap();

            debug!(
                "Entity Search Query Pairs Defined: {:?}",
                entity_search_query_pairs
            );

            for query_pair in entity_search_query_pairs {
                debug!("Adding Query Pair: {:?}", query_pair);
                url.query_pairs_mut()
                    .append_pair(&query_pair.0, &query_pair.1);
            }
        }

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
                        "Resolver Query Pairs Defined: {:?}",
                        find_one_resolver.search_query.as_ref().unwrap()
                    );
                    debug!("Current URL: {:?}", url);
                    let resolver_query_pairs = find_one_resolver.search_query.as_ref().unwrap();

                    for query_pair in resolver_query_pairs {
                        url.query_pairs_mut()
                            .append_pair(&query_pair.0, &query_pair.1);
                    }

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
                    let find_one_resolver = match entity_resolver.unwrap() {
                        FindMany(find_one_resolver) => find_one_resolver,
                        _ => unreachable!(),
                    };

                    debug!(
                        "Resolver Query Pairs Defined: {:?}",
                        find_one_resolver.search_query.as_ref().unwrap()
                    );
                    debug!("Current URL: {:?}", url);
                    let resolver_query_pairs = find_one_resolver.search_query.as_ref().unwrap();

                    for query_pair in resolver_query_pairs {
                        url.query_pairs_mut()
                            .append_pair(&query_pair.0, &query_pair.1);
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
                        _ => unreachable!(),
                    };

                    debug!(
                        "Resolver Query Pairs Defined: {:?}",
                        create_one_resolver.search_query.as_ref().unwrap()
                    );

                    let resolver_query_pairs = create_one_resolver.search_query.as_ref().unwrap();

                    for query_pair in resolver_query_pairs {
                        url.query_pairs_mut()
                            .append_pair(&query_pair.0, &query_pair.1);
                    }

                    return Ok(url);
                }
                url
            }
        };
        Ok(url)
    }

    pub async fn replace_identifier(
        identifier_variable: String,
        input: &Document,
    ) -> Result<String, async_graphql::Error> {
        info!("Replacing Identifier");
        debug!("Identifier Variable {:?}", identifier_variable);

        if let Some(identifier) = identifier_variable.chars().nth(0) {
            debug!("Identifier: {:?}", identifier);

            if identifier.to_string() == ":" {
                debug!("Replacing Identifier");
                let mut chars = identifier_variable.chars();
                chars.next();
                let param = input.get(chars.as_str());
                debug!("Param: {:?}", param);

                if param.is_some() {
                    debug!("Returning Replaced Identifier");
                    Ok(param.unwrap().to_string())
                } else {
                    debug!("No Param Found, Returning Original Identifier");
                    Ok(identifier_variable)
                }
            } else {
                debug!("Not Valid Identifier, Returning Original Identifier");
                Ok(identifier_variable)
            }
        } else {
            debug!("Valid Identifier, Returning Original Identifier");
            Ok(identifier_variable)
        }
    }

    pub async fn create_query_string_filters(
        mut url: Url,
        input: &ValueAccessor<'_>,
    ) -> Result<Url, async_graphql::Error> {
        debug!("Creating Query String Filters");

        let url_cloned = Url::parse(url.as_str()).unwrap();

        let mut query_pairs = url_cloned.query_pairs();

        let document = input.deserialize::<Document>()?;
        debug!("Deserialized Input: {:?}", document);

        url.query_pairs_mut().clear();
        let mut url = Url::parse(url.as_str())?;

        while let Some(query_pair) = query_pairs.next() {
            debug!("Query Pair: {:?}", query_pair);
            let name =
                HttpDataSource::replace_identifier(query_pair.0.to_string(), &document).await?;
            let value =
                HttpDataSource::replace_identifier(query_pair.1.to_string(), &document).await?;

            url.query_pairs_mut().append_pair(&name, &value);
        }

        Ok(url)
    }
}
