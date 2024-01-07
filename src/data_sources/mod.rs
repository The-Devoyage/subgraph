use async_graphql::dynamic::FieldValue;
use bson::Document;
use log::debug;

use crate::{
    cli_args::CliArgs,
    configuration::subgraph::{
        data_sources::ServiceDataSourceConfig, entities::ServiceEntityConfig, SubGraphConfig,
    },
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
    subgraph_config: SubGraphConfig,
}

impl DataSources {
    /// Initialize Data Sources
    pub async fn init(
        service_data_source_configs: Vec<ServiceDataSourceConfig>,
        args: &CliArgs,
        subgraph_config: &SubGraphConfig,
    ) -> DataSources {
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
                    data_sources
                        .push(sql::SqlDataSource::init(&conf, args, subgraph_config.clone()).await);
                }
            };
        }

        DataSources {
            sources: data_sources,
            subgraph_config: subgraph_config.clone(),
        }
    }

    /// Provide entity and all data sources to get the data source for the entity.
    pub fn get_entity_data_soruce<'a>(
        data_sources: &'a DataSources,
        entity: &ServiceEntityConfig,
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

    pub fn get_data_source_by_name(data_soruces: &DataSources, name: &str) -> DataSource {
        debug!("Getting Data Source by Name");
        let data_source = data_soruces
            .sources
            .iter()
            .find(|data_source| match data_source {
                DataSource::Mongo(ds) => &ds.config.name == name,
                DataSource::HTTP(ds) => &ds.config.name == name,
                DataSource::SQL(ds) => &ds.config.name == name,
            })
            .unwrap();
        data_source.clone()
    }

    /// Execute a data source operation.
    pub async fn execute<'a>(
        data_sources: &DataSources,
        input: Document,
        entity: ServiceEntityConfig,
        resolver_type: ResolverType,
        subgraph_config: &SubGraphConfig,
    ) -> Result<Option<FieldValue<'a>>, async_graphql::Error> {
        debug!("Executing Datasource Operation");

        let cloned_entity = entity.clone();

        let data_source = DataSources::get_entity_data_soruce(data_sources, &entity);

        match data_source {
            DataSource::Mongo(_ds) => Ok(mongo::MongoDataSource::execute_operation(
                data_source,
                input,
                cloned_entity,
                resolver_type,
                subgraph_config,
            )
            .await?),
            DataSource::HTTP(_ds) => Ok(http::HttpDataSource::execute_operation(
                data_source,
                input,
                cloned_entity,
                resolver_type,
                subgraph_config,
            )
            .await?),
            DataSource::SQL(_ds) => Ok(sql::SqlDataSource::execute_operation(
                data_source,
                input,
                cloned_entity,
                resolver_type,
                subgraph_config,
            )
            .await?),
        }
    }
}
