// Copyright 2023 Greptime Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Instant;

use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, HeaderName, HeaderValue, StatusCode};
use axum::response::IntoResponse;
use axum::Extension;
use axum_extra::TypedHeader;
use common_error::ext::ErrorExt;
use common_telemetry::{debug, error};
use headers::ContentType;
use once_cell::sync::Lazy;
use pipeline::{
    GreptimePipelineParams, PipelineDefinition, GREPTIME_INTERNAL_IDENTITY_PIPELINE_NAME,
};
use serde_json::{json, Deserializer, Value};
use session::context::{Channel, QueryContext};
use snafu::{ensure, ResultExt};
use vrl::value::Value as VrlValue;

use crate::error::{
    status_code_to_http_status, InvalidElasticsearchInputSnafu, ParseJsonSnafu,
    Result as ServersResult,
};
use crate::http::event::{
    extract_pipeline_params_map_from_headers, ingest_logs_inner, LogIngesterQueryParams, LogState,
    PipelineIngestRequest,
};
use crate::http::header::constants::GREPTIME_PIPELINE_NAME_HEADER_NAME;
use crate::metrics::{
    METRIC_ELASTICSEARCH_LOGS_DOCS_COUNT, METRIC_ELASTICSEARCH_LOGS_INGESTION_ELAPSED,
};

// The headers for every response of Elasticsearch API.
static ELASTICSEARCH_HEADERS: Lazy<HeaderMap> = Lazy::new(|| {
    HeaderMap::from_iter([
        (
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        ),
        (
            HeaderName::from_static("x-elastic-product"),
            HeaderValue::from_static("Elasticsearch"),
        ),
    ])
});

// The fake version of Elasticsearch and used for `_version` API.
const ELASTICSEARCH_VERSION: &str = "8.16.0";

// Return fake response for Elasticsearch ping request.
#[axum_macros::debug_handler]
pub async fn handle_get_version() -> impl IntoResponse {
    let body = serde_json::json!({
        "version": {
            "number": ELASTICSEARCH_VERSION
        }
    });
    (StatusCode::OK, elasticsearch_headers(), axum::Json(body))
}

// Return fake response for Elasticsearch license request.
// Reference: https://www.elastic.co/guide/en/elasticsearch/reference/current/get-license.html.
#[axum_macros::debug_handler]
pub async fn handle_get_license() -> impl IntoResponse {
    let body = serde_json::json!({
        "license": {
            "uid": "cbff45e7-c553-41f7-ae4f-9205eabd80xx",
            "type": "oss",
            "status": "active",
            "expiry_date_in_millis": 4891198687000_i64,
        }
    });
    (StatusCode::OK, elasticsearch_headers(), axum::Json(body))
}

/// Process `_bulk` API requests. Only support to create logs.
/// Reference: https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-bulk.html#docs-bulk-api-request.
#[axum_macros::debug_handler]
pub async fn handle_bulk_api(
    State(log_state): State<LogState>,
    Query(params): Query<LogIngesterQueryParams>,
    Extension(query_ctx): Extension<QueryContext>,
    TypedHeader(_content_type): TypedHeader<ContentType>,
    headers: HeaderMap,
    payload: String,
) -> impl IntoResponse {
    do_handle_bulk_api(log_state, None, params, query_ctx, headers, payload).await
}

/// Process `/${index}/_bulk` API requests. Only support to create logs.
/// Reference: https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-bulk.html#docs-bulk-api-request.
#[axum_macros::debug_handler]
pub async fn handle_bulk_api_with_index(
    State(log_state): State<LogState>,
    Path(index): Path<String>,
    Query(params): Query<LogIngesterQueryParams>,
    Extension(query_ctx): Extension<QueryContext>,
    TypedHeader(_content_type): TypedHeader<ContentType>,
    headers: HeaderMap,
    payload: String,
) -> impl IntoResponse {
    do_handle_bulk_api(log_state, Some(index), params, query_ctx, headers, payload).await
}

