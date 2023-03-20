# Changelog

## [v0.0.7]

### Added
- HTTP Data Source Added with Find One, Create One, and Find Many Resolvers
- Optional property `exclude_from_input` is now a part of a Field definition.

### Fixed
- The `_id` property will no longer be removed automatically from input generation.

## [v0.0.6]

### Added
- Multi data sources and entity data source mapping.

## [v0.0.5]

###
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
