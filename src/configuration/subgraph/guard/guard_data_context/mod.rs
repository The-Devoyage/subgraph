use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
/// The GuardDataContext is used to define the context for a guard.
/// It queries the database and returns the result as a context key.
/// The context key can then be used in the guard to check if the user has access to the resource.
/// [[entities.data_source.resolvers.update_many.guards.context]]
/// entity_name = "todo_access"
/// query = '''
///  {
///    "get_todo_accesss_input": {
///      "query": {
///       "AND": {{uuid}}
///      }
///    }
///  }
/// '''
/// variables = [
///  ["{{uuid}}", "input(\"query\", \"uuid\")"],
/// ]

pub struct GuardDataContext {
    /// The name of the entity to be queried.
    pub entity_name: String,
    /// The name given to the context key. Defaults to entity_name if excluded.
    pub name: Option<String>,
    /// The graphql query to be executed.
    pub query: String,
    /// The variables to be used in the query.
    /// Used to replace the variables in the query.
    pub variables: Vec<VariablePair>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VariablePair(pub String, pub String);
