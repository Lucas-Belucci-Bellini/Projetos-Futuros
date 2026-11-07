use axum::{
    extract::{Path, Query, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};

use crate::AppState;

const ALLOWED_ORIGINS: [&str; 2] = ["http://127.0.0.1:5173", "http://localhost:5173"];
const MAX_TITLE_LENGTH: usize = 160;
const MAX_CONTENT_LENGTH: usize = 12_000;
const MAX_TAGS_LENGTH: usize = 300;

#[derive(Debug, Deserialize)]
pub struct DocumentQuery {
    q: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DocumentInput {
    title: String,
    content: String,
    tags: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Document {
    id: i64,
    title: String,
    content: String,
    tags: String,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct DocumentSummary {
    id: i64,
    title: String,
    excerpt: String,
    tags: String,
    updated_at: String,
}

#[derive(Debug)]
pub struct LibraryError {
    status: StatusCode,
    code: &'static str,
    message: &'static str,
}

impl LibraryError {
    fn invalid() -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            code: "INVALID_DOCUMENT",
            message: "Confira o título, o conteúdo e as etiquetas do registro.",
        }
    }

    fn forbidden_origin() -> Self {
        Self {
            status: StatusCode::FORBIDDEN,
            code: "ORIGIN_NOT_ALLOWED",
            message: "A origem da solicitação não é permitida para esta operação local.",
        }
    }

    fn not_found() -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            code: "DOCUMENT_NOT_FOUND",
            message: "O registro não foi encontrado.",
        }
    }

    fn unavailable() -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            code: "LIBRARY_UNAVAILABLE",
            message: "Não foi possível acessar a biblioteca local.",
        }
    }
}

impl IntoResponse for LibraryError {
    fn into_response(self) -> Response {
        (
            self.status,
            Json(serde_json::json!({
                "error": {
                    "code": self.code,
                    "message": self.message,
                }
            })),
        )
            .into_response()
    }
}

fn verify_local_origin(headers: &HeaderMap, allow_missing: bool) -> Result<(), LibraryError> {
    let origin_is_allowed = match headers.get(header::ORIGIN) {
        Some(value) => value
            .to_str()
            .is_ok_and(|origin| ALLOWED_ORIGINS.contains(&origin)),
        None => allow_missing,
    };
    if origin_is_allowed {
        Ok(())
    } else {
        Err(LibraryError::forbidden_origin())
    }
}

fn validate_document(input: &DocumentInput) -> Result<(), LibraryError> {
    let tags = input.tags.as_deref().unwrap_or_default();
    if input.title.trim().is_empty()
        || input.title.chars().count() > MAX_TITLE_LENGTH
        || input.title.chars().any(char::is_control)
        || input.content.trim().is_empty()
        || input.content.chars().count() > MAX_CONTENT_LENGTH
        || tags.chars().count() > MAX_TAGS_LENGTH
        || tags.chars().any(char::is_control)
    {
        return Err(LibraryError::invalid());
    }
    Ok(())
}

fn connect(database_path: &std::path::Path) -> Result<Connection, LibraryError> {
    let connection = Connection::open(database_path).map_err(|error| {
        tracing::error!(%error, "Não foi possível abrir o banco da biblioteca");
        LibraryError::unavailable()
    })?;
    connection
        .busy_timeout(std::time::Duration::from_secs(5))
        .map_err(|error| {
            tracing::error!(%error, "Não foi possível configurar o banco da biblioteca");
            LibraryError::unavailable()
        })?;
    Ok(connection)
}

