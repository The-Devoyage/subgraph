use async_graphql::dynamic::FieldValue;
use bson::{doc, to_document, Document};
use log::{debug, error, trace, warn};
use mongodb::{options::ClientOptions, Client, Database};

use crate::{
    configuration::subgraph::{
        data_sources::mongo::MongoDataSourceConfig,
        entities::{service_entity_field::ServiceEntityFieldConfig, ServiceEntityConfig},
        SubGraphConfig,
    },
    filter_operator::FilterOperator,
    graphql::{
        entity::create_return_types::{ResolverResponse, ResolverResponseMeta},
        schema::create_options_input::{DirectionEnum, OptionsInput},
    },
    resolver_type::ResolverType,
    scalar_option::{to_mongo::MongoValue, ScalarOption},
};

use super::DataSource;

pub mod services;

#[derive(Debug, Clone)]
pub struct MongoDataSource {
    pub client: Client,
    pub db: Database,
    pub config: MongoDataSourceConfig,
}

#[derive(Debug, Clone)]
pub struct EagerLoadOptions {
    pub from: String,
    pub local_field: String,
    pub foreign_field: String,
    pub as_field: String,
}

impl MongoDataSource {
    pub async fn init(mongo_data_source_config: &MongoDataSourceConfig) -> DataSource {
        debug!("Initializing Mongo");
        let client_options = ClientOptions::parse(&mongo_data_source_config.uri)
            .await
            .expect("Failed to parse mongo client options.");

        let client = Client::with_options(client_options).expect("Failed to create client");
        let db = client.database(&mongo_data_source_config.db);

        debug!("Created Mongo Data Source");
        debug!("{:?}", client);
        debug!("{:?}", db);

        DataSource::Mongo(MongoDataSource {
            client,
            db,
            config: mongo_data_source_config.clone(),
        })
    }

