use crate::{
    configuration::subgraph::data_sources::sql::DialectEnum,
    data_sources::sql::{SqlDataSource, SqlValueEnum},
};

impl SqlDataSource {
    pub fn create_update_one_query(
        table_name: &str,
        value_keys: &Vec<String>,
        dialect: &DialectEnum,
        where_keys: &Vec<String>,
        where_values: &Vec<SqlValueEnum>,
    ) -> String {
        let mut query = String::new();
        query.push_str("UPDATE ");
        query.push_str(table_name);
        query.push_str(" SET ");

        for i in 0..value_keys.len() {
            query.push_str(&value_keys[i]);
            query.push_str(" = ");
            query.push_str(SqlDataSource::get_placeholder(dialect, Some(i as i32)).as_str());
            if i != value_keys.len() - 1 {
                query.push_str(", ");
            }
        }

        query.push_str(" WHERE ");

        let parameterized_query =
            SqlDataSource::create_where_clause(where_keys, dialect, None, where_values);
        query.push_str(&parameterized_query);

        query.push_str(" LIMIT 1");

        if !query.ends_with(';') {
            query.push(';');
        }

        query
    }
}
