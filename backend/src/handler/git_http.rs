mod git_info_refs;
mod git_receive_pack;
mod git_upload_pack;

use axum::{
    Router,
    routing::{get, post},
};

use crate::app::AppState;

use git_info_refs::git_info_refs;
use git_receive_pack::git_receive_pack;
use git_upload_pack::git_upload_pack;

pub fn create_git_http_router() -> Router<AppState> {
    Router::new()
        .route("/{owner}/{repo}/info/refs", get(git_info_refs))
        .route("/{owner}/{repo}/git-upload-pack", post(git_upload_pack))
        .route("/{owner}/{repo}/git-receive-pack", post(git_receive_pack))
}
