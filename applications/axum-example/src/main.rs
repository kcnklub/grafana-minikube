use anyhow::Result;
use axum::{routing::get, Router};

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new().route("/hello", get(hello));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    let _ = axum::serve(listener, app).await?;
    Ok(())
}

async fn hello() -> String {
    "Hello from axum".into()
}
