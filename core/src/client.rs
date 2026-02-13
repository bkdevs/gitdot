mod diff;
mod git;
mod git_http;

pub use diff::{DiffClient, DifftClient};
pub use git::{Git2Client, GitClient};
pub use git_http::{GitHttpClient, GitHttpClientImpl};
