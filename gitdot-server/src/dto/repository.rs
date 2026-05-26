use gitdot_api::resource::repository as api;
use gitdot_core::{
    dto::{
        CommitAuthorResponse, CommitDiffResponse, CommitResponse, PathType,
        RepositoryActivityEvent, RepositoryBlobDiffsResponse, RepositoryBlobResponse,
        RepositoryBlobsResponse, RepositoryCommitFilterResponse, RepositoryCommitResponse,
        RepositoryCommitsResponse, RepositoryDiffFileResponse, RepositoryFileResponse,
        RepositoryFolderResponse, RepositoryPath, RepositoryPathsResponse, RepositoryResponse,
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
            description: self.description,
            stars: self.stars,
            user_star: self.user_star,
            readonly: self.readonly,
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

impl IntoApi for CommitResponse {
    type ApiType = api::RepositoryCommitResource;
    fn into_api(self) -> Self::ApiType {
        api::RepositoryCommitResource {
            owner_name: self.owner_name,
            repo_name: self.repo_name,
            sha: self.sha,
            parent_sha: self.parent_sha,
            message: self.message,
            date: self.created_at,
            author: api::CommitAuthorResource {
                id: self.author_id,
                name: self.author_name,
                git_name: self.git_author_name,
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
            owner_name: String::new(),
            repo_name: String::new(),
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
            name: None,
            git_name: self.name,
            email: self.email,
        }
    }
}

impl IntoApi for RepositoryFileResponse {
    type ApiType = api::RepositoryFileResource;
    fn into_api(self) -> Self::ApiType {
        api::RepositoryFileResource {
            commit_sha: self.commit_sha,
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
            commit_sha: self.commit_sha,
            path: self.path,
            entries: self.entries.into_api(),
        }
    }
}

impl IntoApi for RepositoryBlobsResponse {
    type ApiType = api::RepositoryBlobsResource;
    fn into_api(self) -> Self::ApiType {
        api::RepositoryBlobsResource {
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

impl IntoApi for RepositoryBlobDiffsResponse {
    type ApiType = api::RepositoryBlobDiffsResource;
    fn into_api(self) -> Self::ApiType {
        api::RepositoryBlobDiffsResource {
            diffs: self
                .diffs
                .into_iter()
                .map(|(k, v)| (k, v.into_api()))
                .collect(),
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
            left_content: self.left_content,
            right_content: self.right_content,
        }
    }
}

impl IntoApi for RepositoryActivityEvent {
    type ApiType = api::RepositoryActivityEventResource;
    fn into_api(self) -> Self::ApiType {
        match self {
            RepositoryActivityEvent::Starred { user, at } => {
                api::RepositoryActivityEventResource::Starred {
                    user: user.into_api(),
                    at,
                }
            }
        }
    }
}

impl IntoApi for RepositoryCommitFilterResponse {
    type ApiType = api::RepositoryCommitFilterResource;
    fn into_api(self) -> Self::ApiType {
        api::RepositoryCommitFilterResource {
            id: self.id,
            repository_id: self.repository_id,
            name: self.name,
            authors: self.authors,
            tags: self.tags,
            paths: self.paths,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
