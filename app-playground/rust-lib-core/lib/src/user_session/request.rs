use std::marker::PhantomData;

use serde_json::Value as JsonValue;
use teaql_core::{Aggregate, AggregateFunction, EntityDescriptor, Expr, SelectQuery, SmartList};
use teaql_runtime::RuntimeError;

use crate::request_support::*;

impl EntityReference for crate::UserSession {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(&self)
    }
}

impl EntityReference for &crate::UserSession {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(self)
    }
}

// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/user_session
#[derive(Debug)]
pub struct UserSessionRequest<R = crate::UserSession> {
    query: SelectQuery,
    relation_selections: Vec<RelationSelection>,
    relation_filters: Vec<RelationFilter>,
    child_enhancements: Vec<QuerySelection>,
    query_options: QueryOptions,
    marker: PhantomData<R>,
}

impl<R> Clone for UserSessionRequest<R> {
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

impl<R> UserSessionRequest<R> {
    pub(crate) fn new() -> Self {
        Self {
            query: SelectQuery::new("UserSession")
                .project("id")
                .project("version"),
            relation_selections: Vec::new(),
            relation_filters: Vec::new(),
            child_enhancements: Vec::new(),
            query_options: QueryOptions::default(),
            marker: PhantomData,
        }
    }

    pub fn return_type<T>(self) -> UserSessionRequest<T> {
        UserSessionRequest {
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
        context.user_context().observe_generated_list("UserSession", async {
        let mut total = None;
        if !use_id_set {
            if let Some((alias, count_query)) = &requested_count {
                let count_rows = context.fetch_compact_rows("UserSession", count_query).await?;
                total = Some(count_rows.first().and_then(|row| row.get(alias))
                    .and_then(teaql_core::Value::try_u64)
                    .ok_or_else(|| RuntimeError::Graph("requested list count is missing or not numeric".into()))?);
            }
        }
        let (mut rows, facets) = if query_options.facets.is_empty() {
            let rows = context.fetch_entity_smart_list::<R>(
                "UserSession",
                query,
                relation_aggregates,
            ).await?;
            (rows, std::collections::BTreeMap::new())
        } else {
            let rows = context.fetch_entity_smart_list::<R>(
                "UserSession",
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
                let count_rows = context.fetch_compact_rows("UserSession", &count_query).await?;
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
        context.fetch_compact_smart_list("UserSession", &query).await
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
            "UserSession", context.fetch_entity_stream("UserSession", query),
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
        let rows = context.fetch_compact_rows("UserSession", &query).await?;
        rows.first()
            .and_then(|row| row.get(&alias))
            .and_then(teaql_core::Value::try_u64)
            .ok_or_else(|| RuntimeError::Graph(format!("count result for UserSession is missing or not numeric")))
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
        let rows = context.fetch_compact_rows("UserSession", &query).await?;
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
            "session_fp" => Some("session_fp"),
            "active" => Some("active"),
            "last_seen_at" => Some("last_seen_at"),
            "expire_at" => Some("expire_at"),
            "create_time" => Some("create_time"),
            "update_time" => Some("update_time"),
            "version" => Some("version"),
            "media_library" | "media_library_id" => Some("media_library_id"),
            _ => None,
        }
    }

    fn apply_dynamic_json_chain_filter(self, head: &str, tail: &str, value: &JsonValue) -> Self {
        let _ = (tail, value);
        match head {
            "media_library" => {
                self.with_media_library_matching(
                    crate::Q::media_libraries_minimal()
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
        self.query = self.query.project("session_fp");
        self.query = self.query.project("active");
        self.query = self.query.project("last_seen_at");
        self.query = self.query.project("expire_at");
        self.query = self.query.project("create_time");
        self.query = self.query.project("update_time");
        self.query = self.query.project("version");
        self.query = self.query.project("media_library_id");
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
        let mut request = self.select_self();
        request = request.select_media_library();
        request
    }

    pub fn select_children(self) -> Self {
        self.select_all()
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


    pub fn select_session_fp(mut self) -> Self {
        self.query = self.query.project("session_fp");
        self
    }

    pub fn project_session_fp(self) -> Self {
        self.select_session_fp()
    }

    pub fn select_session_fp_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_session_fp_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_session_fp_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("session_fp", raw_sql_segment));
        self
    }

    pub fn group_by_session_fp(self) -> Self {
        self.group_by("session_fp")
    }

    pub fn group_by_session_fp_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("session_fp");
        request.query = request
            .query
            .project_expr(alias, Expr::column("session_fp"));
        request
    }

    pub fn group_by_session_fp_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("session_fp")
            .aggregate_with_function("session_fp", alias, function)
    }

    pub fn count_session_fp(self) -> Self {
        self.count_session_fp_as("session_fp_count")
    }

    pub fn count_session_fp_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("session_fp", alias)
    }

    pub fn sum_session_fp(self) -> Self {
        self.sum_session_fp_as("sum_session_fp")
    }

    pub fn sum_session_fp_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("session_fp", alias)
    }

    pub fn avg_session_fp(self) -> Self {
        self.avg_session_fp_as("avg_session_fp")
    }

    pub fn avg_session_fp_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("session_fp", alias)
    }

    pub fn min_session_fp(self) -> Self {
        self.min_session_fp_as("min_session_fp")
    }

    pub fn min_session_fp_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("session_fp", alias)
    }

