mod info_refs;
mod receive_pack;
mod upload_pack;

pub use info_refs::InfoRefsRequest;
pub use receive_pack::ReceivePackRequest;
pub use upload_pack::UploadPackRequest;

#[derive(Debug, Clone)]
pub struct GitHttpResponse {
    pub status_code: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}