    /// Recursively convert all string object ids to object ids.
    /// Uses field definitions to determine if a field is an object id.
    pub fn convert_object_id_string_to_object_id_from_doc(
        filter: Document,
        entity: &ServiceEntityConfig,
        subgraph_config: &SubGraphConfig,
        resolver_type: &ResolverType,
        key: Option<String>, // Provide a key to keep track of nested fields.
    ) -> Result<(Document, Vec<EagerLoadOptions>), async_graphql::Error> {
        debug!("Serialize String Object IDs to Object IDs");
        trace!("Filter: {:?}", filter);

        let mut converted = filter.clone();
        let mut combined_eager_options = vec![];

        for (k, value) in filter.iter() {
            trace!(
                "Current Key: {:?}, Processing Key: {}, Value: {}",
                key.clone(),
                k,
                value
            );

            // If it is a servie defined field, iterate through the fields to find the correct field.
            if k == "query"
                || k == "values"
                || FilterOperator::list()
                    .iter()
                    .map(|x| x.as_str())
                    .any(|x| x == k)
            {
                let document = match value.as_document() {
                    Some(document) => document,
                    None => {
                        error!("Failed to get document from value");
                        return Err(async_graphql::Error::from(
                            "Failed to get document from value",
                        ));
                    }
                };
                // Send through recursive function to convert the object id string to object id
                let (nested_converted, nested_eager_load_options) =
                    match MongoDataSource::convert_object_id_string_to_object_id_from_doc(
                        document.clone(),
                        entity,
                        subgraph_config,
                        resolver_type,
                        key.clone(),
                    ) {
                        Ok(nested) => nested,
                        Err(e) => {
                            error!(
                                "Failed to convert object id string to object id. Error: {:?}",
                                e
                            );
                            return Err(e);
                        }
                    };
                converted.insert(k.clone(), nested_converted);
                combined_eager_options.extend(nested_eager_load_options);
                continue;
            }

            let fields = match ServiceEntityConfig::get_fields_recursive(entity, &k) {
                Ok(fields) => fields,
                Err(_) => {
                    continue;
                }
            };

            // Since searching by a single key above, the last field is guaranteed to be the field we are looking for.
            if let Some(field) = fields.last() {
                // Certain scalars need to be converted to mongo types.
                // If they do, replace them in the doc.
                if !field.eager.unwrap_or(false)
                    || resolver_type == &ResolverType::CreateOne
                    || resolver_type == &ResolverType::UpdateOne
                    || resolver_type == &ResolverType::UpdateMany
                {
                    match field.scalar.bson_to_mongo_value(value) {
                        Ok(mongo_value) => {
                            if mongo_value.is_some() {
                                match mongo_value.unwrap() {
                                    MongoValue::ObjectID(object_id) => {
                                        //update the cooresponding value in converted
                                        converted.insert(k.clone(), object_id);
                                    }
                                    MongoValue::DateTime(date_time) => {
                                        //update the cooresponding value in converted
                                        converted.insert(k.clone(), date_time);
                                    }
                                    _ => {}
                                }
                            }
                        }
                        Err(_) => {}
                    };
                }

                // Handle object types and eager loaded fields
                match field.scalar {
                    ScalarOption::Object => {
                        trace!("Found Object Scalar");
                        trace!("Current Key: {:?}", key.clone());
                        let separator = if key.is_none() { "" } else { "." };
                        let separated = format!("{}{}", separator, k);
                        let key = Some(separated);
                        let (nested_converted, nested_eager_load_options) =
                            match MongoDataSource::convert_object_id_string_to_object_id_from_doc(
                                value.as_document().unwrap().clone(),
                                entity,
                                subgraph_config,
                                resolver_type,
                                key.clone(),
                            ) {
                                Ok(nested) => nested,
                                Err(e) => {
                                    error!(
                                        "Failed to convert object id string to object id. Error: {:?}",
                                        e
                                    );
                                    return Err(e);
                                }
                            };
                        trace!("Nested Converted: {:?}", nested_converted);
                        trace!("Inserting Key: {:?}", key);
                        converted.insert(key.as_ref().unwrap().clone(), nested_converted);
                        combined_eager_options.extend(nested_eager_load_options);
                    }
                    _ => (),
                }

                // If the resolver type is not find one or find many, we don't need to handle eager loaded fields.
                if resolver_type != &ResolverType::FindOne
                    && resolver_type != &ResolverType::FindMany
                {
                    continue;
                }

                // Handle eager loaded fields
                let eager_load_options =
                    match MongoDataSource::handle_eager_fields(field, entity, subgraph_config) {
                        Ok(eager_load_options) => eager_load_options,
                        Err(e) => {
                            error!("Failed to handle eager fields. Error: {:?}", e);
                            return Err(e);
                        }
                    };

                if let Some((eager_load_option, eager_entity)) = eager_load_options {
                    // Send through recursive function to convert the object id string to object id
                    // for eager loaded fields.
                    let (value, nested_eager_opts) =
                        match MongoDataSource::convert_object_id_string_to_object_id_from_doc(
                            value.as_document().unwrap().clone(), // If eager, this will always be a document.
                            &eager_entity,
                            subgraph_config,
                            resolver_type,
                            key.clone(),
                        ) {
                            Ok(nested) => nested,
                            Err(e) => {
                                error!(
                                    "Failed to convert object id string to object id. Error: {:?}",
                                    e
                                );
                                return Err(e);
                            }
                        };
                    combined_eager_options.extend(nested_eager_opts);

                    // replace the key with the eager load key.
                    converted.remove(&k);
                    converted.insert(eager_load_option.as_field.clone(), value);
                    combined_eager_options.push(eager_load_option);
                }
            }
        }

        trace!("Converted: {:?}", converted);
        Ok((converted, combined_eager_options))
    }

    pub fn handle_eager_fields(
        field: &ServiceEntityFieldConfig,
        entity: &ServiceEntityConfig,
        subgraph_config: &SubGraphConfig,
    ) -> Result<Option<(EagerLoadOptions, ServiceEntityConfig)>, async_graphql::Error> {
        debug!("Handle eager fields");
        // Since searching by a single key above, the last field is guaranteed to be the field we are looking for.
        if field.eager.is_none() {
            trace!("Field is not eager");
            return Ok(None);
        }

        // Get the name of the field to eager load - this can help to get the correct collection name.
        // the join_on value will be the name of the parent entity
        let join_on = if let Some(join_on) = field.join_on.clone() {
            join_on
        } else {
            return Err(async_graphql::Error::new(format!(
                "Eager load failed. Failed to get join_on for field: {}. Ensure propety `join_on` is present on the field definiton for this field.",
                field.name
            )));
        };

        let as_type = if let Some(as_type) = field.as_type.clone() {
            as_type
        } else {
            return Err(async_graphql::Error::new(format!(
                "Eager load failed. Failed to get `as_type` for field: {}",
                field.name
            )));
        };

        // Get the entity to reference the correct collection name.
        let eager_entity = match subgraph_config.clone().get_entity(&as_type) {
            Some(entity) => entity,
            None => {
                return Err(async_graphql::Error::new(format!(
                    "Eager load failed. Failed to get entity for key: {}",
                    join_on
                )));
            }
        };

        // Check if it has a collection name.
        let collection_name = if let Some(ds) = eager_entity.data_source.clone() {
            if ds.collection.is_some() {
                ds.collection.unwrap().clone()
            } else {
                eager_entity.name.clone()
            }
        } else {
            eager_entity.name.clone()
        };

        let join_from = if let Some(join_from) = field.join_from.clone() {
            join_from
        } else {
            field.name.clone()
        };

        //replace the key with the eager loaded key.
        let eager_key = format!("{}_{}_{}", entity.name, field.name, join_on);

        let eager_load_options = EagerLoadOptions {
            from: collection_name,
            local_field: join_from,
            foreign_field: join_on,
            as_field: eager_key.clone(),
        };

        trace!("Eager load options: {:?}", eager_load_options);
        Ok(Some((eager_load_options, eager_entity)))
    }

