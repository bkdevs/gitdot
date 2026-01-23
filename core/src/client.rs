mod git_client;
mod git_http_backend_client;

pub use git_client::{Git2Client, GitClient};
pub use git_http_backend_client::{GitHttpBackendClient, GitHttpBackendClientImpl};
