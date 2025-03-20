use anyhow::Result;
use axum::{routing::get, Router};
use axum_prometheus::PrometheusMetricLayer;
use std::process;
use tower_http::trace::TraceLayer;
use tracing_loki::url::Url;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    let (layer, task) = tracing_loki::builder()
        .label("host", "mine")?
        .extra_field("pid", format!("{}", process::id()))?
        .build_url(Url::parse("http://127.0.0.1:3100").unwrap())?;

    // We need to register our layer with `tracing`.
    tracing_subscriber::registry()
        .with(layer)
        // One could add more layers here, for example logging to stdout:
        // .with(tracing_subscriber::fmt::Layer::new())
        .init();

    tokio::spawn(task);

    tracing::info!(
        task = "tracing_setup",
        result = "success",
        "This is a new message for this morning",
    );

    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();
    let app = Router::new()
        .route("/hello", get(hello))
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .layer(prometheus_layer)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    let _ = axum::serve(listener, app).await?;
    Ok(())
}

#[tracing::instrument(name = "hello!!!!")]
async fn hello() -> String {
    "Hello from axum".into()
}
