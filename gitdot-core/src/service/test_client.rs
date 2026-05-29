use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use bytes::Bytes;
use chrono::{DateTime, Utc};
use mockall::mock;

use crate::{
    client::GitClient,
    dto::{
        InitialCommitFile, RepositoryBlobResponse, RepositoryBlobsResponse,
        RepositoryCommitResponse, RepositoryDiffFileResponse, RepositoryDiffStatResponse,
        RepositoryPathsResponse,
    },
    error::GitError,
    util::git::GitHookType,
};

mock! {
    pub ImageClient {}
    impl Clone for ImageClient {
        fn clone(&self) -> Self;
    }
    #[async_trait]
    impl crate::client::ImageClient for ImageClient {
        async fn convert_to_webp(&self, bytes: Bytes) -> Result<Bytes, crate::error::ImageError>;
        async fn generate_user_image(&self, email: &str) -> Result<Bytes, crate::error::ImageError>;
        async fn generate_org_image(&self, name: &str) -> Result<Bytes, crate::error::ImageError>;
    }
}

mock! {
    pub R2Client {}
    impl Clone for R2Client {
        fn clone(&self) -> Self;
    }
    #[async_trait]
    impl crate::client::R2Client for R2Client {
        async fn upload_object(&self, key: &str, body: Bytes) -> Result<(), crate::error::R2Error>;
    }
}

/// `mockall` can't generate a mock for this trait: several methods take
/// `Option<&str>`, which neither elides nor accepts an explicit lifetime through
/// `async_trait` + `mock!`.
#[derive(Clone, Default)]
pub struct MockGitClient {
    renames: Arc<Mutex<Vec<(String, String)>>>,
    repo_exists: bool,
    created_repos: Arc<Mutex<Vec<(String, String)>>>,
    deleted_repos: Arc<Mutex<Vec<(String, String)>>>,
}

impl MockGitClient {
    /// Sets what [`GitClient::repo_exists`] reports for every repo.
    pub fn with_repo_exists(mut self, exists: bool) -> Self {
        self.repo_exists = exists;
        self
    }

    pub fn renames(&self) -> Vec<(String, String)> {
        self.renames.lock().unwrap().clone()
    }

    pub fn created_repos(&self) -> Vec<(String, String)> {
        self.created_repos.lock().unwrap().clone()
    }

    pub fn deleted_repos(&self) -> Vec<(String, String)> {
        self.deleted_repos.lock().unwrap().clone()
    }
}

#[async_trait]
impl GitClient for MockGitClient {
    async fn rename_owner(&self, old_owner: &str, new_owner: &str) -> Result<(), GitError> {
        self.renames
            .lock()
            .unwrap()
            .push((old_owner.to_string(), new_owner.to_string()));
        Ok(())
    }

