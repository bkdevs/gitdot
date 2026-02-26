use gitdot_api::resource::repository as api;
use gitdot_core::dto::{
    CommitAuthorResponse, DiffChange, DiffLine, DiffPair, FilePreview,
    RepositoryCommitDiffResponse, RepositoryCommitResponse, RepositoryCommitStatResponse,
    RepositoryCommitsResponse, RepositoryDiffResponse, RepositoryFileResponse,
    RepositoryPreviewEntry, RepositoryPreviewResponse, RepositoryResponse, RepositoryTreeEntry,
    RepositoryTreeResponse, SyntaxHighlight,
};

use super::IntoApi;

impl IntoApi for RepositoryResponse {
    type ApiType = api::RepositoryResource;
    fn into_api(self) -> Self::ApiType {
        api::RepositoryResource {
            id: self.id,
            name: self.name,
            owner: self.owner,
            visibility: self.visibility,
            created_at: self.created_at,
        }
    }
}

impl IntoApi for RepositoryCommitsResponse {
    type ApiType = api::RepositoryCommitsResource;
    fn into_api(self) -> Self::ApiType {
        api::RepositoryCommitsResource {
            commits: self.commits.into_api(),
            has_next: self.has_next,
        }
    }
}

impl IntoApi for RepositoryCommitResponse {
    type ApiType = api::RepositoryCommitResource;
    fn into_api(self) -> Self::ApiType {
        api::RepositoryCommitResource {
            sha: self.sha,
            parent_sha: self.parent_sha,
            message: self.message,
            date: self.date,
            author: self.author.into_api(),
        }
    }
}

impl IntoApi for CommitAuthorResponse {
    type ApiType = api::CommitAuthorResource;
    fn into_api(self) -> Self::ApiType {
        api::CommitAuthorResource {
            id: self.id,
            name: self.name,
            email: self.email,
        }
    }
}

impl IntoApi for RepositoryFileResponse {
    type ApiType = api::RepositoryFileResource;
    fn into_api(self) -> Self::ApiType {
        api::RepositoryFileResource {
            ref_name: self.ref_name,
            path: self.path,
            commit_sha: self.commit_sha,
            sha: self.sha,
            content: self.content,
            encoding: self.encoding,
        }
    }
}

impl IntoApi for RepositoryPreviewResponse {
    type ApiType = api::RepositoryPreviewResource;
    fn into_api(self) -> Self::ApiType {
        api::RepositoryPreviewResource {
            ref_name: self.ref_name,
            commit_sha: self.commit_sha,
            entries: self.entries.into_api(),
        }
    }
}

impl IntoApi for RepositoryPreviewEntry {
    type ApiType = api::RepositoryPreviewEntryResource;
    fn into_api(self) -> Self::ApiType {
        api::RepositoryPreviewEntryResource {
            path: self.path,
            name: self.name,
            sha: self.sha,
            preview: self.preview.into_api(),
        }
    }
}

impl IntoApi for FilePreview {
    type ApiType = api::FilePreviewResource;
    fn into_api(self) -> Self::ApiType {
        api::FilePreviewResource {
            content: self.content,
            total_lines: self.total_lines,
            preview_lines: self.preview_lines,
            truncated: self.truncated,
            encoding: self.encoding,
        }
    }
}

impl IntoApi for RepositoryTreeResponse {
    type ApiType = api::RepositoryTreeResource;
    fn into_api(self) -> Self::ApiType {
        api::RepositoryTreeResource {
            ref_name: self.ref_name,
            commit_sha: self.commit_sha,
            entries: self.entries.into_api(),
        }
    }
}

impl IntoApi for RepositoryTreeEntry {
    type ApiType = api::RepositoryTreeEntryResource;
    fn into_api(self) -> Self::ApiType {
        api::RepositoryTreeEntryResource {
            path: self.path,
            name: self.name,
            entry_type: self.entry_type,
            sha: self.sha,
            commit: self.commit.into_api(),
        }
    }
}

impl IntoApi for RepositoryCommitStatResponse {
    type ApiType = api::RepositoryCommitStatResource;
    fn into_api(self) -> Self::ApiType {
        api::RepositoryCommitStatResource {
            path: self.path,
            lines_added: self.lines_added,
            lines_removed: self.lines_removed,
        }
    }
}

impl IntoApi for RepositoryCommitDiffResponse {
    type ApiType = api::RepositoryCommitDiffResource;
    fn into_api(self) -> Self::ApiType {
        api::RepositoryCommitDiffResource {
            diff: self.diff.into_api(),
            left: self.left.into_api(),
            right: self.right.into_api(),
        }
    }
}

impl IntoApi for RepositoryDiffResponse {
    type ApiType = api::RepositoryDiffResource;
    fn into_api(self) -> Self::ApiType {
        api::RepositoryDiffResource {
            lines_added: self.lines_added,
            lines_removed: self.lines_removed,
            hunks: self.hunks.into_iter().map(|hunk| hunk.into_api()).collect(),
        }
    }
}

impl IntoApi for DiffPair {
    type ApiType = api::DiffPairResource;
    fn into_api(self) -> Self::ApiType {
        api::DiffPairResource {
            lhs: self.lhs.into_api(),
            rhs: self.rhs.into_api(),
        }
    }
}

impl IntoApi for DiffLine {
    type ApiType = api::DiffLineResource;
    fn into_api(self) -> Self::ApiType {
        api::DiffLineResource {
            line_number: self.line_number,
            changes: self.changes.into_api(),
        }
    }
}

impl IntoApi for DiffChange {
    type ApiType = api::DiffChangeResource;
    fn into_api(self) -> Self::ApiType {
        api::DiffChangeResource {
            start: self.start,
            end: self.end,
            content: self.content,
            highlight: self.highlight.into_api(),
        }
    }
}

impl IntoApi for SyntaxHighlight {
    type ApiType = api::SyntaxHighlight;
    fn into_api(self) -> Self::ApiType {
        match self {
            SyntaxHighlight::Delimiter => api::SyntaxHighlight::Delimiter,
            SyntaxHighlight::Normal => api::SyntaxHighlight::Normal,
            SyntaxHighlight::String => api::SyntaxHighlight::String,
            SyntaxHighlight::Type => api::SyntaxHighlight::Type,
            SyntaxHighlight::Comment => api::SyntaxHighlight::Comment,
            SyntaxHighlight::Keyword => api::SyntaxHighlight::Keyword,
            SyntaxHighlight::TreeSitterError => api::SyntaxHighlight::TreeSitterError,
        }
    }
}