fn list_documents(
    database_path: &std::path::Path,
    query: Option<&str>,
) -> Result<Vec<DocumentSummary>, LibraryError> {
    let connection = connect(database_path)?;
    let query = query.map(str::trim).filter(|query| !query.is_empty());
    let mut statement = connection
        .prepare(
            "SELECT id, title, substr(content, 1, 240), tags, updated_at
             FROM library_documents
             WHERE ?1 IS NULL
                OR instr(lower(title), lower(?1)) > 0
                OR instr(lower(content), lower(?1)) > 0
                OR instr(lower(tags), lower(?1)) > 0
             ORDER BY CASE WHEN ?1 IS NOT NULL AND instr(lower(title), lower(?1)) > 0
                           THEN 0 ELSE 1 END,
                      updated_at DESC
             LIMIT 100",
        )
        .map_err(|error| {
            tracing::error!(%error, "Não foi possível preparar a busca da biblioteca");
            LibraryError::unavailable()
        })?;
    let rows = statement
        .query_map([query], |row| {
            Ok(DocumentSummary {
                id: row.get(0)?,
                title: row.get(1)?,
                excerpt: row.get(2)?,
                tags: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })
        .map_err(|error| {
            tracing::error!(%error, "Não foi possível pesquisar a biblioteca");
            LibraryError::unavailable()
        })?;
    rows.collect::<Result<Vec<_>, _>>().map_err(|error| {
        tracing::error!(%error, "Não foi possível ler resultados da biblioteca");
        LibraryError::unavailable()
    })
}

fn get_document(database_path: &std::path::Path, id: i64) -> Result<Document, LibraryError> {
    let connection = connect(database_path)?;
    connection
        .query_row(
            "SELECT id, title, content, tags, created_at, updated_at
             FROM library_documents WHERE id = ?1",
            [id],
            |row| {
                Ok(Document {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    content: row.get(2)?,
                    tags: row.get(3)?,
                    created_at: row.get(4)?,
                    updated_at: row.get(5)?,
                })
            },
        )
        .optional()
        .map_err(|error| {
            tracing::error!(%error, "Não foi possível abrir o registro da biblioteca");
            LibraryError::unavailable()
        })?
        .ok_or_else(LibraryError::not_found)
}

fn save_document(
    database_path: &std::path::Path,
    input: DocumentInput,
    id: Option<i64>,
) -> Result<Document, LibraryError> {
    validate_document(&input)?;
    let connection = connect(database_path)?;
    let tags = input.tags.unwrap_or_default().trim().to_owned();

    if let Some(id) = id {
        let changed = connection
            .execute(
                "UPDATE library_documents
                 SET title = ?1, content = ?2, tags = ?3, updated_at = CURRENT_TIMESTAMP
                 WHERE id = ?4",
                params![input.title.trim(), input.content, tags, id],
            )
            .map_err(|error| {
                tracing::error!(%error, "Não foi possível atualizar o registro da biblioteca");
                LibraryError::unavailable()
            })?;
        if changed == 0 {
            return Err(LibraryError::not_found());
        }
        get_document(database_path, id)
    } else {
        connection
            .execute(
                "INSERT INTO library_documents (title, content, tags)
                 VALUES (?1, ?2, ?3)",
                params![input.title.trim(), input.content, tags],
            )
            .map_err(|error| {
                tracing::error!(%error, "Não foi possível salvar o registro da biblioteca");
                LibraryError::unavailable()
            })?;
        get_document(database_path, connection.last_insert_rowid())
    }
}

fn delete_document(database_path: &std::path::Path, id: i64) -> Result<(), LibraryError> {
    let connection = connect(database_path)?;
    let changed = connection
        .execute("DELETE FROM library_documents WHERE id = ?1", [id])
        .map_err(|error| {
            tracing::error!(%error, "Não foi possível excluir o registro da biblioteca");
            LibraryError::unavailable()
        })?;
    if changed == 0 {
        return Err(LibraryError::not_found());
    }
    Ok(())
}

pub async fn list(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<DocumentQuery>,
) -> Result<Json<Vec<DocumentSummary>>, LibraryError> {
    verify_local_origin(&headers, true)?;
    if query
        .q
        .as_deref()
        .is_some_and(|query| query.chars().count() > MAX_TITLE_LENGTH)
    {
        return Err(LibraryError::invalid());
    }
    let database_path = state.library_database_path.clone();
    let query = query.q;
    tokio::task::spawn_blocking(move || list_documents(&database_path, query.as_deref()))
        .await
        .map_err(|error| {
            tracing::error!(%error, "Tarefa de busca da biblioteca falhou");
            LibraryError::unavailable()
        })?
        .map(Json)
}

pub async fn get(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i64>,
) -> Result<Json<Document>, LibraryError> {
    verify_local_origin(&headers, true)?;
    let database_path = state.library_database_path.clone();
    tokio::task::spawn_blocking(move || get_document(&database_path, id))
        .await
        .map_err(|error| {
            tracing::error!(%error, "Tarefa de leitura da biblioteca falhou");
            LibraryError::unavailable()
        })?
        .map(Json)
}

pub async fn create(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(input): Json<DocumentInput>,
) -> Result<Json<Document>, LibraryError> {
    verify_local_origin(&headers, false)?;
    let database_path = state.library_database_path.clone();
    tokio::task::spawn_blocking(move || save_document(&database_path, input, None))
        .await
        .map_err(|error| {
            tracing::error!(%error, "Tarefa de criação da biblioteca falhou");
            LibraryError::unavailable()
        })?
        .map(Json)
}

pub async fn update(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i64>,
    Json(input): Json<DocumentInput>,
) -> Result<Json<Document>, LibraryError> {
    verify_local_origin(&headers, false)?;
    let database_path = state.library_database_path.clone();
    tokio::task::spawn_blocking(move || save_document(&database_path, input, Some(id)))
        .await
        .map_err(|error| {
            tracing::error!(%error, "Tarefa de atualização da biblioteca falhou");
            LibraryError::unavailable()
        })?
        .map(Json)
}

pub async fn delete(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i64>,
) -> Result<StatusCode, LibraryError> {
    verify_local_origin(&headers, false)?;
    let database_path = state.library_database_path.clone();
    tokio::task::spawn_blocking(move || delete_document(&database_path, id))
        .await
        .map_err(|error| {
            tracing::error!(%error, "Tarefa de exclusão da biblioteca falhou");
            LibraryError::unavailable()
        })??;
    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, time::SystemTime};

    fn temporary_database() -> std::path::PathBuf {
        let nonce = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "catalogo-library-test-{}-{nonce}.sqlite3",
            std::process::id()
        ));
        let connection = Connection::open(&path).unwrap();
        connection
            .execute_batch(include_str!("../../../database/002_library.sql"))
            .unwrap();
        path
    }

    fn sample_document(title: &str, content: &str) -> DocumentInput {
        DocumentInput {
            title: title.to_owned(),
            content: content.to_owned(),
            tags: Some("obra, orçamento".to_owned()),
        }
    }

    #[test]
    fn document_can_be_created_found_updated_and_deleted() {
        let path = temporary_database();
        let created =
            save_document(&path, sample_document("Notas da obra", "cimento"), None).unwrap();
        assert_eq!(created.title, "Notas da obra");

        let found = list_documents(&path, Some("cimento")).unwrap();
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].id, created.id);

        let updated = save_document(
            &path,
            sample_document("Notas revisadas", "argamassa"),
            Some(created.id),
        )
        .unwrap();
        assert_eq!(updated.title, "Notas revisadas");
        assert!(list_documents(&path, Some("cimento")).unwrap().is_empty());

        delete_document(&path, created.id).unwrap();
        assert!(list_documents(&path, None).unwrap().is_empty());
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn document_validation_rejects_empty_and_oversized_content() {
        let path = temporary_database();
        assert!(save_document(&path, sample_document(" ", "conteúdo"), None).is_err());
        assert!(save_document(
            &path,
            sample_document("Título", &"x".repeat(MAX_CONTENT_LENGTH + 1)),
            None
        )
        .is_err());
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn library_rejects_requests_from_other_origins() {
        let mut headers = HeaderMap::new();
        headers.insert(header::ORIGIN, "https://example.com".parse().unwrap());
        assert!(verify_local_origin(&headers, false).is_err());

        headers.insert(header::ORIGIN, "http://localhost:5173".parse().unwrap());
        assert!(verify_local_origin(&headers, false).is_ok());
        headers.remove(header::ORIGIN);
        assert!(verify_local_origin(&headers, true).is_ok());
    }
}
