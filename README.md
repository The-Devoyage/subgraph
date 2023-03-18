# @The-Devoyage/subgraph

Currently, a POC written in Rust in order to dynamically generate a functional API from a simple configuration/schema.

## Quick Start

1. Define Entities

```toml
[service]
service_name = "GeneratedAPI"

[[service.data_sources]]

# MongoDB data source that connects to a MongoDB instance
[service.data_sources.Mongo]
name = "mongo_1"
uri = "mongodb://users:users@127.0.0.1:27017/users"
db = "sun"

# Another MongoDB data source
[[service.data_sources]]
[service.data_sources.Mongo]
name = "mongo_2"
uri = "mongodb+srv://dogs:dogs@cluster0.dog1234.mongodb.net/?retryWrites=true&w=majority"
db = "dogs"

# HTTP data source
[[service.data_sources]]
[service.data_sources.HTTP]
name = "todos"
url = "https://jsonplaceholder.typicode.com"

[service.cors]
# Allows any origin to access the service
allow_any_origin = true
# Allowed headers
allow_headers = ["Authorization", "Content-Type"]

[[service.cors.allow_methods]]
method = "POST"

[[service.entities]]
# Person entity
name = "Person"
[service.entities.data_source]
from = "mongo_1"
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
name = "age"
scalar = "Int"
required = false

[[service.entities.fields]]
name = "married"
scalar = "Boolean"
required = true

# Dog entity from a second mongo database, hosted elsewhere
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

# Todo entity from a HTTP data source (external API)
[[service.entities]]
name = "todo"
[service.entities.data_source]
from = "todos"
path = "/todos"

[[service.entities.data_source.resolvers]]
[service.entities.data_source.resolvers.FindOne]
path = "/:id"

[[service.entities.data_source.resolvers]]
[service.entities.data_source.resolvers.FindMany]
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

If not defined, entities are associated with the first defined data source but can be assigned to a data source.

The `from` field is associated with the `name` of the data source associated with the entity.

```toml
[[service.entities]]
name = "Person"

[service.entities.data_source]
from = "secondary_data_source" # The name of the data source to associate with.
collection = "users"

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
