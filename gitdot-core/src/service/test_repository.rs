use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use ipnetwork::IpNetwork;
use mockall::mock;
use uuid::Uuid;

use crate::{
    model::{
        Answer, AuthCode, AuthProvider, Comment, CommentSide, Commit, CommitDiff, Diff, DiffStatus,
        EmailVerificationCode, Organization, OrganizationMember, OrganizationRole, Question,
        Repository, RepositoryOwnerType, RepositoryStar, RepositoryVisibility, Review,
        ReviewComment, ReviewStatus, Reviewer, Revision, Session, User, UserEmail,
        UserOrganization, Verdict, VoteResult, VoteTarget,
    },
    repository::{AuthCodeVerification, EmailCodeVerification},
};

mock! {
    pub OrganizationRepository {}
    impl Clone for OrganizationRepository {
        fn clone(&self) -> Self;
    }
    #[async_trait]
    impl crate::repository::OrganizationRepository for OrganizationRepository {
        async fn create(&self, org_name: &str, owner_id: Uuid, readme: Option<String>) -> Result<Organization, crate::error::DatabaseError>;
        async fn get(&self, org_name: &str) -> Result<Option<Organization>, crate::error::DatabaseError>;
        async fn get_id(&self, org_name: &str) -> Result<Option<Uuid>, crate::error::DatabaseError>;
        async fn touch_image(&self, org_id: Uuid) -> Result<(), crate::error::DatabaseError>;
        async fn is_member(&self, org_id: Uuid, user_id: Uuid) -> Result<bool, crate::error::DatabaseError>;
        async fn add_member(&self, org_name: &str, user_name: &str, role: OrganizationRole, role_description: Option<String>) -> Result<Option<OrganizationMember>, crate::error::DatabaseError>;
        async fn get_member_role(&self, org_name: &str, user_id: Uuid) -> Result<Option<OrganizationRole>, crate::error::DatabaseError>;
        async fn get_member(&self, org_name: &str, member_id: Uuid) -> Result<Option<OrganizationMember>, crate::error::DatabaseError>;
        async fn update(&self, org_name: &str, location: Option<String>, readme: Option<String>, links: Option<Vec<String>>, display_name: Option<String>) -> Result<Option<Organization>, crate::error::DatabaseError>;
        async fn update_member(&self, org_name: &str, member_id: Uuid, role_description: Option<String>) -> Result<Option<OrganizationMember>, crate::error::DatabaseError>;
        async fn list(&self, cursor: Option<crate::dto::Cursor>, limit: i64) -> Result<(Vec<Organization>, Option<crate::dto::Cursor>), crate::error::DatabaseError>;
        async fn list_by_user_id(&self, user_id: Uuid) -> Result<Vec<Organization>, crate::error::DatabaseError>;
        async fn list_memberships_by_user_id(&self, user_id: Uuid, cursor: Option<crate::dto::Cursor>, limit: i64) -> Result<(Vec<UserOrganization>, Option<crate::dto::Cursor>), crate::error::DatabaseError>;
    }
}

