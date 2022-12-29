use http::method;
use serde::{Deserialize, Serialize};

pub struct SurrealDbConnector {
    pub url: String,
}
// implement default
impl Default for SurrealDbConnector {
    fn default() -> Self {
        SurrealDbConnector {
            url: "http://localhost:4400".to_string(),
        }
    }
}

impl SurrealDbConnector {
    pub fn get_builder() -> http::request::Builder {
        http::Request::builder()
            .method(method::Method::POST)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("NS", "my_namespace")
            .header("DB", "my_namespace")
            .header("Authorization", "Basic cm9vdDpyb290")
            .uri("http://0.0.0.0:4400/sql")
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SurrealdbResponse {
    pub time: String,
    pub status: String,
    pub result: Vec<serde_json::Value>,
}
