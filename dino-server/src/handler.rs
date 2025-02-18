use crate::{error::AppError, AppRouter, AppState, JsWorker, Req};
use axum::{
    body::Bytes,
    extract::{Query, State},
    http::{request::Parts, Response},
    response::IntoResponse,
};
use axum_extra::extract::Host;
use matchit::Match;
use std::collections::HashMap;
use tracing::info;

/// we only support requests and return JSON responses
/// get router from state
/// match router with parts.path fer a handler
/// convert request data into Req and call handler with a js runtime
/// convert Req into response and return
#[allow(unused)]
pub(crate) async fn handler(
    State(state): State<AppState>,
    parts: Parts,
    Host(mut host): Host,
    Query(query): Query<HashMap<String, String>>,
    body: Bytes,
) -> Result<impl IntoResponse, AppError> {
    // info!("state: {:?}", state);
    // info!("parts: {:?}", parts);
    // info!("query: {:?}", query);
    // info!("body: {:?}", body);
    // info!("host: {:?}", host);

    let router = get_router_by_host(host, state)?;

    let matched = router.match_it(parts.method.clone(), parts.uri.path())?;

    let handler = matched.value;

    let req = assemble_req(&matched, &parts, body, query)?;

    // call handler with req
    // TODO: build worker pool, and send req vis mpsc channel and get res from oneshot channel
    let work = JsWorker::try_new(&router.code)?;
    let res = work.run(handler, req)?;

    // covert Req into response and return
    Ok(Response::from(res))
}

#[allow(unused_must_use)]
fn get_router_by_host(mut host: String, state: AppState) -> Result<AppRouter, AppError> {
    host.split_off(host.find(":").unwrap_or(host.len()));

    info!("Introduction host: {:?}", host);

    let router = state
        .routes
        .get(&host)
        .ok_or(AppError::HostNotFound(host))?
        .load();

    Ok(router)
}

fn assemble_req(
    matched: &Match<&str>,
    parts: &Parts,
    body: Bytes,
    query: HashMap<String, String>,
) -> Result<Req, AppError> {
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

    Ok(req)
}
