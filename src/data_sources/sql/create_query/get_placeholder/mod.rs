use crate::{
    configuration::subgraph::data_sources::sql::DialectEnum, data_sources::sql::SqlDataSource,
};

impl SqlDataSource {
    pub fn get_placeholder(dialect: &DialectEnum, index: Option<i32>) -> String {
        match dialect {
            DialectEnum::POSTGRES => "$".to_string() + &(index.unwrap() + 1).to_string(),
            DialectEnum::MYSQL | DialectEnum::SQLITE => "?".to_string(),
        }
    }
}
