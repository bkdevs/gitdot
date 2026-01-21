use async_trait::async_trait;
use std::io::Write;
use std::process::{Command, Stdio};

use crate::dto::GitHttpBackendResponse;
use crate::errors::GitHttpBackendError;

const REPO_SUFFIX: &str = ".git";

#[async_trait]
pub trait GitHttpBackendClient: Send + Sync + Clone + 'static {
    async fn info_refs(
        &self,
        owner: &str,
        repo: &str,
        service: &str,
    ) -> Result<GitHttpBackendResponse, GitHttpBackendError>;

    async fn service_rpc(
        &self,
        owner: &str,
        repo: &str,
        service: &str,
        content_type: &str,
        body: &[u8],
    ) -> Result<GitHttpBackendResponse, GitHttpBackendError>;

    fn normalize_repo_name(&self, repo_name: &str) -> String {
        format!(
            "{}{}",
            repo_name.strip_suffix(REPO_SUFFIX).unwrap_or(repo_name),
            REPO_SUFFIX
        )
    }
}

#[derive(Debug, Clone)]
pub struct GitHttpBackendClientImpl {
    project_root: String,
}

impl GitHttpBackendClientImpl {
    pub fn new(project_root: String) -> Self {
        Self { project_root }
    }

    fn validate_service(&self, service: &str) -> Result<(), GitHttpBackendError> {
        match service {
            "git-upload-pack" | "git-receive-pack" => Ok(()),
            _ => Err(GitHttpBackendError::InvalidService(service.to_string())),
        }
    }

    fn parse_cgi_response(
        &self,
        output: Vec<u8>,
    ) -> Result<GitHttpBackendResponse, GitHttpBackendError> {
        let separator_pos = self.find_header_separator(&output).ok_or_else(|| {
            GitHttpBackendError::InvalidCgiResponse("Missing header/body separator".to_string())
        })?;

        let (header_section, rest) = output.split_at(separator_pos.0);
        let body = &rest[separator_pos.1..];

        let headers_str = String::from_utf8_lossy(header_section);
        let mut headers = Vec::new();
        let mut status_code = 200u16;

        for line in headers_str.lines() {
            if let Some((name, value)) = line.split_once(": ") {
                if name.eq_ignore_ascii_case("Status") {
                    if let Some(code_str) = value.split_whitespace().next() {
                        status_code = code_str.parse().unwrap_or(200);
                    }
                } else {
                    headers.push((name.to_string(), value.to_string()));
                }
            }
        }

        Ok(GitHttpBackendResponse {
            status_code,
            headers,
            body: body.to_vec(),
        })
    }

    fn find_header_separator(&self, data: &[u8]) -> Option<(usize, usize)> {
        // Look for \r\n\r\n first
        for i in 0..data.len().saturating_sub(3) {
            if &data[i..i + 4] == b"\r\n\r\n" {
                return Some((i, 4));
            }
        }
        // Fall back to \n\n
        for i in 0..data.len().saturating_sub(1) {
            if &data[i..i + 2] == b"\n\n" {
                return Some((i, 2));
            }
        }
        None
    }
}

#[async_trait]
impl GitHttpBackendClient for GitHttpBackendClientImpl {
    async fn info_refs(
        &self,
        owner: &str,
        repo: &str,
        service: &str,
    ) -> Result<GitHttpBackendResponse, GitHttpBackendError> {
        self.validate_service(service)?;
        let repo_name = self.normalize_repo_name(repo);

        let child = Command::new("git")
            .arg("http-backend")
            .env("REQUEST_METHOD", "GET")
            .env("PATH_INFO", format!("/{}/{}/info/refs", owner, repo_name))
            .env("QUERY_STRING", format!("service={}", service))
            .env("GIT_PROJECT_ROOT", &self.project_root)
            .env("GIT_HTTP_EXPORT_ALL", "1")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(GitHttpBackendError::SpawnError)?;

        let output = child
            .wait_with_output()
            .map_err(GitHttpBackendError::ReadError)?;

        if !output.status.success() {
            return Err(GitHttpBackendError::ProcessFailed {
                code: output.status.code().unwrap_or(-1),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            });
        }

        self.parse_cgi_response(output.stdout)
    }

    async fn service_rpc(
        &self,
        owner: &str,
        repo: &str,
        service: &str,
        content_type: &str,
        body: &[u8],
    ) -> Result<GitHttpBackendResponse, GitHttpBackendError> {
        let repo_name = self.normalize_repo_name(repo);

        let mut child = Command::new("git")
            .arg("http-backend")
            .env("REQUEST_METHOD", "POST")
            .env(
                "PATH_INFO",
                format!("/{}/{}/git-{}", owner, repo_name, service),
            )
            .env("CONTENT_TYPE", content_type)
            .env("CONTENT_LENGTH", body.len().to_string())
            .env("GIT_PROJECT_ROOT", &self.project_root)
            .env("GIT_HTTP_EXPORT_ALL", "1")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(GitHttpBackendError::SpawnError)?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin
                .write_all(body)
                .map_err(GitHttpBackendError::WriteError)?;
            drop(stdin);
        }

        let output = child
            .wait_with_output()
            .map_err(GitHttpBackendError::ReadError)?;

        if !output.status.success() {
            return Err(GitHttpBackendError::ProcessFailed {
                code: output.status.code().unwrap_or(-1),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            });
        }

        self.parse_cgi_response(output.stdout)
    }
}
