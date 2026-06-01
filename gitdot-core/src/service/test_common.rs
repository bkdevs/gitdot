use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::{
    dto::GitHubEmail,
    model::{
        AuthProvider, Commit, CommitRepository, Organization, OrganizationMember, OrganizationRole,
        Repository, RepositoryOwnerType, RepositoryVisibility, Session, User, UserEmail,
    },
};

pub fn create_session(user_id: Uuid) -> Session {
    Session {
        id: Uuid::new_v4(),
        user_id,
        refresh_token_hash: "refresh-token-hash".to_string(),
        refresh_token_family: Uuid::new_v4(),
        user_agent: None,
        ip_address: None,
        created_at: Utc::now(),
        expires_at: Utc::now() + Duration::days(30),
        revoked_at: None,
    }
}

pub fn create_user(name: &str) -> User {
    User {
        id: Uuid::new_v4(),
        name: name.to_string(),
        provider: AuthProvider::Email,
        display_name: None,
        location: None,
        readme: None,
        links: vec![],
        created_at: Utc::now(),
        image_updated_at: Utc::now(),
        deleted_at: None,
        emails: vec![],
    }
}

pub fn create_user_email(user_id: Uuid, email: &str) -> UserEmail {
    UserEmail {
        id: Uuid::new_v4(),
        user_id,
        email: email.to_string(),
        is_primary: false,
        is_verified: true,
        created_at: Utc::now(),
    }
}

pub fn create_repository(
    owner_id: Uuid,
    owner_type: RepositoryOwnerType,
    visibility: RepositoryVisibility,
) -> Repository {
    Repository {
        id: Uuid::new_v4(),
        name: "myrepo".to_string(),
        owner_id,
        owner_name: "owner".to_string(),
        owner_type,
        visibility,
        description: None,
        stars: 0,
        user_star: false,
        readonly: false,
        created_at: Utc::now(),
    }
}

pub fn create_organization(name: &str) -> Organization {
    Organization {
        id: Uuid::new_v4(),
        name: name.to_string(),
        display_name: None,
        location: None,
        readme: None,
        links: vec![],
        created_at: Utc::now(),
        image_updated_at: Utc::now(),
        members: None,
    }
}

pub fn create_member(user_name: &str, role: OrganizationRole) -> OrganizationMember {
    OrganizationMember {
        id: Uuid::new_v4(),
        user_id: Uuid::new_v4(),
        user_name: user_name.to_string(),
        role,
        role_description: None,
        created_at: Utc::now(),
        image_updated_at: Utc::now(),
    }
}

pub fn create_commit(sha: &str) -> Commit {
    Commit {
        id: Uuid::new_v4(),
        repo_id: Uuid::new_v4(),
        author_id: Some(Uuid::new_v4()),
        author_name: Some("alice".to_string()),
        author_image_updated_at: None,
        git_author_name: "Alice".to_string(),
        git_author_email: "alice@example.com".to_string(),
        ref_name: "refs/heads/main".to_string(),
        sha: sha.to_string(),
        parent_sha: String::new(),
        message: "msg".to_string(),
        created_at: Utc::now(),
        review_number: None,
        diff_position: None,
        diffs: vec![],
        repository: CommitRepository {
            id: Uuid::new_v4(),
            owner_name: "alice".to_string(),
            name: "repo".to_string(),
            visibility: "public".to_string(),
        },
    }
}

pub fn create_github_email(email: &str, primary: bool, verified: bool) -> GitHubEmail {
    GitHubEmail {
        email: email.to_string(),
        primary,
        verified,
    }
}
