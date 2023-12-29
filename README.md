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
name = "dogs"
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

### Filtering

Subgraph provides the ability for the client to specify custom filters with ease. Filters can be combined and used together (recursively) to create a query that targets the exact results needed for the application.

### Guards - Security

Protecting the API is simple and can be done by adding `Guards` in the configuration file. Guards are quick to write expressions that if evaluated truthy result in a 403 like error (or custom error).

- Access to common datapoints such as input values and header values.
- Easy to implement and logical using the [EvalExpr Crate](https://docs.rs/evalexpr/latest/evalexpr/) Syntax.
- Custom error message support.
- Guard at different points in the request lifecycle (before, during, after request).

### Auth - Webauthn

Authorization and Authentication are built into the service using WebAuthN and Biscuit Tokens. Enabling auth is done in a few lines in the configuration which
creates register and login routes. Controllers can be guarded in combination with authennticatoin processes.

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
name = "demo"
```

### Defining Data Sources

You must define at least one Data Source. See the `Data Source Enum` table in the `API` section of this readme for supported Data Sources. You may define multiple Data Sources.

```toml
[service]
name = "demo"

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
Additionally, HTTP Data sources are limited by what the API expects to receive. Use best judgement when working with these data sources.

**Join and Extend Entities**

Declaring fields can be used automatically join entities to one another within the same service. The `as_type` property allows reference to another type within the service. The `join_on` field allows to associate the parent field with the child field.

The `join_on` field may be left empty to create a "virtual" join based on the input values without a parent value constraint.

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

### Authorization and Authentication with WebAuthN (Passwordless)

Authentication and Authorization can be enabled by following a few quick steps.

1. Define the auth config. Environment variables are handy when managing multiple environments.

```toml
[service.auth]
requesting_party = "$TRICERATASK_RP" # The requesting party - Mut contain origin name. "localhost"
requesting_party_name = "$TRICERATASK_RPN" # Name of the requesting party. Can be almost anything. "triceratask.com"
requesting_party_origin = "$TRICERATASK_RPO" # Origin of the client. "http://localhost:5173"
data_source = "name_of_datasource_from_config" # To save user information
private_key = "$BASE_64_PRIVATE_KEY" # Optional - `subgraph --generate-keypair` to generate a keypair
```

Private key is an optional value. If not provided, subgraph will generate new keys each time it is restarted, effectivly
epxiring previous keys.

2. If you are using SQL, ensure you create the `subgraph_user` table.

```sql
-- Create the subgraph_user table sqlite
CREATE TABLE IF NOT EXISTS subgraph_user (
  uuid uuid NOT NULL UNIQUE,
  identifier TEXT NOT NULL UNIQUE,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  registration_state TEXT DEFAULT NULL,
  passkey TEXT DEFAULT NULL,
  authentication_state TEXT DEFAULT NULL
);
```

3. Register and Authenticate

Once the service is started you will have access to 4 resolvers to handle authentication and registration. Webauthn
is a passworless method of authentication and uses the native 'navigate' api found in most browsers.

Below is a clientside example using typescript.

- Send a mutation to the `register_start` resolver. This will return a `CredentialCreationOptions` object.
- Use the browsers `navigator` api to create a public key credential.

```ts
// Send mutation to api using a graphql client of choice.
// Response includes the Credential Creation Options to build the credential.
const cco: CredentialCreationOptions = await mutation(REGISTER_START, {
  variables: {
    identifier: "my_username",
  },
});

// Use navigator api to create the `credential`
const credential = await navigator.credentials.create({
  publicKey: {
    ...cco.publicKey,
    //Ensure to pass a byte array.
    challenge: Base64.toUint8Array(
      (cco.publicKey?.challenge as unknown) as string
    ),
    user: {
      ...cco.publicKey?.user,
      id: Base64.toUint8Array((cco.publicKey?.user?.id as unknown) as string),
    },
  },
});
```

- Finalize the registration process by sending a mutation to the `register_finish` resolver.

```ts
// Create a new credential onbject so that you can base64 encode the following properties.
// Subgraph expects these to be base64 encoded.
const credential = {
  id: credential.id,
  type: credential.type,
  rawId: Base64.fromUint8Array(new Uint8Array(credential.rawId), true),
  extensions: credential.getClientExtensionResults(),
  response: {
    clientDataJSON: Base64.fromUint8Array(
      new Uint8Array(credential.response.clientDataJSON),
      true
    ),
    attestationObject: Base64.fromUint8Array(
      new Uint8Array(
        ((credential.response as unknown) as Record<string, ArrayBuffer>)
          .attestationObject as ArrayBuffer
      ),
      true
    ),
  },
};
// Send the credential to the resolver using a graphql client of your choosing.
const register_success = await mutation(REGISTER_FINISH, {
  variables: { credential },
});
```

- Use the `authenticate_start` resolver to start the authenticaion process.
- Use the browsers `navigator` api to get the matching public key.

```ts
// Call the authenticateStart resolver using a client of your choice.
// The response contains CredentialRequestOpions, used to get the credential
// from the browser.
const credentialRequestOptions: CredentialRequestOptions = await mutation(
  AUTHENTICATE_START,
  { variables: { identifier: "my_username" } }
);

// Get the credential from the browser.
// Make sure to convert the challenge to a byte array.
const credential = await navigator.credentials.get({
  publicKey: {
    ...credentialRequestOptions.publicKey,
    challenge: Base64.toUint8Array(
      (credentialRequestOptions.publicKey?.challenge as unknown) as string
    ),
    allowCredentials: credentialRequestOptions.publicKey?.allowCredentials?.map(
      (c) => ({
        ...c,
        id: Base64.toUint8Array((c.id as unknown) as string),
      })
    ),
  },
});
```

- Use the `authenticate_finish` to obtain the auth token, which can be used for persisted autoriation to the API.

```ts
// Ensure the response object exists and contains the data we need.
const response = credential.response as AuthenticatorAssertionResponse;
if (!response.userHandle) return;

// Create a object with the proper base_64 encided values.
const public_key = {
  id: credential.id,
  type: credential.type,
  rawId: Base64.fromUint8Array(new Uint8Array(credential.rawId), true),
  extensions: credential.getClientExtensionResults(),
  response: {
    authenticatorData: Base64.fromUint8Array(
      new Uint8Array(response.authenticatorData),
      true
    ),
    clientDataJSON: Base64.fromUint8Array(
      new Uint8Array(response.clientDataJSON),
      true
    ),
    signature: Base64.fromUint8Array(new Uint8Array(response.signature), true),
    userHandle: Base64.fromUint8Array(
      new Uint8Array(response.userHandle),
      true
    ),
  },
};

// Receive the token by sending a request using a client of your choosing.
// Save the token for furthor authorization.
const token = await mutation(AUTHENTICATE_FINISH, {
  variables: {
    identifier: "my_username",
    public_key,
  },
});
```

4. Authorizing the Request

Receiving the token allows the client to persist authorization between requests. The token currently does not expire unless the service's private key changes.
The service's private key will automatically change after each restart as long as the `private_key` field is not present in the auth config.

Using the token to authorize requests can be done by utilizing the service's header and guard features.

1. Ensure `authorization` header is allowed in the service config.

```toml
[service.cors]
allow_headers = ["Authorization", "Content-Type"]
```

2. Attach the token to each request sent to the API.

```
# Headers
Authorization = $TOKEN
```

3. Use guards to protect the API based on the data extracted from the provided token in the Authorization header.

```toml
[[service.guards]]
name = "Permissions Error"
if_expr = "token_data(\"user_uuid\") != input(\"created_by\")"
then_msg = "Permission Denied - You can only manage your own entities."
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

### Filtering

Filtering in a query can be done in a variety of combinations.

**AND Filtering**

Find user that is 20 years old AND is married.

```graphql
{
  get_user(
    get_user_input: { query: { AND: [{ age: 20 }, { married: false }] } }
  ) {
    _id
  }
}
```

**OR Filtering**

Find user that is 20 years old or is not married.

```graphql
{
  get_user(
    get_user_input: { query: { OR: [{ age: 20 }, { married: false }] } }
  ) {
    _id
  }
}
```

Nested Examples

Find user that is (20 and not married) OR (is 21 and not married.)

```graphql
{
  get_user(
    get_user_input: {
      query: { OR: [{ age: 20, married: false }, { age: 21, married: false }] }
    }
  ) {
    _id
  }
}
```

```graphql
{
  get_user(
    get_user_input: {
        query: { OR: [
            { OR: [...] },
            { OR: [...]}
        ]}
    }
  ) {
    _id
  }
}
```

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
name = "espresso"

[[service.guards]]
name = "role"
if_expr = "headers(\"role\") != \"Admin\""
then_msg = "Invalid Role - You may not request from this service."
```

The configuration, `guards.toml`, in the examples folder demonstrates remaining use cases, Resolver, Entity, and Field Guards.

### Imports

Imports is an optional array of paths that can be provided to the service configuration. Each path should lead to a toml file containing a valid service resource.

Currently this only supports importing an entity from each file defined.

## API

### CLI Options

- `--help` - View available commands.
- `--config <path>` - Path to the config file.
- `--port <port>` - The port for the service to run.
- `--log-level <level>` - Options include `info` or `debug`
- `--watch` - Listens for changes for all files from the directory containing the config file.
  Debounces 1 second to avoid duplicate restarts. If initial config is incorrect, server will not start.

### Config File Options

| Service\*      | Description                                               | Type          |
| -------------- | --------------------------------------------------------- | ------------- |
| name\*         | The name of this service.                                 | String        |
| data_sources\* | Where the data is located.                                | Data Source[] |
| entities\*     | The data to be defined.                                   | Entity[]      |
| cors           | Cors options for the GraphQL Server.                      | Cors Config   |
| guards         | Guards applied at the sservice level.                     | Guard[]       |
| imports        | An array of paths to import entities from separate files. | String[]      |
| port           | The port of which to run the service.                     | Int           |

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

| SQL Config        | Description                                             | Type           |
| ----------------- | ------------------------------------------------------- | -------------- |
| name              | The name of the SQL data source.                        | String         |
| uri               | SQLX Compatible URI (rust crate).                       | String         |
| dialect           | The dialect of the SQL DB.                              | Dialect Option |
| sqlite_extensions | Array to specify path to file for sqlite extension.\*\* | String         |
| migrations_path   | Path to folder containing migrations to run.\*\*        | String         |

**Note**
Extensions are loaded automatically if provided.
Migrations are only executed if subgraph is run with the flag `--migrate run`

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

| Entity\*    | Description                         | Type                      |
| ----------- | ----------------------------------- | ------------------------- |
| name\*      | The name of the entity.             | String                    |
| fields\*    | The fields of the entity.           | Field[]                   |
| data_source | The source of the entity's data.    | Entity Data Source Config |
| guards      | Guards applied at the entity level. | Guard[]                   |
| required    | Non nullable entity.                | bool                      |

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
| UpdateOne       | Configuration for the Update One Resolver | Entity Resolver Config |
| UpdateMany      | Configuration for the Update One Resolver | Entity Resolver Config |

| Entity Resolver Config | Description                                                              | Type         |
| ---------------------- | ------------------------------------------------------------------------ | ------------ |
| search_query           | A parameterized search query to append to the entity path.               | String       |
| path                   | A parameterized url path (endpoint) to append to the (HTTP Data Source). | String       |
| method                 | Override the default method for the resolver (HTTP Data Source)          | MethodOption |
| guards                 | Guards applied at the resolverlevel.                                     | Guard[]      |

#### Field

| Field\*             | Description                                                                   | Type               |
| ------------------- | ----------------------------------------------------------------------------- | ------------------ |
| name\*              | The name of the field.                                                        | String             |
| scalar\*            | The scalar type of the field.                                                 | Scalar Options     |
| required            | Whether or not the field is required. Defaults to false.                      | Option<bool>       |
| exclude_from_input  | A list of resolvers of which not to apply to the associated input.            | ExcludeFromInput[] |
| exclude_from_output | A list of resolvers of which not to apply to the associated input.            | bool               |
| list                | Defines the scalar as a list or a singular value.                             | Option<bool>       |
| as_type             | Associates the field with another entity type for joining/extending           | Option<String>     |
| join_on             | The 'foreign key' of the type to be joined on.                                | Option<String>     |
| join_from           | The source key to join from when perfoming associations.                      | Option<String>     |
| guards              | A list of guards to apply to a field.                                         | Option<Guard>      |
| default_value       | An eval expr calculated value that is applied for Update and Create Resolvers | Option<String>     |
| is_virtual          | Define properties on graphql inputs that do not exist in the database         | Option<bool>       |
| eager               | Search for entity based on the fields of another entity                       | Option<bool>       |

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
| All              |

#### Guard

| Guard    | Description                                                                                    | Type   |
| -------- | ---------------------------------------------------------------------------------------------- | ------ |
| name     | The name of the guard. Used as the key of the key value pair if/when guard is invoked.         | String |
| if_expr  | [EvalExpr](https://docs.rs/evalexpr/latest/evalexpr/) syntax to evaluate a boolean expression. | String |
| then_msg | The message of the key value pair if/when guard is invoked.                                    | String |

Additional guard functions that may be used within the `if_expr` syntax. Currently not supported nativly in EvalExpr.

| Additional Guard Functions | Description                                             | Usage                                                |
| -------------------------- | ------------------------------------------------------- | ---------------------------------------------------- |
| headers                    | Extracts a header value from request headers.           | "headers(\"authoriation\") == \"1234\""              |
| input                      | Extracts a value from the user input.                   | "input(\"query\", \"comments.user.id\") != \"23\""   |
| token_data                 | Extracts data from auth token, identifier and user_uuid | "token_data(\"user_uuid\") != input(\"created_by\")" |
| resolver_type              | Shows the type of resolver in guard function            | "resolver_type() == \"FindOne\""                     |
| context                    | Extracts a value from the context provided from a guard | "context(\"user.id\")"                               |

**Input Function**

The input function can be used to extract data from the request input submitted by the client. Since support for nested querying has been
implemented, this function returns a tuple or array of values that match the provided key.

The first argument specifies the root location. Valid values include "query" and "values", which match to the required inputs for queries and mutations.

The second argument is the key, which supports dot notation for nested values.
