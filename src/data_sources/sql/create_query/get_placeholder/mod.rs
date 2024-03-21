use log::{debug, trace};

use crate::{
    configuration::subgraph::data_sources::sql::DialectEnum, data_sources::sql::SqlDataSource,
};

impl SqlDataSource {
    pub fn get_placeholder(dialect: &DialectEnum, index: Option<i32>) -> String {
        debug!("Creating Placeholder");
        trace!("Index: {:?}", index);
        trace!("Dialect: {:?}", dialect);
        let placeholder = match dialect {
            DialectEnum::POSTGRES => "$".to_string() + &(index.unwrap() + 1).to_string(),
            DialectEnum::MYSQL | DialectEnum::SQLITE => "?".to_string(),
        };
        trace!("Placeholder: {:?}", placeholder);
        placeholder
    }
}
