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

cfg_modules!("main", {
    mod organization;
    mod user;
    mod repository;
    mod git_http;
    pub mod legacy_repository;
    mod oauth;
    mod question;
    pub use git_http::*;
    pub use oauth::*;
});

cfg_modules!("ci", {
    mod dag;
    mod runner;
    mod task;
});