mock! {
    pub RepositoryRepository {}
    impl Clone for RepositoryRepository {
        fn clone(&self) -> Self;
    }
    #[async_trait]
    impl crate::repository::RepositoryRepository for RepositoryRepository {
        async fn create(&self, name: &str, owner_id: Uuid, owner_type: &RepositoryOwnerType, visibility: &RepositoryVisibility, description: Option<String>, readonly: bool, created_at: Option<chrono::DateTime<chrono::Utc>>) -> Result<Repository, crate::error::DatabaseError>;
        async fn get(&self, owner: &str, repo: &str, viewer_id: Option<Uuid>) -> Result<Option<Repository>, crate::error::DatabaseError>;
        async fn get_id(&self, owner: &str, repo: &str) -> Result<Option<Uuid>, crate::error::DatabaseError>;
        async fn get_by_id(&self, id: Uuid, viewer_id: Option<Uuid>) -> Result<Option<Repository>, crate::error::DatabaseError>;
        async fn list_by_owner(&self, owner_name: &str, viewer_id: Option<Uuid>, cursor: Option<crate::dto::Cursor>, limit: i64) -> Result<(Vec<Repository>, Option<crate::dto::Cursor>), crate::error::DatabaseError>;
        async fn list_latest(&self, limit: i64) -> Result<Vec<Repository>, crate::error::DatabaseError>;
        async fn list_trending(&self, limit: i64) -> Result<Vec<Repository>, crate::error::DatabaseError>;
        async fn delete(&self, id: Uuid) -> Result<(), crate::error::DatabaseError>;
        async fn update(&self, id: Uuid, description: Option<String>, readonly: Option<bool>) -> Result<Option<Repository>, crate::error::DatabaseError>;
        async fn star(&self, id: Uuid, user_id: Uuid) -> Result<Option<RepositoryStar>, crate::error::DatabaseError>;
        async fn unstar(&self, id: Uuid, user_id: Uuid) -> Result<bool, crate::error::DatabaseError>;
        async fn list_recent_stars(&self, repository_id: Uuid, limit: i64) -> Result<Vec<(User, DateTime<Utc>)>, crate::error::DatabaseError>;
        async fn list_commit_filters(&self, repository_id: Uuid, cursor: Option<crate::dto::Cursor>, limit: i64) -> Result<(Vec<crate::model::CommitFilter>, Option<crate::dto::Cursor>), crate::error::DatabaseError>;
        async fn create_commit_filter(&self, repository_id: Uuid, name: &str, authors: Option<Vec<String>>, tags: Option<Vec<String>>, paths: Option<Vec<String>>) -> Result<crate::model::CommitFilter, crate::error::DatabaseError>;
        async fn update_commit_filter(&self, repository_id: Uuid, filter_id: Uuid, name: &str, authors: Option<Vec<String>>, tags: Option<Vec<String>>, paths: Option<Vec<String>>) -> Result<Option<crate::model::CommitFilter>, crate::error::DatabaseError>;
        async fn delete_commit_filter(&self, repository_id: Uuid, filter_id: Uuid) -> Result<bool, crate::error::DatabaseError>;
    }
}

mock! {
    pub QuestionRepository {}
    impl Clone for QuestionRepository {
        fn clone(&self) -> Self;
    }
    #[async_trait]
    impl crate::repository::QuestionRepository for QuestionRepository {
        async fn create_question(&self, author_id: Uuid, repository_id: Uuid, title: &str, body: &str) -> Result<Question, crate::error::DatabaseError>;
        async fn update_question(&self, repository_id: Uuid, number: i32, title: &str, body: &str) -> Result<Option<Question>, crate::error::DatabaseError>;
        async fn get_question(&self, repository_id: Uuid, number: i32, user_id: Option<Uuid>) -> Result<Option<Question>, crate::error::DatabaseError>;
        async fn get_question_id(&self, owner: &str, repo: &str, question_number: i32) -> Result<Option<Uuid>, crate::error::DatabaseError>;
        async fn list_questions(&self, repository_id: Uuid, user_id: Option<Uuid>, cursor: Option<crate::dto::Cursor>, limit: i64) -> Result<(Vec<Question>, Option<crate::dto::Cursor>), crate::error::DatabaseError>;
        async fn create_answer(&self, owner: &str, repo: &str, question_number: i32, author_id: Uuid, body: &str) -> Result<Option<Answer>, crate::error::DatabaseError>;
        async fn update_answer(&self, id: Uuid, body: &str) -> Result<Option<Answer>, crate::error::DatabaseError>;
        async fn create_comment(&self, parent_id: Uuid, author_id: Uuid, body: &str) -> Result<Comment, crate::error::DatabaseError>;
        async fn create_question_comment(&self, owner: &str, repo: &str, question_number: i32, author_id: Uuid, body: &str) -> Result<Option<Comment>, crate::error::DatabaseError>;
        async fn update_comment(&self, id: Uuid, body: &str) -> Result<Option<Comment>, crate::error::DatabaseError>;
        async fn get_question_author_id(&self, owner: &str, repo: &str, question_number: i32) -> Result<Option<Uuid>, crate::error::DatabaseError>;
        async fn get_answer_author_id(&self, id: Uuid) -> Result<Option<Uuid>, crate::error::DatabaseError>;
        async fn get_comment_author_id(&self, id: Uuid) -> Result<Option<Uuid>, crate::error::DatabaseError>;
        async fn vote(&self, user_id: Uuid, target_id: Uuid, target_type: VoteTarget, value: i16) -> Result<VoteResult, crate::error::DatabaseError>;
    }
}

