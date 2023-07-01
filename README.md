# @The-Devoyage/subgraph

Currently, a POC written in Rust in order to dynamically generate a functional API from a simple configuration/schema.

Not yet for production use - There is still a bit of work to get to v0.1.0! That being said, I hope that you enjoy what has been made so far.

## Quick Start

Define a configuration to run the service. The configuration tells subgraph how to generate the API around the data you need.

### 1. Download or Clone

You can download binary from the [Releases Page](https://github.com/The-Devoyage/subgraph/releases) or clone the repo to use cargo to run.

### 2. Define your configuration file.

Check out some [Example Configurations](examples) for SqLite, MySQL, Postgres, HTTP and other use cases.

```toml
[service]
service_name = "dogs"
port = 5011

[[service.data_sources]]
[service.data_sources.Mongo]
name = "dogs_db"
uri = "mongodb://user:password@127.0.0.1:27017/db_name"
db = "db_name"

[[service.entities]]
name = "Dog"
fields = [
    { name = "_id", scalar = "ObjectID", required = true },
    { name = "name", scalar = "String", required = true },
    { name = "age", scalar = "Int" },
    { name = "is_love", scalar = "Boolean"}
]
```

### 3. Start the Service

From binary release:

```bash
subgraph --config ./path-to-config.toml
```

From repo

```bash
cargo run -- -c ./config.toml
```

### 4. Start Querying

Use the GraphQL Sandbox runs on the specified port (the option defined in the CLI, `-p 5011`).

```
# In the browser:
http://localhost:5011
```

Start sending Queries and Mutations to the `/graphql` endpoint from your application.

## Features

### Simple Schema

Simple TOML configuration to define the entities to be resolved.

### CRUD

Resolvers are created for each defined entity allowing you to find and manipulate the data you need.

- Find One
- Find Many
- Create One
- Update One
- Update Many

\*Note, Update One Resolvers are excluded from Postgres and SQLite Data Sources as a true `LIMIT 1` query is invalid syntax when using these dialects.
\*Note, Mongo DB will upsert update documents.

### Data Sources

Connect many data sources to a single API. Supports multiple instances of every data source (for example 2 mongo data sources and 3 http data sources).

- Mongo DB Data Source - Connect mongo database(s).
- Postgres Data Source - Connect Postgres Database(s).
- MySQL Data Source - Connect MySQL Database(s).
- SQLite Data Source - Connecct to SQLite Database(s).
- HTTP Data Source - Integrate external Restful API(s).

### Guards - Security

Protecting the API is simple and can be done by adding `Guards` in the configuration file. Guards are quick to write expressions that if evaluated truthy result in a 403 like error (or custom error).

- Access to common datapoints such as input values and header values.
- Easy to implement and logical using the [EvalExpr Crate](https://docs.rs/evalexpr/latest/evalexpr/) Syntax.
- Custom error message support.
- Guard at different points in the request lifecycle (before, during, after request).

### Sandbox

Once started, view the sandbox in the browser hosted at the specified port. For example `http://localhost:5011`.

- View the generated schema using the schema tab.
- Write and execute GraphQL queries in the playground.

## Usage - The Config File

Using subgraph is simple - Define a configuration and start the server.

### The Service Name

Define the service at the top of the config.

```toml
[service]
service_name = "demo"
```

### Defining Data Sources

You must define at least one Data Source. See the `Data Source Enum` table in the `API` section of this readme for supported Data Sources. You may define multiple Data Sources.

```toml
[service]
service_name = "demo"

[[service.data_sources]]
[service.data_sources.Mongo]
name = "mongo_1"
uri = "mongodb://user:pass127.0.0.1:27017/db_name"
db = "local_db"

[[service.data_sources]]
[service.data_sources.SQL]
name = "espresso_db"
uri = "sqlite:/home/nickisyourfan/Desktop/DEV/dbs/espresso.db"
dialect = "SQLITE"

[[service.data_sources]]
[service.data_sources.HTTP]
name = "todos"
url = "https://jsonplaceholder.typicode.com"
default_headers = [{ name = "Authorization", value = "Bearer $OPENAI_KEY" }]
```

### Defining Entities

Entities are the assets returned from the data source. You may define multiple entities. Entities require field definitions to describe the properties of the entity.

```toml
[[service.entities]]
name = "Person"
fields = [
    { name = "_id", scalar = "ObjectID", required = true },
    { name = "name", scalar = "String", required = true },
    { name = "age", scalar = "Int" },
    { name = "friends", scalar = "String", list = true }
]
```

Fields may be nested using Object Scalars. See a full list of available scalars within the API Section of this README.

```toml
[[service.entities.fields]]
name = "usage"
scalar = "Object"
required = true
fields = [
    { name = "prompt_tokens", scalar = "Int", required = true },
    { name = "completion_tokens", scalar = "Int", required = true },
    { name = "total_tokens", scalar = "Int", required = true },
]
```

**Entity Data Source**

If not defined, entities are associated with the first data source in the config file but can be assigned to any data source.

The `from` field is associated with the `name` of the data source associated with the entity.

```toml
[[service.entities]]
name = "Person"

[service.entities.data_source]
from = "secondary_data_source" # The name of the data source to associate with.
collection = "users" # For use with Mongo Data Source
path = "/users" # For use with HTTP Data Source to define the endpoint relative to the url of the associated HTTP Data Source.

[service.entities.data_source.resolvers]
[service.entities.data_source.resolvers.find_one]
path = "/:id"  # Converts the ID property from the GraphQL Input into the ID Path Parameter for HTTP Data Sources.
[service.entities.data_source.resolvers.find_many]
search_query = [["userId", ":userId"], ["completed", ":completed"]] #Append Search Query to Path for HTTP Data Sources.

[[service.entities.fields]]
name = "_id"
scalar = "ObjectID"
required = true

[[service.entities.fields]]
name = "name"
scalar = "String"
required = true
```

**HTTP Data Source - Parameterized Variables**

As you can see above, defining a path or search query can be done at the entity level and at the resolver level, allowing you to customize the endpoint/search query to be used.

1. Specify the `path` or `search_query` on the entity at `service.entities.data_source` to provide a shared path or search query for all resolvers.
2. Specify the same properties within each resolver config to extend the options set at the entity level.

For example, the configuration directly above would result in:

- Find One - `/users/12`
- Find Many - `/users?userId=1&completed=true`

Note, defining a variable uses the prefix `:`. The variable is extracted from the GraphQL Input. If excluded from the GraphQL Input, the path or query string excludes the definition. You may set hard coded values in the config.

**Join and Extend Entities**

Declaring fields can be used automatically join entities to one another within the same service. The `as_type` property allows reference to another type within the service. The `join_on` field allows to associate the parent field with the child field.

```toml
[[service.entities]]
name = "user"

[[service.entities.fields]]
name = "_id"
scalar = "ObjectID"
required = true
exclude_from_input = ["CreateOne", "UpdateOne"]

# Declare a field that is an object id.
# Use the `as_type` and `join_on` values to tell the service how to extend the user type.
# The ObjectID Stored in the database under type `User.friends` is the `_id` field of the User Type, joining to the same data set.
[[service.entities.fields]]
name = "friends"
scalar = "ObjectID"
required = false
list = true
as_type = "user"
join_on = "_id"
# The `Int` stored in the database under type `User.fav_coffees` is the `id` from type `coffee`, which happens to come from a different data source.
[[service.entities.fields]]
name = "fav_coffees"
scalar = "Int"
required = false
list = true
as_type = "Coffee"
join_on = "id"
```

### CORS Options

Allow specific HTTP Methods, Origins, and Headers if needed. By default this server allows all origins, POST HTTP Methods (since it is a GraphQL server), and `Content-Type` Headers.

```toml
[service.cors]
allow_any_origin = true
allow_origins = ["http://localhost:3000"]
allow_headers = ["Authorization", "Content-Type"]

[[service.cors.allow_methods]]
method = "POST"

[[service.cors.allow_methods]]
method = "GET"
```

### Environment Variables

Use environment variables in the configuration file with `$` syntax.

```toml
default_headers = [{ name = "Authorization", value = "Bearer $OPENAI_KEY" }]
```

### Resolvers

By default, all resolvers are created for all entities. This is with the exception of the Update One resolver, in which SqLite and Postgres do not support the `LIMIT 1` query.

### Guards

Guards are boolean expressions that if evaluated true will block access to the service with a 403 like error. They are added to the config file, allowing the developer to place guards at different points in the request lifecycle.

- Service Guards - Guards access to the entire service. Evaluated before query logic is executed.
- Resolver Guards - Guards access to the entire resolver. Evaluated on a per resolver basis, before query logic is executed.
- Entity Guards - Guards access to the entire entity. Evaluated before queyr logic.
- Field Guards - Guards access when field is present and truthy evaluation. Evaluated before query logic, if field is present in field list.

Guards grant access to various datapoints such as Headers and Input values.

Guard Service:

```toml
[service]
service_name = "espresso"

[[service.guards]]
name = "role"
if_expr = "headers(\"role\") != \"Admin\""
then_msg = "Invalid Role - You may not request from this service."
```

The configuration, `guards.toml`, in the examples folder demonstrates remaining use cases, Resolver, Entity, and Field Guards.

## API

### CLI Options

- `--help` - View available commands.
- `--config <path>` - Path to the config file.
- `--port <port>` - The port for the service to run.
- `--log-level <level>` - Options include `info` or `debug`

### Config File Options

| Service\*    | Description                          | Type          |
| ------------ | ------------------------------------ | ------------- |
| service_name | The name of this service.            | String        |
| data_sources | Where the data is located.           | Data Source[] |
| entities\*   | The data to be defined.              | Entity[]      |
| cors         | Cors options for the GraphQL Server. | Cors Config   |

#### Data Sources

| Data Source\* | Description       | Type         |
| ------------- | ----------------- | ------------ |
| Mongo         | Mongo Data Source | Mongo Config |
| HTTP          | HTTP Data Source  | HTTP Config  |
| SQL           | SQL Data Source   | SQL Config   |

| Mongo Config | Description                         | Type   |
| ------------ | ----------------------------------- | ------ |
| name\*       | The name of the mongo data source.  | String |
| uri\*        | The connection string for the mongo | String |
| db\*         | The name of the mongo database.     | String |

| HTTP Config     | Description                            | Type            |
| --------------- | -------------------------------------- | --------------- |
| name\*          | The name of the HTTP data source.      | String          |
| url\*           | The base URL for the HTTP data source. | String          |
| default_headers | Headers to include with every request  | DefaultHeader[] |

| Default Header | Description                             | Type   |
| -------------- | --------------------------------------- | ------ |
| name           | The key of the key value header pair.   | String |
| value          | The value of the key value header pair. | String |

| SQL Config | Description                       | Type           |
| ---------- | --------------------------------- | -------------- |
| name       | The name of the SQL data source.  | String         |
| uri        | SQLX Compatible URI (rust crate). | String         |
| dialect    | The dialect of the SQL DB.        | Dialect Option |

| Dialect Option |
| -------------- |
| SQLITE         |
| POSTGRES       |
| MYSQL          |

#### Cors Config

| Cors Config      | Description                                              | Type           |
| ---------------- | -------------------------------------------------------- | -------------- |
| allow_any_origin | A boolean value indicating whether any origin is allowed | Boolean        |
| allow_origins    | A list of allowed origins                                | String[]       |
| allow_headers    | A list of allowed headers                                | String[]       |
| allow_methods    | A list of allowed HTTP methods                           | MethodConfig[] |

| MethodConfig | Description                                     | Type         |
| ------------ | ----------------------------------------------- | ------------ |
| method       | A string representation of the method to allow. | MethodOption |

| MethodOption |
| ------------ |
| GET          |
| Options      |
| Get          |
| Post         |
| Put          |
| Delete       |
| Head         |
| Trace        |
| Connect      |
| Patch        |

#### Entity

| Entity\*    | Description                      | Type                      |
| ----------- | -------------------------------- | ------------------------- |
| name\*      | The name of the entity.          | String                    |
| fields\*    | The fields of the entity.        | Field[]                   |
| data_source | The source of the entity's data. | Entity Data Source Config |

| Entity Data Source Config | Description                                                         | Type              |
| ------------------------- | ------------------------------------------------------------------- | ----------------- |
| from\*                    | The name of the associated HTTP Data Source.                        | String            |
| collection                | The name of the associated Mongo Collection.                        | String            |
| table                     | The name of the associated SQL Table.                               | String            |
| path                      | The path/endpoint relative to the associated HTTP Data Source Path. | String            |
| resolvers                 | Configuration to apply per generated resolver.                      | Entity Resolver[] |

| Entity Resolver | Description                               | Type                   |
| --------------- | ----------------------------------------- | ---------------------- |
| FindOne         | Configuration for the Find One Resolver   | Entity Resolver Config |
| FindMany        | Configuration for the Find Many Resolver  | Entity Resolver Config |
| CreateOne       | Configuration for the Create One Resolver | Entity Resolver Config |

| Entity Resolver Config | Description                                                              | Type         |
| ---------------------- | ------------------------------------------------------------------------ | ------------ |
| search_query           | A parameterized search query to append to the entity path.               | String       |
| path                   | A parameterized url path (endpoint) to append to the (HTTP Data Source). | String       |
| method                 | Override the default method for the resolver (HTTP Data Source)          | MethodOption |

#### Field

| Field\*             | Description                                                         | Type               |
| ------------------- | ------------------------------------------------------------------- | ------------------ |
| name\*              | The name of the field.                                              | String             |
| scalar\*            | The scalar type of the field.                                       | Scalar Options     |
| required            | Whether or not the field is required. Defaults to false.            | Option<bool>       |
| exclude_from_input  | A list of resolvers of which not to apply to the associated input.  | ExcludeFromInput[] |
| exclude_from_output | A list of resolvers of which not to apply to the associated input.  | bool               |
| list                | Defines the scalar as a list or a singular value.                   | Option<bool>       |
| as_type             | Associates the field with another entity type for joining/extending | Option<String>     |
| join_on             | The 'foreign key' of the type to be joined on.                      | Option<String>     |

| Scalar Options |
| -------------- |
| String         |
| Int            |
| Boolean        |
| ObjectID       |
| Object         |

| ResolverType |
| ------------ |
| FindOne      |
| FindMany     |
| CreateOne    |
| UpdateOne    |
| UpdateMany   |

| ExcludeFromInput |
| ---------------- |
| FindOne          |
| FindMany         |
| CreateOne        |
| UpdateOne        |
| UpdateMany       |
| UpdateOneQuery   |
| UpdateManyQuery  |

#### Guard

| Guard    | Description                                                                                    | Type   |
| -------- | ---------------------------------------------------------------------------------------------- | ------ |
| name     | The name of the guard. Used as the key of the key value pair if/when guard is invoked.         | String |
| if_expr  | [EvalExpr](https://docs.rs/evalexpr/latest/evalexpr/) syntax to evaluate a boolean expression. | String |
| then_msg | The message of the key value pair if/when guard is invoked.                                    | String |

Additional guard functions that may be used within the `if_expr` syntax. Currently not supported nativly in EvalExpr.

| Additional Guard Functions | Description                                                                   | Usage                                                                                           |
| -------------------------- | ----------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------- |
| headers                    | Extracts a header value from request headers.                                 | "headers(\"authoriation\") == \"1234\""                                                         |
| input                      | Extracts a value from the user input.                                         | "input(\"comments.user.id\") != \"23\""                                                         |
| contains                   | Checks if arg 2 (value) exists in arg 1 (tuple/vec/array).                    | "contains((\"Dallas\", \"Houston\"), \"Houston\")"                                              |
| contains_any               | Checks if arg 2 (tuple/vec/array of values) exists in arg 1 (tuple/vec/array) | "contains_any((\"Dallas\", \"Houston\", \"Denver\"), (\"Denver\", \"Fort Worth\", \"Austin\"))" |
