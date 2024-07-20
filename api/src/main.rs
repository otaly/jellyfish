use std::{env, net::SocketAddr};

use axum::{extract::Query, http::StatusCode, routing::get, Json, Router};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new().route("/hello", get(hello));

    let addr = SocketAddr::from((
        [0, 0, 0, 0],
        env::var("PORT").expect("PORT not set").parse().unwrap(),
    ));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

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
