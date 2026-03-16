use gitdot_api::resource::{repository as repo_api, review as api};
use gitdot_core::dto::{
    DiffChange, DiffLine, DiffPair, DiffResponse, ReviewAuthorResponse, ReviewCommentResponse,
    ReviewDiffResponse, ReviewFileDiffResponse, ReviewResponse, ReviewerResponse, RevisionResponse,
    SyntaxHighlight,
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
            parent_hash: self.parent_hash,
            created_at: self.created_at,
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

impl IntoApi for ReviewDiffResponse {
    type ApiType = gitdot_api::endpoint::review::get_review_diff::GetReviewDiffResponse;
    fn into_api(self) -> Self::ApiType {
        gitdot_api::endpoint::review::get_review_diff::GetReviewDiffResponse {
            files: self.files.into_api(),
        }
    }
}

impl IntoApi for ReviewFileDiffResponse {
    type ApiType = repo_api::RepositoryDiffFileResource;
    fn into_api(self) -> Self::ApiType {
        repo_api::RepositoryDiffFileResource {
            path: self.path,
            lines_added: self.diff.lines_added,
            lines_removed: self.diff.lines_removed,
            hunks: self
                .diff
                .hunks
                .into_iter()
                .map(|h| h.into_iter().map(|p| p.into_api()).collect())
                .collect(),
            left_content: None,
            right_content: None,
        }
    }
}

impl IntoApi for DiffPair {
    type ApiType = repo_api::DiffPairResource;
    fn into_api(self) -> Self::ApiType {
        repo_api::DiffPairResource {
            lhs: self.lhs.map(|l| l.into_api()),
            rhs: self.rhs.map(|r| r.into_api()),
        }
    }
}

impl IntoApi for DiffLine {
    type ApiType = repo_api::DiffLineResource;
    fn into_api(self) -> Self::ApiType {
        repo_api::DiffLineResource {
            line_number: self.line_number,
            changes: self.changes.into_iter().map(|c| c.into_api()).collect(),
        }
    }
}

impl IntoApi for DiffChange {
    type ApiType = repo_api::DiffChangeResource;
    fn into_api(self) -> Self::ApiType {
        repo_api::DiffChangeResource {
            start: self.start,
            end: self.end,
            content: self.content,
            highlight: self.highlight.into_api(),
        }
    }
}

impl IntoApi for SyntaxHighlight {
    type ApiType = repo_api::SyntaxHighlight;
    fn into_api(self) -> Self::ApiType {
        match self {
            SyntaxHighlight::Delimiter => repo_api::SyntaxHighlight::Delimiter,
            SyntaxHighlight::Normal => repo_api::SyntaxHighlight::Normal,
            SyntaxHighlight::String => repo_api::SyntaxHighlight::String,
            SyntaxHighlight::Type => repo_api::SyntaxHighlight::Type,
            SyntaxHighlight::Comment => repo_api::SyntaxHighlight::Comment,
            SyntaxHighlight::Keyword => repo_api::SyntaxHighlight::Keyword,
            SyntaxHighlight::TreeSitterError => repo_api::SyntaxHighlight::TreeSitterError,
        }
    }
}
