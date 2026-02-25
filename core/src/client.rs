mod diff;
mod git;
mod git_http;
mod github;
mod s2;

pub use diff::{DiffClient, DifftClient};
pub use git::{Git2Client, GitClient};
pub use git_http::{GitHttpClient, GitHttpClientImpl};
pub use github::{GitHubClient, OctocrabClient};
pub use s2::{S2Client, S2ClientImpl};
