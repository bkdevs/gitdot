#[derive(Debug, Clone)]
pub struct GitHttpBackendResponse {
    pub status_code: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}
