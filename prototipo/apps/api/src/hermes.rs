use axum::{
    extract::Json,
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::process::Command;

const ALLOWED_ORIGINS: [&str; 2] = ["http://127.0.0.1:5173", "http://localhost:5173"];
const MAX_QUESTION_LENGTH: usize = 500;
const RESEARCH_TIMEOUT: Duration = Duration::from_secs(120);

#[derive(Debug, Deserialize)]
pub struct ResearchRequest {
    question: String,
}

#[derive(Debug, Serialize)]
pub struct HermesStatus {
    installed: bool,
    version: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ResearchResponse {
    answer: String,
}

#[derive(Debug)]
pub struct HermesError {
    status: StatusCode,
    code: &'static str,
    message: &'static str,
}

impl HermesError {
    fn forbidden_origin() -> Self {
        Self {
            status: StatusCode::FORBIDDEN,
            code: "ORIGIN_NOT_ALLOWED",
            message: "A origem da solicitação não é permitida para esta operação local.",
        }
    }

    fn not_installed() -> Self {
        Self {
            status: StatusCode::SERVICE_UNAVAILABLE,
            code: "HERMES_NOT_INSTALLED",
            message: "Hermes não foi localizado. Instale e configure o Hermes para pesquisar.",
        }
    }

    fn invalid_question() -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            code: "INVALID_QUESTION",
            message: "A pergunta deve ter entre 1 e 500 caracteres.",
        }
    }

    fn failed() -> Self {
        Self {
            status: StatusCode::BAD_GATEWAY,
            code: "HERMES_RESEARCH_FAILED",
            message: "Hermes não concluiu a pesquisa. Confira a configuração e tente novamente.",
        }
    }
}

impl IntoResponse for HermesError {
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

fn verify_local_origin(headers: &HeaderMap, allow_missing: bool) -> Result<(), HermesError> {
    let origin_is_allowed = match headers.get(header::ORIGIN) {
        Some(value) => value
            .to_str()
            .is_ok_and(|origin| ALLOWED_ORIGINS.contains(&origin)),
        None => allow_missing,
    };
    if origin_is_allowed {
        Ok(())
    } else {
        Err(HermesError::forbidden_origin())
    }
}

fn validate_question(question: &str) -> Result<&str, HermesError> {
    let question = question.trim();
    if question.is_empty() || question.chars().count() > MAX_QUESTION_LENGTH {
        return Err(HermesError::invalid_question());
    }
    Ok(question)
}

pub async fn status(headers: HeaderMap) -> Result<Json<HermesStatus>, HermesError> {
    verify_local_origin(&headers, true)?;
    let result = tokio::time::timeout(
        Duration::from_secs(15),
        Command::new("hermes")
            .arg("--version")
            .kill_on_drop(true)
            .output(),
    )
    .await;

    match result {
        Ok(Ok(output)) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout)
                .lines()
                .next()
                .unwrap_or_default()
                .trim()
                .to_owned();
            Ok(Json(HermesStatus {
                installed: true,
                version: (!version.is_empty()).then_some(version),
            }))
        }
        Ok(Err(error)) if error.kind() == std::io::ErrorKind::NotFound => Ok(Json(HermesStatus {
            installed: false,
            version: None,
        })),
        Ok(Err(error)) => {
            tracing::error!(%error, "Não foi possível verificar o Hermes");
            Err(HermesError::failed())
        }
        Ok(Ok(output)) => {
            tracing::warn!(
                stderr = %String::from_utf8_lossy(&output.stderr),
                "O Hermes retornou erro ao verificar a versão"
            );
            Ok(Json(HermesStatus {
                installed: false,
                version: None,
            }))
        }
        Err(_) => Err(HermesError::failed()),
    }
}

pub async fn research(
    headers: HeaderMap,
    Json(request): Json<ResearchRequest>,
) -> Result<Json<ResearchResponse>, HermesError> {
    verify_local_origin(&headers, false)?;
    let question = validate_question(&request.question)?;

    let prompt = format!(
        "Pesquise na web sobre: {question}\n\
         Responda em português com um resumo conciso e inclua URLs das fontes utilizadas. \
         Use somente ferramentas de pesquisa e leitura da web. Não use terminal, não altere \
         arquivos e não solicite acesso a documentos locais."
    );
    let output = tokio::time::timeout(
        RESEARCH_TIMEOUT,
        Command::new("hermes")
            .args([
                "chat",
                "-q",
                prompt.as_str(),
                "--toolsets",
                "web",
                "--max-turns",
                "3",
            ])
            .kill_on_drop(true)
            .output(),
    )
    .await
    .map_err(|_| HermesError::failed())?
    .map_err(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            HermesError::not_installed()
        } else {
            tracing::error!(%error, "Não foi possível iniciar a pesquisa do Hermes");
            HermesError::failed()
        }
    })?;

    if !output.status.success() {
        tracing::warn!(
            stderr = %String::from_utf8_lossy(&output.stderr),
            "A pesquisa do Hermes terminou com erro"
        );
        return Err(HermesError::failed());
    }

    let answer = String::from_utf8_lossy(&output.stdout)
        .chars()
        .take(12_000)
        .collect::<String>()
        .trim()
        .to_owned();
    if answer.is_empty() {
        return Err(HermesError::failed());
    }
    Ok(Json(ResearchResponse { answer }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn research_rejects_requests_from_other_origins() {
        let mut headers = HeaderMap::new();
        headers.insert(header::ORIGIN, "https://example.com".parse().unwrap());
        assert!(verify_local_origin(&headers, false).is_err());

        headers.insert(header::ORIGIN, "http://127.0.0.1:5173".parse().unwrap());
        assert!(verify_local_origin(&headers, false).is_ok());
        headers.remove(&header::ORIGIN);
        assert!(verify_local_origin(&headers, true).is_ok());
    }

    #[test]
    fn research_question_is_bounded_and_nonempty() {
        assert!(validate_question("   ").is_err());
        assert!(validate_question(&"x".repeat(MAX_QUESTION_LENGTH + 1)).is_err());
        assert_eq!(
            validate_question("  pesquisa local  ").unwrap(),
            "pesquisa local"
        );
    }
}
