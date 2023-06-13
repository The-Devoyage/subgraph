use crate::{
    configuration::subgraph::data_sources::sql::DialectEnum,
    data_sources::sql::{SqlDataSource, SqlValueEnum},
};
use log::debug;

impl SqlDataSource {
    pub fn create_where_clause(
        where_keys: &Vec<String>,
        dialect: &DialectEnum,
        mut offset: Option<i32>,
        where_values: &Vec<SqlValueEnum>,
    ) -> String {
        debug!("Creating Where Clause");
        let parameterized_query = if !where_keys.is_empty() {
            let mut query = String::new();
            query.push_str(" WHERE ");

            for i in 0..where_keys.len() {
                query.push_str(&where_keys[i]);
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

                let index = if offset.is_some() {
                    Some(i as i32 + offset.unwrap())
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
                            _ => 0,
                        };

                        for j in 0..placeholder_count {
                            query.push_str(&SqlDataSource::get_placeholder(dialect, index));
                            if j != placeholder_count - 1 {
                                query.push_str(", ");
                            }
                        }
                        offset = Some(offset.unwrap_or(0) + placeholder_count as i32 - 1);
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
            query
        } else {
            String::new()
        };
        debug!("Where Clause: {}", parameterized_query);
        parameterized_query
    }
}