mock! {
    pub UserRepository {}
    impl Clone for UserRepository {
        fn clone(&self) -> Self;
    }
    #[async_trait]
    impl crate::repository::UserRepository for UserRepository {
        async fn create(&self, email: &str, is_email_verified: bool, provider: AuthProvider) -> Result<User, crate::error::DatabaseError>;
        async fn get(&self, user_name: &str) -> Result<Option<User>, crate::error::DatabaseError>;
        async fn update(&self, id: Uuid, name: Option<String>, location: Option<String>, readme: Option<String>, links: Option<Vec<String>>, display_name: Option<String>) -> Result<User, crate::error::DatabaseError>;
        async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, crate::error::DatabaseError>;
        async fn mark_user_as_deleted(&self, id: Uuid) -> Result<(), crate::error::DatabaseError>;
        async fn touch_image(&self, id: Uuid) -> Result<(), crate::error::DatabaseError>;
        async fn get_by_primary_email(&self, email: &str) -> Result<Option<User>, crate::error::DatabaseError>;
        async fn get_by_emails(&self, emails: &[String]) -> Result<Vec<(String, Uuid)>, crate::error::DatabaseError>;
        async fn verify_email(&self, id: Uuid) -> Result<(), crate::error::DatabaseError>;
        async fn is_name_taken(&self, name: &str) -> Result<bool, crate::error::DatabaseError>;
        async fn is_email_taken(&self, email: &str) -> Result<bool, crate::error::DatabaseError>;
        async fn list_emails(&self, user_id: Uuid) -> Result<Vec<UserEmail>, crate::error::DatabaseError>;
        async fn upsert_verified_emails(&self, user_id: Uuid, emails: &[String]) -> Result<(), crate::error::DatabaseError>;
        async fn list_repositories(&self, user_id: Uuid, viewer_id: Option<Uuid>, cursor: Option<crate::dto::Cursor>, limit: i64) -> Result<(Vec<(Repository, Option<i64>, Option<chrono::DateTime<chrono::Utc>>)>, Option<crate::dto::Cursor>), crate::error::DatabaseError>;
        async fn list_starred_repositories(&self, user_id: Uuid, viewer_id: Option<Uuid>, cursor: Option<crate::dto::Cursor>, limit: i64) -> Result<(Vec<Repository>, Option<crate::dto::Cursor>), crate::error::DatabaseError>;
        async fn list_contributed_repositories(&self, user_id: Uuid, viewer_id: Option<Uuid>, since: chrono::DateTime<chrono::Utc>, cursor: Option<crate::dto::Cursor>, limit: i64) -> Result<(Vec<(Repository, i64, chrono::DateTime<chrono::Utc>)>, Option<crate::dto::Cursor>), crate::error::DatabaseError>;
    }
}

