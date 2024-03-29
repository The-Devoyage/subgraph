use crate::{
    configuration::subgraph::data_sources::sql::DialectEnum, data_sources::sql::SqlDataSource,
    filter_operator::FilterOperator, sql_value::SqlValue,
};
use log::{debug, error, trace};

impl SqlDataSource {
    pub fn create_where_clause(
        where_keys: &Vec<String>,
        dialect: &DialectEnum,
        mut pg_param_offset: Option<i32>,
        where_values: &Vec<SqlValue>,
        filter_operator: FilterOperator,
    ) -> Result<(String, i32), async_graphql::Error> {
        debug!("Creating Where Clause");
        trace!("Where Keys: {:?}", where_keys);
        trace!("Where Values: {:?}", where_values);
        let parameterized_query = if !where_keys.is_empty() {
            let mut query = String::new();

            for i in 0..where_keys.len() {
                query.push_str(&where_keys[i]);

                // If where_values[i] does not exist, return error.
                if where_values.len() <= i {
                    error!("Where value for key does not exist: {}", where_keys[i]);
                    return Err(async_graphql::Error::new(format!(
                        "Where value for key does not exist.",
                    )));
                }

                let is_list = match where_values[i] {
                    SqlValue::StringList(_) | SqlValue::IntList(_) | SqlValue::BoolList(_) => true,
                    _ => false,
                };

                if is_list {
                    query.push_str(" IN (");
                } else {
                    let sql_operator = FilterOperator::get_sql_operator(&filter_operator);
                    query.push_str(sql_operator);
                }

                // This is used to offset the placeholder index for postgres.
                // It is incremented by the number of placeholders added to the query.
                let index = if pg_param_offset.is_some() {
                    trace!("Existing Pg Param Offset: {:?}", pg_param_offset);
                    Some(i as i32 + pg_param_offset.unwrap())
                } else {
                    trace!("No Existing Pg Param Offset");
                    Some(0)
                };

                match where_values[i] {
                    SqlValue::StringList(_) | SqlValue::IntList(_) | SqlValue::BoolList(_) => {
                        let placeholder_count = match where_values[i] {
                            SqlValue::StringList(ref list) => list.len(),
                            SqlValue::IntList(ref list) => list.len(),
                            SqlValue::BoolList(ref list) => list.len(),
                            SqlValue::UUIDList(ref list) => list.len(),
                            SqlValue::DateTimeList(ref list) => list.len(),
                            _ => 0,
                        };

                        for j in 0..placeholder_count {
                            query.push_str(&SqlDataSource::get_placeholder(dialect, index));
                            if j != placeholder_count - 1 {
                                query.push_str(", ");
                            }
                        }

                        pg_param_offset =
                            Some(pg_param_offset.unwrap_or(0) + placeholder_count as i32 - 1);
                    }
                    _ => query.push_str(&SqlDataSource::get_placeholder(dialect, index)),
                };

                if is_list {
                    query.push_str(")");
                }

                if i != where_keys.len() - 1 {
                    query.push_str(" AND ");
                }
            }
            pg_param_offset = Some(pg_param_offset.unwrap_or(0) + where_keys.len() as i32);
            query
        } else {
            String::new()
        };
        trace!("Where Clause: {}", parameterized_query);
        trace!("Parameter Offset: {:?}", pg_param_offset.unwrap_or(0));
        Ok((parameterized_query, pg_param_offset.unwrap_or(0)))
    }
}
