use std::marker::PhantomData;

use serde_json::Value as JsonValue;
use teaql_core::{Aggregate, AggregateFunction, EntityDescriptor, Expr, SelectQuery, SmartList};
use teaql_runtime::RuntimeError;

use crate::request_support::*;

impl EntityReference for crate::MediaLibrary {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(&self)
    }
}

impl EntityReference for &crate::MediaLibrary {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(self)
    }
}

// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/media_library
#[derive(Debug)]
pub struct MediaLibraryRequest<R = crate::MediaLibrary> {
    query: SelectQuery,
    relation_selections: Vec<RelationSelection>,
    relation_filters: Vec<RelationFilter>,
    child_enhancements: Vec<QuerySelection>,
    query_options: QueryOptions,
    marker: PhantomData<R>,
}

impl<R> Clone for MediaLibraryRequest<R> {
    fn clone(&self) -> Self {
        Self {
            query: self.query.clone(),
            relation_selections: self.relation_selections.clone(),
            relation_filters: self.relation_filters.clone(),
            child_enhancements: self.child_enhancements.clone(),
            query_options: self.query_options.clone(),
            marker: PhantomData,
        }
    }
}

impl<R> MediaLibraryRequest<R> {
    pub(crate) fn new() -> Self {
        Self {
            query: SelectQuery::new("MediaLibrary")
                .project("id")
                .project("version"),
            relation_selections: Vec::new(),
            relation_filters: Vec::new(),
            child_enhancements: Vec::new(),
            query_options: QueryOptions::default(),
            marker: PhantomData,
        }
    }

    pub fn return_type<T>(self) -> MediaLibraryRequest<T> {
        MediaLibraryRequest {
            query: self.query,
            relation_selections: self.relation_selections,
            relation_filters: self.relation_filters,
            child_enhancements: self.child_enhancements,
            query_options: self.query_options,
            marker: PhantomData,
        }
    }

    pub fn query(&self) -> &SelectQuery {
        &self.query
    }

    pub fn relation_selections(&self) -> &[RelationSelection] {
        &self.relation_selections
    }

    pub fn relation_filters(&self) -> &[RelationFilter] {
        &self.relation_filters
    }

    pub fn child_enhancements(&self) -> &[QuerySelection] {
        &self.child_enhancements
    }

    pub fn query_options(&self) -> &QueryOptions {
        &self.query_options
    }

    pub fn into_query(self) -> SelectQuery {
        self.query
    }


    pub fn purpose(self, purpose: impl Into<String>) -> crate::PurposedQuery<Self> {
        crate::PurposedQuery::new(self, purpose)
    }