mock! {
    pub ReviewRepository {}
    impl Clone for ReviewRepository {
        fn clone(&self) -> Self;
    }
    #[async_trait]
    impl crate::repository::ReviewRepository for ReviewRepository {
        async fn get_review_by_number(&self, owner: &str, repo: &str, number: i32) -> Result<Option<Review>, crate::error::DatabaseError>;
        async fn list_reviews(&self, owner: &str, repo: &str, viewer_id: Option<Uuid>, cursor: Option<crate::dto::Cursor>, limit: i64) -> Result<(Vec<Review>, Option<crate::dto::Cursor>), crate::error::DatabaseError>;
        async fn list_reviews_by_user(&self, user_name: &str, viewer_id: Option<Uuid>, status: Option<String>, owner: Option<String>, repo: Option<String>, cursor: Option<crate::dto::Cursor>, limit: i64) -> Result<(Vec<Review>, Option<crate::dto::Cursor>), crate::error::DatabaseError>;
        async fn create_review(&self, repository_id: Uuid, author_id: Uuid, target_branch: &str) -> Result<Review, crate::error::DatabaseError>;
        async fn update_review(&self, review_id: Uuid, status: Option<ReviewStatus>, title: Option<String>, description: Option<String>) -> Result<(), crate::error::DatabaseError>;
        async fn create_diff(&self, review_id: Uuid, position: i32, message: &str) -> Result<Diff, crate::error::DatabaseError>;
        async fn update_diff(&self, diff_id: Uuid, status: Option<DiffStatus>, message: Option<String>) -> Result<(), crate::error::DatabaseError>;
        async fn create_revision(&self, diff_id: Uuid, number: i32, commit_hash: &str, parent_hash: &str) -> Result<Revision, crate::error::DatabaseError>;
        async fn update_revision_sha(&self, revision_id: Uuid, commit_hash: &str, parent_hash: &str) -> Result<(), crate::error::DatabaseError>;
        async fn add_reviewer(&self, review_id: Uuid, reviewer_id: Uuid) -> Result<Option<Reviewer>, crate::error::DatabaseError>;
        async fn remove_reviewer(&self, review_id: Uuid, reviewer_id: Uuid) -> Result<bool, crate::error::DatabaseError>;
        async fn create_verdict(&self, diff_id: Uuid, revision_id: Uuid, reviewer_id: Uuid, verdict: Verdict) -> Result<(), crate::error::DatabaseError>;
        async fn create_comment(&self, review_id: Uuid, diff_id: Uuid, revision_id: Uuid, author_id: Uuid, body: &str, parent_id: Option<Uuid>, file_path: Option<String>, line_number_start: Option<i32>, line_number_end: Option<i32>, start_character: Option<i32>, end_character: Option<i32>, side: Option<CommentSide>) -> Result<ReviewComment, crate::error::DatabaseError>;
        async fn get_comment(&self, comment_id: Uuid) -> Result<Option<ReviewComment>, crate::error::DatabaseError>;
        async fn update_comment(&self, comment_id: Uuid, body: &str) -> Result<ReviewComment, crate::error::DatabaseError>;
        async fn resolve_comment(&self, comment_id: Uuid, resolved: bool) -> Result<(), crate::error::DatabaseError>;
    }
}

mock! {
    pub CommitRepository {}
    impl Clone for CommitRepository {
        fn clone(&self) -> Self;
    }
    #[async_trait]
    impl crate::repository::CommitRepository for CommitRepository {
        async fn get_commit(&self, repo_id: Uuid, sha: &str) -> Result<Option<Commit>, crate::error::DatabaseError>;
        async fn list_by_repository(&self, repo_id: Uuid, ref_name: &str, from: DateTime<Utc>, to: DateTime<Utc>, cursor: Option<crate::dto::Cursor>, limit: i64) -> Result<(Vec<Commit>, Option<crate::dto::Cursor>), crate::error::DatabaseError>;
        async fn list_by_user(&self, author_id: Uuid, viewer_id: Option<Uuid>, from: DateTime<Utc>, to: DateTime<Utc>, cursor: Option<crate::dto::Cursor>, limit: i64) -> Result<(Vec<(Commit, bool)>, Option<crate::dto::Cursor>), crate::error::DatabaseError>;
        async fn create_bulk(&self, author_ids: &[Option<Uuid>], git_author_names: &[String], git_author_emails: &[String], repo_ids: &[Uuid], ref_names: &[String], shas: &[String], parent_shas: &[String], messages: &[String], created_ats: &[DateTime<Utc>], diffs: &[Vec<CommitDiff>], review_numbers: &[Option<i32>], diff_positions: &[Option<i32>]) -> Result<Vec<Commit>, crate::error::DatabaseError>;
    }
}

