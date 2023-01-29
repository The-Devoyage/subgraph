# @The-Devoyage/subgraph

Currently, a POC written in Rust in order to generate a functional API generated from a simple configuration/schema.

## A Dynamic GraphQL Api Generator

1. Define Entities

```toml
# config.toml

[service]
service_name = "pets"

[service.database_config]
mongo_uri = "mongodb://user:pass@127.0.0.1:27017/db"
mongo_db = "myDb"

[[service.entities]]
name = "Dog"

[[service.entities.fields]]
name = "name"
scalar = "string"
required = true

[[service.entities.fields]]
name = "weight"
scalar = "number"

[[service.entities]]
name = "Cat"

[[service.entities.fields]]
name = "name"
scalar = "string"

[[service.entities.fields]]
name = "weight"
scalar = "number"
```

2. Start the Service

```bash
./subgraph --config ./config.toml --port 5011 --log-level debug
```

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

- Create Many
- Find One

### Sandbox

Once started, view the sandbox in the browser hosted at the specified port. For example `http://localhost:5011`.

- View the generated schema using the schema tab.
- Write and execute GraphQL queries in the playground.

## API

### CLI Options

- `--help` - View available commands.
- `--config <path>` - Path to the config file. 
- `--port <port>` -  The port for the service to run.
- `--log-level <level>` - Options include `info` or `debug`

### Config File Options

- service: Table

|key             | value          |
|----------------|----------------|
|service_name    | String         |
|entities        | entity[]       |
|database_config | DatabaseConfig |

- database config

|key          | value    |
|-------------|----------|
|mongo_uri    | String   |
|mongo_db     | String   |



- entity

|key          | value    |
|-------------|----------|
|name         | String   |
|fields       | field[]  |


- field

|key          | value    |
|-------------|----------|
|name         | String   |
|scalar       | String   |
|required     | Boolean  |

