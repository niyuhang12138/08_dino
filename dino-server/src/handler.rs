use crate::AppState;
use axum::{
    extract::{Query, State},
    http::request::Parts,
    Json,
};

/// we only support requests and return JSON responses
#[allow(unused)]
pub(crate) async fn handler(
    State(state): State<AppState>,
    parts: Parts,
    Query(query): Query<serde_json::Value>,
    Json(body): Json<serde_json::Value>,
) {
    println!("{:?}", state);
    println!("{:?}", parts);
    println!("{:?}", query);
    println!("{:?}", body);

    // get router from state
    // match router with parts.path fer a handler
    // convert request data into Req and call handler with a js runtime
    // convert Req into response and return
    todo!()
}