/// Hand-written because [`SessionRepository::create_session`] takes
/// `Option<&str>`, which `mockall` + `async_trait` can't generate a mock for
/// (the same limitation that forces [`super::test_client::MockGitClient`]).
///
/// Configure the two methods the service branches on with
/// [`with_verification`](MockSessionRepository::with_verification) and
/// [`with_session`](MockSessionRepository::with_session); the recorders
/// (`revoked_sessions`, `revoked_families`, `created_sessions`, etc.) let tests
/// assert which writes happened.
#[derive(Clone)]
pub struct MockSessionRepository {
    verification: AuthCodeVerification,
    session: Option<Session>,
    invalidated_users: Arc<Mutex<Vec<Uuid>>>,
    created_auth_codes: Arc<Mutex<usize>>,
    created_sessions: Arc<Mutex<usize>>,
    revoked_sessions: Arc<Mutex<Vec<Uuid>>>,
    revoked_families: Arc<Mutex<Vec<Uuid>>>,
    revoked_users: Arc<Mutex<Vec<Uuid>>>,
}

impl Default for MockSessionRepository {
    fn default() -> Self {
        Self {
            verification: AuthCodeVerification::Success,
            session: None,
            invalidated_users: Arc::default(),
            created_auth_codes: Arc::default(),
            created_sessions: Arc::default(),
            revoked_sessions: Arc::default(),
            revoked_families: Arc::default(),
            revoked_users: Arc::default(),
        }
    }
}

impl MockSessionRepository {
    pub fn with_verification(mut self, verification: AuthCodeVerification) -> Self {
        self.verification = verification;
        self
    }

    pub fn with_session(mut self, session: Session) -> Self {
        self.session = Some(session);
        self
    }

    pub fn invalidated_users(&self) -> Vec<Uuid> {
        self.invalidated_users.lock().unwrap().clone()
    }

    pub fn created_auth_codes(&self) -> usize {
        *self.created_auth_codes.lock().unwrap()
    }

    pub fn created_sessions(&self) -> usize {
        *self.created_sessions.lock().unwrap()
    }

    pub fn revoked_sessions(&self) -> Vec<Uuid> {
        self.revoked_sessions.lock().unwrap().clone()
    }

    pub fn revoked_families(&self) -> Vec<Uuid> {
        self.revoked_families.lock().unwrap().clone()
    }

    pub fn revoked_users(&self) -> Vec<Uuid> {
        self.revoked_users.lock().unwrap().clone()
    }
}

