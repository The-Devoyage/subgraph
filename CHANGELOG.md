# Changelog

## [v0.0.11]

### Added

- New option to exclude field from "All" inputs.
- Builtin Auth with WebAuthn.
- Resolve `__typename` support.
- Option to allow nullable root entity.
- Config to load SQLITE Plugins/Extensions.
- `join_from` on Entity Field Config allows to reuse existing values to extend entity.
- Run migrations before starting.
- `token_data` built in functions for guards to access user_id and identifier in guard context.
- Imports functionality allows to import entities from a pathpuf in service config to distribute config file length.

### Fixed

- Including all options as `exclude_from_input` caused playground to not introspect. Now excludes inputs that are empty.
- Resolving Root Objects fixed and added improved tests.
- Update Many SQL - Remove escape characters for string types in where clause.
- Replace ENV Vars Function is now more accurate and retains multiple replacements at once.
- Join only works when joining from a mongo source. Now supports sql sources.
- SQL Find One queries no longer return error if not found.
- Input guard function now accepts optional properties by default.

### Changed

- Refactored various field resolvers and provided better error handling.
- Entity field property, `join_on`, is now optional. Leaving it as a None value will result in a virtual join based on the input criteria.

## [v0.0.10]

### Fixed

- SqLite Update Many Return Failure.
- MySQL Resolves i32 and i64 Integers - When using BigInt as datatype to for ID, resolver had mix matched types.
- Re-enable MySQL Update One Resolver. - Was inadvertently removed from resolvers in previous version refactor. Now reflected correctly in docs.
- HTTP Data Sources return failure response if not response status 200-299.
- Update Query Input is now unique and not the same as `find_one` input.
- `exclude_from_input` options now allow excluding fields from update query inputs.
- Update `service_name` field to `name` in configuration.
- Create one, non required, input value list option fixed.

## [v0.0.9]

### Added

- Guards to assist with Authorization.
- Port and Log Level options now supported in config file.
- Join and extend types between databases.

### Fixed

- Mongo filter support for arrays and array of objects.
- List options for boolean input scalars fixed.
- Object ID lists can now be resolved.

## [v0.0.8]

## Added

- SQL Data Source - Postgres, MySql, and SqLite support.
- Update Many Support

## [v0.0.7]

### Added

- HTTP Data Source Added with Find One, Create One, and Find Many Resolvers
- Optional property `exclude_from_input` is now a part of a Field definition.
- Update One Resolver enables the ability to update mongo or put/patch http entities.
- Object Scalars - Allows for nested field definitions and filtering.
- List Options - Allow all scalars to be implemented as a list.
- Environment Variable Support - Use env vars in config file with $VARIABLE syntax.

### Fixed

- The `_id` property will no longer be removed automatically from input generation.
- The `required` property will now default to false when defining fields in the configuration.

## [v0.0.6]

### Added

- Multi data sources and entity data source mapping.

## [v0.0.5]

### Added

- Simple CORS Configuration including Headers, Methods, and Allowed Origins.

## [v0.0.4]

### Added

- ObjectID Scalar Type

## [v0.0.3]

### Added

- Find Many Resolver

## [v0.0.2]

### Added

- Include new database options for entity, including the ability to specify the collection name. If no collection name is provided the entity name is used as default mongo collection name value.
