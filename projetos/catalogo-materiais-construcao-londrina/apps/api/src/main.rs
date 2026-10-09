use axum::{routing::get, Json, Router};
use serde::Serialize;
use std::{env, net::SocketAddr};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
    version: &'static str,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok", service: "catalogo-api", version: env!("CARGO_PKG_VERSION") })
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("catalogo_api=info,tower_http=info")))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let port = env::var("PORT").ok().and_then(|value| value.parse::<u16>().ok()).unwrap_or(8080);
    let app = Router::new().route("/api/v1/health", get(health)).layer(TraceLayer::new_for_http());
    let address = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(address).await.expect("Não foi possível abrir a porta da API");
    tracing::info!(%address, "API iniciada");
    axum::serve(listener, app).with_graceful_shutdown(async { let _ = tokio::signal::ctrl_c().await; }).await.expect("A API encerrou com erro");
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;

    #[tokio::test]
    async fn health_endpoint_returns_expected_json() {
        let app = Router::new().route("/api/v1/health", get(health));
        let response = app.oneshot(Request::builder().uri("/api/v1/health").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(body["status"], "ok");
        assert_eq!(body["service"], "catalogo-api");
    }
}
