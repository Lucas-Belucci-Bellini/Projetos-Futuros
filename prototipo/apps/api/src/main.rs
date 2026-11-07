mod catalog;
mod git;
mod hermes;
mod library;

use axum::{
    extract::DefaultBodyLimit,
    http::{header, HeaderValue, Method},
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;
use std::{
    env,
    net::SocketAddr,
    path::{Path, PathBuf},
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
pub struct AppState {
    database_path: PathBuf,
    library_database_path: PathBuf,
    git: git::GitRepository,
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
    version: &'static str,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "catalogo-api",
        version: env!("CARGO_PKG_VERSION"),
    })
}

fn project_root() -> Result<PathBuf, String> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| "Não foi possível localizar a pasta do catálogo.".to_owned())?
        .canonicalize()
        .map_err(|error| format!("Não foi possível resolver a pasta do catálogo: {error}"))
}

fn initialize_database(path: &Path) -> Result<(), String> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .map_err(|error| format!("Não foi possível criar a pasta do banco: {error}"))?;
    }
    let connection = rusqlite::Connection::open(path)
        .map_err(|error| format!("Não foi possível abrir o banco do catálogo: {error}"))?;
    connection
        .execute_batch(include_str!("../../../database/001_initial_schema.sql"))
        .map_err(|error| format!("Não foi possível inicializar o banco do catálogo: {error}"))?;
    Ok(())
}

fn default_library_database_path() -> Result<PathBuf, String> {
    let data_directory = env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .or_else(|| {
            env::var_os("USERPROFILE")
                .or_else(|| env::var_os("HOME"))
                .map(PathBuf::from)
                .map(|home| home.join(".local").join("share"))
        })
        .ok_or_else(|| "Não foi possível localizar a pasta de dados do usuário.".to_owned())?;
    Ok(data_directory
        .join("CatalogoMateriais")
        .join("biblioteca.sqlite3"))
}

fn initialize_library_database(path: &Path) -> Result<(), String> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent).map_err(|error| {
            format!("Não foi possível criar a pasta da biblioteca local: {error}")
        })?;
    }
    let connection = rusqlite::Connection::open(path)
        .map_err(|error| format!("Não foi possível abrir o banco da biblioteca local: {error}"))?;
    connection
        .execute_batch(include_str!("../../../database/002_library.sql"))
        .map_err(|error| format!("Não foi possível inicializar a biblioteca local: {error}"))?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                tracing_subscriber::EnvFilter::new("catalogo_api=info,tower_http=info")
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let port = env::var("PORT")
        .ok()
        .map(|value| value.parse::<u16>())
        .transpose()?
        .unwrap_or(8080);
    let project_root = project_root()?;
    let database_path = env::var_os("CATALOG_DB_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|| project_root.join("database").join("catalogo.sqlite3"));
    let library_database_path = match env::var_os("LIBRARY_DB_PATH") {
        Some(path) => PathBuf::from(path),
        None => default_library_database_path()?,
    };
    initialize_database(&database_path)?;
    initialize_library_database(&library_database_path)?;
    let git = git::GitRepository::discover(&project_root)?;
    let state = AppState {
        database_path,
        library_database_path,
        git,
    };

    let allowed_origins = [
        HeaderValue::from_static("http://127.0.0.1:5173"),
        HeaderValue::from_static("http://localhost:5173"),
    ];
    let cors = CorsLayer::new()
        .allow_origin(allowed_origins)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([header::ACCEPT, header::CONTENT_TYPE]);

    let app = Router::new()
        .route("/api/v1/health", get(health))
        .route("/api/v1/categories", get(catalog::list_categories))
        .route("/api/v1/products", get(catalog::list_products))
        .route("/api/v1/products/{id}", get(catalog::get_product))
        .route(
            "/api/v1/library/documents",
            get(library::list).post(library::create),
        )
        .route(
            "/api/v1/library/documents/{id}",
            get(library::get)
                .put(library::update)
                .delete(library::delete),
        )
        .route("/api/v1/hermes/status", get(hermes::status))
        .route("/api/v1/hermes/research", post(hermes::research))
        .route("/api/v1/git/status", get(git::get_status))
        .route("/api/v1/git/commit", post(git::create_commit))
        .with_state(state)
        .layer(DefaultBodyLimit::max(64 * 1024))
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let address = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(address).await?;

    tracing::info!(%address, "API iniciada");
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;

    #[tokio::test]
    async fn health_endpoint_returns_expected_json() {
        let app = Router::new().route("/api/v1/health", get(health));
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), axum::http::StatusCode::OK);

        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(body["status"], "ok");
        assert_eq!(body["service"], "catalogo-api");
    }
}
