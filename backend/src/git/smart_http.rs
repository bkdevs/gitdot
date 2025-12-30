use crate::config::settings::Settings;
use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::Response,
};
use std::io::Write;
use std::process::{Command, Stdio};

#[derive(serde::Deserialize)]
pub struct InfoRefsQuery {
    service: String,
}

pub async fn git_info_refs(
    State(settings): State<Settings>,
    Path((owner, repo)): Path<(String, String)>,
    Query(params): Query<InfoRefsQuery>,
) -> Result<Response<Body>, StatusCode> {
    if params.service != "git-upload-pack" && params.service != "git-receive-pack" {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Strip .git suffix if present, or add it if not
    let repo_path = if repo.ends_with(".git") {
        repo.clone()
    } else {
        format!("{}.git", repo)
    };

    let child = Command::new("git")
        .arg("http-backend")
        .env("REQUEST_METHOD", "GET")
        .env("PATH_INFO", format!("/{}/{}/info/refs", owner, repo_path))
        .env("QUERY_STRING", format!("service={}", params.service))
        .env("GIT_PROJECT_ROOT", &settings.git_project_root)
        .env("GIT_HTTP_EXPORT_ALL", "1")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let output = child
        .wait_with_output()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if !output.status.success() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    parse_cgi_response(output.stdout)
}

pub async fn git_upload_pack(
    State(settings): State<Settings>,
    Path((owner, repo)): Path<(String, String)>,
    headers: HeaderMap,
    body: Body,
) -> Result<Response<Body>, StatusCode> {
    git_service_rpc(settings, owner, repo, "upload-pack", headers, body).await
}

pub async fn git_receive_pack(
    State(settings): State<Settings>,
    Path((owner, repo)): Path<(String, String)>,
    headers: HeaderMap,
    body: Body,
) -> Result<Response<Body>, StatusCode> {
    git_service_rpc(settings, owner, repo, "receive-pack", headers, body).await
}

async fn git_service_rpc(
    settings: Settings,
    owner: String,
    repo: String,
    service: &str,
    headers: HeaderMap,
    body: Body,
) -> Result<Response<Body>, StatusCode> {
    // Strip .git suffix if present, or add it if not
    let repo_path = if repo.ends_with(".git") {
        repo.clone()
    } else {
        format!("{}.git", repo)
    };

    let body_bytes = axum::body::to_bytes(body, usize::MAX)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let content_type = headers
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let mut child = Command::new("git")
        .arg("http-backend")
        .env("REQUEST_METHOD", "POST")
        .env(
            "PATH_INFO",
            format!("/{}/{}/git-{}", owner, repo_path, service),
        )
        .env("CONTENT_TYPE", content_type)
        .env("CONTENT_LENGTH", body_bytes.len().to_string())
        .env("GIT_PROJECT_ROOT", &settings.git_project_root)
        .env("GIT_HTTP_EXPORT_ALL", "1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(&body_bytes)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        drop(stdin);
    }
    let output = child
        .wait_with_output()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if !output.status.success() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    parse_cgi_response(output.stdout)
}

fn parse_cgi_response(cgi_output: Vec<u8>) -> Result<Response<Body>, StatusCode> {
    let mut headers_end = 0;
    for i in 0..cgi_output.len().saturating_sub(3) {
        if &cgi_output[i..i + 4] == b"\r\n\r\n" {
            headers_end = i + 4;
            break;
        }
    }
    if headers_end == 0 {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    let header_section = &cgi_output[..headers_end - 4];
    let body_section = &cgi_output[headers_end..];

    let mut response = Response::builder();
    let headers_str = String::from_utf8_lossy(header_section);
    for line in headers_str.lines() {
        if let Some((name, value)) = line.split_once(": ") {
            response = response.header(name, value);
        }
    }
    response
        .body(Body::from(body_section.to_vec()))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