    pub fn finalize_input(
        filter: Document,
        entity: &ServiceEntityConfig,
        subgraph_config: &SubGraphConfig,
        resolver_type: &ResolverType,
    ) -> Result<(Document, Vec<EagerLoadOptions>), async_graphql::Error> {
        debug!("Finalizing Input Filters");
        trace!("Filter: {:?}", filter);

        let mut finalized = filter.clone();
        let mut eager_filters = Vec::new();

        for (key, value) in filter.iter() {
            if key == "query" {
                let query = value.as_document().unwrap();
                let (query_finalized, eager_opts) = MongoDataSource::finalize_input(
                    query.clone(),
                    entity,
                    subgraph_config,
                    &resolver_type,
                )?;
                finalized.insert(key.clone(), query_finalized);
                eager_filters.extend(eager_opts);
            }

            // Values is an object without filters, so we can just return it.
            if key == "values" {
                finalized.insert(key.clone(), value.clone());
            }

            if FilterOperator::list()
                .iter()
                .map(|operator| operator.as_str())
                .collect::<Vec<&str>>()
                .contains(&key.as_str())
            {
                trace!("Found filter operator key: {}", key);
                let mut recursive_filters = Vec::new();
                let filters = match value.as_array() {
                    Some(filters) => filters.clone(),
                    None => {
                        let filters = vec![value.clone()];
                        filters
                    }
                };
                for filter in filters {
                    let filter = filter.as_document().unwrap();
                    let (filter_finalized, eager_opts) = MongoDataSource::finalize_input(
                        filter.clone(),
                        entity,
                        subgraph_config,
                        &resolver_type,
                    )?;
                    recursive_filters.push(filter_finalized);
                    eager_filters.extend(eager_opts);
                }
                finalized.remove(key);
                let filter_operator = FilterOperator::from_str(key).unwrap();
                let operator_key = FilterOperator::get_mongo_operator(&filter_operator);

                match filter_operator {
                    FilterOperator::And => {
                        finalized.insert(operator_key, recursive_filters);
                    }
                    FilterOperator::Or => {
                        finalized.insert(operator_key, recursive_filters);
                    }
                    _ => {
                        // The rest of the filter operators require the reverse format.
                        // For example { $eq: { name: "test" } } becomes { name: { $eq: "test" } }
                        let mut new_filter = Document::new();
                        for filter in recursive_filters {
                            for (key, value) in filter.iter() {
                                let mut new_value = Document::new();
                                new_value.insert(operator_key, value.clone());
                                new_filter.insert(key.clone(), new_value);
                            }
                        }
                        finalized.insert("$and".to_string(), vec![new_filter]);
                    }
                }
            }

            // Add the options back to the filter.
            if key == "opts" {
                finalized.insert(key.clone(), value.clone());
            }
        }

        // Parse the provided object eager options and convert them to the correct format.
        let eager_load_options;
        (finalized, eager_load_options) =
            MongoDataSource::convert_object_id_string_to_object_id_from_doc(
                finalized,
                entity,
                subgraph_config,
                &resolver_type,
                None,
            )?;
        eager_filters.extend(eager_load_options);

        trace!("Filter Finalized");
        trace!("Finalized: {:?}", finalized);
        trace!("Total Eager Load Options: {:?}", eager_filters);

        Ok((finalized, eager_filters))
    }

