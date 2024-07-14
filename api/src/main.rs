use axum::{extract::Query, http::StatusCode, routing::get, Json, Router};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello", get(hello));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello(params: Query<Params>) -> (StatusCode, Json<Greeting>) {
    let Query(params) = params;
    (
        StatusCode::OK,
        Json(Greeting {
            greeting: match params {
                Params { name: Some(name) } => format!("Hello, World! {}!!", name),
                Params { name: None } => "Hello, World!".into(),
            },
        }),
    )
}

#[derive(Debug, Deserialize, Default)]
struct Params {
    name: Option<String>,
}

#[derive(Serialize)]
struct Greeting {
    greeting: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hello() {
        assert_eq!(hello(Query(Params { name: None })).await.0, StatusCode::OK);
    }
}
