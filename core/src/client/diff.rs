use crate::dto::{RepositoryDiffResponse, RepositoryFileResponse};
use crate::error::GitError;

pub trait DiffClient: Send + Sync + Clone + 'static {
    fn diff_files(
        &self,
        left: Option<&RepositoryFileResponse>,
        right: Option<&RepositoryFileResponse>,
    ) -> Result<RepositoryDiffResponse, GitError>;
}