    pub fn max_session_fp(self) -> Self {
        self.max_session_fp_as("max_session_fp")
    }

    pub fn max_session_fp_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("session_fp", alias)
    }

    pub fn unselect_session_fp(mut self) -> Self {
        self.query.projection.retain(|field| field != "session_fp");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "session_fp");
        self
    }


    pub fn with_session_fp(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "session_fp",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_session_fp_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "session_fp",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_session_fp_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("session_fp", value));
        self
    }



    pub fn with_session_fp_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("session_fp", value));
        self
    }

    pub fn with_session_fp_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("session_fp", value));
        self
    }

    pub fn with_session_fp_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("session_fp", value));
        self
    }

    pub fn with_session_fp_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("session_fp", value));
        self
    }

    pub fn with_session_fp_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("session_fp", value));
        self
    }

    pub fn with_session_fp_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("session_fp", lower, upper));
        self
    }

    pub fn with_session_fp_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "session_fp",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_session_fp_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "session_fp",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_session_fp_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "session_fp",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_session_fp_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("session_fp", value));
        self
    }

    pub fn with_session_fp_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("session_fp", value));
        self
    }

    pub fn with_session_fp_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("session_fp", value));
        self
    }

    pub fn with_session_fp_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("session_fp", value));
        self
    }

    pub fn with_session_fp_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("session_fp", value));
        self
    }

    pub fn with_session_fp_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("session_fp", value));
        self
    }

    pub fn with_session_fp_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("session_fp", value));
        self
    }
    pub fn with_session_fp_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("session_fp", value));
        self
    }

    pub fn with_session_fp_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("session_fp", value));
        self
    }

    pub fn with_session_fp_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("session_fp"));
        self
    }



    pub fn with_session_fp_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("session_fp"));
        self
    }


    pub fn order_by_session_fp_asc(mut self) -> Self {
        self.query = self.query.order_asc("session_fp");
        self
    }

    pub fn order_by_session_fp_desc(mut self) -> Self {
        self.query = self.query.order_desc("session_fp");
        self
    }

    pub fn order_by_session_fp_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("session_fp");
        self
    }

    pub fn order_by_session_fp_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("session_fp");
        self
    }


    pub fn select_active(mut self) -> Self {
        self.query = self.query.project("active");
        self
    }

    pub fn project_active(self) -> Self {
        self.select_active()
    }

    pub fn select_active_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_active_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_active_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("active", raw_sql_segment));
        self
    }

    pub fn group_by_active(self) -> Self {
        self.group_by("active")
    }

    pub fn group_by_active_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("active");
        request.query = request
            .query
            .project_expr(alias, Expr::column("active"));
        request
    }

    pub fn group_by_active_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("active")
            .aggregate_with_function("active", alias, function)
    }

    pub fn count_active(self) -> Self {
        self.count_active_as("active_count")
    }

    pub fn count_active_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("active", alias)
    }

    pub fn sum_active(self) -> Self {
        self.sum_active_as("sum_active")
    }

    pub fn sum_active_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("active", alias)
    }

    pub fn avg_active(self) -> Self {
        self.avg_active_as("avg_active")
    }

    pub fn avg_active_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("active", alias)
    }

    pub fn min_active(self) -> Self {
        self.min_active_as("min_active")
    }

    pub fn min_active_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("active", alias)
    }

    pub fn max_active(self) -> Self {
        self.max_active_as("max_active")
    }

    pub fn max_active_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("active", alias)
    }

    pub fn unselect_active(mut self) -> Self {
        self.query.projection.retain(|field| field != "active");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "active");
        self
    }

    pub fn which_are_active(mut self) -> Self {
        self.query = self.query.and_filter(Expr::eq("active", true));
        self
    }

    pub fn which_are_not_active(mut self) -> Self {
        self.query = self.query.and_filter(Expr::eq("active", false));
        self
    }

    pub fn with_active_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("active"));
        self
    }

    pub fn with_active_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("active"));
        self
    }
    pub fn order_by_active_asc(mut self) -> Self {
        self.query = self.query.order_asc("active");
        self
    }

    pub fn order_by_active_desc(mut self) -> Self {
        self.query = self.query.order_desc("active");
        self
    }

    pub fn order_by_active_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("active");
        self
    }

    pub fn order_by_active_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("active");
        self
    }


    pub fn select_last_seen_at(mut self) -> Self {
        self.query = self.query.project("last_seen_at");
        self
    }

    pub fn project_last_seen_at(self) -> Self {
        self.select_last_seen_at()
    }

    pub fn select_last_seen_at_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_last_seen_at_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_last_seen_at_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("last_seen_at", raw_sql_segment));
        self
    }

    pub fn group_by_last_seen_at(self) -> Self {
        self.group_by("last_seen_at")
    }

    pub fn group_by_last_seen_at_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("last_seen_at");
        request.query = request
            .query
            .project_expr(alias, Expr::column("last_seen_at"));
        request
    }

    pub fn group_by_last_seen_at_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("last_seen_at")
            .aggregate_with_function("last_seen_at", alias, function)
    }

    pub fn count_last_seen_at(self) -> Self {
        self.count_last_seen_at_as("last_seen_at_count")
    }

    pub fn count_last_seen_at_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("last_seen_at", alias)
    }

    pub fn sum_last_seen_at(self) -> Self {
        self.sum_last_seen_at_as("sum_last_seen_at")
    }

    pub fn sum_last_seen_at_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("last_seen_at", alias)
    }

    pub fn avg_last_seen_at(self) -> Self {
        self.avg_last_seen_at_as("avg_last_seen_at")
    }

    pub fn avg_last_seen_at_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("last_seen_at", alias)
    }

    pub fn min_last_seen_at(self) -> Self {
        self.min_last_seen_at_as("min_last_seen_at")
    }

    pub fn min_last_seen_at_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("last_seen_at", alias)
    }

    pub fn max_last_seen_at(self) -> Self {
        self.max_last_seen_at_as("max_last_seen_at")
    }

    pub fn max_last_seen_at_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("last_seen_at", alias)
    }

    pub fn unselect_last_seen_at(mut self) -> Self {
        self.query.projection.retain(|field| field != "last_seen_at");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "last_seen_at");
        self
    }


    pub fn with_last_seen_at(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "last_seen_at",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_last_seen_at_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "last_seen_at",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_last_seen_at_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("last_seen_at", value));
        self
    }



    pub fn with_last_seen_at_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("last_seen_at", value));
        self
    }

    pub fn with_last_seen_at_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("last_seen_at", value));
        self
    }

    pub fn with_last_seen_at_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("last_seen_at", value));
        self
    }

    pub fn with_last_seen_at_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("last_seen_at", value));
        self
    }

    pub fn with_last_seen_at_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("last_seen_at", value));
        self
    }

    pub fn with_last_seen_at_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("last_seen_at", lower, upper));
        self
    }

    pub fn with_last_seen_at_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "last_seen_at",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_last_seen_at_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "last_seen_at",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_last_seen_at_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "last_seen_at",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_last_seen_at_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("last_seen_at", value));
        self
    }

    pub fn with_last_seen_at_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("last_seen_at", value));
        self
    }

    pub fn with_last_seen_at_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("last_seen_at", value));
        self
    }

    pub fn with_last_seen_at_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("last_seen_at", value));
        self
    }

    pub fn with_last_seen_at_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("last_seen_at", value));
        self
    }

    pub fn with_last_seen_at_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("last_seen_at", value));
        self
    }

    pub fn with_last_seen_at_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("last_seen_at", value));
        self
    }
    pub fn with_last_seen_at_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("last_seen_at", value));
        self
    }

    pub fn with_last_seen_at_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("last_seen_at", value));
        self
    }

    pub fn with_last_seen_at_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("last_seen_at"));
        self
    }



    pub fn with_last_seen_at_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("last_seen_at"));
        self
    }


    pub fn order_by_last_seen_at_asc(mut self) -> Self {
        self.query = self.query.order_asc("last_seen_at");
        self
    }

    pub fn order_by_last_seen_at_desc(mut self) -> Self {
        self.query = self.query.order_desc("last_seen_at");
        self
    }

    pub fn order_by_last_seen_at_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("last_seen_at");
        self
    }

    pub fn order_by_last_seen_at_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("last_seen_at");
        self
    }


    pub fn select_expire_at(mut self) -> Self {
        self.query = self.query.project("expire_at");
        self
    }

    pub fn project_expire_at(self) -> Self {
        self.select_expire_at()
    }

    pub fn select_expire_at_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_expire_at_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_expire_at_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("expire_at", raw_sql_segment));
        self
    }

    pub fn group_by_expire_at(self) -> Self {
        self.group_by("expire_at")
    }

    pub fn group_by_expire_at_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("expire_at");
        request.query = request
            .query
            .project_expr(alias, Expr::column("expire_at"));
        request
    }

    pub fn group_by_expire_at_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("expire_at")
            .aggregate_with_function("expire_at", alias, function)
    }

    pub fn count_expire_at(self) -> Self {
        self.count_expire_at_as("expire_at_count")
    }

    pub fn count_expire_at_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("expire_at", alias)
    }

    pub fn sum_expire_at(self) -> Self {
        self.sum_expire_at_as("sum_expire_at")
    }

    pub fn sum_expire_at_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("expire_at", alias)
    }

    pub fn avg_expire_at(self) -> Self {
        self.avg_expire_at_as("avg_expire_at")
    }

    pub fn avg_expire_at_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("expire_at", alias)
    }

    pub fn min_expire_at(self) -> Self {
        self.min_expire_at_as("min_expire_at")
    }

    pub fn min_expire_at_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("expire_at", alias)
    }

    pub fn max_expire_at(self) -> Self {
        self.max_expire_at_as("max_expire_at")
    }

    pub fn max_expire_at_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("expire_at", alias)
    }

    pub fn unselect_expire_at(mut self) -> Self {
        self.query.projection.retain(|field| field != "expire_at");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "expire_at");
        self
    }


    pub fn with_expire_at(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "expire_at",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_expire_at_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "expire_at",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_expire_at_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("expire_at", value));
        self
    }



    pub fn with_expire_at_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("expire_at", value));
        self
    }

    pub fn with_expire_at_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("expire_at", value));
        self
    }

    pub fn with_expire_at_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("expire_at", value));
        self
    }

    pub fn with_expire_at_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("expire_at", value));
        self
    }

    pub fn with_expire_at_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("expire_at", value));
        self
    }

    pub fn with_expire_at_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("expire_at", lower, upper));
        self
    }

    pub fn with_expire_at_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "expire_at",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_expire_at_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "expire_at",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_expire_at_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "expire_at",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_expire_at_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("expire_at", value));
        self
    }

    pub fn with_expire_at_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("expire_at", value));
        self
    }

    pub fn with_expire_at_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("expire_at", value));
        self
    }

    pub fn with_expire_at_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("expire_at", value));
        self
    }

    pub fn with_expire_at_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("expire_at", value));
        self
    }

    pub fn with_expire_at_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("expire_at", value));
        self
    }

    pub fn with_expire_at_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("expire_at", value));
        self
    }
    pub fn with_expire_at_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("expire_at", value));
        self
    }

    pub fn with_expire_at_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("expire_at", value));
        self
    }

    pub fn with_expire_at_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("expire_at"));
        self
    }



    pub fn with_expire_at_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("expire_at"));
        self
    }


    pub fn order_by_expire_at_asc(mut self) -> Self {
        self.query = self.query.order_asc("expire_at");
        self
    }

    pub fn order_by_expire_at_desc(mut self) -> Self {
        self.query = self.query.order_desc("expire_at");
        self
    }

    pub fn order_by_expire_at_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("expire_at");
        self
    }

    pub fn order_by_expire_at_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("expire_at");
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
    pub fn filter_by_media_library(mut self, value: impl EntityReference) -> Self {
        self.query = self.query.and_filter(Expr::eq("media_library_id", value.entity_id_value()));
        self
    }

    pub fn with_media_library_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "media_library_id",
            <crate::MediaLibrary as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("media_library", selection));
        self
    }


    pub fn without_media_library_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "media_library_id",
            <crate::MediaLibrary as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("media_library", selection));
        self
    }


    pub fn have_media_library(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("media_library_id"));
        self
    }

    pub fn have_no_media_library(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("media_library_id"));
        self
    }


    pub fn group_by_media_library(self) -> Self {
        self.group_by("media_library_id")
    }

    pub fn group_by_media_library_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("media_library_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("media_library_id"));
        request
    }

    pub fn group_by_media_library_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("media_library_id")
            .aggregate_with_function("media_library_id", alias, function)
    }

    pub fn group_by_media_library_with(mut self, request: impl Into<QuerySelection>) -> Self {
        self.query = self.query.group_by("media_library_id");
        self.query_options.object_group_bys.push(ObjectGroupBy::new(
            "media_library",
            "media_library_id",
            request,
        ));
        self
    }

    pub fn group_by_media_library_with_details(self) -> Self {
        self.group_by_media_library_with_details_from(crate::Q::media_libraries().unlimited())
    }

    pub fn group_by_media_library_with_details_from(self, request: impl Into<QuerySelection>) -> Self {
        self.group_by_media_library_with(request)
    }


    pub fn roll_up_to_media_library(self) -> Self {
        self.roll_up_to_media_library_with(crate::Q::media_libraries().unlimited())
    }

    pub fn roll_up_to_media_library_with(self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.with_media_library_matching(selection.clone())
            .group_by_media_library_with(selection)
    }

    pub fn count_media_library(self) -> Self {
        self.count_media_library_as("media_library_count")
    }

    pub fn count_media_library_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("media_library_id", alias)
    }

    pub fn unselect_media_library(mut self) -> Self {
        self.query.projection.retain(|field| field != "media_library_id");
        self.query.relations.retain(|relation| relation.name != "media_library");
        self
    }
    pub fn select_media_library(mut self) -> Self {
        self.query.relations.push(teaql_core::RelationLoad::new("media_library")
            .with_operation(QueryOperation::new(QueryOperationKind::RelationSelect, "select_media_library").with_relation("media_library")));
        self
    }

    pub fn select_media_library_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query.relations.push(teaql_core::RelationLoad::with_query("media_library", selection.into_query())
            .with_operation(QueryOperation::new(QueryOperationKind::RelationSelect, "select_media_library_with").with_relation("media_library")));
        self
}

    pub fn facet_by_media_library_as(self, facet_name: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.facet_by_media_library_internal(facet_name, request, true, "facet_by_media_library_as")
    }

    pub fn facet_by_media_library_as_with_options(
        self,
        facet_name: impl Into<String>,
        request: impl Into<QuerySelection>,
        include_all_facets: bool,
    ) -> Self {
        self.facet_by_media_library_internal(facet_name, request, include_all_facets, "facet_by_media_library_as_with_options")
    }

    fn facet_by_media_library_internal(mut self, facet_name: impl Into<String>, request: impl Into<QuerySelection>, include_all_facets: bool, method: &'static str) -> Self {
        let facet_name = facet_name.into();
        let operation = QueryOperation::new(QueryOperationKind::Facet, method)
            .with_alias(&facet_name).with_relation("media_library");
        self.query_options.facets.push(FacetRequest::new(
            facet_name,
            "media_library",
            request,
            include_all_facets,
        ).with_operation(operation));
        self
    }
}

