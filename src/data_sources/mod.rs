use crate::configuration::subgraph::data_sources::ServiceDataSourceConfig;

pub mod mongo;

pub enum DataSource {
    Mongo(mongo::MongoDataSource),
}

pub struct DataSources {
    sources: Vec<DataSource>,
}

impl DataSources {
    pub async fn init(service_data_source_configs: Vec<ServiceDataSourceConfig>) -> DataSources {
        let mut data_sources = vec![];
        for service_data_source_config in service_data_source_configs {
            match service_data_source_config {
                ServiceDataSourceConfig::MongoDataSourceConfig(conf) => {
                    data_sources.push(mongo::MongoDataSource::init_mongo(&conf).await);
                }
            };
        }

        DataSources {
            sources: data_sources,
        }
    }
}
