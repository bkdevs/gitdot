mod build;
mod git_http;
mod internal;
mod migration;
mod organization;
mod question;
mod repository;
mod review;
mod runner;
mod settings;
mod task;
mod user;
mod webhook;

pub use git_http::*;
pub use internal::*;

pub trait FromApi: Sized {
    type ApiType;
    fn from_api(api: Self::ApiType) -> Self;
}

impl<T: FromApi> FromApi for Vec<T> {
    type ApiType = Vec<T::ApiType>;
    fn from_api(api: Vec<T::ApiType>) -> Self {
        api.into_iter().map(T::from_api).collect()
    }
}

impl<T: FromApi> FromApi for Option<T> {
    type ApiType = Option<T::ApiType>;
    fn from_api(api: Option<T::ApiType>) -> Self {
        api.map(T::from_api)
    }
}

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
