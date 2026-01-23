mod git;
mod git_http;

pub use git::{Git2Client, GitClient};
pub use git_http::{GitHttpBackendClient, GitHttpBackendClientImpl};
