use dino_macros::{FromJs, IntoJs};
use std::collections::HashMap;

#[derive(IntoJs, Debug)]
pub struct Request {
    pub headers: HashMap<String, String>,
    pub method: String,
    pub url: String,
    pub body: Option<String>,
}

#[derive(FromJs, Debug)]
pub struct Response {
    pub headers: HashMap<String, String>,
    pub status: u16,
    pub body: Option<String>,
}

fn main() {}
