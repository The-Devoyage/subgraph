# @The-Devoyage/subgraph

A dynamic GraphQL API Generator.

_Advise: This is a work in progress and not yet intended for production use._

## Usage

1. Define the entities.

```toml
# config.toml

[service]
service_name = "pets"

[[service.entities]]
name = "Dog"

[[service.entities.fields]]
name = "name"
scalar = "string"

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

- Sandbox runs on the specified port.

```
http://localhost:5011
```

- Query from the `/graphql` endpoint.

## Features

### Simple Schema

Define the entities that will be resolved and start the API.

### CRUD

There are four resolvers that are created for each entity.

- Create Many
- Find One
- Find Many
- Delete Many
- Update Many

### Database Management

- Automatic indexing based off config file options.

## API

### CLI Options

- `--help` - View available commands.
- `--config <path>` - Path to the config file. 
- `--port <port>` -  The port for the service to run.
- `--log-level <level>` - Options include `info` or `debug`

### Config File Options

- service: Table

|-------------|----------|
|service_name | String   |
|-------------|----------|
|entities     | entity[] |
|-------------|----------|

- entity

|-------------|----------|
|name         | String   |
|-------------|----------|
|fields       | field[]  |
|-------------|----------|


- field

|-------------|----------|
|name         | String   |
|-------------|----------|
|scalar       | String   |
|-------------|----------|