impl<R> Default for UserSessionRequest<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R> From< UserSessionRequest<R> > for SelectQuery {
    fn from(request: UserSessionRequest<R>) -> Self {
        QuerySelection::from(request).into_query()
    }
}

impl<R> From< UserSessionRequest<R> > for QuerySelection {
    fn from(request: UserSessionRequest<R>) -> Self {
        Self {
            query: request.query,
            relation_selections: request.relation_selections,
            relation_filters: request.relation_filters,
            child_enhancements: request.child_enhancements,
            query_options: request.query_options,
        }
    }
}


impl<'a, C> crate::request_support::AuditedSave<'a, C> for teaql_core::Audited<crate::UserSession> 
where C: crate::TeaqlRuntime + ?Sized + 'a
{
    type Error = teaql_runtime::RuntimeError;
    type Entity = crate::UserSession;
    fn save(self, context: &'a C) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Entity, Self::Error>> + Send + '_>> {
        Box::pin(async move {
            context.save_audited_entity(self).await
        })
    }
}

impl<R: teaql_core::Entity> crate::PurposedQuery<UserSessionRequest<R>> {
    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.inner.query_options.comment = Some(comment.into());
        self
    }

    pub fn new_entity<C>(&self, context: &C) -> crate::UserSession
    where
        C: crate::TeaqlRuntime + ?Sized,
    {
        self.require_comment();
        let mut entity = crate::UserSession::runtime_new(context.user_context().entity_runtime_state());
        if let Ok(id) = context.user_context().next_id(crate::UserSession::ENTITY_NAME) {
            entity.update_id(id);
        }
        teaql_core::Entity::mark_as_new(&mut entity);
        entity
    }

    fn into_inner_with_trace(mut self) -> UserSessionRequest<R> {
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
