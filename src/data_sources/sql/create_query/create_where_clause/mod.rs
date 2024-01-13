use crate::{
    configuration::subgraph::data_sources::sql::DialectEnum,
    data_sources::sql::{SqlDataSource, SqlValueEnum},
};
use log::{debug, error, trace};

impl SqlDataSource {
    pub fn create_where_clause(
        where_keys: &Vec<String>,
        dialect: &DialectEnum,
        mut pg_param_offset: Option<i32>,
        where_values: &Vec<SqlValueEnum>,
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
                    SqlValueEnum::StringList(_)
                    | SqlValueEnum::IntList(_)
                    | SqlValueEnum::BoolList(_) => true,
                    _ => false,
                };
                let operator = match is_list {
                    true => " IN (",
                    false => " = ",
                };
                query.push_str(operator);

                // This is used to offset the placeholder index for postgres.
                // It is incremented by the number of placeholders added to the query.
                let index = if pg_param_offset.is_some() {
                    Some(i as i32 + pg_param_offset.unwrap())
                } else {
                    Some(0)
                };

                match where_values[i] {
                    SqlValueEnum::StringList(_)
                    | SqlValueEnum::IntList(_)
                    | SqlValueEnum::BoolList(_) => {
                        let placeholder_count = match where_values[i] {
                            SqlValueEnum::StringList(ref list) => list.len(),
                            SqlValueEnum::IntList(ref list) => list.len(),
                            SqlValueEnum::BoolList(ref list) => list.len(),
                            SqlValueEnum::UUIDList(ref list) => list.len(),
                            SqlValueEnum::DateTimeList(ref list) => list.len(),
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
        Ok((parameterized_query, pg_param_offset.unwrap_or(0)))
    }
}
