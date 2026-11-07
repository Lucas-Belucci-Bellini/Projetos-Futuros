use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    path::{Component, Path, PathBuf},
    process::{Command, Output},
};

use crate::AppState;

const ALLOWED_ORIGINS: [&str; 2] = ["http://127.0.0.1:5173", "http://localhost:5173"];
const MAX_SELECTED_FILES: usize = 200;
const MAX_COMMIT_MESSAGE_LENGTH: usize = 120;

#[derive(Clone, Debug)]
pub struct GitRepository {
    root: PathBuf,
    scope: String,
}

#[derive(Debug, Serialize)]
pub struct GitFile {
    path: String,
    status: String,
}

#[derive(Debug, Serialize)]
pub struct GitStatus {
    scope: String,
    files: Vec<GitFile>,
}

#[derive(Debug, Deserialize)]
pub struct CommitRequest {
    files: Vec<String>,
    message: String,
    confirmed: bool,
}

#[derive(Debug, Serialize)]
pub struct CommitResponse {
    commit: String,
}

#[derive(Debug)]
pub struct GitError {
    status: StatusCode,
    code: &'static str,
    message: &'static str,
}

impl GitError {
    fn invalid(message: &'static str) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            code: "INVALID_GIT_REQUEST",
            message,
        }
    }

    fn unavailable() -> Self {
        Self {
            status: StatusCode::SERVICE_UNAVAILABLE,
            code: "GIT_UNAVAILABLE",
            message: "Não foi possível acessar o repositório Git local.",
        }
    }

    fn operation_failed() -> Self {
        Self {
            status: StatusCode::CONFLICT,
            code: "GIT_OPERATION_FAILED",
            message:
                "O Git não concluiu a operação. Atualize o status e confira a configuração local.",
        }
    }
}

impl IntoResponse for GitError {
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

impl GitRepository {
    pub fn discover(project_root: &Path) -> Result<Self, String> {
        let project_root = project_root
            .canonicalize()
            .map_err(|error| format!("Não foi possível resolver a pasta do catálogo: {error}"))?;
        let output = Command::new("git")
            .arg("-C")
            .arg(&project_root)
            .args(["rev-parse", "--show-toplevel"])
            .output()
            .map_err(|error| format!("Não foi possível executar git: {error}"))?;
        if !output.status.success() {
            return Err("A pasta do catálogo não está dentro de um repositório Git.".into());
        }

        let root = PathBuf::from(
            String::from_utf8(output.stdout)
                .map_err(|error| format!("O caminho do repositório não é UTF-8: {error}"))?
                .trim(),
        )
        .canonicalize()
        .map_err(|error| format!("Não foi possível resolver a raiz Git: {error}"))?;
        let relative_scope = project_root
            .strip_prefix(&root)
            .map_err(|_| "A pasta do catálogo não está dentro da raiz Git.")?;
        let scope = path_to_git_string(relative_scope);
        if scope.is_empty() || scope == "." {
            return Err("A pasta do catálogo precisa ser isolada dentro do repositório.".into());
        }

        Ok(Self { root, scope })
    }

    fn status(&self) -> Result<GitStatus, GitError> {
        let output = self.run_status()?;
        if !output.status.success() {
            tracing::error!(
                stderr = %String::from_utf8_lossy(&output.stderr),
                "git status falhou"
            );
            return Err(GitError::unavailable());
        }

        let files = parse_status(&output.stdout)?;
        Ok(GitStatus {
            scope: self.scope.clone(),
            files,
        })
    }

