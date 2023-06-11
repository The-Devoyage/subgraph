use crate::{
    configuration::subgraph::data_sources::sql::DialectEnum, data_sources::sql::SqlDataSource,
};

impl SqlDataSource {
    pub fn create_create_one_query(
        table_name: &str,
        value_keys: &Vec<String>,
        dialect: &DialectEnum,
    ) -> String {
        let mut query = String::new();
        query.push_str("INSERT INTO ");
        query.push_str(table_name);
        query.push_str(" (");

        for i in 0..value_keys.len() {
            query.push_str(&value_keys[i]);
            if i != value_keys.len() - 1 {
                query.push_str(", ");
            }
        }

        query.push_str(") VALUES (");
        for i in 0..value_keys.len() {
            query.push_str(SqlDataSource::get_placeholder(dialect, Some(i as i32)).as_str());
            if i != value_keys.len() - 1 {
                query.push_str(", ");
            }
        }
        query.push_str(")");

        match dialect {
            DialectEnum::POSTGRES => {
                query.push_str(" RETURNING *");
            }
            _ => {}
        }

        if !query.ends_with(';') {
            query.push(';');
        }
        query
    }
}
