use anyhow::Result;
use axum::{routing::get, Router};
use axum_prometheus::PrometheusMetricLayer;

#[tokio::main]
async fn main() -> Result<()> {
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();
    let app = Router::new()
        .route("/hello", get(hello))
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .layer(prometheus_layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    let _ = axum::serve(listener, app).await?;
    Ok(())
}

async fn hello() -> String {
    "Hello from axum".into()
}