    fn commit(&self, request: CommitRequest) -> Result<CommitResponse, GitError> {
        if !request.confirmed {
            return Err(GitError::invalid(
                "Confirme explicitamente a criação do commit.",
            ));
        }
        let message = request.message.trim();
        if message.is_empty()
            || message.chars().count() > MAX_COMMIT_MESSAGE_LENGTH
            || message.chars().any(char::is_control)
        {
            return Err(GitError::invalid(
                "A mensagem deve ter de 1 a 120 caracteres e não pode conter controles.",
            ));
        }
        if request.files.is_empty() || request.files.len() > MAX_SELECTED_FILES {
            return Err(GitError::invalid(
                "Selecione entre 1 e 200 arquivos alterados.",
            ));
        }

        let mut selected = HashSet::new();
        for path in &request.files {
            if !self.is_in_scope(path) || !selected.insert(path) {
                return Err(GitError::invalid(
                    "A seleção contém caminho inválido ou arquivo repetido.",
                ));
            }
        }

        let status = self.status()?;
        let changed: HashSet<&str> = status.files.iter().map(|file| file.path.as_str()).collect();
        if request
            .files
            .iter()
            .any(|path| !changed.contains(path.as_str()))
        {
            return Err(GitError::invalid(
                "Um ou mais arquivos não estão mais alterados; atualize o status.",
            ));
        }

        let pathspecs: Vec<String> = request
            .files
            .iter()
            .map(|path| format!(":(literal){path}"))
            .collect();
        let add_result = self.run_git(["add", "--"], pathspecs.iter().map(String::as_str))?;
        if !add_result.status.success() {
            tracing::error!(
                stderr = %String::from_utf8_lossy(&add_result.stderr),
                "git add falhou"
            );
            return Err(GitError::operation_failed());
        }

        let mut args = vec!["commit", "--only", "-m", message, "--"];
        args.extend(pathspecs.iter().map(String::as_str));
        let output = self.run_git(args.iter().copied(), std::iter::empty())?;
        if !output.status.success() {
            tracing::error!(
                stderr = %String::from_utf8_lossy(&output.stderr),
                "git commit falhou"
            );
            return Err(GitError::operation_failed());
        }

        let output = self.run_git(["rev-parse", "--short", "HEAD"], std::iter::empty())?;
        let commit = String::from_utf8_lossy(&output.stdout).trim().to_owned();
        if !output.status.success() || commit.is_empty() {
            tracing::error!("Commit criado, mas não foi possível ler o identificador.");
            return Err(GitError::operation_failed());
        }

        Ok(CommitResponse { commit })
    }

    fn run_status(&self) -> Result<Output, GitError> {
        Command::new("git")
            .arg("-C")
            .arg(&self.root)
            .args([
                "status",
                "--porcelain=v1",
                "-z",
                "--no-renames",
                "--untracked-files=all",
                "--",
            ])
            .arg(&self.scope)
            .output()
            .map_err(|error| {
                tracing::error!(%error, "Não foi possível executar git status");
                GitError::unavailable()
            })
    }

    fn run_git<'a>(
        &self,
        args: impl IntoIterator<Item = &'a str>,
        extra: impl IntoIterator<Item = &'a str>,
    ) -> Result<Output, GitError> {
        let output = Command::new("git")
            .arg("-C")
            .arg(&self.root)
            .args(args)
            .args(extra)
            .output()
            .map_err(|error| {
                tracing::error!(%error, "Não foi possível executar Git");
                GitError::unavailable()
            })?;
        if !output.status.success() {
            tracing::error!(
                stderr = %String::from_utf8_lossy(&output.stderr),
                "Comando Git falhou"
            );
        }
        Ok(output)
    }

    fn is_in_scope(&self, path: &str) -> bool {
        let scoped_prefix = format!("{}/", self.scope);
        path.starts_with(&scoped_prefix)
            && !path.contains('\\')
            && !path.contains('\0')
            && Path::new(path)
                .components()
                .all(|part| matches!(part, Component::Normal(_)))
    }
}

