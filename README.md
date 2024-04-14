# @The-Devoyage/subgraph

A tool to spin up a dynamic graphql web server based on a simple configuration file. Define the config to inform subgraph about the shape of your data and the available data sources. Subgraph connects to your database automatically, generates CRU(D) resolvers, authentication/authorization controntrolls and more.

Deploy as you see fit and own your data. Subgraph helps you to interact with your data, allowing you to focus on building interfaces instead of building web servers.

Use in production at your own risk - There is still a bit of work to get to v0.1.0 from our pre-alpha state. That being said, I hope that you enjoy what has been made so far.

## Docs

Subgraph Documentation has moved to our new [Documentation Site](https://www.thedevoyage.com/subgraph).

## Quick Start

Check out the [Quick Start](https://www.thedevoyage/com/subgraph/quickstart) in the new Docs Page!

## License

Use subgraph without a license for 20 minutes or enable unrestricted access by purchasing a [License Key](https://thedevoyage.gumroad.com/l/subgraph). Make sure to start the service
with the key in the service configuration to disable the timeout.

## API

### CLI Options

- `--help` - View available commands.
- `--config <path>` - Path to the config file.
- `--port <port>` - The port for the service to run.
- `--log-level <level>` - Options include `info` or `debug`
- `--watch` - Listens for changes for all files from the directory containing the config file.
  Debounces 1 second to avoid duplicate restarts. If initial config is incorrect, server will not start.
- `--generate-keypair` - Generates a keypair that can be shared between microservices or create a consistent private key.

### Config File Options

| Service\*      | Description                                                          | Type         |
| -------------- | -------------------------------------------------------------------- | ------------ |
| name\*         | The name of this service.                                            | String       |
| version        | The version of the API.                                              | String       |
| data_sources\* | Where the data is located.                                           | DataSource[] |
| entities\*     | The data to be defined.                                              | Entity[]     |
| cors           | Cors options for the GraphQL Server.                                 | Cors Config  |
| guards         | Guards applied at the sservice level.                                | Guard[]      |
| imports        | An array of paths to import entities from separate files.            | String[]     |
| port           | The port of which to run the service.                                | Int          |
| license_key    | Provide a key to remove the 20 minute demo limit.                    | String       |
| host           | Enable the ability to host on 0.0.0.0 instead of loaclhost/127.0.0.1 | bool         |

#### Data Sources

| DataSource\* | Description       | Type         |
| ------------ | ----------------- | ------------ |
| Mongo        | Mongo Data Source | Mongo Config |
| HTTP         | HTTP Data Source  | HTTP Config  |
| SQL          | SQL Data Source   | SQL Config   |

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

| SQL Config        | Description                                             | Type          |
| ----------------- | ------------------------------------------------------- | ------------- |
| name\*            | The name of the SQL data source.                        | String        |
| uri\*             | SQLX Compatible URI                                     | String        |
| dialect \*        | The dialect of the SQL DB.                              | DialectOption |
| sqlite_extensions | Array to specify path to file for sqlite extension.\*\* | String        |
| migrations_path   | Path to folder containing migrations to run.\*\*        | String        |

**Note**
Extensions are loaded automatically if provided.
Migrations are only executed if subgraph is run with the flag `--migrate run`

| DialectOption |
| ------------- |
| SQLITE        |
| POSTGRES      |
| MYSQL         |

#### Cors Config

| Cors Config      | Description                                              | Type           |
| ---------------- | -------------------------------------------------------- | -------------- |
| allow_any_origin | A boolean value indicating whether any origin is allowed | Boolean        |
| allow_origins    | A list of allowed origins                                | String[]       |
| allow_headers    | A list of allowed headers                                | String[]       |
| allow_methods    | A list of allowed HTTP methods                           | MethodConfig[] |

| MethodConfig | Description                                     | Type         |
| ------------ | ----------------------------------------------- | ------------ |
| method\*     | A string representation of the method to allow. | MethodOption |

| MethodOption |
| ------------ |
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

| Entity\*            | Description                                | Type                      |
| ------------------- | ------------------------------------------ | ------------------------- |
| name\*              | The name of the entity.                    | String                    |
| fields\*            | The fields of the entity.                  | Field[]                   |
| data_source         | The source of the entity's data.           | Entity Data Source Config |
| guards              | Guards applied at the entity level.        | Guard[]                   |
| required            | Non nullable entity.                       | bool                      |
| exclude_from_output | Remove the ability to resolve this entity. | bool                      |

| Entity Data Source Config | Description                                                         | Type            |
| ------------------------- | ------------------------------------------------------------------- | --------------- |
| from\*                    | The name of the associated HTTP Data Source.                        | String          |
| collection                | The name of the associated Mongo Collection.                        | String          |
| table                     | The name of the associated SQL Table.                               | String          |
| path                      | The path/endpoint relative to the associated HTTP Data Source Path. | String          |
| resolvers                 | Configuration to apply per generated resolver.                      | Entity Resolver |

| Entity Resolver | Description                                                  | Type                   |
| --------------- | ------------------------------------------------------------ | ---------------------- |
| FindOne         | Configuration for the Find One Resolver                      | Entity Resolver Config |
| FindMany        | Configuration for the Find Many Resolver                     | Entity Resolver Config |
| CreateOne       | Configuration for the Create One Resolver                    | Entity Resolver Config |
| UpdateOne       | Configuration for the Update One Resolver                    | Entity Resolver Config |
| UpdateMany      | Configuration for the Update One Resolver                    | Entity Resolver Config |
| Internal        | Internal types may only be interacted with when using Guards | None                   |

| Entity Resolver Config | Description                                                              | Type         |
| ---------------------- | ------------------------------------------------------------------------ | ------------ |
| search_query           | A parameterized search query to append to the entity path.               | String       |
| path                   | A parameterized url path (endpoint) to append to the (HTTP Data Source). | String       |
| method                 | Override the default method for the resolver (HTTP Data Source)          | MethodOption |
| guards                 | Guards applied at the resolverlevel.                                     | Guard[]      |

#### Field

| Field\*             | Description                                                                                           | Type               |
| ------------------- | ----------------------------------------------------------------------------------------------------- | ------------------ |
| name\*              | The name of the field.                                                                                | String             |
| scalar\*            | The scalar type of the field.                                                                         | Scalar Options     |
| required            | Whether or not the field is required. Defaults to false.                                              | bool               |
| exclude_from_input  | A list of resolvers of which not to apply to the associated input.                                    | ExcludeFromInput[] |
| exclude_from_output | Remove the ability to resolve this field.                                                             | bool               |
| list                | Defines the scalar as a list or a singular value.                                                     | bool               |
| as_type             | Associates the field with another entity type for joining/extending                                   | String             |
| join_on             | The 'foreign key' of the type to be joined on.                                                        | String             |
| join_from           | The source key to join from when performing associations.                                             | String             |
| guards              | A list of guards to apply to a field.                                                                 | Guard              |
| default_value       | An eval expr calculated value that is applied for Update and Create Resolvers. Use "null" for `null`. | String             |
| is_virtual          | Define properties on graphql inputs that do not exist in the database                                 | bool               |
| eager               | Search for entity based on the fields of another entity                                               | bool               |
| primary_key         | Use field to override the default primary key (\_id for mongo, id for sql )                           | bool               |
| enum_values         | A list of strings representing the possible values for a field.                                       | String             |

| Scalar Options |
| -------------- |
| String         |
| Int            |
| Boolean        |
| ObjectID       |
| Object         |
| Enum           |
| UUID           |
| DateTime       |

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
| All              |

#### Guard

| Guard      | Description                                                                                    | Type               |
| ---------- | ---------------------------------------------------------------------------------------------- | ------------------ |
| name\*     | The name of the guard. Used as the key of the key value pair if/when guard is invoked.         | String             |
| if_expr\*  | [EvalExpr](https://docs.rs/evalexpr/latest/evalexpr/) syntax to evaluate a boolean expression. | String             |
| then_msg\* | The message of the key value pair if/when guard is invoked.                                    | String             |
| context    | Options to define additional context from other calls to data sources.                         | GuardDataContext[] |

| GuardDataContext | Description                                                          | Type               |
| ---------------- | -------------------------------------------------------------------- | ------------------ |
| entity_name\*    | The name of the entity of where to source the context                | String             |
| name             | The unique identifier of the context to be added.                    | String             |
| query\*          | The graphql query expression to populate the context                 | String             |
| variables\*      | A vector of tuples representing the substitute and the substitution. | [String, String][] |

Additional guard functions that may be used within the `if_expr` syntax. Currently not supported nativly in EvalExpr.

| Additional Guard Functions | Description                                                             | Usage                                              |
| -------------------------- | ----------------------------------------------------------------------- | -------------------------------------------------- |
| headers                    | Extracts a header value from request headers.                           | headers(\"authoriation\") == "1234"                |
| input                      | Extracts a value from the user input. Returns tuple.                    | contains(input("query", "comments.user.id"), "23") |
| token_data                 | Extracts data from auth token, identifier and user_uuid                 | token_data("user_uuid") != input("created_by")     |
| resolver_type              | Shows the type of resolver in guard function                            | resolver_type() == "FindOne"                       |
| context                    | Extracts a value from the context provided from a guard. Returns tuple. | every(context("user.id"), 23)                      |
| now                        | Returns the current `datetime`.                                         | now()                                              |
| uuid                       | Generates a UUIDv4.                                                     | uuid()                                             |
