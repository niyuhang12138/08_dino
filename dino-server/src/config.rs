use anyhow::Result;
use axum::http::Method;
use indexmap::IndexMap;
use serde::Deserialize;
use std::path::Path;

pub type ProjectRoutes = IndexMap<String, Vec<ProjectRoute>>;

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub struct ProjectConfig {
    pub name: String,
    pub routes: ProjectRoutes,
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub struct ProjectRoute {
    #[serde(deserialize_with = "deserialize_method")]
    pub method: Method,
    pub handler: String,
}

fn deserialize_method<'de, D>(deserializer: D) -> Result<Method, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.to_uppercase().as_str() {
        "GET" => Ok(Method::GET),
        "POST" => Ok(Method::POST),
        "PUT" => Ok(Method::PUT),
        "DELETE" => Ok(Method::DELETE),
        "PATCH" => Ok(Method::PATCH),
        "HEAD" => Ok(Method::HEAD),
        "OPTIONS" => Ok(Method::OPTIONS),
        "CONNECT" => Ok(Method::CONNECT),
        "TRACE" => Ok(Method::TRACE),
        _ => Err(serde::de::Error::custom("invalid method")),
    }
}

impl ProjectConfig {
    pub fn load(filename: impl AsRef<Path>) -> Result<Self> {
        let content = std::fs::read_to_string(filename)?;
        let config = serde_yml::from_str(&content)?;
        Ok(config)
    }
}
