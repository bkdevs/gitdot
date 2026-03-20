use gitdot_api::resource::repository as api;
use gitdot_core::{
    dto::{
        CommitAuthorResponse, CommitDiffResponse, CommitResponse, CommitsResponse, PathType,
        RepositoryBlobResponse, RepositoryBlobsResponse, RepositoryCommitResponse,
        RepositoryCommitsResponse, RepositoryDiffFileResponse, RepositoryFileResponse,
        RepositoryFolderResponse, RepositoryPath, RepositoryPathsResponse, RepositoryResponse,
        RepositorySettingsResponse,
    },
    model::CommitDiff,
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
        }
    }
}

impl IntoApi for CommitsResponse {
    type ApiType = api::RepositoryCommitsResource;
    fn into_api(self) -> Self::ApiType {
        api::RepositoryCommitsResource {
            commits: self.commits.into_iter().map(|c| c.into_api()).collect(),
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
            review_number: self.review_number,
            diff_position: self.diff_position,
            diffs: self.diffs.into_iter().map(|d| d.into_api()).collect(),
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
            review_number: None,
            diff_position: None,
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

impl IntoApi for CommitDiff {
    type ApiType = api::RepositoryDiffStatResource;
    fn into_api(self) -> Self::ApiType {
        api::RepositoryDiffStatResource {
            path: self.path,
            lines_added: self.lines_added as u32,
            lines_removed: self.lines_removed as u32,
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

impl IntoApi for RepositorySettingsResponse {
    type ApiType = api::RepositorySettingsResource;
    fn into_api(self) -> Self::ApiType {
        api::RepositorySettingsResource {
            commit_filters: self
                .commit_filters
                .map(|filters| filters.into_iter().map(|f| f.into_api()).collect()),
        }
    }
}

impl IntoApi for RepositoryDiffFileResponse {
    type ApiType = api::RepositoryDiffFileResource;
    fn into_api(self) -> Self::ApiType {
        api::RepositoryDiffFileResource {
            path: self.path,
            lines_added: self.lines_added,
            lines_removed: self.lines_removed,
            hunks: self
                .hunks
                .into_iter()
                .map(|h| h.into_iter().map(|p| p.into_api()).collect())
                .collect(),
            left_content: self.left_content,
            right_content: self.right_content,
        }
    }
}
