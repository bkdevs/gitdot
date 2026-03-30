pub mod create_repository;
pub mod delete_repository;
pub mod get_repository_blob;
pub mod get_repository_blob_diffs;
pub mod get_repository_blobs;
pub mod get_repository_commit;
pub mod get_repository_commit_diff;
pub mod get_repository_commits;
pub mod get_repository_paths;
pub mod get_repository_resources;
pub mod get_repository_settings;
pub mod update_repository_settings;

fn default_visibility() -> String {
    "public".to_string()
}

fn default_ref() -> String {
    "HEAD".to_string()
}