async fn do_handle_bulk_api(
    log_state: LogState,
    index: Option<String>,
    params: LogIngesterQueryParams,
    mut query_ctx: QueryContext,
    headers: HeaderMap,
    payload: String,
) -> impl IntoResponse {
    let start = Instant::now();
    debug!(
        "Received bulk request, params: {:?}, payload: {:?}",
        params, payload
    );

    // The `schema` is already set in the query_ctx in auth process.
    query_ctx.set_channel(Channel::Elasticsearch);

    let db = query_ctx.current_schema();

    // Record the ingestion time histogram.
    let _timer = METRIC_ELASTICSEARCH_LOGS_INGESTION_ELAPSED
        .with_label_values(&[&db])
        .start_timer();

    // If pipeline_name is not provided, use the internal pipeline.
    let pipeline_name = params.pipeline_name.as_deref().unwrap_or_else(|| {
        headers
            .get(GREPTIME_PIPELINE_NAME_HEADER_NAME)
            .and_then(|v| v.to_str().ok())
            .unwrap_or(GREPTIME_INTERNAL_IDENTITY_PIPELINE_NAME)
    });

    // Read the ndjson payload and convert it to a vector of Value.
    let requests = match parse_bulk_request(&payload, &index, &params.msg_field) {
        Ok(requests) => requests,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                elasticsearch_headers(),
                axum::Json(write_bulk_response(
                    start.elapsed().as_millis() as i64,
                    0,
                    StatusCode::BAD_REQUEST.as_u16() as u32,
                    e.to_string().as_str(),
                )),
            );
        }
    };
    let log_num = requests.len();

    let pipeline = match PipelineDefinition::from_name(pipeline_name, None, None) {
        Ok(pipeline) => pipeline,
        Err(e) => {
            // should be unreachable
            error!(e; "Failed to ingest logs");
            return (
                status_code_to_http_status(&e.status_code()),
                elasticsearch_headers(),
                axum::Json(write_bulk_response(
                    start.elapsed().as_millis() as i64,
                    0,
                    e.status_code() as u32,
                    e.to_string().as_str(),
                )),
            );
        }
    };
    let pipeline_params =
        GreptimePipelineParams::from_map(extract_pipeline_params_map_from_headers(&headers));
    if let Err(e) = ingest_logs_inner(
        log_state.log_handler,
        pipeline,
        requests,
        Arc::new(query_ctx),
        pipeline_params,
    )
    .await
    {
        error!(e; "Failed to ingest logs");
        return (
            status_code_to_http_status(&e.status_code()),
            elasticsearch_headers(),
            axum::Json(write_bulk_response(
                start.elapsed().as_millis() as i64,
                0,
                e.status_code() as u32,
                e.to_string().as_str(),
            )),
        );
    }

    // Record the number of documents ingested.
    METRIC_ELASTICSEARCH_LOGS_DOCS_COUNT
        .with_label_values(&[&db])
        .inc_by(log_num as u64);

    (
        StatusCode::OK,
        elasticsearch_headers(),
        axum::Json(write_bulk_response(
            start.elapsed().as_millis() as i64,
            log_num,
            StatusCode::CREATED.as_u16() as u32,
            "",
        )),
    )
}

// It will generate the following response when write _bulk request to GreptimeDB successfully:
// {
//     "took": 1000,
//     "errors": false,
//     "items": [
//         { "create": { "status": 201 } },
//         { "create": { "status": 201 } },
//         ...
//     ]
// }
// If the status code is not 201, it will generate the following response:
// {
//     "took": 1000,
//     "errors": true,
//     "items": [
//         { "create": { "status": 400, "error": { "type": "illegal_argument_exception", "reason": "<error_reason>" } } }
//     ]
// }
fn write_bulk_response(took_ms: i64, n: usize, status_code: u32, error_reason: &str) -> Value {
    if error_reason.is_empty() {
        let items: Vec<Value> = (0..n)
            .map(|_| {
                json!({
                    "create": {
                        "status": status_code
                    }
                })
            })
            .collect();
        json!({
            "took": took_ms,
            "errors": false,
            "items": items,
        })
    } else {
        json!({
            "took": took_ms,
            "errors": true,
            "items": [
                { "create": { "status": status_code, "error": { "type": "illegal_argument_exception", "reason": error_reason } } }
            ]
        })
    }
}

