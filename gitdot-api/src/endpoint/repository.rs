pub mod create_repository;
pub mod delete_repository;
pub mod get_repository_blob;
pub mod get_repository_blobs;
pub mod get_repository_commit;
pub mod get_repository_commit_diff;
pub mod get_repository_commits;
pub mod get_repository_file_commits;
pub mod get_repository_paths;
pub mod get_repository_preview;

fn default_visibility() -> String {
    "public".to_string()
}

fn default_ref() -> String {
    "HEAD".to_string()
}

fn default_page() -> u32 {
    1
}

fn default_per_page() -> u32 {
    30
}
