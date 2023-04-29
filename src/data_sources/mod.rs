use async_graphql::dynamic::{FieldValue, ValueAccessor};
use log::debug;

use crate::{
    configuration::subgraph::{data_sources::ServiceDataSourceConfig, entities::ServiceEntity},
    graphql::schema::ResolverType,
};

pub mod http;
pub mod mongo;
pub mod sql;

#[derive(Debug, Clone)]
pub enum DataSource {
    Mongo(mongo::MongoDataSource),
    HTTP(http::HttpDataSource),
    SQL(sql::SqlDataSource),
}

#[derive(Debug, Clone)]
pub struct DataSources {
    sources: Vec<DataSource>,
}

impl DataSources {
    /// Initialize Data Sources
    pub async fn init(service_data_source_configs: Vec<ServiceDataSourceConfig>) -> DataSources {
        debug!("Initializing Data Sources");
        let mut data_sources = vec![];
        for service_data_source_config in service_data_source_configs {
            match service_data_source_config {
                ServiceDataSourceConfig::Mongo(conf) => {
                    data_sources.push(mongo::MongoDataSource::init(&conf).await);
                }
                ServiceDataSourceConfig::HTTP(conf) => {
                    data_sources.push(http::HttpDataSource::init(&conf).await);
                }
                ServiceDataSourceConfig::SQL(conf) => {
                    data_sources.push(sql::SqlDataSource::init(&conf).await);
                }
            };
        }

        DataSources {
            sources: data_sources,
        }
    }

    pub fn get_data_source_for_entity<'a>(
        data_sources: &'a DataSources,
        entity: &ServiceEntity,
    ) -> &'a DataSource {
        debug!("Getting Data Source for Entity");
        if entity.data_source.is_some() {
            let data_source = match entity.data_source.as_ref().unwrap().from.as_ref() {
                Some(ds_name) => {
                    let data_source = data_sources
                        .sources
                        .iter()
                        .find(|data_source| match data_source {
                            DataSource::Mongo(ds) => &ds.config.name == ds_name,
                            DataSource::HTTP(ds) => &ds.config.name == ds_name,
                            DataSource::SQL(ds) => &ds.config.name == ds_name,
                        })
                        .unwrap();
                    data_source
                }
                _ => panic!("Data source specified for entity but not found."),
            };
            data_source
        } else {
            data_sources.sources.first().unwrap()
        }
    }

    /// Execute a data source operation.
    pub async fn execute<'a>(
        data_sources: &DataSources,
        input: &ValueAccessor<'_>,
        entity: ServiceEntity,
        resolver_type: ResolverType,
    ) -> Result<FieldValue<'a>, async_graphql::Error> {
        debug!("Executing Datasource Operation");

        let cloned_entity = entity.clone();

        let data_source = DataSources::get_data_source_for_entity(data_sources, &entity);

        match data_source {
            DataSource::Mongo(_ds) => Ok(mongo::MongoDataSource::execute_operation(
                data_source,
                input,
                cloned_entity,
                resolver_type,
            )
            .await?),
            DataSource::HTTP(_ds) => Ok(http::HttpDataSource::execute_operation(
                data_source,
                input,
                cloned_entity,
                resolver_type,
            )
            .await?),
            DataSource::SQL(_ds) => Ok(sql::SqlDataSource::execute_operation(
                data_source,
                input,
                cloned_entity,
                resolver_type,
            )
            .await?),
        }
    }
}
