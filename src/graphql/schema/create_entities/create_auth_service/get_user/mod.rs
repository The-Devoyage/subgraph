use bson::doc;
use log::debug;

use crate::{
    configuration::subgraph::data_sources::sql::DialectEnum,
    data_sources::{sql::PoolEnum, DataSource},
    graphql::schema::{create_entities::create_auth_service::ServiceUser, ServiceSchemaBuilder},
};

impl ServiceSchemaBuilder {
    pub async fn get_user(data_source: &DataSource, identifier: &str) -> bool {
        let user_exists = match &data_source {
            DataSource::Mongo(mongo_ds) => {
                debug!("Mongo data source");
                let filter = doc! {
                    "identifier": &identifier
                };

                let user = mongo_ds
                    .db
                    .collection::<ServiceUser>("subgraph_user")
                    .find_one(filter, None)
                    .await;

                user.is_ok()
            }
            DataSource::SQL(sql_ds) => {
                debug!("SQL data source");
                let user_exists = match sql_ds.config.dialect {
                    DialectEnum::MYSQL => {
                        let query =
                            sqlx::query("SELECT * FROM subgraph_user WHERE identifier = ?;")
                                .bind(&identifier);

                        let user_exists = match sql_ds.pool.clone() {
                            PoolEnum::MySql(pool) => query.fetch_one(&pool).await.is_ok(),
                            _ => unreachable!(),
                        };
                        user_exists
                    }
                    DialectEnum::SQLITE => {
                        let query =
                            sqlx::query("SELECT * FROM subgraph_user WHERE identifier = ?;")
                                .bind(&identifier);

                        let user_exists = match sql_ds.pool.clone() {
                            PoolEnum::SqLite(pool) => query.fetch_one(&pool).await.is_ok(),
                            _ => panic!("Pool not supported."),
                        };

                        debug!("SQLITE User exists: {}", user_exists);

                        user_exists
                    }
                    DialectEnum::POSTGRES => {
                        let query =
                            sqlx::query("SELECT * FROM subgraph_user WHERE identifier = $1;")
                                .bind(&identifier);

                        let user_exists = match sql_ds.pool.clone() {
                            PoolEnum::Postgres(pool) => query.fetch_one(&pool).await.is_ok(),
                            _ => panic!("Pool not supported."),
                        };
                        user_exists
                    }
                };
                user_exists
            }
            _ => panic!("Data Source not supported."),
        };

        user_exists
    }
}