/// Returns the headers for every response of Elasticsearch API.
pub fn elasticsearch_headers() -> HeaderMap {
    ELASTICSEARCH_HEADERS.clone()
}

// Parse the Elasticsearch bulk request and convert it to multiple LogIngestRequests.
// The input will be Elasticsearch bulk request in NDJSON format.
// For example, the input will be like this:
// { "index" : { "_index" : "test", "_id" : "1" } }
// { "field1" : "value1" }
// { "index" : { "_index" : "test", "_id" : "2" } }
// { "field2" : "value2" }
fn parse_bulk_request(
    input: &str,
    index_from_url: &Option<String>,
    msg_field: &Option<String>,
) -> ServersResult<Vec<PipelineIngestRequest>> {
    // Read the ndjson payload and convert it to `Vec<Value>`. Return error if the input is not a valid JSON.
    let values: Vec<VrlValue> = Deserializer::from_str(input)
        .into_iter::<VrlValue>()
        .collect::<Result<_, _>>()
        .context(ParseJsonSnafu)?;

    // Check if the input is empty.
    ensure!(
        !values.is_empty(),
        InvalidElasticsearchInputSnafu {
            reason: "empty bulk request".to_string(),
        }
    );

    let mut requests: Vec<PipelineIngestRequest> = Vec::with_capacity(values.len() / 2);
    let mut values = values.into_iter();

    // Read the ndjson payload and convert it to a (index, value) vector.
    // For Elasticsearch post `_bulk` API, each chunk contains two objects:
    //   1. The first object is the command, it should be `create` or `index`.
    //   2. The second object is the document data.
    while let Some(cmd) = values.next() {
        // NOTE: Although the native Elasticsearch API supports upsert in `index` command, we don't support change any data in `index` command and it's same as `create` command.
        let mut cmd = cmd.into_object();
        let index = if let Some(cmd) = cmd.as_mut().and_then(|c| c.remove("create")) {
            get_index_from_cmd(cmd)?
        } else if let Some(cmd) = cmd.as_mut().and_then(|c| c.remove("index")) {
            get_index_from_cmd(cmd)?
        } else {
            return InvalidElasticsearchInputSnafu {
                reason: format!(
                    "invalid bulk request, expected 'create' or 'index' but got {:?}",
                    cmd
                ),
            }
            .fail();
        };

        // Read the second object to get the document data. Stop the loop if there is no document.
        if let Some(document) = values.next() {
            // If the msg_field is provided, fetch the value of the field from the document data.
            let log_value = if let Some(msg_field) = msg_field {
                get_log_value_from_msg_field(document, msg_field)
            } else {
                document
            };

            ensure!(
                index.is_some() || index_from_url.is_some(),
                InvalidElasticsearchInputSnafu {
                    reason: "missing index in bulk request".to_string(),
                }
            );

            requests.push(PipelineIngestRequest {
                table: index.unwrap_or_else(|| index_from_url.as_ref().unwrap().clone()),
                values: vec![log_value],
            });
        }
    }

    debug!(
        "Received {} log ingest requests: {:?}",
        requests.len(),
        requests
    );

    Ok(requests)
}

// Get the index from the command. We will take index as the table name in GreptimeDB.
fn get_index_from_cmd(v: VrlValue) -> ServersResult<Option<String>> {
    let Some(index) = v.into_object().and_then(|mut m| m.remove("_index")) else {
        return Ok(None);
    };

    if let VrlValue::Bytes(index) = index {
        Ok(Some(String::from_utf8_lossy(&index).to_string()))
    } else {
        // If the `_index` exists, it should be a string.
        InvalidElasticsearchInputSnafu {
            reason: "index is not a string in bulk request",
        }
        .fail()
    }
}

