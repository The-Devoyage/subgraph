use crate::{
    configuration::subgraph::data_sources::sql::DialectEnum,
    data_sources::sql::{SqlDataSource, SqlValueEnum},
};

impl SqlDataSource {
    pub fn create_find_many_query(
        table_name: &str,
        where_keys: &Vec<String>,
        dialect: &DialectEnum,
        where_values: &Vec<SqlValueEnum>,
    ) -> String {
        let mut query = String::new();
        query.push_str("SELECT * FROM ");
        query.push_str(table_name);

        let parameterized_query =
            SqlDataSource::create_where_clause(where_keys, dialect, None, where_values);
        query.push_str(&parameterized_query);

        if !query.ends_with(';') {
            query.push(';');
        }

        query
    }
}
