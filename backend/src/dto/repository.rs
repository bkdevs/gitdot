mod create_repository;
mod get_repository_file;
mod get_repository_tree;

pub use create_repository::{CreateRepositoryServerRequest, CreateRepositoryServerResponse};
pub use get_repository_file::{GetRepositoryFileQuery, GetRepositoryFileServerResponse};
pub use get_repository_tree::{GetRepositoryTreeQuery, GetRepositoryTreeServerResponse};

fn default_ref() -> String {
    "HEAD".to_string()
}