#[async_trait]
impl crate::repository::SessionRepository for MockSessionRepository {
    async fn create_auth_code(
        &self,
        user_id: Uuid,
        code_hash: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<AuthCode, crate::error::DatabaseError> {
        *self.created_auth_codes.lock().unwrap() += 1;
        Ok(AuthCode {
            id: Uuid::new_v4(),
            user_id,
            code_hash: code_hash.to_string(),
            attempt_count: 0,
            created_at: Utc::now(),
            expires_at,
            used_at: None,
        })
    }

    async fn get_auth_code(
        &self,
        _user_id: Uuid,
        _code_hash: &str,
    ) -> Result<Option<AuthCode>, crate::error::DatabaseError> {
        Ok(None)
    }

    async fn mark_auth_code_used(&self, _id: Uuid) -> Result<(), crate::error::DatabaseError> {
        Ok(())
    }

    async fn invalidate_auth_codes(
        &self,
        user_id: Uuid,
    ) -> Result<(), crate::error::DatabaseError> {
        self.invalidated_users.lock().unwrap().push(user_id);
        Ok(())
    }

    async fn verify_and_consume_auth_code(
        &self,
        _user_id: Uuid,
        _code_hash: &str,
        _max_attempts: i16,
    ) -> Result<AuthCodeVerification, crate::error::DatabaseError> {
        Ok(self.verification)
    }

    async fn create_session(
        &self,
        user_id: Uuid,
        refresh_token_hash: &str,
        refresh_token_family: Uuid,
        user_agent: Option<&str>,
        ip_address: Option<IpNetwork>,
        expires_at: DateTime<Utc>,
    ) -> Result<Session, crate::error::DatabaseError> {
        *self.created_sessions.lock().unwrap() += 1;
        Ok(Session {
            id: Uuid::new_v4(),
            user_id,
            refresh_token_hash: refresh_token_hash.to_string(),
            refresh_token_family,
            user_agent: user_agent.map(str::to_string),
            ip_address,
            created_at: Utc::now(),
            expires_at,
            revoked_at: None,
        })
    }

    async fn get_session_by_refresh_hash(
        &self,
        _hash: &str,
    ) -> Result<Option<Session>, crate::error::DatabaseError> {
        Ok(self.session.clone())
    }

    async fn revoke_session(&self, id: Uuid) -> Result<(), crate::error::DatabaseError> {
        self.revoked_sessions.lock().unwrap().push(id);
        Ok(())
    }

    async fn revoke_sessions_by_family(
        &self,
        family: Uuid,
    ) -> Result<(), crate::error::DatabaseError> {
        self.revoked_families.lock().unwrap().push(family);
        Ok(())
    }

    async fn revoke_sessions_by_user(
        &self,
        user_id: Uuid,
    ) -> Result<(), crate::error::DatabaseError> {
        self.revoked_users.lock().unwrap().push(user_id);
        Ok(())
    }
}

/// Hand-written to mirror [`MockSessionRepository`]: the success variant of
/// [`EmailCodeVerification`] carries a `UserEmail`, which `mockall`'s
/// `.returning` ergonomics handle poorly, and a builder keeps the account-service
/// tests readable.
///
/// Configure the verify outcome with
/// [`with_verification`](MockEmailVerificationRepository::with_verification); the
/// recorders (`created_codes`, `invalidated`) let tests assert which writes
/// happened.
#[derive(Clone)]
pub struct MockEmailVerificationRepository {
    verification: EmailCodeVerification,
    created_codes: Arc<Mutex<usize>>,
    invalidated: Arc<Mutex<Vec<(Uuid, String)>>>,
}

impl Default for MockEmailVerificationRepository {
    fn default() -> Self {
        Self {
            verification: EmailCodeVerification::NoActiveCode,
            created_codes: Arc::default(),
            invalidated: Arc::default(),
        }
    }
}

impl MockEmailVerificationRepository {
    pub fn with_verification(mut self, verification: EmailCodeVerification) -> Self {
        self.verification = verification;
        self
    }

    pub fn created_codes(&self) -> usize {
        *self.created_codes.lock().unwrap()
    }

    pub fn invalidated(&self) -> Vec<(Uuid, String)> {
        self.invalidated.lock().unwrap().clone()
    }
}

#[async_trait]
impl crate::repository::EmailVerificationRepository for MockEmailVerificationRepository {
    async fn create_code(
        &self,
        user_id: Uuid,
        email: &str,
        code_hash: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<EmailVerificationCode, crate::error::DatabaseError> {
        *self.created_codes.lock().unwrap() += 1;
        Ok(EmailVerificationCode {
            id: Uuid::new_v4(),
            user_id,
            email: email.to_string(),
            code_hash: code_hash.to_string(),
            attempt_count: 0,
            created_at: Utc::now(),
            expires_at,
            used_at: None,
        })
    }

    async fn get_code_by_hash(
        &self,
        _code_hash: &str,
    ) -> Result<Option<EmailVerificationCode>, crate::error::DatabaseError> {
        Ok(None)
    }

    async fn invalidate_codes_for_email(
        &self,
        user_id: Uuid,
        email: &str,
    ) -> Result<(), crate::error::DatabaseError> {
        self.invalidated
            .lock()
            .unwrap()
            .push((user_id, email.to_string()));
        Ok(())
    }

    async fn verify_and_consume_email_code(
        &self,
        _user_id: Uuid,
        _email: &str,
        _code_hash: &str,
        _max_attempts: i16,
    ) -> Result<EmailCodeVerification, crate::error::DatabaseError> {
        Ok(self.verification.clone())
    }
}
