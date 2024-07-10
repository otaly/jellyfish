use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> &'static str {
    "Hello, World!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hello() {
        assert_eq!(hello().await, "Hello, World!");
    }
}