// If the msg_field is provided, fetch the value of the field from the document data.
// For example, if the `msg_field` is `message`, and the document data is `{"message":"hello"}`, the log value will be Value::String("hello").
fn get_log_value_from_msg_field(v: VrlValue, msg_field: &str) -> VrlValue {
    let VrlValue::Object(mut m) = v else {
        return v;
    };

    if let Some(message) = m.remove(msg_field) {
        match message {
            VrlValue::Bytes(bytes) => {
                match serde_json::from_slice::<VrlValue>(&bytes) {
                    Ok(v) => v,
                    // If the message is not a valid JSON, return a map with the original message key and value.
                    Err(_) => {
                        let map = BTreeMap::from([(
                            msg_field.to_string().into(),
                            VrlValue::Bytes(bytes),
                        )]);
                        VrlValue::Object(map)
                    }
                }
            }
            // If the message is not a string, just use the original message as the log value.
            _ => message,
        }
    } else {
        // If the msg_field is not found, just use the original message as the log value.
        VrlValue::Object(m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bulk_request() {
        let test_cases = vec![
            // Normal case.
            (
                r#"
                {"create":{"_index":"test","_id":"1"}}
                {"foo1":"foo1_value", "bar1":"bar1_value"}
                {"create":{"_index":"test","_id":"2"}}
                {"foo2":"foo2_value","bar2":"bar2_value"}
                "#,
                None,
                None,
                Ok(vec![
                    PipelineIngestRequest {
                        table: "test".to_string(),
                        values: vec![
                            json!({"foo1": "foo1_value", "bar1": "bar1_value"}).into(),
                        ],
                    },
                    PipelineIngestRequest {
                        table: "test".to_string(),
                        values: vec![
                            json!({"foo2": "foo2_value", "bar2": "bar2_value"}).into(),
                        ],
                    },
                ]),
            ),
            // Case with index.
            (
                r#"
                {"create":{"_index":"test","_id":"1"}}
                {"foo1":"foo1_value", "bar1":"bar1_value"}
                {"create":{"_index":"logs","_id":"2"}}
                {"foo2":"foo2_value","bar2":"bar2_value"}
                "#,
                Some("logs".to_string()),
                None,
                Ok(vec![
                    PipelineIngestRequest {
                        table: "test".to_string(),
                        values: vec![
                            json!({"foo1": "foo1_value", "bar1": "bar1_value"}).into(),
                        ],
                    },
                    PipelineIngestRequest {
                        table: "logs".to_string(),
                        values: vec![
                            json!({"foo2": "foo2_value", "bar2": "bar2_value"}).into(),
                        ],
                    },
                ]),
            ),
            // Case with index.
            (
                r#"
                {"create":{"_index":"test","_id":"1"}}
                {"foo1":"foo1_value", "bar1":"bar1_value"}
                {"create":{"_index":"logs","_id":"2"}}
                {"foo2":"foo2_value","bar2":"bar2_value"}
                "#,
                Some("logs".to_string()),
                None,
                Ok(vec![
                    PipelineIngestRequest {
                        table: "test".to_string(),
                        values: vec![
                            json!({"foo1": "foo1_value", "bar1": "bar1_value"}).into(),
                        ],
                    },
                    PipelineIngestRequest {
                        table: "logs".to_string(),
                        values: vec![
                            json!({"foo2": "foo2_value", "bar2": "bar2_value"}).into(),
                        ],
                    },
                ]),
            ),
            // Case with incomplete bulk request.
            (
                r#"
                {"create":{"_index":"test","_id":"1"}}
                {"foo1":"foo1_value", "bar1":"bar1_value"}
                {"create":{"_index":"logs","_id":"2"}}
                "#,
                Some("logs".to_string()),
                None,
                Ok(vec![
                    PipelineIngestRequest {
                        table: "test".to_string(),
                        values: vec![
                            json!({"foo1": "foo1_value", "bar1": "bar1_value"}).into(),
                        ],
                    },
                ]),
            ),
            // Specify the `data` field as the message field and the value is a JSON string.
            (
                r#"
                {"create":{"_index":"test","_id":"1"}}
                {"data":"{\"foo1\":\"foo1_value\", \"bar1\":\"bar1_value\"}", "not_data":"not_data_value"}
                {"create":{"_index":"test","_id":"2"}}
                {"data":"{\"foo2\":\"foo2_value\", \"bar2\":\"bar2_value\"}", "not_data":"not_data_value"}
                "#,
                None,
                Some("data".to_string()),
                Ok(vec![
                    PipelineIngestRequest {
                        table: "test".to_string(),
                        values: vec![
                            json!({"foo1": "foo1_value", "bar1": "bar1_value"}).into(),
                        ],
                    },
                    PipelineIngestRequest {
                        table: "test".to_string(),
                        values: vec![
                            json!({"foo2": "foo2_value", "bar2": "bar2_value"}).into(),
                        ],
                    },
                ]),
            ),
            // Simulate the log data from Logstash.
            (
                r#"
                {"create":{"_id":null,"_index":"logs-generic-default","routing":null}}
                {"message":"172.16.0.1 - - [25/May/2024:20:19:37 +0000] \"GET /contact HTTP/1.1\" 404 162 \"-\" \"Mozilla/5.0 (iPhone; CPU iPhone OS 14_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Mobile/15E148 Safari/604.1\"","@timestamp":"2025-01-04T04:32:13.868962186Z","event":{"original":"172.16.0.1 - - [25/May/2024:20:19:37 +0000] \"GET /contact HTTP/1.1\" 404 162 \"-\" \"Mozilla/5.0 (iPhone; CPU iPhone OS 14_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Mobile/15E148 Safari/604.1\""},"host":{"name":"orbstack"},"log":{"file":{"path":"/var/log/nginx/access.log"}},"@version":"1","data_stream":{"type":"logs","dataset":"generic","namespace":"default"}}
                {"create":{"_id":null,"_index":"logs-generic-default","routing":null}}
                {"message":"10.0.0.1 - - [25/May/2024:20:18:37 +0000] \"GET /images/logo.png HTTP/1.1\" 304 0 \"-\" \"Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:89.0) Gecko/20100101 Firefox/89.0\"","@timestamp":"2025-01-04T04:32:13.868723810Z","event":{"original":"10.0.0.1 - - [25/May/2024:20:18:37 +0000] \"GET /images/logo.png HTTP/1.1\" 304 0 \"-\" \"Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:89.0) Gecko/20100101 Firefox/89.0\""},"host":{"name":"orbstack"},"log":{"file":{"path":"/var/log/nginx/access.log"}},"@version":"1","data_stream":{"type":"logs","dataset":"generic","namespace":"default"}}
                "#,
                None,
                Some("message".to_string()),
                Ok(vec![
                    PipelineIngestRequest {
                        table: "logs-generic-default".to_string(),
                        values: vec![
                            json!({"message": "172.16.0.1 - - [25/May/2024:20:19:37 +0000] \"GET /contact HTTP/1.1\" 404 162 \"-\" \"Mozilla/5.0 (iPhone; CPU iPhone OS 14_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Mobile/15E148 Safari/604.1\""}).into(),
                        ],
                    },
                    PipelineIngestRequest {
                        table: "logs-generic-default".to_string(),
                        values: vec![
                            json!({"message": "10.0.0.1 - - [25/May/2024:20:18:37 +0000] \"GET /images/logo.png HTTP/1.1\" 304 0 \"-\" \"Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:89.0) Gecko/20100101 Firefox/89.0\""}).into(),
                        ],
                    },
                ]),
            ),
            // With invalid bulk request.
            (
                r#"
                { "not_create_or_index" : { "_index" : "test", "_id" : "1" } }
                { "foo1" : "foo1_value", "bar1" : "bar1_value" }
                "#,
                None,
                None,
                Err(InvalidElasticsearchInputSnafu {
                    reason: "it's a invalid bulk request".to_string(),
                }),
            ),
        ];

        for (input, index, msg_field, expected) in test_cases {
            let requests = parse_bulk_request(input, &index, &msg_field);
            if expected.is_ok() {
                assert_eq!(requests.unwrap(), expected.unwrap());
            } else {
                assert!(requests.is_err());
            }
        }
    }
}
