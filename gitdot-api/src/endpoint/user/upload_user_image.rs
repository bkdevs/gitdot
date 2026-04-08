use crate::{endpoint::Endpoint, resource::user::UploadUserImageResource};

pub struct UploadUserImage;

impl Endpoint for UploadUserImage {
    const PATH: &'static str = "/user/image";
    const METHOD: http::Method = http::Method::POST;

    type Request = ();
    type Response = UploadUserImageResource;
}

pub type UploadUserImageResponse = UploadUserImageResource;