    pub fn create_aggregation(
        query_doc: &Document,
        eager_load_options: Vec<EagerLoadOptions>,
        opts_doc: Option<OptionsInput>,
    ) -> Result<Vec<Document>, async_graphql::Error> {
        debug!("Creating Aggregation");
        trace!("Query Doc: {:?}", query_doc);
        trace!("Eager Load Options: {:?}", eager_load_options);
        trace!("Opts Doc: {:?}", opts_doc);
        let mut pipeline = Vec::new();
        for eager_load_option in eager_load_options {
            let lookup = doc! {
                "$lookup": {
                    "from": eager_load_option.from,
                    "localField": eager_load_option.local_field,
                    "foreignField": eager_load_option.foreign_field,
                    "as": eager_load_option.as_field.clone(),
                }
            };
            pipeline.push(lookup);
            let unwind = doc! {
                "$unwind": {
                    "path": format!("${}", eager_load_option.as_field),
                    "preserveNullAndEmptyArrays": true,
                }
            };
            pipeline.push(unwind);
        }

        let match_doc = doc! {
            "$match": query_doc
        };
        pipeline.push(match_doc);

        // Start the facet pipeline.
        let mut facet_doc = doc! {
            "total_count": [
                {
                    "$count": "total_count"
                }
            ]
        };

        let mut paginated_facet_doc = vec![];

        // Handle sorting and paginating
        if let Some(opts) = opts_doc {
            let mut sort_doc = doc! {};
            let mut skip = 0;
            let mut limit = 10;
            if let Some(sort) = opts.sort {
                for sort_input in sort.iter() {
                    sort_doc.insert(
                        sort_input.field.clone(),
                        match sort_input.direction {
                            DirectionEnum::Asc => 1,
                            DirectionEnum::Desc => -1,
                        },
                    );
                }
            }

            // Get the limit from the opts input
            if let Some(per_page) = opts.per_page {
                limit = per_page;
            }

            // If opts.page and opts.per_page, calculate the new skip and limit values.
            if let Some(page_value) = opts.page {
                if let Some(per_page_value) = opts.per_page {
                    skip = (page_value - 1) * per_page_value;
                    limit = per_page_value;
                }
            }

            trace!("Sort Doc: {:?}", sort_doc);

            if !sort_doc.is_empty() {
                let sort = doc! {
                    "$sort": sort_doc
                };
                paginated_facet_doc.push(sort);
            }
            if skip > 0 {
                let skip = doc! {
                    "$skip": skip
                };
                paginated_facet_doc.push(skip);
            }
            if limit > 0 {
                let limit = doc! {
                    "$limit": limit
                };
                paginated_facet_doc.push(limit);
            }
        }

        facet_doc.insert("documents", paginated_facet_doc);

        pipeline.push(doc! {
            "$facet": facet_doc
        });

        trace!("Pipeline: {:?}", pipeline);
        Ok(pipeline)
    }

