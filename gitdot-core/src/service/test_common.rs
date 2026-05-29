use async_trait::async_trait;
use bytes::Bytes;
use chrono::{DateTime, Utc};
use mockall::mock;
use uuid::Uuid;

use crate::{
    model::{
        Answer, AuthProvider, Comment, CommentSide, Commit, CommitDiff, Diff, DiffStatus,
        Organization, OrganizationMember, OrganizationRole, Question, Repository,
        RepositoryOwnerType, RepositoryStar, RepositoryVisibility, Review, ReviewComment,
        ReviewStatus, Reviewer, Revision, User, UserEmail, UserOrganization, Verdict, VoteResult,
        VoteTarget,
    },
    repository::{
        CommitRepository, OrganizationRepository, QuestionRepository, RepositoryRepository,
        ReviewRepository, UserRepository,
    },
};

mock! {
    pub OrganizationRepo {}
    impl Clone for OrganizationRepo {
        fn clone(&self) -> Self;
    }
    #[async_trait]
    impl OrganizationRepository for OrganizationRepo {
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
    pub RepositoryRepo {}
    impl Clone for RepositoryRepo {
        fn clone(&self) -> Self;
    }
    #[async_trait]
    impl RepositoryRepository for RepositoryRepo {
        async fn create(&self, name: &str, owner_id: Uuid, owner_type: &RepositoryOwnerType, visibility: &RepositoryVisibility, description: Option<String>, readonly: bool, created_at: Option<chrono::DateTime<chrono::Utc>>) -> Result<Repository, crate::error::DatabaseError>;
        async fn get(&self, owner: &str, repo: &str, viewer_id: Option<Uuid>) -> Result<Option<Repository>, crate::error::DatabaseError>;
        async fn get_id(&self, owner: &str, repo: &str) -> Result<Option<Uuid>, crate::error::DatabaseError>;
        async fn get_by_id(&self, id: Uuid, viewer_id: Option<Uuid>) -> Result<Option<Repository>, crate::error::DatabaseError>;
        async fn list_by_owner(&self, owner_name: &str, viewer_id: Option<Uuid>, cursor: Option<crate::dto::Cursor>, limit: i64) -> Result<(Vec<Repository>, Option<crate::dto::Cursor>), crate::error::DatabaseError>;
        async fn list_latest(&self, limit: i64) -> Result<Vec<Repository>, crate::error::DatabaseError>;
        async fn list_trending(&self, limit: i64) -> Result<Vec<Repository>, crate::error::DatabaseError>;
        async fn delete(&self, id: Uuid) -> Result<(), crate::error::DatabaseError>;
        async fn update(&self, id: Uuid, description: Option<String>) -> Result<Option<Repository>, crate::error::DatabaseError>;
        async fn disable_readonly(&self, owner: &str, repo: &str) -> Result<Option<Repository>, crate::error::DatabaseError>;
        async fn star(&self, id: Uuid, user_id: Uuid) -> Result<Option<RepositoryStar>, crate::error::DatabaseError>;
        async fn unstar(&self, id: Uuid, user_id: Uuid) -> Result<bool, crate::error::DatabaseError>;
        async fn list_recent_stars(&self, repository_id: Uuid, limit: i64) -> Result<Vec<(User, DateTime<Utc>)>, crate::error::DatabaseError>;
        async fn list_commit_filters(&self, repository_id: Uuid, cursor: Option<crate::dto::Cursor>, limit: i64) -> Result<(Vec<crate::model::CommitFilter>, Option<crate::dto::Cursor>), crate::error::DatabaseError>;
        async fn create_commit_filter(&self, repository_id: Uuid, name: &str, authors: Option<Vec<String>>, tags: Option<Vec<String>>, paths: Option<Vec<String>>) -> Result<crate::model::CommitFilter, crate::error::DatabaseError>;
        async fn update_commit_filter(&self, filter_id: Uuid, name: &str, authors: Option<Vec<String>>, tags: Option<Vec<String>>, paths: Option<Vec<String>>) -> Result<Option<crate::model::CommitFilter>, crate::error::DatabaseError>;
        async fn delete_commit_filter(&self, filter_id: Uuid) -> Result<bool, crate::error::DatabaseError>;
    }
}

mock! {
    pub QuestionRepo {}
    impl Clone for QuestionRepo {
        fn clone(&self) -> Self;
    }
    #[async_trait]
    impl QuestionRepository for QuestionRepo {
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
    pub UserRepo {}
    impl Clone for UserRepo {
        fn clone(&self) -> Self;
    }
    #[async_trait]
    impl UserRepository for UserRepo {
        async fn create(&self, email: &str, is_email_verified: bool, provider: AuthProvider) -> Result<User, crate::error::DatabaseError>;
        async fn get(&self, user_name: &str) -> Result<Option<User>, crate::error::DatabaseError>;
        async fn update(&self, id: Uuid, name: Option<String>, location: Option<String>, readme: Option<String>, links: Option<Vec<String>>, display_name: Option<String>) -> Result<User, crate::error::DatabaseError>;
        async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, crate::error::DatabaseError>;
        async fn touch_image(&self, id: Uuid) -> Result<(), crate::error::DatabaseError>;
        async fn get_by_email(&self, email: &str) -> Result<Option<User>, crate::error::DatabaseError>;
        async fn get_by_emails(&self, emails: &[String]) -> Result<Vec<(String, Uuid)>, crate::error::DatabaseError>;
        async fn verify_email(&self, id: Uuid) -> Result<(), crate::error::DatabaseError>;
        async fn is_name_taken(&self, name: &str) -> Result<bool, crate::error::DatabaseError>;
        async fn list_emails(&self, user_id: Uuid) -> Result<Vec<UserEmail>, crate::error::DatabaseError>;
        async fn create_email(&self, user_id: Uuid, email: &str) -> Result<UserEmail, crate::error::DatabaseError>;
        async fn get_email_for_user(&self, user_id: Uuid, email: &str) -> Result<Option<UserEmail>, crate::error::DatabaseError>;
        async fn upsert_verified_emails(&self, user_id: Uuid, emails: &[String]) -> Result<(), crate::error::DatabaseError>;
        async fn list_repositories(&self, user_id: Uuid, viewer_id: Option<Uuid>, cursor: Option<crate::dto::Cursor>, limit: i64) -> Result<(Vec<(Repository, Option<i64>, Option<chrono::DateTime<chrono::Utc>>)>, Option<crate::dto::Cursor>), crate::error::DatabaseError>;
        async fn list_starred_repositories(&self, user_id: Uuid, viewer_id: Option<Uuid>, cursor: Option<crate::dto::Cursor>, limit: i64) -> Result<(Vec<Repository>, Option<crate::dto::Cursor>), crate::error::DatabaseError>;
        async fn list_contributed_repositories(&self, user_id: Uuid, viewer_id: Option<Uuid>, since: chrono::DateTime<chrono::Utc>, cursor: Option<crate::dto::Cursor>, limit: i64) -> Result<(Vec<(Repository, i64, chrono::DateTime<chrono::Utc>)>, Option<crate::dto::Cursor>), crate::error::DatabaseError>;
    }
}

mock! {
    pub ReviewRepo {}
    impl Clone for ReviewRepo {
        fn clone(&self) -> Self;
    }
    #[async_trait]
    impl ReviewRepository for ReviewRepo {
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
    pub CommitRepo {}
    impl Clone for CommitRepo {
        fn clone(&self) -> Self;
    }
    #[async_trait]
    impl CommitRepository for CommitRepo {
        async fn get_commit(&self, repo_id: Uuid, sha: &str) -> Result<Option<Commit>, crate::error::DatabaseError>;
        async fn list_by_repository(&self, repo_id: Uuid, ref_name: &str, from: DateTime<Utc>, to: DateTime<Utc>, cursor: Option<crate::dto::Cursor>, limit: i64) -> Result<(Vec<Commit>, Option<crate::dto::Cursor>), crate::error::DatabaseError>;
        async fn list_by_user(&self, author_id: Uuid, viewer_id: Option<Uuid>, from: DateTime<Utc>, to: DateTime<Utc>, cursor: Option<crate::dto::Cursor>, limit: i64) -> Result<(Vec<(Commit, bool)>, Option<crate::dto::Cursor>), crate::error::DatabaseError>;
        async fn create_bulk(&self, author_ids: &[Option<Uuid>], git_author_names: &[String], git_author_emails: &[String], repo_ids: &[Uuid], ref_names: &[String], shas: &[String], parent_shas: &[String], messages: &[String], created_ats: &[DateTime<Utc>], diffs: &[Vec<CommitDiff>], review_numbers: &[Option<i32>], diff_positions: &[Option<i32>]) -> Result<Vec<Commit>, crate::error::DatabaseError>;
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
