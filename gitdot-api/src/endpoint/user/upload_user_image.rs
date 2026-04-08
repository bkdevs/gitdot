use crate::{endpoint::Endpoint, resource::user::UserResource};

pub struct UploadUserImage;

impl Endpoint for UploadUserImage {
    const PATH: &'static str = "/user/image";
    const METHOD: http::Method = http::Method::POST;

    type Request = ();
    type Response = UserResource;
}

pub type UploadUserImageResponse = UserResource;
