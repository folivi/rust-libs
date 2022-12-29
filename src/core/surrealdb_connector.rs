pub struct SurrealDbConnector {
    pub url: String,
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