    pub async fn execute_operation<'a>(
        data_source: &DataSource,
        mut input: Document,
        entity: ServiceEntityConfig,
        resolver_type: ResolverType,
        subgraph_config: &SubGraphConfig,
    ) -> Result<Option<FieldValue<'a>>, async_graphql::Error> {
        debug!("Executing Operation - Mongo Data Source");
        trace!("Input: {:?}", input);

        let eager_load_options;
        (input, eager_load_options) =
            MongoDataSource::finalize_input(input, &entity, subgraph_config, &resolver_type)?;

        let db = match data_source {
            DataSource::Mongo(ds) => ds.db.clone(),
            _ => unreachable!(),
        };

        debug!("Database Found");

        let collection_name = ServiceEntityConfig::get_mongo_collection_name(&entity);

        debug!("Found Collection Name");
        trace!("{:?}", collection_name);

        match resolver_type {
            ResolverType::FindOne => {
                let result =
                    services::Services::find_one(db, input, collection_name, eager_load_options)
                        .await?;
                let res = ResolverResponse {
                    data: vec![FieldValue::owned_any(result)],
                    meta: ResolverResponseMeta {
                        request_id: uuid::Uuid::new_v4().to_string(),
                        service_name: subgraph_config.service.name.clone(),
                        service_version: subgraph_config.service.version.clone(),
                        executed_at: chrono::Utc::now()
                            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                        count: 1,
                        total_count: 1,
                        page: 1,
                        total_pages: 1,
                        user_uuid: None,
                    },
                };
                Ok(Some(FieldValue::owned_any(res)))
            }
            ResolverType::FindMany => {
                let (results, total_count) = services::Services::find_many(
                    db,
                    input.clone(),
                    collection_name,
                    eager_load_options,
                )
                .await?;
                let opts_doc = if input.clone().get("opts").is_some() {
                    trace!("Options Document Found: {:?}", input.get("opts").unwrap());
                    to_document(input.get("opts").unwrap()).unwrap()
                } else {
                    trace!("Options Document Not Found. Defaulting to 10 per page.");
                    let mut d = Document::new();
                    d.insert("per_page", 10);
                    d.insert("page", 1);
                    trace!("created opts: {:?}", d);
                    d
                };
                let page = if let Some(page_value) = opts_doc.get("page") {
                    page_value.as_i32().unwrap() as i64
                } else {
                    1
                };
                let total_pages = if let Some(per_page_value) = opts_doc.get("per_page") {
                    let mut per_page = per_page_value.as_i32();
                    if per_page.is_none() {
                        let per_page_i64 = per_page_value.as_i64();
                        if per_page_i64.is_none() {
                            warn!("Invalid per_page value. Defaulting to 10.");
                            per_page = Some(10);
                        } else {
                            per_page = Some(per_page_i64.unwrap() as i32);
                        }
                    }
                    if total_count as i32 % per_page.unwrap() as i32 == 0 {
                        total_count as i32 / per_page.unwrap() as i32
                    } else {
                        (total_count as i32 / per_page.unwrap() as i32) + 1
                    }
                } else {
                    1
                };

                let res = ResolverResponse {
                    data: results
                        .clone()
                        .into_iter()
                        .map(|doc| FieldValue::owned_any(doc))
                        .collect(),
                    meta: ResolverResponseMeta {
                        request_id: uuid::Uuid::new_v4().to_string(),
                        service_name: subgraph_config.service.name.clone(),
                        service_version: subgraph_config.service.version.clone(),
                        executed_at: chrono::Utc::now()
                            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                        count: results.len() as i64,
                        total_count: total_count as i64,
                        page,
                        total_pages: total_pages as i64,
                        user_uuid: None,
                    },
                };
                Ok(Some(FieldValue::owned_any(res)))
            }
            ResolverType::CreateOne => {
                let result = services::Services::create_one(db, input, collection_name).await?;
                let res = ResolverResponse {
                    data: vec![FieldValue::owned_any(result)],
                    meta: ResolverResponseMeta {
                        request_id: uuid::Uuid::new_v4().to_string(),
                        service_name: subgraph_config.service.name.clone(),
                        service_version: subgraph_config.service.version.clone(),
                        executed_at: chrono::Utc::now()
                            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                        count: 1,
                        total_count: 1,
                        page: 1,
                        total_pages: 1,
                        user_uuid: None,
                    },
                };
                Ok(Some(FieldValue::owned_any(res)))
            }
            ResolverType::UpdateOne => {
                let result =
                    services::Services::update_one(db, input, collection_name, &entity).await?;
                let res = ResolverResponse {
                    data: vec![FieldValue::owned_any(result)],
                    meta: ResolverResponseMeta {
                        request_id: uuid::Uuid::new_v4().to_string(),
                        service_name: subgraph_config.service.name.clone(),
                        service_version: subgraph_config.service.version.clone(),
                        executed_at: chrono::Utc::now()
                            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                        count: 1,
                        total_count: 1,
                        page: 1,
                        total_pages: 1,
                        user_uuid: None,
                    },
                };
                Ok(Some(FieldValue::owned_any(res)))
            }
            ResolverType::UpdateMany => {
                let results =
                    services::Services::update_many(db, input, collection_name, &entity).await?;
                let count = results.len();
                let res = ResolverResponse {
                    data: results
                        .into_iter()
                        .map(|doc| FieldValue::owned_any(doc))
                        .collect(),
                    meta: ResolverResponseMeta {
                        request_id: uuid::Uuid::new_v4().to_string(),
                        service_name: subgraph_config.service.name.clone(),
                        service_version: subgraph_config.service.version.clone(),
                        executed_at: chrono::Utc::now()
                            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                        count: count as i64,
                        total_count: count as i64,
                        page: 1,
                        total_pages: 1,
                        user_uuid: None,
                    },
                };
                Ok(Some(FieldValue::owned_any(res)))
            }
            _ => panic!("Invalid resolver type"),
        }
    }
}
