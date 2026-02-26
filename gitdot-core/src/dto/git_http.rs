mod info_refs;
mod receive_pack;
mod upload_pack;

use std::pin::Pin;

use futures::Stream;
use nutype::nutype;

pub use info_refs::InfoRefsRequest;
pub use receive_pack::ReceivePackRequest;
pub use upload_pack::UploadPackRequest;

#[nutype(
    validate(predicate = |s| s == "git-upload-pack" || s == "git-receive-pack"),
    derive(Debug, Clone, PartialEq, Eq, AsRef, Deref)
)]
pub struct GitService(String);

#[nutype(
    validate(predicate = |s| s == "application/x-git-upload-pack-request"
                           || s == "application/x-git-receive-pack-request"),
    derive(Debug, Clone, PartialEq, Eq, AsRef, Deref)
)]
pub struct GitContentType(String);

pub enum GitHttpBody {
    Buffered(Vec<u8>),
    Stream(Pin<Box<dyn Stream<Item = Result<Vec<u8>, std::io::Error>> + Send>>),
}

pub struct GitHttpResponse {
    pub status_code: u16,
    pub headers: Vec<(String, String)>,
    pub body: GitHttpBody,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod git_service {
        use super::*;

        #[test]
        fn valid_upload_pack() {
            let service = GitService::try_new("git-upload-pack".to_string()).unwrap();
            assert_eq!(service.as_ref(), "git-upload-pack");
        }

        #[test]
        fn valid_receive_pack() {
            let service = GitService::try_new("git-receive-pack".to_string()).unwrap();
            assert_eq!(service.as_ref(), "git-receive-pack");
        }

        #[test]
        fn rejects_invalid_service() {
            assert!(GitService::try_new("git-fetch".to_string()).is_err());
            assert!(GitService::try_new("upload-pack".to_string()).is_err());
            assert!(GitService::try_new("".to_string()).is_err());
            assert!(GitService::try_new("git-upload-pack ".to_string()).is_err());
        }
    }

    mod git_content_type {
        use super::*;

        #[test]
        fn valid_upload_pack_request() {
            let ct = GitContentType::try_new("application/x-git-upload-pack-request".to_string())
                .unwrap();
            assert_eq!(ct.as_ref(), "application/x-git-upload-pack-request");
        }

        #[test]
        fn valid_receive_pack_request() {
            let ct = GitContentType::try_new("application/x-git-receive-pack-request".to_string())
                .unwrap();
            assert_eq!(ct.as_ref(), "application/x-git-receive-pack-request");
        }

        #[test]
        fn rejects_invalid_content_type() {
            assert!(GitContentType::try_new("text/plain".to_string()).is_err());
            assert!(GitContentType::try_new("application/json".to_string()).is_err());
            assert!(GitContentType::try_new("".to_string()).is_err());
        }
    }
}
