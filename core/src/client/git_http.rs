use std::process::Stdio;

use async_trait::async_trait;
use futures::{StreamExt, stream};
use tokio::io::{AsyncRead, AsyncReadExt};
use tokio::process::Command;

use crate::dto::{GitHttpBody, GitHttpResponse};
use crate::error::GitHttpError;
use crate::util::git::REPO_SUFFIX;

#[async_trait]
pub trait GitHttpClient: Send + Sync + Clone + 'static {
    async fn info_refs(
        &self,
        owner: &str,
        repo: &str,
        service: &str,
    ) -> Result<GitHttpResponse, GitHttpError>;

    async fn service_rpc(
        &self,
        owner: &str,
        repo: &str,
        service: &str,
        content_type: &str,
        body: Box<dyn AsyncRead + Unpin + Send>,
    ) -> Result<GitHttpResponse, GitHttpError>;

    fn normalize_repo_name(&self, repo_name: &str) -> String {
        format!(
            "{}{}",
            repo_name.strip_suffix(REPO_SUFFIX).unwrap_or(repo_name),
            REPO_SUFFIX
        )
    }
}

#[derive(Debug, Clone)]
pub struct GitHttpClientImpl {
    project_root: String,
}

impl GitHttpClientImpl {
    pub fn new(project_root: String) -> Self {
        Self { project_root }
    }

    fn parse_cgi_headers(
        header_bytes: &[u8],
    ) -> Result<(u16, Vec<(String, String)>), GitHttpError> {
        let headers_str = String::from_utf8_lossy(header_bytes);
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

        Ok((status_code, headers))
    }

    fn find_header_separator(data: &[u8]) -> Option<(usize, usize)> {
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

    /// Reads from stdout until the CGI header separator is found.
    /// Returns the position and length of the separator within the buffer.
    async fn read_until_cgi_separator(
        stdout: &mut tokio::process::ChildStdout,
        buf: &mut Vec<u8>,
    ) -> Result<(usize, usize), GitHttpError> {
        let mut tmp = [0u8; 4096];
        loop {
            let n = stdout
                .read(&mut tmp)
                .await
                .map_err(GitHttpError::ReadError)?;
            if n == 0 {
                return Err(GitHttpError::InvalidCgiResponse(
                    "EOF before CGI header separator".to_string(),
                ));
            }
            buf.extend_from_slice(&tmp[..n]);

            if let Some(sep) = Self::find_header_separator(buf) {
                return Ok(sep);
            }
        }
    }

    fn parse_cgi_response(output: Vec<u8>) -> Result<GitHttpResponse, GitHttpError> {
        let sep = Self::find_header_separator(&output).ok_or_else(|| {
            GitHttpError::InvalidCgiResponse("Missing header/body separator".to_string())
        })?;

        let (header_section, rest) = output.split_at(sep.0);
        let body = &rest[sep.1..];
        let (status_code, headers) = Self::parse_cgi_headers(header_section)?;

        Ok(GitHttpResponse {
            status_code,
            headers,
            body: GitHttpBody::Buffered(body.to_vec()),
        })
    }
}

#[async_trait]
impl GitHttpClient for GitHttpClientImpl {
    async fn info_refs(
        &self,
        owner: &str,
        repo: &str,
        service: &str,
    ) -> Result<GitHttpResponse, GitHttpError> {
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
            .map_err(GitHttpError::SpawnError)?;

        let output = child
            .wait_with_output()
            .await
            .map_err(GitHttpError::ReadError)?;

        if !output.status.success() {
            return Err(GitHttpError::ProcessFailed {
                code: output.status.code().unwrap_or(-1),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            });
        }

        Self::parse_cgi_response(output.stdout)
    }

    async fn service_rpc(
        &self,
        owner: &str,
        repo: &str,
        service: &str,
        content_type: &str,
        mut body: Box<dyn AsyncRead + Unpin + Send>,
    ) -> Result<GitHttpResponse, GitHttpError> {
        let repo_name = self.normalize_repo_name(repo);

        let mut child = Command::new("git")
            .arg("http-backend")
            .env("REQUEST_METHOD", "POST")
            .env(
                "PATH_INFO",
                format!("/{}/{}/git-{}", owner, repo_name, service),
            )
            .env("CONTENT_TYPE", content_type)
            .env("GIT_PROJECT_ROOT", &self.project_root)
            .env("GIT_HTTP_EXPORT_ALL", "1")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(GitHttpError::SpawnError)?;

        let mut stdin = child.stdin.take().unwrap();
        let mut stdout = child.stdout.take().unwrap();
        let mut stderr = child.stderr.take().unwrap();

        // Spawn stdin copy in background so stdout reading isn't blocked
        tokio::spawn(async move {
            let _ = tokio::io::copy(&mut body, &mut stdin).await;
            drop(stdin);
        });

        // Spawn stderr reader in background
        tokio::spawn(async move {
            let mut buf = Vec::new();
            let _ = tokio::io::copy(&mut stderr, &mut buf).await;
        });

        // Read stdout until CGI header separator is found
        let mut header_buf = Vec::with_capacity(4096);
        let sep = Self::read_until_cgi_separator(&mut stdout, &mut header_buf).await?;

        let (status_code, headers) = Self::parse_cgi_headers(&header_buf[..sep.0])?;

        // Remaining bytes after the separator that were already buffered
        let leftover = header_buf[sep.0 + sep.1..].to_vec();

        // Build a stream: leftover chunk first, then read stdout in chunks.
        // The child process handle is moved into the stream so it stays alive
        // until the stream is fully consumed.
        let stdout_stream = stream::unfold((stdout, child), |(mut stdout, child)| async move {
            let mut buf = vec![0u8; 65536];
            match stdout.read(&mut buf).await {
                Ok(0) => {
                    // stdout closed — child process is done, drop child handle
                    drop(child);
                    None
                }
                Ok(n) => {
                    buf.truncate(n);
                    Some((Ok(buf), (stdout, child)))
                }
                Err(e) => Some((Err(e), (stdout, child))),
            }
        });

        let body_stream: GitHttpBody = if leftover.is_empty() {
            GitHttpBody::Stream(Box::pin(stdout_stream))
        } else {
            GitHttpBody::Stream(Box::pin(
                stream::once(async { Ok(leftover) }).chain(stdout_stream),
            ))
        };

        Ok(GitHttpResponse {
            status_code,
            headers,
            body: body_stream,
        })
    }
}
