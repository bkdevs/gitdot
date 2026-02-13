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

pub mod create_repository;
pub mod get_repository_commits;
pub mod get_repository_file;
pub mod get_repository_file_commits;
pub mod get_repository_preview;
