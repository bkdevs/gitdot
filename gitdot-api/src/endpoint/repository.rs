pub mod create_repository;
pub mod create_repository_commit_filter;
pub mod delete_repository;
pub mod delete_repository_commit_filter;
pub mod get_repository;
pub mod get_repository_activity;
pub mod get_repository_blob;
pub mod get_repository_blobs;
pub mod get_repository_commit;
pub mod get_repository_commit_blobs;
pub mod get_repository_paths;
pub mod get_repository_resources;
pub mod list_latest_repositories;
pub mod list_repository_commit_filters;
pub mod list_repository_commits;
pub mod list_trending_repositories;
pub mod star_repository;
pub mod unstar_repository;
pub mod update_repository;
pub mod update_repository_commit_filter;

fn default_visibility() -> String {
    "public".to_string()
}

fn default_ref() -> String {
    "HEAD".to_string()
}
