mod build;
mod git_http;
mod internal;
mod migration;
mod oauth;
mod organization;
mod question;
mod repository;
mod runner;
mod task;
mod user;

pub use git_http::*;
pub use internal::*;

pub trait IntoApi {
    type ApiType;
    fn into_api(self) -> Self::ApiType;
}

impl<T> IntoApi for Vec<T>
where
    T: IntoApi,
{
    type ApiType = Vec<T::ApiType>;
    fn into_api(self) -> Self::ApiType {
        self.into_iter().map(|item| item.into_api()).collect()
    }
}

impl<T> IntoApi for Option<T>
where
    T: IntoApi,
{
    type ApiType = Option<T::ApiType>;
    fn into_api(self) -> Self::ApiType {
        self.map(|item| item.into_api())
    }
}
