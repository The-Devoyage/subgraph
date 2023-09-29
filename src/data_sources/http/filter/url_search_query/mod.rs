use bson::Document;
use log::{debug, info};
use reqwest::Url;

use crate::{
    configuration::subgraph::entities::ServiceEntityConfig, data_sources::http::HttpDataSource,
    graphql::schema::ResolverType,
};

impl HttpDataSource {
    pub async fn create_parameratized_search_query(
        mut url: Url,
        entity: &ServiceEntityConfig,
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

                let search_query = find_one_resolver.unwrap().search_query.as_ref();

                if search_query.is_none() {
                    return Ok(url);
                }

                debug!(
                    "Resolver Query Pairs Defined: {:?}",
                    find_one_resolver.unwrap().search_query.as_ref().unwrap()
                );

                debug!("Current URL: {:?}", url);
                let resolver_query_pairs = search_query.as_ref().unwrap();

                for query_pair in resolver_query_pairs.iter() {
                    url.query_pairs_mut()
                        .append_pair(&query_pair.0, &query_pair.1);
                }

                url
            }
            ResolverType::FindMany => {
                let find_many_resolver = entity_resolvers.as_ref().unwrap().find_many.as_ref();

                if find_many_resolver.is_none() {
                    return Ok(url);
                }

                let search_query = find_many_resolver.unwrap().search_query.as_ref();

                if search_query.is_none() {
                    return Ok(url);
                }

                debug!(
                    "Resolver Query Pairs Defined: {:?}",
                    search_query.as_ref().unwrap()
                );

                debug!("Current URL: {:?}", url);

                let resolver_query_pairs = search_query.as_ref().unwrap();

                for query_pair in resolver_query_pairs.iter() {
                    url.query_pairs_mut()
                        .append_pair(&query_pair.0, &query_pair.1);
                }

                url
            }
            ResolverType::CreateOne => {
                let create_one_resolver = entity_resolvers.as_ref().unwrap().create_one.as_ref();

                if create_one_resolver.is_none() {
                    return Ok(url);
                }

                let search_query = create_one_resolver.unwrap().search_query.as_ref();

                if search_query.is_none() {
                    return Ok(url);
                }

                debug!(
                    "Resolver Query Pairs Defined: {:?}",
                    search_query.as_ref().unwrap()
                );

                let resolver_query_pairs = search_query.as_ref().unwrap();

                for query_pair in resolver_query_pairs.iter() {
                    url.query_pairs_mut()
                        .append_pair(&query_pair.0, &query_pair.1);
                }

                url
            }
            ResolverType::UpdateOne => {
                let update_one_resolver = entity_resolvers.as_ref().unwrap().update_one.as_ref();

                if update_one_resolver.is_none() {
                    return Ok(url);
                }

                let search_query = update_one_resolver.unwrap().search_query.as_ref();

                if search_query.is_none() {
                    return Ok(url);
                }

                debug!(
                    "Resolver Query Pairs Defined: {:?}",
                    search_query.as_ref().unwrap()
                );

                let resolver_query_pairs = search_query.as_ref().unwrap();

                for query_pair in resolver_query_pairs.iter() {
                    url.query_pairs_mut()
                        .append_pair(&query_pair.0, &query_pair.1);
                }

                url
            }
            ResolverType::UpdateMany => {
                let update_many_resolver = entity_resolvers.as_ref().unwrap().update_many.as_ref();

                if update_many_resolver.is_none() {
                    return Ok(url);
                }

                let search_query = update_many_resolver.unwrap().search_query.as_ref();

                if search_query.is_none() {
                    return Ok(url);
                }

                debug!(
                    "Resolver Query Pairs Defined: {:?}",
                    search_query.as_ref().unwrap()
                );

                let resolver_query_pairs = search_query.as_ref().unwrap();

                for query_pair in resolver_query_pairs.iter() {
                    url.query_pairs_mut()
                        .append_pair(&query_pair.0, &query_pair.1);
                }

                url
            }
            _ => panic!("Invalid resolver type"),
        };
        Ok(url)
    }

    pub async fn replace_identifier(
        identifier_variable: String,
        input: &Document,
    ) -> Result<Option<String>, async_graphql::Error> {
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
                    Ok(Some(param.unwrap().to_string()))
                } else {
                    debug!("No Param Found, Returning None Identifier");
                    Ok(None)
                }
            } else {
                debug!("Not Valid Identifier, Returning Original Identifier");
                Ok(Some(identifier_variable))
            }
        } else {
            debug!("Valid Identifier, Returning Original Identifier");
            Ok(Some(identifier_variable))
        }
    }

    pub async fn create_query_string_filters(
        mut url: Url,
        input: Document,
    ) -> Result<Url, async_graphql::Error> {
        debug!("Creating Query String Filters");

        let url_cloned = Url::parse(url.as_str()).unwrap();

        let mut query_pairs = url_cloned.query_pairs();

        url.query_pairs_mut().clear();
        let mut url = Url::parse(url.as_str())?;

        while let Some(query_pair) = query_pairs.next() {
            debug!("Query Pair: {:?}", query_pair);
            let value =
                HttpDataSource::replace_identifier(query_pair.1.to_string(), &input).await?;

            if value.is_none() {
                continue;
            }

            url.query_pairs_mut()
                .append_pair(&query_pair.0.to_string(), &value.unwrap());
        }

        Ok(url)
    }
}
