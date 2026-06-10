mod info_refs;
mod receive_pack;
mod upload_pack;

use std::pin::Pin;

use futures::Stream;

pub use info_refs::InfoRefsRequest;
pub use receive_pack::ReceivePackRequest;
pub use upload_pack::UploadPackRequest;

pub enum GitHttpBody {
    Buffered(Vec<u8>),
    Stream(Pin<Box<dyn Stream<Item = Result<Vec<u8>, std::io::Error>> + Send>>),
}

pub struct GitHttpResponse {
    pub status_code: u16,
    pub headers: Vec<(String, String)>,
    pub body: GitHttpBody,
}
