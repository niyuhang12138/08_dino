use crate::{error::AppError, AppState};
use axum::{
    body::Bytes,
    extract::{Query, State},
    http::request::Parts,
    response::IntoResponse,
    Json,
};
use axum_extra::extract::Host;
use serde_json::json;
use std::collections::HashMap;

/// we only support requests and return JSON responses
#[allow(unused)]
pub(crate) async fn handler(
    State(state): State<AppState>,
    parts: Parts,
    Host(mut host): Host,
    Query(query): Query<serde_json::Value>,
    body: Bytes,
) -> Result<impl IntoResponse, AppError> {
    // get router from state
    // match router with parts.path fer a handler
    // convert request data into Req and call handler with a js runtime
    // convert Req into response and return
    // info!("state: {:?}", state);
    // info!("parts: {:?}", parts);
    // info!("query: {:?}", query);
    // info!("body: {:?}", body);
    // info!("host: {:?}", host);

    host.split_off(host.find(":").unwrap_or(host.len()));

    let router = state
        .routes
        .get(&host)
        .ok_or(AppError::HostNotFound(host))?
        .load();

    let matched = router.match_it(parts.method, parts.uri.path())?;

    let handler = matched.value;

    let params = matched
        .params
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect::<HashMap<_, _>>();

    let body = if body.is_empty() {
        serde_json::Value::Null
    } else {
        serde_json::from_slice(&body)?
    };

    Ok(Json(json!(
       {
        "handler": handler,
        "params": params,
        "query": query,
        "body": body
       }
    )))
}
