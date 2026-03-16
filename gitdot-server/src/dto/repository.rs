use gitdot_api::resource::repository as api;
use gitdot_core::{
    dto::{
        CommitAuthorResponse, CommitDiffResponse, CommitFileDiffResponse, CommitResponse,
        CommitsResponse, PathType, RepositoryBlobResponse, RepositoryBlobsResponse,
        RepositoryCommitResponse, RepositoryCommitsResponse, RepositoryFileResponse,
        RepositoryFolderResponse, RepositoryPath, RepositoryPathsResponse, RepositoryResponse,
    },
    model::{
        CommitDiff, CommitDiffChange, CommitDiffLine, CommitDiffPair, CommitDiffSyntaxHighlight,
    },
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

impl IntoApi for CommitsResponse {
    type ApiType = api::RepositoryCommitsResource;
    fn into_api(self) -> Self::ApiType {
        api::RepositoryCommitsResource {
            commits: self.commits.into_iter().map(|c| c.into_api()).collect(),
            has_next: self.has_next,
        }
    }
}

// TODO: think a tad on the commit author this is no longer real.
impl IntoApi for CommitResponse {
    type ApiType = api::RepositoryCommitResource;
    fn into_api(self) -> Self::ApiType {
        api::RepositoryCommitResource {
            sha: self.sha,
            parent_sha: self.parent_sha,
            message: self.message,
            date: self.created_at,
            author: api::CommitAuthorResource {
                id: self.author_id,
                name: self.git_author_name,
                email: self.git_author_email,
            },
            diffs: self
                .diffs
                .into_iter()
                .map(|d| api::RepositoryDiffResource {
                    path: d.path,
                    lines_added: d.lines_added as u32,
                    lines_removed: d.lines_removed as u32,
                    hunks: d.hunks.into_iter().map(|h| h.into_api()).collect(),
                })
                .collect(),
        }
    }
}

impl IntoApi for RepositoryCommitResponse {
    type ApiType = api::RepositoryCommitResource;
    fn into_api(self) -> Self::ApiType {
        api::RepositoryCommitResource {
            sha: self.sha,
            parent_sha: self
                .parent_sha
                .unwrap_or_else(|| "0000000000000000000000000000000000000000".to_string()),
            message: self.message,
            date: self.date,
            author: self.author.into_api(),
            diffs: vec![],
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
            path: self.path,
            sha: self.sha,
            content: self.content,
            encoding: self.encoding,
        }
    }
}

impl IntoApi for RepositoryFolderResponse {
    type ApiType = api::RepositoryFolderResource;
    fn into_api(self) -> Self::ApiType {
        api::RepositoryFolderResource {
            path: self.path,
            entries: self.entries.into_api(),
        }
    }
}

impl IntoApi for RepositoryBlobsResponse {
    type ApiType = api::RepositoryBlobsResource;
    fn into_api(self) -> Self::ApiType {
        api::RepositoryBlobsResource {
            ref_name: self.ref_name,
            commit_sha: self.commit_sha,
            blobs: self.blobs.into_api(),
        }
    }
}

impl IntoApi for RepositoryBlobResponse {
    type ApiType = api::RepositoryBlobResource;
    fn into_api(self) -> Self::ApiType {
        match self {
            RepositoryBlobResponse::File(f) => api::RepositoryBlobResource::File(f.into_api()),
            RepositoryBlobResponse::Folder(f) => api::RepositoryBlobResource::Folder(f.into_api()),
        }
    }
}

impl IntoApi for RepositoryPathsResponse {
    type ApiType = api::RepositoryPathsResource;
    fn into_api(self) -> Self::ApiType {
        api::RepositoryPathsResource {
            ref_name: self.ref_name,
            commit_sha: self.commit_sha,
            entries: self.entries.into_api(),
        }
    }
}

impl IntoApi for RepositoryPath {
    type ApiType = api::RepositoryPathResource;
    fn into_api(self) -> Self::ApiType {
        api::RepositoryPathResource {
            path: self.path,
            name: self.name,
            path_type: self.path_type.into_api(),
            sha: self.sha,
        }
    }
}

impl IntoApi for PathType {
    type ApiType = api::PathType;
    fn into_api(self) -> Self::ApiType {
        match self {
            PathType::Blob => api::PathType::Blob,
            PathType::Tree => api::PathType::Tree,
            PathType::Commit => api::PathType::Commit,
            PathType::Unknown => api::PathType::Unknown,
        }
    }
}

impl IntoApi for CommitDiffPair {
    type ApiType = api::DiffPairResource;
    fn into_api(self) -> Self::ApiType {
        api::DiffPairResource {
            lhs: self.lhs.into_api(),
            rhs: self.rhs.into_api(),
        }
    }
}

impl IntoApi for CommitDiffLine {
    type ApiType = api::DiffLineResource;
    fn into_api(self) -> Self::ApiType {
        api::DiffLineResource {
            line_number: self.line_number,
            changes: self.changes.into_api(),
        }
    }
}

impl IntoApi for CommitDiffChange {
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

impl IntoApi for CommitDiffSyntaxHighlight {
    type ApiType = api::SyntaxHighlight;
    fn into_api(self) -> Self::ApiType {
        match self {
            CommitDiffSyntaxHighlight::Delimiter => api::SyntaxHighlight::Delimiter,
            CommitDiffSyntaxHighlight::Normal => api::SyntaxHighlight::Normal,
            CommitDiffSyntaxHighlight::String => api::SyntaxHighlight::String,
            CommitDiffSyntaxHighlight::Type => api::SyntaxHighlight::Type,
            CommitDiffSyntaxHighlight::Comment => api::SyntaxHighlight::Comment,
            CommitDiffSyntaxHighlight::Keyword => api::SyntaxHighlight::Keyword,
            CommitDiffSyntaxHighlight::TreeSitterError => api::SyntaxHighlight::TreeSitterError,
        }
    }
}

impl IntoApi for CommitDiff {
    type ApiType = api::RepositoryDiffResource;
    fn into_api(self) -> Self::ApiType {
        api::RepositoryDiffResource {
            path: self.path,
            lines_added: self.lines_added as u32,
            lines_removed: self.lines_removed as u32,
            hunks: self.hunks.into_iter().map(|h| h.into_api()).collect(),
        }
    }
}

impl IntoApi for CommitDiffResponse {
    type ApiType = api::RepositoryCommitDiffResource;
    fn into_api(self) -> Self::ApiType {
        api::RepositoryCommitDiffResource {
            sha: self.sha,
            parent_sha: self.parent_sha,
            files: self.files.into_iter().map(|f| f.into_api()).collect(),
        }
    }
}

impl IntoApi for CommitFileDiffResponse {
    type ApiType = api::CommitFileDiffResource;
    fn into_api(self) -> Self::ApiType {
        api::CommitFileDiffResource {
            path: self.path,
            left_content: self.left_content,
            right_content: self.right_content,
            diff: self.diff.into_api(),
        }
    }
}
