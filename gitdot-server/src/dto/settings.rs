use gitdot_api::resource::settings as api;
use gitdot_core::model::CommitFilter;

use super::{FromApi, IntoApi};

impl FromApi for CommitFilter {
    type ApiType = api::CommitFilterResource;
    fn from_api(f: api::CommitFilterResource) -> Self {
        CommitFilter {
            name: f.name,
            authors: f.authors,
            tags: f.tags,
            included_paths: f.included_paths,
            excluded_paths: f.excluded_paths,
            created_at: f.created_at,
            updated_at: f.updated_at,
        }
    }
}

impl IntoApi for CommitFilter {
    type ApiType = api::CommitFilterResource;
    fn into_api(self) -> Self::ApiType {
        api::CommitFilterResource {
            name: self.name,
            authors: self.authors,
            tags: self.tags,
            included_paths: self.included_paths,
            excluded_paths: self.excluded_paths,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
