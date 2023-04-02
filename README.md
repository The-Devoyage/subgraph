# @The-Devoyage/subgraph

Currently, a POC written in Rust in order to dynamically generate a functional API from a simple configuration/schema.

## Quick Start

Define a configuration to run the service. The configuration tells subgraph how to generate the API around the data you need.

1. Define your configuration file.

- [Basic Mongo Example](examples/basic-mongo.toml)
- [Two Mongo Data Sources](examples/two-mongo-dbs.toml)
- [HTTP Data Source (External API)](examples/basic-http.toml)
- [Advanced Mixed Data Sources](examples/advanced-mixed.toml)

2. Start the Service

```bash
cargo run -- --config ./config.toml --port 5011 --log-level debug
```

Read below for binary/build options.

3. Use the API

- GraphQL Sandbox runs on the specified port.

```
# In the browser:
http://localhost:5011
```

- Queries and Mutations to the `/graphql` endpoint.

## Features

### Simple Schema

Simple TOML configuration to define the entities to be resolved.

### CRUD

Resolvers are created for each defined entity.

- Find One
- Find Many
- Create One

### Sandbox

Once started, view the sandbox in the browser hosted at the specified port. For example `http://localhost:5011`.

- View the generated schema using the schema tab.
- Write and execute GraphQL queries in the playground.

## Usage

### Build

If downloading from source, running the `cargo run` command as demonstrated in the quick start is useful. When using in production, create a release to generate a executable binary.

```bash
cargo build --relesae
```

### Defining The Service

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
[service.data_sources.Mongo]
name = "mongo_2"
uri = "mongodb+srv://user:pass@cluster298.an37alj.mongodb.net/?retryWrites=true&w=majority"
db = "remote_db"

[[service.data_sources]]
[service.data_sources.HTTP]
name = "todos"
url = "https://jsonplaceholder.typicode.com"
```

### Defining Entities

Entities are the assets returned from the data source. You may define multiple entities. Entities require field definitions to describe the properties of the entity.

```toml
[[service.entities]]
name = "Person"

[[service.entities.fields]]
name = "_id"
scalar = "ObjectID"
required = true

[[service.entities.fields]]
name = "name"
scalar = "String"
required = true

[[service.entities.fields]]
name = "age"
scalar = "String"
# required = false by default 
```

Fields be nested using Object Scalars. See a full list of avaiable scalars within the API Section of this README.

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

| Mongo Config | Description                         | Type   |
| ------------ | ----------------------------------- | ------ |
| name\*       | The name of the mongo data source.  | String |
| uri\*        | The connection string for the mongo | String |
| db\*         | The name of the mongo database.     | String |

| HTTP Config | Description                            | Type   |
| ----------- | -------------------------------------- | ------ |
| name\*      | The name of the HTTP data source.      | String |
| url\*       | The base URL for the HTTP data source. | String |

#### Cors Config

| Cors Config      | Description                                              | Type            |
| ---------------- | -------------------------------------------------------- | --------------- |
| allow_any_origin | A boolean value indicating whether any origin is allowed | Boolean         |
| allow_origins    | A list of allowed origins                                | String[]        |
| allow_headers    | A list of allowed headers                                | String[]        |
| allow_methods    | A list of allowed HTTP methods                           | MethodConfig [] |

| MethodConfig | Description                                     | Type         |
| ------------ | ------------------------------------------------| ------------ |
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
| from                      | The name of the associated HTTP Data Source.                        | String            |
| collection                | The name of the associated Mongo collection.                        | String            |
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

| Field\*             | Description                                                                | Type           |
| ------------------- | -------------------------------------------------------------------------- | -------------- |
| name\*              | The name of the field.                                                     | String         |
| scalar\*            | The scalar type of the field.                                              | Scalar Options |
| required            | Whether or not the field is required. Defaults to false.                   | Boolean        |
| exclude_from_input  | A list of resolvers of which not to apply to the associated input.         | ResolverType[] |

| Scalar Options |
| -------------- |
| String         |
| Int            |
| Boolean        |
| ObjectID       |
| Object         |

| ResolverType  |
| ------------- |
| FindOne       |
| FindMany      |
| CreateOne     |
| UpdateOne     |
