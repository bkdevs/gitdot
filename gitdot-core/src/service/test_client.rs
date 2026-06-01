use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use async_trait::async_trait;
use bytes::Bytes;
use chrono::{DateTime, Utc};
use mockall::mock;
use serde::{Serialize, de::DeserializeOwned};
use uuid::Uuid;

use crate::{
    client::{GitClient, RedisClient, TokenClient},
    dto::{
        CommitDiffResponse, InitialCommitFile, RepositoryBlobResponse, RepositoryCommitResponse,
        RepositoryPathsResponse,
    },
    error::{GitError, RedisError, TokenError},
    model::TokenType,
    util::git::GitHookType,
};

mock! {
    pub EmailClient {}
    impl Clone for EmailClient {
        fn clone(&self) -> Self;
    }
    #[async_trait]
    impl crate::client::EmailClient for EmailClient {
        async fn send_email(&self, from: &str, to: &str, subject: &str, html: &str) -> Result<(), crate::error::EmailError>;
    }
}

mock! {
    pub GitHubClient {}
    impl Clone for GitHubClient {
        fn clone(&self) -> Self;
    }
    #[async_trait]
    impl crate::client::GitHubClient for GitHubClient {
        fn get_authorization_url(&self, state: &str) -> String;
        async fn exchange_code(&self, code: &str) -> Result<String, crate::error::GitHubError>;
        async fn get_user(&self, access_token: &str) -> Result<crate::dto::GitHubUser, crate::error::GitHubError>;
        async fn get_user_emails(&self, access_token: &str) -> Result<Vec<crate::dto::GitHubEmail>, crate::error::GitHubError>;
        async fn get_user_membership(&self, org_name: &str, user_name: &str, access_token: &str) -> Result<crate::dto::GitHubMembership, crate::error::GitHubError>;
        fn get_github_app_install_url(&self, user_id: Uuid, action: crate::dto::GitHubAppInstallAction) -> Result<String, crate::error::GitHubError>;
        fn verify_install_state(&self, token: &str) -> Result<crate::dto::InstallStatePayload, crate::error::GitHubError>;
        async fn get_installation(&self, installation_id: u64) -> Result<octocrab::models::Installation, crate::error::GitHubError>;
        async fn get_installation_access_token(&self, installation_id: u64) -> Result<String, crate::error::GitHubError>;
        async fn list_installation_repositories(&self, installation_id: u64) -> Result<octocrab::models::InstallationRepositories, crate::error::GitHubError>;
        fn verify_webhook_signature(&self, body: &[u8], signature_header: &str) -> Result<(), crate::error::GitHubError>;
    }
}

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
        _paths: &[String],
        _refs: &[String],
    ) -> Result<Vec<Option<RepositoryBlobResponse>>, GitError> {
        unimplemented!("MockGitClient::get_repo_blobs is not stubbed")
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
    async fn get_repo_commit_diff(
        &self,
        _owner: &str,
        _repo: &str,
        _left_ref: Option<&str>,
        _right_ref: &str,
    ) -> Result<Vec<CommitDiffResponse>, GitError> {
        unimplemented!("MockGitClient::get_repo_commit_diff is not stubbed")
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

/// Hand-written because [`RedisClient`]'s `get` / `set_with_ttl` /
/// `set_nx_with_ttl` are generic methods, which `mockall` can't generate.
/// Backed by an in-memory JSON map so `SET NX` claim-once semantics behave
/// realistically for the refresh-token rotation tests. Build with
/// [`failing`](MockRedisClient::failing) to make every operation error, or
/// [`seed`](MockRedisClient::seed) to pre-populate a key.
#[derive(Clone, Default)]
pub struct MockRedisClient {
    store: Arc<Mutex<HashMap<String, String>>>,
    fail: bool,
    phantom_nx_conflict: bool,
}

impl MockRedisClient {
    pub fn failing() -> Self {
        Self {
            fail: true,
            ..Default::default()
        }
    }

    /// Simulates the Redis-eviction race the real client guards against: a
    /// `SET NX` reports the key is already claimed, yet a follow-up `GET` finds
    /// nothing (the replacement was evicted in between). The in-memory store
    /// can't reproduce this atomically, so this flag forces it.
    pub fn phantom_nx_conflict() -> Self {
        Self {
            phantom_nx_conflict: true,
            ..Default::default()
        }
    }

    pub fn seed<T: Serialize>(self, key: &str, value: &T) -> Self {
        self.store
            .lock()
            .unwrap()
            .insert(key.to_string(), serde_json::to_string(value).unwrap());
        self
    }

    pub fn contains(&self, key: &str) -> bool {
        self.store.lock().unwrap().contains_key(key)
    }
}

/// Fabricates a [`RedisError`] for the failure-path tests without needing a live
/// connection.
fn redis_error() -> RedisError {
    RedisError::Serialization(serde_json::from_str::<i32>("x").unwrap_err())
}

#[async_trait]
impl RedisClient for MockRedisClient {
    async fn get<T: DeserializeOwned + Send>(&self, key: &str) -> Result<Option<T>, RedisError> {
        if self.fail {
            return Err(redis_error());
        }
        if self.phantom_nx_conflict {
            return Ok(None);
        }
        match self.store.lock().unwrap().get(key) {
            Some(raw) => Ok(Some(serde_json::from_str(raw)?)),
            None => Ok(None),
        }
    }

    async fn set_with_ttl<T: Serialize + Send + Sync>(
        &self,
        key: &str,
        value: &T,
        _ttl: Duration,
    ) -> Result<(), RedisError> {
        if self.fail {
            return Err(redis_error());
        }
        self.store
            .lock()
            .unwrap()
            .insert(key.to_string(), serde_json::to_string(value)?);
        Ok(())
    }

    async fn set_nx_with_ttl<T: Serialize + Send + Sync>(
        &self,
        key: &str,
        value: &T,
        _ttl: Duration,
    ) -> Result<bool, RedisError> {
        if self.fail {
            return Err(redis_error());
        }
        if self.phantom_nx_conflict {
            return Ok(false);
        }
        let mut store = self.store.lock().unwrap();
        if store.contains_key(key) {
            return Ok(false);
        }
        store.insert(key.to_string(), serde_json::to_string(value)?);
        Ok(true)
    }

    async fn delete(&self, key: &str) -> Result<(), RedisError> {
        if self.fail {
            return Err(redis_error());
        }
        self.store.lock().unwrap().remove(key);
        Ok(())
    }

    async fn ping(&self) -> Result<(), RedisError> {
        Ok(())
    }
}

/// Hand-written because [`TokenClient::generate_jwt`] is generic, which
/// `mockall` can't generate. Returns fixed, predictable values so token
/// assertions are deterministic. [`with_invalid_oauth_state`] flips
/// `verify_oauth_state` to `Err`.
#[derive(Clone)]
pub struct MockTokenClient {
    oauth_state_valid: bool,
}

impl Default for MockTokenClient {
    fn default() -> Self {
        Self {
            oauth_state_valid: true,
        }
    }
}

impl MockTokenClient {
    pub fn with_invalid_oauth_state(mut self) -> Self {
        self.oauth_state_valid = false;
        self
    }
}

impl TokenClient for MockTokenClient {
    fn generate_high_entropic_code(&self) -> (String, String) {
        (
            "refresh-token-raw".to_string(),
            "refresh-token-hash".to_string(),
        )
    }

    fn generate_readable_code(&self) -> (String, String) {
        ("ABC234".to_string(), "auth-code-hash".to_string())
    }

    fn get_auth_code_expiry_in_seconds(&self) -> u64 {
        600
    }

    fn get_access_token_expiry_in_seconds(&self) -> u64 {
        3600
    }

    fn get_refresh_token_expiry_in_seconds(&self) -> u64 {
        2_592_000
    }

    fn get_device_code_expiry_in_seconds(&self) -> u64 {
        600
    }

    fn get_polling_interval_in_seconds(&self) -> u64 {
        1
    }

    fn generate_access_token(&self, _token_type: &TokenType) -> (String, String) {
        (
            "access-token-raw".to_string(),
            "access-token-hash".to_string(),
        )
    }

    fn validate_token_format(&self, _token: &str) -> bool {
        true
    }

    fn generate_oauth_state(&self) -> String {
        "oauth-state".to_string()
    }

    fn verify_oauth_state(&self, _state: &str) -> Result<(), String> {
        if self.oauth_state_valid {
            Ok(())
        } else {
            Err("invalid state".to_string())
        }
    }

    fn generate_jwt<T: Serialize + Send + Sync>(&self, _claims: &T) -> Result<String, TokenError> {
        Ok("jwt".to_string())
    }

    fn generate_gitdot_jwt(&self, _user_id: Uuid, _username: &str) -> Result<String, TokenError> {
        Ok("gitdot-jwt".to_string())
    }
}