fn path_to_git_string(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn parse_status(bytes: &[u8]) -> Result<Vec<GitFile>, GitError> {
    let mut files = Vec::new();
    for record in bytes
        .split(|byte| *byte == 0)
        .filter(|record| !record.is_empty())
    {
        if record.len() < 4 || record[2] != b' ' {
            tracing::error!("git status retornou registro em formato inesperado");
            return Err(GitError::unavailable());
        }
        let status = std::str::from_utf8(&record[..2]).map_err(|error| {
            tracing::error!(%error, "Status Git não é UTF-8");
            GitError::unavailable()
        })?;
        let path = std::str::from_utf8(&record[3..]).map_err(|error| {
            tracing::error!(%error, "Caminho Git não é UTF-8");
            GitError::unavailable()
        })?;
        files.push(GitFile {
            path: path.to_owned(),
            status: status.to_owned(),
        });
    }
    Ok(files)
}

pub async fn get_status(State(state): State<AppState>) -> Result<Json<GitStatus>, GitError> {
    let git = state.git.clone();
    tokio::task::spawn_blocking(move || git.status())
        .await
        .map_err(|error| {
            tracing::error!(%error, "Tarefa git status falhou");
            GitError::unavailable()
        })?
        .map(Json)
}

pub async fn create_commit(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<CommitRequest>,
) -> Result<Json<CommitResponse>, GitError> {
    let origin = headers
        .get(axum::http::header::ORIGIN)
        .and_then(|value| value.to_str().ok());
    if !origin.is_some_and(|value| ALLOWED_ORIGINS.contains(&value)) {
        return Err(GitError::invalid(
            "A origem da solicitação não é permitida para esta operação local.",
        ));
    }

    let git = state.git.clone();
    tokio::task::spawn_blocking(move || git.commit(request))
        .await
        .map_err(|error| {
            tracing::error!(%error, "Tarefa git commit falhou");
            GitError::unavailable()
        })?
        .map(Json)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, time::SystemTime};

    fn run_git(root: &Path, args: &[&str]) -> Output {
        Command::new("git")
            .arg("-C")
            .arg(root)
            .args(args)
            .output()
            .unwrap()
    }

    fn temporary_repository() -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("catalogo-git-test-{}-{nonce}", std::process::id()))
    }

    #[test]
    fn parses_porcelain_status_records() {
        let files = parse_status(b" M file one.txt\0?? new.txt\0").unwrap();
        assert_eq!(files.len(), 2);
        assert_eq!(files[0].path, "file one.txt");
        assert_eq!(files[0].status, " M");
        assert_eq!(files[1].status, "??");
    }

    #[test]
    fn rejects_paths_outside_repository_scope() {
        let repository = GitRepository {
            root: PathBuf::new(),
            scope: "projetos/catalogo".into(),
        };
        assert!(repository.is_in_scope("projetos/catalogo/apps/web/App.tsx"));
        assert!(!repository.is_in_scope("projetos/other/file.txt"));
        assert!(!repository.is_in_scope("projetos/catalogo/../other/file.txt"));
        assert!(!repository.is_in_scope("projetos/catalogo\\apps\\web"));
    }

    #[test]
    fn commit_includes_only_selected_catalog_file() {
        let root = temporary_repository();
        fs::create_dir_all(root.join("catalog")).unwrap();
        fs::create_dir_all(root.join("outside")).unwrap();
        fs::write(root.join("catalog/one.txt"), "old one").unwrap();
        fs::write(root.join("catalog/two.txt"), "old two").unwrap();
        fs::write(root.join("outside/keep.txt"), "old outside").unwrap();
        assert!(run_git(&root, &["init"]).status.success());
        assert!(run_git(&root, &["config", "user.name", "Catalog Test"])
            .status
            .success());
        assert!(run_git(
            &root,
            &["config", "user.email", "catalog-test@example.invalid"]
        )
        .status
        .success());
        assert!(run_git(&root, &["config", "commit.gpgsign", "false"])
            .status
            .success());
        assert!(run_git(&root, &["add", "--all"]).status.success());
        assert!(run_git(&root, &["commit", "-m", "initial"])
            .status
            .success());

        fs::write(root.join("catalog/one.txt"), "new one").unwrap();
        fs::write(root.join("catalog/two.txt"), "new two").unwrap();
        fs::write(root.join("outside/keep.txt"), "new outside").unwrap();
        assert!(run_git(&root, &["add", "--", "outside/keep.txt"])
            .status
            .success());

        let repository = GitRepository {
            root: root.clone(),
            scope: "catalog".into(),
        };
        let result = repository
            .commit(CommitRequest {
                files: vec!["catalog/one.txt".into()],
                message: "selected only".into(),
                confirmed: true,
            })
            .unwrap();
        assert!(!result.commit.is_empty());

        let committed = run_git(&root, &["show", "--format=", "--name-only", "HEAD"]);
        assert!(committed.status.success());
        assert_eq!(
            String::from_utf8_lossy(&committed.stdout).trim(),
            "catalog/one.txt"
        );
        let staged_outside = run_git(&root, &["diff", "--cached", "--name-only"]);
        assert_eq!(
            String::from_utf8_lossy(&staged_outside.stdout).trim(),
            "outside/keep.txt"
        );
        let remaining_catalog = repository.status().unwrap();
        assert_eq!(remaining_catalog.files.len(), 1);
        assert_eq!(remaining_catalog.files[0].path, "catalog/two.txt");

        fs::remove_dir_all(root).unwrap();
    }
}
