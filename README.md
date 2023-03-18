# @The-Devoyage/subgraph

Currently, a POC written in Rust in order to dynamically generate a functional API from a simple configuration/schema.

## Quick Start

Define a configuration to run the service. The configuration tells subgraph how to generate the API around the data you need.

1. An example configuration with two Mongo databases and a connection to a RESTful API providing 3 entities.

```toml
[service]
# This is the name of the service
service_name = "pets"

# This is a list of data sources used by the service
[[service.data_sources]]

# This is a data source that connects to a MongoDB instance
[service.data_sources.Mongo]
# This is the name of the data source
name = "mongo_1"
# This is the connection URI for the MongoDB instance
uri = "mongodb://users:users@127.0.0.1:27017/users"
# This is the name of the database in the MongoDB instance
db = "sun"

# This is another MongoDB data source using Mongo Atlas
[[service.data_sources]]
[service.data_sources.Mongo]
name = "mongo_2"
uri = "mongodb+srv://dogs:dogs@cluster0.dog1234.mongodb.net/?retryWrites=true&w=majority"
db = "dogs"

# This is an HTTP data source connecting to an external API
[[service.data_sources]]
[service.data_sources.HTTP]
name = "todos"
url = "https://jsonplaceholder.typicode.com"

# This is the CORS configuration for the service
[service.cors]
allow_any_origin = true
allow_headers = ["Authorization", "Content-Type"]

[[service.cors.allow_methods]]
method = "POST"

# This is a list of entities used by the service
[[service.entities]]
name = "Person"

# This is the data source for the entity
[service.entities.data_source]
from = "mongo_1"
collection = "users"

# These are the fields of the entity
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
scalar = "Int"
required = false

[[service.entities.fields]]
name = "married"
scalar = "Boolean"
required = true

# This is another entity
[[service.entities]]
name = "Dog"
[service.entities.data_source]
from = "mongo_2"
collection = "users"
[[service.entities.fields]]
name = "_id"
scalar = "ObjectID"
required = true
[[service.entities.fields]]
name = "name"
scalar = "String"
required = true
[[service.entities.fields]]
name = "breed"
scalar = "String"
required = true

# This is another entity
[[service.entities]]
name = "todo"
[service.entities.data_source]
from = "todos"
path = "/todos"
[service.entities.data_source.resolvers]
[service.entities.data_source.resolvers.find_one]
path = "/:id"
[service.entities.data_source.resolvers.find_many]
search_query = [["userId", ":userId"], ["completed", ":completed"], ["id", ":id"]]
[[service.entities.fields]]
name = "userId"
scalar = "Int"
required = true
[[service.entities.fields]]
name = "id"
scalar = "Int"
required = true
[[service.entities.fields]]
name = "title"
scalar = "String"
required = true
[[service.entities.fields]]
name = "completed"
scalar = "Boolean"
required = true
```

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

## API

### CLI Options

- `--help` - View available commands.
- `--config <path>` - Path to the config file.
- `--port <port>` - The port for the service to run.
- `--log-level <level>` - Options include `info` or `debug`

### Config File Options

### Config File Options

| Service\*    |                    |
| ------------ | ------------------ |
| service_name | String             |
| entities\*   | Entity[]           |
| data_sources | Data Source Enum[] |
| cors         | Cors Config        |

#### Data Sources

| Data Source Enum\* |                   |
| ------------------ | ----------------- |
| Mongo              | Mongo Data Source |
| HTTP               | HTTP Data Source  |

| Mongo Data Source |        |
| ----------------- | ------ |
| name\*            | String |
| uri\*             | String |
| db\*              | String |

| HTTP Data Source |        |
| ---------------- | ------ |
| name\*           | String |
| url\*            | String |

#### Cors

| Cors Config      |                |
| ---------------- | -------------- |
| allow_any_origin | Boolean        |
| allow_origins    | String[]       |
| allow_headers    | String[]       |
| allow_methods    | MethodOption[] |

| Method Option |        |
| ------------- | ------ |
| method        | String |

#### Entities

| Entity\*    |                           |
| ----------- | ------------------------- |
| name\*      | String                    |
| fields\*    | Field[]                   |
| data_source | Entity Data Source Config |

| EntityDataSourceConfig | Description                                                         | Type             |
| ---------------------- | ------------------------------------------------------------------- | ---------------- |
| collection             | The name of the associated mongo collection                         | String           |
| from                   | The name of the associated data source.                             | String           |
| path                   | The path/endpoint relative to the associated HTTP Data Source Path. | String           |
| resolvers              | Configuration to apply per generated resolver.                      | EntityResolver[] |

| EntityResolver | Description                               | Type                 |
| -------------- | ----------------------------------------- | -------------------- |
| FindOne        | Configuration for the Find One Resolver   | EntityResolverConfig |
| FindMany       | Configuration for the Find Many Resolver  | EntityResolverConfig |
| CreateOne      | Configuration for the Create One Resolver | EntityResolverConfig |

| EntityResolverConfig | Description                                                               | Type   |
| -------------------- | ------------------------------------------------------------------------- | ------ |
| search_query         | A parameterized search query to append to the entity path.                | String |
| path                 | A parameterized url path (endpoint) to append to the HTTP datasource url. | String |

| Field\*    |               |
| ---------- | ------------- |
| name\*     | String        |
| scalar\*   | ScalarOptions |
| required\* | Boolean       |

| ScalarOptions |
| ------------- |
| String        |
| Int           |
| Boolean       |
| ObjectID      |

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
```

### Defining Entities

Entities are the assets returned from the data source. You may define multiple entities. Entities require field definitions, to describe the properties of the entity.

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
```

#### Entity Configuration

**Entity Data Source**

If not defined, entities are associated with the first defined data source but can be assigned to any data source defined.

The `from` field is associated with the `name` of the data source associated with the entity.

```toml
[[service.entities]]
name = "Person"

[service.entities.data_source]
from = "secondary_data_source" # The name of the data source to associate with.
collection = "users" # For use with Mongo Data Source
path = "/users" # For use with HTTP Data Source

[service.entities.data_source.resolvers]
[service.entities.data_source.resolvers.find_many]
path = "/:id"  # Converts the ID property of the GraphQL Input into the ID Path Parameter for HTTP Data Sources.
[service.entities.data_source.resolvers.find_many]
search_query = [["userId", ":userId"], ["completed", ":completed"], ["id", ":id"]] #Append Search Query to Path for HTTP Data Sources.

[[service.entities.fields]]
name = "_id"
scalar = "ObjectID"
required = true

[[service.entities.fields]]
name = "name"
scalar = "String"
required = true
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
