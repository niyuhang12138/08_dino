use crate::{error::AppError, AppState, JsWorker, Req};
use axum::{
    body::Bytes,
    extract::{Query, State},
    http::{request::Parts, Response},
    response::IntoResponse,
};
use axum_extra::extract::Host;
use std::collections::HashMap;

/// we only support requests and return JSON responses
#[allow(unused)]
pub(crate) async fn handler(
    State(state): State<AppState>,
    parts: Parts,
    Host(mut host): Host,
    Query(query): Query<HashMap<String, String>>,
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

    let matched = router.match_it(parts.method.clone(), parts.uri.path())?;

    let handler = matched.value;

    let params = matched
        .params
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect::<HashMap<_, _>>();

    let headers = parts
        .headers
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap().to_string()))
        .collect::<HashMap<_, _>>();

    let body = String::from_utf8(body.to_vec()).ok();

    let req = Req::builder()
        .method(parts.method.to_string())
        .url(parts.uri.to_string())
        .headers(headers)
        .query(query)
        .params(params)
        .body(body)
        .build();

    // call handler with req
    let work = JsWorker::try_new(&router.code)?;
    let res = work.run(handler, req)?;

    // covert Req into response and return
    Ok(Response::from(res))
}