    pub(crate) async fn _execute_for_list<'a, C>(
        self,
        context: &'a C,
    ) -> Result<SmartList<R>, RuntimeError>
    where
        C: TeaqlRuntime + ?Sized,
        R: teaql_core::Entity + Send + 'a,
    {
        let query_options = self.query_options.clone();
        let relation_aggregates = runtime_relation_aggregates(&query_options);
        let mut query = authorize_query(apply_runtime_metadata(
            self.query,
            &query_options,
            &self.child_enhancements,
        ))?;
        let requested_count = query.take_requested_list_count(&query_options);
        let use_id_set = query.as_query().id_set_pagination.is_some();
        context.user_context().observe_generated_list("MediaLibrary", async {
        let mut total = None;
        if !use_id_set {
            if let Some((alias, count_query)) = &requested_count {
                let count_rows = context.fetch_compact_rows("MediaLibrary", count_query).await?;
                total = Some(count_rows.first().and_then(|row| row.get(alias))
                    .and_then(teaql_core::Value::try_u64)
                    .ok_or_else(|| RuntimeError::Graph("requested list count is missing or not numeric".into()))?);
            }
        }
        let (mut rows, facets) = if query_options.facets.is_empty() {
            let rows = context.fetch_entity_smart_list::<R>(
                "MediaLibrary",
                query,
                relation_aggregates,
            ).await?;
            (rows, std::collections::BTreeMap::new())
        } else {
            let rows = context.fetch_entity_smart_list::<R>(
                "MediaLibrary",
                query.clone(),
                relation_aggregates,
            ).await?;
            let facets = execute_facets(context, query.as_query(), &query_options)
                .await?;
            (rows, facets)
        };
        if let Some((alias, count_query)) = requested_count {
            if total.is_none() { total = rows.total_count; }
            if total.is_none() {
                let count_rows = context.fetch_compact_rows("MediaLibrary", &count_query).await?;
                total = Some(count_rows.first().and_then(|row| row.get(&alias))
                    .and_then(teaql_core::Value::try_u64)
                    .ok_or_else(|| RuntimeError::Graph("requested list count is missing or not numeric".into()))?);
            }
            rows.total_count = total;
            rows.aggregations.insert(alias, teaql_core::Value::U64(total.expect("requested count resolved")));
        }
        attach_facets(&mut rows, facets);
        Ok(rows)
        }).await
    }

    pub(crate) async fn _execute_for_rows<'a, C>(
        self,
        context: &'a C,
    ) -> Result<SmartList<teaql_core::CompactRow>, RuntimeError>
    where
        C: TeaqlRuntime + ?Sized,
    {
        let query = authorize_query(apply_runtime_metadata(
            self.query,
            &self.query_options,
            &self.child_enhancements,
        ))?;
        context.fetch_compact_smart_list("MediaLibrary", &query).await
    }

    pub(crate) async fn _execute_for_stream<'a, C>(
        self,
        context: &'a C,
    ) -> Result<TeaqlEntityStream<'a, R, RuntimeError>, RuntimeError>
    where
        C: TeaqlRuntime + ?Sized,
        R: teaql_core::Entity + Send + 'a,
    {
        let query = authorize_query(apply_runtime_metadata(
            self.query,
            &self.query_options,
            &self.child_enhancements,
        ))?;
        Ok(context.user_context().observe_generated_stream(
            "MediaLibrary", context.fetch_entity_stream("MediaLibrary", query),
        ))
    }

    pub(crate) async fn _execute_for_first<'a, C>(
        self,
        context: &'a C,
    ) -> Result<Option<R>, RuntimeError>
    where
        C: TeaqlRuntime + ?Sized,
        R: teaql_core::Entity + Send + 'a,
    {
        let rows = self.limit(1)._execute_for_list(context).await?;
        Ok(rows.into_iter().next())
    }

    pub(crate) async fn _execute_for_one<'a, C>(
        self,
        context: &'a C,
    ) -> Result<Option<R>, RuntimeError>
    where
        C: TeaqlRuntime + ?Sized,
        R: teaql_core::Entity + Send + 'a,
    {
        self._execute_for_first(context).await
    }


    pub(crate) async fn _execute_for_count<'a, C>(
        self,
        context: &'a C,
    ) -> Result<u64, RuntimeError>
    where
        C: TeaqlRuntime + ?Sized,
    {
        let query_options = self.query_options.clone();
        let query = apply_runtime_metadata(
            self.query,
            &query_options,
            &self.child_enhancements,
        );
        let operation = query_options.page_count_operation.clone().unwrap_or_else(||
            QueryOperation::new(QueryOperationKind::NamedCount, "execute_for_count"));
        let alias = operation.alias().unwrap_or(COUNT_ALIAS).to_owned();
        let query = authorize_query(query)?.with_facet_diagnostics(&query_options)
            .for_exact_count_with_operation(&alias, operation);
        let rows = context.fetch_compact_rows("MediaLibrary", &query).await?;
        rows.first()
            .and_then(|row| row.get(&alias))
            .and_then(teaql_core::Value::try_u64)
            .ok_or_else(|| RuntimeError::Graph(format!("count result for MediaLibrary is missing or not numeric")))
    }

    pub(crate) async fn _execute_for_exists<'a, C>(
        self,
        context: &'a C,
    ) -> Result<bool, RuntimeError>
    where
        C: TeaqlRuntime + ?Sized,
    {
        let mut query = apply_runtime_metadata(
            self.query,
            &self.query_options,
            &self.child_enhancements,
        ).limit(1);
        query.relations.clear();
        let query = authorize_query(query)?;
        let rows = context.fetch_compact_rows("MediaLibrary", &query).await?;
        Ok(!rows.is_empty())
    }

    pub fn search_with_text(mut self, text: impl Into<String>) -> Self {
        self.query = self.query.search_with_text(text);
        self
    }

    pub fn filter(mut self, filter: Expr) -> Self {
        self.query = self.query.filter(filter);
        self
    }

    pub fn and_filter(mut self, filter: Expr) -> Self {
        self.query = self.query.and_filter(filter);
        self
    }

    pub fn or_filter(mut self, filter: Expr) -> Self {
        self.query = self.query.or_filter(filter);
        self
    }

    pub fn append_search_criteria(self, criteria: Expr) -> Self {
        self.and_filter(criteria)
    }

    pub fn filter_property(
        mut self,
        property1: impl AsRef<str>,
        operator: FieldOperator,
        property2: impl AsRef<str>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_column_expr(
            property1.as_ref(),
            operator,
            property2.as_ref(),
        ));
        self
    }

    pub fn with_deleted_rows(mut self) -> Self {
        self.query.filter = remove_default_live_filter(self.query.filter);
        self
    }

    pub fn deleted_rows_only(mut self) -> Self {
        self.query.filter = remove_default_live_filter(self.query.filter);
        self.query = self.query.and_filter(Expr::lte("version", 0_i64));
        self
    }

    pub fn match_types(
        mut self,
        types: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(TYPE_FIELD, types.into_iter().map(Into::into)));
        self
    }


    pub fn with_type_group(mut self) -> Self {
        self.query = self.query.project(TYPE_GROUP_FIELD);
        self
    }

    pub fn matching_any_of(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        let entity = EntityDescriptor::new(selection.query.entity.clone());
        self.query = self.query.and_filter(Expr::in_subquery("id", entity, selection.query.clone(), "id"));
        self
    }

    pub fn match_any_of(self, request: impl Into<QuerySelection>) -> Self {
        self.matching_any_of(request)
    }

    pub fn enhance_child(mut self, request: impl Into<QuerySelection>) -> Self {
        self.child_enhancements.push(request.into());
        self
    }

    pub fn enhance_children_if_needed(self) -> Self {
        let request = self;
        request
    }


    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.query_options.comment = Some(comment.into());
        self
    }

    pub fn raw_sql(self, raw_sql: impl Into<String>) -> Self {
        self.unsafe_raw_sql(UnsafeRawSqlSegment::trusted(raw_sql))
    }

    pub fn unsafe_raw_sql(mut self, raw_sql: UnsafeRawSqlSegment) -> Self {
        self.query_options.raw_sql = Some(raw_sql.into_sql());
        self
    }

    pub fn raw_sql_filter(self, raw_sql: impl Into<String>) -> Self {
        self.unsafe_raw_sql_filter(UnsafeRawSqlSegment::trusted(raw_sql))
    }

    pub fn unsafe_raw_sql_filter(mut self, raw_sql: UnsafeRawSqlSegment) -> Self {
        self.query_options.raw_sql_search_criteria.push(raw_sql.into_sql());
        self
    }
    pub fn filter_with_json(self, json_expr: impl Into<String>) -> Self {
        self.merge_dynamic_json_expr(json_expr.into())
    }

    fn merge_dynamic_json_expr(self, json_expr: String) -> Self {
        let json = serde_json::from_str::<JsonValue>(&json_expr)
            .unwrap_or_else(|_| panic!("Input JSON format error: {json_expr}"));
        self.merge_dynamic_json(&json)
    }

    fn merge_dynamic_json(mut self, json: &JsonValue) -> Self {
        let Some(object) = json.as_object() else {
            return self;
        };

        for (field, value) in object {
            if field.starts_with('_') {
                continue;
            }
            self = self.apply_dynamic_json_filter(field, value);
        }

        self = self.apply_dynamic_json_order_by(object.get("_orderBy"));

        if let Some(offset) = dynamic_json_u64_field(object, "_start") {
            self = self.skip(offset);
        }
        if let Some(size) = dynamic_json_u64_field(object, "_size") {
            self = self.limit(size);
        }

        if let Some(page_size) = dynamic_json_u64_field(object, "_pageSize") {
            self = self.limit(page_size);
        }
        if let Some(page_number) = dynamic_json_u64_field(object, "_page") {
            if page_number > 0 {
                let size = dynamic_json_u64_field(object, "_pageSize")
                    .or_else(|| self.query.slice.as_ref().and_then(|slice| slice.limit))
                    .unwrap_or(10);
                let offset = page_number.saturating_sub(1).saturating_mul(size);
                self = self.page_offset(offset, size);
            }
        }

        self
    }

    pub(crate) fn apply_dynamic_json_filter(self, field: &str, value: &JsonValue) -> Self {
        if let Some((head, tail)) = field.split_once('.') {
            self.apply_dynamic_json_chain_filter(head, tail, value)
        } else if let Some(storage_field) = Self::dynamic_json_self_field(field) {
            self.and_filter(dynamic_json_filter_expr(storage_field, value))
        } else {
            self
        }
    }

    fn apply_dynamic_json_order_by(mut self, order_by: Option<&JsonValue>) -> Self {
        match order_by {
            Some(JsonValue::String(field)) => {
                if let Some(storage_field) = Self::dynamic_json_self_field(field) {
                    self.query = self.query.order_desc(storage_field);
                }
            }
            Some(JsonValue::Object(order_by)) => {
                self = self.apply_dynamic_json_single_order_by(order_by);
            }
            Some(JsonValue::Array(order_bys)) => {
                for order_by in order_bys {
                    if let Some(order_by) = order_by.as_object() {
                        self = self.apply_dynamic_json_single_order_by(order_by);
                    }
                }
            }
            _ => {}
        }
        self
    }

    fn apply_dynamic_json_single_order_by(
        mut self,
        order_by: &serde_json::Map<String, JsonValue>,
    ) -> Self {
        let Some(field) = order_by.get("field").and_then(JsonValue::as_str) else {
            return self;
        };
        let Some(storage_field) = Self::dynamic_json_self_field(field) else {
            return self;
        };
        if order_by
            .get("useAsc")
            .and_then(JsonValue::as_bool)
            .unwrap_or(false)
        {
            self.query = self.query.order_asc(storage_field);
        } else {
            self.query = self.query.order_desc(storage_field);
        }
        self
    }

    fn dynamic_json_self_field(field: &str) -> Option<&'static str> {
        match field {
            "id" => Some("id"),
            "library_name" => Some("library_name"),
            "root_path" => Some("root_path"),
            "create_time" => Some("create_time"),
            "update_time" => Some("update_time"),
            "version" => Some("version"),
            _ => None,
        }
    }

    fn apply_dynamic_json_chain_filter(self, head: &str, tail: &str, value: &JsonValue) -> Self {
        let _ = (tail, value);
        match head {
            "media_asset_list" => {
                self.with_media_asset_list_matching(
                    crate::Q::media_assets_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "grant_status_list" => {
                self.with_grant_status_list_matching(
                    crate::Q::grant_statuses_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "ip_access_grant_list" => {
                self.with_ip_access_grant_list_matching(
                    crate::Q::ip_access_grants_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "invite_status_list" => {
                self.with_invite_status_list_matching(
                    crate::Q::invite_statuses_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "invite_code_list" => {
                self.with_invite_code_list_matching(
                    crate::Q::invite_codes_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "app_setting_list" => {
                self.with_app_setting_list_matching(
                    crate::Q::app_settings_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "proxy_trust_rule_list" => {
                self.with_proxy_trust_rule_list_matching(
                    crate::Q::proxy_trust_rules_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "share_handle_list" => {
                self.with_share_handle_list_matching(
                    crate::Q::share_handles_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            _ => self,
        }
    }

    pub fn create_property_as(
        self,
        property_name: impl Into<String>,
        raw_sql_segment: impl Into<String>,
    ) -> Self {
        self.unsafe_create_property_as(property_name, UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn unsafe_create_property_as(
        mut self,
        property_name: impl Into<String>,
        raw_sql_segment: UnsafeRawSqlSegment,
    ) -> Self {
        self.query_options
            .dynamic_properties
            .push(RawDynamicProperty::new(property_name, raw_sql_segment));
        self
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.query = self.query.limit(limit);
        self
    }

    pub fn stream(mut self, chunk_size: usize) -> Self {
        assert!(chunk_size > 0, "stream chunk size must be positive");
        self.query = self.query.stream(chunk_size);
        self
    }

    pub fn stream_default(mut self) -> Self {
        self.query = self.query.stream_default();
        self
    }

    pub fn skip(mut self, offset: u64) -> Self {
        self.query = self.query.offset(offset);
        self
    }

    pub fn offset_only(self, offset: u64) -> Self {
        self.skip(offset)
    }

    pub fn offset(self, offset: u64, size: u64) -> Self {
        self.page_offset(offset, size)
    }

    pub fn page_offset(mut self, offset: u64, limit: u64) -> Self {
        self.query = self.query.page(offset, limit);
        self
    }

    pub fn optimize_for_continuous_page_fetch(mut self) -> Self {
        self.query = self.query.optimize_for_continuous_page_fetch();
        self
    }

    pub fn optimize_for_continuous_page_fetch_with(
        mut self,
        namespace: impl Into<String>,
        ttl_seconds: u64,
    ) -> Self {
        self.query = self
            .query
            .optimize_for_continuous_page_fetch_with(namespace, ttl_seconds);
        self
    }

    pub fn optimize_pagination_with_id_set(mut self) -> Self {
        self.query = self.query.optimize_pagination_with_id_set();
        self
    }

    pub fn optimize_pagination_with_id_set_config(
        mut self,
        namespace: impl Into<String>,
        ttl_seconds: u64,
        max_ids: u64,
    ) -> Self {
        self.query = self
            .query
            .optimize_pagination_with_id_set_config(namespace, ttl_seconds, max_ids);
        self
    }

    /// Select bounded indexed probes for a per-parent Top-N relation only
    /// when the already-loaded parent count is at or below `threshold`.
    /// Passing zero explicitly selects the provider window plan.
    pub fn top_n_probe_parent_threshold(mut self, threshold: usize) -> Self {
        self.query = self.query.top_n_probe_parent_threshold(threshold);
        self
    }

    pub fn top(self, top_n: u64) -> Self {
        self.limit(top_n)
    }

    pub fn offset_size(self, offset: u64, size: u64) -> Self {
        self.offset(offset, size)
    }

    pub fn unlimited(mut self) -> Self {
        self.query.slice = None;
        self
    }

    pub fn page_number(self, page_number: u64, page_size: u64) -> Self {
        let offset = page_number.saturating_sub(1).saturating_mul(page_size);
        self.page_offset(offset, page_size)
    }

    pub fn page_number_default(self, page_number: u64) -> Self {
        self.page_number(page_number, 10)
    }

    pub fn page(self, page_number: u64, page_size: u64) -> Self {
        self.page_number(page_number, page_size)
    }

    pub fn page_default(self, page_number: u64) -> Self {
        self.page_number_default(page_number)
    }

    pub fn select_self(mut self) -> Self {
        self.query = self.query.project("id");
        self.query = self.query.project("library_name");
        self.query = self.query.project("root_path");
        self.query = self.query.project("create_time");
        self.query = self.query.project("update_time");
        self.query = self.query.project("version");
        self
    }

    pub fn select_dynamic_fields_with(mut self, selection: teaql_core::dynamic_fields::DynamicFieldSelection) -> Self {
        self.query = self.query.select_dynamic_fields(selection);
        self
    }

    pub fn select_self_fields(self) -> Self {
        self.select_self()
    }

    pub fn select_self_without_parent(self) -> Self {
        self.select_self_fields()
    }

    pub fn select_all(self) -> Self {
        self.select_self()
    }

    pub fn select_children(self) -> Self {
        let mut request = self.select_all();
        request = request.select_media_asset_list();
        request = request.select_grant_status_list();
        request = request.select_ip_access_grant_list();
        request = request.select_invite_status_list();
        request = request.select_invite_code_list();
        request = request.select_app_setting_list();
        request = request.select_proxy_trust_rule_list();
        request = request.select_share_handle_list();
        request
    }

    pub fn select_any(self) -> Self {
        self.select_children()
    }

    pub fn group_by(mut self, field: impl Into<String>) -> Self {
        self.query = self.query.group_by(field);
        self
    }

    pub fn count(mut self) -> Self {
        self.query_options.page_count_operation = Some(QueryOperation::new(QueryOperationKind::NamedCount, "count"));
        self.aggregate_count("count")
    }

    pub fn count_as(mut self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        self.query_options.page_count_operation = Some(QueryOperation::new(QueryOperationKind::NamedCount, "count_as").with_alias(&alias));
        self.aggregate_count(alias)
    }

    pub fn aggregate_count(mut self, alias: impl Into<String>) -> Self {
        self.query = self.query.count(alias);
        self
    }

    pub fn aggregate_count_field(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.count_field(field, alias);
        self
    }

    pub fn aggregate_with_function(
        mut self,
        field: impl Into<String>,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.query = self.query.aggregate(Aggregate::new(function, field, alias));
        self
    }

    pub fn aggregate_sum(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.sum(field, alias);
        self
    }

    pub fn aggregate_avg(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.avg(field, alias);
        self
    }

    pub fn aggregate_min(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.min(field, alias);
        self
    }

    pub fn aggregate_max(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.max(field, alias);
        self
    }

    pub fn aggregate_stddev(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.stddev(field, alias);
        self
    }

    pub fn aggregate_stddev_pop(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.stddev_pop(field, alias);
        self
    }

    pub fn aggregate_var_samp(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.var_samp(field, alias);
        self
    }

    pub fn aggregate_var_pop(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.var_pop(field, alias);
        self
    }

    pub fn aggregate_bit_and(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.bit_and(field, alias);
        self
    }

    pub fn aggregate_bit_or(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.bit_or(field, alias);
        self
    }

    pub fn aggregate_bit_xor(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.bit_xor(field, alias);
        self
    }

    pub fn enable_aggregation_cache(mut self) -> Self {
        self.query = self.query.enable_aggregation_cache();
        self
    }

    pub fn enable_aggregation_cache_for(mut self, cache_expired_millis: u64) -> Self {
        self.query = self.query.enable_aggregation_cache_for(cache_expired_millis);
        self
    }

    pub fn propagate_aggregation_cache(mut self, cache_expired_millis: u64) -> Self {
        self.query = self.query.propagate_aggregation_cache(cache_expired_millis);
        self
    }

    pub fn group_by_id(self) -> Self {
        self.group_by("id")
    }

    pub fn group_by_id_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("id"));
        request
    }

    pub fn group_by_id_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("id")
            .aggregate_with_function("id", alias, function)
    }

    pub fn count_id(self) -> Self {
        self.count_id_as("id_count")
    }

    pub fn count_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("id", alias)
    }

    pub fn sum_id(self) -> Self {
        self.sum_id_as("sum_id")
    }

    pub fn sum_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("id", alias)
    }

    pub fn avg_id(self) -> Self {
        self.avg_id_as("avg_id")
    }

    pub fn avg_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("id", alias)
    }

    pub fn min_id(self) -> Self {
        self.min_id_as("min_id")
    }

    pub fn min_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("id", alias)
    }

    pub fn max_id(self) -> Self {
        self.max_id_as("max_id")
    }

    pub fn max_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("id", alias)
    }


    pub fn with_id(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "id",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_id_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "id",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_id_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("id", value));
        self
    }



    pub fn with_id_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("id", value));
        self
    }

    pub fn with_id_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "id",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_id_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "id",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn order_by_id_asc(mut self) -> Self {
        self.query = self.query.order_asc("id");
        self
    }

    pub fn order_by_id_desc(mut self) -> Self {
        self.query = self.query.order_desc("id");
        self
    }

    pub fn order_by_id_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("id");
        self
    }

    pub fn order_by_id_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("id");
        self
    }


    pub fn select_library_name(mut self) -> Self {
        self.query = self.query.project("library_name");
        self
    }

    pub fn project_library_name(self) -> Self {
        self.select_library_name()
    }

    pub fn select_library_name_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_library_name_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_library_name_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("library_name", raw_sql_segment));
        self
    }

    pub fn group_by_library_name(self) -> Self {
        self.group_by("library_name")
    }

    pub fn group_by_library_name_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("library_name");
        request.query = request
            .query
            .project_expr(alias, Expr::column("library_name"));
        request
    }

    pub fn group_by_library_name_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("library_name")
            .aggregate_with_function("library_name", alias, function)
    }

    pub fn count_library_name(self) -> Self {
        self.count_library_name_as("library_name_count")
    }

    pub fn count_library_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("library_name", alias)
    }

    pub fn sum_library_name(self) -> Self {
        self.sum_library_name_as("sum_library_name")
    }

    pub fn sum_library_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("library_name", alias)
    }

    pub fn avg_library_name(self) -> Self {
        self.avg_library_name_as("avg_library_name")
    }

    pub fn avg_library_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("library_name", alias)
    }

    pub fn min_library_name(self) -> Self {
        self.min_library_name_as("min_library_name")
    }

    pub fn min_library_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("library_name", alias)
    }

    pub fn max_library_name(self) -> Self {
        self.max_library_name_as("max_library_name")
    }

    pub fn max_library_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("library_name", alias)
    }

    pub fn unselect_library_name(mut self) -> Self {
        self.query.projection.retain(|field| field != "library_name");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "library_name");
        self
    }


    pub fn with_library_name(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "library_name",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_library_name_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "library_name",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_library_name_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("library_name", value));
        self
    }



    pub fn with_library_name_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("library_name", value));
        self
    }

    pub fn with_library_name_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("library_name", value));
        self
    }

    pub fn with_library_name_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("library_name", value));
        self
    }

    pub fn with_library_name_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("library_name", value));
        self
    }

    pub fn with_library_name_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("library_name", value));
        self
    }

    pub fn with_library_name_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("library_name", lower, upper));
        self
    }

    pub fn with_library_name_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "library_name",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_library_name_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "library_name",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_library_name_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "library_name",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_library_name_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("library_name", value));
        self
    }

    pub fn with_library_name_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("library_name", value));
        self
    }

    pub fn with_library_name_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("library_name", value));
        self
    }

    pub fn with_library_name_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("library_name", value));
        self
    }

    pub fn with_library_name_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("library_name", value));
        self
    }

    pub fn with_library_name_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("library_name", value));
        self
    }

    pub fn with_library_name_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("library_name", value));
        self
    }
    pub fn with_library_name_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("library_name", value));
        self
    }

    pub fn with_library_name_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("library_name", value));
        self
    }

    pub fn with_library_name_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("library_name"));
        self
    }



    pub fn with_library_name_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("library_name"));
        self
    }


    pub fn order_by_library_name_asc(mut self) -> Self {
        self.query = self.query.order_asc("library_name");
        self
    }

    pub fn order_by_library_name_desc(mut self) -> Self {
        self.query = self.query.order_desc("library_name");
        self
    }

    pub fn order_by_library_name_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("library_name");
        self
    }

    pub fn order_by_library_name_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("library_name");
        self
    }


    pub fn select_root_path(mut self) -> Self {
        self.query = self.query.project("root_path");
        self
    }

    pub fn project_root_path(self) -> Self {
        self.select_root_path()
    }

    pub fn select_root_path_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_root_path_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_root_path_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("root_path", raw_sql_segment));
        self
    }

    pub fn group_by_root_path(self) -> Self {
        self.group_by("root_path")
    }

    pub fn group_by_root_path_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("root_path");
        request.query = request
            .query
            .project_expr(alias, Expr::column("root_path"));
        request
    }

    pub fn group_by_root_path_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("root_path")
            .aggregate_with_function("root_path", alias, function)
    }

    pub fn count_root_path(self) -> Self {
        self.count_root_path_as("root_path_count")
    }

    pub fn count_root_path_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("root_path", alias)
    }

    pub fn sum_root_path(self) -> Self {
        self.sum_root_path_as("sum_root_path")
    }

    pub fn sum_root_path_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("root_path", alias)
    }

    pub fn avg_root_path(self) -> Self {
        self.avg_root_path_as("avg_root_path")
    }

    pub fn avg_root_path_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("root_path", alias)
    }

    pub fn min_root_path(self) -> Self {
        self.min_root_path_as("min_root_path")
    }

    pub fn min_root_path_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("root_path", alias)
    }

    pub fn max_root_path(self) -> Self {
        self.max_root_path_as("max_root_path")
    }

    pub fn max_root_path_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("root_path", alias)
    }

    pub fn unselect_root_path(mut self) -> Self {
        self.query.projection.retain(|field| field != "root_path");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "root_path");
        self
    }


    pub fn with_root_path(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "root_path",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_root_path_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "root_path",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_root_path_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("root_path", value));
        self
    }



    pub fn with_root_path_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("root_path", value));
        self
    }

    pub fn with_root_path_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("root_path", value));
        self
    }

    pub fn with_root_path_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("root_path", value));
        self
    }

    pub fn with_root_path_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("root_path", value));
        self
    }

    pub fn with_root_path_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("root_path", value));
        self
    }

    pub fn with_root_path_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("root_path", lower, upper));
        self
    }

    pub fn with_root_path_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "root_path",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_root_path_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "root_path",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_root_path_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "root_path",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_root_path_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("root_path", value));
        self
    }

    pub fn with_root_path_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("root_path", value));
        self
    }

    pub fn with_root_path_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("root_path", value));
        self
    }

    pub fn with_root_path_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("root_path", value));
        self
    }

    pub fn with_root_path_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("root_path", value));
        self
    }

    pub fn with_root_path_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("root_path", value));
        self
    }

    pub fn with_root_path_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("root_path", value));
        self
    }
    pub fn with_root_path_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("root_path", value));
        self
    }

    pub fn with_root_path_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("root_path", value));
        self
    }

    pub fn with_root_path_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("root_path"));
        self
    }



    pub fn with_root_path_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("root_path"));
        self
    }


    pub fn order_by_root_path_asc(mut self) -> Self {
        self.query = self.query.order_asc("root_path");
        self
    }

    pub fn order_by_root_path_desc(mut self) -> Self {
        self.query = self.query.order_desc("root_path");
        self
    }

    pub fn order_by_root_path_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("root_path");
        self
    }

    pub fn order_by_root_path_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("root_path");
        self
    }


    pub fn select_create_time(mut self) -> Self {
        self.query = self.query.project("create_time");
        self
    }

    pub fn project_create_time(self) -> Self {
        self.select_create_time()
    }

    pub fn select_create_time_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_create_time_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_create_time_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("create_time", raw_sql_segment));
        self
    }

    pub fn group_by_create_time(self) -> Self {
        self.group_by("create_time")
    }

    pub fn group_by_create_time_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("create_time");
        request.query = request
            .query
            .project_expr(alias, Expr::column("create_time"));
        request
    }

    pub fn group_by_create_time_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("create_time")
            .aggregate_with_function("create_time", alias, function)
    }

    pub fn count_create_time(self) -> Self {
        self.count_create_time_as("create_time_count")
    }

    pub fn count_create_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("create_time", alias)
    }

    pub fn sum_create_time(self) -> Self {
        self.sum_create_time_as("sum_create_time")
    }

    pub fn sum_create_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("create_time", alias)
    }

    pub fn avg_create_time(self) -> Self {
        self.avg_create_time_as("avg_create_time")
    }

    pub fn avg_create_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("create_time", alias)
    }

    pub fn min_create_time(self) -> Self {
        self.min_create_time_as("min_create_time")
    }

    pub fn min_create_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("create_time", alias)
    }

    pub fn max_create_time(self) -> Self {
        self.max_create_time_as("max_create_time")
    }

    pub fn max_create_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("create_time", alias)
    }

    pub fn unselect_create_time(mut self) -> Self {
        self.query.projection.retain(|field| field != "create_time");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "create_time");
        self
    }


    pub fn with_create_time(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "create_time",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_create_time_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "create_time",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_create_time_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("create_time", value));
        self
    }



    pub fn with_create_time_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("create_time", value));
        self
    }

    pub fn with_create_time_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("create_time", value));
        self
    }

    pub fn with_create_time_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("create_time", value));
        self
    }

    pub fn with_create_time_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("create_time", value));
        self
    }

    pub fn with_create_time_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("create_time", value));
        self
    }

    pub fn with_create_time_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("create_time", lower, upper));
        self
    }

    pub fn with_create_time_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "create_time",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_create_time_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "create_time",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_create_time_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "create_time",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_create_time_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("create_time", value));
        self
    }

    pub fn with_create_time_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("create_time", value));
        self
    }

    pub fn with_create_time_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("create_time"));
        self
    }



    pub fn with_create_time_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("create_time"));
        self
    }


    pub fn order_by_create_time_asc(mut self) -> Self {
        self.query = self.query.order_asc("create_time");
        self
    }

    pub fn order_by_create_time_desc(mut self) -> Self {
        self.query = self.query.order_desc("create_time");
        self
    }

    pub fn order_by_create_time_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("create_time");
        self
    }

    pub fn order_by_create_time_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("create_time");
        self
    }


    pub fn select_update_time(mut self) -> Self {
        self.query = self.query.project("update_time");
        self
    }

    pub fn project_update_time(self) -> Self {
        self.select_update_time()
    }

    pub fn select_update_time_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_update_time_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_update_time_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("update_time", raw_sql_segment));
        self
    }

    pub fn group_by_update_time(self) -> Self {
        self.group_by("update_time")
    }

    pub fn group_by_update_time_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("update_time");
        request.query = request
            .query
            .project_expr(alias, Expr::column("update_time"));
        request
    }

    pub fn group_by_update_time_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("update_time")
            .aggregate_with_function("update_time", alias, function)
    }

    pub fn count_update_time(self) -> Self {
        self.count_update_time_as("update_time_count")
    }

    pub fn count_update_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("update_time", alias)
    }

    pub fn sum_update_time(self) -> Self {
        self.sum_update_time_as("sum_update_time")
    }

    pub fn sum_update_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("update_time", alias)
    }

    pub fn avg_update_time(self) -> Self {
        self.avg_update_time_as("avg_update_time")
    }

    pub fn avg_update_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("update_time", alias)
    }

    pub fn min_update_time(self) -> Self {
        self.min_update_time_as("min_update_time")
    }

    pub fn min_update_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("update_time", alias)
    }

    pub fn max_update_time(self) -> Self {
        self.max_update_time_as("max_update_time")
    }

    pub fn max_update_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("update_time", alias)
    }

    pub fn unselect_update_time(mut self) -> Self {
        self.query.projection.retain(|field| field != "update_time");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "update_time");
        self
    }


    pub fn with_update_time(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "update_time",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_update_time_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "update_time",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_update_time_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("update_time", value));
        self
    }



    pub fn with_update_time_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("update_time", value));
        self
    }

    pub fn with_update_time_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("update_time", value));
        self
    }

    pub fn with_update_time_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("update_time", value));
        self
    }

    pub fn with_update_time_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("update_time", value));
        self
    }

    pub fn with_update_time_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("update_time", value));
        self
    }

    pub fn with_update_time_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("update_time", lower, upper));
        self
    }

    pub fn with_update_time_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "update_time",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_update_time_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "update_time",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_update_time_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "update_time",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_update_time_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("update_time", value));
        self
    }

    pub fn with_update_time_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("update_time", value));
        self
    }

    pub fn with_update_time_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("update_time"));
        self
    }



    pub fn with_update_time_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("update_time"));
        self
    }


    pub fn order_by_update_time_asc(mut self) -> Self {
        self.query = self.query.order_asc("update_time");
        self
    }

    pub fn order_by_update_time_desc(mut self) -> Self {
        self.query = self.query.order_desc("update_time");
        self
    }

    pub fn order_by_update_time_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("update_time");
        self
    }

    pub fn order_by_update_time_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("update_time");
        self
    }

    pub fn group_by_version(self) -> Self {
        self.group_by("version")
    }

    pub fn group_by_version_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("version");
        request.query = request
            .query
            .project_expr(alias, Expr::column("version"));
        request
    }

    pub fn group_by_version_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("version")
            .aggregate_with_function("version", alias, function)
    }

    pub fn count_version(self) -> Self {
        self.count_version_as("version_count")
    }

    pub fn count_version_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("version", alias)
    }

    pub fn sum_version(self) -> Self {
        self.sum_version_as("sum_version")
    }

    pub fn sum_version_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("version", alias)
    }

    pub fn avg_version(self) -> Self {
        self.avg_version_as("avg_version")
    }

    pub fn avg_version_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("version", alias)
    }

    pub fn min_version(self) -> Self {
        self.min_version_as("min_version")
    }

    pub fn min_version_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("version", alias)
    }

    pub fn max_version(self) -> Self {
        self.max_version_as("max_version")
    }

    pub fn max_version_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("version", alias)
    }

    pub fn order_by_version_asc(mut self) -> Self {
        self.query = self.query.order_asc("version");
        self
    }

    pub fn order_by_version_desc(mut self) -> Self {
        self.query = self.query.order_desc("version");
        self
    }

    pub fn order_by_version_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("version");
        self
    }

    pub fn order_by_version_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("version");
        self
    }
    pub fn with_library_name_is_home_video_library(self) -> Self {
        self.with_library_name_is("Home Video Library")
    }



    pub fn with_library_name_is_not_home_video_library(self) -> Self {
        self.with_library_name_is_not("Home Video Library")
    }



    pub fn with_root_path_is_srv_media(self) -> Self {
        self.with_root_path_is("/srv/media")
    }



    pub fn with_root_path_is_not_srv_media(self) -> Self {
        self.with_root_path_is_not("/srv/media")
    }



    pub fn with_create_time_is_create_time(self) -> Self {
        self.with_create_time_is("createTime()")
    }



    pub fn with_create_time_is_not_create_time(self) -> Self {
        self.with_create_time_is_not("createTime()")
    }



    pub fn with_update_time_is_update_time(self) -> Self {
        self.with_update_time_is("updateTime()")
    }



    pub fn with_update_time_is_not_update_time(self) -> Self {
        self.with_update_time_is_not("updateTime()")
    }




    pub fn have_media_assets(self) -> Self {
        self.with_media_asset_list_matching(crate::Q::media_assets_minimal())
    }

    pub fn have_no_media_assets(self) -> Self {
        self.without_media_asset_list_matching(crate::Q::media_assets_minimal())
    }

    pub fn with_media_asset_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::MediaAsset as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "media_library_id",
        ));
        self.relation_filters.push(RelationFilter::new("media_asset_list", selection));
        self
    }

    pub fn without_media_asset_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::MediaAsset as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "media_library_id",
        ));
        self.relation_filters.push(RelationFilter::new("media_asset_list", selection));
        self
    }

    pub fn select_media_asset_list(mut self) -> Self {
        self.query.relations.push(teaql_core::RelationLoad::new("media_asset_list")
            .with_operation(QueryOperation::new(QueryOperationKind::RelationSelect, "select_media_asset_list").with_relation("media_asset_list")));
        self
    }

    pub fn select_media_asset_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query.relations.push(teaql_core::RelationLoad::with_query("media_asset_list", selection.into_query())
            .with_operation(QueryOperation::new(QueryOperationKind::RelationSelect, "select_media_asset_list_with").with_relation("media_asset_list")));
        self
}

    pub fn have_grant_statuses(self) -> Self {
        self.with_grant_status_list_matching(crate::Q::grant_statuses_minimal())
    }

    pub fn have_no_grant_statuses(self) -> Self {
        self.without_grant_status_list_matching(crate::Q::grant_statuses_minimal())
    }

    pub fn with_grant_status_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::GrantStatus as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "media_library_id",
        ));
        self.relation_filters.push(RelationFilter::new("grant_status_list", selection));
        self
    }

    pub fn without_grant_status_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::GrantStatus as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "media_library_id",
        ));
        self.relation_filters.push(RelationFilter::new("grant_status_list", selection));
        self
    }

    pub fn select_grant_status_list(mut self) -> Self {
        self.query.relations.push(teaql_core::RelationLoad::new("grant_status_list")
            .with_operation(QueryOperation::new(QueryOperationKind::RelationSelect, "select_grant_status_list").with_relation("grant_status_list")));
        self
    }

    pub fn select_grant_status_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query.relations.push(teaql_core::RelationLoad::with_query("grant_status_list", selection.into_query())
            .with_operation(QueryOperation::new(QueryOperationKind::RelationSelect, "select_grant_status_list_with").with_relation("grant_status_list")));
        self
}

    pub fn have_ip_access_grants(self) -> Self {
        self.with_ip_access_grant_list_matching(crate::Q::ip_access_grants_minimal())
    }

    pub fn have_no_ip_access_grants(self) -> Self {
        self.without_ip_access_grant_list_matching(crate::Q::ip_access_grants_minimal())
    }

    pub fn with_ip_access_grant_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::IpAccessGrant as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "media_library_id",
        ));
        self.relation_filters.push(RelationFilter::new("ip_access_grant_list", selection));
        self
    }

    pub fn without_ip_access_grant_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::IpAccessGrant as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "media_library_id",
        ));
        self.relation_filters.push(RelationFilter::new("ip_access_grant_list", selection));
        self
    }

    pub fn select_ip_access_grant_list(mut self) -> Self {
        self.query.relations.push(teaql_core::RelationLoad::new("ip_access_grant_list")
            .with_operation(QueryOperation::new(QueryOperationKind::RelationSelect, "select_ip_access_grant_list").with_relation("ip_access_grant_list")));
        self
    }

    pub fn select_ip_access_grant_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query.relations.push(teaql_core::RelationLoad::with_query("ip_access_grant_list", selection.into_query())
            .with_operation(QueryOperation::new(QueryOperationKind::RelationSelect, "select_ip_access_grant_list_with").with_relation("ip_access_grant_list")));
        self
}

    pub fn have_invite_statuses(self) -> Self {
        self.with_invite_status_list_matching(crate::Q::invite_statuses_minimal())
    }

    pub fn have_no_invite_statuses(self) -> Self {
        self.without_invite_status_list_matching(crate::Q::invite_statuses_minimal())
    }

    pub fn with_invite_status_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::InviteStatus as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "media_library_id",
        ));
        self.relation_filters.push(RelationFilter::new("invite_status_list", selection));
        self
    }

    pub fn without_invite_status_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::InviteStatus as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "media_library_id",
        ));
        self.relation_filters.push(RelationFilter::new("invite_status_list", selection));
        self
    }

    pub fn select_invite_status_list(mut self) -> Self {
        self.query.relations.push(teaql_core::RelationLoad::new("invite_status_list")
            .with_operation(QueryOperation::new(QueryOperationKind::RelationSelect, "select_invite_status_list").with_relation("invite_status_list")));
        self
    }

    pub fn select_invite_status_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query.relations.push(teaql_core::RelationLoad::with_query("invite_status_list", selection.into_query())
            .with_operation(QueryOperation::new(QueryOperationKind::RelationSelect, "select_invite_status_list_with").with_relation("invite_status_list")));
        self
}

    pub fn have_invite_codes(self) -> Self {
        self.with_invite_code_list_matching(crate::Q::invite_codes_minimal())
    }

    pub fn have_no_invite_codes(self) -> Self {
        self.without_invite_code_list_matching(crate::Q::invite_codes_minimal())
    }

    pub fn with_invite_code_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::InviteCode as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "media_library_id",
        ));
        self.relation_filters.push(RelationFilter::new("invite_code_list", selection));
        self
    }

    pub fn without_invite_code_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::InviteCode as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "media_library_id",
        ));
        self.relation_filters.push(RelationFilter::new("invite_code_list", selection));
        self
    }

    pub fn select_invite_code_list(mut self) -> Self {
        self.query.relations.push(teaql_core::RelationLoad::new("invite_code_list")
            .with_operation(QueryOperation::new(QueryOperationKind::RelationSelect, "select_invite_code_list").with_relation("invite_code_list")));
        self
    }

    pub fn select_invite_code_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query.relations.push(teaql_core::RelationLoad::with_query("invite_code_list", selection.into_query())
            .with_operation(QueryOperation::new(QueryOperationKind::RelationSelect, "select_invite_code_list_with").with_relation("invite_code_list")));
        self
}

    pub fn have_app_settings(self) -> Self {
        self.with_app_setting_list_matching(crate::Q::app_settings_minimal())
    }

    pub fn have_no_app_settings(self) -> Self {
        self.without_app_setting_list_matching(crate::Q::app_settings_minimal())
    }

    pub fn with_app_setting_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::AppSetting as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "media_library_id",
        ));
        self.relation_filters.push(RelationFilter::new("app_setting_list", selection));
        self
    }

    pub fn without_app_setting_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::AppSetting as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "media_library_id",
        ));
        self.relation_filters.push(RelationFilter::new("app_setting_list", selection));
        self
    }

    pub fn select_app_setting_list(mut self) -> Self {
        self.query.relations.push(teaql_core::RelationLoad::new("app_setting_list")
            .with_operation(QueryOperation::new(QueryOperationKind::RelationSelect, "select_app_setting_list").with_relation("app_setting_list")));
        self
    }

    pub fn select_app_setting_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query.relations.push(teaql_core::RelationLoad::with_query("app_setting_list", selection.into_query())
            .with_operation(QueryOperation::new(QueryOperationKind::RelationSelect, "select_app_setting_list_with").with_relation("app_setting_list")));
        self
}

    pub fn have_proxy_trust_rules(self) -> Self {
        self.with_proxy_trust_rule_list_matching(crate::Q::proxy_trust_rules_minimal())
    }

    pub fn have_no_proxy_trust_rules(self) -> Self {
        self.without_proxy_trust_rule_list_matching(crate::Q::proxy_trust_rules_minimal())
    }

    pub fn with_proxy_trust_rule_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::ProxyTrustRule as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "media_library_id",
        ));
        self.relation_filters.push(RelationFilter::new("proxy_trust_rule_list", selection));
        self
    }

    pub fn without_proxy_trust_rule_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::ProxyTrustRule as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "media_library_id",
        ));
        self.relation_filters.push(RelationFilter::new("proxy_trust_rule_list", selection));
        self
    }

    pub fn select_proxy_trust_rule_list(mut self) -> Self {
        self.query.relations.push(teaql_core::RelationLoad::new("proxy_trust_rule_list")
            .with_operation(QueryOperation::new(QueryOperationKind::RelationSelect, "select_proxy_trust_rule_list").with_relation("proxy_trust_rule_list")));
        self
    }

    pub fn select_proxy_trust_rule_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query.relations.push(teaql_core::RelationLoad::with_query("proxy_trust_rule_list", selection.into_query())
            .with_operation(QueryOperation::new(QueryOperationKind::RelationSelect, "select_proxy_trust_rule_list_with").with_relation("proxy_trust_rule_list")));
        self
}

    pub fn have_share_handles(self) -> Self {
        self.with_share_handle_list_matching(crate::Q::share_handles_minimal())
    }

    pub fn have_no_share_handles(self) -> Self {
        self.without_share_handle_list_matching(crate::Q::share_handles_minimal())
    }

    pub fn with_share_handle_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::ShareHandle as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "media_library_id",
        ));
        self.relation_filters.push(RelationFilter::new("share_handle_list", selection));
        self
    }

    pub fn without_share_handle_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::ShareHandle as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "media_library_id",
        ));
        self.relation_filters.push(RelationFilter::new("share_handle_list", selection));
        self
    }

    pub fn select_share_handle_list(mut self) -> Self {
        self.query.relations.push(teaql_core::RelationLoad::new("share_handle_list")
            .with_operation(QueryOperation::new(QueryOperationKind::RelationSelect, "select_share_handle_list").with_relation("share_handle_list")));
        self
    }

    pub fn select_share_handle_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query.relations.push(teaql_core::RelationLoad::with_query("share_handle_list", selection.into_query())
            .with_operation(QueryOperation::new(QueryOperationKind::RelationSelect, "select_share_handle_list_with").with_relation("share_handle_list")));
        self
}
    pub fn count_media_assets(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::RelationCount, "count_media_assets");
        self.count_media_assets_internal("count_media_assets".to_owned(), crate::Q::media_assets().unlimited(), operation)
    }

    pub fn count_media_assets_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::RelationCount, "count_media_assets_as").with_alias(&alias);
        self.count_media_assets_internal(alias, crate::Q::media_assets().unlimited(), operation)
    }

    pub fn count_media_assets_with(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::RelationCount, "count_media_assets_with").with_alias(&alias);
        self.count_media_assets_internal(alias, request, operation)
    }

    fn count_media_assets_internal(mut self, alias: String, request: impl Into<QuerySelection>, operation: QueryOperation) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "media_asset_list",
            alias,
            selection,
            true,
        ).with_operation(operation.with_relation("media_asset_list")));
        self
    }

    pub fn stats_from_media_assets(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_media_assets_as("refinements", request)
    }

    pub fn stats_from_media_assets_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "stats_from_media_assets_as")
            .with_alias(&alias).with_relation("media_asset_list");
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "media_asset_list",
            alias,
            selection,
            false,
        ).with_operation(operation));
        self
    }

    fn scalar_from_media_assets_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>, operation: QueryOperation) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "media_asset_list",
            alias,
            selection,
            true,
        ).with_operation(operation.with_relation("media_asset_list")));
        self
    }

    pub fn group_by_media_assets_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_media_assets(request)
    }


    pub fn sum_file_size_of_media_assets(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "sum_file_size_of_media_assets");
        self.scalar_from_media_assets_as("sum_file_size_of_media_assets", crate::Q::media_assets().unlimited().into_query().sum("file_size", "sum_file_size_of_media_assets"), operation)
    }

    pub fn sum_file_size_of_media_assets_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "sum_file_size_of_media_assets_as").with_alias(&alias);
        let selection = request.into().into_query().sum("file_size", alias.clone());
        self.scalar_from_media_assets_as(alias, selection, operation)
    }
    pub fn min_file_size_of_media_assets(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "min_file_size_of_media_assets");
        self.scalar_from_media_assets_as("min_file_size_of_media_assets", crate::Q::media_assets().unlimited().into_query().min("file_size", "min_file_size_of_media_assets"), operation)
    }

    pub fn min_file_size_of_media_assets_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "min_file_size_of_media_assets_as").with_alias(&alias);
        let selection = request.into().into_query().min("file_size", alias.clone());
        self.scalar_from_media_assets_as(alias, selection, operation)
    }
    pub fn max_file_size_of_media_assets(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "max_file_size_of_media_assets");
        self.scalar_from_media_assets_as("max_file_size_of_media_assets", crate::Q::media_assets().unlimited().into_query().max("file_size", "max_file_size_of_media_assets"), operation)
    }

    pub fn max_file_size_of_media_assets_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "max_file_size_of_media_assets_as").with_alias(&alias);
        let selection = request.into().into_query().max("file_size", alias.clone());
        self.scalar_from_media_assets_as(alias, selection, operation)
    }
    pub fn avg_file_size_of_media_assets(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "avg_file_size_of_media_assets");
        self.scalar_from_media_assets_as("avg_file_size_of_media_assets", crate::Q::media_assets().unlimited().into_query().avg("file_size", "avg_file_size_of_media_assets"), operation)
    }

    pub fn avg_file_size_of_media_assets_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "avg_file_size_of_media_assets_as").with_alias(&alias);
        let selection = request.into().into_query().avg("file_size", alias.clone());
        self.scalar_from_media_assets_as(alias, selection, operation)
    }
    pub fn standard_deviation_file_size_of_media_assets(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "standard_deviation_file_size_of_media_assets");
        self.scalar_from_media_assets_as("standard_deviation_file_size_of_media_assets", crate::Q::media_assets().unlimited().into_query().stddev("file_size", "standard_deviation_file_size_of_media_assets"), operation)
    }

    pub fn standard_deviation_file_size_of_media_assets_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "standard_deviation_file_size_of_media_assets_as").with_alias(&alias);
        let selection = request.into().into_query().stddev("file_size", alias.clone());
        self.scalar_from_media_assets_as(alias, selection, operation)
    }
    pub fn square_root_of_population_standard_deviation_file_size_of_media_assets(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "square_root_of_population_standard_deviation_file_size_of_media_assets");
        self.scalar_from_media_assets_as("square_root_of_population_standard_deviation_file_size_of_media_assets", crate::Q::media_assets().unlimited().into_query().stddev_pop("file_size", "square_root_of_population_standard_deviation_file_size_of_media_assets"), operation)
    }

    pub fn square_root_of_population_standard_deviation_file_size_of_media_assets_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "square_root_of_population_standard_deviation_file_size_of_media_assets_as").with_alias(&alias);
        let selection = request.into().into_query().stddev_pop("file_size", alias.clone());
        self.scalar_from_media_assets_as(alias, selection, operation)
    }
    pub fn sample_variance_file_size_of_media_assets(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "sample_variance_file_size_of_media_assets");
        self.scalar_from_media_assets_as("sample_variance_file_size_of_media_assets", crate::Q::media_assets().unlimited().into_query().var_samp("file_size", "sample_variance_file_size_of_media_assets"), operation)
    }

    pub fn sample_variance_file_size_of_media_assets_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "sample_variance_file_size_of_media_assets_as").with_alias(&alias);
        let selection = request.into().into_query().var_samp("file_size", alias.clone());
        self.scalar_from_media_assets_as(alias, selection, operation)
    }
    pub fn sample_population_variance_file_size_of_media_assets(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "sample_population_variance_file_size_of_media_assets");
        self.scalar_from_media_assets_as("sample_population_variance_file_size_of_media_assets", crate::Q::media_assets().unlimited().into_query().var_pop("file_size", "sample_population_variance_file_size_of_media_assets"), operation)
    }

    pub fn sample_population_variance_file_size_of_media_assets_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "sample_population_variance_file_size_of_media_assets_as").with_alias(&alias);
        let selection = request.into().into_query().var_pop("file_size", alias.clone());
        self.scalar_from_media_assets_as(alias, selection, operation)
    }
    pub fn sum_duration_seconds_of_media_assets(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "sum_duration_seconds_of_media_assets");
        self.scalar_from_media_assets_as("sum_duration_seconds_of_media_assets", crate::Q::media_assets().unlimited().into_query().sum("duration_seconds", "sum_duration_seconds_of_media_assets"), operation)
    }

    pub fn sum_duration_seconds_of_media_assets_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "sum_duration_seconds_of_media_assets_as").with_alias(&alias);
        let selection = request.into().into_query().sum("duration_seconds", alias.clone());
        self.scalar_from_media_assets_as(alias, selection, operation)
    }
    pub fn min_duration_seconds_of_media_assets(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "min_duration_seconds_of_media_assets");
        self.scalar_from_media_assets_as("min_duration_seconds_of_media_assets", crate::Q::media_assets().unlimited().into_query().min("duration_seconds", "min_duration_seconds_of_media_assets"), operation)
    }

    pub fn min_duration_seconds_of_media_assets_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "min_duration_seconds_of_media_assets_as").with_alias(&alias);
        let selection = request.into().into_query().min("duration_seconds", alias.clone());
        self.scalar_from_media_assets_as(alias, selection, operation)
    }
    pub fn max_duration_seconds_of_media_assets(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "max_duration_seconds_of_media_assets");
        self.scalar_from_media_assets_as("max_duration_seconds_of_media_assets", crate::Q::media_assets().unlimited().into_query().max("duration_seconds", "max_duration_seconds_of_media_assets"), operation)
    }

    pub fn max_duration_seconds_of_media_assets_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "max_duration_seconds_of_media_assets_as").with_alias(&alias);
        let selection = request.into().into_query().max("duration_seconds", alias.clone());
        self.scalar_from_media_assets_as(alias, selection, operation)
    }
    pub fn avg_duration_seconds_of_media_assets(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "avg_duration_seconds_of_media_assets");
        self.scalar_from_media_assets_as("avg_duration_seconds_of_media_assets", crate::Q::media_assets().unlimited().into_query().avg("duration_seconds", "avg_duration_seconds_of_media_assets"), operation)
    }

    pub fn avg_duration_seconds_of_media_assets_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "avg_duration_seconds_of_media_assets_as").with_alias(&alias);
        let selection = request.into().into_query().avg("duration_seconds", alias.clone());
        self.scalar_from_media_assets_as(alias, selection, operation)
    }
    pub fn standard_deviation_duration_seconds_of_media_assets(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "standard_deviation_duration_seconds_of_media_assets");
        self.scalar_from_media_assets_as("standard_deviation_duration_seconds_of_media_assets", crate::Q::media_assets().unlimited().into_query().stddev("duration_seconds", "standard_deviation_duration_seconds_of_media_assets"), operation)
    }

    pub fn standard_deviation_duration_seconds_of_media_assets_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "standard_deviation_duration_seconds_of_media_assets_as").with_alias(&alias);
        let selection = request.into().into_query().stddev("duration_seconds", alias.clone());
        self.scalar_from_media_assets_as(alias, selection, operation)
    }
    pub fn square_root_of_population_standard_deviation_duration_seconds_of_media_assets(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "square_root_of_population_standard_deviation_duration_seconds_of_media_assets");
        self.scalar_from_media_assets_as("square_root_of_population_standard_deviation_duration_seconds_of_media_assets", crate::Q::media_assets().unlimited().into_query().stddev_pop("duration_seconds", "square_root_of_population_standard_deviation_duration_seconds_of_media_assets"), operation)
    }

    pub fn square_root_of_population_standard_deviation_duration_seconds_of_media_assets_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "square_root_of_population_standard_deviation_duration_seconds_of_media_assets_as").with_alias(&alias);
        let selection = request.into().into_query().stddev_pop("duration_seconds", alias.clone());
        self.scalar_from_media_assets_as(alias, selection, operation)
    }
    pub fn sample_variance_duration_seconds_of_media_assets(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "sample_variance_duration_seconds_of_media_assets");
        self.scalar_from_media_assets_as("sample_variance_duration_seconds_of_media_assets", crate::Q::media_assets().unlimited().into_query().var_samp("duration_seconds", "sample_variance_duration_seconds_of_media_assets"), operation)
    }

    pub fn sample_variance_duration_seconds_of_media_assets_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "sample_variance_duration_seconds_of_media_assets_as").with_alias(&alias);
        let selection = request.into().into_query().var_samp("duration_seconds", alias.clone());
        self.scalar_from_media_assets_as(alias, selection, operation)
    }
    pub fn sample_population_variance_duration_seconds_of_media_assets(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "sample_population_variance_duration_seconds_of_media_assets");
        self.scalar_from_media_assets_as("sample_population_variance_duration_seconds_of_media_assets", crate::Q::media_assets().unlimited().into_query().var_pop("duration_seconds", "sample_population_variance_duration_seconds_of_media_assets"), operation)
    }

    pub fn sample_population_variance_duration_seconds_of_media_assets_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "sample_population_variance_duration_seconds_of_media_assets_as").with_alias(&alias);
        let selection = request.into().into_query().var_pop("duration_seconds", alias.clone());
        self.scalar_from_media_assets_as(alias, selection, operation)
    }
    pub fn min_create_time_of_media_assets(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "min_create_time_of_media_assets");
        self.scalar_from_media_assets_as("min_create_time_of_media_assets", crate::Q::media_assets().unlimited().into_query().min("create_time", "min_create_time_of_media_assets"), operation)
    }

    pub fn min_create_time_of_media_assets_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "min_create_time_of_media_assets_as").with_alias(&alias);
        let selection = request.into().into_query().min("create_time", alias.clone());
        self.scalar_from_media_assets_as(alias, selection, operation)
    }
    pub fn max_create_time_of_media_assets(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "max_create_time_of_media_assets");
        self.scalar_from_media_assets_as("max_create_time_of_media_assets", crate::Q::media_assets().unlimited().into_query().max("create_time", "max_create_time_of_media_assets"), operation)
    }

    pub fn max_create_time_of_media_assets_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "max_create_time_of_media_assets_as").with_alias(&alias);
        let selection = request.into().into_query().max("create_time", alias.clone());
        self.scalar_from_media_assets_as(alias, selection, operation)
    }
    pub fn min_update_time_of_media_assets(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "min_update_time_of_media_assets");
        self.scalar_from_media_assets_as("min_update_time_of_media_assets", crate::Q::media_assets().unlimited().into_query().min("update_time", "min_update_time_of_media_assets"), operation)
    }

    pub fn min_update_time_of_media_assets_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "min_update_time_of_media_assets_as").with_alias(&alias);
        let selection = request.into().into_query().min("update_time", alias.clone());
        self.scalar_from_media_assets_as(alias, selection, operation)
    }
    pub fn max_update_time_of_media_assets(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "max_update_time_of_media_assets");
        self.scalar_from_media_assets_as("max_update_time_of_media_assets", crate::Q::media_assets().unlimited().into_query().max("update_time", "max_update_time_of_media_assets"), operation)
    }

    pub fn max_update_time_of_media_assets_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "max_update_time_of_media_assets_as").with_alias(&alias);
        let selection = request.into().into_query().max("update_time", alias.clone());
        self.scalar_from_media_assets_as(alias, selection, operation)
    }

    pub fn count_grant_statuses(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::RelationCount, "count_grant_statuses");
        self.count_grant_statuses_internal("count_grant_statuses".to_owned(), crate::Q::grant_statuses().unlimited(), operation)
    }

    pub fn count_grant_statuses_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::RelationCount, "count_grant_statuses_as").with_alias(&alias);
        self.count_grant_statuses_internal(alias, crate::Q::grant_statuses().unlimited(), operation)
    }

    pub fn count_grant_statuses_with(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::RelationCount, "count_grant_statuses_with").with_alias(&alias);
        self.count_grant_statuses_internal(alias, request, operation)
    }

    fn count_grant_statuses_internal(mut self, alias: String, request: impl Into<QuerySelection>, operation: QueryOperation) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "grant_status_list",
            alias,
            selection,
            true,
        ).with_operation(operation.with_relation("grant_status_list")));
        self
    }

    pub fn stats_from_grant_statuses(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_grant_statuses_as("refinements", request)
    }

    pub fn stats_from_grant_statuses_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "stats_from_grant_statuses_as")
            .with_alias(&alias).with_relation("grant_status_list");
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "grant_status_list",
            alias,
            selection,
            false,
        ).with_operation(operation));
        self
    }

    fn scalar_from_grant_statuses_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>, operation: QueryOperation) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "grant_status_list",
            alias,
            selection,
            true,
        ).with_operation(operation.with_relation("grant_status_list")));
        self
    }

    pub fn group_by_grant_statuses_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_grant_statuses(request)
    }


    pub fn sum_display_order_of_grant_statuses(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "sum_display_order_of_grant_statuses");
        self.scalar_from_grant_statuses_as("sum_display_order_of_grant_statuses", crate::Q::grant_statuses().unlimited().into_query().sum("display_order", "sum_display_order_of_grant_statuses"), operation)
    }

    pub fn sum_display_order_of_grant_statuses_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "sum_display_order_of_grant_statuses_as").with_alias(&alias);
        let selection = request.into().into_query().sum("display_order", alias.clone());
        self.scalar_from_grant_statuses_as(alias, selection, operation)
    }
    pub fn min_display_order_of_grant_statuses(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "min_display_order_of_grant_statuses");
        self.scalar_from_grant_statuses_as("min_display_order_of_grant_statuses", crate::Q::grant_statuses().unlimited().into_query().min("display_order", "min_display_order_of_grant_statuses"), operation)
    }

    pub fn min_display_order_of_grant_statuses_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "min_display_order_of_grant_statuses_as").with_alias(&alias);
        let selection = request.into().into_query().min("display_order", alias.clone());
        self.scalar_from_grant_statuses_as(alias, selection, operation)
    }
    pub fn max_display_order_of_grant_statuses(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "max_display_order_of_grant_statuses");
        self.scalar_from_grant_statuses_as("max_display_order_of_grant_statuses", crate::Q::grant_statuses().unlimited().into_query().max("display_order", "max_display_order_of_grant_statuses"), operation)
    }

    pub fn max_display_order_of_grant_statuses_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "max_display_order_of_grant_statuses_as").with_alias(&alias);
        let selection = request.into().into_query().max("display_order", alias.clone());
        self.scalar_from_grant_statuses_as(alias, selection, operation)
    }
    pub fn avg_display_order_of_grant_statuses(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "avg_display_order_of_grant_statuses");
        self.scalar_from_grant_statuses_as("avg_display_order_of_grant_statuses", crate::Q::grant_statuses().unlimited().into_query().avg("display_order", "avg_display_order_of_grant_statuses"), operation)
    }

    pub fn avg_display_order_of_grant_statuses_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "avg_display_order_of_grant_statuses_as").with_alias(&alias);
        let selection = request.into().into_query().avg("display_order", alias.clone());
        self.scalar_from_grant_statuses_as(alias, selection, operation)
    }
    pub fn standard_deviation_display_order_of_grant_statuses(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "standard_deviation_display_order_of_grant_statuses");
        self.scalar_from_grant_statuses_as("standard_deviation_display_order_of_grant_statuses", crate::Q::grant_statuses().unlimited().into_query().stddev("display_order", "standard_deviation_display_order_of_grant_statuses"), operation)
    }

    pub fn standard_deviation_display_order_of_grant_statuses_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "standard_deviation_display_order_of_grant_statuses_as").with_alias(&alias);
        let selection = request.into().into_query().stddev("display_order", alias.clone());
        self.scalar_from_grant_statuses_as(alias, selection, operation)
    }
    pub fn square_root_of_population_standard_deviation_display_order_of_grant_statuses(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "square_root_of_population_standard_deviation_display_order_of_grant_statuses");
        self.scalar_from_grant_statuses_as("square_root_of_population_standard_deviation_display_order_of_grant_statuses", crate::Q::grant_statuses().unlimited().into_query().stddev_pop("display_order", "square_root_of_population_standard_deviation_display_order_of_grant_statuses"), operation)
    }

    pub fn square_root_of_population_standard_deviation_display_order_of_grant_statuses_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "square_root_of_population_standard_deviation_display_order_of_grant_statuses_as").with_alias(&alias);
        let selection = request.into().into_query().stddev_pop("display_order", alias.clone());
        self.scalar_from_grant_statuses_as(alias, selection, operation)
    }
    pub fn sample_variance_display_order_of_grant_statuses(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "sample_variance_display_order_of_grant_statuses");
        self.scalar_from_grant_statuses_as("sample_variance_display_order_of_grant_statuses", crate::Q::grant_statuses().unlimited().into_query().var_samp("display_order", "sample_variance_display_order_of_grant_statuses"), operation)
    }

    pub fn sample_variance_display_order_of_grant_statuses_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "sample_variance_display_order_of_grant_statuses_as").with_alias(&alias);
        let selection = request.into().into_query().var_samp("display_order", alias.clone());
        self.scalar_from_grant_statuses_as(alias, selection, operation)
    }
    pub fn sample_population_variance_display_order_of_grant_statuses(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "sample_population_variance_display_order_of_grant_statuses");
        self.scalar_from_grant_statuses_as("sample_population_variance_display_order_of_grant_statuses", crate::Q::grant_statuses().unlimited().into_query().var_pop("display_order", "sample_population_variance_display_order_of_grant_statuses"), operation)
    }

    pub fn sample_population_variance_display_order_of_grant_statuses_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "sample_population_variance_display_order_of_grant_statuses_as").with_alias(&alias);
        let selection = request.into().into_query().var_pop("display_order", alias.clone());
        self.scalar_from_grant_statuses_as(alias, selection, operation)
    }

    pub fn count_ip_access_grants(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::RelationCount, "count_ip_access_grants");
        self.count_ip_access_grants_internal("count_ip_access_grants".to_owned(), crate::Q::ip_access_grants().unlimited(), operation)
    }

    pub fn count_ip_access_grants_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::RelationCount, "count_ip_access_grants_as").with_alias(&alias);
        self.count_ip_access_grants_internal(alias, crate::Q::ip_access_grants().unlimited(), operation)
    }

    pub fn count_ip_access_grants_with(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::RelationCount, "count_ip_access_grants_with").with_alias(&alias);
        self.count_ip_access_grants_internal(alias, request, operation)
    }

    fn count_ip_access_grants_internal(mut self, alias: String, request: impl Into<QuerySelection>, operation: QueryOperation) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "ip_access_grant_list",
            alias,
            selection,
            true,
        ).with_operation(operation.with_relation("ip_access_grant_list")));
        self
    }

    pub fn stats_from_ip_access_grants(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_ip_access_grants_as("refinements", request)
    }

    pub fn stats_from_ip_access_grants_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "stats_from_ip_access_grants_as")
            .with_alias(&alias).with_relation("ip_access_grant_list");
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "ip_access_grant_list",
            alias,
            selection,
            false,
        ).with_operation(operation));
        self
    }

    fn scalar_from_ip_access_grants_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>, operation: QueryOperation) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "ip_access_grant_list",
            alias,
            selection,
            true,
        ).with_operation(operation.with_relation("ip_access_grant_list")));
        self
    }

    pub fn group_by_ip_access_grants_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_ip_access_grants(request)
    }


    pub fn min_create_time_of_ip_access_grants(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "min_create_time_of_ip_access_grants");
        self.scalar_from_ip_access_grants_as("min_create_time_of_ip_access_grants", crate::Q::ip_access_grants().unlimited().into_query().min("create_time", "min_create_time_of_ip_access_grants"), operation)
    }

    pub fn min_create_time_of_ip_access_grants_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "min_create_time_of_ip_access_grants_as").with_alias(&alias);
        let selection = request.into().into_query().min("create_time", alias.clone());
        self.scalar_from_ip_access_grants_as(alias, selection, operation)
    }
    pub fn max_create_time_of_ip_access_grants(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "max_create_time_of_ip_access_grants");
        self.scalar_from_ip_access_grants_as("max_create_time_of_ip_access_grants", crate::Q::ip_access_grants().unlimited().into_query().max("create_time", "max_create_time_of_ip_access_grants"), operation)
    }

    pub fn max_create_time_of_ip_access_grants_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "max_create_time_of_ip_access_grants_as").with_alias(&alias);
        let selection = request.into().into_query().max("create_time", alias.clone());
        self.scalar_from_ip_access_grants_as(alias, selection, operation)
    }
    pub fn min_update_time_of_ip_access_grants(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "min_update_time_of_ip_access_grants");
        self.scalar_from_ip_access_grants_as("min_update_time_of_ip_access_grants", crate::Q::ip_access_grants().unlimited().into_query().min("update_time", "min_update_time_of_ip_access_grants"), operation)
    }

    pub fn min_update_time_of_ip_access_grants_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "min_update_time_of_ip_access_grants_as").with_alias(&alias);
        let selection = request.into().into_query().min("update_time", alias.clone());
        self.scalar_from_ip_access_grants_as(alias, selection, operation)
    }
    pub fn max_update_time_of_ip_access_grants(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "max_update_time_of_ip_access_grants");
        self.scalar_from_ip_access_grants_as("max_update_time_of_ip_access_grants", crate::Q::ip_access_grants().unlimited().into_query().max("update_time", "max_update_time_of_ip_access_grants"), operation)
    }

    pub fn max_update_time_of_ip_access_grants_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "max_update_time_of_ip_access_grants_as").with_alias(&alias);
        let selection = request.into().into_query().max("update_time", alias.clone());
        self.scalar_from_ip_access_grants_as(alias, selection, operation)
    }

    pub fn count_invite_statuses(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::RelationCount, "count_invite_statuses");
        self.count_invite_statuses_internal("count_invite_statuses".to_owned(), crate::Q::invite_statuses().unlimited(), operation)
    }

    pub fn count_invite_statuses_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::RelationCount, "count_invite_statuses_as").with_alias(&alias);
        self.count_invite_statuses_internal(alias, crate::Q::invite_statuses().unlimited(), operation)
    }

    pub fn count_invite_statuses_with(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::RelationCount, "count_invite_statuses_with").with_alias(&alias);
        self.count_invite_statuses_internal(alias, request, operation)
    }

    fn count_invite_statuses_internal(mut self, alias: String, request: impl Into<QuerySelection>, operation: QueryOperation) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "invite_status_list",
            alias,
            selection,
            true,
        ).with_operation(operation.with_relation("invite_status_list")));
        self
    }

    pub fn stats_from_invite_statuses(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_invite_statuses_as("refinements", request)
    }

    pub fn stats_from_invite_statuses_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "stats_from_invite_statuses_as")
            .with_alias(&alias).with_relation("invite_status_list");
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "invite_status_list",
            alias,
            selection,
            false,
        ).with_operation(operation));
        self
    }

    fn scalar_from_invite_statuses_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>, operation: QueryOperation) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "invite_status_list",
            alias,
            selection,
            true,
        ).with_operation(operation.with_relation("invite_status_list")));
        self
    }

    pub fn group_by_invite_statuses_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_invite_statuses(request)
    }


    pub fn sum_display_order_of_invite_statuses(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "sum_display_order_of_invite_statuses");
        self.scalar_from_invite_statuses_as("sum_display_order_of_invite_statuses", crate::Q::invite_statuses().unlimited().into_query().sum("display_order", "sum_display_order_of_invite_statuses"), operation)
    }

    pub fn sum_display_order_of_invite_statuses_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "sum_display_order_of_invite_statuses_as").with_alias(&alias);
        let selection = request.into().into_query().sum("display_order", alias.clone());
        self.scalar_from_invite_statuses_as(alias, selection, operation)
    }
    pub fn min_display_order_of_invite_statuses(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "min_display_order_of_invite_statuses");
        self.scalar_from_invite_statuses_as("min_display_order_of_invite_statuses", crate::Q::invite_statuses().unlimited().into_query().min("display_order", "min_display_order_of_invite_statuses"), operation)
    }

    pub fn min_display_order_of_invite_statuses_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "min_display_order_of_invite_statuses_as").with_alias(&alias);
        let selection = request.into().into_query().min("display_order", alias.clone());
        self.scalar_from_invite_statuses_as(alias, selection, operation)
    }
    pub fn max_display_order_of_invite_statuses(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "max_display_order_of_invite_statuses");
        self.scalar_from_invite_statuses_as("max_display_order_of_invite_statuses", crate::Q::invite_statuses().unlimited().into_query().max("display_order", "max_display_order_of_invite_statuses"), operation)
    }

    pub fn max_display_order_of_invite_statuses_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "max_display_order_of_invite_statuses_as").with_alias(&alias);
        let selection = request.into().into_query().max("display_order", alias.clone());
        self.scalar_from_invite_statuses_as(alias, selection, operation)
    }
    pub fn avg_display_order_of_invite_statuses(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "avg_display_order_of_invite_statuses");
        self.scalar_from_invite_statuses_as("avg_display_order_of_invite_statuses", crate::Q::invite_statuses().unlimited().into_query().avg("display_order", "avg_display_order_of_invite_statuses"), operation)
    }

    pub fn avg_display_order_of_invite_statuses_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "avg_display_order_of_invite_statuses_as").with_alias(&alias);
        let selection = request.into().into_query().avg("display_order", alias.clone());
        self.scalar_from_invite_statuses_as(alias, selection, operation)
    }
    pub fn standard_deviation_display_order_of_invite_statuses(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "standard_deviation_display_order_of_invite_statuses");
        self.scalar_from_invite_statuses_as("standard_deviation_display_order_of_invite_statuses", crate::Q::invite_statuses().unlimited().into_query().stddev("display_order", "standard_deviation_display_order_of_invite_statuses"), operation)
    }

    pub fn standard_deviation_display_order_of_invite_statuses_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "standard_deviation_display_order_of_invite_statuses_as").with_alias(&alias);
        let selection = request.into().into_query().stddev("display_order", alias.clone());
        self.scalar_from_invite_statuses_as(alias, selection, operation)
    }
    pub fn square_root_of_population_standard_deviation_display_order_of_invite_statuses(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "square_root_of_population_standard_deviation_display_order_of_invite_statuses");
        self.scalar_from_invite_statuses_as("square_root_of_population_standard_deviation_display_order_of_invite_statuses", crate::Q::invite_statuses().unlimited().into_query().stddev_pop("display_order", "square_root_of_population_standard_deviation_display_order_of_invite_statuses"), operation)
    }

    pub fn square_root_of_population_standard_deviation_display_order_of_invite_statuses_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "square_root_of_population_standard_deviation_display_order_of_invite_statuses_as").with_alias(&alias);
        let selection = request.into().into_query().stddev_pop("display_order", alias.clone());
        self.scalar_from_invite_statuses_as(alias, selection, operation)
    }
    pub fn sample_variance_display_order_of_invite_statuses(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "sample_variance_display_order_of_invite_statuses");
        self.scalar_from_invite_statuses_as("sample_variance_display_order_of_invite_statuses", crate::Q::invite_statuses().unlimited().into_query().var_samp("display_order", "sample_variance_display_order_of_invite_statuses"), operation)
    }

    pub fn sample_variance_display_order_of_invite_statuses_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "sample_variance_display_order_of_invite_statuses_as").with_alias(&alias);
        let selection = request.into().into_query().var_samp("display_order", alias.clone());
        self.scalar_from_invite_statuses_as(alias, selection, operation)
    }
    pub fn sample_population_variance_display_order_of_invite_statuses(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "sample_population_variance_display_order_of_invite_statuses");
        self.scalar_from_invite_statuses_as("sample_population_variance_display_order_of_invite_statuses", crate::Q::invite_statuses().unlimited().into_query().var_pop("display_order", "sample_population_variance_display_order_of_invite_statuses"), operation)
    }

    pub fn sample_population_variance_display_order_of_invite_statuses_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "sample_population_variance_display_order_of_invite_statuses_as").with_alias(&alias);
        let selection = request.into().into_query().var_pop("display_order", alias.clone());
        self.scalar_from_invite_statuses_as(alias, selection, operation)
    }

    pub fn count_invite_codes(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::RelationCount, "count_invite_codes");
        self.count_invite_codes_internal("count_invite_codes".to_owned(), crate::Q::invite_codes().unlimited(), operation)
    }

    pub fn count_invite_codes_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::RelationCount, "count_invite_codes_as").with_alias(&alias);
        self.count_invite_codes_internal(alias, crate::Q::invite_codes().unlimited(), operation)
    }

    pub fn count_invite_codes_with(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::RelationCount, "count_invite_codes_with").with_alias(&alias);
        self.count_invite_codes_internal(alias, request, operation)
    }

    fn count_invite_codes_internal(mut self, alias: String, request: impl Into<QuerySelection>, operation: QueryOperation) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "invite_code_list",
            alias,
            selection,
            true,
        ).with_operation(operation.with_relation("invite_code_list")));
        self
    }

    pub fn stats_from_invite_codes(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_invite_codes_as("refinements", request)
    }

    pub fn stats_from_invite_codes_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "stats_from_invite_codes_as")
            .with_alias(&alias).with_relation("invite_code_list");
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "invite_code_list",
            alias,
            selection,
            false,
        ).with_operation(operation));
        self
    }

    fn scalar_from_invite_codes_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>, operation: QueryOperation) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "invite_code_list",
            alias,
            selection,
            true,
        ).with_operation(operation.with_relation("invite_code_list")));
        self
    }

    pub fn group_by_invite_codes_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_invite_codes(request)
    }


    pub fn min_create_time_of_invite_codes(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "min_create_time_of_invite_codes");
        self.scalar_from_invite_codes_as("min_create_time_of_invite_codes", crate::Q::invite_codes().unlimited().into_query().min("create_time", "min_create_time_of_invite_codes"), operation)
    }

    pub fn min_create_time_of_invite_codes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "min_create_time_of_invite_codes_as").with_alias(&alias);
        let selection = request.into().into_query().min("create_time", alias.clone());
        self.scalar_from_invite_codes_as(alias, selection, operation)
    }
    pub fn max_create_time_of_invite_codes(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "max_create_time_of_invite_codes");
        self.scalar_from_invite_codes_as("max_create_time_of_invite_codes", crate::Q::invite_codes().unlimited().into_query().max("create_time", "max_create_time_of_invite_codes"), operation)
    }

    pub fn max_create_time_of_invite_codes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "max_create_time_of_invite_codes_as").with_alias(&alias);
        let selection = request.into().into_query().max("create_time", alias.clone());
        self.scalar_from_invite_codes_as(alias, selection, operation)
    }
    pub fn min_update_time_of_invite_codes(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "min_update_time_of_invite_codes");
        self.scalar_from_invite_codes_as("min_update_time_of_invite_codes", crate::Q::invite_codes().unlimited().into_query().min("update_time", "min_update_time_of_invite_codes"), operation)
    }

    pub fn min_update_time_of_invite_codes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "min_update_time_of_invite_codes_as").with_alias(&alias);
        let selection = request.into().into_query().min("update_time", alias.clone());
        self.scalar_from_invite_codes_as(alias, selection, operation)
    }
    pub fn max_update_time_of_invite_codes(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "max_update_time_of_invite_codes");
        self.scalar_from_invite_codes_as("max_update_time_of_invite_codes", crate::Q::invite_codes().unlimited().into_query().max("update_time", "max_update_time_of_invite_codes"), operation)
    }

    pub fn max_update_time_of_invite_codes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "max_update_time_of_invite_codes_as").with_alias(&alias);
        let selection = request.into().into_query().max("update_time", alias.clone());
        self.scalar_from_invite_codes_as(alias, selection, operation)
    }

    pub fn count_app_settings(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::RelationCount, "count_app_settings");
        self.count_app_settings_internal("count_app_settings".to_owned(), crate::Q::app_settings().unlimited(), operation)
    }

    pub fn count_app_settings_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::RelationCount, "count_app_settings_as").with_alias(&alias);
        self.count_app_settings_internal(alias, crate::Q::app_settings().unlimited(), operation)
    }

    pub fn count_app_settings_with(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::RelationCount, "count_app_settings_with").with_alias(&alias);
        self.count_app_settings_internal(alias, request, operation)
    }

    fn count_app_settings_internal(mut self, alias: String, request: impl Into<QuerySelection>, operation: QueryOperation) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "app_setting_list",
            alias,
            selection,
            true,
        ).with_operation(operation.with_relation("app_setting_list")));
        self
    }

    pub fn stats_from_app_settings(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_app_settings_as("refinements", request)
    }

    pub fn stats_from_app_settings_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "stats_from_app_settings_as")
            .with_alias(&alias).with_relation("app_setting_list");
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "app_setting_list",
            alias,
            selection,
            false,
        ).with_operation(operation));
        self
    }

    fn scalar_from_app_settings_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>, operation: QueryOperation) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "app_setting_list",
            alias,
            selection,
            true,
        ).with_operation(operation.with_relation("app_setting_list")));
        self
    }

    pub fn group_by_app_settings_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_app_settings(request)
    }


    pub fn min_create_time_of_app_settings(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "min_create_time_of_app_settings");
        self.scalar_from_app_settings_as("min_create_time_of_app_settings", crate::Q::app_settings().unlimited().into_query().min("create_time", "min_create_time_of_app_settings"), operation)
    }

    pub fn min_create_time_of_app_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "min_create_time_of_app_settings_as").with_alias(&alias);
        let selection = request.into().into_query().min("create_time", alias.clone());
        self.scalar_from_app_settings_as(alias, selection, operation)
    }
    pub fn max_create_time_of_app_settings(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "max_create_time_of_app_settings");
        self.scalar_from_app_settings_as("max_create_time_of_app_settings", crate::Q::app_settings().unlimited().into_query().max("create_time", "max_create_time_of_app_settings"), operation)
    }

    pub fn max_create_time_of_app_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "max_create_time_of_app_settings_as").with_alias(&alias);
        let selection = request.into().into_query().max("create_time", alias.clone());
        self.scalar_from_app_settings_as(alias, selection, operation)
    }
    pub fn min_update_time_of_app_settings(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "min_update_time_of_app_settings");
        self.scalar_from_app_settings_as("min_update_time_of_app_settings", crate::Q::app_settings().unlimited().into_query().min("update_time", "min_update_time_of_app_settings"), operation)
    }

    pub fn min_update_time_of_app_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "min_update_time_of_app_settings_as").with_alias(&alias);
        let selection = request.into().into_query().min("update_time", alias.clone());
        self.scalar_from_app_settings_as(alias, selection, operation)
    }
    pub fn max_update_time_of_app_settings(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "max_update_time_of_app_settings");
        self.scalar_from_app_settings_as("max_update_time_of_app_settings", crate::Q::app_settings().unlimited().into_query().max("update_time", "max_update_time_of_app_settings"), operation)
    }

    pub fn max_update_time_of_app_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "max_update_time_of_app_settings_as").with_alias(&alias);
        let selection = request.into().into_query().max("update_time", alias.clone());
        self.scalar_from_app_settings_as(alias, selection, operation)
    }

    pub fn count_proxy_trust_rules(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::RelationCount, "count_proxy_trust_rules");
        self.count_proxy_trust_rules_internal("count_proxy_trust_rules".to_owned(), crate::Q::proxy_trust_rules().unlimited(), operation)
    }

    pub fn count_proxy_trust_rules_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::RelationCount, "count_proxy_trust_rules_as").with_alias(&alias);
        self.count_proxy_trust_rules_internal(alias, crate::Q::proxy_trust_rules().unlimited(), operation)
    }

    pub fn count_proxy_trust_rules_with(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::RelationCount, "count_proxy_trust_rules_with").with_alias(&alias);
        self.count_proxy_trust_rules_internal(alias, request, operation)
    }

    fn count_proxy_trust_rules_internal(mut self, alias: String, request: impl Into<QuerySelection>, operation: QueryOperation) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "proxy_trust_rule_list",
            alias,
            selection,
            true,
        ).with_operation(operation.with_relation("proxy_trust_rule_list")));
        self
    }

    pub fn stats_from_proxy_trust_rules(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_proxy_trust_rules_as("refinements", request)
    }

    pub fn stats_from_proxy_trust_rules_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "stats_from_proxy_trust_rules_as")
            .with_alias(&alias).with_relation("proxy_trust_rule_list");
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "proxy_trust_rule_list",
            alias,
            selection,
            false,
        ).with_operation(operation));
        self
    }

    fn scalar_from_proxy_trust_rules_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>, operation: QueryOperation) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "proxy_trust_rule_list",
            alias,
            selection,
            true,
        ).with_operation(operation.with_relation("proxy_trust_rule_list")));
        self
    }

    pub fn group_by_proxy_trust_rules_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_proxy_trust_rules(request)
    }


    pub fn min_create_time_of_proxy_trust_rules(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "min_create_time_of_proxy_trust_rules");
        self.scalar_from_proxy_trust_rules_as("min_create_time_of_proxy_trust_rules", crate::Q::proxy_trust_rules().unlimited().into_query().min("create_time", "min_create_time_of_proxy_trust_rules"), operation)
    }

    pub fn min_create_time_of_proxy_trust_rules_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "min_create_time_of_proxy_trust_rules_as").with_alias(&alias);
        let selection = request.into().into_query().min("create_time", alias.clone());
        self.scalar_from_proxy_trust_rules_as(alias, selection, operation)
    }
    pub fn max_create_time_of_proxy_trust_rules(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "max_create_time_of_proxy_trust_rules");
        self.scalar_from_proxy_trust_rules_as("max_create_time_of_proxy_trust_rules", crate::Q::proxy_trust_rules().unlimited().into_query().max("create_time", "max_create_time_of_proxy_trust_rules"), operation)
    }

    pub fn max_create_time_of_proxy_trust_rules_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "max_create_time_of_proxy_trust_rules_as").with_alias(&alias);
        let selection = request.into().into_query().max("create_time", alias.clone());
        self.scalar_from_proxy_trust_rules_as(alias, selection, operation)
    }
    pub fn min_update_time_of_proxy_trust_rules(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "min_update_time_of_proxy_trust_rules");
        self.scalar_from_proxy_trust_rules_as("min_update_time_of_proxy_trust_rules", crate::Q::proxy_trust_rules().unlimited().into_query().min("update_time", "min_update_time_of_proxy_trust_rules"), operation)
    }

    pub fn min_update_time_of_proxy_trust_rules_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "min_update_time_of_proxy_trust_rules_as").with_alias(&alias);
        let selection = request.into().into_query().min("update_time", alias.clone());
        self.scalar_from_proxy_trust_rules_as(alias, selection, operation)
    }
    pub fn max_update_time_of_proxy_trust_rules(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "max_update_time_of_proxy_trust_rules");
        self.scalar_from_proxy_trust_rules_as("max_update_time_of_proxy_trust_rules", crate::Q::proxy_trust_rules().unlimited().into_query().max("update_time", "max_update_time_of_proxy_trust_rules"), operation)
    }

    pub fn max_update_time_of_proxy_trust_rules_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "max_update_time_of_proxy_trust_rules_as").with_alias(&alias);
        let selection = request.into().into_query().max("update_time", alias.clone());
        self.scalar_from_proxy_trust_rules_as(alias, selection, operation)
    }

    pub fn count_share_handles(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::RelationCount, "count_share_handles");
        self.count_share_handles_internal("count_share_handles".to_owned(), crate::Q::share_handles().unlimited(), operation)
    }

    pub fn count_share_handles_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::RelationCount, "count_share_handles_as").with_alias(&alias);
        self.count_share_handles_internal(alias, crate::Q::share_handles().unlimited(), operation)
    }

    pub fn count_share_handles_with(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::RelationCount, "count_share_handles_with").with_alias(&alias);
        self.count_share_handles_internal(alias, request, operation)
    }

    fn count_share_handles_internal(mut self, alias: String, request: impl Into<QuerySelection>, operation: QueryOperation) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "share_handle_list",
            alias,
            selection,
            true,
        ).with_operation(operation.with_relation("share_handle_list")));
        self
    }

    pub fn stats_from_share_handles(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_share_handles_as("refinements", request)
    }

    pub fn stats_from_share_handles_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "stats_from_share_handles_as")
            .with_alias(&alias).with_relation("share_handle_list");
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "share_handle_list",
            alias,
            selection,
            false,
        ).with_operation(operation));
        self
    }

    fn scalar_from_share_handles_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>, operation: QueryOperation) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "share_handle_list",
            alias,
            selection,
            true,
        ).with_operation(operation.with_relation("share_handle_list")));
        self
    }

    pub fn group_by_share_handles_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_share_handles(request)
    }


    pub fn min_create_time_of_share_handles(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "min_create_time_of_share_handles");
        self.scalar_from_share_handles_as("min_create_time_of_share_handles", crate::Q::share_handles().unlimited().into_query().min("create_time", "min_create_time_of_share_handles"), operation)
    }

    pub fn min_create_time_of_share_handles_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "min_create_time_of_share_handles_as").with_alias(&alias);
        let selection = request.into().into_query().min("create_time", alias.clone());
        self.scalar_from_share_handles_as(alias, selection, operation)
    }
    pub fn max_create_time_of_share_handles(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "max_create_time_of_share_handles");
        self.scalar_from_share_handles_as("max_create_time_of_share_handles", crate::Q::share_handles().unlimited().into_query().max("create_time", "max_create_time_of_share_handles"), operation)
    }

    pub fn max_create_time_of_share_handles_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "max_create_time_of_share_handles_as").with_alias(&alias);
        let selection = request.into().into_query().max("create_time", alias.clone());
        self.scalar_from_share_handles_as(alias, selection, operation)
    }
    pub fn min_update_time_of_share_handles(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "min_update_time_of_share_handles");
        self.scalar_from_share_handles_as("min_update_time_of_share_handles", crate::Q::share_handles().unlimited().into_query().min("update_time", "min_update_time_of_share_handles"), operation)
    }

    pub fn min_update_time_of_share_handles_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "min_update_time_of_share_handles_as").with_alias(&alias);
        let selection = request.into().into_query().min("update_time", alias.clone());
        self.scalar_from_share_handles_as(alias, selection, operation)
    }
    pub fn max_update_time_of_share_handles(self) -> Self {
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "max_update_time_of_share_handles");
        self.scalar_from_share_handles_as("max_update_time_of_share_handles", crate::Q::share_handles().unlimited().into_query().max("update_time", "max_update_time_of_share_handles"), operation)
    }

    pub fn max_update_time_of_share_handles_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let alias = alias.into();
        let operation = QueryOperation::new(QueryOperationKind::Aggregate, "max_update_time_of_share_handles_as").with_alias(&alias);
        let selection = request.into().into_query().max("update_time", alias.clone());
        self.scalar_from_share_handles_as(alias, selection, operation)
    }
}

