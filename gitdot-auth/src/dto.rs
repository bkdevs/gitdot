mod auth;

pub trait IntoApi {
    type ApiType;
    fn into_api(self) -> Self::ApiType;
}
