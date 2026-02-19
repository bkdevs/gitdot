mod diff;
mod git;
mod git_http;
mod github;

pub use diff::{DiffClient, DifftClient};
pub use git::{Git2Client, GitClient};
pub use git_http::{GitHttpClient, GitHttpClientImpl};
pub use github::{GitHubClient, GitHubClientImpl};
