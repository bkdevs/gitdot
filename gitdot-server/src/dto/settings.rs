use gitdot_api::resource::settings as api;
use gitdot_core::model::CommitFilter;

use super::IntoApi;

impl IntoApi for CommitFilter {
    type ApiType = api::CommitFilterResource;
    fn into_api(self) -> Self::ApiType {
        api::CommitFilterResource {
            authors: self.authors,
            tags: self.tags,
            included_paths: self.included_paths,
            excluded_paths: self.excluded_paths,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
