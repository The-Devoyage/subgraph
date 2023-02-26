use async_graphql::dynamic::{FieldValue, ValueAccessor};
use log::{debug, info};

use crate::{
    configuration::subgraph::{data_sources::ServiceDataSourceConfig, entities::ServiceEntity},
    graphql::schema::ResolverType,
};

pub mod mongo;

#[derive(Debug)]
pub enum DataSource {
    Mongo(mongo::MongoDataSource),
}

#[derive(Debug)]
pub struct DataSources {
    sources: Vec<DataSource>,
}

impl DataSources {
    pub async fn init(service_data_source_configs: Vec<ServiceDataSourceConfig>) -> DataSources {
        let mut data_sources = vec![];
        for service_data_source_config in service_data_source_configs {
            match service_data_source_config {
                ServiceDataSourceConfig::Mongo(conf) => {
                    data_sources.push(mongo::MongoDataSource::init_mongo(&conf).await);
                }
            };
        }

        DataSources {
            sources: data_sources,
        }
    }

    pub async fn execute<'a>(
        data_sources: &DataSources,
        input: &ValueAccessor<'_>,
        entity: ServiceEntity,
        resolver_type: ResolverType,
    ) -> FieldValue<'a> {
        info!("Executing Datasource Operation");

        let cloned_entity = entity.clone();

        match entity.data_source.unwrap().from {
            Some(ds_name) => {
                let data_source = data_sources
                    .sources
                    .iter()
                    .find(|data_source| match data_source {
                        DataSource::Mongo(ds) => ds.config.name == ds_name,
                    })
                    .unwrap();

                info!("Matched Entity Data Source Configurtaion");
                debug!("{:?}", data_source);

                match data_source {
                    DataSource::Mongo(_ds) => {
                        mongo::MongoDataSource::execute_operation(
                            data_source,
                            input,
                            cloned_entity,
                            resolver_type,
                        )
                        .await
                    }
                }
            }
            //TODO: Finish
            None => FieldValue::owned_any("string".to_string()),
        }
    }
}