impl<R> Default for MediaLibraryRequest<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R> From< MediaLibraryRequest<R> > for SelectQuery {
    fn from(request: MediaLibraryRequest<R>) -> Self {
        QuerySelection::from(request).into_query()
    }
}

impl<R> From< MediaLibraryRequest<R> > for QuerySelection {
    fn from(request: MediaLibraryRequest<R>) -> Self {
        Self {
            query: request.query,
            relation_selections: request.relation_selections,
            relation_filters: request.relation_filters,
            child_enhancements: request.child_enhancements,
            query_options: request.query_options,
        }
    }
}


impl<'a, C> crate::request_support::AuditedSave<'a, C> for teaql_core::Audited<crate::MediaLibrary> 
where C: crate::TeaqlRuntime + ?Sized + 'a
{
    type Error = teaql_runtime::RuntimeError;
    type Entity = crate::MediaLibrary;
    fn save(self, context: &'a C) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Entity, Self::Error>> + Send + '_>> {
        Box::pin(async move {
            context.save_audited_entity(self).await
        })
    }
}

impl<R: teaql_core::Entity> crate::PurposedQuery<MediaLibraryRequest<R>> {
    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.inner.query_options.comment = Some(comment.into());
        self
    }

    pub fn new_entity<C>(&self, context: &C) -> crate::MediaLibrary
    where
        C: crate::TeaqlRuntime + ?Sized,
    {
        self.require_comment();
        let mut entity = crate::MediaLibrary::runtime_new(context.user_context().entity_runtime_state());
        if let Ok(id) = context.user_context().next_id(crate::MediaLibrary::ENTITY_NAME) {
            entity.update_id(id);
        }
        teaql_core::Entity::mark_as_new(&mut entity);
        entity
    }

    fn into_inner_with_trace(mut self) -> MediaLibraryRequest<R> {
        self.require_comment();
        self.inner.query.trace_chain.push(teaql_core::TraceNode::typed(
            teaql_core::TraceKind::Purpose,
            self.inner.query.entity.clone(),
            None,
            self.purpose,
        ));
        self.inner
    }

    fn require_comment(&self) {
        assert!(
            self.inner
                .query_options
                .comment
                .as_deref()
                .is_some_and(|comment| !comment.trim().is_empty()),
            "query comment must not be empty"
        );
    }

    pub async fn execute_for_exists<'a, C>(
        self,
        context: &'a C,
    ) -> Result<bool, teaql_runtime::RuntimeError>
    where
        C: crate::TeaqlRuntime + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_exists(context).await
    }

    pub async fn execute_for_list<'a, C>(self, context: &'a C) -> Result<teaql_core::SmartList<R>, teaql_runtime::RuntimeError>
    where
        C: crate::TeaqlRuntime + ?Sized,
        R: teaql_core::Entity + Send + 'a,
    {
        self.into_inner_with_trace()._execute_for_list(context).await
    }

    pub async fn execute_for_rows<'a, C>(self, context: &'a C) -> Result<teaql_core::SmartList<teaql_core::CompactRow>, teaql_runtime::RuntimeError>
    where
        C: crate::TeaqlRuntime + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_rows(context).await
    }

    /// Execute query as a lazy entity stream without materializing the result set.
    /// Set chunk size via .stream(chunk_size) or .stream_default() on the query.
    pub async fn execute_for_stream<'a, C>(self, context: &'a C) -> Result<crate::request_support::TeaqlEntityStream<'a, R, teaql_runtime::RuntimeError>, teaql_runtime::RuntimeError>
    where
        C: crate::TeaqlRuntime + ?Sized,
        R: teaql_core::Entity + Send + 'a,
    {
        self.into_inner_with_trace()._execute_for_stream(context).await
    }

    pub async fn execute_for_first<'a, C>(self, context: &'a C) -> Result<Option<R>, teaql_runtime::RuntimeError>
    where
        C: crate::TeaqlRuntime + ?Sized,
        R: teaql_core::Entity + Send + 'a,
    {
        self.into_inner_with_trace()._execute_for_first(context).await
    }

    pub async fn execute_for_one<'a, C>(self, context: &'a C) -> Result<Option<R>, teaql_runtime::RuntimeError>
    where
        C: crate::TeaqlRuntime + ?Sized,
        R: teaql_core::Entity + Send + 'a,
    {
        self.into_inner_with_trace()._execute_for_one(context).await
    }


    pub async fn execute_for_count<'a, C>(self, context: &'a C) -> Result<u64, teaql_runtime::RuntimeError>
    where
        C: crate::TeaqlRuntime + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_count(context).await
    }
}
