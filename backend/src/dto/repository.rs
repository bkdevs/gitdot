use gitdot_api::resource::repository as api;
use gitdot_core::dto::{
    CommitAuthorResponse, FilePreview, RepositoryCommitResponse, RepositoryCommitsResponse,
    RepositoryFileResponse, RepositoryPreviewEntry, RepositoryPreviewResponse, RepositoryResponse,
    RepositoryTreeEntry, RepositoryTreeResponse,
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