    async fn repo_exists(&self, _owner: &str, _repo: &str) -> bool {
        self.repo_exists
    }
    async fn create_repo(&self, owner: &str, repo: &str) -> Result<(), GitError> {
        self.created_repos
            .lock()
            .unwrap()
            .push((owner.to_string(), repo.to_string()));
        Ok(())
    }
    async fn delete_repo(&self, owner: &str, repo: &str) -> Result<(), GitError> {
        self.deleted_repos
            .lock()
            .unwrap()
            .push((owner.to_string(), repo.to_string()));
        Ok(())
    }
    async fn mirror_repo(&self, _owner: &str, _repo: &str, _url: &str) -> Result<(), GitError> {
        unimplemented!("MockGitClient::mirror_repo is not stubbed")
    }
    async fn get_default_ref(&self, _owner: &str, _repo: &str) -> Result<String, GitError> {
        unimplemented!("MockGitClient::get_default_ref is not stubbed")
    }
    async fn fetch_ref(
        &self,
        _owner: &str,
        _repo: &str,
        _url: &str,
        _ref_name: &str,
        _sha: &str,
    ) -> Result<(), GitError> {
        unimplemented!("MockGitClient::fetch_ref is not stubbed")
    }
    async fn create_ref(
        &self,
        _owner: &str,
        _repo: &str,
        _ref_name: &str,
        _sha: &str,
    ) -> Result<(), GitError> {
        unimplemented!("MockGitClient::create_ref is not stubbed")
    }
    async fn update_ref(
        &self,
        _owner: &str,
        _repo: &str,
        _ref_name: &str,
        _sha: &str,
    ) -> Result<(), GitError> {
        unimplemented!("MockGitClient::update_ref is not stubbed")
    }
    async fn get_repo_blob(
        &self,
        _owner: &str,
        _repo: &str,
        _ref_name: &str,
        _path: &str,
    ) -> Result<RepositoryBlobResponse, GitError> {
        unimplemented!("MockGitClient::get_repo_blob is not stubbed")
    }
    async fn get_repo_blobs(
        &self,
        _owner: &str,
        _repo: &str,
        _ref_name: &str,
        _paths: &[String],
    ) -> Result<RepositoryBlobsResponse, GitError> {
        unimplemented!("MockGitClient::get_repo_blobs is not stubbed")
    }
    async fn get_repo_blobs_at_ref(
        &self,
        _owner: &str,
        _repo: &str,
        _ref_name: Option<&str>,
        _paths: &[String],
    ) -> Result<Vec<Option<RepositoryBlobResponse>>, GitError> {
        unimplemented!("MockGitClient::get_repo_blob_responses_at_ref is not stubbed")
    }
    async fn get_repo_blob_at_refs(
        &self,
        _owner: &str,
        _repo: &str,
        _path: &str,
        _refs: &[String],
    ) -> Result<RepositoryBlobsResponse, GitError> {
        unimplemented!("MockGitClient::get_repo_blob_at_refs is not stubbed")
    }
    async fn get_repo_paths(
        &self,
        _owner: &str,
        _repo: &str,
        _ref_name: &str,
    ) -> Result<RepositoryPathsResponse, GitError> {
        unimplemented!("MockGitClient::get_repo_paths is not stubbed")
    }
    async fn get_repo_commit(
        &self,
        _owner: &str,
        _repo: &str,
        _ref_name: &str,
    ) -> Result<RepositoryCommitResponse, GitError> {
        unimplemented!("MockGitClient::get_repo_commit is not stubbed")
    }
    async fn get_repo_diff_files(
        &self,
        _owner: &str,
        _repo: &str,
        _left_ref: Option<&str>,
        _right_ref: &str,
    ) -> Result<Vec<RepositoryDiffFileResponse>, GitError> {
        unimplemented!("MockGitClient::get_repo_diff_files is not stubbed")
    }
    async fn get_repo_diff_stats(
        &self,
        _owner: &str,
        _repo: &str,
        _left_ref: Option<&str>,
        _right_ref: &str,
    ) -> Result<Vec<RepositoryDiffStatResponse>, GitError> {
        unimplemented!("MockGitClient::get_repo_diff_stats is not stubbed")
    }
    async fn rev_list(
        &self,
        _owner: &str,
        _repo: &str,
        _old_sha: &str,
        _new_sha: &str,
    ) -> Result<Vec<RepositoryCommitResponse>, GitError> {
        unimplemented!("MockGitClient::rev_list is not stubbed")
    }
    async fn resolve_ref_sha(
        &self,
        _owner: &str,
        _repo: &str,
        _ref_name: &str,
    ) -> Result<String, GitError> {
        unimplemented!("MockGitClient::resolve_ref_sha is not stubbed")
    }
    async fn get_commit_patch_id(
        &self,
        _owner: &str,
        _repo: &str,
        _sha: &str,
    ) -> Result<String, GitError> {
        unimplemented!("MockGitClient::get_commit_patch_id is not stubbed")
    }
    async fn cherry_pick_commit(
        &self,
        _owner: &str,
        _repo: &str,
        _commit_sha: &str,
        _new_parent_sha: &str,
    ) -> Result<String, GitError> {
        unimplemented!("MockGitClient::cherry_pick_commit is not stubbed")
    }
    async fn create_initial_commit(
        &self,
        _owner: &str,
        _repo: &str,
        _files: Vec<InitialCommitFile>,
        _author_name: &str,
        _author_email: &str,
        _committed_at: DateTime<Utc>,
    ) -> Result<String, GitError> {
        unimplemented!("MockGitClient::create_initial_commit is not stubbed")
    }
    async fn install_hook(
        &self,
        _owner: &str,
        _repo: &str,
        _hook_type: GitHookType,
        _hook_script: &str,
    ) -> Result<(), GitError> {
        Ok(())
    }
    async fn empty_hooks(&self, _owner: &str, _repo: &str) -> Result<(), GitError> {
        unimplemented!("MockGitClient::empty_hooks is not stubbed")
    }
}
