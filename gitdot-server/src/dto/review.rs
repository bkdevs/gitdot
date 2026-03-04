use gitdot_api::resource::review as api;
use gitdot_core::dto::{
    DiffResponse, ReviewAuthorResponse, ReviewCommentResponse, ReviewResponse, ReviewerResponse,
    RevisionResponse,
};

use super::IntoApi;

impl IntoApi for ReviewResponse {
    type ApiType = api::ReviewResource;
    fn into_api(self) -> Self::ApiType {
        api::ReviewResource {
            id: self.id,
            number: self.number,
            author_id: self.author_id,
            repository_id: self.repository_id,
            title: self.title,
            description: self.description,
            target_branch: self.target_branch,
            status: self.status,
            created_at: self.created_at,
            updated_at: self.updated_at,
            author: self.author.into_api(),
            diffs: self.diffs.into_api(),
            reviewers: self.reviewers.into_api(),
            comments: self.comments.into_api(),
        }
    }
}

impl IntoApi for ReviewAuthorResponse {
    type ApiType = api::ReviewAuthorResource;
    fn into_api(self) -> Self::ApiType {
        api::ReviewAuthorResource {
            id: self.id,
            name: self.name,
        }
    }
}

impl IntoApi for DiffResponse {
    type ApiType = api::DiffResource;
    fn into_api(self) -> Self::ApiType {
        api::DiffResource {
            id: self.id,
            review_id: self.review_id,
            position: self.position,
            title: self.title,
            description: self.description,
            status: self.status,
            created_at: self.created_at,
            updated_at: self.updated_at,
            revisions: self.revisions.into_api(),
        }
    }
}

impl IntoApi for RevisionResponse {
    type ApiType = api::RevisionResource;
    fn into_api(self) -> Self::ApiType {
        api::RevisionResource {
            id: self.id,
            diff_id: self.diff_id,
            number: self.number,
            commit_hash: self.commit_hash,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl IntoApi for ReviewerResponse {
    type ApiType = api::ReviewerResource;
    fn into_api(self) -> Self::ApiType {
        api::ReviewerResource {
            id: self.id,
            review_id: self.review_id,
            reviewer_id: self.reviewer_id,
            status: self.status,
            created_at: self.created_at,
            user: self.user.into_api(),
        }
    }
}

impl IntoApi for ReviewCommentResponse {
    type ApiType = api::ReviewCommentResource;
    fn into_api(self) -> Self::ApiType {
        api::ReviewCommentResource {
            id: self.id,
            review_id: self.review_id,
            diff_id: self.diff_id,
            revision_id: self.revision_id,
            author_id: self.author_id,
            parent_id: self.parent_id,
            body: self.body,
            file_path: self.file_path,
            line_number: self.line_number,
            side: self.side,
            resolved: self.resolved,
            created_at: self.created_at,
            updated_at: self.updated_at,
            author: self.author.into_api(),
        }
    }
}
